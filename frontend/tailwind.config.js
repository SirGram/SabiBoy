/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend:{
      colors: {
        base: {
          foreground: "#FFFFFF",
          background: "#08000f",
          border: "#2D2438",
        },
        primary: {
          DEFAULT: "#d22581",
          foreground: "#FFFFFF",
          hover: "#c11671",
        },
        secondary: {
          DEFAULT: "#7C3AED",
          foreground: "#FFFFFF",
          hover: "#6D28D9",
        },
        accent: {
          DEFAULT: "#14B8A6",
          foreground: "#FFFFFF",
          hover: "#0D9488",
        },
        destructive: {
          DEFAULT: "#bd2b2b",
          foreground: "#FFFFFF",
          hover: "#aa1717",
        },
        muted: {
          DEFAULT: "#4F4867",
          foreground: "#9CA3AF",
          hover: "#635D80",
        },
      }
    }
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
