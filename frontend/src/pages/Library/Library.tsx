import { useEffect, useState } from "react";
import GameCard from "./components/GameCard";
import Layout from "../../components/Layout";
import { SearchIcon } from "lucide-react";
import GameInfo from "./components/GameInfo";
import { useGameboy } from "../../context/GameboyContext";

export type TGame = {
  id: string;
  name: string;
  coverPath?: string;
  romPath?: string;
};
export default function Library() {
  const [games, setGames] = useState<TGame[]>([]);

  useEffect(() => {
    const loadGames = async () => {
      try {
        const response = await fetch("api/games");
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data: TGame[] = await response.json();
        console.log(data);
        setGames(data);
      } catch (error) {
        console.error("Failed to load ROM:", error);
      }
    };

    loadGames();
    setCurrentGame(null)
  }, []);

  const {currentGame, setCurrentGame} = useGameboy()

  return (
    <Layout>
      <div className="flex w-full">
        {!currentGame ? (
          <div className="flex flex-col py-10 px-5 w-full">
            <SearchBar />
            <div className="flex flex-wrap gap-4 pt-10 justify-center w-full">
              {games.map((game) => (
                <GameCard 
                  key={String(game.id)} 
                  game={game} 
                  onClick={() => setCurrentGame(game)}
                />
              ))}
            </div>
          </div>
        ) : (
          <GameInfo 
          />
        )}
      </div>
    </Layout>
  );
}

function SearchBar() {
  const [searchTerm, setSearchTerm] = useState("");

  const handleSearchChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSearchTerm(event.target.value);
  };

  return (
    <div className="flex items-center gap-2 w-full ">
      <SearchIcon />
      <input
        type="text"
        value={searchTerm}
        onChange={handleSearchChange}
        placeholder="Search..."
        className="w-full rounded-lg border border-base-border bg-base-background px-4 py-2 text-sm"
      />
    </div>
  );
}
