import type { parseProblem } from "../../catalog";

export type Problem = NonNullable<ReturnType<typeof parseProblem>> & Annotation;
export type Annotation = { filename: string; notes: string; favorite: boolean; review: boolean };
export type View = "all" | "favorite" | "review";
export const viewTitles: Record<View, string> = { all: "All solutions", favorite: "Favorites", review: "To revisit" };
export function dateLabel(date: string | null) {
  return date ? new Date(`${date}T12:00:00`).toLocaleDateString("en-US", { month: "short", day: "numeric", year: "numeric" }) : "Not recorded";
}
