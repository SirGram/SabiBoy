import React, { createContext, useState, useContext, ReactNode } from "react";

interface ModalContextType {
  showUploadModal: boolean;
  updateShowUploadModal: (showUploadModal: boolean) => void;
}

const ModalContext = createContext<ModalContextType>({
  showUploadModal: false,
  updateShowUploadModal: () => {},
});

export const ModalProvider: React.FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [showUploadModal, setShowUploadModal] = useState(false);

  const updateShowUploadModal = (showUploadModal: boolean) => {
    setShowUploadModal(showUploadModal);
  };

  return (
    <ModalContext.Provider
      value={{
        showUploadModal,
        updateShowUploadModal,
      }}
    >
      {children}
    </ModalContext.Provider>
  );
};
export const useModal = () => useContext(ModalContext);
