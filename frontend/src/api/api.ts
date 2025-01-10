import { TPaginatedResponse } from "../pages/Library/Library";
import { TGame, TGameDetails } from "../types";
import { SortType } from "../context/OptionsContext";
import api from "./client";

export const loadGameImage = async <T extends TGame | TGameDetails>(
  game: T
): Promise<T> => {
  if (!game.coverPath) return game;

  try {
    const response = await api.get(game.coverPath, {
      responseType: "blob",
    });

    if (!(response.data instanceof Blob)) {
      console.error(`Invalid response data type for ${game.name}`);
      return game;
    }

    const coverURL = URL.createObjectURL(response.data);
    if (!coverURL) {
      console.error(`Failed to create blob URL for ${game.name}`);
      return game;
    }

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
): Promise<
  | {
      gamesWithImages: TGame[];
      pagination: { page: number; total: number; totalPages: number };
    }
  | undefined
> => {
  try {
    const sortBy =
      {
        [SortType.DATE_NEW]: "recent_desc",
        [SortType.DATE_OLD]: "recent_asc",
        [SortType.NAME_ASC]: "name_asc",
        [SortType.NAME_DESC]: "name_desc",
      }[sortType] || "recent_desc";

    const { data: responseData, status } = await api.get<TPaginatedResponse>(
      "/api/games",
      {
        params: {
          page,
          search,
          limit,
          sortBy,
        },
      }
    );

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
        totalPages: responseData.totalPages,
      },
    };
  } catch (error) {
    console.error("Failed to load games:", error);
    return undefined;
  }
};

export const loadGameInfo = async (
  gameslug: string
): Promise<TGameDetails | undefined> => {
  try {
    const { data: gameDetails, status } = await api.get<TGameDetails>(
      `/api/games/${gameslug}`
    );

    if (status !== 200 || !gameDetails) {
      throw new Error(`Failed to load game details for ${gameslug}`);
    }

    if (gameDetails.coverPath) {
      try {
        const response = await api.get(gameDetails.coverPath, {
          responseType: "blob",
        });

        if (response.status === 200 && response.data instanceof Blob) {
          // Create a new blob with explicit MIME type
          const blob = new Blob([response.data], {
            type: response.data.type || "image/webp",
          });
          const coverURL = URL.createObjectURL(blob);

          return {
            ...gameDetails,
            coverURL,
          };
        }
      } catch (error) {
        console.error(`Failed to load cover image for ${gameslug}:`, error);
      }
    }

    return gameDetails;
  } catch (error) {
    console.error("Failed to load game details:", error);
    return undefined;
  }
};
