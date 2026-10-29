# LeetCode Rust · local solution library

Rust solutions live in `src/bin`. A React + TypeScript website turns them into a searchable notebook, styled with Tailwind CSS 4 and daisyUI 5. Vite provides development hot reload; a Bun API handles solution imports and SQLite notes.

```sh
bun install
bun run dev
```

Open **http://localhost:3000**. Bun 1.3+ is recommended. One command starts both Vite and the Bun API, with API requests proxied through Vite. To use another port in PowerShell: `$env:PORT=3001; bun run dev`.

- Search titles, problem numbers, source code, and saved notes.
- Open a solution to read highlighted Rust code, copy it, or visit the problem on LeetCode.
- Save notes, favorite problems, and mark problems to revisit. These persist in `data/library.sqlite` across browser and server restarts.
- Filter by inferred topic and sort by completion date, number, or title.
- Add or edit `.rs` files in `src/bin` as usual. The library rescans on each API request, and the browser refreshes the list every three seconds while visible. Reopen an already open solution to load code changes.

Use numbered filenames like `42trapping-rain-water.rs` or `42.trapping-rain-water.rs`. Existing LeetCode headers (`* [42] Trapping Rain Water`) provide the display title. An optional `// Completed 10/26/2026` comment provides the completion date. Files without a completion comment show **Not recorded**; the site does not assume an imported solution was accepted. Topics are heuristic labels based on the source code.

The website displays your actual source files. Problem descriptions and interactive Rust execution are not imported; each solution links to its LeetCode problem. Your existing Cargo workflow stays available.

The server binds to local loopback only. Rust source files stay in Git; personal annotations are stored separately in the ignored `data/` folder. Back up that folder to preserve notes. Renaming a solution file gives it a new annotation identity; deleting a file hides it from the library without erasing its saved notes.

```sh
bun test             # import, live sync, persistence, and API checks
bun run typecheck    # TypeScript checks for frontend and backend
bun run build        # typecheck and build the React app into dist/
bun run start        # serve the built app and API with Bun (build first)
```

Frontend: `web/client/src/App.tsx`, reusable components in `components/`, and data synchronization in `hooks/useProblems.ts`. Tailwind and the custom daisyUI `archive` theme are configured in `styles.css`. Prism highlights Rust code; Lucide provides icons. All assets are built locally, with no CDN requirements.

Backend: `web/server.ts` serves the API and production assets. `web/catalog.ts` imports Rust solutions. `web/dev.ts` starts the development API and Vite together and shuts them down together. Restart `bun run dev` after editing backend files; frontend changes hot reload automatically.
