import {
  ConflictException,
  Injectable,
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
  coverPath?: string;
}
export interface GameDetails extends Game {
  coverPath?: string;
  romPath?: string;
  screenshotPaths?: string[];
}
@Injectable()
export class GamesService {
  private readonly logger = new Logger(GamesService.name);

  private readonly gamesPath = path.join(process.cwd(), '..', 'games');

  constructor(@InjectModel(Game.name) private gameModel: Model<Game>) {}

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

  async getGamesList(
    page = 1,
    limit = 10,
    searchTerm = '',
  ): Promise<{
    games: GameListItem[];
    total: number;
    page: number;
    totalPages: number;
  }> {
    this.logger.log('Getting all games');
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

      // Fetch games from database
      const totalGames = await this.gameModel.countDocuments(dbQuery);
      const games = await this.gameModel
        .find(dbQuery)
        .select('name slug') // Only select necessary fields
        .skip((page - 1) * sanitizedLimit)
        .limit(sanitizedLimit);
      const listedGames: GameListItem[] = [];
      for (const game of games) {
        try {
          const gameFolderPath = path.join(this.gamesPath, game.slug);
          const files = await fs.readdir(gameFolderPath);

          // Find cover
          const coverFile = files.find((file) =>
            file.match(/^cover\.(png|jpg|jpeg)$/i),
          );
          const coverPath = coverFile
            ? `api/games/${game.slug}/${coverFile}`
            : undefined;

          listedGames.push({
            slug: game.slug,
            name: game.name,
            coverPath,
          });
        } catch (folderErr) {
          // If folder doesn't exist, add game without cover
          listedGames.push({
            slug: game.slug,
            name: game.name,
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
      // Fetch game from database
      const game = await this.gameModel.findOne({ slug });
      if (!game) {
        throw new NotFoundException(`Game with slug ${slug} not found`);
      }

      // Convert to plain object to add additional properties
      const gameDetails = game.toObject() as GameDetails;

      // Check game folder
      const gameFolderPath = path.join(this.gamesPath, slug);
      const files = await fs.readdir(gameFolderPath);

      // Find cover
      const coverFile = files.find((file) =>
        file.match(/^cover\.(png|jpg|jpeg)$/i),
      );
      gameDetails.coverPath = coverFile
        ? `api/games/${slug}/${coverFile}`
        : undefined;

      // Find ROM
      const romFile = files.find((file) => file.endsWith('.gb'));
      gameDetails.romPath = romFile
        ? `api/games/${slug}/${romFile}`
        : undefined;

      // Find screenshots
      const screenshotDir = path.join(gameFolderPath, 'screenshots');
      try {
        const screenshotFiles = await fs.readdir(screenshotDir);
        gameDetails.screenshotPaths = screenshotFiles.map(
          (file) => `api/games/${slug}/screenshots/${file}`,
        );
      } catch (screenshotErr) {
        // No screenshots folder found
        gameDetails.screenshotPaths = [];
      }

      return gameDetails;
    } catch (err) {
      this.logger.error(`Error fetching game details for ${slug}:`, err);
      throw new NotFoundException(
        `Could not retrieve game details for ${slug}`,
      );
    }
  }
  async getGamesByIds(gameIds: string[]): Promise<GameDetails[]> {
    try {
      const games = await this.gameModel.find({
        _id: { $in: gameIds },
      });

      const processedGames: GameDetails[] = [];

      for (const game of games) {
        try {
          const gameDetails = game.toObject() as GameDetails;
          const gameFolderPath = path.join(this.gamesPath, game.slug);
          const files = await fs.readdir(gameFolderPath);

          const coverFile = files.find((file) =>
            file.match(/^cover\.(png|jpg|jpeg)$/i),
          );
          gameDetails.coverPath = coverFile
            ? `api/games/${game.slug}/${coverFile}`
            : undefined;

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
}
