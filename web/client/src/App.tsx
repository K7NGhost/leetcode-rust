import { useEffect, useMemo, useState } from "react";
import { ChevronRight, CodeXml } from "lucide-react";
import { Sidebar } from "./components/Sidebar";
import { Stats } from "./components/Stats";
import { Library } from "./components/Library";
import { SolutionDialog } from "./components/SolutionDialog";
import { useProblems } from "./hooks/useProblems";
import { viewTitles, type Problem, type View } from "./types";

export default function App() {
  const [active, setActive] = useState<Problem | null>(null);
  const { problems, loading, error, save } = useProblems(active !== null);
  const [view, setView] = useState<View>("all");
  const [topic, setTopic] = useState("");
  const [query, setQuery] = useState("");
  const [sort, setSort] = useState("recent");
  const [pending, setPending] = useState(new Set<string>());
  const [saveError, setSaveError] = useState("");
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "/" && !active && !(event.target instanceof HTMLElement && (event.target.isContentEditable || /INPUT|TEXTAREA|SELECT/.test(event.target.tagName)))) {
        event.preventDefault(); document.getElementById("solution-search")?.focus();
      }
    };
    document.addEventListener("keydown", onKeyDown);
    return () => document.removeEventListener("keydown", onKeyDown);
  }, [active]);
  useEffect(() => {
    function onHash() {
      // Only open links automatically; closing goes through the dialog's unsaved-change guard.
      if (!location.hash) return;
      try {
        const filename = decodeURIComponent(location.hash.slice(1));
        const problem = problems.find(p => p.filename === filename);
        if (problem) setActive(current => current?.filename === filename ? current : problem);
      } catch { /* Ignore malformed deep links. */ }
    }
    if (!active) onHash();
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  }, [problems, active]);
  const topics = useMemo(() => [...new Set(problems.map(p => p.topic))].sort(), [problems]);
  const filtered = useMemo(() => {
    const search = query.toLowerCase().trim();
    return problems.filter(p => (view === "all" || p[view]) && (!topic || p.topic === topic) && `${p.id} ${p.title} ${p.code} ${p.notes}`.toLowerCase().includes(search))
      .sort(sort === "number" ? (a, b) => a.id - b.id : sort === "title" ? (a, b) => a.title.localeCompare(b.title) : (a, b) => (b.completed ?? "").localeCompare(a.completed ?? "") || a.id - b.id);
  }, [problems, view, topic, query, sort]);
  function open(problem: Problem) {
    setActive(problem);
    history.replaceState(null, "", `#${encodeURIComponent(problem.filename)}`);
  }
  function close() { history.replaceState(null, "", location.pathname); setActive(null); }
  async function favorite(problem: Problem) {
    setPending(current => new Set(current).add(problem.filename));
    try { await save({ ...problem, favorite: !problem.favorite }); setSaveError(""); }
    catch (e) { setSaveError(e instanceof Error ? e.message : "Could not save favorite"); }
    finally { setPending(current => { const next = new Set(current); next.delete(problem.filename); return next; }); }
  }
  return <>
    <Sidebar problems={problems} view={view} topic={topic} onView={next => { setView(next); setTopic(""); }} onTopic={next => { setTopic(current => current === next ? "" : next); setView("all"); }} />
    <main className="px-5 lg:ml-60 lg:px-10 xl:px-14">
      <header className="flex h-16 items-center justify-between border-b border-base-300 text-[11px] text-base-content/50 lg:h-20"><span className="flex items-center gap-3">Workspace<ChevronRight size={12} className="text-base-content/25" />Solution library</span><span className="flex items-center gap-2 text-[9px] font-medium tracking-widest"><span className="status status-success size-1.5" />LOCAL ONLY</span></header>
      <section className="pt-9 pb-8 lg:pt-12"><p className="eyebrow">A little better, one problem at a time.</p><h1 className="mt-3 mb-3 text-3xl font-medium tracking-tight sm:text-4xl xl:text-[40px]">Your practice, collected<span className="text-primary/70">.</span></h1><p className="text-xs leading-relaxed text-base-content/50 sm:text-[13px]">A growing library of the problems you’ve worked through in Rust.</p></section>
      <Stats problems={problems} />
      <Library problems={filtered} total={problems.length} title={topic || viewTitles[view]} query={query} onQuery={setQuery} sort={sort} onSort={setSort} topic={topic} topics={topics} onTopic={next => { setTopic(next); setView("all"); }} onOpen={open} onFavorite={p => { void favorite(p); }} pending={pending} loading={loading} error={error || saveError} />
      <footer className="flex items-center justify-center gap-2 py-8 text-[10px] text-base-content/40"><CodeXml size={15} className="text-primary/40" />Keep showing up. The collection takes care of itself.</footer>
    </main>
    {active && <SolutionDialog key={active.filename} problem={active} onClose={close} onSave={save} />}
  </>;
}
