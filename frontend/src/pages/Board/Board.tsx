import { useEffect, useState } from "react";
import Layout from "../../components/Layout";
import CollapsibleList from "./components/CollapsibleList";
import { useGameboy } from "../../context/GameboyContext";
import GameInfo from "../Library/components/GameInfo";
import { TPaginatedResponse } from "../Library/Library";

export type TGame = {
  id: string;
  name: string;
  coverPath?: string;
  romPath?: string;
};
export default function Board() {
  const [games, setGames] = useState<TGame[]>([]);

  useEffect(() => {
    const loadGames = async () => {
      try {
        const response = await fetch("api/games");
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data: TPaginatedResponse = await response.json();
        console.log(data);
        setGames(data.games);
      } catch (error) {
        console.error("Failed to load ROM:", error);
      }
    };

    loadGames();
    setCurrentGame(null);
  }, []);

  const recentlyPlayedGames = games.slice(0, 3);
  const playLaterGames = games.slice(3, 6);
  const recentlyAddedGames = games.slice(6);

  const [openMenuId, setOpenMenuId] = useState<string | null>(null);
  const updateOpenMenuId = (id: string) => {
    setOpenMenuId(id);
  };

  const { currentGame, setCurrentGame } = useGameboy();

  return (
    <Layout>
      <div className="flex w-full"></div>
      {!currentGame ? (
        <div className="flex flex-col  h-full w-full py-5 px-5 ">
          <CollapsibleList
            title="Recently Played"
            games={recentlyPlayedGames}
            openMenuId={openMenuId}
            updateOpenMenuId={updateOpenMenuId}
          ></CollapsibleList>
          <CollapsibleList
            title="Play Later"
            games={playLaterGames}
            openMenuId={openMenuId}
            updateOpenMenuId={updateOpenMenuId}
          ></CollapsibleList>
          <CollapsibleList
            title="Recently Added"
            games={recentlyAddedGames}
            openMenuId={openMenuId}
            updateOpenMenuId={updateOpenMenuId}
          ></CollapsibleList>
        </div>
      ) : (
        <GameInfo />
      )}
    </Layout>
  );
}
