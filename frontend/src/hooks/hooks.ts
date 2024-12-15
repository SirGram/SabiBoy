import { useEffect } from "react";

export function useClickOutside(
  ref: React.RefObject<HTMLElement>,
  onClose: () => void
) {
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (ref.current && !ref.current.contains(event.target as Node)) {
        onClose();
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [ref, onClose]);
}

export function usePreventDefaultTouch() {
  useEffect(() => {
    // Prevent default touch behavior on mobile devices
    const preventDefaultTouch = (e: TouchEvent) => e.preventDefault();
    window.addEventListener("touchmove", preventDefaultTouch, {
      passive: false,
    });
    return () => {
      window.removeEventListener("touchmove", preventDefaultTouch);
    };
  }, []);
}
