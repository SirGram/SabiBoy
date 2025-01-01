import { useEffect, useState } from "react";
import { ChevronDown, ChevronRight } from "lucide-react";
import GameCard from "../../Library/components/GameCard";
import { useGameboy } from "../../../context/GameboyContext";
import { WithContextMenu } from "./WithContextMenu";
import { useAuth } from "../../../context/AuthContext";
import { useNavigate } from "react-router-dom";
import { TGame } from "../../../types";

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
  const { fetchWithAuth } = useAuth();
  const [loadedGames, setLoadedGames] = useState<TGame[]>([]);

  const toggleCollapse = () => {
    setIsOpen(!isOpen);
  };

  const loadGameImage = async (game: TGame): Promise<TGame> => {
    if (!game.coverPath) return game;

    try {
      const response = await fetchWithAuth(game.coverPath);
      if (!response.ok) return game;

      const blob = await response.blob();
      const coverURL = URL.createObjectURL(blob);
      return { ...game, coverURL };
    } catch (error) {
      console.error(`Failed to load image for ${game.name}:`, error);
      return game;
    }
  };

  const handleGameSelect = async (slug: string) => {
    try {
      const response = await fetchWithAuth(`/api/games/${slug}`);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const gameDetails = await response.json();
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
