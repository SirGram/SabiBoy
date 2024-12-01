import { useCallback, useRef, useState } from "react";
import {
  ChevronDown,
  ChevronLeft,
  ChevronUp,
  MaximizeIcon,
  PauseIcon,
  PlayIcon,
} from "lucide-react";
import { GameboyFrame } from "./components/Frame/GameboyFrame";
import GameboyDisplay from "./components/GameboyDisplay";
import { useGameboy } from "../../context/GameboyContext";
import { useNavigate } from "react-router-dom";

export interface CartridgeHeaderState {
  title: string;
  kind: string;
  rom_size: string;
  ram_size: string;
  destination: string;
  sgb_flag: string;
  rom_version: string;
  licensee_code: string;
}
export default function Emulator() {
  const [fps, setFps] = useState<number>(0);
  const [cartridgeInfo, setCartridgeInfo] = useState<CartridgeHeaderState>({
    title: "",
    kind: "",
    rom_size: "",
    ram_size: "",
    destination: "",
    sgb_flag: "",
    rom_version: "",
    licensee_code: "",
  });
  const [isGameboyPaused, setIsGameboyPaused] = useState(false);

  const { gameboy } = useGameboy();
  const [pressedKeys, setPressedKeys] = useState(0xff);
  const keyMapping: { [key: string]: { mask: number; bit: number } } = {
    ArrowRight: { mask: 0xfe, bit: 0 },
    ArrowLeft: { mask: 0xfd, bit: 1 },
    ArrowUp: { mask: 0xfb, bit: 2 },
    ArrowDown: { mask: 0xf7, bit: 3 },
    z: { mask: 0xef, bit: 4 },
    x: { mask: 0xdf, bit: 5 },
    Backspace: { mask: 0xbf, bit: 6 },
    Enter: { mask: 0x7f, bit: 7 },
  };

  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      if (!gameboy || !keyMapping[event.key]) return;

      const { mask } = keyMapping[event.key];
      const newPressedKeys = pressedKeys & mask;

      event.preventDefault();
      setPressedKeys(newPressedKeys);
      gameboy.handle_keys(newPressedKeys);
    },
    [gameboy, pressedKeys]
  );

  const handleKeyUp = useCallback(
    (event: KeyboardEvent) => {
      if (!gameboy || !keyMapping[event.key]) return;

      const { bit } = keyMapping[event.key];
      const newPressedKeys = pressedKeys | (1 << bit);

      event.preventDefault();
      setPressedKeys(newPressedKeys);
      gameboy.handle_keys(newPressedKeys);
    },
    [gameboy, pressedKeys]
  );

  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const toggleFullscreen = () => {
    const canvas = canvasRef.current;
    if (canvas) {
      if (!document.fullscreenElement) {
        canvas.requestFullscreen({ navigationUI: "show" });
      } else {
        document.exitFullscreen();
      }
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen p-4">
      <BackButton />

      <div className=" mb-4">
        FPS: {fps}
        <GameboyFrame handleKeyDown={handleKeyDown} handleKeyUp={handleKeyUp}>
          <div className="group relative">
            <div className="rounded-md group-hover:brightness-50 overflow-hidden">
              <GameboyDisplay
                setFps={setFps}
                setCartridgeInfo={setCartridgeInfo}
                isGameboyPaused={isGameboyPaused}
                handleKeyDown={handleKeyDown}
                handleKeyUp={handleKeyUp}
                canvasRef={canvasRef}
              />
            </div>
            <GameboyOptions
              isGameboyPaused={isGameboyPaused}
              setIsGameboyPaused={setIsGameboyPaused}
              toggleFullScreen={toggleFullscreen}
            />
          </div>
        </GameboyFrame>
      </div>
      <CartridgeInfo info={cartridgeInfo} />
    </div>
  );
}

function CartridgeInfo({ info }: { info: CartridgeHeaderState }) {
  const [isOpen, setIsOpen] = useState(false);

  const cartridgeInfoItems = [
    { label: "Title", value: info.title },
    { label: "Kind", value: info.kind },
    { label: "ROM Size", value: info.rom_size },
    { label: "RAM Size", value: info.ram_size },
    { label: "Destination", value: info.destination },
    { label: "SGB Flag", value: info.sgb_flag },
    { label: "ROM Version", value: info.rom_version },
    { label: "Licensee Code", value: info.licensee_code },
  ];

  return (
    <div className="w-full  mx-auto rounded-lg shadow-lg overflow-hidden">
      <div
        onClick={() => setIsOpen(!isOpen)}
        className="flex justify-between items-center p-4  cursor-pointer hover:bg-base-border transition-colors"
      >
        <h2 className="text-xl font-semibold text-white">
          Cartridge Information
        </h2>
        {isOpen ? (
          <ChevronUp className="text-white" />
        ) : (
          <ChevronDown className="text-white" />
        )}
      </div>

      {isOpen && (
        <div className="p-4 space-y-2">
          {cartridgeInfoItems.map((item, index) => (
            <div
              key={index}
              className="flex justify-between border-b border-gray-700 pb-2 last:border-b-0"
            >
              <span className="text-gray-300 font-medium">{item.label}:</span>
              <span className="text-white text-right">
                {item.value || "N/A"}
              </span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

function BackButton() {
  //use navigate
  const navigate = useNavigate();
  return (
    <div className="absolute top-0 left-0 p-6">
      <button
        className=" text-xl text-white font-bold "
        onClick={() => navigate("/")}
      >
        <ChevronLeft />
      </button>
    </div>
  );
}

function GameboyOptions({
  isGameboyPaused,
  setIsGameboyPaused,
  toggleFullScreen,
}: {
  isGameboyPaused: boolean;
  setIsGameboyPaused: React.Dispatch<React.SetStateAction<boolean>>;
  toggleFullScreen: () => void;
}) {
  return (
    <div className="absolute inset-0 z-10 hidden group-hover:block ">
      <div className="absolute bottom-2 left-2 right-2 flex justify-between items-center text-sm text-white font-bold">
        <button
          className=" p-1 rounded hover:bg-primary"
          onClick={() => setIsGameboyPaused(!isGameboyPaused)}
        >
          {isGameboyPaused ? <PlayIcon /> : <PauseIcon />}
        </button>
        <button
          className=" p-1 rounded hover:bg-primary"
          onClick={toggleFullScreen}
        >
          <MaximizeIcon size={25} />
        </button>
      </div>
    </div>
  );
}
