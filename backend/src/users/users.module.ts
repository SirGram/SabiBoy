import { Module } from '@nestjs/common';
import { MongooseModule } from '@nestjs/mongoose';
import { UsersService } from './users.service';
import { UsersController } from './users.controller';
import { User, UserSchema } from '../schemas/user.schema';
import { AuthModule } from 'src/auth/auth.module';
import { Game, GameSchema } from 'src/schemas/game.shema';
import { GamesService } from 'src/games/games.service';

@Module({
  imports: [
    MongooseModule.forFeature([
      { name: User.name, schema: UserSchema },
      {
        name: Game.name,
        schema: GameSchema,
      },
    ]),
    AuthModule,
  ],
  providers: [UsersService, GamesService],
  controllers: [UsersController],
})
export class UsersModule {}
