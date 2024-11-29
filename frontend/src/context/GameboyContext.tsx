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
  initGameboy: (romData: Uint8Array) => void;
  romData: Uint8Array;
  setRomData: (romData: Uint8Array) => void;
}>({
  gameboy: null,
  initGameboy: () => {},
  romData: new Uint8Array(),
  setRomData: () => {},
});
export const GameboyProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [gameboy, setGameboy] = useState<GameboyWasm | null>(null);
  const [romData, setRomData] = useState<Uint8Array>(new Uint8Array());

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
      romData,
      setRomData,
    }),
    [gameboy, initGameboy, romData, setRomData]
  );

  return (
    <GameboyContext.Provider value={value}>{children}</GameboyContext.Provider>
  );
};

export const useGameboy = () => useContext(GameboyContext);
