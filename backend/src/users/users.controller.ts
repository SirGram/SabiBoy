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
  Res,
  NotFoundException,
} from '@nestjs/common';
import { UsersService } from './users.service';
import { CreateUserDto } from '../auth/dto/create_user.dto';
import { User, UserRole } from '../schemas/user.schema';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';
import { RolesGuard } from '../auth/guards/roles.guard';
import { Roles } from '../decorators/roles.decorator';
import { Request } from 'express';
import { Response } from 'express';

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
    return this.usersService.deleteUser(user.id, id);
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
    if (user.id !== id && user.role !== UserRole.SUPERUSER) {
      throw new ForbiddenException(
        'You are not authorized to update this profile',
      );
    }
    const { password, ...safeUpdateData } = updateData;
    return this.usersService.updateUser(id, safeUpdateData);
  }

  @Patch(':id/library/:slug/save-state')
  async updateGameSaveState(
    @Param('id') id: string,
    @Param('slug') slug: string,
    @Req() req: Request,
  ) {
    const user = req.user;
    if (user.id !== id) {
      throw new ForbiddenException('You can only update your own save states');
    }

    if (!req.body) {
      throw new BadRequestException('No save state data provided');
    }

    // Express raw body parser will provide the binary data as a Buffer
    const saveState = req.body;

    return this.usersService.updateGameSaveState(id, slug, saveState);
  }

  @Get(':id/library/:slug/save-state')
  async getGameSaveState(
    @Param('id') id: string,
    @Param('slug') slug: string,
    @Req() req: Request,
    @Res() res: Response,
  ) {
    const user = req.user;
    if (user.id !== id) {
      throw new ForbiddenException('You can only access your own save states');
    }
    const saveState = await this.usersService.getGameSaveState(id, slug);
    if (!saveState) {
      throw new NotFoundException('No save state found');
    }

    res.set('Content-Type', 'application/octet-stream');
    return res.send(saveState);
  }

  @Delete(':id/library/:slug/save-state')
  async resetGameSaveState(
    @Param('id') id: string,
    @Param('slug') slug: string,
    @Req() req: Request,
  ) {
    const user = req.user;
    if (user.id !== id) {
      throw new ForbiddenException('You can only reset your own save states');
    }
    return this.usersService.resetGameSaveState(id, slug);
  }

  @Patch(':id/library/:slug/visibility')
  async updateGameVisibility(
    @Param('id') id: string,
    @Param('slug') slug: string,
    @Body('showInMainboard') showInMainboard: boolean,
    @Req() req: Request,
  ) {
    const user = req.user;
    if (user.id !== id) {
      throw new ForbiddenException('You can only update your own library');
    }
    return this.usersService.updateGameVisibility(id, slug, showInMainboard);
  }
}
