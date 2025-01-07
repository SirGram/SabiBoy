import { TPaginatedResponse } from "../pages/Library/Library";
import { TGame } from "../types";
import { SortType } from "../context/OptionsContext";
import api from "./client";

export const loadGameImage = async (game: TGame): Promise<TGame> => {
  if (!game.coverPath) return game;

  try {
    const response = await api.get(game.coverPath, {
      responseType: 'blob'
    });
    
    const coverURL = URL.createObjectURL(response.data);
    return { ...game, coverURL };
  } catch (error) {
    console.error(`Failed to load image for ${game.name}:`, error);
    return game;
  }
};


export const loadGames = async (
  page = 1,
  search = "",
  limit = 10,
  sortType: SortType
): Promise<{ gamesWithImages: TGame[]; pagination: { page: number; total: number; totalPages: number } } | undefined> => {
  try {
    const sortBy = {
      [SortType.DATE_NEW]: "recent_desc",
      [SortType.DATE_OLD]: "recent_asc",
      [SortType.NAME_ASC]: "name_asc",
      [SortType.NAME_DESC]: "name_desc"
    }[sortType] || "recent_desc";
    
    const { data: responseData, status } = await api.get<TPaginatedResponse>('/api/games', {
      params: {
        page,
        search,
        limit,
        sortBy
      }
    });
    
    if (status !== 200) {
      throw new Error(`HTTP error! status: ${status}`);
    }

    const gamesWithImages = await Promise.all(
      responseData.games.map(loadGameImage)
    );

    return {
      gamesWithImages,
      pagination: {
        page: responseData.page,
        total: responseData.total,
        totalPages: responseData.totalPages
      }
    };

  } catch (error) {
    console.error("Failed to load games:", error);
    return undefined;
  }
};