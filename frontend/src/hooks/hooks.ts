import { useEffect, useState } from "react";

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

export const useImageLoader = (imagePath: string | undefined) => {
  const [imageURL, setImageURL] = useState<string | null>(null);

  useEffect(() => {
    const fetchImage = async () => {
      if (!imagePath) return;

      const token = localStorage.getItem("access_token");
      if (!token) return;

      try {
        const response = await fetch(imagePath, {
          headers: { Authorization: `Bearer ${token}` },
        });

        if (!response.ok) {
          console.error(`Failed to fetch image: ${response.statusText}`);
          return;
        }

        const blob = await response.blob();
        const url = URL.createObjectURL(blob);
        setImageURL(url);
      } catch (error) {
        console.error("Error fetching image:", error);
      }
    };

    fetchImage();

    return () => {
      if (imageURL) URL.revokeObjectURL(imageURL);
    };
  }, [imagePath]);

  return imageURL;
};
