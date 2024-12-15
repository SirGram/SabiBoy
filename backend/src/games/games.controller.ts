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
import { CreateGameDto } from './dto/create-game.dto';



@Controller('games')
export class GamesController {
  constructor(private readonly gamesService: GamesService) {}

  private readonly logger = new Logger(GamesController.name);

  @Get()
  async getGamesList(
    @Query('page') page = 1,
    @Query('limit') limit = 10,
    @Query('search') search = ''
  ) {
    return this.gamesService.getGamesList(page, limit, search);
  }

  @Get(':slug')
  async getGameDetails(@Param('slug') slug: string) {
    return this.gamesService.getGameDetails(slug);
  }

  @Post()
  create(@Body() createGameDto: CreateGameDto) {
    return this.gamesService.create(createGameDto);
  }
}
