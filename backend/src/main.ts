import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { NestExpressApplication } from '@nestjs/platform-express';
import { join } from 'path';
import { Logger, ValidationPipe } from '@nestjs/common';
import { UsersService } from './users/users.service';
import * as jwt from 'jsonwebtoken';
import { ConfigService } from '@nestjs/config';
import * as express from 'express';

async function bootstrap() {
  const logger = new Logger('Bootstrap');
  const app = await NestFactory.create<NestExpressApplication>(AppModule);

  // Enable global validation pipes
  app.useGlobalPipes(new ValidationPipe());

  const usersService = app.get(UsersService);
  const configService = app.get(ConfigService);

  await usersService.createFirstUserIfNoneExist();

  app.setGlobalPrefix('api');

  // Serve static assets from the "games" folder
  const gamesMiddleware = createGamesMiddleware(configService);
  app.use('/api/games', gamesMiddleware);
  app.useStaticAssets(join(__dirname, '..', '..', 'games'), {
    prefix: '/api/games',
  });

  logger.log('Serving protected static assets from /api/games directory');

  // Enable CORS
  app.enableCors();

  const port = process.env.PORT ?? 3000;
  await app.listen(port);
  logger.log(`Server listening on http://localhost:${port}`);
}

function createGamesMiddleware(configService: ConfigService) {
  return (req: express.Request, res: express.Response, next: express.NextFunction) => {
    const authHeader = req.headers.authorization;

    if (!authHeader) {
      return res.status(401).json({
        statusCode: 401,
        message: 'Unauthorized: No token provided',
      });
    }

    try {
      // Extract token from Authorization header
      const token = authHeader.split(' ')[1];

      // Verify the token using the secret from ConfigService
      const secret = configService.get<string>('JWT_SECRET');
      jwt.verify(token, secret);

      next();
    } catch (error) {
      console.error('JWT verification failed:', error.message);

      return res.status(401).json({
        statusCode: 401,
        message: 'Unauthorized: Invalid or expired token',
      });
    }
  };
}

bootstrap();
