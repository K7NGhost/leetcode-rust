import { useEffect, useMemo, useRef, useState } from "react";
import { Check, Copy, ExternalLink, Save, X } from "lucide-react";
import Prism from "prismjs";
import "prismjs/components/prism-rust";
import type { Annotation, Problem } from "../types";
import { dateLabel } from "../types";

type Props = { problem: Problem; onClose: () => void; onSave: (annotation: Annotation) => Promise<void> };
export function SolutionDialog({ problem, onClose, onSave }: Props) {
  const dialog = useRef<HTMLDialogElement>(null);
  const [notes, setNotes] = useState(problem.notes);
  const [favorite, setFavorite] = useState(problem.favorite);
  const [review, setReview] = useState(problem.review);
  const [saved, setSaved] = useState<Annotation>(problem);
  const [saving, setSaving] = useState(false);
  const [status, setStatus] = useState("");
  const [copied, setCopied] = useState(false);
  const dirty = notes !== saved.notes || favorite !== saved.favorite || review !== saved.review;
  const highlighted = useMemo(() => Prism.highlight(problem.code, Prism.languages.rust!, "rust"), [problem.code]);
  useEffect(() => {
    const element = dialog.current!;
    element.showModal();
    return () => element.close();
  }, []);
  useEffect(() => {
    const beforeUnload = (event: BeforeUnloadEvent) => { if (dirty || saving) { event.preventDefault(); event.returnValue = ""; } };
    window.addEventListener("beforeunload", beforeUnload);
    return () => window.removeEventListener("beforeunload", beforeUnload);
  }, [dirty, saving]);
  function close() {
    if (saving || (dirty && !window.confirm("Discard unsaved notes?"))) return;
    onClose();
  }
  async function save() {
    const annotation = { filename: problem.filename, notes, favorite, review };
    setSaving(true); setStatus("");
    try { await onSave(annotation); setSaved(annotation); setStatus("Saved locally"); }
    catch (error) { setStatus(error instanceof Error ? error.message : "Could not save notes"); }
    finally { setSaving(false); }
  }
  async function copy() {
    try { await navigator.clipboard.writeText(problem.code); setCopied(true); }
    catch { setStatus("Clipboard unavailable. Select the code to copy it."); }
  }
  return <dialog ref={dialog} className="modal" onCancel={event => { event.preventDefault(); close(); }} aria-labelledby="solution-title">
    <div className="modal-box max-h-[90dvh] w-[94vw] max-w-5xl rounded-xl border border-base-300 bg-base-100 p-0">
      <div className="flex items-center justify-between border-b border-base-300 px-6 py-4"><span className="eyebrow">Solution notebook</span><button className="btn btn-ghost btn-square btn-sm" onClick={close} disabled={saving} aria-label="Close solution"><X size={18} /></button></div>
      <div className="px-6 py-6"><span className="font-mono text-[10px] tracking-widest text-base-content/45">PROBLEM {problem.id}</span><h2 id="solution-title" className="mt-2 mb-4 text-2xl font-medium tracking-tight sm:text-3xl">{problem.title}</h2><div className="flex flex-wrap items-center gap-4 text-xs text-base-content/50"><span className="badge badge-soft badge-sm topic-badge">{problem.topic}</span><span>{dateLabel(problem.completed)}</span><a className="link link-hover ml-auto flex items-center gap-1 text-primary" href={`https://leetcode.com/problems/${encodeURIComponent(problem.slug)}/`} target="_blank" rel="noreferrer">Open on LeetCode<ExternalLink size={12} /></a></div></div>
      <div className="flex items-center justify-between gap-3 bg-[#24342b] px-5 py-3 text-[#a4b6a9]"><span className="min-w-0 truncate font-mono text-[10px]">{problem.filename}</span><button className="btn btn-outline btn-xs shrink-0 border-white/20 text-[#cbd7c7] hover:bg-white/10" onClick={() => { void copy(); }}>{copied ? <Check size={12} /> : <Copy size={12} />}{copied ? 'Copied!' : 'Copy code'}</button></div>
      <pre className="source-code max-h-[48vh] overflow-auto p-5 text-xs leading-7" tabIndex={0} aria-label="Rust source code"><code dangerouslySetInnerHTML={{ __html: highlighted }} /></pre>
      <form className="px-6 py-6" onSubmit={e => { e.preventDefault(); void save(); }}><label htmlFor="solution-notes" className="block text-sm font-medium">Your notes</label><p className="mt-2 mb-4 text-xs text-base-content/45">Capture the idea, complexity, or what you’d do differently.</p><textarea id="solution-notes" className="textarea min-h-32 w-full border-base-300 text-xs leading-relaxed" maxLength={50000} value={notes} onChange={e => { setNotes(e.target.value); setStatus(""); }} placeholder="What clicked? What should future you remember?" disabled={saving} /><div className="mt-4 flex flex-wrap items-center gap-4 text-xs text-base-content/65"><label className="flex cursor-pointer items-center gap-2"><input className="checkbox checkbox-primary checkbox-xs" type="checkbox" checked={favorite} onChange={e => setFavorite(e.target.checked)} disabled={saving} />Favorite</label><label className="flex cursor-pointer items-center gap-2"><input className="checkbox checkbox-primary checkbox-xs" type="checkbox" checked={review} onChange={e => setReview(e.target.checked)} disabled={saving} />Revisit this problem</label><span className="ml-auto text-[10px]" role="status">{saving ? 'Saving…' : dirty ? status || 'Unsaved changes' : status}</span><button className="btn btn-primary btn-sm" type="submit" disabled={saving}>{saving ? <span className="loading loading-spinner loading-xs" /> : <Save size={13} />}Save notes</button></div></form>
    </div>
  </dialog>;
}
