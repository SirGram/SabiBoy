import { useImageLoader } from "../../../hooks/hooks";
import { TGame } from "../Library";
type GameCardProps = {
  game: TGame;
  onClick: () => void;
};

export default function GameCard({ game, onClick }: GameCardProps) {
  const imageUrl = useImageLoader(game.coverPath) || "/placeholder-image.png";

  return (
    <div className="flex flex-col w-40 rounded-lg bg-transparent hover:outline outline-accent overflow-hidden">
      <button className="w-full h-full" onClick={onClick}>
        <img
          src={imageUrl}
          alt={game.name}
          className="w-full h-full object-fill"
          title={game.name}
        />
      </button>
    </div>
  );
}
