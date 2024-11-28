import { Game } from '../../../frontend/src/types';
import fs from 'fs';
import path from 'path';

export function scanRomLibrary({
  path: libraryPath,
}: {
  path: string;
}): Game[] {
  const games: Game[] = [];

  try {
    const gameFolders = fs
      .readdirSync(libraryPath)
      .filter((folder) =>
        fs.statSync(path.join(libraryPath, folder)).isDirectory(),
      );

    gameFolders.forEach((gameFolder, index) => {
      const game = readGameFolder(libraryPath, gameFolder, index + 1);
      if (game) {
        games.push(game);
      }
    });
  } catch (error) {
    console.error('Error scanning rom library:', error);
  }

  return games;
}

function readGameFolder(
  baseLibraryPath: string,
  gameFolder: string,
  id: number,
): Game | undefined {
  const gamePath = path.join(baseLibraryPath, gameFolder);

  // Find the rom file
  const romFiles = fs
    .readdirSync(gamePath)
    .filter((file) => file.endsWith('.gb'));

  if (romFiles.length === 0) return; // Skip if no ROM found

  const romFile = romFiles[0];
  const romPath = path.join(gamePath, romFile);

  // Find cover image
  const coverFiles = fs
    .readdirSync(gamePath)
    .filter((file) =>
      ['cover.jpg', 'cover.png', 'cover.webp'].includes(file.toLowerCase()),
    );

  const coverImage =
    coverFiles.length > 0
      ? path.join(gamePath, coverFiles[0])
      : '/default-cover.png';

  return {
    id: id,
    title: gameFolder.replace(/_/g, ' '),
    image: coverImage,
    rom_path: romPath,
  };
}

export default scanRomLibrary;
