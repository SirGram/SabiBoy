import { useCallback, useEffect, useState } from "react";
import GameCard from "./components/GameCard";
import Layout from "../../components/Layout/MainLayout";
import { ChevronRight, PlusCircle, SearchIcon, XIcon } from "lucide-react";
import Pagination from "./components/Pagination";
import { SortType, useOptions } from "../../context/OptionsContext";
import { debounce } from "../../utils/utils";
import Loading from "../../components/Loading";
import { TGame } from "../../types";
import { loadGames } from "../../api/api";
import { useModal } from "../../context/ModalContext";
import { useNavigate } from "react-router-dom";

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
  const debouncedLoadGames = useCallback(
    debounce(async () => {
      setIsLoading(true);
      try {
        const result = await loadGames(
          pagination.page,
          searchTerm,
          options.limitOptions,
          options.sortType
        );

        if (result) {
          const { gamesWithImages, pagination } = result;
          setGames(gamesWithImages);
          console.log(gamesWithImages);
          setPagination(pagination);
        } else {
          console.error("Failed to load games");
        }
      } catch (error) {
        console.error("Error while loading games:", error);
      } finally {
        setIsLoading(false);
      }
    }, 300),
    [options.limitOptions, searchTerm, pagination.page, options.sortType]
  );

  useEffect(() => {
    if (pagination.page > 0 && options.limitOptions > 0) {
      debouncedLoadGames();
    }
  }, [options.limitOptions, searchTerm, pagination.page, debouncedLoadGames]);

  const handlePageChange = (newPage: number) => {
    if (newPage > 0 && newPage <= pagination.totalPages) {
      setPagination((prevPagination) => ({
        ...prevPagination,
        page: newPage,
      }));
    }
  };

  const navigate = useNavigate();
  const handleGameSelect = async (slug: string) => {
    navigate(`/library/${slug}`);
  };
  const { updateShowUploadModal } = useModal();
  const handleAddNewGame = () => {
    updateShowUploadModal(true);
  };
  return (
    <Layout>
      <div className="flex flex-col gap-8 px-4 py-2 max-w-7xl mx-auto w-full">
        <div className="flex flex-col gap-6">
          <div className="flex flex-col md:flex-row gap-4 items-stretch md:items-center justify-between max-w-xl self-center w-full">
            <SearchBar
              searchTerm={searchTerm}
              onSearchChange={(term) => {
                setSearchTerm(term);
                setPagination((prev) => ({ ...prev, page: 1 }));
              }}
              handleClearSearch={() => setSearchTerm("")}
            />

            <div className="flex gap-3 items-center w-full justify-center">
              <button
                onClick={() => {
                  cycleLimitOptions();
                  setPagination((prev) => ({ ...prev, page: 1 }));
                }}
                className="flex items-center gap-2 px-3 h-10 rounded-md border border-base-border hover:border-primary transition-colors bg-transparent"
              >
                <span className="text-sm font-medium text-muted">
                  {options.limitOptions}
                </span>
                <ChevronRight size={16} className="text-base-foreground/50" />
              </button>

              <div className="relative">
                <select
                  value={options.sortType}
                  onChange={(e) => updateSortType(e.target.value as SortType)}
                  className="appearance-none text-muted px-3 pr-8 h-10 rounded-md border border-base-border bg-transparent hover:border-primary transition-colors outline-none cursor-pointer"
                  aria-label="Sort by"
                >
                  {Object.values(SortType).map((type) => (
                    <option
                      key={type}
                      value={type}
                      className="bg-base-background"
                    >
                      {type}
                    </option>
                  ))}
                </select>
                
              </div>
            </div>
          </div>
          <div className="flex justify-center w-full  ">
            {isLoading ? (
              <Loading />
            ) : (
              <div className="flex flex-wrap gap-4 justify-center w-full ">
                {games.length > 0 ? (
                  games.map((game) => (
                    <GameCard
                      key={String(game.slug)}
                      game={game}
                      onClick={() => handleGameSelect(game.slug)}
                    />
                  ))
                ) : (
                  <p className="text-muted">No results in the library</p>
                )}
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
        {/* Games */}
        <div className="flex flex-col items-center justify-center">
          <div className="max-w-sm w-full  bg-base-border rounded-lg h-0.5 mb-6"></div>
          <button
            className="w-40 h-auto py-2 flex items-center flex-col justify-center self-center  text-base-foreground/20 bg-base-background/20 border-base-border border-dashed border-2 rounded-lg hover:text-base-foreground transition-all"
            onClick={handleAddNewGame}
          >
            <PlusCircle size={40} />
            <span>Add a new game</span>
          </button>
        </div>
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
  return (
    <div className="relative flex-1">
      <div className="flex items-center h-10 gap-2 px-4 w-full rounded-md border border-base-border focus-within:border-primary bg-transparent">
        <SearchIcon className="w-4 h-4 text-base-foreground/50" />
        <input
          type="text"
          value={searchTerm}
          onChange={(e) => onSearchChange(e.target.value)}
          placeholder="Search games..."
          className="flex-1 h-full bg-transparent outline-none border-none focus:outline-none focus:ring-0 placeholder:text-base-foreground/30"
        />
        {searchTerm && (
          <button
            onClick={handleClearSearch}
            className="text-base-foreground/50 hover:text-base-foreground transition-colors"
          >
            <XIcon className="w-4 h-4" />
          </button>
        )}
      </div>
    </div>
  );
}
