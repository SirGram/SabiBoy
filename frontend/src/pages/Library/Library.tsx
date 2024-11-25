import GameCard from "./components/GameCard";
import tetrisImage from "../../../test/tetris.jpg";
import tennisImage from "../../../test/tennis.png";

export default function Library() {
  type Game = {
    id: number;
    title: string;
    image: string;
    rom_path: string;
  };
  const games: Game[] = [
    {
      id: 1,
      title: "Tetris",
      image: tetrisImage,
      rom_path: "tetris",
    },
    {
      id: 2,
      title: "Tennis",
      image: tennisImage,
      rom_path: "tennis",
    },
  ];
  return (
    <div className="">
      <h1>Library</h1>
      <div className="grid grid-cols-4 gap-10 w-full h-full justify-center items-center">
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
