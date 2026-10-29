import { createServer } from "vite";
import { createApp } from "./server";

const port = Number(process.env.PORT ?? 3000);
const app = createApp({ allowedOrigins: [`http://localhost:${port}`, `http://127.0.0.1:${port}`] });
const api = Bun.serve({ hostname: "127.0.0.1", port: 0, fetch: app.fetch });
let vite: Awaited<ReturnType<typeof createServer>> | undefined;
let stopping = false;
async function shutdown() {
  if (stopping) return;
  stopping = true;
  await vite?.close();
  await api.stop(true);
  app.close();
}
try {
  vite = await createServer({ server: { proxy: { "/api": { target: `http://127.0.0.1:${api.port}`, changeOrigin: false } } } });
  await vite.listen();
  console.log(`\n  Rust / archive → http://localhost:${port}`);
  console.log("  React + Tailwind + daisyUI · API + SQLite · src/bin auto-sync\n");
} catch (error) {
  await shutdown();
  throw error;
}
for (const signal of ["SIGINT", "SIGTERM"] as const) process.on(signal, () => { void shutdown().then(() => process.exit(0)); });
