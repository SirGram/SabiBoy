
import { useGameboy } from "../../../context/GameboyContext";
import { TGame } from "../Library";
type GameCardProps = {
  game: TGame;
};
export default function GameCard({ game }: GameCardProps) {
  const { setCurrentGame } = useGameboy();

  return (
    <div className=" flex flex-col w-40 h-52 rounded-lg bg-muted/20  overflow-hidden">
      <button className="w-full h-full" onClick={() => setCurrentGame(game)}>
        <img
          src={game.coverPath}
          alt={game.name}
          className="w-full h-full object-fill"
        />
      </button>
    </div>
  );
}
