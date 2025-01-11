import { useEffect, useState } from "react";
import GameCard from "../../Library/components/GameCard";
import { useGameboy } from "../../../context/GameboyContext";
import { useNavigate } from "react-router-dom";
import { TGame } from "../../../types";
import { loadGameImage } from "../../../api/api";
import api from "../../../api/client";
import { BookMarkedIcon, PlayCircleIcon } from "lucide-react";

type GameListProps = {
  games: TGame[];
  openMenuId: string | null;
  updateOpenMenuId: (id: string) => void;
};

export default function GameList({
  games,
}: GameListProps) {
  const { setCurrentGame } = useGameboy();
  const [loadedGames, setLoadedGames] = useState<TGame[]>([]);

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
    if (games.length > 0 && loadedGames.length === 0) {
      Promise.all(games.map(loadGameImage))
        .then((gamesWithImages) => {
          setLoadedGames(gamesWithImages);
        })
        .catch((error) => {
          console.error("Failed to load game images:", error);
        });
    }
  }, [games]);
  const navigate = useNavigate();

  return (
    <>
      {loadedGames.length > 0 ? (
        loadedGames.map((game) => (
          <div className="relative">
            {/* Wrapper for the Bookmark Button */}
            <div className="absolute top-0 right-0 z-20">
              <button
                onClick={() => navigate(`/library/${game.slug}`)}
                className="flex gap-1 bg-black/70 backdrop-blur-sm rounded-bl-md p-2 opacity-100 transition-opacity"
              title="See more details"
              >
                <BookMarkedIcon size={25} className="hover:scale-110" />
              </button>
            </div>

            {/* Wrapper for the GameCard and Play Icon */}
            <div className="relative group">
              {/* Play Icon (visible on hover) */}
              <div className="absolute inset-0 z-10 w-full h-full flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none">
                <PlayCircleIcon size={40} className="text-white" />
              </div>

              {/* GameCard */}
              <GameCard
                game={game}
                onClick={() => handleGameSelect(game.slug)}
              />
            </div>
          </div>
        ))
      ) : (
        <p className="text-gray-500">No games in this category</p>
      )}
    </>
  );
}
