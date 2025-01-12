import { useState, useEffect } from "react";
import { EditIcon, SaveIcon, XIcon } from "lucide-react";
import {
  PREDEFINED_PALETTES,
  useOptions,
} from "../../../context/OptionsContext";
import { Buttons } from "../../../context/OptionsContext";
import PaletteEditor from "./Palette";
import CollapsibleList from "../../../components/CollapsibleList";

export default function Options() {
  const {
    options,
    resetToDefaultKeys,
    updateKeyMapping,
    toggleShowFrame,
    updatePalette,
    toggleDebug,
    toggleMuteOnStart,
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
    setNewKey(options.keys[button].mapped);
  };

  const saveKeyChange = () => {
    if (editingButton && newKey) {
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
    <div className="flex flex-col gap-6 h-full items-center max-w-md mx-auto p-4">
      <CollapsibleList title="General Settings">
        <div className="w-full space-y-4">
          <label className="flex justify-between items-center w-full py-2">
            <span className="text-base">Mute on Start</span>
            <input
              type="checkbox"
              checked={options.muteOnStart}
              onChange={toggleMuteOnStart}
              className="size-6"
            />
          </label>
          <label className="flex justify-between items-center w-full py-2">
            <span className="text-base">Show Gameboy Frame</span>
            <input
              type="checkbox"
              checked={options.showFrame}
              onChange={toggleShowFrame}
              className="size-6"
            />
          </label>

          <label className="flex justify-between items-center w-full py-2">
            <span className="text-base">Debug Mode</span>
            <input
              type="checkbox"
              checked={options.debug}
              onChange={toggleDebug}
              className="size-6"
            />
          </label>
        </div>
      </CollapsibleList>

      <CollapsibleList title="Key Mappings">
        <div className="w-full space-y-2">
          {Object.entries(options.keys).map(([button, mapping]) => (
            <div
              key={button}
              className="flex justify-between items-center py-3 px-2 hover:bg-muted/10 rounded-lg"
            >
              <span className="font-medium">{button}</span>
              {editingButton === button ? (
                <div className="flex items-center space-x-2">
                  <input
                    type="text"
                    value={newKey}
                    onChange={(e) => setNewKey(e.target.value.toLowerCase())}
                    maxLength={1}
                    className="w-16 px-2 py-1 border rounded bg-base-background text-center"
                    autoFocus
                  />
                  <button
                    onClick={saveKeyChange}
                    className="hover:bg-secondary/20 p-1 rounded"
                  >
                    <SaveIcon size={20} />
                  </button>
                  <button
                    onClick={cancelKeyEdit}
                    className="hover:bg-destructive/20 p-1 rounded"
                  >
                    <XIcon size={20} />
                  </button>
                </div>
              ) : (
                <div className="flex items-center space-x-3">
                  <span className="min-w-12 text-center text-muted-foreground">
                    {mapping.mapped.toUpperCase()}
                  </span>
                  <button
                    onClick={() => handleKeyEdit(button as Buttons)}
                    className="text-muted-foreground hover:bg-muted/20 p-1 rounded"
                  >
                    <EditIcon size={16} />
                  </button>
                </div>
              )}
            </div>
          ))}
          <div className="pt-4">
            <button
              onClick={resetToDefaultKeys}
              className="w-full bg-destructive/90 text-white px-4 py-2 rounded-lg hover:bg-destructive transition"
            >
              Reset to Default Keys
            </button>
          </div>
        </div>
      </CollapsibleList>

      <CollapsibleList title="Color Palette">
        <div className="w-full">
          <PaletteEditor
            currentPalette={options.palette}
            onPaletteChange={updatePalette}
            predefinedPalettes={PREDEFINED_PALETTES}
          />
        </div>
      </CollapsibleList>
    </div>
  );
}
