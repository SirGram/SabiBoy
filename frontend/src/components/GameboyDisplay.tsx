import { useEffect, useRef, useState } from "react";
import { GameboyWasm } from "../wasm/pkg/gameboy_wasm";
import { useParams } from "react-router-dom";
import { CartridgeHeaderState } from "../pages/Game";

type GameboyDisplayProps = {
  setFps: React.Dispatch<React.SetStateAction<number>>;
  setCartridgeInfo: (info: CartridgeHeaderState) => void;
};
const GameboyDisplay = ({ setFps, setCartridgeInfo }: GameboyDisplayProps) => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const gameboyRef = useRef<GameboyWasm | null>(null);
  const contextRef = useRef<CanvasRenderingContext2D | null>(null);
  const imageDataRef = useRef<ImageData | null>(null);
  const animationFrameRef = useRef<number | null>(null);

  const [pressedKeys, setPressedKeys] = useState(0xff);

  const { itemId } = useParams<{ itemId: string }>();

  type KeyMappingType = {
    [key: string]: { mask: number; bit: number };
  };
  const keyMapping: KeyMappingType = {
    ArrowRight: { mask: 0xfe, bit: 0 },
    ArrowLeft: { mask: 0xfd, bit: 1 },
    ArrowUp: { mask: 0xfb, bit: 2 },
    ArrowDown: { mask: 0xf7, bit: 3 },
    z: { mask: 0xef, bit: 4 },
    x: { mask: 0xdf, bit: 5 },
    Backspace: { mask: 0xbf, bit: 6 },
    Enter: { mask: 0x7f, bit: 7 },
  };

  const handleKeyDown = (event: KeyboardEvent) => {
    if (!gameboyRef.current || !keyMapping[event.key]) return;

    const { mask } = keyMapping[event.key];
    const newPressedKeys = pressedKeys & mask;

    event.preventDefault();
    setPressedKeys(newPressedKeys);
    gameboyRef.current.handle_keys(newPressedKeys);
  };

  const handleKeyUp = (event: KeyboardEvent) => {
    if (!gameboyRef.current || !keyMapping[event.key]) return;

    const { bit } = keyMapping[event.key];
    const newPressedKeys = pressedKeys | (1 << bit);

    event.preventDefault();
    setPressedKeys(newPressedKeys);
    gameboyRef.current.handle_keys(newPressedKeys);
  };

  async function handleCartridgeInfo() {
    if (!gameboyRef.current) return;
    try {
      const info = await gameboyRef.current.get_cartridge_info();
      console.log("Raw info:", info);

      // Safely extract and set cartridge info
      setCartridgeInfo({
        title: info.title ?? "",
        kind: info.kind ?? "",
        rom_size: info.rom_size ?? "",
        ram_size: info.ram_size ?? "",
        destination: info.destination ?? "",
        sgb_flag: info.sgb_flag ?? "",
        rom_version: info.rom_version ?? "",
        licensee_code: info.licensee_code ?? "",
      });
    } catch (error) {
      console.error("Error setting cartridge info:", error);
    }
  }

  useEffect(() => {
    const initGameboy = async () => {
      const canvas = canvasRef.current!;
      const ctx = canvas.getContext("2d")!;

      canvas.width = 160;
      canvas.height = 144;

      contextRef.current = ctx;
      imageDataRef.current = ctx.createImageData(160, 144);

      const gameboy = new GameboyWasm();
      if (!itemId) return;
      gameboy.init(itemId);
      gameboyRef.current = gameboy;
      handleCartridgeInfo();

      let frameCount = 0;
      let lastFpsUpdate = performance.now();

      const renderFrame = () => {
        const gameboy = gameboyRef.current;
        const ctx = contextRef.current;
        const imageData = imageDataRef.current;

        if (!gameboy || !ctx || !imageData) return;

        // Run full frame
        gameboy.run_frame();

        // Render frame
        const frameBuffer = gameboy.get_frame_buffer();
        imageData.data.set(frameBuffer);
        ctx.putImageData(imageData, 0, 0);

        // FPS calculation
        frameCount++;
        const now = performance.now();
        if (now - lastFpsUpdate >= 1000) {
          setFps(frameCount);
          frameCount = 0;
          lastFpsUpdate = now;
        }

        // Schedule next frame
        animationFrameRef.current = requestAnimationFrame(renderFrame);
      };

      animationFrameRef.current = requestAnimationFrame(renderFrame);

      window.addEventListener("keydown", handleKeyDown);
      window.addEventListener("keyup", handleKeyUp);

      return () => {
        if (animationFrameRef.current) {
          cancelAnimationFrame(animationFrameRef.current);
        }
        window.removeEventListener("keydown", handleKeyDown);
        window.removeEventListener("keyup", handleKeyUp);
        gameboy.free();
      };
    };

    initGameboy();
  }, [itemId]);

  return (
    <div className="flex flex-col items-center justify-center bg-gray-900 p-4 rounded-lg">
      <canvas
        ref={canvasRef}
        className=" rounded-lg shadow-lg"
        style={{
          imageRendering: "pixelated",
          width: "480px",
          height: "432px",
          backgroundColor: "#9BA4B5",
        }}
      />
    </div>
  );
};

export default GameboyDisplay;
