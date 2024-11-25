import { useEffect, useRef, useState } from "react";
import init, { GameboyWasm } from "../wasm/pkg/gameboy_wasm";

const TICKS_PER_FRAME = 70224;

const GameboyDisplay = () => {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const gameboyRef = useRef<GameboyWasm | null>(null);
  const animationFrameRef = useRef<number | null>(null);
  const contextRef = useRef<CanvasRenderingContext2D | null>(null);
  const imageDataRef = useRef<ImageData | null>(null);

  useEffect(() => {
    const initGameboy = async () => {
      try {
        // Initialize WebAssembly module
        await init();

        // Set up canvas context
        const canvas = canvasRef.current;
        if (!canvas) throw new Error("Canvas not found");

        const ctx = canvas.getContext("2d");
        if (!ctx) throw new Error("Could not get canvas context");

        // Set up canvas dimensions
        canvas.width = 160; // GameBoy native resolution
        canvas.height = 144;

        // Store context and create ImageData for efficient rendering
        contextRef.current = ctx;
        imageDataRef.current = ctx.createImageData(160, 144);

        // Initialize GameBoy
        const gameboy = new GameboyWasm();
        gameboy.init();
        gameboyRef.current = gameboy;

        // Start the render loop
        startRenderLoop();
      } catch (error) {
        console.error("GameBoy initialization failed:", error);
      }
    };

    const startRenderLoop = () => {
      const renderFrame = () => {
        const gameboy = gameboyRef.current;
        const ctx = contextRef.current;
        const imageData = imageDataRef.current;

        if (!gameboy || !ctx || !imageData) return;

        try {
          for (let i = 0; i < TICKS_PER_FRAME; i++) {
            gameboy.tick();
          }

          // Get the frame buffer and update ImageData
          const frameBuffer = gameboy.get_frame_buffer();
          imageData.data.set(frameBuffer);
          ctx.putImageData(imageData, 0, 0);
        } catch (error) {
          console.error("Error during frame render:", error);
          return;
        }

        // Schedule next frame
        animationFrameRef.current = requestAnimationFrame(renderFrame);
      };

      renderFrame();
    };

    initGameboy();
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);

    // Cleanup function
    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
      if (gameboyRef.current) {
        gameboyRef.current.free();
      }
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, []);

  const [pressedKeys, setPressedKeys] = useState(0xff);

  const handleKeyDown = (event: KeyboardEvent) => {
    if (!gameboyRef.current) {
      return;
    }
    let newPressedKeys = pressedKeys;
    switch (event.key) {
      case "ArrowRight":
        newPressedKeys &= 0xfe;
        break;
      case "ArrowLeft":
        newPressedKeys &= 0xfd;
        break;
      case "ArrowUp":
        newPressedKeys &= 0xfb;
        break;
      case "ArrowDown":
        newPressedKeys &= 0xf7;
        break;
      case "z":
        newPressedKeys &= 0xef; // A
        break;
      case "x":
        newPressedKeys &= 0xdf; // B
        break;
      case "Backspace":
        newPressedKeys &= 0xbf; // Select
        break;
      case "Enter":
        newPressedKeys &= 0x7f; // Start
        break;
      default:
        return;
    }
    event.preventDefault();
    setPressedKeys(newPressedKeys);
    gameboyRef.current.handle_keys(newPressedKeys);
  };

  const handleKeyUp = (event: KeyboardEvent) => {
    if (!gameboyRef.current) {
      return;
    }
    let newPressedKeys = pressedKeys;
    switch (event.key) {
      case "ArrowRight":
        newPressedKeys |= 0x01;
        break;
      case "ArrowLeft":
        newPressedKeys |= 0x02;
        break;
      case "ArrowUp":
        newPressedKeys |= 0x04;
        break;
      case "ArrowDown":
        newPressedKeys |= 0x08;
        break;
      case "z":
        newPressedKeys |= 0x10; // A
        break;
      case "x":
        newPressedKeys |= 0x20; // B
        break;
      case "Backspace":
        newPressedKeys |= 0x40; // Select
        break;
      case "Enter":
        newPressedKeys |= 0x80; // Start
        break;
    }
    event.preventDefault();
    setPressedKeys(newPressedKeys);
    gameboyRef.current.handle_keys(newPressedKeys);
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 p-4">
      <canvas
        ref={canvasRef}
        className="border-4 border-gray-700 rounded-lg shadow-lg"
        style={{
          imageRendering: "pixelated",
          width: "480px", // 160 * 3
          height: "432px", // 144 * 3
          backgroundColor: "#9BA4B5",
        }}
      />
    </div>
  );
};

export default GameboyDisplay;
