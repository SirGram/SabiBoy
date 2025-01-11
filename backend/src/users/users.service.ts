import {
  BadRequestException,
  Injectable,
  NotFoundException,
  ForbiddenException,
  InternalServerErrorException,
} from '@nestjs/common';
import { InjectModel } from '@nestjs/mongoose';
import { Model } from 'mongoose';
import { User, UserDocument, UserRole } from '../schemas/user.schema';
import { AuthService } from '../auth/auth.service';
import { CreateUserDto } from '../auth/dto/create_user.dto';
import { Game, GameDocument } from 'src/schemas/game.shema';
import { GameDetails, GameListItem, GamesService } from 'src/games/games.service';
import { ConfigService } from '@nestjs/config';

@Injectable()
export class UsersService {
  constructor(
    @InjectModel(User.name)
    private userModel: Model<UserDocument>,
    private readonly authService: AuthService,
    private readonly gamesService: GamesService,
    @InjectModel(Game.name)
    private gameModel: Model<GameDocument>,

    private readonly configService: ConfigService,
  ) {}

  async createFirstUserIfNoneExist(): Promise<void> {
    const existingUser = await this.userModel.findOne();
    if (!existingUser) {
      const email = this.configService.get<string>('INITIAL_USER_MAIL');
      const password = this.configService.get<string>('INITIAL_USER_PASSWORD');
      
      const createUserDto: CreateUserDto = {
        email: email,
        password: password,
        role: UserRole.SUPERUSER,
      };
      const user = new this.userModel(createUserDto);
      await user.save();
      console.log('First superuser created');
    }
  }

  async createUser(createUserDto: CreateUserDto): Promise<Partial<User>> {
    return this.authService.register(createUserDto);
  }

  async findAll(role?: UserRole): Promise<User[]> {
    const filter = role ? { role } : {};
    return this.userModel.find(filter).select('-password');
  }

  async findUserById(id: string): Promise<User | null> {
    try {
      const user = await this.userModel.findById(id).select('-password');
      if (!user) {
        throw new NotFoundException('User not found');
      }
      return user;
    } catch (error) {
      if (error.name === 'CastError') {
        throw new BadRequestException('Invalid user ID');
      }
      throw error;
    }
  }

  async findByEmail(email: string): Promise<User | null> {
    return this.userModel.findOne({ email }).select('+password');
  }

  async updateUserRole(id: string, role: UserRole): Promise<User | null> {
    if (!Object.values(UserRole).includes(role)) {
      throw new BadRequestException('Invalid user role');
    }
    try {
      const updatedUser = await this.userModel
        .findByIdAndUpdate(id, { role }, { new: true, runValidators: true })
        .select('-password');
      if (!updatedUser) {
        throw new NotFoundException('User not found');
      }
      return updatedUser;
    } catch (error) {
      if (error.name === 'CastError') {
        throw new BadRequestException('Invalid user ID');
      }
      throw error;
    }
  }

  async deleteUser(userId: string, id: string): Promise<void> {
    try {
      const result = await this.userModel.findById(id).select('-password');
      if (result.role === 'superuser') {
        throw new BadRequestException('Cannot delete superuser');
      }
      if (!result) {
        throw new NotFoundException('User not found');
      }
    } catch (error) {
      if (error.name === 'CastError') {
        throw new BadRequestException('Invalid user ID');
      }
      throw error;
    }
  }

  async updateUser(
    id: string,
    updateData: Partial<CreateUserDto>,
  ): Promise<User | null> {
    try {
      const { password, ...updateFields } = updateData;
      const updatedUser = await this.userModel
        .findByIdAndUpdate(id, updateFields, { new: true, runValidators: true })
        .select('-password');
      if (!updatedUser) {
        throw new NotFoundException('User not found');
      }
      return updatedUser;
    } catch (error) {
      if (error.name === 'CastError') {
        throw new BadRequestException('Invalid user ID');
      }
      throw error;
    }
  }

  async addGameToUserLibrary(
    id: string,
    slug: string,
    userId: string,
  ): Promise<User | null> {
    if (userId !== id) {
      throw new ForbiddenException(
        'You are not authorized to modify this user',
      );
    }
    try {
      const game = await this.gameModel.findOne({ slug });
      if (!game) throw new NotFoundException('Game not found');

      const updatedUser = await this.userModel
        .findByIdAndUpdate(
          id,
          {
            $push: {
              library: {
                game: game._id,
                showInMainboard: true,
                saveState: null,
              },
            },
          },
          { new: true },
        )
        .populate('library.game');

      if (!updatedUser) throw new NotFoundException('User not found');
      return updatedUser;
    } catch (error) {
      if (error.name === 'CastError') {
        throw new BadRequestException('Invalid user ID');
      }
      throw error;
    }
  }

  async deleteGameFromUserLibrary(
    userId: string,
    gameSlug: string,
    requestingUserId: string,
  ) {
    try {
      if (userId !== requestingUserId) {
        throw new ForbiddenException('You can only modify your own library');
      }

      const game = await this.gameModel.findOne({ slug: gameSlug });
      if (!game) {
        throw new NotFoundException('Game not found');
      }

      const updatedUser = await this.userModel
        .findByIdAndUpdate(
          userId,
          { $pull: { library: { game: game._id } } },
          { new: true },
        )
        .populate('library.game');

      if (!updatedUser) {
        throw new NotFoundException('User not found');
      }
      return updatedUser;
    } catch (error) {
      console.error('Error removing game from library:', error);
      throw new InternalServerErrorException(
        'Failed to remove game from library',
      );
    }
  }

  async getUserLibrary(userId: string): Promise<GameListItem[]> {
    try {
      const user = await this.userModel.findById(userId).select('library');

      if (!user) {
        throw new NotFoundException('User not found');
      }

      const gameIds = user.library
        .filter((item) => item.showInMainboard)
        .map((item) => item.game.toString());
      console.log('gameIds', gameIds);

      return this.gamesService.getGamesByIds(gameIds);
    } catch (error) {
      throw new InternalServerErrorException('Could not retrieve user library');
    }
  }

  async getGameStatus(
    userId: string,
    gameSlug: string,
  ): Promise<{
    hasSaveState: boolean;
    lastPlayed: string | null;
    isInLibrary: boolean;
  }> {
    try {
      const user = await this.userModel
        .findById(userId)
        .populate('library.game');

      if (!user) {
        throw new NotFoundException('User not found');
      }

      const game = await this.gameModel.findOne({ slug: gameSlug });
      if (!game) {
        throw new NotFoundException('Game not found');
      }

      const libraryItem = user.library.find((item) => {
        // Skip items where game reference is null
        if (!item.game) return false;

        return item.game._id.toString() === game._id.toString();
      });

      if (!libraryItem) {
        return {
          hasSaveState: false,
          lastPlayed: null,
          isInLibrary: false,
        };
      }

      return {
        hasSaveState: !!libraryItem.saveState,
        // Handle case where lastAccessed might be undefined
        lastPlayed: libraryItem.lastAccessed
          ? libraryItem.lastAccessed.toISOString()
          : null,
        isInLibrary: true,
      };
    } catch (error) {
      console.error('Error getting game status:', error);
      throw new InternalServerErrorException('Could not get game status');
    }
  }

  async getGameSaveState(userId: string, slug: string): Promise<Buffer | null> {
    try {
      console.log('Getting save state for user:', userId, 'game:', slug);

      const user = await this.userModel.findById(userId);
      if (!user) throw new NotFoundException('User not found');

      const game = await this.gameModel.findOne({ slug });
      if (!game) throw new NotFoundException('Game not found');

      const libraryItem = user.library.find((item) =>
        item.game.equals(game._id),
      );
      if (!libraryItem) {
        throw new NotFoundException('Game not found in user library');
      }

      // Update last accessed time
      await this.updateGameLastAccessed(userId, slug);

      console.log('Found save state size:', libraryItem.saveState?.length);
      return libraryItem.saveState;
    } catch (error) {
      console.error('Error retrieving save state:', error);
      throw error;
    }
  }

  async updateGameLastAccessed(
    userId: string,
    slug: string,
  ): Promise<User | null> {
    try {
      const game = await this.gameModel.findOne({ slug });
      if (!game) throw new NotFoundException('Game not found');

      const updatedUser = await this.userModel
        .findOneAndUpdate(
          {
            _id: userId,
            'library.game': game._id,
          },
          {
            $set: {
              'library.$.lastAccessed': new Date(),
            },
          },
          { new: true },
        )
        .populate('library.game');

      if (!updatedUser) throw new NotFoundException('User or game not found');

      return updatedUser;
    } catch (error) {
      if (error.name === 'CastError') {
        throw new BadRequestException('Invalid ID format');
      }
      throw new InternalServerErrorException(
        'Failed to update last accessed time',
      );
    }
  }

  async updateGameSaveState(
    userId: string,
    slug: string,
    saveState: Buffer,
  ): Promise<User | null> {
    try {
      console.log('Received save state size:', saveState.length);

      if (!Buffer.isBuffer(saveState)) {
        console.error('Save state is not a buffer:', typeof saveState);
        throw new BadRequestException('Save state must be a buffer');
      }

      const user = await this.userModel.findById(userId);
      if (!user) throw new NotFoundException('User not found');

      const game = await this.gameModel.findOne({ slug });
      if (!game) throw new NotFoundException('Game not found');

      const libraryItem = user.library.find((item) =>
        item.game.equals(game._id),
      );
      if (!libraryItem) {
        throw new NotFoundException('Game not found in user library');
      }

      console.log('Updating save state for user:', userId, 'game:', slug);

      const updatedUser = await this.userModel
        .findOneAndUpdate(
          {
            _id: userId,
            'library.game': game._id,
          },
          {
            $set: {
              'library.$.saveState': saveState,
            },
          },
          { new: true },
        )
        .populate('library.game');

      if (!updatedUser) {
        throw new InternalServerErrorException('Failed to update save state');
      }

      // Verify the save state was stored
      const verifyUser = await this.userModel.findById(userId);
      const verifyItem = verifyUser?.library.find((item) =>
        item.game.equals(game._id),
      );
      console.log('Verified save state size:', verifyItem?.saveState?.length);

      return updatedUser;
    } catch (error) {
      console.error('Save state error:', error);
      if (error.name === 'CastError') {
        throw new BadRequestException('Invalid ID format');
      }
      if (error.status) throw error;
      throw new InternalServerErrorException('Failed to update save state');
    }
  }

  async resetGameSaveState(userId: string, slug: string): Promise<User | null> {
    try {
      console.log('Resetting save state for user:', userId, 'game:', slug);

      const user = await this.userModel.findById(userId);
      if (!user) throw new NotFoundException('User not found');

      const game = await this.gameModel.findOne({ slug });
      if (!game) throw new NotFoundException('Game not found');

      const libraryItem = user.library.find((item) =>
        item.game.equals(game._id),
      );
      if (!libraryItem) {
        throw new NotFoundException('Game not found in user library');
      }

      const updatedUser = await this.userModel
        .findOneAndUpdate(
          {
            _id: userId,
            'library.game': game._id,
          },
          {
            $set: {
              'library.$.saveState': null,
            },
          },
          { new: true },
        )
        .populate('library.game');

      if (!updatedUser) {
        throw new InternalServerErrorException('Failed to reset save state');
      }

      return updatedUser;
    } catch (error) {
      console.error('Reset save state error:', error);
      if (error.name === 'CastError') {
        throw new BadRequestException('Invalid ID format');
      }
      if (error.status) throw error;
      throw new InternalServerErrorException('Failed to reset save state');
    }
  }

  async updateGameVisibility(
    userId: string,
    slug: string,
    showInMainboard: boolean,
  ): Promise<User | null> {
    try {
      const game = await this.gameModel.findOne({ slug });
      if (!game) throw new NotFoundException('Game not found');

      const updatedUser = await this.userModel
        .findOneAndUpdate(
          {
            _id: userId,
            'library.game': game._id,
          },
          {
            $set: {
              'library.$.showInMainboard': showInMainboard,
            },
          },
          { new: true },
        )
        .populate('library.game');

      if (!updatedUser) throw new NotFoundException('User or game not found');

      return updatedUser;
    } catch (error) {
      if (error.name === 'CastError') {
        throw new BadRequestException('Invalid ID format');
      }
      throw new InternalServerErrorException(
        'Failed to update game visibility',
      );
    }
  }

  async changePassword(
    userId: string,
    currentPassword: string,
    newPassword: string,
  ): Promise<void> {
    const user = await this.userModel.findById(userId).select('+password');

    if (!user) {
      throw new NotFoundException('User not found');
    }

    const isPasswordValid = await this.authService.comparePasswords(
      currentPassword,
      user.password,
    );

    if (!isPasswordValid) {
      throw new ForbiddenException('Current password is incorrect');
    }

    user.password = newPassword;
    await user.save();
  }
  async getRecentlyPlayedGames(
    userId: string,
    days: number = 7,
  ): Promise<GameListItem[]> {
    try {
      const oneWeekAgo = new Date();
      oneWeekAgo.setDate(oneWeekAgo.getDate() - days);

      const user = await this.userModel.findById(userId).select('library');

      if (!user) {
        throw new NotFoundException('User not found');
      }

      const recentGameIds = user.library
        .filter((item) => item.lastAccessed && item.lastAccessed >= oneWeekAgo)
        .sort((a, b) => {
          if (!a.lastAccessed || !b.lastAccessed) return 0;
          return b.lastAccessed.getTime() - a.lastAccessed.getTime();
        })
        .map((item) => item.game.toString()); // Convert ObjectId to string

      // Get the game details using the IDs
      return this.gamesService.getGamesByIds(recentGameIds);
    } catch (error) {
      console.error('Error getting recently played games:', error);
      throw new InternalServerErrorException(
        'Could not retrieve recently played games',
      );
    }
  }
}
