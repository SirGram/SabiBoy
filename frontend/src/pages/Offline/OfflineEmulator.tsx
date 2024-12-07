import React, { useEffect, useRef, useState } from "react";
import Layout from "../../components/Layout";
import { useGameboy } from "../../context/GameboyContext";
import Emulator from "../Game/Emulator";
import { TGame } from "../Library/Library";
import { useOptions } from "../../context/OptionsContext";

export default function OfflineEmulator() {
  const { currentGame, setCurrentGame } = useGameboy();
  const [romFileName, setRomFileName] = useState<string>("");
  const [saveFileName, setSaveFileName] = useState<string>("");
  const romInputRef = useRef<HTMLInputElement>(null);
  const stateInputRef = useRef<HTMLInputElement>(null);
  const [gameToLoad, setGameToLoad] = useState<TGame | null>(null);
  const [stateData, setStateData] = useState<Uint8Array | undefined>(undefined);

  const handleRomUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = async (e) => {
      const arrayBuffer = e.target?.result as ArrayBuffer;
      if (arrayBuffer) {
        try {
          const romData = new Uint8Array(arrayBuffer);

          setRomFileName(file.name);
          setGameToLoad((prev) => ({
            id: "0",
            name: file.name,
            romPath: URL.createObjectURL(new Blob([romData])),
            ...(prev || {}),
          }));
          console.log(`ROM loaded: ${file.name}`);
        } catch (error) {
          console.error("Failed to load ROM:", error);
          alert("Failed to load ROM. Please check the file and try again.");
        }
      }
    };
    reader.readAsArrayBuffer(file);
  };

  const handleStateUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = async (e) => {
      const arrayBuffer = e.target?.result as ArrayBuffer;
      if (arrayBuffer) {
        try {
          const loadedStateData = new Uint8Array(arrayBuffer);

          setSaveFileName(file.name);
          setStateData(loadedStateData);
        } catch (error) {
          console.error("Failed to load save state:", error);
          alert(
            "Failed to load save state. Please check the file and try again."
          );
        }
      }
    };
    reader.readAsArrayBuffer(file);
  };

  const handleLoadEmulator = () => {
    if (gameToLoad) {
      setCurrentGame(gameToLoad);
    }
  };
  useEffect(() => { 
    setCurrentGame(null);
  }, []);

  return (
    <Layout>
      <div className="flex flex-col items-center space-y-4 pt-10">
        {!currentGame ? (
          <div className="flex flex-col items-center space-y-4">
            <div className="flex space-x-4">
              {/* ROM Upload Button */}
              <div className="flex flex-col items-center">
                <button
                  onClick={() => romInputRef.current?.click()}
                  className="bg-primary hover:bg-primary text-white font-bold py-2 px-4 rounded"
                >
                  Upload ROM (.gb)
                </button>
                <input
                  type="file"
                  ref={romInputRef}
                  onChange={handleRomUpload}
                  accept=".gb"
                  className="hidden"
                />
                {romFileName && (
                  <p className="text-muted text-sm mt-2">ROM: {romFileName}</p>
                )}
              </div>

              {/* Save State Upload Button */}
              <div className="flex flex-col items-center">
                <button
                  onClick={() => stateInputRef.current?.click()}
                  className="bg-secondary hover:bg-secondary text-white font-bold py-2 px-4 rounded"
                  disabled={!romFileName}
                >
                  Upload Save State
                </button>
                <input
                  type="file"
                  ref={stateInputRef}
                  onChange={handleStateUpload}
                  accept=".gb.state"
                  className="hidden"
                />
                {saveFileName && (
                  <p className="text-sm text-gray-600 mt-2">
                    Save State: {saveFileName}
                  </p>
                )}
              </div>
            </div>

            {(saveFileName || romFileName) && (
              <button
                onClick={handleLoadEmulator}
                className="bg-accent hover:bg-accent-hover text-white font-bold py-2 px-4 rounded"
              >
                Load Emulator
              </button>
            )}
          </div>
        ) : (
          <Emulator />
        )}
      </div>
    </Layout>
  );
}
