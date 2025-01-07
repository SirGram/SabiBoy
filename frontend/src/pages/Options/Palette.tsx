import { useState, useEffect } from "react";
import { PlusIcon, EditIcon, XIcon, Trash2Icon, CheckIcon } from "lucide-react";

const LOCAL_STORAGE_CUSTOM_PALETTES_KEY = "sabiboy-custom-palettes";

type CustomPalette = {
  id: string;
  name: string;
  colors: number[];
};

export default function PaletteEditor({
  currentPalette,
  onPaletteChange,
  predefinedPalettes,
}: {
  currentPalette: number[];
  onPaletteChange: (palette: number[]) => void;
  predefinedPalettes: { name: string; colors: number[] }[];
}) {
  const [localPalette, setLocalPalette] = useState(currentPalette);
  const [customPalettes, setCustomPalettes] = useState<CustomPalette[]>([]);
  const [editingName, setEditingName] = useState<string | null>(null);
  const [newPaletteName, setNewPaletteName] = useState("");
  const [showSaveDialog, setShowSaveDialog] = useState(false);

  useEffect(() => {
    const stored = localStorage.getItem(LOCAL_STORAGE_CUSTOM_PALETTES_KEY);
    if (stored) {
      setCustomPalettes(JSON.parse(stored));
    }
  }, []);

  useEffect(() => {
    localStorage.setItem(
      LOCAL_STORAGE_CUSTOM_PALETTES_KEY,
      JSON.stringify(customPalettes)
    );
  }, [customPalettes]);

  const readHexColor = (color: string) => {
    const hexColor = color.replace("#", "");
    return parseInt(hexColor, 16);
  };

  const handleColorChange = (index: number, newColor: string) => {
    const newPaletteColors = [...localPalette];
    newPaletteColors[index] = readHexColor(newColor);
    setLocalPalette(newPaletteColors);
    onPaletteChange(newPaletteColors); // Auto-apply changes
  };

  const savePalette = () => {
    if (!newPaletteName.trim()) return;

    const newPalette: CustomPalette = {
      id: Date.now().toString(),
      name: newPaletteName,
      colors: localPalette,
    };

    setCustomPalettes((prev) => [...prev, newPalette]);
    setNewPaletteName("");
    setShowSaveDialog(false);
  };

  const deletePalette = (id: string) => {
    setCustomPalettes((prev) => prev.filter((palette) => palette.id !== id));
  };

  const startRename = (id: string, currentName: string) => {
    setEditingName(id);
    setNewPaletteName(currentName);
  };

  const finishRename = (id: string) => {
    if (!newPaletteName.trim()) return;

    setCustomPalettes((prev) =>
      prev.map((palette) =>
        palette.id === id ? { ...palette, name: newPaletteName } : palette
      )
    );

    setEditingName(null);
    setNewPaletteName("");
  };

  return (
    <div className="w-full space-y-6 text-base-foreground">
      <div className=" rounded-lg p-6">
        <div className="flex items-center justify-between pb-4 border-b border-base-border">
          <button
            onClick={() => setShowSaveDialog(true)}
            className="inline-flex items-center gap-2 px-3 py-1 text-sm bg-primary  rounded hover:bg-primary-hover transition-colors"
          >
            <PlusIcon size={16} />
            Save Current
          </button>
        </div>

        <div className="mt-6">
          <div className="grid grid-cols-4 gap-4 mb-6">
            {localPalette.map((color, index) => (
              <div key={index} className="flex flex-col items-center gap-2">
                <label className="text-sm font-medium">Color {index + 1}</label>
                <div className="relative w-full aspect-square">
                  <input
                    type="color"
                    value={`#${color.toString(16).padStart(6, "0")}`}
                    onChange={(e) => handleColorChange(index, e.target.value)}
                    className="absolute inset-0 w-full h-full cursor-pointer opacity-0"
                  />
                  <div
                    className="w-full h-full rounded border border-border"
                    style={{
                      backgroundColor: `#${color
                        .toString(16)
                        .padStart(6, "0")}`,
                    }}
                  />
                </div>
                <code className="text-xs bg-muted px-2 py-1 rounded">
                  #{color.toString(16).padStart(6, "0")}
                </code>
              </div>
            ))}
          </div>

          {showSaveDialog && (
            <div className="bg-muted rounded p-4 mb-6">
              <div className="flex items-center gap-2">
                <input
                  type="text"
                  value={newPaletteName}
                  onChange={(e) => setNewPaletteName(e.target.value)}
                  placeholder="Enter palette name"
                  className="flex-1 px-3 py-2 text-sm rounded border bg-background"
                  autoFocus
                />
                <button
                  onClick={savePalette}
                  className="p-2 text-primary hover:bg-primary/10 rounded hover:text-primary-hover border border-primary"
                  disabled={!newPaletteName.trim()}
                >
                  <CheckIcon size={16} />
                </button>
                <button
                  onClick={() => setShowSaveDialog(false)}
                  className="p-2 text-destructive hover:text-destructive-hover rounded border border-destructive"
                >
                  <XIcon size={16} />
                </button>
              </div>
            </div>
          )}

          {customPalettes.length > 0 && (
            <div className="space-y-3 mb-6">
              <h3 className="font-medium text-sm">Custom Palettes</h3>
              <div className="grid grid-cols-2 gap-3">
                {customPalettes.map((palette) => (
                  <div key={palette.id} className="bg-muted rounded p-3">
                    <div className="flex justify-between items-center mb-2">
                      {editingName === palette.id ? (
                        <div className="flex items-center gap-2 flex-1 relative">
                          <input
                            type="text"
                            value={newPaletteName}
                            onChange={(e) => setNewPaletteName(e.target.value)}
                            className="flex-1 px-2 py-1 text-sm rounded border bg-background"
                            autoFocus
                          />
                          <div className="absolute right-0 ">
                            <button
                              onClick={() => finishRename(palette.id)}
                              className="p-1 text-primary hover:bg-primary/10 rounded"
                            >
                              <CheckIcon size={14} />
                            </button>
                            <button
                              onClick={() => setEditingName(null)}
                              className="p-1 text-destructive hover:bg-destructive/10 rounded"
                            >
                              <XIcon size={14} />
                            </button>
                          </div>
                        </div>
                      ) : (
                        <>
                          <span className="text-sm font-medium">
                            {palette.name}
                          </span>
                          <div className="flex items-center gap-1">
                            <button
                              onClick={() =>
                                startRename(palette.id, palette.name)
                              }
                              className="p-1 hover:bg-secondary rounded"
                            >
                              <EditIcon size={14} />
                            </button>
                            <button
                              onClick={() => deletePalette(palette.id)}
                              className="p-1 text-destructive hover:bg-destructive/10 rounded"
                            >
                              <Trash2Icon size={14} />
                            </button>
                          </div>
                        </>
                      )}
                    </div>
                    <button
                      onClick={() => {
                        setLocalPalette(palette.colors);
                        onPaletteChange(palette.colors);
                      }}
                      className="w-full h-8  gap-1 rounded  hover:ring-2 ring-ring transition-all"
                    >
                      <div className="h-6 grid grid-cols-4 gap-1 rounded overflow-hidden w-full">
                        {palette.colors.map((color, index) => (
                          <div
                            key={index}
                            style={{
                              backgroundColor: `#${color
                                .toString(16)
                                .padStart(6, "0")}`,
                            }}
                          />
                        ))}
                      </div>
                    </button>
                  </div>
                ))}
              </div>
            </div>
          )}

          <div className="space-y-3">
            <h3 className=" text-sm  text-base-foreground">
              Predefined Palettes
            </h3>
            <div className="grid grid-cols-2  gap-3">
              {predefinedPalettes.map((palette) => (
                <button
                  key={palette.name}
                  onClick={() => {
                    setLocalPalette(palette.colors);
                    onPaletteChange(palette.colors);
                  }}
                  className="flex flex-col gap-2 p-3 bg-muted rounded hover:bg-secondary transition-colors"
                >
                  <span className="text-sm font-medium">{palette.name}</span>
                  <div className="h-6 grid grid-cols-4 gap-1 rounded overflow-hidden w-full">
                    {palette.colors.map((color, index) => (
                      <div
                        key={index}
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
      </div>
    </div>
  );
}
