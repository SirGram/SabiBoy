import { TPaginatedResponse } from "../pages/Library/Library";
import { TGame } from "../types";
import { SortType } from "../context/OptionsContext";

export const loadGameImage = async (
  game: TGame,
  fetchWithAuth: any
): Promise<TGame> => {
  if (!game.coverPath) return game;

  try {
    const response = await fetchWithAuth(game.coverPath);
    if (!response.ok) return game;

    const blob = await response.blob();
    const coverURL = URL.createObjectURL(blob);
    return { ...game, coverURL };
  } catch (error) {
    console.error(`Failed to load image for ${game.name}:`, error);
    return game;
  }
};

export const loadGames = async (
  fetchWithAuth: any,
  page = 1,
  search = "",
  limit = 10,
  sortType: SortType
): Promise<{ gamesWithImages: TGame[]; pagination: { page: number; total: number; totalPages: number } } | undefined> => {
  try {
    const url = new URL("/api/games", window.location.origin);
    url.searchParams.set("page", page.toString());
    url.searchParams.set("search", search);
    url.searchParams.set("limit", limit.toString());
    
    // Convert SortType to API parameters
    let sortBy: string;
    switch (sortType) {
      case SortType.DATE_NEW:
        sortBy = "recent_desc";
        break;
      case SortType.DATE_OLD:
        sortBy = "recent_asc";
        break;
      case SortType.NAME_ASC:
        sortBy = "name_asc";
        break;
      case SortType.NAME_DESC:
        sortBy = "name_desc";
        break;
      default:
        sortBy = "recent_desc";
    }
    url.searchParams.set("sortBy", sortBy);

    const response = await fetchWithAuth(url.toString());
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data: TPaginatedResponse = await response.json();
    const gamesWithImages = await Promise.all(
      data.games.map((game) => loadGameImage(game, fetchWithAuth))
    );

    const pagination = {
      page: data.page,
      total: data.total,
      totalPages: data.totalPages,
    };

    return { gamesWithImages, pagination };
  } catch (error) {
    console.error("Failed to load games:", error);
    return undefined;
  }
};