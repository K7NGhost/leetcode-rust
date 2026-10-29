import { BookOpen, CodeXml, FolderCode, Layers3, RotateCcw, Star } from "lucide-react";
import type { Problem, View } from "../types";
import { viewTitles } from "../types";

type Props = { problems: Problem[]; view: View; topic: string; onView: (view: View) => void; onTopic: (topic: string) => void };
export function Sidebar({ problems, view, topic, onView, onTopic }: Props) {
  const topics = [...new Set(problems.map(p => p.topic))].sort();
  const icons = { all: Layers3, favorite: Star, review: RotateCcw };
  return <aside className="flex flex-col border-b border-base-300 bg-base-200 px-5 py-5 lg:fixed lg:inset-y-0 lg:w-60 lg:border-r lg:px-5 lg:py-8">
    <a href="/" className="flex items-center gap-2 text-xl font-semibold tracking-tight"><span className="grid size-9 place-items-center rounded-lg bg-primary text-primary-content"><CodeXml size={21} /></span>rust<span className="font-normal text-base-content/45">/ archive</span></a>
    <p className="eyebrow mt-12 mb-4 hidden px-3 lg:block">Personal workspace</p>
    <nav aria-label="Library views" className="mt-5 lg:mt-0"><ul className="menu menu-horizontal w-full flex-nowrap gap-1 p-0 lg:menu-vertical lg:gap-2">
      {(Object.keys(viewTitles) as View[]).map(key => { const Icon = icons[key]; return <li key={key}><button onClick={() => onView(key)} className={`text-xs lg:py-3 ${view === key && !topic ? 'menu-active font-medium' : 'text-base-content/60'}`} aria-current={view === key && !topic ? "page" : undefined}><Icon size={16} />{viewTitles[key]}{key === 'all' && <span className="badge badge-sm border-0 bg-primary/10 text-primary max-lg:hidden">{problems.length}</span>}</button></li>; })}
    </ul></nav>
    <div className="hidden lg:block"><p className="eyebrow mt-10 mb-4 px-3">Explore by topic</p><ul className="menu w-full gap-1 p-0">{topics.map(t => <li key={t}><button className={`text-xs text-base-content/60 ${t === topic ? 'menu-active' : ''}`} onClick={() => onTopic(t)} aria-pressed={t === topic}><span className="size-1.5 rounded-sm bg-primary/40" />{t}<span className="ml-auto text-[10px] text-base-content/40">{problems.filter(p => p.topic === t).length}</span></button></li>)}</ul></div>
    <div className="mt-auto hidden px-3 pt-10 text-[11px] text-base-content/60 lg:block"><div className="flex items-center gap-2"><span className="status status-success size-1.5" />Connected to your repo</div><div className="mt-3 flex items-center gap-2 text-[10px] text-base-content/40"><FolderCode size={13} /><code>src/bin/*.rs</code></div><p className="mt-2 text-[10px] text-base-content/40">New solutions sync automatically.</p><BookOpen size={16} className="mt-5 text-primary/35" /></div>
  </aside>;
}
