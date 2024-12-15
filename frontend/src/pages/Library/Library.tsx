import { useEffect, useState } from "react";
import GameCard from "./components/GameCard";
import Layout from "../../components/Layout";
import { ChevronRight, SearchIcon } from "lucide-react";
import GameInfo from "./components/GameInfo";
import { useGameboy } from "../../context/GameboyContext";
import Pagination from "./components/Pagination";

export type TGame = {
  slug: string;
  name: string;
  coverPath?: string;
};
export type TGameDetails = TGame & {
  romPath?: string;
  screenshotPaths: string[];
  description?: string;
  originalTitle?: string;
  rating?: number;
  releaseDate?: string; // Use string to handle date from JSON
  developers?: string[];
  genres?: string[];
};
export type TPaginatedResponse = {
  games: TGame[];
  total: number;
  page: number;
  totalPages: number;
};

export default function Library() {
  const [games, setGames] = useState<TGame[]>([]);
  const [searchTerm, setSearchTerm] = useState("");
  const [limit, setLimit] = useState(10);
  const [pagination, setPagination] = useState<{
    page: number;
    total: number;
    totalPages: number;
  }>({
    page: 1,
    total: 0,
    totalPages: 0,
  });

  const loadGames = async (page = 1, search = "", limit = 10) => {
    try {
      const url = new URL("/api/games", window.location.origin);
      url.searchParams.set("page", page.toString());
      url.searchParams.set("search", search);
      url.searchParams.set("limit", limit.toString());

      const response = await fetch(url.toString());
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data: TPaginatedResponse = await response.json();
      setGames(data.games);
      setPagination({
        page: data.page,
        total: data.total,
        totalPages: data.totalPages,
      });
    } catch (error) {
      console.error("Failed to load games:", error);
    }
  };

  useEffect(() => {
    loadGames();
    setCurrentGame(null);
  }, []);

  const { currentGame, setCurrentGame } = useGameboy();
  const handlePageChange = (newPage: number) => {
    loadGames(newPage, searchTerm);
  };
  const handleSearch = (term: string) => {
    setSearchTerm(term);
    loadGames(1, term);
  };
  const limitOptions = [10, 20, 50];
  const handleLimitChange = () => {
    const newLimit = limitOptions[(limitOptions.indexOf(limit) + 1) % 3];
    setLimit(newLimit);
    loadGames(1, searchTerm, newLimit);
  };
  const handleGameSelect = async (slug: string) => {
    try {
      const response = await fetch(`/api/games/${slug}`);
      console.log("Response:", response);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const gameDetails: TGameDetails = await response.json();

      setCurrentGame(gameDetails);
    } catch (error) {
      console.error("Failed to load game details:", error);
    }
  };

  return (
    <Layout>
      <div className="flex w-full">
        {!currentGame ? (
          <div className="flex flex-col py-10 px-5 w-full">
            <div className="flex gap-x-4 items-center">
              <SearchBar
                searchTerm={searchTerm}
                onSearchChange={handleSearch}
              />
              <div
                className="h-full flex items-center border border-base-border rounded-md px-2 py-1 
                           transition-colors duration-200 
                           cursor-pointer group"
                onClick={handleLimitChange}
              >
                <div className="flex  items-center text-cente gap-1">
                  <span className="text-sm font-medium " title="Limit">
                    {limit}
                  </span>
                  <ChevronRight
                    size={16}
                    className="text-gray-500 group-hover:text-gray-700 
                               transition-colors duration-200"
                  />
                </div>
              </div>
            </div>
            <div className="flex flex-wrap gap-4 pt-10 justify-center w-full">
              {games.map((game) => (
                <GameCard
                  key={String(game.slug)}
                  game={game}
                  onClick={() => handleGameSelect(game.slug)}
                />
              ))}
            </div>
            {pagination.totalPages > 1 && (
              <Pagination
                currentPage={pagination.page}
                totalPages={pagination.totalPages}
                onPageChange={handlePageChange}
              />
            )}
          </div>
        ) : (
          <GameInfo  />
        )}
      </div>
    </Layout>
  );
}

function SearchBar({
  searchTerm,
  onSearchChange,
}: {
  searchTerm: string;
  onSearchChange: (term: string) => void;
}) {
  const handleSearchChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    onSearchChange(event.target.value);
  };

  return (
    <div className="flex items-center gap-2 w-full">
      <SearchIcon />
      <input
        type="text"
        value={searchTerm}
        onChange={handleSearchChange}
        placeholder="Search games..."
        className="w-full rounded-lg border border-base-border bg-base-background px-4 py-2 text-sm"
      />
    </div>
  );
}
