import { Prop, Schema, SchemaFactory } from '@nestjs/mongoose';
import { HydratedDocument } from 'mongoose';

@Schema()
export class Game {
  @Prop({ required: true })
  name: string;

  @Prop({ required: true, unique: true })
  slug: string; // folder name

  @Prop({ required: true })
  language: string;

  @Prop()
  description: string;

  @Prop()
  originalTitle: string;

  @Prop({ min: 0, max: 100 })
  rating: number;

  @Prop()
  releaseDate: Date;

  @Prop({ default: [] })
  developers: string[];

  @Prop({ default: [] })
  genres: string[];
}
export const GameSchema = SchemaFactory.createForClass(Game);
export type GameDocument = HydratedDocument<Game>;
