import { ArrowLeft, Book, Calendar, Star, Tag, Users } from "lucide-react";
import { useGameboy } from "../../../context/GameboyContext";
import { useNavigate } from "react-router-dom";
import { useImageLoader } from "../../../hooks/hooks";

export default function GameInfo() {
  const { currentGame, setCurrentGame } = useGameboy();
  const navigate = useNavigate();

  const handlePlayGame = () => {
    navigate(`/emulator`);
    console.log("Launching game:", currentGame?.name);
  };

  if (!currentGame) return null;

  const formatDate = (date: string | undefined) => {
    if (!date) return "Unknown";
    try {
      return new Intl.DateTimeFormat("en-US", {
        year: "numeric",
        month: "long",
        day: "numeric",
      }).format(new Date(date));
    } catch {
      return "Unknown";
    }
  };

  const placeholder = "/placeholder-image.png"
  const coverImageURL = useImageLoader(currentGame.coverPath) || placeholder;

  return (
    <div className="flex flex-col items-start p-6 md:min-w-[400px] bg-base-background overflow-y-auto shadow-lg border-t md:border-t-0 md:border-l border-base-border">
      <button
        onClick={() => setCurrentGame(null)}
        className="flex items-center mb-4 text-base-foreground/60 hover:text-base-foreground"
      >
        <ArrowLeft className="mr-2" /> Back to Library
      </button>

      <div className="flex w-full mb-6 gap-10">
        <img
          src={coverImageURL}
          alt={`${currentGame.name ?? "Unknown Game"} cover`}
          className="h-80 object-cover rounded-lg shadow-md"
        />
        <div>
          <h1 className="mb-1">{currentGame.name ?? "Untitled Game"}</h1>
          {currentGame.originalTitle && (
            <p className="text-base-foreground/60 mb-4">
              Original: {currentGame.originalTitle}
            </p>
          )}
          <div className="flex gap-4">
            <button
              onClick={handlePlayGame}
              className="mb-4 py-2 px-4 rounded-md bg-primary hover:bg-primary-hover transition-colors"
            >
              Play Game
            </button>
            <button className="mb-4 py-2 px-4 rounded-md bg-primary hover:bg-primary-hover transition-colors">
              Add to Board
            </button>
          </div>

          <div className="mb-4">
            <h3 className="text-xl font-semibold mb-2 flex items-center">
              <Book className="mr-2 text-blue-600" size={20} /> Description
            </h3>
            <p className="leading-relaxed">
              {currentGame.description ?? "No description available."}
            </p>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4 mb-4">
        <div className="flex items-center">
          <Calendar className="mr-2 text-green-600" size={18} />
          <span className="font-medium text-base-foreground/60">
            Release: {formatDate(currentGame.releaseDate)}
          </span>
        </div>
        <div className="flex items-center">
          <Tag className="mr-2 text-purple-600" size={18} />
          <span className="font-medium text-base-foreground/60">
            Genres: {currentGame.genres?.join(", ") ?? "No genres"}
          </span>
        </div>
        <div className="flex items-center">
          <Users className="mr-2 text-red-600" size={18} />
          <span className="font-medium text-base-foreground/60">
            Developers: {currentGame.developers?.join(", ") ?? "Unknown"}
          </span>
        </div>
        <div className="flex items-center">
          <Star className="mr-2 text-yellow-500" size={18} />
          <span className="font-medium text-base-foreground/60">
            Rating: {currentGame.rating?.toFixed(2) ?? "N/A"} / 100
          </span>
        </div>
      </div>

      {/* Screenshots Section */}
      {currentGame.screenshotPaths && currentGame.screenshotPaths.length > 0 && (
        <div className="mb-4">
          <h3 className="text-xl font-semibold mb-2">Screenshots</h3>
          <div className="flex flex-wrap gap-4">
            {currentGame.screenshotPaths.map((screenshot, index) => {
              const screenshotURL = useImageLoader(screenshot) || placeholder;
              return (
                <img
                  key={index}
                  src={screenshotURL}
                  alt={`Screenshot ${index + 1}`}
                  className="h-36 object-cover rounded-lg shadow-md"
                />
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
}
