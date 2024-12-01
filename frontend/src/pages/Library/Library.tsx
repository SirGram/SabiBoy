import { useEffect, useState } from "react";
import GameCard from "./components/GameCard";
import Layout from "../../components/Layout";
import GameInfoCard from "./components/GameInfo";

export type TGame = {
  id: string;
  name: string;
  coverPath?: string;
  romPath?: string;
};
export default function Library() {
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

  return (
    <Layout>
      <div className="flex flex-col md:flex-row h-full w-full justify-between">
        <div className="flex flex-wrap gap-4  py-20 px-5 justify-center w-full">
          {games.map((game) => (
            <GameCard key={String(game.id)} game={game} />
          ))}
        </div>
        <GameInfoCard />
      </div>
    </Layout>
  );
}
