import { useCallback, useEffect, useRef, useState } from "react";
import { useParams } from "react-router-dom";
import { CartridgeHeaderState } from "../pages/Game";
import { useGameboy } from "../context/GameboyContext";

type GameboyDisplayProps = {
  setFps: React.Dispatch<React.SetStateAction<number>>;
  setCartridgeInfo: (info: CartridgeHeaderState) => void;
  isGameboyPaused: boolean;
};
const GameboyDisplay = ({
  setFps,
  setCartridgeInfo,
  isGameboyPaused,
}: GameboyDisplayProps) => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const contextRef = useRef<CanvasRenderingContext2D | null>(null);
  const imageDataRef = useRef<ImageData | null>(null);
  const animationFrameRef = useRef<number | null>(null);
  const [pressedKeys, setPressedKeys] = useState(0xff);

  const { gameboy, initGameboy, romData, setRomData } = useGameboy();

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

  const handleCartridgeInfo = useCallback(async () => {
    if (!gameboy) return;
    try {
      const info = await gameboy.get_cartridge_info();
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
  }, [gameboy, setCartridgeInfo]);

  useEffect(() => {
    if (!gameboy) return;

    const canvas = canvasRef.current!;
    const ctx = canvas.getContext("2d")!;

    canvas.width = 160;
    canvas.height = 144;

    contextRef.current = ctx;
    imageDataRef.current = ctx.createImageData(160, 144);

    handleCartridgeInfo();

    let frameCount = 0;
    let lastFpsUpdate = performance.now();

    const renderFrame = () => {
      if (!gameboy || !ctx || !imageDataRef.current) return;

      gameboy.run_frame();

      const frameBuffer = gameboy.get_frame_buffer();
      imageDataRef.current.data.set(frameBuffer);
      ctx.putImageData(imageDataRef.current, 0, 0);

      frameCount++;
      const now = performance.now();
      if (now - lastFpsUpdate >= 1000) {
        setFps(frameCount);
        frameCount = 0;
        lastFpsUpdate = now;
      }

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
    };
  }, [gameboy, handleKeyDown, handleKeyUp, handleCartridgeInfo, setFps]);

  useEffect(() => {
    // control buttons
    if (gameboy) {
      if (isGameboyPaused) {
        gameboy.pause();
      } else {
        gameboy.resume();
      }
    }
  }, [gameboy, isGameboyPaused]);

  useEffect(() => {
    const loadRom = async () => {
      const response = await fetch(drMarioRom);
      const romArrayBuffer = await response.arrayBuffer();
      const romData = new Uint8Array(romArrayBuffer);

      if (romData.length > 0) {
        try {
          initGameboy(romData);
          console.log("initializing gameboy");
        } catch (e) {
          console.error("Error initializing GameBoy emulator:", e);
        }
      }
    };
    loadRom();
  }, []);

  return (
    <div className="flex flex-col items-center justify-center bg-gray-900 p-4 rounded-lg">
      <canvas
        ref={canvasRef}
        className="rounded-lg shadow-lg"
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
