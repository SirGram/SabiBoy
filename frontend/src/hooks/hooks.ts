import { useEffect, useState } from "react";
import api from "../api/client";

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
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    setIsLoading(true);
    let isMounted = true;

    const fetchImage = async () => {
      if (!imagePath) {
        setIsLoading(false);
        return;
      }

      try {
        const response = await api.get(imagePath, {
          responseType: 'blob'
        });

        if (isMounted) {
          const url = URL.createObjectURL(response.data);
          setImageURL(url);
          setIsLoading(false);
        }
      } catch (error) {
        console.error("Error fetching image:", error);
        if (isMounted) {
          setIsLoading(false);
        }
      }
    };

    fetchImage();

    return () => {
      isMounted = false;
      if (imageURL) {
        URL.revokeObjectURL(imageURL);
      }
    };
  }, [imagePath]);

  return { imageURL, isLoading };
};

