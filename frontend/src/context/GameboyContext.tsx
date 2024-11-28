import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
} from "react";
import { GameboyWasm } from "../wasm/pkg/gameboy_wasm";

const GameboyContext = createContext<{
  gameboy: GameboyWasm | null;
  initGameboy: (itemId: string) => void;
}>({
  gameboy: null,
  initGameboy: () => {},
});
export const GameboyProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [gameboy, setGameboy] = useState<GameboyWasm | null>(null);

  const initGameboy = useCallback((itemId: string) => {
    console.log("Initializing Gameboy with itemId:", itemId);

    // Create new gameboy instance
    const newGameboy = new GameboyWasm();

    try {
      newGameboy.init(itemId);
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
    }),
    [gameboy, initGameboy]
  );

  return (
    <GameboyContext.Provider value={value}>{children}</GameboyContext.Provider>
  );
};

export const useGameboy = () => useContext(GameboyContext);
