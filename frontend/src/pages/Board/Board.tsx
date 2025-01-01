import { useEffect, useState } from "react";
import Layout from "../../components/Layout/MainLayout";
import CollapsibleList from "./components/CollapsibleList";
import { useGameboy } from "../../context/GameboyContext";
import GameInfo from "../Library/components/GameInfo";
import { useAuth } from "../../context/AuthContext";
import { TGame } from "../../types";
import { loadGames } from "../../api/api"; // Assuming loadGames is already implemented and reusable
import { SortType } from "../../context/OptionsContext";

export default function Board() {
  const [recentlyAddedGames, setRecentlyAddedGames] = useState<TGame[]>([]);
  const [recentlyPlayedGames, setRecentlyPlayedGames] = useState<TGame[]>([]);
  const [playLaterGames, setPlayLaterGames] = useState<TGame[]>([]);
  const { fetchWithAuth, user } = useAuth();

  const [openMenuId, setOpenMenuId] = useState<string | null>(null);
  const updateOpenMenuId = (id: string) => {
    setOpenMenuId(id);
  };

  const { currentGame, setCurrentGame } = useGameboy();

  useEffect(() => {
    const fetchGames = async () => {
      if (!user) return;

      try {
        // Recently Added
        const result = await loadGames(
          fetchWithAuth,
          1,
          "",
          5,
          SortType.DATE_NEW
        );
        if (result) {
          setRecentlyAddedGames(result.gamesWithImages);
        }

        // User Library
        const response = await fetchWithAuth(`/api/users/${user.id}/library`);
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        const libraryGames: TGame[] = await response.json();
        console.log(response, libraryGames);
        setPlayLaterGames(libraryGames);

        // Recently Played during last week
        const recentlyPlayedResponse = await fetchWithAuth(
          `/api/users/${user.id}/recently-played`
        );
        if (recentlyPlayedResponse.ok) {
          const recentGames: TGame[] = await recentlyPlayedResponse.json();
          setRecentlyPlayedGames(recentGames);
        }
      } catch (error) {
        console.error("Failed to load games:", error);
      }
    };

    fetchGames();
    setCurrentGame(null);
  }, [fetchWithAuth, user, setCurrentGame]);

  return (
    <Layout>
      {!currentGame ? (
        <div className="flex flex-col h-full w-full ">
          <CollapsibleList
            title="Recently Played"
            games={recentlyPlayedGames}
            openMenuId={openMenuId}
            updateOpenMenuId={updateOpenMenuId}
          />
          <CollapsibleList
            title="My Library"
            games={playLaterGames}
            openMenuId={openMenuId}
            updateOpenMenuId={updateOpenMenuId}
          />
          <CollapsibleList
            title="Recently Added"
            games={recentlyAddedGames}
            openMenuId={openMenuId}
            updateOpenMenuId={updateOpenMenuId}
          />
        </div>
      ) : (
        <GameInfo />
      )}
    </Layout>
  );
}
