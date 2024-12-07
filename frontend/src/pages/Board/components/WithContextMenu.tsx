import { useRef, useState } from "react";
import { MoreVertical } from "lucide-react"; // Three-dot icon
import { useClickOutside } from "../../../hooks/hooks";
import { useGameboy } from "../../../context/GameboyContext";
import { TGame } from "../../Library/Library";

function ContextMenu({ game, onClose }: { game: TGame; onClose: () => void }) {
  const { setCurrentGame } = useGameboy();
  return (
    <div className="absolute top-9  bg-base-background   w-full">
      <ul>
        <li
          className="px-4 py-2 cursor-pointer hover:bg-base-background-hover "
          onClick={() => {
            setCurrentGame(game);
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
  setOpenMenuId: (id: string ) => void;
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
    <div className="relative" ref={ref}>
      <div className="group relative">
        {children}
        <button
          className={`absolute p-2 top-0 right-0 transition-all rounded-bl-md opacity-0 bg-base-background ${
            isMenuVisible ? "opacity-100" : "group-hover:opacity-100"
          } cursor-pointer`}
          onClick={handleToggleMenu}
        >
          <MoreVertical size={20} />
        </button>
      </div>

      {isMenuVisible && (
        <ContextMenu game={game} onClose={() => setOpenMenuId("")} />
      )}
    </div>
  );
}
