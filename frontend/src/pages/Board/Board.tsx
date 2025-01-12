import { useEffect, useState } from "react";
import Layout from "../../components/Layout/MainLayout";
import CollapsibleList from "../../components/CollapsibleList";
import { useGameboy } from "../../context/GameboyContext";
import { useAuth } from "../../context/AuthContext";
import { TGame } from "../../types";
import { loadGames } from "../../api/api";
import { SortType } from "../../context/OptionsContext";
import api from "../../api/client";
import GameList from "./components/GameList";

export default function Board() {
  const [recentlyPlayedGames, setRecentlyPlayedGames] = useState<TGame[]>([]);
  const [playLaterGames, setPlayLaterGames] = useState<TGame[]>([]);
  const [recentlyAddedGames, setRecentlyAddedGames] = useState<TGame[]>([]);
  const { user } = useAuth();

  const [openMenuId, setOpenMenuId] = useState<string | null>(null);
  const updateOpenMenuId = (id: string) => {
    setOpenMenuId(id);
  };

  const {  setCurrentGame } = useGameboy();

  useEffect(() => {
    const fetchGames = async () => {
      if (!user) return;

      try {
        // User Library
        const libraryResponse = await api.get(`/api/users/${user.id}/library`);
        if (libraryResponse.status === 200) {
          const libraryGames: TGame[] = libraryResponse.data;
          setPlayLaterGames(libraryGames);
        } else {
          throw new Error(
            `Failed to fetch user library: ${libraryResponse.status}`
          );
        }

        // Recently Played Games
        const recentlyPlayedResponse = await api.get(
          `/api/users/${user.id}/recently-played`
        );
        if (recentlyPlayedResponse.status === 200) {
          const recentGames: TGame[] = recentlyPlayedResponse.data;
          setRecentlyPlayedGames(recentGames);
        }
        // Recently Added Games
        const result = await loadGames(1, "", 5, SortType.DATE_NEW);
        if (result) {
          setRecentlyAddedGames(result.gamesWithImages);
        }
      } catch (error) {
        console.error("Failed to load games:", error);
      }
    };

    fetchGames();
    setCurrentGame(null);
  }, [user, setCurrentGame]);

  return (
    <Layout>
      <div className="flex flex-col h-full w-full ">
        <CollapsibleList title="Recently Played">
          <GameList
            games={recentlyPlayedGames}
            openMenuId={openMenuId}
            updateOpenMenuId={updateOpenMenuId}
          />
        </CollapsibleList>
        <CollapsibleList title="My Library">
          <GameList
            games={playLaterGames}
            openMenuId={openMenuId}
            updateOpenMenuId={updateOpenMenuId}
          />
        </CollapsibleList>
        <CollapsibleList title="Recently Added">
          <GameList
            games={recentlyAddedGames}
            openMenuId={openMenuId}
            updateOpenMenuId={updateOpenMenuId}
          />
        </CollapsibleList>
      </div>
    </Layout>
  );
}
