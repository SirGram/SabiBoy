import { useCallback, useEffect, useRef } from "react";
import { CartridgeHeaderState } from "../Emulator";
import { useGameboy } from "../../../context/GameboyContext";
import { useOptions } from "../../../context/OptionsContext";
import { useAuth } from "../../../context/AuthContext";

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
};

const GameboyDisplay = ({
  setFps,
  setCartridgeInfo,
  isGameboyPaused,
  handleKeyDown,
  handleKeyUp,
  canvasRef,
  isAudioEnabled,
  playAudioFrame,
  volume,
}: GameboyDisplayProps) => {
  const contextRef = useRef<CanvasRenderingContext2D | null>(null);
  const imageDataRef = useRef<ImageData | null>(null);
  const animationFrameRef = useRef<number | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const audioContextRef = useRef<AudioContext | null>(null);
  const gainNodeRef = useRef<GainNode | null>(null);

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
  }, []); // Empty dependency array to run once

  // Handle volume changes
  useEffect(() => {
    if (gainNodeRef.current) {
      gainNodeRef.current.gain.value = volume / 100;
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

    let frameCount = 0;
    let lastFpsUpdate = performance.now();

    const renderFrame = () => {
      if (!gameboy || !ctx || !imageDataRef.current) return;

      gameboy.run_frame();

      // Only play audio if context and gain node exist and audio is enabled
      if (audioContextRef.current && gainNodeRef.current && isAudioEnabled) {
        playAudioFrame(audioContextRef.current, gainNodeRef.current);
      }

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
  }, [
    gameboy,
    handleKeyDown,
    handleKeyUp,
    handleCartridgeInfo,
    setFps,
    isAudioEnabled,
    playAudioFrame,
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

  const { fetchWithAuth, user } = useAuth();

  useEffect(() => {
    const loadEmulator = async () => {
      if (!currentGame?.rom) return;
      console.log(currentGame);

      try {
        let romData: Uint8Array;
        let saveStateData: Uint8Array | undefined;

        // Handle ROM loading
        if (currentGame.rom.type === "blob" && currentGame.rom.data) {
          romData = currentGame.rom.data;
        } else {
          console.log(`Fetching ROM from: ${currentGame.rom.path}`);
          const romResponse = await fetchWithAuth(currentGame.rom.path);
          if (!romResponse.ok) {
            console.error(`Failed to fetch ROM: ${romResponse.statusText}`);
            return;
          }
          const romArrayBuffer = await romResponse.arrayBuffer();
          romData = new Uint8Array(romArrayBuffer);
        }

        // Handle save state loading
        if (currentGame.saveState) {
          if (
            currentGame.saveState.type === "blob" &&
            currentGame.saveState.data
          ) {
            saveStateData = currentGame.saveState.data;
          } else {
            try {
              if (!user) {
                console.log("No user found when trying to load save state");
                return;
              }
              const stateResponse = await fetchWithAuth(
                `/api/users/${user.id}/library/${currentGame.slug}/save-state`
              );

              if (stateResponse.ok) {
                const stateArrayBuffer = await stateResponse.arrayBuffer();
                saveStateData = new Uint8Array(stateArrayBuffer);
                console.log("Save state loaded, size:", saveStateData.length);
              } else {
                console.error(
                  "Failed to load save state:",
                  stateResponse.status,
                  await stateResponse.text()
                );
              }
            } catch (stateError) {
              console.error("Error loading save state:", stateError);
            }
          }
        }

        if (romData.length > 0) {
          try {
            console.log("Initializing GameBoy emulator...");
            await initGameboy(romData, options.palette, saveStateData);
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
  }, [currentGame, options.palette, initGameboy, fetchWithAuth]);

  return (
    <div ref={containerRef} className="relative w-full h-full">
      <canvas
        ref={canvasRef}
        className="w-full h-full"
        style={{
          imageRendering: "pixelated",
          backgroundColor: "#000000",
          display: "block",
        }}
      />
    </div>
  );
};

export default GameboyDisplay;
