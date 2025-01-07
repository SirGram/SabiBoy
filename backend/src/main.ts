import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { NestExpressApplication } from '@nestjs/platform-express';
import { join } from 'path';
import { Logger, ValidationPipe } from '@nestjs/common';
import { UsersService } from './users/users.service';
import * as jwt from 'jsonwebtoken';
import { ConfigService } from '@nestjs/config';
import * as express from 'express';
import { json, raw, urlencoded } from 'express';
import * as getRawBody from 'raw-body';
import * as fs from 'fs';

async function bootstrap() {
  const logger = new Logger('Bootstrap');
  const app = await NestFactory.create<NestExpressApplication>(AppModule, {
    rawBody: true,
  });

  app.enableCors({
    origin: '*', // Add your frontend URL
    methods: 'GET,HEAD,PUT,PATCH,POST,DELETE,OPTIONS',
    credentials: true,
    allowedHeaders: ['Content-Type', 'Authorization', 'Accept'],
    preflightContinue: false,
    optionsSuccessStatus: 204,
  });

  // Enable global validation pipes
  app.useGlobalPipes(new ValidationPipe());

  const usersService = app.get(UsersService);
  const configService = app.get(ConfigService);

  await usersService.createFirstUserIfNoneExist();

  app.setGlobalPrefix('api');

  // Set up raw body parsing for binary data
  app.use(
    '/api/users/:id/library/:slug/save-state',
    raw({ type: 'application/octet-stream', limit: '100kb' }),
  );
  app.use(json({ limit: '100kb' }));
  app.use(urlencoded({ limit: '100kb', extended: true }));

  // Serve static assets from the "games" folder
  const gamesMiddleware = createGamesMiddleware(configService);
  app.use('/api/games', gamesMiddleware);

  // Modified path resolution for games directory
  // In main.ts, before setting up static assets
  const gamesPath =
    process.env.NODE_ENV === 'production'
      ? '/app/games'
      : join(__dirname, '..', '..', 'games');

  // Add detailed logging
  logger.log(`Node Environment: ${process.env.NODE_ENV}`);
  logger.log(`Games Path: ${gamesPath}`);
  logger.log(`Directory contents:`);
  try {
    const contents = fs.readdirSync(gamesPath);
    logger.log(contents);

    // Check permissions
    const stats = fs.statSync(gamesPath);
    logger.log(`Directory permissions: ${stats.mode}`);
  } catch (error) {
    logger.error(`Error accessing games directory: ${error.message}`);
  }

  app.useStaticAssets(gamesPath, {
    prefix: '/api/games',
  });

  logger.log(`Serving protected static assets from ${gamesPath} directory`);

  const port = process.env.PORT ?? 3000;
  await app.listen(port, '0.0.0.0');
  logger.log(`Server listening on http://localhost:${port}`);
}

function createGamesMiddleware(configService: ConfigService) {
  return (
    req: express.Request,
    res: express.Response,
    next: express.NextFunction,
  ) => {
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
