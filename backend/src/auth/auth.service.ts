import {
  Injectable,
  ConflictException,
  UnauthorizedException,
  Logger,
} from '@nestjs/common';
import { InjectModel } from '@nestjs/mongoose';
import { Model } from 'mongoose';
import { JwtService } from '@nestjs/jwt';
import { User, UserDocument } from 'src/schemas/user.schema';
import { CreateUserDto } from './dto/create_user.dto';
import { LoginDto } from './dto/login_user.dto';
import * as bcrypt from 'bcrypt';

@Injectable()
export class AuthService {
  private readonly logger = new Logger(AuthService.name);
  constructor(
    @InjectModel(User.name)
    private userModel: Model<UserDocument>,
    private jwtService: JwtService,
  ) {}

  async register(createUserDto: CreateUserDto): Promise<User> {
    // Check if user already exists
    const existingUser = await this.userModel.findOne({
      email: createUserDto.email,
    });

    if (existingUser) {
      throw new ConflictException('User already exists');
    }

    const createdUser = new this.userModel(createUserDto);
    return createdUser.save();
  }

  async login(loginDto: LoginDto) {
    
    const { email, password } = loginDto;
    this.logger.log(`Login attempt for email: ${email}`);
    const user = await this.userModel.findOne({ email });

    if (!user) {
      this.logger.warn(`Login failed for email: ${email} - User not found`); 
      throw new UnauthorizedException('Invalid credentials');
    }

    const isPasswordValid = await bcrypt.compare(password, user.password);
    if (!isPasswordValid) {
      this.logger.warn(`Login failed for email: ${email} - Invalid password`);
      throw new UnauthorizedException('Invalid credentials');
    }

    this.logger.log(`Login successful for email: ${email}`); 

    return {
      access_token: this.generateToken(user),
      user: { 
        id: user._id, 
        email: user.email, 
        role: user.role 
      }
    };
  }

  generateToken(user: UserDocument) {
    return this.jwtService.sign({
      sub: user._id,
      email: user.email,
      role: user.role,
    });
  }

  async findById(id: string): Promise<User | null> {
    return this.userModel.findById(id).select('-password');
  }
}
