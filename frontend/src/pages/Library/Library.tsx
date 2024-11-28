import { useEffect, useState } from "react";
import GameCard from "./components/GameCard";
import { Game } from "../../types";
import drMarioImage from "../../../../test/Library/Doctor Mario/cover.jpg";

export default function Library() {
  const [games, setGames] = useState<Game[]>([]);

  useEffect(() => {
    const games: Game[] = [
      {
        id: 1,
        title: "Doctor Mario",
        image: drMarioImage,
        rom_path: "dr_mario",
      },
    ];
    setGames(games);
  }, []);

  return (
    <div className="">
      <h1>Library</h1>
      <div className="flex flex-cols-4 gap-10 w-full h-full justify-center items-center">
        {games.map((game) => (
          <GameCard
            key={String(game.id)}
            title={game.title}
            image={game.image}
            rom_path={game.rom_path}
          />
        ))}
      </div>
    </div>
  );
}
