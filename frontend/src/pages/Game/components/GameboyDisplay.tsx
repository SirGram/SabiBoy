import { useCallback, useEffect, useRef, useState } from "react";
import { CartridgeHeaderState } from "../Emulator";
import { useGameboy } from "../../../context/GameboyContext";
import { useOptions } from "../../../context/OptionsContext";
import { useAuth } from "../../../context/AuthContext";
import api from "../../../api/client";

type GameboyDisplayProps = {
  setFps: React.Dispatch<React.SetStateAction<number>>;
  setCartridgeInfo: (info: CartridgeHeaderState) => void;
  isGameboyPaused: boolean;
  handleKeyDown: (event: KeyboardEvent) => void;
  handleKeyUp: (event: KeyboardEvent) => void;
  canvasRef: React.RefObject<HTMLCanvasElement>;
  isAudioEnabled: boolean;
  playAudioFrame: (audioContext: AudioContext, gainNode: GainNode) => void;
  volume: number;
  isDoubleSpeed: boolean;
};

export default function GameboyDisplay({
  setFps,
  setCartridgeInfo,
  isGameboyPaused,
  handleKeyDown,
  handleKeyUp,
  canvasRef,
  isAudioEnabled,
  playAudioFrame,
  volume,
  isDoubleSpeed = false,
}: GameboyDisplayProps) {
  const contextRef = useRef<CanvasRenderingContext2D | null>(null);
  const imageDataRef = useRef<ImageData | null>(null);
  const animationFrameRef = useRef<number | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const audioContextRef = useRef<AudioContext | null>(null);
  const gainNodeRef = useRef<GainNode | null>(null);
  const framesProcessedRef = useRef(0);
  const lastFrameTimeRef = useRef(0);
  const frameTimeAccumulatorRef = useRef(0);

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
    audioContextRef.current = new AudioContext();
    gainNodeRef.current = audioContextRef.current.createGain();
    gainNodeRef.current.connect(audioContextRef.current.destination);
    gainNodeRef.current.gain.value = volume / 100;

    return () => {
      gainNodeRef.current?.disconnect();
      audioContextRef.current?.close();
    };
  }, []);

  useEffect(() => {
    if (gainNodeRef.current) {
      gainNodeRef.current.gain.value = (volume / 100) * 0.1;
    }
  }, [volume]);


  useEffect(() => {
    if (!gameboy) return;

    const canvas = canvasRef.current!;
    const ctx = canvas.getContext("2d")!;

    canvas.width = 160;
    canvas.height = 144;

    contextRef.current = ctx;
    imageDataRef.current = ctx.createImageData(160, 144);

    handleCartridgeInfo();

    let lastFpsUpdate = performance.now();
    framesProcessedRef.current = 0;

    // Target frame time in milliseconds (16.67ms for 60fps)
    const BASE_FRAME_TIME = 1000 / 60;
    const MAX_FRAME_ACCUMULATOR = BASE_FRAME_TIME * 2; // Only allow up to 2 frames of lag
    
    const renderFrame = (timestamp: number) => {
      if (!gameboy || !ctx || !imageDataRef.current) return;

      if (!lastFrameTimeRef.current) {
        lastFrameTimeRef.current = timestamp;
      }

      // Calculate time since last frame
      const deltaTime = Math.min(timestamp - lastFrameTimeRef.current, 50); // Cap max delta time to 50ms
      frameTimeAccumulatorRef.current = Math.min(
        frameTimeAccumulatorRef.current + deltaTime,
        MAX_FRAME_ACCUMULATOR
      );
      
      const targetFrameTime = isDoubleSpeed ? BASE_FRAME_TIME / 2 : BASE_FRAME_TIME;
      let framesThisLoop = 0;
      const maxFramesPerLoop = isDoubleSpeed ? 4 : 2; // Limit max frames per loop
      
      while (frameTimeAccumulatorRef.current >= targetFrameTime && 
             framesThisLoop < maxFramesPerLoop) {
        gameboy.run_frame();
        framesProcessedRef.current++;
        framesThisLoop++;

        if (audioContextRef.current && gainNodeRef.current && isAudioEnabled) {
          playAudioFrame(audioContextRef.current, gainNodeRef.current);
        }

        frameTimeAccumulatorRef.current -= targetFrameTime;
      }

      // Always render the latest frame
      const frameBuffer = gameboy.get_frame_buffer();
      imageDataRef.current.data.set(frameBuffer);
      ctx.putImageData(imageDataRef.current, 0, 0);

      // Update FPS counter
      const now = performance.now();
      if (now - lastFpsUpdate >= 1000) {
        setFps(framesProcessedRef.current);
        framesProcessedRef.current = 0;
        lastFpsUpdate = now;
      }

      lastFrameTimeRef.current = timestamp;
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
  }, [
    gameboy,
    handleKeyDown,
    handleKeyUp,
    handleCartridgeInfo,
    setFps,
    isAudioEnabled,
    playAudioFrame,
    isDoubleSpeed,
  ]);

  useEffect(() => {
    if (gameboy) {
      if (isGameboyPaused) {
        gameboy.pause();
      } else {
        gameboy.resume();
      }
    }
  }, [gameboy, isGameboyPaused]);

  const { user } = useAuth();
  const [loadingState, setLoadingState] = useState<
    "rom" | "savestate" | "error" | null
  >(null);

  useEffect(() => {
    const loadEmulator = async () => {
      if (!currentGame?.rom) {
        setLoadingState("error");
        return;
      }

      try {
        let romData: Uint8Array;
        let saveStateData: Uint8Array | undefined;

        // Load ROM

        setLoadingState("rom");
        if (currentGame.rom.type === "blob" && currentGame.rom.data) {
          romData = currentGame.rom.data;
        } else {
          const { data } = await api.get(currentGame.rom.path, {
            responseType: "arraybuffer",
          });
          romData = new Uint8Array(data);
        }

        // Load save state
        setLoadingState("savestate");
        if (currentGame.saveState && user) {
          if (
            currentGame.saveState.type === "blob" &&
            currentGame.saveState.data
          ) {
            saveStateData = currentGame.saveState.data;
          } else {
            try {
              const { data } = await api.get(
                `/api/users/${user.id}/library/${currentGame.slug}/save-state`,
                { responseType: "arraybuffer" }
              );
              saveStateData = new Uint8Array(data);
            } catch (error) {
              console.error("Error loading save state:", error);
            }
          }
        }

        if (romData.length > 0) {
          try {
            console.log("Initializing GameBoy emulator...");
            await initGameboy(romData, options.palette, saveStateData);
            setLoadingState(null);
            console.log("GameBoy emulator initialized successfully.");
          } catch (initError) {
            console.error("Error initializing GameBoy emulator:", initError);
            setLoadingState("error");
          }
        }
      } catch (error) {
        console.error("Error loading the emulator:", error);
        setLoadingState("error");
      }
    };

    loadEmulator();
  }, [currentGame, options.palette, initGameboy]);

  return (
    <div
      ref={containerRef}
      className="relative w-full h-full min-w-[330px] min-h-[297px]"
    >
      <canvas
        ref={canvasRef}
        className="w-full h-full "
        style={{
          imageRendering: "pixelated",
          backgroundColor: "#000000",
          display: "block",
        }}
      />
      {loadingState && <LoadingScreen loadingState={loadingState} />}
    </div>
  );
}

const LoadingScreen = ({
  loadingState,
}: {
  loadingState: "rom" | "savestate" | null | "error";
}) => {
  const [glitterIndex, setGlitterIndex] = useState(0);
  const letters = "SABIBOY".split("");

  useEffect(() => {
    const interval = setInterval(() => {
      if (loadingState === "rom" || loadingState === "savestate") {
        setGlitterIndex((prev) => (prev + 1) % letters.length);
      }
    }, 200);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="absolute  inset-0 flex flex-col items-center justify-center bg-black">
      <div className="mb-8 flex space-x-2">
        {letters.map((letter, index) => (
          <span
            key={index}
            className={`text-4xl font-bold transition-opacity duration-200 ${
              index === glitterIndex ? "text-secondary" : "text-secondary-hover"
            }`}
          >
            {letter}
          </span>
        ))}
      </div>

      <div className="w-64">
        <div className="mb-2 text-secondary text-sm text-center">
          {loadingState === "rom" && "Loading ROM..."}
          {loadingState === "savestate" && "Loading Save State..."}
        </div>

        {loadingState === "error" ? (
          <div className="text-red-500 text-sm text-center">
            Failed to load ROM
          </div>
        ) : (
          <div className="h-2 bg-secondary-hover rounded-full overflow-hidden">
            <div
              className="h-full bg-secondary transition-all duration-300 rounded-full"
              style={{
                width: loadingState === "rom" ? "50%" : "100%",
              }}
            />
          </div>
        )}
      </div>
    </div>
  );
};
