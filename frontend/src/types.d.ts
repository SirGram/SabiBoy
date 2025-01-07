export type Game = {
    id: number;
    title: string;
    image: string;
    rom: Uint8Array;
  };
export type TGame = {
  slug: string;
  name: string;  
  language:string
  coverPath?: string;
  coverURL?: string;
};

export type TRomSource = {
  type: 'url' | 'blob';
  path: string;
  data?: Uint8Array;
};

export type TSaveStateSource = {
  type: 'url' | 'blob';
  path: string;
  data?: Uint8Array;
};

export type TGameDetails = TGame & {
  rom: TRomSource;
  screenshotPaths: string[];
  description?: string;
  originalTitle?: string;
  rating?: number;
  releaseDate?: string;
  developers?: string[];
  genres?: string[];
};
export type TGameDetailsWithSaveState = TGameDetails & {
  saveState?: TSaveStateSource;
};