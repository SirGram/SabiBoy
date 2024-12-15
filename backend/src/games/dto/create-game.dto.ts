import { Transform, Type } from 'class-transformer';
import {
  IsString,
  IsOptional,
  IsNumber,
  IsDate,
  IsArray,
  Min,
  Max,
} from 'class-validator';
export class CreateGameDto {
  @IsString()
  name: string;

  @IsString()
  slug: string;

  @IsOptional()
  @IsString()
  description?: string;

  @IsOptional()
  @IsString()
  originalTitle?: string;

  @IsOptional()
  @IsNumber()
  @Min(0)
  @Max(100)
  rating?: number;

  @IsOptional()
  @Transform(({ value }) => {
    if (value instanceof Date) return value;

    // If it's a number (timestamp), convert to Date
    if (typeof value === 'number') {
      return new Date(value * 1000);
    }

    // If it's a string that looks like a timestamp, convert it
    if (typeof value === 'string' && /^\d+$/.test(value)) {
      return new Date(parseInt(value) * 1000);
    }

    return value;
  })
  @Type(() => Date)
  @IsDate()
  releaseDate?: Date;

  @IsOptional()
  @IsArray()
  @IsString({ each: true })
  developers?: string[];

  @IsOptional()
  @IsArray()
  @IsString({ each: true })
  genres?: string[];
}

export class UpdateGameDto extends CreateGameDto {}
