import { useEffect, useState } from "react";
import GameCard from "../../Library/components/GameCard";
import { useGameboy } from "../../../context/GameboyContext";
import { WithContextMenu } from "./WithContextMenu";
import { useNavigate } from "react-router-dom";
import { TGame } from "../../../types";
import { loadGameImage } from "../../../api/api";
import api from "../../../api/client";

type GameListProps = {
  games: TGame[];
  openMenuId: string | null;
  updateOpenMenuId: (id: string) => void;
};

export default function GameList({
  games,
  openMenuId,
  updateOpenMenuId,
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
    if ( games.length > 0 && loadedGames.length === 0) {
      Promise.all(games.map(loadGameImage))
        .then((gamesWithImages) => {
          setLoadedGames(gamesWithImages);
        })
        .catch((error) => {
          console.error("Failed to load game images:", error);
        });
    }
  }, [ games]);
  const navigate = useNavigate();

  return (
    <>
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
        </>
  );
}
