import { useCallback, useEffect, useState } from "react";
import GameCard from "./components/GameCard";
import Layout from "../../components/Layout";
import { ChevronRight, SearchIcon } from "lucide-react";
import GameInfo from "./components/GameInfo";
import { useGameboy } from "../../context/GameboyContext";
import Pagination from "./components/Pagination";
import { useOptions } from "../../context/OptionsContext";
import { debounce } from "../../utils/utils";
import { useAuth } from "../../context/AuthContext";

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
  releaseDate?: string;
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
  const { options, cycleLimitOptions } = useOptions();
  const [pagination, setPagination] = useState<{
    page: number;
    total: number;
    totalPages: number;
  }>({
    page: 1,
    total: 0,
    totalPages: 0,
  });
  const { fetchWithAuth } = useAuth();

  const loadGames = async (
    page = 1,
    search = "",
    limit = options.limitOptions
  ) => {
    try {
      const url = new URL("/api/games", window.location.origin);
      url.searchParams.set("page", page.toString());
      url.searchParams.set("search", search);
      url.searchParams.set("limit", limit.toString());

      const response = await fetchWithAuth(url.toString());
      console.log(response);
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

  const debouncedLoadGames = useCallback(
    debounce(() => {
      loadGames(pagination.page, searchTerm, options.limitOptions);
      setCurrentGame(null);
    }, 300), // to avoid spamming the server
    [options.limitOptions, searchTerm, pagination.page]
  );
  useEffect(() => {
    if (pagination.page > 0 && options.limitOptions > 0) {
      debouncedLoadGames();
    }
  }, [options.limitOptions, searchTerm, pagination.page, debouncedLoadGames]);

  const { currentGame, setCurrentGame } = useGameboy();

  const handlePageChange = (newPage: number) => {
    if (newPage > 0 && newPage <= pagination.totalPages) {
      setPagination((prevPagination) => ({
        ...prevPagination,
        page: newPage,
      }));
    }
  };

  const handleSearch = (term: string) => {
    setSearchTerm(term);
  };

  const handleLimitChange = () => {
    cycleLimitOptions();
  };

  const handleGameSelect = async (slug: string) => {
    try {
      const response = await fetchWithAuth(`/api/games/${slug}`);
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
  console.log(options.limitOptions);

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
                <div className="flex items-center text-center gap-1">
                  <span className="text-sm font-medium " title="Limit">
                    {options.limitOptions}
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
          <GameInfo />
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
