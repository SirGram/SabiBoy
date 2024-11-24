import React, { useEffect, useRef } from 'react';
import init, { GameboyWasm } from '../wasm/pkg/gameboy_wasm';

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
        if (!canvas) throw new Error('Canvas not found');
        
        const ctx = canvas.getContext('2d');
        if (!ctx) throw new Error('Could not get canvas context');
        
        // Set up canvas dimensions
        canvas.width = 160;  // GameBoy native resolution
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
        console.error('GameBoy initialization failed:', error);
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
          console.error('Error during frame render:', error);
          return;
        }
        
        // Schedule next frame
        animationFrameRef.current = requestAnimationFrame(renderFrame);
      };

      renderFrame();
    };

    initGameboy();

    // Cleanup function
    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
      if (gameboyRef.current) {
        gameboyRef.current.free();
      }
    };
  }, []);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 p-4">
      <canvas
        ref={canvasRef}
        className="border-4 border-gray-700 rounded-lg shadow-lg"
        style={{
          imageRendering: 'pixelated',
          width: '480px',      // 160 * 3
          height: '432px',     // 144 * 3
          backgroundColor: '#9BA4B5'
        }}
      />
    </div>
  );
};

export default GameboyDisplay;