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
    <div className=" flex flex-col w-full h-full ">
      <img src={image} alt={title} />
          <h2>{title}</h2>
          <button onClick={handleClick}>Play</button>
    </div>
  );
}


