import { useState } from "react";
import GameboyDisplay from "../../components/GameboyDisplay";
import { ChevronDown, ChevronUp, PauseIcon, PlayIcon } from "lucide-react";
import { GameboyProvider } from "../../context/GameboyContext";
import { GameboyFrame } from "./components/GameboyFrame";

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
export default function Game() {
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
  const [speed, setSpeed] = useState(1);
  return (
    <GameboyProvider>
      <div className="flex flex-col items-center justify-center min-h-screen  p-4">
        <ControlButtons
          isGameboyPaused={isGameboyPaused}
          setIsGameboyPaused={setIsGameboyPaused}
          speed={speed}
          setSpeed={setSpeed}
        />
          <div className="text-white mb-4">
            FPS: {fps}
        <GameboyFrame>
            <GameboyDisplay
              setFps={setFps}
              setCartridgeInfo={setCartridgeInfo}
              isGameboyPaused={isGameboyPaused}
            />
        </GameboyFrame>
          </div>
        <CartridgeInfo info={cartridgeInfo} />
      </div>
    </GameboyProvider>
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
    <div className="w-full  mx-auto bg-gray-800 rounded-lg shadow-lg overflow-hidden">
      <div
        onClick={() => setIsOpen(!isOpen)}
        className="flex justify-between items-center p-4 bg-gray-700 cursor-pointer hover:bg-gray-600 transition-colors"
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

function ControlButtons({
  isGameboyPaused,
  setIsGameboyPaused,
  speed,
  setSpeed,
}: {
  isGameboyPaused: boolean;
  setIsGameboyPaused: React.Dispatch<React.SetStateAction<boolean>>;
  speed: number;
  setSpeed: React.Dispatch<React.SetStateAction<number>>;
}) {
  const speedOptions = [1, 2];
  return (
    <div className="flex flex-row justify-between items-center h-10">
      <button
        className="bg-gray-700 hover:bg-gray-600 text-white font-bold py-2 px-4 rounded-lg h-full"
        onClick={() => setIsGameboyPaused(!isGameboyPaused)}
      >
        {isGameboyPaused ? <PlayIcon /> : <PauseIcon />}
      </button>
      <div className="flex items-center h-full rounded-lg">
        {speedOptions.map((speedOption) => (
          <button
            key={speedOption}
            className={`
              px-3 py-1  h-full
              ${
                speed === speedOption
                  ? "bg-gray-600 "
                  : "bg-gray-400 text-gray-700 hover:bg-gray-500"
              }
            `}
            onClick={() => setSpeed(speedOption)}
          >
            {speedOption}x
          </button>
        ))}
      </div>
    </div>
  );
}
