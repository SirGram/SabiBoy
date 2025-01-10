/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        // Base Colors
        base: {
          foreground: "#FFFFFF",
          background: "#120C15",
          border: "#262626",
        },

        // Primary Colors (for main actions, highlights)
        primary: {
          DEFAULT: "#D89B2B", // Blue
          foreground: "#FFFFFF",
          hover: "#C27126",
        },

        // Secondary Colors (for secondary actions)
        secondary: {
          DEFAULT: "#A5890D", // Indigo
          foreground: "#FFFFFF",
          hover: "#8F650A",
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
  plugins: [
    require("@tailwindcss/forms"),
    function ({ addUtilities }) {
      // Add custom clip-path utility for the hole
      addUtilities({
        ".clip-hole": {
          clipPath: "circle(50% at center)",
          WebkitClipPath: "circle(50% at center)", // for Webkit browsers like Safari
        },
      }, ['responsive', 'hover']); // Optionally add responsive and hover variants if needed
    },
  ],
};
