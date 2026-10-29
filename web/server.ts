import { Database } from "bun:sqlite";
import { mkdirSync } from "node:fs";
import { resolve, sep } from "node:path";
import { scanProblems } from "./catalog";

const root = resolve(import.meta.dir, "..");
export function createApp(options: { directory?: string; database?: string; allowedOrigins?: string[]; staticDirectory?: string } = {}) {
  if (!options.database) mkdirSync(resolve(root, "data"), { recursive: true });
  const db = new Database(options.database ?? resolve(root, "data/library.sqlite"));
  db.run("PRAGMA journal_mode = WAL");
  db.run(`CREATE TABLE IF NOT EXISTS annotations (
    filename TEXT PRIMARY KEY, notes TEXT NOT NULL DEFAULT '', favorite INTEGER NOT NULL DEFAULT 0,
    review INTEGER NOT NULL DEFAULT 0
  )`);
  const directory = options.directory ?? resolve(root, "src/bin");
  const json = (data: unknown, status = 200) => Response.json(data, { status, headers: { "Cache-Control": "no-store" } });
  async function fetch(req: Request) {
    const url = new URL(req.url);
    if (req.method !== "GET") {
      const origin = req.headers.get("origin");
      if (origin && origin !== url.origin && !options.allowedOrigins?.includes(origin)) return json({ error: "Origin rejected" }, 403);
      if (req.method !== "PUT" || url.pathname !== "/api/annotation") return json({ error: "Method not allowed" }, 405);
      if (!req.headers.get("content-type")?.startsWith("application/json")) return json({ error: "Expected JSON" }, 415);
      if (Number(req.headers.get("content-length")) > 100_000) return json({ error: "Notes too large" }, 413);
      let body;
      try { body = await req.json(); } catch { return json({ error: "Invalid JSON" }, 400); }
      if (!body || typeof body.filename !== "string" || typeof body.notes !== "string" || body.notes.length > 50_000 || typeof body.favorite !== "boolean" || typeof body.review !== "boolean") return json({ error: "Invalid annotation" }, 400);
      if (!(await scanProblems(directory)).some(p => p.filename === body.filename)) return json({ error: "Solution not found" }, 404);
      db.query("INSERT INTO annotations (filename, notes, favorite, review) VALUES (?, ?, ?, ?) ON CONFLICT(filename) DO UPDATE SET notes=excluded.notes, favorite=excluded.favorite, review=excluded.review")
        .run(body.filename, body.notes, Number(body.favorite), Number(body.review));
      return json({ saved: true });
    }
    if (url.pathname === "/api/problems") {
      const annotations = new Map((db.query("SELECT * FROM annotations").all() as { filename: string; notes: string; favorite: number; review: number }[]).map(a => [a.filename, a]));
      return json((await scanProblems(directory)).map(p => {
        const a = annotations.get(p.filename);
        return { ...p, notes: a?.notes ?? "", favorite: Boolean(a?.favorite), review: Boolean(a?.review) };
      }));
    }
    const staticDirectory = options.staticDirectory ?? resolve(root, "dist");
    let pathname: string;
    try { pathname = decodeURIComponent(url.pathname); } catch { return new Response("Invalid path", { status: 400 }); }
    if (pathname !== "/" && pathname !== "/index.html" && !pathname.startsWith("/assets/")) return new Response("Not found", { status: 404 });
    const assetPath = resolve(staticDirectory, pathname === "/" ? "index.html" : pathname.slice(1));
    if (!assetPath.startsWith(resolve(staticDirectory) + sep)) return new Response("Not found", { status: 404 });
    const file = Bun.file(assetPath);
    if (!(await file.exists())) return new Response(pathname === "/" ? "Frontend missing. Run bun run build first, or use bun run dev." : "Not found", { status: 404 });
    return new Response(file, { headers: { "Cache-Control": pathname.startsWith("/assets/") ? "public, max-age=31536000, immutable" : "no-cache", "X-Content-Type-Options": "nosniff" } });
  }
  return { fetch, close: () => db.close() };
}

if (import.meta.main) {
  const app = createApp();
  const server = Bun.serve({ hostname: "127.0.0.1", port: Number(process.env.PORT ?? 3000), fetch: app.fetch });
  console.log(`\n  Rust / archive → http://localhost:${server.port}\n  Reading src/bin · notes saved in data/library.sqlite\n`);
}
