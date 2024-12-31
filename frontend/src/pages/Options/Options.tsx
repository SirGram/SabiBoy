import { useState, useEffect } from "react";
import { EditIcon, SaveIcon, XIcon } from "lucide-react";
import Layout from "../../components/Layout/MainLayout";
import { PREDEFINED_PALETTES, useOptions } from "../../context/OptionsContext";
import { Buttons } from "../../context/OptionsContext"; // Adjust import path
import PaletteEditor from "./Palette";

export default function Options() {
  const {
    options,
    resetToDefaultKeys,
    updateKeyMapping,
    toggleShowFrame,
    updatePalette,
    toggleDebug,
  } = useOptions();
  const [editingButton, setEditingButton] = useState<Buttons | null>(null);
  const [newKey, setNewKey] = useState("");

  const handleKeyPress = (event: KeyboardEvent) => {
    if (editingButton) {
      setNewKey(event.key);
    }
  };

  useEffect(() => {
    if (editingButton) {
      window.addEventListener("keydown", handleKeyPress);
    }

    return () => {
      window.removeEventListener("keydown", handleKeyPress);
    };
  }, [editingButton]);

  const handleKeyEdit = (button: Buttons) => {
    setEditingButton(button);
    setNewKey(options.keys[button].mapped); // Pre-fill with the current key
  };

  const saveKeyChange = () => {
    if (editingButton && newKey) {
      // Check if the new key is already in use
      const isKeyAlreadyUsed = Object.values(options.keys).some(
        (mapping) => mapping.mapped === newKey
      );

      if (!isKeyAlreadyUsed) {
        updateKeyMapping(editingButton, newKey);
        setEditingButton(null);
      } else {
        alert(`The key "${newKey}" is already in use.`);
      }
    }
  };

  const cancelKeyEdit = () => {
    setEditingButton(null);
    setNewKey("");
  };

  return (
    <Layout>
      <div className="flex flex-col gap-4 h-full items-center max-w-md mx-auto ">
        <h1 className="text-2xl font-bold mb-6">Emulator Settings</h1>

        {/* Frame Toggle */}
        <div className="w-full flex justify-between items-center border-base-border border-b   pb-4 mb-4">
          <label className="flex justify-between space-x-2 w-full">
            <span>Show Gameboy Frame</span>
            <input
              type="checkbox"
              checked={options.showFrame}
              onChange={toggleShowFrame}
              className="size-6"
            />
          </label>
        </div>
        {/* Debug */}
        <div className="w-full flex justify-between items-center border-base-border border-b pb-4 mb-4">
          <label className="flex justify-between space-x-2 w-full">
            <span>Debug Mode</span>
            <input
              type="checkbox"
              checked={options.debug}
              onChange={toggleDebug}
              className="size-6"
            />
          </label>
        </div>

        {/* Key Mappings */}
        <div className="w-full">
          <h2 className="text-xl font-semibold mb-4">Key Mappings</h2>
          {Object.entries(options.keys).map(([button, mapping]) => (
            <div
              key={button}
              className="flex justify-between items-center border-base-border border-b py-2 h-12"
            >
              <span className="font-medium">{button}</span>
              {editingButton === button ? (
                <div className="flex items-center space-x-2">
                  <input
                    type="text"
                    value={newKey}
                    onChange={(e) => setNewKey(e.target.value.toLowerCase())}
                    maxLength={1}
                    className="w-fit px-2 py-1 border rounded bg-base-background"
                    autoFocus
                  />
                  <button
                    onClick={saveKeyChange}
                    className=" hover:bg-secondary-hover p-1 rounded"
                  >
                    <SaveIcon size={20} />
                  </button>
                  <button
                    onClick={cancelKeyEdit}
                    className=" hover:bg-destructive-hover p-1 rounded"
                  >
                    <XIcon size={20} />
                  </button>
                </div>
              ) : (
                <div className="flex items-center space-x-2">
                  <span>{mapping.mapped.toUpperCase()}</span>
                  <button
                    onClick={() => handleKeyEdit(button as Buttons)}
                    className="text-muted hover:bg-muted p-1 rounded"
                  >
                    <EditIcon size={16} />
                  </button>
                </div>
              )}
            </div>
          ))}

          {/* Reset to Default Button */}
          <div className="mt-6 flex justify-center">
            <button
              onClick={resetToDefaultKeys}
              className="bg-destructive text-white px-4 py-2 rounded hover:bg-destructive-hover transition"
            >
              Reset to Default Keys
            </button>
          </div>
        </div>
        {/* Palette Editor */}
        <PaletteEditor
          currentPalette={options.palette}
          onPaletteChange={updatePalette}
          predefinedPalettes={PREDEFINED_PALETTES}
        />
      </div>
    </Layout>
  );
}
