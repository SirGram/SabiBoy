import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
} from "react";
import { GameboyWasm } from "../wasm/pkg/gameboy_wasm";
import { TGame, TGameDetails } from "../pages/Library/Library";


const GameboyContext = createContext<{
  gameboy: GameboyWasm | null;
  initGameboy: (romData: Uint8Array, palette: number[], saveStateData?: Uint8Array) => void;
  currentGame: TGameDetails | null;
  setCurrentGame: (game: TGameDetails | null) => void;
}>({
  gameboy: null,
  initGameboy: () => {},
  currentGame: null,
  setCurrentGame: () => {},
});
export const GameboyProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [gameboy, setGameboy] = useState<GameboyWasm | null>(null);
  const [currentGame, setCurrentGame] = useState<TGameDetails | null>(null);

  const initGameboy = useCallback((romData: Uint8Array, palette: number[], saveStateData?: Uint8Array) => {
    // Create new gameboy instance
    const paletteArray = new Uint32Array(palette);
    console.log("Palette Array:", paletteArray);
    const newGameboy = new GameboyWasm(paletteArray);

    try {
      newGameboy.init(romData, saveStateData);
      console.log("Gameboy initialized successfully");
      setGameboy(newGameboy);
    } catch (error) {
      console.error("Failed to initialize Gameboy:", error);
    }
  }, []);

  const value = useMemo(
    () => ({
      gameboy,
      initGameboy,
      currentGame,
      setCurrentGame,
    }),
    [gameboy, initGameboy, currentGame, setCurrentGame]
  );

  return (
    <GameboyContext.Provider value={value}>{children}</GameboyContext.Provider>
  );
};

export const useGameboy = () => useContext(GameboyContext);
