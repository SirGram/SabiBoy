import { useEffect, useRef, useState } from "react";
import init, { GameboyWasm } from "../wasm/pkg/gameboy_wasm";
import { useParams } from "react-router-dom";


const GameboyDisplay = () => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const gameboyRef = useRef<GameboyWasm | null>(null);
  const contextRef = useRef<CanvasRenderingContext2D | null>(null);
  const imageDataRef = useRef<ImageData | null>(null);
  const animationFrameRef = useRef<number | null>(null);

  const [fps, setFps] = useState(0);
  const [pressedKeys, setPressedKeys] = useState(0xFF);

  const { itemId } = useParams<{ itemId: string }>();

  type KeyMappingType = {
    [key: string]: { mask: number; bit: number };
  };
  const keyMapping: KeyMappingType = {
    "ArrowRight":  { mask: 0xfe, bit: 0 },
    "ArrowLeft":   { mask: 0xfd, bit: 1 },
    "ArrowUp":     { mask: 0xfb, bit: 2 },
    "ArrowDown":   { mask: 0xf7, bit: 3 },
    "z":           { mask: 0xef, bit: 4 }, 
    "x":           { mask: 0xdf, bit: 5 }, 
    "Backspace":   { mask: 0xbf, bit: 6 }, 
    "Enter":       { mask: 0x7f, bit: 7 }  
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
      if (!itemId) return;
      gameboy.init(itemId);
      gameboyRef.current = gameboy;

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