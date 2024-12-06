import { useState } from "react";
import { ChevronDown, ChevronRight} from "lucide-react";
import GameCard from "../../Library/components/GameCard";
import { TGame } from "../Board";

type CollapsibleListProps = {
  title: string;
  games: TGame[];
  defaultOpen?: boolean;
};

export default function CollapsibleList({
  title,
  games,
  defaultOpen = true,
}: CollapsibleListProps) {
  const [isOpen, setIsOpen] = useState(defaultOpen);

  const toggleCollapse = () => {
    setIsOpen(!isOpen);
  };

  return (
    <div className="flex flex-col gap-4 w-full   rounded-lg p-2">
      <div
        className="flex items-end justify-start cursor-pointer md:max-w-fit text-muted-foreground"
        onClick={toggleCollapse}
      >
        {isOpen ? <ChevronDown /> : <ChevronRight />}
        <h2 className="text-xl font-semibold ">{title}</h2>
      </div>

      {isOpen && (
        <div className="flex flex-wrap gap-4 py-2 px-5 justify-start w-full">
          {games.length > 0 ? (
            games.map((game) => <GameCard key={String(game.id)} game={game} />)
          ) : (
            <p className="text-gray-500">No games in this category</p>
          )}
        </div>
      )}
    </div>
  );
}
