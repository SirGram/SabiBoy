import { Injectable, Logger } from '@nestjs/common';
import { CreateGameDto } from './dto/create-game.dto';
import { UpdateGameDto } from './dto/update-game.dto';
import * as path from 'path';
import { Game } from './games.controller';

import * as fs from 'fs/promises';

@Injectable()
export class GamesService {
  private readonly logger = new Logger(GamesService.name);

  private readonly gamesPath = path.join(process.cwd(), '..', 'test', 'games');

  async getAllGames(): Promise<Game[]> {
    this.logger.log('Getting all games');
    try {
      const games: Game[] = [];
      const gameFolders = await fs.readdir(this.gamesPath);
      for (const gameFolder of gameFolders) {
        const gameFolderPath = path.join(this.gamesPath, gameFolder);
        try {
          const files = await fs.readdir(gameFolderPath);
          const coverFile = files.find(
            (file) => file.endsWith('.png') || file.endsWith('.jpg'),
          );
          const coverPath = coverFile
            ? `api/games/${gameFolder}/${coverFile}`
            : undefined;
            const romFile = files.find(
            (file) => file.endsWith('.gb'),
          );
          const romPath = romFile
            ? `api/games/${gameFolder}/${romFile}`
            : undefined;

          games.push({
            id: gameFolder,
            name: gameFolder,
            coverPath: coverPath,
          });
        } catch (err) {
          this.logger.error('Error reading game folder:', err);
        }
      }

      return games;
    } catch (err) {
      this.logger.error('Error reading games directory:', err);
      return [];
    }
  }
}
