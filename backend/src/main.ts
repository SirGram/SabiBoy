import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { NestExpressApplication } from '@nestjs/platform-express';
import { join } from 'path';

async function bootstrap() {
  const app = await NestFactory.create<NestExpressApplication>(AppModule);

  app.setGlobalPrefix('api');
  // Folder from which files are served
  app.useStaticAssets(join(__dirname, '..', '..', 'test'), {
    prefix: '/api',
  });

  const corsOptions = {};
  app.enableCors(corsOptions);

  const port = process.env.PORT ?? 3000;
  await app.listen(port);
  console.log(`Server listening on http://localhost:${port}`);
}
bootstrap();
