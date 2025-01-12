import React, { useEffect, useRef } from "react";

interface ModalProps {
  children: React.ReactNode;
  isOpen: boolean;
  onClose: () => void;
  title?: string;
}

export default function Modal({
  children,
  isOpen,
  onClose,
  title,
}: ModalProps) {
  const modalRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        modalRef.current &&
        !modalRef.current.contains(event.target as Node)
      ) {
        onClose();
      }
    };

    if (isOpen) {
      document.addEventListener("mousedown", handleClickOutside);
      document.body.style.overflow = "hidden";
    }

    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.body.style.overflow = "unset";
    };
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center">
      <div
        ref={modalRef}
        className=" py-4 px-3  bg-base-background rounded-lg shadow-xl w-full max-w-md max-h-[80vh] overflow-y-auto"
      >
        <h1 className="text-3xl font-bold w-full text-center">{title}</h1>

        <div className="h-1 my-4 w-full bg-gradient-to-tr from-accent to-transparent"></div>
        {children}
      </div>
    </div>
  );
}
