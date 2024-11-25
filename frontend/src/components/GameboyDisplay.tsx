import { useEffect, useRef, useState } from "react";
import init, { GameboyWasm } from "../wasm/pkg/gameboy_wasm";

const TICKS_PER_FRAME = 70224;
const TARGET_FPS = 60;
const FRAME_DURATION = 1000 / TARGET_FPS;

const GameboyDisplay = () => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const gameboyRef = useRef<GameboyWasm | null>(null);
  const contextRef = useRef<CanvasRenderingContext2D | null>(null);
  const imageDataRef = useRef<ImageData | null>(null);
  const [fps, setFps] = useState(0);
  const [pressedKeys, setPressedKeys] = useState(0xFF);

  type KeyMapping = {
    [key: string]: {
      mask: number;
      bit: number;
    };
  }

  const keyMapping: KeyMapping = {
    "ArrowRight":  { mask: 0xfe, bit: 0 },
    "ArrowLeft":   { mask: 0xfd, bit: 1 },
    "ArrowUp":     { mask: 0xfb, bit: 2 },
    "ArrowDown":   { mask: 0xf7, bit: 3 },
    "z":           { mask: 0xef, bit: 4 }, // A
    "x":           { mask: 0xdf, bit: 5 }, // B
    "Backspace":   { mask: 0xbf, bit: 6 }, // Select
    "Enter":       { mask: 0x7f, bit: 7 }  // Start
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

  useEffect(() => {
    const initGameboy = async () => {
      await init();
      const canvas = canvasRef.current!;
      const ctx = canvas.getContext("2d")!;

      canvas.width = 160;
      canvas.height = 144;

      contextRef.current = ctx;
      imageDataRef.current = ctx.createImageData(160, 144);

      const gameboy = new GameboyWasm();
      gameboy.init();
      gameboyRef.current = gameboy;

      const lastFrameTime = { value: performance.now() };
      let frameCount = 0;
      let fpsUpdateTime = lastFrameTime.value;

      const renderFrame = (currentTime: number) => {
        const gameboy = gameboyRef.current;
        const ctx = contextRef.current;
        const imageData = imageDataRef.current;

        if (!gameboy || !ctx || !imageData) return;

        // Run emulation ticks for this frame
        for (let i = 0; i < TICKS_PER_FRAME; i++) {
          gameboy.tick();
        }

        // Render frame
        const frameBuffer = gameboy.get_frame_buffer();
        imageData.data.set(frameBuffer);
        ctx.putImageData(imageData, 0, 0);

        // FPS calculation
        frameCount++;
        if (currentTime - fpsUpdateTime >= 1000) {
          setFps(frameCount);
          frameCount = 0;
          fpsUpdateTime = currentTime;
        }

        // Ensure consistent frame timing
        const elapsed = currentTime - lastFrameTime.value;
        if (elapsed < FRAME_DURATION) {
          setTimeout(() => requestAnimationFrame(renderFrame), FRAME_DURATION - elapsed);
        } else {
          requestAnimationFrame(renderFrame);
        }

        lastFrameTime.value = currentTime;
      };

      requestAnimationFrame(renderFrame);
      window.addEventListener("keydown", handleKeyDown);
      window.addEventListener("keyup", handleKeyUp);

      return () => {
        window.removeEventListener("keydown", handleKeyDown);
        window.removeEventListener("keyup", handleKeyUp);
        gameboy.free();
      };
    };

    initGameboy();
  }, []);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 p-4">
      <div className="text-white mb-4">FPS: {fps}</div>
      <canvas
        ref={canvasRef}
        className="border-4 border-gray-700 rounded-lg shadow-lg"
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