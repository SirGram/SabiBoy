import { useRef } from "react";
import {
  MoreVertical,
  PlayCircle,
  PlayCircleIcon,
  PlayIcon,
  PlaySquareIcon,
} from "lucide-react"; // Three-dot icon
import { useClickOutside } from "../../../hooks/hooks";
import { useGameboy } from "../../../context/GameboyContext";
import { TGame, TGameDetails } from "../../../types";
import api from "../../../api/client";

function ContextMenu({ game, onClose }: { game: TGame; onClose: () => void }) {
  const { setCurrentGame } = useGameboy();

  const handleGameSelect = async (slug: string) => {
    try {
      const response = await api.get(`api/games/${slug}`);
      const gameDetails: TGameDetails = await response.data;
      console.log(gameDetails);
      setCurrentGame(gameDetails);
    } catch (error) {
      console.error("Failed to load game details:", error);
    }
  };
  return (
    <div className="absolute top-9  bg-base-background   w-full ">
      <ul>
        <li
          className="px-4 py-2 cursor-pointer hover:bg-base-background-hover "
          onClick={() => {
            handleGameSelect(game.slug);
            onClose();
          }}
        >
          Game Information
        </li>
        <li
          className="px-4 py-2 cursor-pointer  hover:bg-base-background-hover"
          onClick={() => {
            console.log("Option 2 clicked");
            onClose();
          }}
        >
          Option 2
        </li>
      </ul>
    </div>
  );
}
type WithContextMenuProps = {
  children: React.ReactNode;
  menuId: string;
  openMenuId: string | null;
  setOpenMenuId: (id: string) => void;
  game: TGame;
};
export function WithContextMenu({
  children,
  menuId,
  openMenuId,
  setOpenMenuId,
  game,
}: WithContextMenuProps) {
  const ref = useRef<HTMLDivElement>(null);
  const isMenuVisible = openMenuId === menuId;

  // Close menu when clicking outside
  useClickOutside(ref, () => {
    if (isMenuVisible) {
      setOpenMenuId("");
    }
  });

  const handleToggleMenu = () => {
    setOpenMenuId(isMenuVisible ? "" : menuId);
  };

  return (
    <div className="relative pb-1" ref={ref}>
      <div className="group relative">
        {children}
        <button
          className={`rounded-tr-md absolute p-2 top-0 right-0 transition-all rounded-bl-md opacity-0 bg-base-background ${
            isMenuVisible ? "opacity-100" : "group-hover:opacity-100"
          } cursor-pointer`}
          onClick={handleToggleMenu}
        >
          <MoreVertical size={20} />
        </button>

        <div
          className={`absolute inset-0 w-full h-full flex items-center justify-center pointer-events-none transition-opacity delay-100 opacity-0 group-hover:opacity-100
          }`}
        >
          <PlayCircle size={40} />
        </div>
      </div>

      {isMenuVisible && (
        <ContextMenu game={game} onClose={() => setOpenMenuId("")} />
      )}
    </div>
  );
}
