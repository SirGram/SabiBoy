import {
  Controller,
  Get,
  Post,
  Body,
  Param,
  Patch,
  Delete,
  UseGuards,
  Query,
  Req,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { UsersService } from './users.service';
import { CreateUserDto } from '../auth/dto/create_user.dto';
import { User, UserRole } from '../schemas/user.schema';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';
import { RolesGuard } from '../auth/guards/roles.guard';
import { Roles } from '../decorators/roles.decorator';
import { Request } from 'express';

@Controller('users')
@UseGuards(JwtAuthGuard, RolesGuard)
export class UsersController {
  constructor(private readonly usersService: UsersService) {}

  @Post()
  @Roles(UserRole.SUPERUSER)
  async createUser(
    @Body() createUserDto: CreateUserDto,
  ): Promise<Partial<User>> {
    return this.usersService.createUser(createUserDto);
  }

  @Get()
  @Roles(UserRole.SUPERUSER)
  async findAllUsers(@Query('role') role?: UserRole): Promise<User[]> {
    return this.usersService.findAll(role);
  }

  @Get(':id')
  @Roles(UserRole.SUPERUSER)
  async findUserById(@Param('id') id: string): Promise<User | null> {
    return this.usersService.findUserById(id);
  }

  @Patch(':id/role')
  @Roles(UserRole.SUPERUSER)
  async updateUserRole(
    @Param('id') id: string,
    @Body('role') role: UserRole,
  ): Promise<User | null> {
    return this.usersService.updateUserRole(id, role);
  }

  @Delete(':id')
  @Roles(UserRole.SUPERUSER)
  async deleteUser(@Param('id') id: string, @Req() req: Request) {
    const user = req.user;

    return this.usersService.deleteUser(user.id,id);
  }

  @Patch(':id')
  @Roles(UserRole.SUPERUSER)
  async updateUser(
    @Param('id') id: string,
    @Body() updateData: Partial<CreateUserDto>,
  ): Promise<User | null> {
    return this.usersService.updateUser(id, updateData);
  }

  @Post(':id/library')
  async addGameToUserLibrary(
    @Param('id') id: string,
    @Body('slug') slug: string,
    @Req() req: Request,
  ): Promise<User | null> {
    const user = req.user;
    console.log(user, slug);
    return this.usersService.addGameToUserLibrary(id, slug, user.id);
  }
  @Get(':id/library')
  async getUserLibrary(@Param('id') id: string, @Req() req: Request) {
    const user = req.user;

    if (user.id !== id) {
      throw new ForbiddenException('You can only access your own library');
    }

    return this.usersService.getUserLibrary(id);
  }
  @Delete(':id/library/')
  async deleteGameFromLibrary(
    @Param('id') id: string,
    @Body('slug') slug: string,
    @Req() req: Request,
  ) {
    const user = req.user;

    if (user.id !== id) {
      throw new ForbiddenException('You can only access your own library');
    }

    return this.usersService.deleteGameFromUserLibrary(id, slug, user.id);
  }
  @Get(':id/library/check')
  async checkGameInLibrary(
    @Param('id') id: string,
    @Query('slug') slug: string,
    @Req() req: Request,
  ) {
    const user = req.user;

    if (user.id !== id) {
      throw new ForbiddenException('You can only check your own library');
    }

    return {
      inLibrary: await this.usersService.isGameInUserLibrary(id, slug),
    };
  }
  @Patch('change-password')
  async changePassword(
    @Req() req: Request,
    @Body() body: { currentPassword: string; newPassword: string },
  ) {
    const user = req.user;

    // Basic password strength validation
    if (body.newPassword.length < 6) {
      throw new BadRequestException(
        'Password must be at least 6 characters long',
      );
    }

    await this.usersService.changePassword(
      user.id,
      body.currentPassword,
      body.newPassword,
    );

    return { message: 'Password changed successfully' };
  }

  @Patch(':id')
  async updateUserProfile(
    @Param('id') id: string,
    @Req() req: Request,
    @Body() updateData: Partial<CreateUserDto>,
  ) {
    const user = req.user;

    // Ensure user can only update their own profile
    if (user.id !== id && user.role !== UserRole.SUPERUSER) {
      throw new ForbiddenException(
        'You are not authorized to update this profile',
      );
    }

    // Remove password from update data if present
    const { password, ...safeUpdateData } = updateData;

    return this.usersService.updateUser(id, safeUpdateData);
  }
}
