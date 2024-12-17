import { Injectable } from '@nestjs/common';
import { InjectModel } from '@nestjs/mongoose';
import { Model } from 'mongoose';
import { User, UserDocument, UserRole } from '../schemas/user.schema';
import { AuthService } from 'src/auth/auth.service';
import { CreateUserDto } from 'src/auth/dto/create_user.dto';

@Injectable()
export class UsersService {
  constructor(
    @InjectModel(User.name) 
    private userModel: Model<UserDocument>,
    private readonly authService: AuthService
  ) {}
  async createFirstUserIfNoneExist(): Promise<void> {
    const existingUser = await this.userModel.findOne();

    if (!existingUser) {
      const createUserDto: CreateUserDto = {
        email: 'admin@example.com',  
        password: 'supersecretpassword', 
        role: UserRole.SUPERUSER,  
      };

      const user = new this.userModel({
        ...createUserDto
      });

      await user.save();
      console.log('First superuser created');
    }
  }

  async findAll(role?: UserRole): Promise<User[]> {
    const filter = role ? { role } : {};
    return this.userModel.find(filter).select('-password');
  }

  async findById(id: string): Promise<User | null> {
    return this.userModel.findById(id).select('-password');
  }

  async findByEmail(email: string): Promise<User | null> {
    return this.userModel.findOne({ email }).select('+password');
  }

  async updateUserRole(id: string, role: UserRole): Promise<User | null> {
    return this.userModel.findByIdAndUpdate(
      id, 
      { role }, 
      { new: true }
    ).select('-password');
  }
}