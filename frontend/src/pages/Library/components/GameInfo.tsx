import { ArrowLeft, Book, Calendar, Star, Tag, Users } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { useGameboy } from "../../../context/GameboyContext";
import { useEffect, useState } from "react";

type GameInfo = {
  id: number;
  alternative_names: {
    id: number;
    comment: string;
    game: number;
    name: string;
    checksum: string;
  }[];
  cover: {
    path: string;
  };
  first_release_date: number; // Unix timestamp
  genres: {
    id: number;
    name: string;
  }[];
  involved_companies: {
    id: number;
    company: {
      id: number;
      name: string;
    };
  }[];
  name: string;
  rating: number;
  screenshots: {
    path: string;
  }[];
  slug: string;
  summary: string;
  total_rating: number;
  language_supports: {
    id: number;
    game: number;
    language: number;
    language_support_type: number;
    created_at: number;
    updated_at: number;
    checksum: string;
  }[];
};

export default function GameInfo() {
  const formatDate = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleDateString();
  };

  const navigate = useNavigate();
  const { currentGame, setCurrentGame } = useGameboy();
  const handleClick = () => {
    navigate(`/emulator`);
  };
  useEffect(() => {
    const fetchGameInfo = async () => {
      if (!currentGame) return;
      try {
        const response = await fetch(
          `api/games/${currentGame.name}/metadata.json`
        );
        const data = await response.json();
        setGameInfo(data);
      } catch (error) {
        console.error("Error fetching game info:", error);
      }
    };
    fetchGameInfo();
    console.log(gameInfo);
  }, [currentGame]);
  const [gameInfo, setGameInfo] = useState<GameInfo | null>(null);
  return (
    gameInfo && (
      <div className="flex flex-col items-start p-6 md:min-w-[400px] bg-base-background overflow-y-auto shadow-lg border-t md:border-t-0 md:border-l border-base-border">
        <button
          onClick={() => setCurrentGame(null)}
          className="flex items-center mb-4 text-base-foreground/60 hover:text-base-foreground"
        >
          <ArrowLeft className="mr-2" /> Back to Library
        </button>

        <div className="flex w-full mb-6 gap-10">
          <img
            src={gameInfo.cover.path}
            alt={`${gameInfo.name} cover`}
            className=" h-80 object-cover rounded-lg shadow-md"
          />
          <div>
            <h1 className=" mb-1">{gameInfo.name}</h1>

            {gameInfo.alternative_names &&
              gameInfo.alternative_names.length > 0 && (
                <div className="mb-4 flex flex-col">
                  {gameInfo.alternative_names
                    .filter((altName) =>
                      altName.comment.toLowerCase().includes("original")
                    )
                    .map((altName, index) => (
                      <p key={index} className="text-base-foreground/60">
                        Original: {altName.name}
                      </p>
                    ))}
                </div>
              )}
            <button
              onClick={handleClick}
              className="mb-4 py-2 px-4 rounded-md bg-primary hover:bg-primary-hover transition-colors"
            >
              Play Game
            </button>

            <div className="mb-4">
              <h3 className="text-xl font-semibold mb-2 flex items-center">
                <Book className="mr-2 text-blue-600" size={20} /> Description
              </h3>
              <p className="leading-relaxed">{gameInfo.summary}</p>
            </div>
          </div>
        </div>
        <div className="grid grid-cols-2 gap-4 mb-4">
          <div className="flex items-center">
            <Calendar className="mr-2 text-green-600" size={18} />
            <span className="font-medium text-base-foreground/60">
              Release: {formatDate(gameInfo.first_release_date)}
            </span>
          </div>
          <div className="flex items-center">
            <Tag className="mr-2 text-purple-600" size={18} />
            <span className="font-medium text-base-foreground/60">
              Genres: {gameInfo.genres.map((genre) => genre.name).join(", ")}
            </span>
          </div>
          <div className="flex items-center">
            <Users className="mr-2 text-red-600" size={18} />
            <span className="font-medium text-base-foreground/60">
              Developers:{" "}
              {gameInfo.involved_companies
                .map((company) => company.company.name)
                .join(", ")}
            </span>
          </div>
          <div className="flex items-center">
            <Star className="mr-2 text-yellow-500" size={18} />
            <span className="font-medium text-base-foreground/60">
              Rating: {gameInfo.total_rating.toFixed(2)} / 100
            </span>
          </div>
        </div>

        {gameInfo.screenshots && gameInfo.screenshots.length > 0 && (
          <div className="mb-4">
            <h3 className="text-xl font-semibold mb-2">Screenshots</h3>
            <div className="flex flex-wrap gap-4">
              {gameInfo.screenshots.map((screenshot, index) => (
                <img
                  key={index}
                  src={screenshot.path}
                  alt={`Screenshot ${index + 1}`}
                  className=" h-36 object-cover rounded-lg shadow-md"
                />
              ))}
            </div>
          </div>
        )}
      </div>
    )
  );
}
