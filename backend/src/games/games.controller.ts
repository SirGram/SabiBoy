import { Controller, Get, Post, Body, Patch, Param, Delete, Logger } from '@nestjs/common';
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
  async getAllGames() : Promise<Game[]>{
    try{

      return await this.gamesService.getAllGames();
    }catch(err){
      this.logger.error('Failed to get games:', err);
      return [];
   
    }
  }
  /* @Get(':gameName/rom')
  async getRom() : {

  } */

}
