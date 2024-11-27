import { useState } from "react";
import GameboyDisplay from "../../components/GameboyDisplay";
import { ChevronDown, ChevronUp } from "lucide-react";

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
  return (
    <div className="flex flex-col items-center justify-center min-h-screen  p-4">
      <div className="text-white mb-4">
        FPS: {fps}
        <GameboyDisplay setFps={setFps} setCartridgeInfo={setCartridgeInfo} />
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
