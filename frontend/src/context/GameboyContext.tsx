import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
} from "react";
import { GameboyWasm } from "../wasm/pkg/gameboy_wasm";
import { TGame } from "../pages/Library/Library";

const GameboyContext = createContext<{
  gameboy: GameboyWasm | null;
  initGameboy: (romData: Uint8Array) => void;
  currentGame: TGame | null;
  setCurrentGame: (game: TGame | null) => void;
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
  const [currentGame, setCurrentGame] = useState<TGame | null>(null);

  const initGameboy = useCallback((romData: Uint8Array) => {

    // Create new gameboy instance
    const newGameboy = new GameboyWasm();

    try {
      newGameboy.init(romData);
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
    [gameboy, initGameboy, currentGame, setCurrentGame],
  );

  return (
    <GameboyContext.Provider value={value}>{children}</GameboyContext.Provider>
  );
};

export const useGameboy = () => useContext(GameboyContext);
