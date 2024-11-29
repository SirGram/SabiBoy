import { useEffect, useState } from "react";
import GameCard from "./components/GameCard";
import { useGameboy } from "../../context/GameboyContext";

export default function Library() {
  const [games, setGames] = useState<Game[]>([]);
  const { setRomData } = useGameboy();

  type Game = {
    id: string;
    name: string;
    coverPath?: string;
    romPath?: string;
  };
  useEffect(() => {
    const loadGames = async () => {
      try {
        const response = await fetch("api/games");
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data: Game[] = await response.json();
        console.log(data);
        setGames(data);
      } catch (error) {
        console.error("Failed to load ROM:", error);
      }
    };

    loadGames();
  }, []);

  return (
    <div className="flex  gap-10 h-full  justify-center items-center">
      {games.map((game) => (
        <GameCard
          key={String(game.id)}
          name={game.name}
          coverPath={game.coverPath}
        />
      ))}
    </div>
  );
}
