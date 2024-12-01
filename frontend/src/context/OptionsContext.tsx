import React, { useState, useMemo, useContext, createContext } from "react";

export enum Buttons {
  UP = "Up",
  DOWN = "Down", 
  LEFT = "Left",
  RIGHT = "Right",
  A = "A",
  B = "B",
  SELECT = "Select",
  START = "Start"
}

type KeyMappingEntry = {
  mapped: string;
  mask: number;
  bit: number;
}

type KeyMapping = {
  [key in Buttons]: KeyMappingEntry;
}

const DEFAULT_KEY_MAPPING: KeyMapping = {
  [Buttons.RIGHT]: {mapped:"ArrowRight", mask: 0xfe, bit: 0},
  [Buttons.LEFT]: {mapped:"ArrowLeft", mask: 0xfd, bit: 1},
  [Buttons.UP]: {mapped:"ArrowUp", mask: 0xfb, bit: 2},
  [Buttons.DOWN]: {mapped:"ArrowDown", mask: 0xf7, bit: 3},
  [Buttons.B]: {mapped:"z", mask: 0xdf, bit: 5},
  [Buttons.A]: {mapped:"x", mask: 0xef, bit: 4},
  [Buttons.SELECT]: {mapped:"Backspace", mask: 0xbf, bit: 6},
  [Buttons.START]: {mapped:"Enter", mask: 0x7f, bit: 7},
};

type Options = {
  showFrame: boolean;
  keys: KeyMapping;
};

const defaultOptions: Options = {
  showFrame: true,
  keys: DEFAULT_KEY_MAPPING,
};

const OptionsContext = createContext<{
  options: Options;
  setOptions: React.Dispatch<React.SetStateAction<Options>>;
  toggleShowFrame: () => void;
  updateKeyMapping: (button: Buttons, newKey: string) => void;
  resetToDefaultKeys: () => void;
}>({
  options: defaultOptions,
  setOptions: () => {},
  toggleShowFrame: () => {},
  updateKeyMapping: () => {},
  resetToDefaultKeys: () => {},
});

export const OptionsProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [options, setOptions] = useState<Options>(defaultOptions);

  const toggleShowFrame = () => {
    setOptions(prev => ({
      ...prev,
      showFrame: !prev.showFrame
    }));
  };

  const updateKeyMapping = (button: Buttons, newKey: string) => {
    setOptions(prev => {
      // Check if the new key is already in use
      const isKeyAlreadyUsed = Object.values(prev.keys).some(
        mapping => mapping.mapped === newKey
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
            mapped: newKey
          }
        }
      };
    });
  };

  const resetToDefaultKeys = () => {
    setOptions(prev => ({
      ...prev,
      keys: DEFAULT_KEY_MAPPING
    }));
  };

  const value = useMemo(
    () => ({
      options,
      setOptions,
      toggleShowFrame,
      updateKeyMapping,
      resetToDefaultKeys
    }),
    [options]
  );

  return (
    <OptionsContext.Provider value={value}>
      {children}
    </OptionsContext.Provider>
  );
};

export const useOptions = () => {
  const context = useContext(OptionsContext);
  
  if (!context) {
    throw new Error('useOptions must be used within an OptionsProvider');
  }

  return context;
};