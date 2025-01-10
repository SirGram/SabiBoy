import {
  Controller,
  Get,
  Post,
  Body,
  Param,
  Logger,
  Query,
  Delete,
  UseGuards,
  UploadedFiles,
  UseInterceptors,
} from '@nestjs/common';
import { GamesService } from './games.service';
import { CreateGameDto } from './dto/create-game.dto';
import { Roles } from 'src/decorators/roles.decorator';
import { JwtAuthGuard } from 'src/auth/guards/jwt-auth.guard';
import { RolesGuard } from 'src/auth/guards/roles.guard';
import { UserRole } from 'src/schemas/user.schema';
import { FilesInterceptor } from '@nestjs/platform-express';

@Controller('games')
export class GamesController {
  constructor(private readonly gamesService: GamesService) {}

  private readonly logger = new Logger(GamesController.name);

  @Get()
  async getGamesList(
    @Query('page') page = 1,
    @Query('limit') limit = 10,
    @Query('search') search = '',
    @Query('sortBy')
    sortBy:
      | 'recent_desc'
      | 'recent_asc'
      | 'name_asc'
      | 'name_desc' = 'recent_desc',
  ) {
    return this.gamesService.getGamesList(+page, +limit, search, sortBy);
  }

  @Get(':slug')
  async getGameDetails(@Param('slug') slug: string) {
    return this.gamesService.getGameDetails(slug);
  }

  @Post()
  @UseGuards(JwtAuthGuard, RolesGuard)
  @Roles(UserRole.SUPERUSER)
  create(@Body() createGameDto: CreateGameDto) {
    return this.gamesService.create(createGameDto);
  }

  @Delete()
  @UseGuards(JwtAuthGuard, RolesGuard)
  @Roles(UserRole.SUPERUSER)
  async deleteAllGames() {
    return this.gamesService.deleteAllGames();
  }

  @Delete(':slug')
  @UseGuards(JwtAuthGuard, RolesGuard)
  @Roles(UserRole.SUPERUSER)
  async deleteGame(@Param('slug') slug: string) {
    return this.gamesService.deleteGame(slug);
  }

  @Post('upload')
  @UseGuards(JwtAuthGuard, RolesGuard)
  @Roles(UserRole.SUPERUSER)
  @UseInterceptors(FilesInterceptor('files'))
  async createGameWithFiles(
    @Body() createGameDto: CreateGameDto,
    @UploadedFiles() files: Express.Multer.File[],
  ) {
    return this.gamesService.createGameWithFiles(createGameDto, files);
  }
}
