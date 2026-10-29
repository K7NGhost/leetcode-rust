import { useCallback, useEffect, useState } from "react";
import type { Annotation, Problem } from "../types";

export function useProblems(paused: boolean) {
  const [problems, setProblems] = useState<Problem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  useEffect(() => {
    if (paused) return;
    const controller = new AbortController();
    let inFlight = false;
    async function sync() {
      if (inFlight || document.hidden) return;
      inFlight = true;
      try {
        const response = await fetch("/api/problems", { signal: controller.signal });
        if (!response.ok) throw new Error("Could not read solutions");
        const data: Problem[] = await response.json();
        if (!controller.signal.aborted) { setProblems(data); setError(""); }
      } catch {
        if (!controller.signal.aborted) setError("Connection lost. Retrying automatically…");
      } finally {
        inFlight = false;
        if (!controller.signal.aborted) setLoading(false);
      }
    }
    void sync();
    const interval = window.setInterval(() => { void sync(); }, 3000);
    const onVisible = () => { if (!document.hidden) void sync(); };
    document.addEventListener("visibilitychange", onVisible);
    return () => { controller.abort(); clearInterval(interval); document.removeEventListener("visibilitychange", onVisible); };
  }, [paused]);

  const save = useCallback(async (annotation: Annotation) => {
    const response = await fetch("/api/annotation", {
      method: "PUT", headers: { "Content-Type": "application/json" }, body: JSON.stringify(annotation),
    });
    if (!response.ok) throw new Error("Save failed. Please try again.");
    setProblems(current => current.map(p => p.filename === annotation.filename ? { ...p, ...annotation } : p));
  }, []);
  return { problems, loading, error, save };
}
