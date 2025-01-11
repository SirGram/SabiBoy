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
          console.log(gamesWithImages)
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

  const handleSearch = (term: string) => {
    setSearchTerm(term);
    setPagination((prev) => ({ ...prev, page: 1 }));
  };

  const handleLimitChange = () => {
    cycleLimitOptions();
    setPagination((prev) => ({ ...prev, page: 1 }));
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
      <div className="flex w-full flex-col gap-6">
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
