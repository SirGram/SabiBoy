import { Play } from "lucide-react";
import { useNavigate } from "react-router-dom";

type GameCardProps = {
  title: string;
  image: string;
  rom_path: string;
};
export default function GameCard({ title, image, rom_path }: GameCardProps) {
  const navigate = useNavigate();

  const handleClick = () => {
    navigate(`/${rom_path}`);
  };
  return (
    <div className=" flex flex-col w-40 rounded-lg bg-gray-950 h-full overflow-hidden">
      <div className="flex items-center h-20 justify-center py-2">
        <h2 className=" w-full text-center   text-ellipsis">{title}</h2>
      </div>
      <img src={image} alt={title} className="w-full h-full" />
      <button
        onClick={handleClick}
        className=" p-10 flex justify-center items-center hover:bg-gray-900 transition-colors"
      >
        <Play className="text-white" />
      </button>
    </div>
  );
}
