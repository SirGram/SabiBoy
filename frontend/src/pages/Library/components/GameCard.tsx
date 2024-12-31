import { TGame } from "../../../types";


type GameCardProps = {
  game: TGame;
  onClick: () => void;
};

export default function GameCard({ game, onClick }: GameCardProps) {
  const hasImage = game.coverURL || game.coverPath;

  return (
    <div className="flex flex-col w-40 h-52 rounded-lg bg-transparent hover:outline outline-accent overflow-hidden">
      <button className="w-full h-full" onClick={onClick}>
        {hasImage ? (
          <img
            src={game.coverURL || game.coverPath}
            alt={game.name}
            className="w-full h-full object-fill"
            title={game.name}
          />
        ) : (
          <div
            className="w-full h-full flex items-center justify-center bg-gray-200 text-gray-600"
            title={game.name}
          >
            No image available
          </div>
        )}
      </button>
    </div>
  );
}