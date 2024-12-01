import { Play } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { useGameboy } from "../../../context/GameboyContext";
import { TGame } from "../Library";
type GameCardProps = {
  game: TGame;
};
export default function GameCard({ game }: GameCardProps) {
  const navigate = useNavigate();
  const { setCurrentGame } = useGameboy();

  const handleClick = () => {
    setCurrentGame(game);
    console.log(game);
    navigate(`/emulator`);
  };
  return (
    <div className=" flex flex-col w-40 h-72 rounded-lg bg-muted/20  overflow-hidden">
      <div className="flex items-center h-20 justify-center py-2">
        <h2 className=" w-full text-center   text-ellipsis">{game.name}</h2>
      </div>
      <img
        src={game.coverPath}
        alt={game.name}
        className="w-full h-full object-cover"
      />
      <button
        onClick={handleClick}
        className=" p-2 flex h-full justify-center items-center hover:bg-primary transition-colors"
      >
        <Play className="text-base-foreground" />
      </button>
    </div>
  );
}
