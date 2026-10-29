import { test, expect } from "bun:test";
import { mkdir, mkdtemp, writeFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { parseProblem } from "./catalog";
import { createApp } from "./server";

test("parses LeetCode headers, completion comments and dotted filenames", () => {
  const p = parseProblem("643.maximum-average-subarray-i.rs", "// Completed 3/21/2026\n/*\n * [643] Maximum Average Subarray I\n */\nfn main() {}");
  expect(p).toMatchObject({ id: 643, title: "Maximum Average Subarray I", completed: "2026-03-21", slug: "maximum-average-subarray-i" });
  expect(parseProblem("7reverse-integer.rs", "fn main() {}")?.title).toBe("Reverse Integer");
  expect(parseProblem("scratch.rs", "fn main() {}")).toBeNull();
});

test("live file discovery and edits preserve durable annotations across restarts", async () => {
  const directory = await mkdtemp(join(tmpdir(), "rust-library-"));
  const database = join(directory, "test.sqlite");
  let app = createApp({ directory, database });
  const get = () => app.fetch(new Request("http://localhost/api/problems"));
  const put = (body: unknown, origin = "http://localhost") => app.fetch(new Request("http://localhost/api/annotation", { method: "PUT", headers: { "Content-Type": "application/json", origin }, body: JSON.stringify(body) }));
  try {
    expect(await (await get()).json()).toEqual([]);
    await writeFile(join(directory, "1two-sum.rs"), "// completed 4/13/2026\n// * [1] Two Sum\nfn main() {}");
    expect((await (await get()).json())[0].title).toBe("Two Sum");
    const annotation = { filename: "1two-sum.rs", notes: "<script>notes stay text</script>", favorite: true, review: true };
    expect((await put(annotation)).status).toBe(200);
    await writeFile(join(directory, "1two-sum.rs"), "// * [1] Two Sum\n// updated code");
    app.close();
    app = createApp({ directory, database });
    expect((await (await get()).json())[0]).toMatchObject({ ...annotation, code: "// * [1] Two Sum\n// updated code" });
    await writeFile(join(directory, "2add-two-numbers.rs"), "fn main() {}");
    expect(await (await get()).json()).toHaveLength(2);
    expect((await put(annotation, "https://untrusted.example")).status).toBe(403);
    expect((await put({ ...annotation, notes: 42 })).status).toBe(400);
    expect((await put({ ...annotation, filename: "../Cargo.toml" })).status).toBe(404);
    expect((await app.fetch(new Request("http://localhost/Cargo.toml"))).status).toBe(404);
    await rm(join(directory, "1two-sum.rs"));
    expect(await (await get()).json()).toHaveLength(1);
  } finally { app.close(); await rm(directory, { recursive: true, force: true }); }
});

test("serves built React assets and accepts only configured development origins", async () => {
  const directory = await mkdtemp(join(tmpdir(), "rust-library-static-"));
  const staticDirectory = join(directory, "dist");
  await mkdir(join(staticDirectory, "assets"), { recursive: true });
  await writeFile(join(staticDirectory, "index.html"), '<div id="root"></div>');
  await writeFile(join(staticDirectory, "assets/app-123.js"), 'console.log("React app")');
  const app = createApp({ directory, database: ":memory:", staticDirectory, allowedOrigins: ["http://localhost:3001"] });
  const get = (path: string) => app.fetch(new Request(`http://localhost${path}`));
  try {
    expect(await (await get("/")).text()).toContain('id="root"');
    expect((await get("/assets/app-123.js")).headers.get("cache-control")).toContain("immutable");
    expect((await get("/assets/%2e%2e%2f%2e%2e%2fpackage.json")).status).toBe(404);
    expect((await get("/assets/missing.js")).status).toBe(404);
    expect((await get("/api/missing")).status).toBe(404);
    const response = await app.fetch(new Request("http://localhost/api/annotation", { method: "PUT", headers: { origin: "http://localhost:3001", "Content-Type": "application/json" }, body: "{}" }));
    expect(response.status).toBe(400); // Reaches validation after origin check.
  } finally { app.close(); await rm(directory, { recursive: true, force: true }); }
});
