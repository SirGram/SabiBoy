import React, {
  useState,
  useMemo,
  useContext,
  createContext,
  useEffect,
} from "react";

export const PREDEFINED_PALETTES = [
  {
    name: "Classic Green",
    colors: [0x9bbc0f, 0x8bac0f, 0x306230, 0x0f380f],
  },
  {
    name: "Autumn",
    colors: [0xf8e4c8, 0xc0a080, 0x90783c, 0x4c3a29],
  },
  {
    name: "Puke Green",
    colors: [0xe0f8d0, 0x88c070, 0x346856, 0x081820],
  },
  {
    name: "Grayscale",
    colors: [0xffffff, 0xaaaaaa, 0x555555, 0x000000],
  },
  {
    name: "Pastel",
    colors: [0xf0f0c0, 0x90b0a0, 0x709080, 0x405050],
  },
  {
    name: "Ocean Breeze",
    colors: [0xb0d8d8, 0x6ba3a3, 0x4a7c7c, 0x2c4b4b],
  },
  {
    name: "Sunset",
    colors: [0xffd8b1, 0xffaa5e, 0xd45500, 0x7f2a00],
  },
  {
    name: "Cyberpunk",
    colors: [0x00ff00, 0x008f11, 0x002f00, 0x000000],
  },
  {
    name: "Lavender Dream",
    colors: [0xe6e6fa, 0xb0b0e6, 0x6a5acd, 0x483d8b],
  },
  {
    name: "Neon Nights",
    colors: [0x00ffff, 0xff00ff, 0x0000ff, 0x000000],
  },
];

export enum Buttons {
  UP = "Up",
  DOWN = "Down",
  LEFT = "Left",
  RIGHT = "Right",
  A = "A",
  B = "B",
  SELECT = "Select",
  START = "Start",
}

type KeyMappingEntry = {
  mapped: string;
  mask: number;
  bit: number;
};

type KeyMapping = {
  [key in Buttons]: KeyMappingEntry;
};

const DEFAULT_KEY_MAPPING: KeyMapping = {
  [Buttons.RIGHT]: { mapped: "ArrowRight", mask: 0xfe, bit: 0 },
  [Buttons.LEFT]: { mapped: "ArrowLeft", mask: 0xfd, bit: 1 },
  [Buttons.UP]: { mapped: "ArrowUp", mask: 0xfb, bit: 2 },
  [Buttons.DOWN]: { mapped: "ArrowDown", mask: 0xf7, bit: 3 },
  [Buttons.B]: { mapped: "x", mask: 0xef, bit: 4 },
  [Buttons.A]: { mapped: "z", mask: 0xdf, bit: 5 },
  [Buttons.SELECT]: { mapped: "Backspace", mask: 0xbf, bit: 6 },
  [Buttons.START]: { mapped: "Enter", mask: 0x7f, bit: 7 },
};

type Options = {
  showFrame: boolean;
  keys: KeyMapping;
  palette: number[];
  debug: boolean;
};

const DEFAULT_PALETTE = PREDEFINED_PALETTES[0].colors;

const defaultOptions: Options = {
  showFrame: true,
  keys: DEFAULT_KEY_MAPPING,
  palette: DEFAULT_PALETTE,
  debug: false,
};

const OptionsContext = createContext<{
  options: Options;
  setOptions: React.Dispatch<React.SetStateAction<Options>>;
  toggleShowFrame: () => void;
  toggleDebug: () => void;
  updateKeyMapping: (button: Buttons, newKey: string) => void;
  resetToDefaultKeys: () => void;
  updatePalette: (newPalette: number[]) => void;
  resetPalette: () => void;
}>({
  options: defaultOptions,
  setOptions: () => {},
  toggleShowFrame: () => {},
  toggleDebug: () => {},
  updateKeyMapping: () => {},
  resetToDefaultKeys: () => {},
  updatePalette: () => {},
  resetPalette: () => {},
});

const LOCAL_STORAGE_KEY = "sabiboy-options";

export const OptionsProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const loadOptions = (): Options => {
    try {
      const storedOptions = localStorage.getItem(LOCAL_STORAGE_KEY);
      if (storedOptions) {
        return JSON.parse(storedOptions);
      }
    } catch (error) {
      console.error("Failed to load options from localStorage:", error);
    }
    return defaultOptions;
  };
  const [options, setOptions] = useState<Options>(loadOptions);
  useEffect(() => {
    try {
      localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify(options));
    } catch (error) {
      console.error("Failed to save options to localStorage:", error);
    }
  }, [options]);

  const toggleShowFrame = () => {
    let newShowFrameValue = options.debug
      ? options.showFrame
      : !options.showFrame;

    setOptions((prev) => ({
      ...prev,
      showFrame: newShowFrameValue,
    }));
  };
  const toggleDebug = () => {
    let newShowFrameValue = options.debug ? true : false;
    setOptions((prev) => ({
      ...prev,
      debug: !prev.debug,
      showFrame: newShowFrameValue,
    }));
  };

  const updateKeyMapping = (button: Buttons, newKey: string) => {
    setOptions((prev) => {
      // Check if the new key is already in use
      const isKeyAlreadyUsed = Object.values(prev.keys).some(
        (mapping) => mapping.mapped === newKey
      );

      if (isKeyAlreadyUsed) {
        alert(`The key "${newKey}" is already in use.`);
        return prev;
      }

      return {
        ...prev,
        keys: {
          ...prev.keys,
          [button]: {
            ...prev.keys[button],
            mapped: newKey,
          },
        },
      };
    });
  };

  const resetToDefaultKeys = () => {
    setOptions((prev) => ({
      ...prev,
      keys: DEFAULT_KEY_MAPPING,
    }));
  };

  const updatePalette = (newPalette: number[]) => {
    setOptions((prev) => ({
      ...prev,
      palette: newPalette,
    }));
  };
  const resetPalette = () => {
    setOptions((prev) => ({
      ...prev,
      palette: DEFAULT_PALETTE,
    }));
  };

  const value = useMemo(
    () => ({
      options,
      setOptions,
      toggleShowFrame,
      toggleDebug,
      updateKeyMapping,
      resetToDefaultKeys,
      updatePalette,
      resetPalette,
    }),
    [options]
  );

  return (
    <OptionsContext.Provider value={value}>{children}</OptionsContext.Provider>
  );
};

export const useOptions = () => {
  const context = useContext(OptionsContext);

  if (!context) {
    throw new Error("useOptions must be used within an OptionsProvider");
  }

  return context;
};
