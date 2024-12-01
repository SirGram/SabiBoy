import { useCallback, useEffect, useRef, useState } from "react";
import { CartridgeHeaderState } from "../Emulator";
import { useGameboy } from "../../../context/GameboyContext";

type GameboyDisplayProps = {
  setFps: React.Dispatch<React.SetStateAction<number>>;
  setCartridgeInfo: (info: CartridgeHeaderState) => void;
  isGameboyPaused: boolean;
  handleKeyDown: (event: KeyboardEvent) => void;
  handleKeyUp: (event: KeyboardEvent) => void;
  canvasRef: React.RefObject<HTMLCanvasElement>;
};
const GameboyDisplay = ({
  setFps,
  setCartridgeInfo,
  isGameboyPaused,
  handleKeyDown,
  handleKeyUp,
  canvasRef,
}: GameboyDisplayProps) => {
  const contextRef = useRef<CanvasRenderingContext2D | null>(null);
  const imageDataRef = useRef<ImageData | null>(null);
  const animationFrameRef = useRef<number | null>(null);

  const { gameboy, initGameboy, currentGame } = useGameboy();

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
      const response = await fetch(currentGame?.romPath!);
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
    console.log(currentGame);
  }, [currentGame]);

  return (
    <canvas
      ref={canvasRef}
      className=" h-full w-full "
      style={{
        imageRendering: "pixelated",
        backgroundColor: "#000000",
      }}
    />
  );
};

export default GameboyDisplay;
