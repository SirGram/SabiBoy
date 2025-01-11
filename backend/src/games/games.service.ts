import {
  ConflictException,
  Injectable,
  InternalServerErrorException,
  Logger,
  NotFoundException,
} from '@nestjs/common';
import { CreateGameDto } from './dto/create-game.dto';
import { UpdateGameDto } from './dto/update-game.dto';
import * as path from 'path';

import * as fs from 'fs/promises';
import { Model } from 'mongoose';
import { InjectModel } from '@nestjs/mongoose';
import { Game } from 'src/schemas/game.shema';

export interface GameListItem {
  slug: string;
  name: string;
  language: string;
  coverPath?: string;
}
export interface GameDetails extends Game {
  coverPath?: string;
  screenshotPaths?: string[];
  rom: TRomSource;
  saveState?: TSaveStateSource;
}
interface TRomSource {
  type: 'url';
  path: string | undefined;
}
interface TSaveStateSource {
  type: 'url';
  path: string | undefined;
}

@Injectable()
export class GamesService {
  private readonly logger = new Logger(GamesService.name);

  private readonly gamesPath: string;

  constructor(@InjectModel(Game.name) private gameModel: Model<Game>) {
    this.gamesPath =
      process.env.NODE_ENV === 'production'
        ? '/app/games'
        : path.join(process.cwd(), '..', 'games');
  }

  async create(createGameDto: CreateGameDto): Promise<Game> {
    const existingGame = await this.gameModel
      .findOne({ slug: createGameDto.slug })
      .exec();
    if (existingGame) {
      throw new ConflictException(
        `Game with slug ${createGameDto.slug} already exists.`,
      );
    }
    const createdGame = new this.gameModel(createGameDto);
    return createdGame.save();
  }

  async createGameWithFiles(
    createGameDto: CreateGameDto,
    files: Express.Multer.File[],
  ): Promise<Game> {
    const gameFolderPath = path.join(this.gamesPath, createGameDto.slug);
    let createdGame: Game | null = null;


    try {
      // Create game directories
      await fs.mkdir(gameFolderPath, { recursive: true });
      await fs.mkdir(path.join(gameFolderPath, 'screenshots'), {
        recursive: true,
      });

      // Process and validate files
      let hasRomFile = false;

      for (const file of files) {

        // Check if the file is a ROM or screenshot
        if (file.originalname.endsWith('.gb')) {
          hasRomFile = true;
          // Save ROM at the root of the game folder
          const romPath = path.join(gameFolderPath, file.originalname);
          await fs.writeFile(romPath, file.buffer);
        } else if (file.originalname.match(/^cover\.(png|jpg|jpeg|webp)$/i)) {
          // Save cover at the root of the game folder
          const coverPath = path.join(gameFolderPath, file.originalname);
          await fs.writeFile(coverPath, file.buffer);
        } else if (
          file.originalname.match(/^screenshot\d+\.(png|jpg|jpeg|webp)$/i)
        ) {
          // Save screenshots in the `screenshots` folder
          const screenshotPath = path.join(
            gameFolderPath,
            'screenshots',
            file.originalname,
          );
          await fs.writeFile(screenshotPath, file.buffer);
        } else {
          console.warn(`Unrecognized file type for: ${file.originalname}`);
        }
      }

      if (!hasRomFile) {
        throw new InternalServerErrorException('No ROM file found in upload');
      }

      // Create database entry using original create function
      createdGame = await this.create(createGameDto);
      return createdGame;
    } catch (error) {
      // Clean up files if they were created
      await fs.rm(gameFolderPath, { recursive: true, force: true });

      // If database entry was created, delete it
      if (createdGame) {
        await this.gameModel.findByIdAndDelete(createdGame._id);
      }

      if (
        error instanceof ConflictException ||
        error instanceof InternalServerErrorException
      ) {
        throw error;
      }

      this.logger.error(`Failed to create game ${createGameDto.slug}:`, error);
      throw new InternalServerErrorException('Failed to create game');
    }
  }

  async getGamesList(
    page = 1,
    limit = 10,
    searchTerm = '',
    sortBy:
      | 'recent_desc'
      | 'recent_asc'
      | 'name_asc'
      | 'name_desc' = 'recent_desc',
  ): Promise<{
    games: GameListItem[];
    total: number;
    page: number;
    totalPages: number;
  }> {
    this.logger.log('Getting games list');
    try {
      const sanitizedLimit = Math.min(Math.max(1, limit), 50);

      // Prepare database query
      const dbQuery = searchTerm
        ? {
            $or: [
              { name: { $regex: searchTerm, $options: 'i' } },
              { originalTitle: { $regex: searchTerm, $options: 'i' } },
              { genres: { $regex: searchTerm, $options: 'i' } },
            ],
          }
        : {};

      // Determine sort order based on the new sortBy options
      let sortOptions = {};
      switch (sortBy) {
        case 'recent_desc':
          sortOptions = { _id: -1 }; // Newest first
          break;
        case 'recent_asc':
          sortOptions = { _id: 1 }; // Oldest first
          break;
        case 'name_asc':
          sortOptions = { name: 1 }; // A to Z
          break;
        case 'name_desc':
          sortOptions = { name: -1 }; // Z to A
          break;
        default:
          sortOptions = { _id: -1 }; // Default to newest first
      }

      // Fetch games from database
      const totalGames = await this.gameModel.countDocuments(dbQuery);
      const games = await this.gameModel
        .find(dbQuery)
        .select('name slug language')
        .sort(sortOptions)
        .skip((page - 1) * sanitizedLimit)
        .limit(sanitizedLimit);

      const listedGames: GameListItem[] = [];
      for (const game of games) {
        try {
          const gameFolderPath = path.join(this.gamesPath, game.slug);
          const files = await fs.readdir(gameFolderPath);

          // Find cover
          const coverFile = files.find((file) =>
            file.match(/^cover\.(png|jpg|jpeg|webp)$/i),
          );
          const coverPath = coverFile
            ? `api/games/${game.slug}/${coverFile}`
            : undefined;

          listedGames.push({
            slug: game.slug,
            name: game.name,
            coverPath,
            language: game.language,
          });
        } catch (folderErr) {
          // If folder doesn't exist, add game without cover
          listedGames.push({
            slug: game.slug,
            name: game.name,
            language: game.language,
          });
          this.logger.warn(`Folder not found for game: ${game.slug}`);
        }
      }

      return {
        games: listedGames,
        total: totalGames,
        page,
        totalPages: Math.ceil(totalGames / sanitizedLimit),
      };
    } catch (err) {
      this.logger.error('Error fetching games list:', err);
      return {
        games: [],
        total: 0,
        page: 1,
        totalPages: 0,
      };
    }
  }

  async getGameDetails(slug: string): Promise<GameDetails> {
    try {
      const game = await this.gameModel.findOne({ slug });
      if (!game) {
        throw new NotFoundException(`Game with slug ${slug} not found`);
      }

      const gameFolderPath = path.join(this.gamesPath, slug);
      const files = await fs.readdir(gameFolderPath);

      // Find cover
      const coverFile = files.find((file) =>
        file.match(/^cover\.(png|jpg|jpeg|webp)$/i),
      );
      const coverPath = coverFile
        ? `api/games/${slug}/${coverFile}`
        : undefined;

      // Find ROM
      const romFile = files.find((file) => file.endsWith('.gb'));
      if (!romFile) {
        throw new NotFoundException(`ROM file not found for game ${slug}`);
      }

      // Find screenshots
      let screenshotPaths: string[] = [];
      try {
        const screenshotDir = path.join(gameFolderPath, 'screenshots');
        const screenshotFiles = await fs.readdir(screenshotDir);
        screenshotPaths = screenshotFiles.map(
          (file) => `api/games/${slug}/screenshots/${file}`,
        );
      } catch (screenshotErr) {
        this.logger.warn(`No screenshots found for game ${slug}`);
      }

      // Transform to frontend type
      const gameDetails: GameDetails = {
        slug: game.slug,
        name: game.name,
        language: game.language,
        coverPath,
        rom: {
          type: 'url',
          path: romFile ? `api/games/${slug}/${romFile}` : undefined,
        },
        saveState: {
          type: 'url',
          path: undefined,
        },
        screenshotPaths,
        description: game.description,
        originalTitle: game.originalTitle,
        rating: game.rating,
        releaseDate: game.releaseDate,
        developers: game.developers,
        genres: game.genres,
      };

      return gameDetails;
    } catch (err) {
      this.logger.error(`Error fetching game details for ${slug}:`, err);
      throw new NotFoundException(
        `Could not retrieve game details for ${slug}`,
      );
    }
  }

  async getGamesByIds(gameIds: string[]): Promise<GameListItem[]> {
    try {
      const games = await this.gameModel
        .find({
          _id: { $in: gameIds },
        })
        .select('name slug language'); 

      const processedGames: GameListItem[] = [];

      for (const game of games) {
        try {
          const gameFolderPath = path.join(this.gamesPath, game.slug);
          const files = await fs.readdir(gameFolderPath);

          // Find cover - only processing cover file now since that's all we need
          const coverFile = files.find((file) =>
            file.match(/^cover\.(png|jpg|jpeg)$/i),
          );
          const coverPath = coverFile
            ? `api/games/${game.slug}/${coverFile}`
            : undefined;

          const gameDetails: GameListItem = {
            name: game.name,
            slug: game.slug,
            language: game.language,
            coverPath: coverPath || '',
          };

          processedGames.push(gameDetails);
        } catch (folderErr) {
          this.logger.warn(`Folder not found for game: ${game.slug}`);
        }
      }

      return processedGames;
    } catch (err) {
      this.logger.error('Error fetching games by IDs:', err);
      return [];
    }
  }

  async deleteGame(slug: string): Promise<{ message: string }> {
    try {
      const game = await this.gameModel.findOne({ slug });
      if (!game) {
        throw new NotFoundException(`Game with slug ${slug} not found`);
      }

      // Delete from database
      await this.gameModel.deleteOne({ slug });

      // Delete files from filesystem
      try {
        const gameFolderPath = path.join(this.gamesPath, slug);
        await fs.rm(gameFolderPath, { recursive: true, force: true });
      } catch (fsErr) {
        this.logger.warn(
          `Failed to delete game files for ${slug}: ${fsErr.message}`,
        );
      }

      return { message: `Game ${slug} successfully deleted` };
    } catch (err) {
      this.logger.error(`Error deleting game ${slug}:`, err);
      throw err;
    }
  }

  async deleteAllGames(): Promise<{ message: string; count: number }> {
    try {
      // Delete all games from database
      const result = await this.gameModel.deleteMany({});

      // Delete all game files
      try {
        const files = await fs.readdir(this.gamesPath);
        for (const file of files) {
          const filePath = path.join(this.gamesPath, file);
          const stats = await fs.stat(filePath);
          if (stats.isDirectory()) {
            await fs.rm(filePath, { recursive: true, force: true });
          }
        }
      } catch (fsErr) {
        this.logger.warn(`Failed to delete game files: ${fsErr.message}`);
      }

      return {
        message: 'All games successfully deleted',
        count: result.deletedCount,
      };
    } catch (err) {
      this.logger.error('Error deleting all games:', err);
      throw new Error('Failed to delete all games');
    }
  }
}
