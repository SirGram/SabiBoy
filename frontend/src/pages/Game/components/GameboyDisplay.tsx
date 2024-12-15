import { useCallback, useEffect, useRef } from "react";
import { CartridgeHeaderState } from "../Emulator";
import { useGameboy } from "../../../context/GameboyContext";
import { useOptions } from "../../../context/OptionsContext";

type GameboyDisplayProps = {
  setFps: React.Dispatch<React.SetStateAction<number>>;
  setCartridgeInfo: (info: CartridgeHeaderState) => void;
  isGameboyPaused: boolean;
  handleKeyDown: (event: KeyboardEvent) => void;
  handleKeyUp: (event: KeyboardEvent) => void;
  canvasRef: React.RefObject<HTMLCanvasElement>;
  isAudioEnabled: boolean;
  playAudioFrame: (audioContext: AudioContext) => void;
};
const GameboyDisplay = ({
  setFps,
  setCartridgeInfo,
  isGameboyPaused,
  handleKeyDown,
  handleKeyUp,
  canvasRef,
  playAudioFrame,
}: GameboyDisplayProps) => {
  const contextRef = useRef<CanvasRenderingContext2D | null>(null);
  const imageDataRef = useRef<ImageData | null>(null);
  const animationFrameRef = useRef<number | null>(null);

  const { gameboy, initGameboy, currentGame } = useGameboy();
  const { options } = useOptions();

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

    const audioContext = new AudioContext();

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
      playAudioFrame(audioContext);

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
      audioContext.close();
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
    const loadEmulator = async () => {
      if (!currentGame?.romPath) return;

      try {
        // Fetch the ROM file
        console.log(`Fetching ROM from: ${currentGame.romPath}`);
        const romResponse = await fetch(currentGame.romPath);
        if (!romResponse.ok) {
          console.error(`Failed to fetch ROM: ${romResponse.statusText}`);
          return;
        }
        const romArrayBuffer = await romResponse.arrayBuffer();
        const romData = new Uint8Array(romArrayBuffer);

        // Fetch the save state file, if available
        let stateData: Uint8Array | undefined = undefined;
        try {
          const stateResponse = await fetch(`${currentGame.romPath}.state`);
          if (stateResponse.ok) {
            const stateArrayBuffer = await stateResponse.arrayBuffer();
            stateData = new Uint8Array(stateArrayBuffer);
            console.log("State file loaded successfully.");
          } else {
            console.warn(
              `State file not found for ${currentGame.romPath}. Skipping state load.`
            );
          }
        } catch (stateError) {
          console.error("Error fetching state file:", stateError);
        }

        // Initialize the GameBoy emulator with the ROM and optional state data
        if (romData.length > 0) {
          try {
            console.log("Initializing GameBoy emulator...");
            await initGameboy(romData, options.palette, stateData);
            console.log("GameBoy emulator initialized successfully.");
          } catch (initError) {
            console.error("Error initializing GameBoy emulator:", initError);
          }
        }
      } catch (error) {
        console.error("Error loading the emulator:", error);
      }
    };

    loadEmulator();
  }, [currentGame, options.palette, initGameboy]);

  return (
    <canvas
      ref={canvasRef}
      className="w-full h-full"
      style={{
        imageRendering: "pixelated",
        backgroundColor: "#000000",
      }}
    />
  );
};

export default GameboyDisplay;
