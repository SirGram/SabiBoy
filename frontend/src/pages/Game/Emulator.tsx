import { ChangeEvent, useCallback, useEffect, useRef, useState } from "react";
import {
  ChevronLeft,
  DownloadIcon,
  MaximizeIcon,
  PauseIcon,
  PlayIcon,
  SaveIcon,
  SaveOff,
  Volume2,
  VolumeOff,
} from "lucide-react";
import { GameboyFrame } from "./components/Frame/GameboyFrame";
import GameboyDisplay from "./components/GameboyDisplay";
import { useGameboy } from "../../context/GameboyContext";
import { useNavigate } from "react-router-dom";
import { useOptions } from "../../context/OptionsContext";
import { usePreventDefaultTouch } from "../../hooks/hooks";
import { WasmPpuState } from "../../wasm/pkg/gameboy_wasm";
import { useAuth } from "../../context/AuthContext";
import api from "../../api/client";

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

  const { gameboy, currentGame } = useGameboy();
  const [pressedKeys, setPressedKeys] = useState(0xff);

  const { options } = useOptions();
  const [isAudioEnabled, setIsAudioEnabled] = useState(!options.muteOnStart);

  const [volume, setVolume] = useState(!options.muteOnStart ? 100 : 0);
  const { user } = useAuth();

  const handleSaveButton = async () => {
    try {
      const stateData = gameboy!.save_state();
      await api.patch(
        `/api/users/${user?.id}/library/${currentGame?.slug}/save-state`,
        stateData,
        {
          headers: { "Content-Type": "application/octet-stream" },
        }
      );
    } catch (error) {
      console.error("Failed to save state:", error);
      throw error;
    }
  };
  const handleDownloadSaveState = async () => {
    try {
      if (!gameboy) return;
      const stateData = gameboy!.save_state();
      console.log("Save state data generated, size:", stateData.length);

      // Download functionality
      const blob = new Blob([stateData], { type: "application/octet-stream" });
      const url = URL.createObjectURL(blob);
      const link = document.createElement("a");
      link.href = url;
      link.download = `rom.gb.state`;
      link.click();
      URL.revokeObjectURL(url);
    } catch (error) {
      console.error("Failed to save state:", error);
      throw error;
    }
  };
  const handleResetSaveState = async () => {
    try {
      await api.delete(
        `/api/users/${user?.id}/library/${currentGame?.slug}/save-state`
      );
      navigate("/");
    } catch (error) {
      console.error("Failed to reset save state:", error);
      throw error;
    }
  };
  const navigate = useNavigate();
  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      if (!gameboy) return;
      console.log(event);
      // Handle save button first
      if (
        event.key.toLowerCase() === options.keys?.Save?.mapped?.toLowerCase()
      ) {
        handleSaveButton();
        return;
      }

      // Handle other GameBoy buttons
      const button = Object.entries(options.keys).find(
        ([buttonName, mapping]) =>
          buttonName !== "Save" && mapping.mapped === event.key
      );

      if (!button) return;
      const [, { mask }] = button;
      const newPressedKeys = pressedKeys & mask!;

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
      const newPressedKeys = pressedKeys | (1 << bit!);

      event.preventDefault();
      setPressedKeys(newPressedKeys);
      gameboy.handle_keys(newPressedKeys);
    },
    [gameboy, pressedKeys]
  );

  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const displayRef = useRef<HTMLDivElement | null>(null);
  const toggleFullscreen = () => {
    const display = displayRef.current;
    if (display) {
      if (!document.fullscreenElement) {
        display
          .requestFullscreen({ navigationUI: "hide" })
          /*   .then(() => setIsFullscreen(true)) */
          .catch((err) => {
            console.error(
              `Error attempting to enable fullscreen: ${err.message}`
            );
          });
      } else {
        document
          .exitFullscreen()
          /*  .then(() => setIsFullscreen(false)) */
          .catch((err) => {
            console.error(
              `Error attempting to exit fullscreen: ${err.message}`
            );
          });
      }
    }
  };

  const toggleAudio = useCallback(() => {
    if (!gameboy) return;

    const newAudioEnabled = !isAudioEnabled;
    setIsAudioEnabled(newAudioEnabled);
    gameboy.toggle_audio();

    if (newAudioEnabled) {
      setVolume(100);
    } else {
      setVolume(0);
    }
  }, [gameboy, isAudioEnabled]);

  const updateVolume = (newVolume: number) => {
    if (newVolume === 0) {
      if (isAudioEnabled) {
        setIsAudioEnabled(false);
        gameboy?.toggle_audio();
      }
    } else if (!isAudioEnabled) {
      setIsAudioEnabled(true);
      gameboy?.toggle_audio();
    }
    setVolume(newVolume);
  };
  useEffect(() => {
    if (gameboy && !isAudioEnabled) {
      gameboy.toggle_audio(); // Sync initial state with emulator
    }
  }, [gameboy]);
  const playAudioFrame = useCallback(
    (audioContext: AudioContext, gainNode: GainNode) => {
      if (!isAudioEnabled || !gameboy) return;

      const audioSamples = gameboy.get_audio_buffer();
      if (audioSamples.length === 0) return;

      const numSamples = audioSamples.length / 2;
      const sampleRate = 48000;

      const buffer = audioContext.createBuffer(2, numSamples, sampleRate);
      const leftChannel = buffer.getChannelData(0);
      const rightChannel = buffer.getChannelData(1);

      for (let i = 0; i < numSamples; i++) {
        leftChannel[i] = audioSamples[i * 2];
        rightChannel[i] = audioSamples[i * 2 + 1];
      }

      const source = audioContext.createBufferSource();
      source.buffer = buffer;

      // Connect to the persistent gain node
      source.connect(gainNode);

      try {
        source.start();
      } catch (error) {
        console.error("Failed to start audio source:", error);
      }

      source.onended = () => source.disconnect();
    },
    [gameboy, isAudioEnabled]
  );
  usePreventDefaultTouch();
  const [isDoubleSpeed, setIsDoubleSpeed] = useState(false);

  const handleDoubleSpeed = () => {
    setIsDoubleSpeed(!isDoubleSpeed);
    console.log(isDoubleSpeed);
  };
  return (
    <div className="flex flex-col items-center justify-center h-full min-h-screen md:p-4 bg-base-background">
      <BackButton />

      <div className="w-full   rounded-lg shadow-md pt-6 ">
        <div
          className={
            options.debug
              ? "grid grid-cols-1 md:grid-cols-6 lg:grid-cols-10 gap-6"
              : "flex w-full  justify-center"
          }
        >
          <div className="lg:col-span-2">
            <div className="  p-4 ">
              <GameboyFrame
                handleKeyDown={handleKeyDown}
                handleKeyUp={handleKeyUp}
              >
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
                      volume={volume}
                      isDoubleSpeed={isDoubleSpeed}
                    />
                  </div>
                  <GameboyOptions
                    isGameboyPaused={isGameboyPaused}
                    setIsGameboyPaused={setIsGameboyPaused}
                    toggleFullScreen={toggleFullscreen}
                    isAudioEnabled={isAudioEnabled}
                    toggleAudio={toggleAudio}
                    fps={fps}
                    updateVolume={updateVolume}
                    volume={volume}
                    handleSaveButton={handleSaveButton}
                    handleDownloadSaveState={handleDownloadSaveState}
                    handleResetSaveState={handleResetSaveState}
                    handleDoubleSpeed={handleDoubleSpeed}
                    isDoubleSpeed={isDoubleSpeed}
                  />
                </div>
              </GameboyFrame>
            </div>
          </div>
          {options.debug && (
            <>
              <div className="flex flex-col lg:col-span-1">
                <div className=" rounded-lg p-4">
                  <JoypadInfo isGameboyPaused={isGameboyPaused} />
                </div>
              </div>
              <div className="flex flex-col md:col-span-2 lg:col-span-3">
                <div className=" rounded-lg p-4">
                  <BusInfo isGameboyPaused={isGameboyPaused} />
                </div>
              </div>
              <div className="flex flex-col lg:col-span-3">
                <div className=" rounded-lg p-4">
                  <ApuInfo
                    isGameboyPaused={isGameboyPaused}
                    isAudioEnabled={isAudioEnabled}
                  />
                </div>
              </div>
              <div className="md:col-span-1 flex flex-col lg:col-span-2">
                <div className=" rounded-lg p-4">
                  <CartridgeInfo info={cartridgeInfo} />
                </div>
              </div>
              <div className="md:col-span-2 flex flex-col">
                <div className=" rounded-lg p-4">
                  <CpuInfo isGameboyPaused={isGameboyPaused} />
                </div>
              </div>
              <div className="flex flex-col md:col-span-2">
                <div className=" rounded-lg p-4">
                  <PpuInfo isGameboyPaused={isGameboyPaused} />
                </div>
              </div>
              <div className="md:col-span-2 flex flex-col">
                <div className=" rounded-lg p-4">
                  <TimerInfo isGameboyPaused={isGameboyPaused} />
                </div>
              </div>{" "}
            </>
          )}
        </div>
      </div>
    </div>
  );
}

function BackButton() {
  const navigate = useNavigate();
  const { setCurrentGame, gameboy } = useGameboy();
  return (
    <div className="fixed top-0 left-0 p-6">
      <button
        className="   font-bold text-muted hover:text-base-foreground "
        onClick={() => {
          gameboy?.reset();
          setCurrentGame(null);

          navigate("/");
        }}
      >
        <ChevronLeft size={30} />
      </button>
    </div>
  );
}

function GameboyOptions({
  fps,
  isGameboyPaused,
  setIsGameboyPaused,
  toggleFullScreen,
  isAudioEnabled,
  toggleAudio,
  updateVolume,
  volume,
  handleSaveButton,
  handleDownloadSaveState,
  handleResetSaveState,
  handleDoubleSpeed,
  isDoubleSpeed,
}: {
  fps: number;
  isGameboyPaused: boolean;
  setIsGameboyPaused: React.Dispatch<React.SetStateAction<boolean>>;
  toggleFullScreen: () => void;
  isAudioEnabled: boolean;
  toggleAudio: () => void;
  updateVolume: (newVolume: number) => void;
  volume: number;
  handleSaveButton: () => void;
  handleDownloadSaveState: () => void;
  handleResetSaveState: () => void;
  handleDoubleSpeed: () => void;
  isDoubleSpeed: boolean;
}) {
  const { isAuthenticated } = useAuth();
  const handleResetWithConfirmation = async () => {
    const userConfirmed = window.confirm(
      "Are you sure you want to reset the save state? This action cannot be undone."
    );
    if (userConfirmed) {
      handleResetSaveState();
    }
  };

  return (
    <div className="absolute inset-0 z-10 hidden group-hover:block text-primary-foreground">
      <div className="absolute top-2 right-2  font-semibold flex gap-2">
        <button
          title="Double Speed"
          className={`${
            isDoubleSpeed ? "bg-secondary" : "bg-transparent"
          } hover:bg-primary p-2 rounded
          flex gap-1 justify-center items-center`}
          onClick={handleDoubleSpeed}
        >
          x2
          <span className="text-xs">SP</span>
        </button>
        <span className="p-2 flex gap-1 justify-center items-center">
          {fps}
          <span className="text-xs">FPS</span>
        </span>
      </div>
      <div className="absolute top-2 left-2 flex gap-2">
        {isAuthenticated && (
          <>
            <button
              className="p-2 rounded hover:bg-primary"
              title="Save State to Cloud"
              onClick={handleSaveButton}
            >
              <SaveIcon />
            </button>
            <button
              className="p-2 rounded hover:bg-primary"
              title="Reset Save State from Cloud"
              onClick={handleResetWithConfirmation}
            >
              <SaveOff />
            </button>
          </>
        )}
        <button
          className="p-2 rounded hover:bg-primary"
          title="Download Save State"
          onClick={handleDownloadSaveState}
        >
          <DownloadIcon />
        </button>
      </div>
      <div className="absolute bottom-2 left-2 right-2 flex justify-between items-center text-sm  font-bold">
        <div className="flex gap-2 items-center">
          <button
            className=" p-2 rounded hover:bg-primary "
            onClick={() => setIsGameboyPaused(!isGameboyPaused)}
            title={isGameboyPaused ? "Resume" : "Pause"}
          >
            {isGameboyPaused ? <PlayIcon /> : <PauseIcon />}
          </button>
          <button
            className=" p-2 rounded hover:bg-primary"
            onClick={() => toggleAudio()}
            title={isAudioEnabled ? "Disable Audio" : "Enable Audio"}
          >
            {isAudioEnabled ? <Volume2 /> : <VolumeOff />}
          </button>{" "}
          <input
            type="range"
            min="0"
            max="100"
            value={volume}
            onChange={(e) => updateVolume(Number(e.target.value))}
            className="w-20 h-1 bg-gray-600 rounded-lg appearance-none cursor-pointer"
            style={{
              backgroundImage: `linear-gradient(to right, white ${volume}%, gray ${volume}%)`,
            }}
          />
        </div>
        <button
          className=" p-2 rounded hover:bg-primary"
          onClick={toggleFullScreen}
          title="Toggle Fullscreen"
        >
          <MaximizeIcon size={25} />
        </button>
      </div>
    </div>
  );
}

function CartridgeInfo({ info }: { info: CartridgeHeaderState }) {
  const [isOpen, setIsOpen] = useState(true);

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
    <div className="w-fiit  mx-auto rounded-lg shadow-lg overflow-hidden border border-base-border">
      <div
        onClick={() => setIsOpen(!isOpen)}
        className="flex justify-between items-center p-4  cursor-pointer border-b border-base-border transition-colors"
      >
        <h2 className="text-xl font-semibold ">CARTRIDGE</h2>
        {isOpen ? <span>▼</span> : <span>▲</span>}
      </div>

      {isOpen && (
        <div className=" space-y-2">
          {cartridgeInfoItems.map((item, index) => (
            <div
              key={index}
              className="flex gap-4 px-4 py-1 justify-between border-b border-base-border pb-2 last:border-b-0"
            >
              <span className="text-muted font-medium">{item.label}:</span>
              <span className=" text-right">{item.value || "N/A"}</span>
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
  const [isOpen, setIsOpen] = useState(true);
  const [apuChannels, setApuChannels] = useState<{
    ch1: number;
    ch2: number;
    ch3: number;
    ch4: number;
  }>({ ch1: 0.0, ch2: 0.0, ch3: 0.0, ch4: 0.0 });
  const [apuChannelsEnabled, setApuChannelsEnabled] = useState<{
    ch1: boolean;
    ch2: boolean;
    ch3: boolean;
    ch4: boolean;
  }>({ ch1: true, ch2: true, ch3: true, ch4: true });

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
        const apuState = gameboy.get_apu_state();
        let ch1 = apuState.current_ch1_output;
        let ch2 = apuState.current_ch2_output;
        let ch3 = apuState.current_ch3_output;
        let ch4 = apuState.current_ch4_output;
        setApuChannels({ ch1, ch2, ch3, ch4 });
        setApuChannelsEnabled({
          ch1: apuState.ch1_enabled,
          ch2: apuState.ch2_enabled,
          ch3: apuState.ch3_enabled,
          ch4: apuState.ch4_enabled,
        });
      } catch (error) {
        console.error("Failed to fetch APU channels:", error);
      }
    };

    const interval = setInterval(updateApuChannels, 100);
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
    const midY = height / 2; // 0 point

    ctx.clearRect(0, 0, width, height);
    ctx.lineWidth = 3;

    const drawChannel = (history: number[], color: string) => {
      ctx.strokeStyle = color;
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

  const updateChannelEnable = (channel: number) => () => {
    if (gameboy) {
      gameboy.toggle_channel(channel);
    }
  };

  return (
    <div className="w-fit mx-auto rounded-lg shadow-lg overflow-hidden border-base-border border">
      <div
        onClick={() => setIsOpen(!isOpen)}
        className="flex justify-between items-center p-4 cursor-pointer border-b border-base-border transition-colors"
      >
        <h2 className="text-xl font-semibold ">APU</h2>
        {isOpen ? <span>▼</span> : <span>▲</span>}
      </div>

      {isOpen && (
        <div className="flex items-center gap-2 justify-center p-4 space-y-2 ">
          <div className="font-extrabold">
            <p className="flex gap-2 w-full">
              <input
                type="checkbox"
                checked={apuChannelsEnabled.ch1}
                onChange={updateChannelEnable(1)}
              />
              <span className="text-[red] flex-grow">Channel 1</span>
            </p>
            <p className="flex gap-2">
              <input
                type="checkbox"
                checked={apuChannelsEnabled.ch2}
                onChange={updateChannelEnable(2)}
              />
              <span className="text-[blue]">Channel 2</span>
            </p>
            <p className="flex gap-2">
              <input
                type="checkbox"
                checked={apuChannelsEnabled.ch3}
                onChange={updateChannelEnable(3)}
              />
              <span className="text-[green]">Channel 3</span>
            </p>
            <p className="flex gap-2">
              <input
                type="checkbox"
                checked={apuChannelsEnabled.ch4}
                onChange={updateChannelEnable(4)}
              />
              <span className="text-[purple]">Channel 4</span>
            </p>
          </div>
          <div>
            <canvas ref={canvasRef} width={300} height={100}></canvas>
          </div>
        </div>
      )}
    </div>
  );
}
const CpuInfo = ({ isGameboyPaused }: { isGameboyPaused: boolean }) => {
  const [isOpen, setIsOpen] = useState(true);
  const [cpuState, setCpuState] = useState<{
    a: number;
    b: number;
    c: number;
    d: number;
    e: number;
    h: number;
    l: number;
    f: number;
    sp: number;
    pc: number;
    ime: boolean;
    halt: boolean;
    cycles: number;
  }>({
    a: 0,
    b: 0,
    c: 0,
    d: 0,
    e: 0,
    h: 0,
    l: 0,
    f: 0,
    sp: 0,
    pc: 0,
    ime: false,
    halt: false,
    cycles: 0,
  });

  const { gameboy } = useGameboy();

  useEffect(() => {
    if (!gameboy || isGameboyPaused) return;

    const updateCpuState = () => {
      try {
        const cpuState = gameboy.get_cpu_state();
        setCpuState(cpuState);
      } catch (error) {
        console.error("Failed to fetch CPU state:", error);
      }
    };

    const interval = setInterval(updateCpuState, 100); // Update every 100ms
    return () => clearInterval(interval);
  }, [gameboy, isGameboyPaused]);

  return (
    <div className="w-fit mx-auto rounded-lg shadow-lg overflow-hidden border border-base-border">
      <div
        onClick={() => setIsOpen(!isOpen)}
        className="flex justify-between items-center p-4 cursor-pointer  transition-colors"
      >
        <h2 className="text-xl font-semibold">CPU</h2>
        {isOpen ? <span>▼</span> : <span>▲</span>}
      </div>

      {isOpen && (
        <div className="grid grid-cols-3 gap-4  w-full border-t  border-base-border px-2">
          <div className="flex flex-col border-r border-base-border pr-4 ">
            <span className="font-medium text-center">Registers</span>
            <div className="flex flex-col space-y-2 mt-2">
              <div className="flex justify-between">
                <span className="">A:</span>
                <span className="">
                  {cpuState.a.toString(16).padStart(2, "0").toUpperCase()}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="">B:</span>
                <span className="">
                  {cpuState.b.toString(16).padStart(2, "0").toUpperCase()}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="">C:</span>
                <span className="">
                  {cpuState.c.toString(16).padStart(2, "0").toUpperCase()}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="">D:</span>
                <span className="">
                  {cpuState.d.toString(16).padStart(2, "0").toUpperCase()}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="">E:</span>
                <span className="">
                  {cpuState.e.toString(16).padStart(2, "0").toUpperCase()}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="">H:</span>
                <span className="">
                  {cpuState.h.toString(16).padStart(2, "0").toUpperCase()}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="">L:</span>
                <span className="">
                  {cpuState.l.toString(16).padStart(2, "0").toUpperCase()}
                </span>
              </div>
            </div>
          </div>
          <div className="flex flex-col border-r border-base-border pr-4">
            <span className="font-medium text-center">Flags</span>
            <div className="flex flex-col space-y-2 mt-2">
              <div className="flex justify-between gap-2">
                <span className="">Zero</span>
                <span className="">
                  {(cpuState.f & 0b1000_0000) >> 7 ? "✓" : "✗"}
                </span>
              </div>
              <div className="flex justify-between gap-2">
                <span className="">Subtract</span>
                <span className="">
                  {(cpuState.f & 0b0100_0000) >> 6 ? "✓" : "✗"}
                </span>
              </div>
              <div className="flex justify-between gap-2">
                <span className="">Half Carry</span>
                <span className="">
                  {(cpuState.f & 0b0010_0000) >> 5 ? "✓" : "✗"}
                </span>
              </div>
              <div className="flex justify-between gap-2">
                <span className="">Carry</span>
                <span className="">
                  {(cpuState.f & 0b0000_0010) >> 1 ? "✓" : "✗"}
                </span>
              </div>
            </div>
          </div>
          <div className="flex flex-col">
            <span className="font-medium text-center">State</span>
            <div className="flex flex-col space-y-2 mt-2">
              <div className="flex justify-between">
                <span className="">SP:</span>
                <span className="">
                  {cpuState.sp.toString(16).padStart(4, "0").toUpperCase()}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="">PC:</span>
                <span className="">
                  {cpuState.pc.toString(16).padStart(4, "0").toUpperCase()}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="">IME:</span>
                <span className="">{cpuState.ime ? "✓" : "✗"}</span>
              </div>
              <div className="flex justify-between">
                <span className="">Halt:</span>
                <span className="">{cpuState.halt ? "✓" : "✗"}</span>
              </div>
              <div className="flex justify-between">
                <span className="">Cycles:</span>
                <span className="">{cpuState.cycles}</span>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
const TimerInfo = ({ isGameboyPaused }: { isGameboyPaused: boolean }) => {
  const [isOpen, setIsOpen] = useState(true);
  const [timerState, setTimerState] = useState<TimerState>();

  const { gameboy } = useGameboy();
  type TimerState = {
    div_counter: number;
    tima_counter: number;
  };

  useEffect(() => {
    if (!gameboy || isGameboyPaused) return;

    const updateTimerState = () => {
      try {
        const new_state = gameboy.get_timer_state();
        setTimerState(new_state);
      } catch (error) {
        console.error("Failed to fetch state:", error);
      }
    };

    const interval = setInterval(updateTimerState, 100);
    return () => clearInterval(interval);
  }, [gameboy, isGameboyPaused]);

  return (
    <div className="w-fit mx-auto rounded-lg shadow-lg overflow-hidden border border-base-border">
      <div
        onClick={() => setIsOpen(!isOpen)}
        className="flex justify-between items-center p-4 cursor-pointer  transition-colors"
      >
        <h2 className="text-xl font-semibold">TIMER</h2>
        {isOpen ? <span>▼</span> : <span>▲</span>}
      </div>

      {isOpen && (
        <div className="flex flex-col  w-full border-t  border-base-border ">
          <div className="w-full flex justify-between items-center border-b border-base-border  p-2">
            <span className="font-medium text-center ">DIV:</span>
            <span className=" space-y-2 ">{timerState?.div_counter}</span>
          </div>
          <div className="w-full flex justify-between items-center border-b border-base-border  p-2">
            <span className="font-medium text-center ">TIMA:</span>
            <span className=" space-y-2 ">{timerState?.tima_counter}</span>
          </div>
        </div>
      )}
    </div>
  );
};
type JoypadState = {
  register: number;
  keys: number;
};
const JoypadInfo = ({ isGameboyPaused }: { isGameboyPaused: boolean }) => {
  const [isOpen, setIsOpen] = useState(true);
  const { gameboy } = useGameboy();
  const [joypadState, setJoypadState] = useState<JoypadState>({
    register: 0,
    keys: 0,
  });

  const KEY_MAPPING = [
    { key: "Right", bit: 0x01, code: "RIGHT" },
    { key: "Left", bit: 0x02, code: "LEFT" },
    { key: "Up", bit: 0x04, code: "UP" },
    { key: "Down", bit: 0x08, code: "DOWN" },
    { key: "A", bit: 0x10, code: "A" },
    { key: "B", bit: 0x20, code: "B" },
    { key: "Select", bit: 0x40, code: "SELECT" },
    { key: "Start", bit: 0x80, code: "START" },
  ];

  useEffect(() => {
    if (!gameboy || isGameboyPaused) return;
    const updateJoypadState = () => {
      try {
        const busState = gameboy.get_bus_state();
        setJoypadState(busState.joypad);
      } catch (error) {
        console.error("Failed to fetch state:", error);
      }
    };
    const interval = setInterval(updateJoypadState, 100);
    return () => clearInterval(interval);
  }, [gameboy, isGameboyPaused]);

  return (
    <div className="w-fit mx-auto rounded-lg shadow-lg overflow-hidden border border-base-border">
      <div
        onClick={() => setIsOpen(!isOpen)}
        className="flex justify-between items-center p-4 cursor-pointer transition-colors"
      >
        <h2 className="text-xl font-semibold">JOYPAD</h2>
        {isOpen ? <span>▼</span> : <span>▲</span>}
      </div>
      {isOpen && (
        <div className="border-t border-base-border ">
          <div className="flex gap-2 items-center justify-between px-2  py-1">
            <span className="text-sm font-semibold">FF00:</span>
            {joypadState.register.toString(2).padStart(8, "0")}
          </div>
          <table className="w-full border-collapse ">
            <tbody>
              {KEY_MAPPING.map((keyInfo) => (
                <tr key={keyInfo.key} className="text-center">
                  <td className="border-b border-t p-2 border-base-border">
                    {keyInfo.key}
                  </td>
                  <td className="border p-2 border-base-border">
                    <span
                      className={`inline-block w-4 h-4 rounded-full ${
                        joypadState && joypadState.keys & keyInfo.bit
                          ? "bg-green-500"
                          : "bg-red-500"
                      }`}
                    />
                  </td>{" "}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
};
const PpuInfo = ({ isGameboyPaused }: { isGameboyPaused: boolean }) => {
  const [isOpen, setIsOpen] = useState(true);
  const [ppuState, setPpuState] = useState<WasmPpuState | null>(null);

  const { gameboy } = useGameboy();

  useEffect(() => {
    if (!gameboy || isGameboyPaused) return;

    const updatePpuState = () => {
      try {
        const newState = gameboy.get_ppu_state();
        setPpuState(newState);
      } catch (error) {
        console.error("Failed to fetch state:", error);
      }
    };

    const interval = setInterval(updatePpuState, 100);
    return () => clearInterval(interval);
  }, [gameboy, isGameboyPaused]);

  const handleSpriteDebugToggle = (event: ChangeEvent<HTMLInputElement>) => {
    if (gameboy) {
      gameboy.toggle_sprite_debug_mode(event.target.checked);
    }
  };

  const handleWindowDebugToggle = (event: ChangeEvent<HTMLInputElement>) => {
    if (gameboy) {
      gameboy.toggle_window_debug_mode(event.target.checked);
    }
  };

  return (
    <div className="w-fit mx-auto rounded-lg shadow-lg overflow-hidden border border-base-border">
      <div
        onClick={() => setIsOpen(!isOpen)}
        className="flex justify-between items-center p-4 cursor-pointer hover:bg-base-hover transition-colors"
      >
        <h2 className="text-xl font-semibold">PPU</h2>
        {isOpen ? <span>▼</span> : <span>▲</span>}
      </div>

      {isOpen && ppuState && (
        <div className="border-t border-base-border p-4 space-y-4">
          <div className="space-y-2">
            <h3 className="font-semibold text-lg pb-1">
              Mode & Frame Information
            </h3>
            <div className="grid grid-cols-2 gap-x-2 text-sm">
              <div className="flex justify-between">
                <span>Mode:</span>
                <span>{ppuState.mode}</span>
              </div>
              <div className="flex justify-between">
                <span>Cycles:</span>
                <span>{ppuState.mode_cycles}</span>
              </div>
              <div className="flex justify-between">
                <span>Window Triggered:</span>
                <span>{ppuState.window_triggered_this_frame ? "✓" : "✗"}</span>
              </div>
              <div className="flex justify-between">
                <span>New Frame:</span>
                <span>{ppuState.new_frame ? "✓" : "✗"}</span>
              </div>
              <div className="flex justify-between">
                <span>Render X:</span>
                <span>{ppuState.x_render_counter}</span>
              </div>
              <div className="flex justify-between">
                <span>Window Line Increment:</span>
                <span>
                  {ppuState.window_line_counter_incremented_this_scanline
                    ? "✓"
                    : "✗"}
                </span>
              </div>
            </div>
          </div>
          <div className="flex w-full gap-x-4">
            <div className="space-y-2 w-full">
              <h3 className="font-semibold text-lg pb-1">BG/W Fetcher</h3>
              <div className="flex flex-col gap-x-2 text-sm">
                <div className="flex justify-between">
                  <span>Step:</span>
                  <span>{ppuState.fetcher.step}</span>
                </div>
                <div className="flex justify-between">
                  <span>Window Fetch:</span>
                  <span>{ppuState.fetcher.is_window_fetch ? "✓" : "✗"}</span>
                </div>
                <div className="flex justify-between">
                  <span>X Position:</span>
                  <span>{ppuState.fetcher.x_pos_counter}</span>
                </div>
                <div className="flex justify-between">
                  <span>Window Line:</span>
                  <span>{ppuState.fetcher.window_line_counter}</span>
                </div>
                <div className="flex justify-between">
                  <span>Fetcher Paused:</span>
                  <span>{ppuState.fetcher.pause ? "✓" : "✗"}</span>
                </div>
              </div>
            </div>
            <div className="space-y-2 w-full">
              <h3 className="font-semibold text-lg pb-1">Sprite Fetcher</h3>
              <div className="flex flex-col gap-x-2 text-sm">
                <div className="flex justify-between">
                  <span>Active:</span>
                  <span>{ppuState.sprite_fetcher.active ? "✓" : "✗"}</span>
                </div>
                <div className="flex justify-between">
                  <span>Remaining Pixels:</span>
                  <span>{ppuState.sprite_fetcher.remaining_pixels}</span>
                </div>
                <div className="flex justify-between">
                  <span>Sprite Y:</span>
                  <span>{ppuState.sprite_fetcher.sprite.y_pos}</span>
                </div>
                <div className="flex justify-between">
                  <span>Sprite X:</span>
                  <span>{ppuState.sprite_fetcher.sprite.x_pos}</span>
                </div>
              </div>
            </div>
          </div>

          <div className="space-y-2">
            <h3 className="font-semibold text-lg pb-1">Debug Configuration</h3>
            <div className="flex justify-between items-center">
              <span>Sprites Enable</span>
              <input
                type="checkbox"
                checked={ppuState.debug_config.sprite_debug_enabled}
                onChange={handleSpriteDebugToggle}
                className="toggle-checkbox"
              />
            </div>
            <div className="flex justify-between items-center">
              <span>Window Enable</span>
              <input
                type="checkbox"
                checked={ppuState.debug_config.window_debug_enabled}
                onChange={handleWindowDebugToggle}
                className="toggle-checkbox"
              />
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

const BusInfo = ({ isGameboyPaused }: { isGameboyPaused: boolean }) => {
  const [isOpen, setIsOpen] = useState(true);
  const [activeTab, setActiveTab] = useState("vram"); // Track the active tab
  const { gameboy } = useGameboy();
  const [busState, setBusState] = useState<BusState | undefined>(undefined);

  type BusState = {
    joypad: number[];
    io_registers: number[];
    hram: number[];
    ie_register: number;
    vram: number[];
    ram_bank_0: number[];
    ram_bank_n: number[];
  };

  useEffect(() => {
    if (!gameboy || isGameboyPaused) return;

    const updateBusState = () => {
      try {
        const wasmState = gameboy.get_bus_state();
        const newState: BusState = {
          joypad: [wasmState.joypad.register, wasmState.joypad.keys],
          io_registers: Array.from(wasmState.io_registers),
          hram: Array.from(wasmState.hram),
          ie_register: wasmState.ie_register,
          vram: Array.from(wasmState.vram),
          ram_bank_0: Array.from(wasmState.ram_bank_0),
          ram_bank_n: Array.from(wasmState.ram_bank_n),
        };
        setBusState(newState);
      } catch (error) {
        console.error("Failed to fetch state:", error);
      }
    };

    const interval = setInterval(updateBusState, 1000);
    return () => clearInterval(interval);
  }, [gameboy, isGameboyPaused]);

  const formatMemoryValues = (values: number[]) => {
    const rows = [];
    // Iterate over values in 16-byte chunks
    for (let i = 0; i < values.length; i += 16) {
      const rowIndex = i.toString(16).padStart(4, "0").toUpperCase();

      const firstGroup = [];
      for (let j = 0; j < 8; j++) {
        if (i + j < values.length) {
          const value = values[i + j]
            .toString(16)
            .padStart(2, "0")
            .toUpperCase();
          firstGroup.push(
            <td
              key={`${rowIndex}-${j}`}
              className={`p-1 ${value === "00" ? "opacity-40" : ""}`}
            >
              {value}
            </td>
          );
        } else {
          firstGroup.push(
            <td key={`${rowIndex}-${j}`} className="p-1 opacity-40">
              00
            </td>
          );
        }
      }

      const secondGroup = [];
      for (let j = 8; j < 16; j++) {
        if (i + j < values.length) {
          const value = values[i + j]
            .toString(16)
            .padStart(2, "0")
            .toUpperCase();
          secondGroup.push(
            <td
              key={`${rowIndex}-${j}`}
              className={`p-1 ${value === "00" ? "opacity-40" : ""}`}
            >
              {value}
            </td>
          );
        } else {
          secondGroup.push(
            <td key={`${rowIndex}-${j}`} className="p-1 opacity-40">
              00
            </td>
          );
        }
      }

      rows.push(
        <tr key={i}>
          <td className="p-2 mr-2">{rowIndex}</td>
          {firstGroup}
          <td className="px-2"></td>
          {secondGroup}
        </tr>
      );
    }
    return rows;
  };
  const formatIORegisters = (ioRegisters: number[]) => {
    const rows = [];
    const startAddr = 0xff00;

    for (let i = 0; i < 128; i += 3) {
      const address1 = startAddr + i;
      const address2 = startAddr + i + 1;
      const address3 = startAddr + i + 2;

      const value1 = ioRegisters[i] !== undefined ? ioRegisters[i] : 0;
      const value2 = ioRegisters[i + 1] !== undefined ? ioRegisters[i + 1] : 0;
      const value3 = ioRegisters[i + 2] !== undefined ? ioRegisters[i + 2] : 0;

      const rowIndex1 = address1.toString(16).padStart(4, "0").toUpperCase();
      const rowIndex2 = address2.toString(16).padStart(4, "0").toUpperCase();
      const rowIndex3 = address3.toString(16).padStart(4, "0").toUpperCase();

      rows.push(
        <tr key={i} className="w-full">
          <td className="p-2 mr-2">{rowIndex1}</td>
          <td className="p-2 border-r border-base-border">
            {value1.toString(16).padStart(2, "0").toUpperCase()}
          </td>
          <td className="p-2 mx-2">{rowIndex2}</td>
          <td className="p-2  border-r border-base-border">
            {value2.toString(16).padStart(2, "0").toUpperCase()}
          </td>
          <td className="p-2 mr-2">{rowIndex3}</td>
          <td className="p-2">
            {value3.toString(16).padStart(2, "0").toUpperCase()}
          </td>
        </tr>
      );
    }

    return rows;
  };

  // Function to switch tabs
  const handleTabClick = (tab: string) => {
    setActiveTab(tab);
  };

  return (
    <div className="rounded-lg shadow-lg overflow-hidden border border-base-border">
      <div
        onClick={() => setIsOpen(!isOpen)}
        className="flex justify-between items-center p-4 cursor-pointer transition-colors"
      >
        <h2 className="text-xl font-semibold">BUS</h2>
        {isOpen ? <span>▼</span> : <span>▲</span>}
      </div>
      {isOpen && busState && (
        <div className="border-t border-b border-base-border ">
          <div className="px-2  flex space-x-4 py-4">
            {/* Tabs */}
            <button
              onClick={() => handleTabClick("vram")}
              className={`p-2 ${activeTab === "vram" ? "text-secondary" : ""}`}
            >
              VRAM
            </button>
            <button
              onClick={() => handleTabClick("hram")}
              className={`p-2 ${activeTab === "hram" ? "text-secondary" : ""}`}
            >
              HRAM
            </button>
            <button
              onClick={() => handleTabClick("io_registers")}
              className={`p-2 ${
                activeTab === "io_registers" ? "text-secondary" : ""
              }`}
            >
              IO
            </button>
            <button
              onClick={() => handleTabClick("ram_bank_0")}
              className={`p-2 ${
                activeTab === "ram_bank_0" ? "text-secondary" : ""
              }`}
            >
              WRAM0
            </button>
            <button
              onClick={() => handleTabClick("ram_bank_n")}
              className={`p-2 ${
                activeTab === "ram_bank_n" ? "text-secondary" : ""
              }`}
            >
              WRAMN
            </button>
          </div>

          {/* Tab Content */}
          <div className="flex w-full h-60 justify-center  overflow-y-scroll">
            {activeTab === "vram" && (
              <table className="w-full text-sm font-mono">
                <tbody>{formatMemoryValues(busState.vram)}</tbody>
              </table>
            )}

            {activeTab === "hram" && (
              <table className="w-full text-sm font-mono">
                <tbody>{formatMemoryValues(busState.hram)}</tbody>
              </table>
            )}

            {activeTab === "io_registers" && (
              <table className="w-full text-sm font-mono border-base-border border">
                <tbody>{formatIORegisters(busState.io_registers)}</tbody>
              </table>
            )}

            {activeTab === "ram_bank_0" && (
              <table className="w-full text-sm font-mono">
                <tbody>{formatMemoryValues(busState.ram_bank_0)}</tbody>
              </table>
            )}

            {activeTab === "ram_bank_n" && (
              <table className="w-full text-sm font-mono">
                <tbody>{formatMemoryValues(busState.ram_bank_n)}</tbody>
              </table>
            )}

            {activeTab === "joypad" && (
              <table className="w-full text-sm font-mono">
                <tbody>
                  <tr>
                    <td className="p-2">
                      {busState.joypad
                        .map((value) => value.toString(16).padStart(2, "0"))
                        .join(" ")}
                    </td>
                  </tr>
                </tbody>
              </table>
            )}
          </div>
        </div>
      )}
    </div>
  );
};
