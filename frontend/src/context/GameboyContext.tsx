import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
} from "react";
import { GameboyWasm } from "../wasm/pkg/gameboy_wasm";
import { TGameDetails, TGameDetailsWithSaveState } from "../types";

const GameboyContext = createContext<{
  gameboy: GameboyWasm | null;
  initGameboy: (
    romData: Uint8Array,
    palette: number[],
    saveStateData?: Uint8Array
  ) => void;
  currentGame: TGameDetailsWithSaveState | null;
  setCurrentGame: (game: TGameDetailsWithSaveState | null) => void;
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
  const [currentGame, setCurrentGame] =
    useState<TGameDetailsWithSaveState | null>(null);

  const initGameboy = useCallback(
    async (
      romData: Uint8Array,
      palette: number[],
      saveStateData?: Uint8Array
    ) => {
      const paletteArray = new Uint32Array(palette);
      console.log("Palette Array:", paletteArray);

      try {
        const newGameboy = new GameboyWasm(paletteArray);
        await newGameboy.init(romData, saveStateData);
        console.log("Gameboy initialized successfully");
        setGameboy(newGameboy);
      } catch (error) {
        console.error("Failed to initialize Gameboy:", error);
        throw error;
      }
    },
    []
  );

  const value = useMemo(
    () => ({
      gameboy,
      initGameboy,
      currentGame,
      setCurrentGame,
    }),
    [gameboy, initGameboy, currentGame]
  );

  return (
    <GameboyContext.Provider value={value}>{children}</GameboyContext.Provider>
  );
};

export const useGameboy = () => useContext(GameboyContext);
