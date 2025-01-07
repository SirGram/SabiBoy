import { useEffect, useState } from "react";
import { ChevronDown, ChevronRight } from "lucide-react";
import GameCard from "../../Library/components/GameCard";
import { useGameboy } from "../../../context/GameboyContext";
import { WithContextMenu } from "./WithContextMenu";
import { useNavigate } from "react-router-dom";
import { TGame } from "../../../types";
import { loadGameImage } from "../../../api/api";
import api from "../../../api/client";

type CollapsibleListProps = {
  title: string;
  games: TGame[];
  defaultOpen?: boolean;
  openMenuId: string | null;
  updateOpenMenuId: (id: string) => void;
};

export default function CollapsibleList({
  title,
  games,
  defaultOpen = true,
  openMenuId,
  updateOpenMenuId,
}: CollapsibleListProps) {
  const [isOpen, setIsOpen] = useState(defaultOpen);
  const { setCurrentGame } = useGameboy();
  const [loadedGames, setLoadedGames] = useState<TGame[]>([]);

  const toggleCollapse = () => {
    setIsOpen(!isOpen);
  };

  const handleGameSelect = async (slug: string) => {
    try {
      const response = await api.get(`/api/games/${slug}`);
      console.log(response);
      const gameDetails = response.data;
      setCurrentGame(gameDetails);

      navigate("/emulator");
    } catch (error) {
      console.error("Failed to load game details:", error);
    }
  };

  useEffect(() => {
    if (isOpen && games.length > 0 && loadedGames.length === 0) {
      Promise.all(games.map(loadGameImage))
        .then((gamesWithImages) => {
          setLoadedGames(gamesWithImages);
        })
        .catch((error) => {
          console.error("Failed to load game images:", error);
        });
    }
  }, [isOpen, games]);
  const navigate = useNavigate();

  return (
    <div className="flex flex-col gap-4 w-full rounded-lg p-2">
      <div
        className="flex items-center justify-start cursor-pointer w-fit text-muted-foreground"
        onClick={toggleCollapse}
      >
        {isOpen ? <ChevronDown /> : <ChevronRight />}
        <h2 className="text-xl font-semibold">{title}</h2>
      </div>

      {isOpen && (
        <div className="px-1 flex items-center gap-4 py-2  overflow-x-auto whitespace-nowrap md:flex-wrap md:overflow-x-hidden w-full">
          {" "}
          {loadedGames.length > 0 ? (
            loadedGames.map((game) => (
              <WithContextMenu
                key={game.slug}
                menuId={game.slug}
                openMenuId={openMenuId}
                setOpenMenuId={updateOpenMenuId}
                game={game}
              >
                <GameCard
                  game={game}
                  onClick={() => handleGameSelect(game.slug)}
                />
              </WithContextMenu>
            ))
          ) : (
            <p className="text-gray-500">No games in this category</p>
          )}
        </div>
      )}
    </div>
  );
}
