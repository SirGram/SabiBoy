import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
  assetsInclude: ["**/*.gb"],
  server: {
    host: "0.0.0.0",
    port: 5173,
    proxy: {
      "/api": {
        target: process.env.VITE_API_URL || "http://localhost:3000",
        changeOrigin: true,
        configure: (proxy, _options) => {
          proxy.on("proxyReq", (proxyReq, req: any) => {
            if (req.body instanceof ArrayBuffer) {
              proxyReq.setHeader("Content-Length", req.body.byteLength);
              proxyReq.write(Buffer.from(req.body));
            }
          });
        },
      },
    },
  },
});
