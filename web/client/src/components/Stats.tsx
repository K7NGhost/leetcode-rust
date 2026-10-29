import type { Problem } from "../types";

export function Stats({ problems }: { problems: Problem[] }) {
  const stats = [
    { title: "Total solutions", value: problems.length, description: "in your repository" },
    { title: "Completed", value: problems.filter(p => p.completed).length, description: "with a completion date" },
    { title: "Topics explored", value: new Set(problems.map(p => p.topic)).size, description: "inferred from your code" },
    { title: "To revisit", value: problems.filter(p => p.review).length, description: "a little more practice" },
  ];
  return <section aria-label="Library statistics" className="stats stats-vertical grid w-full grid-cols-2 rounded-xl border border-base-300 bg-base-100 shadow-xs lg:stats-horizontal lg:grid-cols-4">{stats.map(s => <div className="stat px-5 py-6 sm:px-7" key={s.title}><div className="stat-title text-xs">{s.title}</div><div className="stat-value my-2 text-4xl font-medium tracking-tight">{s.value}</div><div className="stat-desc text-[10px]">{s.description}</div></div>)}</section>;
}
