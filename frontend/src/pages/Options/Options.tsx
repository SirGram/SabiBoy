import { useState, useEffect } from "react";
import { EditIcon, SaveIcon, XIcon } from "lucide-react";
import Layout from "../../components/Layout";
import { PREDEFINED_PALETTES, useOptions } from "../../context/OptionsContext";
import { Buttons } from "../../context/OptionsContext"; // Adjust import path

export default function Options() {
  const {
    options,
    resetToDefaultKeys,
    updateKeyMapping,
    toggleShowFrame,
    updatePalette,
    resetPalette,
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

  const [localPalette, setLocalPalette] = useState(options.palette);

  const readHexColor = (color: string) => {
    const hexColor = color.replace("#", "");
    return parseInt(hexColor, 16);
  };

  const handleColorChange = (index: number, newColor: string) => {
    const newPaletteColors = [...localPalette];
    newPaletteColors[index] = readHexColor(newColor);
    setLocalPalette(newPaletteColors);
  };

  useEffect(() => {
    setLocalPalette(options.palette);
  }, [options.palette]);

  const handlePredefinedPaletteSelect = (palette: number[]) => {
    setLocalPalette(palette);
    updatePalette(palette);
  };

  return (
    <Layout>
      <div className="flex flex-col gap-4 h-full items-center py-20 max-w-md mx-auto">
        <h1 className="text-2xl font-bold mb-6">Emulator Settings</h1>

        {/* Frame Toggle */}
        <div className="w-full flex justify-between items-center border-b pb-4 mb-4">
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

        {/* Key Mappings */}
        <div className="w-full">
          <h2 className="text-xl font-semibold mb-4">Key Mappings</h2>
          {Object.entries(options.keys).map(([button, mapping]) => (
            <div
              key={button}
              className="flex justify-between items-center border-b py-2 h-12"
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
                    className="text-gray-500 hover:bg-gray-100 p-1 rounded"
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
        <div className="w-full">
          <h2 className="text-xl font-semibold mb-4 flex w-full justify-between">
            Palette Editor
          </h2>
          <div className="flex  w-full justify-between">
            <div className="flex gap-1">
              {localPalette.map((color, index) => (
                <div
                  key={index}
                  className="flex flex-col  items-center justify-between  "
                >
                  <label className=" ">Color {index + 1}</label>
                  <input
                    type="color"
                    value={`#${color.toString(16).padStart(6, "0")}`}
                    onChange={(e) => handleColorChange(index, e.target.value)}
                    className="w-16 h-10"
                  />
                  <span className="">{`#${color
                    .toString(16)
                    .padStart(6, "0")}`}</span>
                </div>
              ))}
            </div>
            <div className="flex justify-between flex-col gap-2">
              <button
                onClick={() => updatePalette(localPalette)}
                className=" bg-primary text-white px-4 py-2 rounded hover:bg-primary-hover transition"
              >
                Save Palette
              </button>
              <button
                onClick={() => resetPalette()}
                className=" bg-destructive text-white px-4 py-2 rounded hover:bg-destructive-hover transition"
              >
                Reset Palette
              </button>
            </div>
          </div>
        </div>
        {/* Predefined Palettes Section */}
        <div className="w-full">
          <h3 className="text-lg font-semibold mb-3">Predefined Palettes</h3>
          <div className="grid grid-cols-5 w-full  gap-2">
            {PREDEFINED_PALETTES.map((palette) => (
              <button
                key={palette.name}
                onClick={() => handlePredefinedPaletteSelect(palette.colors)}
                className="flex flex-col items-center justify-center gap-2 bg-muted text-secondary-foreground px-3 py-2 rounded hover:bg-muted-hover transition"
              >
                <span className="text-sm h-full flex items-center">
                  {palette.name}
                </span>
                <div className="flex ">
                  {palette.colors.map((color, index) => (
                    <div
                      key={index}
                      className="w-4 h-4 border border-white"
                      style={{
                        backgroundColor: `#${color
                          .toString(16)
                          .padStart(6, "0")}`,
                      }}
                    />
                  ))}
                </div>
              </button>
            ))}
          </div>
        </div>
      </div>
    </Layout>
  );
}
