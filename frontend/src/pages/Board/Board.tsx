import { useEffect, useState } from "react";
import Layout from "../../components/Layout";
import CollapsibleList from "./components/CollapsibleList";

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
        const data: TGame[] = await response.json();
        console.log(data);
        setGames(data);
      } catch (error) {
        console.error("Failed to load ROM:", error);
      }
    };

    loadGames();
  }, []);

  const recentlyPlayedGames = games.slice(0, 3);
  const playLaterGames = games.slice(3, 6);
  const recentlyAddedGames = games.slice(6);

  return (
    <Layout>
      <div className="flex flex-col  h-full w-full py-5 px-5 ">
        <CollapsibleList
          title="Recently Played"
          games={recentlyPlayedGames}
        ></CollapsibleList>
        <CollapsibleList
          title="Play Later"
          games={playLaterGames}
        ></CollapsibleList>
        <CollapsibleList
          title="Recently Added"
          games={recentlyAddedGames}
        ></CollapsibleList>
      </div>
    </Layout>
  );
}
