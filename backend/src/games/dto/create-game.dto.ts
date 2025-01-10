import { Transform } from 'class-transformer';
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

  @IsString()
  language: string;

  @IsOptional()
  @IsString()
  description?: string;

  @IsOptional()
  @IsString()
  originalTitle?: string;

  @IsOptional()
  @Transform(({ value }) => (value === '' ? undefined : Number(value)))
  @IsNumber()
  @Min(0)
  @Max(100)
  rating?: number;

  @IsOptional()
  @Transform(({ value }) => {
    if (!value) return undefined;
    if (value instanceof Date) return value;
    if (typeof value === 'string') {
      const date = new Date(value);
      return isNaN(date.getTime()) ? undefined : date;
    }
    return undefined;
  })
  @IsDate()
  releaseDate?: Date;

  @IsOptional()
  @Transform(({ value }) => {
    if (!value) return [];
    if (typeof value === 'string') {
      try {
        return JSON.parse(value);
      } catch {
        return [];
      }
    }
    return Array.isArray(value) ? value : [];
  })
  @IsArray()
  @IsString({ each: true })
  developers?: string[];

  @IsOptional()
  @Transform(({ value }) => {
    if (!value) return [];
    if (typeof value === 'string') {
      try {
        return JSON.parse(value);
      } catch {
        return [];
      }
    }
    return Array.isArray(value) ? value : [];
  })
  @IsArray()
  @IsString({ each: true })
  genres?: string[];
}
