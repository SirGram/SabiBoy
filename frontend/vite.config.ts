import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
  assetsInclude: ["**/*.gb"],
  server:{
    proxy:{
      "/api":{
        target:"http://localhost:3000",
        changeOrigin:true,
      }
    }
  }
});
