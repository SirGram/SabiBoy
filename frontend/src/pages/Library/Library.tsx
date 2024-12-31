import { useCallback, useEffect, useState } from "react";
import GameCard from "./components/GameCard";
import Layout from "../../components/Layout/MainLayout";
import { ChevronRight, SearchIcon, XIcon } from "lucide-react";
import GameInfo from "./components/GameInfo";
import { useGameboy } from "../../context/GameboyContext";
import Pagination from "./components/Pagination";
import { SortType, useOptions } from "../../context/OptionsContext";
import { debounce } from "../../utils/utils";
import { useAuth } from "../../context/AuthContext";
import Loading from "../../components/Loading";
import { TGame, TGameDetails } from "../../types";
import { loadGames } from "../../api/api";

export type TPaginatedResponse = {
  games: TGame[];
  total: number;
  page: number;
  totalPages: number;
};

export default function Library() {
  const [games, setGames] = useState<TGame[]>([]);
  const [searchTerm, setSearchTerm] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const { options, cycleLimitOptions, updateSortType } = useOptions();
  const [pagination, setPagination] = useState<{
    page: number;
    total: number;
    totalPages: number;
  }>({
    page: 1,
    total: 0,
    totalPages: 0,
  });
  const { fetchWithAuth, user } = useAuth();
  const debouncedLoadGames = useCallback(
    debounce(async () => {
      setIsLoading(true);
      try {
        const result = await loadGames(
          fetchWithAuth,
          pagination.page,
          searchTerm,
          options.limitOptions,
          options.sortType
        );

        if (result) {
          const { gamesWithImages, pagination } = result;
          setGames(gamesWithImages);
          setPagination(pagination);
        } else {
          console.error("Failed to load games");
        }

        setCurrentGame(null);
      } catch (error) {
        console.error("Error while loading games:", error);
      } finally {
        setIsLoading(false); // Stop loading
      }
    }, 300),
    [
      options.limitOptions,
      searchTerm,
      pagination.page,
      fetchWithAuth,
      options.sortType,
    ]
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
    setPagination((prev) => ({ ...prev, page: 1 }));
  };

  const handleLimitChange = () => {
    cycleLimitOptions();
  };

  const handleGameSelect = async (slug: string) => {
    if (!user) return;
    try {
      const response = await fetchWithAuth(`/api/games/${slug}`);
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
          <div className="flex flex-col w-full  ">
            <div className="flex  flex-col md:flex-row gap-2 items-center w-full  justify-between">
              <SearchBar
                searchTerm={searchTerm}
                onSearchChange={handleSearch}
                handleClearSearch={() => setSearchTerm("")}
              />
              <div className="flex  gap-4 items-center w-full justify-center">
                <div
                  className="bg-base-background h-full flex items-center border border-base-border rounded-md px-2 py-1 
                           transition-colors duration-200 cursor-pointer group"
                  onClick={handleLimitChange}
                >
                  <div className="flex items-center text-center gap-1  ">
                    <span className="text-sm font-medium">
                      {options.limitOptions}
                    </span>
                    <ChevronRight
                      size={16}
                      className="text-gray-500 group-hover:text-gray-700 transition-colors duration-200"
                    />
                  </div>
                </div>

                <select
                  id="sort-type"
                  value={options.sortType}
                  onChange={(e) => updateSortType(e.target.value as SortType)}
                  className="border rounded px-2 py-1"
                >
                  {Object.values(SortType).map((sortType) => (
                    <option key={sortType} value={sortType}>
                      {sortType}
                    </option>
                  ))}
                </select>
              </div>
            </div>
            <div className="flex justify-center w-full pt-10 ">
              {isLoading ? (
                <Loading />
              ) : (
                <div className="flex flex-wrap gap-4 justify-center w-full mb-10">
                  {games.map((game) => (
                    <GameCard
                      key={String(game.slug)}
                      game={game}
                      onClick={() => handleGameSelect(game.slug)}
                    />
                  ))}
                </div>
              )}
            </div>
            {pagination.totalPages > 1 && (
              <div className="   left-0 right-0 flex justify-center">
                <Pagination
                  currentPage={pagination.page}
                  totalPages={pagination.totalPages}
                  onPageChange={handlePageChange}
                />
              </div>
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
  handleClearSearch,
}: {
  searchTerm: string;
  onSearchChange: (term: string) => void;
  handleClearSearch: () => void;
}) {
  const handleSearchChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    onSearchChange(event.target.value);
  };

  return (
    <div className="flex flex-1 items-center gap-1 px-2 w-full rounded-lg border border-base-border focus-within:border-primary bg-base-background text-sm">
      <SearchIcon />
      <input
        type="text"
        value={searchTerm}
        onChange={handleSearchChange}
        placeholder="Search games..."
        className=" bg-transparent outline-none border-none focus:outline-none focus:ring-0"
      />
      {searchTerm !== "" && (
        <button onClick={handleClearSearch}>
          <XIcon />
        </button>
      )}
    </div>
  );
}
