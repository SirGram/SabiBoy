import React from "react";
import { TGame } from "../../../types";

type GameCardProps = {
  game: TGame;
  onClick: () => void;
};

export default function GameCard({ game, onClick }: GameCardProps) {
  const hasImage = game.coverURL ;
  const lang = game.language?.toUpperCase() || "EN";
  console.log(game.coverURL)

  return (
    <div className="group w-40 h-auto flex flex-col gap-2">
      <button
        className="relative w-40 h-52 rounded-lg overflow-hidden transition-transform duration-200  hover:shadow-lg hover:shadow-primary/20"
        onClick={onClick}
      >
        {/* Background glow effect on hover */}
        <div className="absolute inset-0 opacity-0 group-hover:opacity-100 bg-primary/10 transition-opacity duration-200" />

        {/* Image or fallback */}
        {hasImage ? (
          <img
            src={game.coverURL }
            alt={game.name}
            className="w-full h-full object-cover rounded-lg hover:scale-110"
            title={game.name}
          />
        ) : (
          <div
            className="w-full h-full flex items-center justify-center  text-base-foreground/20 bg-muted/20"
            title={game.name}
          >
            No image available
          </div>
        )}

        {/* Overlay with game name on hover */}
        <div className="absolute inset-0 flex flex-col justify-end bg-gradient-to-t from-black/90 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-200 p-3">
          <span className="text-white text-sm font-medium " title={game.name}>
            {game.name}
          </span>
        </div>

        {/* Language badge */}
        <div className="absolute top-2 left-2 flex gap-1 bg-black/70 backdrop-blur-sm rounded-md px-2 py-1">
          <span className="text-xs font-medium text-white" title={lang}>
            {lang}
          </span>
        </div>
      </button>
    </div>
  );
}
