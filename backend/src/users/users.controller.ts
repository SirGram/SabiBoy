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
    NotFoundException,
    BadRequestException
  } from '@nestjs/common';
  import { InjectModel } from '@nestjs/mongoose';
  import { Model } from 'mongoose';
  import { AuthService } from '../auth/auth.service';
  import { CreateUserDto } from '../auth/dto/create_user.dto';
  import { User, UserDocument, UserRole } from '../schemas/user.schema';
  import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';
  import { RolesGuard } from '../auth/guards/roles.guard';
  import { Roles } from '../decorators/roles.decorator';
  
  @Controller('users')
  @UseGuards(JwtAuthGuard, RolesGuard)
  export class UsersController {
    constructor(
      private readonly authService: AuthService,
      @InjectModel(User.name) private userModel: Model<UserDocument>
    ) {}
  
    @Post()
    @Roles(UserRole.SUPERUSER)
    async createUser(@Body() createUserDto: CreateUserDto): Promise<Partial<User>> {
      return this.authService.register(createUserDto);
    }
  
    @Get()
    @Roles(UserRole.SUPERUSER)
    async findAllUsers(@Query('role') role?: UserRole): Promise<User[]> {
      try {
        // If role is provided, filter by role
        const filter = role ? { role } : {};
        return await this.userModel.find(filter).select('-password');
      } catch (error) {
        throw new BadRequestException('Failed to retrieve users');
      }
    }
  
    @Get(':id')
    @Roles(UserRole.SUPERUSER)
    async findUserById(@Param('id') id: string): Promise<User | null> {
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
  
    @Patch(':id/role')
    @Roles(UserRole.SUPERUSER)
    async updateUserRole(
      @Param('id') id: string, 
      @Body('role') role: UserRole
    ): Promise<User | null> {
      // Validate role
      if (!Object.values(UserRole).includes(role)) {
        throw new BadRequestException('Invalid user role');
      }
  
      try {
        const updatedUser = await this.userModel.findByIdAndUpdate(
          id, 
          { role }, 
          { new: true, runValidators: true }
        ).select('-password');
  
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
  
    @Delete(':id')
    @Roles(UserRole.SUPERUSER)
    async deleteUser(@Param('id') id: string): Promise<void> {
      try {
        const result = await this.userModel.findByIdAndDelete(id);
  
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
  
    @Patch(':id')
    @Roles(UserRole.SUPERUSER)
    async updateUser(
      @Param('id') id: string, 
      @Body() updateData: Partial<CreateUserDto>
    ): Promise<User | null> {
      try {
        // Remove password from update if present
        const { password, ...updateFields } = updateData;
  
        const updatedUser = await this.userModel.findByIdAndUpdate(
          id, 
          updateFields, 
          { new: true, runValidators: true }
        ).select('-password');
  
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
  }