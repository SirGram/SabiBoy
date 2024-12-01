import { useEffect, useState } from "react";
import GameCard from "./components/GameCard";
import Layout from "../../components/Layout";

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
      <div className="flex flex-wrap gap-4 h-full  justify-center py-20">
        {games.map((game) => (
          <GameCard
            key={String(game.id)}
            game={game}
          />
        ))}
      </div>
    </Layout>
  );
}
