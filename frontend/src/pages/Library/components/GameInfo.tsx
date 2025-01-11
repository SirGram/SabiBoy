import {
  ArrowLeft,
  Calendar,
  Plus,
  Star,
  Tag,
  Users,
  X,
  ImageOff,
  EarthIcon,
  Clock,
  Trash2,
  Upload,
} from "lucide-react";
import { useGameboy } from "../../../context/GameboyContext";
import { useNavigate, useParams } from "react-router-dom";
import { useAuth } from "../../../context/AuthContext";
import { useEffect, useRef, useState } from "react";
import Layout from "../../../components/Layout/MainLayout";
import { loadGameInfo } from "../../../api/api";
import api from "../../../api/client";

const getDominantColor = (
  imgEl: HTMLImageElement
): Promise<{ primary: string; accent: string }> => {
  return new Promise((resolve) => {
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d");

    canvas.width = imgEl.width;
    canvas.height = imgEl.height;

    if (!ctx) {
      resolve({ primary: "rgb(0,0,0)", accent: "rgb(0,0,0)" });
      return;
    }

    ctx.drawImage(imgEl, 0, 0);
    const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height).data;
    const colorCounts: { [key: string]: number } = {};

    // Sample pixels with better color filtering
    for (let i = 0; i < imageData.length; i += 16) {
      const r = imageData[i];
      const g = imageData[i + 1];
      const b = imageData[i + 2];

      // Skip colors that are too dark or too light
      const brightness = (r + g + b) / 3;
      if (brightness < 30 || brightness > 225) continue;

      const rgb = `${r},${g},${b}`;
      colorCounts[rgb] = (colorCounts[rgb] || 0) + 1;
    }

    const sortedColors = Object.entries(colorCounts)
      .sort((a, b) => b[1] - a[1])
      .slice(0, 2)
      .map(([color]) => `rgb(${color})`);

    resolve({
      primary: sortedColors[0] || "rgb(0,0,0)",
      accent: sortedColors[1] || sortedColors[0] || "rgb(0,0,0)",
    });
  });
};

export default function GameInfo() {
  const { currentGame, setCurrentGame } = useGameboy();
  const navigate = useNavigate();
  const [isInLibrary, setIsInLibrary] = useState(false);
  const { user } = useAuth();
  const [screenshotURLs, setScreenshotURLs] = useState<(string | null)[]>([]);
  const { gameslug } = useParams<{ gameslug: string }>();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [lastPlayed, setLastPlayed] = useState<string | null>(null);
  const [hasSaveState, setHasSaveState] = useState(false);
  const [colors, setColors] = useState<{ primary: string; accent: string }>({
    primary: "rgb(0,0,0)",
    accent: "rgb(0,0,0)",
  });

  useEffect(() => {
    const fetchGameDetails = async () => {
      if (!user || !gameslug) return;
      try {
        const gameDetails = await loadGameInfo(gameslug);
        if (gameDetails) {
          setCurrentGame(gameDetails);
        }
        console.log(gameDetails);
      } catch (error) {
        console.error("Failed to load game details:", error);
      }
    };

    fetchGameDetails();
  }, [gameslug, user, setCurrentGame]);

  useEffect(() => {
    const loadScreenshots = async () => {
      if (!currentGame?.screenshotPaths) {
        setScreenshotURLs([]);
        return;
      }

      const urls = await Promise.all(
        currentGame.screenshotPaths.map(async (path) => {
          if (!path) return null;
          try {
            const { data } = await api.get(path, { responseType: "blob" });
            return URL.createObjectURL(data);
          } catch (error) {
            console.error("Failed to load screenshot:", error);
            return null;
          }
        })
      );
      setScreenshotURLs(urls);
    };

    loadScreenshots();

    return () => {
      screenshotURLs.forEach((url) => {
        if (url) URL.revokeObjectURL(url);
      });
    };
  }, [currentGame?.screenshotPaths]);

  useEffect(() => {
    const extractColor = async () => {
      if (!currentGame?.coverURL) return;

      const img = new Image();
      img.crossOrigin = "Anonymous";
      img.src = currentGame?.coverURL;

      img.onload = async () => {
        const extractedColors = await getDominantColor(img);
        setColors(extractedColors);
      };
    };

    extractColor();
  }, [currentGame?.coverURL]);

  useEffect(() => {
    const checkGameStatus = async () => {
      if (!user || !currentGame) return;
      try {
        const { data } = await api.get(
          `/api/users/${user.id}/library/${currentGame.slug}/status`
        );
        setHasSaveState(data.hasSaveState);
        setLastPlayed(data.lastPlayed);
        setIsInLibrary(data.isInLibrary);
      } catch (error) {
        console.error("Failed to check game status:", error);
      }
    };

    checkGameStatus();
  }, [currentGame, user]);

  const handlePlayGame = () => {
    navigate(`/emulator`);
  };

  const handleToggleLibrary = async () => {
    if (!user || !currentGame) return;

    try {
      if (isInLibrary) {
        await api.delete(`/api/users/${user.id}/library`, {
          data: { slug: currentGame.slug },
        });
      } else {
        await api.post(`/api/users/${user.id}/library`, {
          slug: currentGame.slug,
        });
      }
      setIsInLibrary(!isInLibrary);
    } catch (error) {
      console.error("Failed to toggle library status:", error);
    }
  };

  const handleStateUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file || !user || !currentGame) return;

    const reader = new FileReader();
    reader.onload = async (e) => {
      const arrayBuffer = e.target?.result as ArrayBuffer;
      if (!arrayBuffer) return;

      try {
        const stateData = new Uint8Array(arrayBuffer);
        await api.patch(
          `/api/users/${user.id}/library/${currentGame.slug}/save-state`,
          stateData,
          {
            headers: { "Content-Type": "application/octet-stream" },
          }
        );
        setHasSaveState(true);
      } catch (error) {
        console.error("Failed to upload save state:", error);
      }
    };
    reader.readAsArrayBuffer(file);
  };

  const handleResetSaveState = async () => {
    if (!user || !currentGame) return;

    if (!confirm("Are you sure you want to delete this save state?")) return;

    try {
      await api.delete(
        `/api/users/${user.id}/library/${currentGame.slug}/save-state`
      );
      setHasSaveState(false);
    } catch (error) {
      console.error("Failed to reset save state:", error);
    }
  };

  const formatDate = (date: string | Date | null | undefined) => {
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

  const handleBackToLibrary = () => {
    navigate("/library");
  };

  const hasValidScreenshots = screenshotURLs.some((url) => url !== null);

  const handleDeleteFromLibrary = async () => {
    if (!user) return;

    if (
      !confirm("Are you sure you want to delete this game from your library?")
    )
      return;

    try {
      await api.delete(`/api/games/${gameslug}`);
      navigate("/library");
    } catch (error) {
      console.error("Failed to delete game from library:", error);
    }
  };

  return (
    <Layout>
      {/* Subtle accent border based on primary color */}
      <div
        className="absolute inset-0 pointer-events-none opacity-20"
        style={{
          background: `linear-gradient(135deg, ${colors.primary} 100%, transparent 100%)`,
          maskImage: "linear-gradient(to bottom, transparent, black)",
          WebkitMaskImage: "linear-gradient(to bottom, transparent, black)",
          zIndex: 1,
        }}
      />
      <div className="z-10 relative flex flex-col items-start rounded-md md:min-w-[400px] overflow-y-auto  border-base-border max-w-5xl">
        <button
          onClick={handleBackToLibrary}
          className="flex items-center mb-4 text-base-foreground hover:text-base-foreground"
        >
          <ArrowLeft className="mr-2" /> Back to Library
        </button>

        <div className="flex flex-col md:flex-row w-full  gap-10">
          <div className="relative min-w-60 h-80 self-center md:self-start">
            <div className="absolute top-2 left-2 flex gap-1 bg-black/50 rounded-md p-1">
              <span
                key={currentGame?.language}
                className="text-lg"
                title={currentGame?.language.toUpperCase()}
              >
                {currentGame?.language.toUpperCase()}
              </span>
            </div>
            <img
              src={currentGame?.coverURL}
              alt={`${currentGame?.name ?? "Unknown Game"} cover`}
              className="w-60 h-80 object-cover rounded-lg shadow-md"
            />
          </div>
          <div className="text-center md:text-left w-full flex flex-col  items-center md:items-start">
            <h1 className="mb-1">{currentGame?.name ?? "Untitled Game"}</h1>
            {currentGame?.originalTitle && (
              <p className="text-base-foreground mb-4">
                Original: {currentGame?.originalTitle}
              </p>
            )}
            <div className="w-full  h-1 bg-base-border mt-4 mb-4 self-center"></div>
            <div className="flex w-full max-w-3xl mx-auto justify-center items-center md:items-stretch  flex-col md:flex-row gap-4 mt-2 mb-6">
              <button
                onClick={handlePlayGame}
                className="flex-1 w-full py-3 px-6 max-w-sm rounded-lg bg-primary hover:bg-primary-hover text-white font-medium transition-colors"
              >
                Play Game
              </button>
              <button
                onClick={handleToggleLibrary}
                className="flex-1 w-full max-w-sm py-3 px-6 rounded-lg bg-secondary hover:bg-secondary-hover text-white font-medium transition-colors flex items-center justify-center"
              >
                {isInLibrary ? (
                  <>
                    <X className="mr-2 min-w-max" size={20} /> Remove from Board
                  </>
                ) : (
                  <>
                    <Plus className="mr-2 min-w-max" size={20} /> Add to Board
                  </>
                )}
              </button>
              {user?.role === "superuser" && (
                <button
                  onClick={handleDeleteFromLibrary}
                  className="flex-1 w-full py-3 px-6 max-w-sm rounded-lg bg-red-500 hover:bg-red-600 text-white font-medium transition-colors"
                >
                  Delete from Library
                </button>
              )}
            </div>
            <div className="w-fit">
              {saveStateSection({
                formatDate,
                fileInputRef,
                handleStateUpload,
                hasSaveState,
                handleResetSaveState,
                lastPlayed: lastPlayed as string | null,
              })}
            </div>
          </div>
        </div>
        <div className="mb-4 bg-base-background/75 p-4 rounded-lg">
          <h3 className="text-xl font-semibold mb-2 flex items-center">
            Description
          </h3>
          <p className="leading-relaxed text-justify font-thin">
            {currentGame?.description ?? "No description available."}
          </p>
        </div>
        <div className="bg-base-background/75 mb-4 p-4 self-center md:self-start rounded-lg">
          <h3 className="text-xl font-semibold mb-2 flex items-center">
            Other Information
          </h3>
          <div className="grid md:grid-cols-2 gap-4  ">
            <div className="flex items-center">
              <Calendar className="mr-2 text-green-600" size={18} />
              <span className=" text-base-foreground font-thin">
                Release: {formatDate(currentGame?.releaseDate)}
              </span>
            </div>
            <div className="flex items-center">
              <Tag className="mr-2 text-purple-600" size={18} />
              <span className="font-thin text-base-foreground">
                Genres: {currentGame?.genres?.join(", ") ?? "No genres"}
              </span>
            </div>
            {currentGame &&
              currentGame.developers &&
              currentGame.developers.length > 0 && (
                <div className="flex items-center">
                  <Users className="mr-2 text-red-600" size={18} />
                  <span className="font-thin text-base-foreground">
                    Developers:{" "}
                    {currentGame?.developers?.join(", ") ?? "Unknown"}
                  </span>
                </div>
              )}
            <div className="flex items-center">
              <Star className="mr-2 text-yellow-500" size={18} />
              <span className="font-thin text-base-foreground">
                Rating: {currentGame?.rating?.toFixed(2) ?? "N/A"} / 100
              </span>
            </div>
            <div className="flex items-center">
              <EarthIcon className="mr-2 text-yellow-500" size={18} />
              <span className="font-thin text-base-foreground">
                Region/Language: {currentGame?.language.toUpperCase()}
              </span>
            </div>
          </div>
        </div>

        <div className="bg-base-background/75 mb-4 p-4 self-center md:self-start rounded-lg">
          <h3 className="text-xl font-semibold mb-2">Screenshots</h3>
          {hasValidScreenshots ? (
            <div className="flex flex-wrap gap-4 w-full items-center justify-center md:justify-normal">
              {screenshotURLs.map((url, index) => {
                if (!url) return null;
                return (
                  <img
                    key={index}
                    src={url}
                    alt={`Screenshot ${index + 1}`}
                    className="h-36 object-cover rounded-lg "
                  />
                );
              })}
            </div>
          ) : (
            <div className="flex flex-col items-center justify-center p-8 border-2 border-dashed border-base-border rounded-lg text-base-foreground">
              <ImageOff size={48} className="mb-2" />
              <p>No screenshots available</p>
            </div>
          )}
        </div>
      </div>
    </Layout>
  );
}

function saveStateSection({
  formatDate,
  fileInputRef,
  handleStateUpload,
  hasSaveState,
  handleResetSaveState,
  lastPlayed,
}: {
  formatDate: (date: Date | string | null | undefined) => string;
  fileInputRef: React.RefObject<HTMLInputElement>;
  handleStateUpload: (event: React.ChangeEvent<HTMLInputElement>) => void;
  hasSaveState: boolean;
  handleResetSaveState: () => void;
  lastPlayed: string | null;
}) {
  return (
    <div className="w-full mb-4 bg-base-background/75 p-4 rounded-lg">
      <h3 className="text-xl font-semibold mb-2 flex items-center">
        User Data
      </h3>

      <div className="flex flex-col gap-4">
        {hasSaveState && (
          <div className="flex items-center">
            <Clock className="mr-2 text-blue-600" size={18} />
            <span className="font-thin text-base-foreground">
              Last played on {formatDate(lastPlayed)}
            </span>
          </div>
        )}

        <div className="flex flex-wrap gap-3">
          <input
            type="file"
            ref={fileInputRef}
            onChange={handleStateUpload}
            accept=".gb.state"
            className="hidden"
          />
          <div className="flex flex-col gap-2 w-full">
            <button
              onClick={() => fileInputRef.current?.click()}
              className="py-2 px-4 rounded-md bg-secondary hover:bg-secondary-hover transition-colors flex items-center "
            >
              <Upload className="mr-2" size={18} />
              Upload Savestate
            </button>

            {hasSaveState && (
              <button
                onClick={handleResetSaveState}
                className="py-2 px-4 rounded-md bg-red-500 hover:bg-red-600 transition-colors flex items-center"
              >
                <Trash2 className="mr-2" size={18} />
                Delete Savestate
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
