import GameCard from "./components/GameCard";

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
      image: '',
      rom_path: "tetris",
    },
    {
      id: 2,
      title: "Dr Mario",
      image: '',
      rom_path: "dr_mario",
    },
  ];
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
