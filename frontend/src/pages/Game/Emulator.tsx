import { useCallback, useEffect, useRef, useState } from "react";
import {
  ChevronDown,
  ChevronLeft,
  ChevronUp,
  MaximizeIcon,
  PauseIcon,
  PlayIcon,
  Volume2,
  VolumeOff,
} from "lucide-react";
import { GameboyFrame } from "./components/Frame/GameboyFrame";
import GameboyDisplay from "./components/GameboyDisplay";
import { useGameboy } from "../../context/GameboyContext";
import { useNavigate } from "react-router-dom";
import { useOptions } from "../../context/OptionsContext";

export interface CartridgeHeaderState {
  title: string;
  kind: string;
  rom_size: string;
  ram_size: string;
  destination: string;
  sgb_flag: string;
  rom_version: string;
  licensee_code: string;
}
export default function Emulator() {
  const [fps, setFps] = useState<number>(0);
  const [cartridgeInfo, setCartridgeInfo] = useState<CartridgeHeaderState>({
    title: "",
    kind: "",
    rom_size: "",
    ram_size: "",
    destination: "",
    sgb_flag: "",
    rom_version: "",
    licensee_code: "",
  });
  const [isGameboyPaused, setIsGameboyPaused] = useState(false);
  const [isAudioEnabled, setIsAudioEnabled] = useState(true);

  const { gameboy } = useGameboy();
  const [pressedKeys, setPressedKeys] = useState(0xff);

  const { options } = useOptions();
  const handleSaveButton = async () => {
    try {
      const stateData = gameboy!.save_state();
      const blob = new Blob([stateData], { type: "application/octet-stream" });
      const url = URL.createObjectURL(blob);

      // Trigger download
      const a = document.createElement("a");
      a.href = url;
      a.download = `rom.gb.state`;
      a.click();
      URL.revokeObjectURL(url);

      console.log("State saved successfully.");
    } catch (error) {
      console.error("Failed to save state:", error);
    }
  };
  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      if (!gameboy) return;
      // save button
      if (event.key.toLowerCase() === "1") {
        handleSaveButton();
        return;
      }

      // Find the button that corresponds to the pressed key
      const button = Object.entries(options.keys).find(
        ([_, mapping]) => mapping.mapped === event.key
      );

      if (!button) return;
      const [, { mask }] = button;
      const newPressedKeys = pressedKeys & mask;

      event.preventDefault();
      setPressedKeys(newPressedKeys);
      gameboy.handle_keys(newPressedKeys);
    },
    [gameboy, pressedKeys, options.keys]
  );

  const handleKeyUp = useCallback(
    (event: KeyboardEvent) => {
      if (!gameboy) return;

      // Find the button that corresponds to the pressed key
      const button = Object.entries(options.keys).find(
        ([_, mapping]) => mapping.mapped === event.key
      );
      if (!button) return;

      const [, { bit }] = button;
      const newPressedKeys = pressedKeys | (1 << bit);

      event.preventDefault();
      setPressedKeys(newPressedKeys);
      gameboy.handle_keys(newPressedKeys);
    },
    [gameboy, pressedKeys]
  );

  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const displayRef = useRef<HTMLDivElement | null>(null);
  const toggleFullscreen = () => {
    const canvas = displayRef.current;
    if (canvas) {
      if (!document.fullscreenElement) {
        canvas.requestFullscreen({ navigationUI: "show" });
      } else {
        document.exitFullscreen();
      }
    }
  };
  const toggleAudio = useCallback(() => {
    if (!gameboy) return;
    gameboy.toggle_audio();

    setIsAudioEnabled(!isAudioEnabled);
  }, [gameboy, isAudioEnabled]);

  const playAudioFrame = useCallback(
    (audioContext: AudioContext) => {
      if (!isAudioEnabled) return;
      const audioSamples = gameboy!.get_audio_buffer();
      if (audioSamples.length > 0) {
        // Ensure consistent buffer creation
        const numSamples = audioSamples.length / 2;
        const buffer = audioContext.createBuffer(2, numSamples, 48000);

        const leftChannel = buffer.getChannelData(0);
        const rightChannel = buffer.getChannelData(1);

        for (let i = 0; i < numSamples; i++) {
          leftChannel[i] = audioSamples[i * 2];
          rightChannel[i] = audioSamples[i * 2 + 1];
        }

        const source = audioContext.createBufferSource();
        const gainNode = audioContext.createGain();

        source.buffer = buffer;
        source.connect(gainNode);
        gainNode.connect(audioContext.destination);

        // Set a reasonable gain to prevent potential clipping
        gainNode.gain.setValueAtTime(0.5, audioContext.currentTime);

        source.start();

        // Add error handling for audio
        source.onended = () => {
          source.disconnect();
          gainNode.disconnect();
        };
      }
    },
    [gameboy]
  );

  return (
    <div className="flex flex-col items-center justify-center min-h-screen p-4 bg-base-background">
      <BackButton />

      <div className=" mb-4">
        FPS: {fps}
        <GameboyFrame handleKeyDown={handleKeyDown} handleKeyUp={handleKeyUp}>
          <div className="group relative" ref={displayRef}>
            <div className="rounded-md group-hover:brightness-50 overflow-hidden">
              <GameboyDisplay
                setFps={setFps}
                setCartridgeInfo={setCartridgeInfo}
                isGameboyPaused={isGameboyPaused}
                handleKeyDown={handleKeyDown}
                handleKeyUp={handleKeyUp}
                canvasRef={canvasRef}
                isAudioEnabled={isAudioEnabled}
                playAudioFrame={playAudioFrame}
              />
            </div>
            <GameboyOptions
              isGameboyPaused={isGameboyPaused}
              setIsGameboyPaused={setIsGameboyPaused}
              toggleFullScreen={toggleFullscreen}
              isAudioEnabled={isAudioEnabled}
              toggleAudio={toggleAudio}
            />
          </div>
        </GameboyFrame>
      </div>
      <CartridgeInfo info={cartridgeInfo} />
      <ApuInfo
        isGameboyPaused={isGameboyPaused}
        isAudioEnabled={isAudioEnabled}
      />
    </div>
  );
}

function CartridgeInfo({ info }: { info: CartridgeHeaderState }) {
  const [isOpen, setIsOpen] = useState(false);

  const cartridgeInfoItems = [
    { label: "Title", value: info.title },
    { label: "Kind", value: info.kind },
    { label: "ROM Size", value: info.rom_size },
    { label: "RAM Size", value: info.ram_size },
    { label: "Destination", value: info.destination },
    { label: "SGB Flag", value: info.sgb_flag },
    { label: "ROM Version", value: info.rom_version },
    { label: "Licensee Code", value: info.licensee_code },
  ];

  return (
    <div className="w-full  mx-auto rounded-lg shadow-lg overflow-hidden">
      <div
        onClick={() => setIsOpen(!isOpen)}
        className="flex justify-between items-center p-4  cursor-pointer hover:bg-base-border transition-colors"
      >
        <h2 className="text-xl font-semibold text-white">
          Cartridge Information
        </h2>
        {isOpen ? (
          <ChevronUp className="text-white" />
        ) : (
          <ChevronDown className="text-white" />
        )}
      </div>

      {isOpen && (
        <div className="p-4 space-y-2">
          {cartridgeInfoItems.map((item, index) => (
            <div
              key={index}
              className="flex justify-between border-b border-gray-700 pb-2 last:border-b-0"
            >
              <span className="text-gray-300 font-medium">{item.label}:</span>
              <span className="text-white text-right">
                {item.value || "N/A"}
              </span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

function ApuInfo({
  isGameboyPaused,
  isAudioEnabled,
}: {
  isGameboyPaused: boolean;
  isAudioEnabled: boolean;
}) {
  const [isOpen, setIsOpen] = useState(false);
  const [apuChannels, setApuChannels] = useState<{
    ch1: number;
    ch2: number;
    ch3: number;
    ch4: number;
  }>({ ch1: 0.0, ch2: 0.0, ch3: 0.0, ch4: 0.0 });

  const { gameboy } = useGameboy();

  // Arrays to store previous values for smooth transition
  const historyLength = 10;
  const [ch1History, setCh1History] = useState<number[]>(
    Array(historyLength).fill(0)
  );
  const [ch2History, setCh2History] = useState<number[]>(
    Array(historyLength).fill(0)
  );
  const [ch3History, setCh3History] = useState<number[]>(
    Array(historyLength).fill(0)
  );
  const [ch4History, setCh4History] = useState<number[]>(
    Array(historyLength).fill(0)
  );

  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    if (!isAudioEnabled) {
      setCh1History(Array(historyLength).fill(0));
      setCh2History(Array(historyLength).fill(0));
      setCh3History(Array(historyLength).fill(0));
      setCh4History(Array(historyLength).fill(0));
      return;
    }
    if (!gameboy || isGameboyPaused) return;

    const updateApuChannels = () => {
      try {
        // Replace this with the actual method to fetch APU channel states
        const [ch1, ch2, ch3, ch4] = gameboy.get_apu_channels();
        setApuChannels({ ch1, ch2, ch3, ch4 });
      } catch (error) {
        console.error("Failed to fetch APU channels:", error);
      }
    };

    const interval = setInterval(updateApuChannels, 100); // Update every 100ms
    return () => clearInterval(interval);
  }, [gameboy, isGameboyPaused, isAudioEnabled]);

  // Update histories with new values
  useEffect(() => {
    setCh1History((prevHistory) => [...prevHistory.slice(1), apuChannels.ch1]);
    setCh2History((prevHistory) => [...prevHistory.slice(1), apuChannels.ch2]);
    setCh3History((prevHistory) => [...prevHistory.slice(1), apuChannels.ch3]);
    setCh4History((prevHistory) => [...prevHistory.slice(1), apuChannels.ch4]);
  }, [apuChannels]);

  const drawGraph = (ctx: CanvasRenderingContext2D) => {
    const width = ctx.canvas.width;
    const height = ctx.canvas.height;
    const midY = height / 2; // Midpoint of the canvas height (this represents the 0 point)

    ctx.clearRect(0, 0, width, height);
    ctx.lineWidth = 3;

    // Function to draw the channel graph from history
    const drawChannel = (history: number[], color: string) => {
      ctx.strokeStyle = color; // Set color for the channel
      ctx.beginPath();

      history.forEach((amplitude, index) => {
        const x = (index / history.length) * width;
        const y = midY - amplitude * midY;
        if (index === 0) {
          ctx.moveTo(x, y);
        } else {
          ctx.lineTo(x, y);
        }
      });

      ctx.stroke();
    };

    // Draw all channels with their respective histories
    drawChannel(ch1History, "red");
    drawChannel(ch2History, "blue");
    drawChannel(ch3History, "green");
    drawChannel(ch4History, "purple");
  };

  useEffect(() => {
    if (canvasRef.current) {
      const ctx = canvasRef.current.getContext("2d");
      if (ctx) {
        drawGraph(ctx);
      }
    }
  }, [ch1History, ch2History, ch3History, ch4History]);

  return (
    <div className="w-full mx-auto rounded-lg shadow-lg overflow-hidden">
      <div
        onClick={() => setIsOpen(!isOpen)}
        className="flex justify-between items-center p-4 cursor-pointer hover:bg-base-border transition-colors"
      >
        <h2 className="text-xl font-semibold text-white">APU Information</h2>
        {isOpen ? (
          <ChevronUp className="text-white" />
        ) : (
          <ChevronDown className="text-white" />
        )}
      </div>

      {isOpen && (
        <div className="flex items-center gap-6 justify-center p-4 space-y-2">
          <div className="font-extrabold">
            <p className="text-[red] ">Channel 1</p>
            <p className="text-[blue]">Channel 2</p>
            <p className="text-[green]">Channel 3</p>
            <p className="text-[purple]">Channel 4</p>
          </div>
          <div>
            <canvas ref={canvasRef} width={300} height={100}></canvas>
          </div>
        </div>
      )}
    </div>
  );
}

function BackButton() {
  //use navigate
  const navigate = useNavigate();
  return (
    <div className="absolute top-0 left-0 p-6">
      <button
        className=" text-xl text-white font-bold "
        onClick={() => navigate("/")}
      >
        <ChevronLeft />
      </button>
    </div>
  );
}

function GameboyOptions({
  isGameboyPaused,
  setIsGameboyPaused,
  toggleFullScreen,
  isAudioEnabled,
  toggleAudio,
}: {
  isGameboyPaused: boolean;
  setIsGameboyPaused: React.Dispatch<React.SetStateAction<boolean>>;
  toggleFullScreen: () => void;
  isAudioEnabled: boolean;
  toggleAudio: () => void;
}) {
  return (
    <div className="absolute inset-0 z-10 hidden group-hover:block ">
      <div className="absolute bottom-2 left-2 right-2 flex justify-between items-center text-sm text-white font-bold">
        <div>
          <button
            className=" p-1 rounded hover:bg-primary"
            onClick={() => setIsGameboyPaused(!isGameboyPaused)}
          >
            {isGameboyPaused ? <PlayIcon /> : <PauseIcon />}
          </button>
          <button
            className=" p-1 rounded hover:bg-primary"
            onClick={() => toggleAudio()}
          >
            {isAudioEnabled ? <Volume2 /> : <VolumeOff />}
          </button>
        </div>
        <button
          className=" p-1 rounded hover:bg-primary"
          onClick={toggleFullScreen}
        >
          <MaximizeIcon size={25} />
        </button>
      </div>
    </div>
  );
}
