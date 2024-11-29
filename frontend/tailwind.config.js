/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        // Base Colors
        base: {
          DEFAULT: "#000000",
          foreground: "#FFFFFF",
          background: "#000000",
          border: "#262626",
        },

        // Primary Colors (for main actions, highlights)
        primary: {
          DEFAULT: "#3B82F6", // Blue
          foreground: "#FFFFFF",
          hover: "#2563EB",
        },

        // Secondary Colors (for secondary actions)
        secondary: {
          DEFAULT: "#6366F1", // Indigo
          foreground: "#FFFFFF",
          hover: "#4F46E5",
        },

        // Accent Colors (for additional highlights)
        accent: {
          DEFAULT: "#10B981", // Emerald
          foreground: "#FFFFFF",
          hover: "#059669",
        },

        // Destructive Colors (for delete, remove actions)
        destructive: {
          DEFAULT: "#EF4444", // Red
          foreground: "#FFFFFF",
          hover: "#DC2626",
        },

        // Muted Colors (for less important elements)
        muted: {
          DEFAULT: "#4B5563", // Gray
          foreground: "#9CA3AF",
          hover: "#6B7280",
        },
      },
    },
  },
  plugins: [],
};
