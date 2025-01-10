import axios, { isAxiosError } from "axios";
import React, { useState, ChangeEvent, FormEvent } from "react";
import api from "../../../api/client";

interface GameFormData {
  name: string;
  slug: string;
  language: string;
  description: string;
  originalTitle: string;
  rating: string;
  releaseDate: string;
  developers: string[];
  genres: string[];
}

interface FolderErrors {
  rom?: string;
}

interface FormErrors {
  name?: string;
  slug?: string;
  language?: string;
  rating?: string;
  folder?: FolderErrors;
}

interface CreateGamePayload {
  name: string;
  slug: string;
  language: string;
  description?: string;
  originalTitle?: string;
  rating?: number;
  releaseDate?: string;
  developers?: string[];
  genres?: string[];
}

const UploadNewGame: React.FC = () => {
  const [formData, setFormData] = useState<GameFormData>({
    name: "",
    slug: "",
    language: "",
    description: "",
    originalTitle: "",
    rating: "",
    releaseDate: "",
    developers: [],
    genres: [],
  });
  const [folder, setFolder] = useState<FileList | null>(null);
  const [errors, setErrors] = useState<FormErrors>({});

  const validateFolderStructure = (
    items: FileList
  ): { isValid: boolean; errors: FolderErrors } => {
    let hasRom = false;

    for (const item of Array.from(items)) {
      if (item.name.endsWith(".gb")) {
        hasRom = true;
        break;
      }
    }

    return {
      isValid: hasRom,
      errors: {
        rom: !hasRom ? "Missing .gb ROM file" : undefined,
      },
    };
  };

  const handleFolderUpload = (e: ChangeEvent<HTMLInputElement>) => {
    const folderInput = e.target;
    if (folderInput.files && folderInput.files.length > 0) {
      const validation = validateFolderStructure(folderInput.files);
      if (!validation.isValid) {
        setErrors((prev) => ({ ...prev, folder: validation.errors }));
        return;
      }
      setFolder(folderInput.files);
      setErrors((prev) => ({ ...prev, folder: undefined }));
    }
  };

  const handleInputChange = (
    e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleArrayInput = (
    e: ChangeEvent<HTMLInputElement>,
    field: keyof Pick<GameFormData, "developers" | "genres">
  ) => {
    const values = e.target.value
      .split(",")
      .map((item) => item.trim())
      .filter(Boolean);
    setFormData((prev) => ({
      ...prev,
      [field]: values,
    }));
  };

  const createGameWithFiles = async (
    gameData: CreateGamePayload,
    files: FileList
  ) => {
    const formData = new FormData();

    // Add required fields
    formData.append("name", gameData.name);
    formData.append("slug", gameData.slug);
    formData.append("language", gameData.language);

    // Add optional fields only if they have values
    if (gameData.description) {
      formData.append("description", gameData.description);
    }

    if (gameData.originalTitle) {
      formData.append("originalTitle", gameData.originalTitle);
    }

    if (gameData.rating !== undefined && gameData.rating !== null) {
      formData.append("rating", gameData.rating.toString());
    }

    if (gameData.releaseDate) {
      formData.append(
        "releaseDate",
        new Date(gameData.releaseDate).toISOString()
      );
    }

    // Handle arrays
    if (gameData.developers?.length) {
      formData.append("developers", JSON.stringify(gameData.developers));
    } else {
      formData.append("developers", JSON.stringify([]));
    }

    if (gameData.genres?.length) {
      formData.append("genres", JSON.stringify(gameData.genres));
    } else {
      formData.append("genres", JSON.stringify([]));
    }

    // Add files
    Array.from(files).forEach((file) => {
      formData.append(
        "files",
        file,
        `${gameData.slug}/${file.webkitRelativePath}`
      );
    });

    try {
      const response = await api.post("/api/games/upload", formData, {
        headers: {
          "Content-Type": "multipart/form-data",
        },
      });
      return response.data;
    } catch (error) {
      if (axios.isAxiosError(error)) {
        const message =
          error.response?.data?.message || "Failed to upload game";
        throw new Error(message);
      }
      throw error;
    }
  };
  const [isLoading, setIsLoading] = useState(false);
  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();

    const newErrors: FormErrors = {};
    if (!formData.name) newErrors.name = "Name is required";
    if (!formData.slug) newErrors.slug = "Slug is required";
    if (!formData.language) newErrors.language = "Language is required";
    if (
      formData.rating &&
      (Number(formData.rating) < 0 || Number(formData.rating) > 100)
    ) {
      newErrors.rating = "Rating must be between 0 and 100";
    }

    if (Object.keys(newErrors).length > 0) {
      setErrors(newErrors);
      return;
    }

    if (!folder) {
      setErrors({ folder: { rom: "Game folder is required" } });
      return;
    }

    try {
      const gameData: CreateGamePayload = {
        name: formData.name,
        slug: formData.slug,
        language: formData.language,
      };

      // Add optional fields only if they have values
      if (formData.description) gameData.description = formData.description;
      if (formData.originalTitle)
        gameData.originalTitle = formData.originalTitle;
      if (formData.rating) gameData.rating = Number(formData.rating);
      if (formData.releaseDate) gameData.releaseDate = formData.releaseDate;
      if (formData.developers?.length)
        gameData.developers = formData.developers;
      if (formData.genres?.length) gameData.genres = formData.genres;

      await createGameWithFiles(gameData, folder);

      // Show success message
      console.log("Game uploaded successfully");
    } catch (error) {
      console.error("Error uploading game:", error);
      setErrors({
        folder: {
          rom:
            error instanceof Error
              ? error.message
              : "Failed to upload game. Please try again.",
        },
      });
      console.log("Failed to upload game");
    } finally {
      setIsLoading(false);
    }
  };
  return (
    <div className=" w-full flex-col flex gap-6 p-4 justify-center items-center">
      <h1 className="text-2xl font-bold">Upload New Game</h1>
      <div className="self-start ">
        <h2 className="text-lg font-bold mb-2">Folder Structure:</h2>
        <pre className=" p-3 rounded text-sm">
          {`your-game-folder/
  ├── screenshots/ (optional)
  │   ├── screenshot0.jpg
  │   ├── screenshot1.jpg
  │   └── ...
  ├── cover/ (optional)
  │   └── cover.jpg
  └── rom.gb (required)`}
        </pre>
      </div>

      <form
        onSubmit={handleSubmit}
        className="space-y-4 py-4 px-2 flex flex-col self-start"
      >
        <div>
          <label className="block mb-2">
            Game Folder
            <input
              type="file"
              webkitdirectory=""
              directory=""
              className="mt-1 block w-full"
              onChange={handleFolderUpload}
            />
          </label>
          {errors.folder?.rom && (
            <div className="text-red-500 text-sm mt-1">{errors.folder.rom}</div>
          )}
        </div>

        <div>
          <label className="block mb-2">
            Name *
            <input
              type="text"
              name="name"
              value={formData.name}
              onChange={handleInputChange}
              className="mt-1 block w-full border rounded p-2 bg-base-background"
              required
            />
          </label>
          {errors.name && (
            <div className="text-red-500 text-sm">{errors.name}</div>
          )}
        </div>

        <div>
          <label className="block mb-2">
            Slug *
            <input
              type="text"
              name="slug"
              value={formData.slug}
              onChange={handleInputChange}
              className="mt-1 block w-full border rounded p-2 bg-base-background"
              required
            />
          </label>
          {errors.slug && (
            <div className="text-red-500 text-sm">{errors.slug}</div>
          )}
        </div>

        <div>
          <label className="block mb-2">
            Language/Region *
            <input
              type="text"
              name="language"
              value={formData.language}
              onChange={handleInputChange}
              className="mt-1 block w-full border rounded p-2 bg-base-background"
              required
            />
          </label>
          {errors.language && (
            <div className="text-red-500 text-sm">{errors.language}</div>
          )}
        </div>

        <div>
          <label className="block mb-2">
            Description
            <textarea
              name="description"
              value={formData.description}
              onChange={handleInputChange}
              className="mt-1 block w-full border rounded p-2 bg-base-background"
              rows={4}
            />
          </label>
        </div>

        <div>
          <label className="block mb-2">
            Original Title
            <input
              type="text"
              name="originalTitle"
              value={formData.originalTitle}
              onChange={handleInputChange}
              className="mt-1 block w-full border rounded p-2 bg-base-background"
            />
          </label>
        </div>

        <div>
          <label className="block mb-2">
            Rating (0-100)
            <input
              type="number"
              name="rating"
              value={formData.rating}
              onChange={handleInputChange}
              min="0"
              max="100"
              className="mt-1 block w-full border rounded p-2 bg-base-background"
            />
          </label>
          {errors.rating && (
            <div className="text-red-500 text-sm">{errors.rating}</div>
          )}
        </div>

        <div>
          <label className="block mb-2">
            Release Date
            <input
              type="date"
              name="releaseDate"
              value={formData.releaseDate}
              onChange={handleInputChange}
              className="mt-1 block w-full border rounded p-2 bg-base-background"
            />
          </label>
        </div>

        <div>
          <label className="block mb-2">
            Developers (comma-separated)
            <input
              type="text"
              value={formData.developers.join(", ")}
              onChange={(e) => handleArrayInput(e, "developers")}
              className="mt-1 block w-full border rounded p-2 bg-base-background"
            />
          </label>
        </div>

        <div>
          <label className="block mb-2">
            Genres (comma-separated)
            <input
              type="text"
              value={formData.genres.join(", ")}
              onChange={(e) => handleArrayInput(e, "genres")}
              className="mt-1 block w-full border rounded p-2 bg-base-background"
            />
          </label>
        </div>

        <button
          type="submit"
          className="bg-primary  px-4 py-2 rounded hover:bg-primary self-center"
        >
          Upload Game
        </button>
      </form>
    </div>
  );
};

export default UploadNewGame;
