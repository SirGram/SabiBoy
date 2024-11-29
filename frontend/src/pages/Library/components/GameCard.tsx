import { Play } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { useGameboy } from "../../../context/GameboyContext";

type GameCardProps = {
  name: string;
  coverPath?: string;
};
export default function GameCard({ name, coverPath }: GameCardProps) {
  const navigate = useNavigate();
  const { setRomData } = useGameboy();

  const handleClick = () => {
    navigate(`/${name}`);
  };
  return (
    <div className=" flex flex-col w-40 h-72 rounded-lg bg-muted/20  overflow-hidden">
      <div className="flex items-center h-20 justify-center py-2">
        <h2 className=" w-full text-center   text-ellipsis">{name}</h2>
      </div>
      <img src={coverPath} alt={name} className="w-full h-full object-cover" />
      <button
        onClick={handleClick}
        className=" p-2 flex h-full justify-center items-center hover:bg-primary transition-colors"
      >
        <Play className="text-base-foreground" />
      </button>
    </div>
  );
}
