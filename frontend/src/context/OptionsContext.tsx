import React, { useState, useMemo, useContext, createContext } from "react";

type Options = {
  showFrame: boolean;
};

const defaultOptions: Options = {
  showFrame: true,
};

const OptionsContext = createContext<{
  options: Options;
  setOptions: React.Dispatch<React.SetStateAction<Options>>;
}>({
  options: defaultOptions,
  setOptions: () => {},
});

export const OptionsProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [options, setOptions] = useState<Options>(defaultOptions);

  const value = useMemo(
    () => ({
      options,
      setOptions,
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
  
  const toggleShowFrame = () => {
    context.setOptions(prevOptions => ({
      ...prevOptions,
      showFrame: !prevOptions.showFrame
    }));
  };

  return {
    ...context,
    toggleShowFrame
  };
};