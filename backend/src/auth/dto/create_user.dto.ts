import { IsEmail, IsString, IsEnum, MinLength } from 'class-validator';

export enum UserRole {
  NORMAL = 'normal',
  SUPERUSER = 'superuser'
}

export class CreateUserDto {
  @IsEmail()
  email: string;

  @IsString()
  password: string;

  @IsEnum(UserRole)
  role: UserRole = UserRole.NORMAL;
}
