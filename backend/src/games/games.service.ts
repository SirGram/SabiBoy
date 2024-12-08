import { Injectable, Logger } from '@nestjs/common';
import { CreateGameDto } from './dto/create-game.dto';
import { UpdateGameDto } from './dto/update-game.dto';
import * as path from 'path';
import { Game } from './games.controller';

import * as fs from 'fs/promises';

@Injectable()
export class GamesService {
  private readonly logger = new Logger(GamesService.name);

  private readonly gamesPath = path.join(process.cwd(), '..', 'games');

  async getGamesByPage(
    page = 1,
    limit = 10,
    searchTerm = '',
  ): Promise<{
    games: Game[];
    total: number;
    page: number;
    totalPages: number;
  }> {
    this.logger.log('Getting all games');
    try {
      const sanitizedLimit = Math.min(Math.max(1, limit), 50);
      const normalizedSearchTerm = searchTerm.toLowerCase().replace(/\s+/g, '');
      let allGameFolders = await fs.readdir(this.gamesPath);
      if (searchTerm) {
        allGameFolders = allGameFolders.filter((folder) =>
          folder
            .toLowerCase()
            .replace(/\s+/g, '')
            .includes(normalizedSearchTerm),
        );
      }
      const startIndex = (page - 1) * sanitizedLimit;
      const endIndex = startIndex + sanitizedLimit;
      const paginatedGameFolders = allGameFolders.slice(startIndex, endIndex);
      const games: Game[] = [];
      for (const gameFolder of paginatedGameFolders) {
        const gameFolderPath = path.join(this.gamesPath, gameFolder);
        try {
          const files = await fs.readdir(gameFolderPath);
          const coverFile = files.find(
            (file) => file.endsWith('.png') || file.endsWith('.jpg'),
          );
          const coverPath = coverFile
            ? `api/games/${gameFolder}/${coverFile}`
            : undefined;
          const romFile = files.find((file) => file.endsWith('.gb'));
          const romPath = romFile
            ? `api/games/${gameFolder}/${romFile}`
            : undefined;

          games.push({
            id: gameFolder,
            name: gameFolder,
            coverPath: coverPath,
            romPath: romPath,
          });
        } catch (err) {
          this.logger.error('Error reading game folder:', err);
        }
      }

      return {
        games,
        total: allGameFolders.length,
        page,
        totalPages: Math.ceil(allGameFolders.length / sanitizedLimit),
      };
    } catch (err) {
      this.logger.error('Error reading games directory:', err);
      return {
        games: [],
        total: 0,
        page: 1,
        totalPages: 0,
      };
    }
  }
}
