import { TGame } from "../../../types";


type GameCardProps = {
  game: TGame;
  onClick: () => void;
};

export default function GameCard({ game, onClick }: GameCardProps) {
  const hasImage = game.coverURL || game.coverPath;
  const lang = game.language?.toUpperCase()  || 'EN';

  return (
    <div className="relative flex flex-col w-40 h-52 rounded-lg bg-transparent hover:outline outline-accent overflow-hidden">
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
        <div className="absolute bottom-1 right-1 flex gap-1 bg-black/50 rounded-md p-1">
            <span key={lang} className="text-sm" title={lang.toUpperCase()}>
              { lang.toUpperCase()}
            </span>
          </div>
      </button>
    </div>
  );
}