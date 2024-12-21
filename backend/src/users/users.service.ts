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
import { GameDetails, GamesService } from 'src/games/games.service';

@Injectable()
export class UsersService {
  constructor(
    @InjectModel(User.name)
    private userModel: Model<UserDocument>,
    private readonly authService: AuthService,
    private readonly gamesService: GamesService,
    @InjectModel(Game.name)
    private gameModel: Model<GameDocument>,
  ) {}

  async createFirstUserIfNoneExist(): Promise<void> {
    const existingUser = await this.userModel.findOne();

    if (!existingUser) {
      const createUserDto: CreateUserDto = {
        email: 'admin@example.com',
        password: 'supersecretpassword',
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
      // only delete if no superuser
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
          { $push: { library: { game: game._id, savedState: false } } },
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
  async getUserLibrary(userId: string): Promise<GameDetails[]> {
    try {
      const user = await this.userModel.findById(userId).select('library');

      if (!user) {
        throw new NotFoundException('User not found');
      }

      const gameIds = user.library.map((libraryItem) =>
        libraryItem.game.toString(),
      );

      // Fetch games by IDs
      const libraryGames = await this.gamesService.getGamesByIds(gameIds);

      // Map saved states to games
      /*  return libraryGames.map((game) => {
        const libraryItem = user.library.find(
          (item) => item.game.toString() === game._id.toString(),
        );

        return {
          ...game,
          savedState: libraryItem?.savedState || false,
        };
      }); */
      return libraryGames;
    } catch (error) {
      throw new InternalServerErrorException('Could not retrieve user library');
    }
  }
  async isGameInUserLibrary(
    userId: string,
    gameSlug: string,
  ): Promise<boolean> {
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

      return user.library.some((libraryItem) =>
        libraryItem.game._id.equals(game._id),
      );
    } catch (error) {
      console.error('Error checking game in library:', error);
      throw new InternalServerErrorException(
        'Could not check game library status',
      );
    }
  }
  async changePassword(
    userId: string,
    currentPassword: string,
    newPassword: string,
  ): Promise<void> {
    // Find the user with the password field included
    const user = await this.userModel.findById(userId).select('+password');

    if (!user) {
      throw new NotFoundException('User not found');
    }

    // Verify current password
    const isPasswordValid = await this.authService.comparePasswords(
      currentPassword,
      user.password,
    );

    if (!isPasswordValid) {
      throw new ForbiddenException('Current password is incorrect');
    }

    // Update password
    user.password = newPassword;
    await user.save();
  }
}
