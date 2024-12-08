import {
  Controller,
  Get,
  Post,
  Body,
  Patch,
  Param,
  Delete,
  Logger,
  Query,
} from '@nestjs/common';
import { GamesService } from './games.service';
import path from 'path';

export interface Game {
  id: string;
  name: string;
  coverPath?: string;
  romPath?: string;
}

@Controller('games')
export class GamesController {
  constructor(private readonly gamesService: GamesService) {}

  private readonly logger = new Logger(GamesController.name);

  @Get()
  async getGames(@Query('page') page = 1, @Query('limit') limit = 10, @Query("search")search="") {
    return this.gamesService.getGamesByPage(Number(page), Number(limit), search);
  }
}
