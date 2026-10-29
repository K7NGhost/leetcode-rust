import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  root: "web/client",
  plugins: [react(), tailwindcss()],
  build: { outDir: "../../dist", emptyOutDir: true },
  server: { host: "127.0.0.1", port: Number(process.env.PORT ?? 3000), strictPort: true },
});
