import { Prop, Schema, SchemaFactory } from '@nestjs/mongoose';
import { Document, HydratedDocument, Types } from 'mongoose';
import * as bcrypt from 'bcrypt';

export enum UserRole {
  NORMAL = 'normal',
  SUPERUSER = 'superuser',
}

@Schema({
  timestamps: true,
  toJSON: {
    transform: (doc, ret) => {
      delete ret.password;
      return ret;
    },
  },
})
export class User {
  @Prop({
    required: true,
    unique: true,
    lowercase: true,
    trim: true,
  })
  email: string;

  @Prop({
    required: true,
    minlength: 6,
  })
  password: string;

  @Prop({
    type: String,
    enum: UserRole,
    default: UserRole.NORMAL,
  })
  role: UserRole;

  @Prop({
    type: [
      {
        game: {
          type: Types.ObjectId,
          ref: 'Game',
        },
        savedState: {
          type: Boolean,
          default: false,
        },
      },
    ],
    default: [],
  })
  library: Array<{
    game: Types.ObjectId;
    savedState: boolean;
  }>;
}

export type UserDocument = HydratedDocument<User>;

export const UserSchema = SchemaFactory.createForClass(User);

UserSchema.pre('save', async function (next) {
  // Only hash the password if it has been modified
  if (!this.isModified('password')) return next();

  try {
    const salt = await bcrypt.genSalt(10);
    this.password = await bcrypt.hash(this.password, salt);
    next();
  } catch (error) {
    next(error as Error);
  }
});
