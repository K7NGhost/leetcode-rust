import { readdir, readFile } from "node:fs/promises";
import { join } from "node:path";

export function parseProblem(filename: string, code: string) {
  const header = code.match(/\*\s*\[(\d+)\]\s*([^\r\n]+)/);
  const name = filename.match(/^(\d+)[.\s-]*(.+)\.rs$/);
  if (!header && !name) return null;
  const id = Number(header?.[1] ?? name![1]);
  const title = header?.[2].trim() ?? name![2].replaceAll("-", " ").replace(/\b\w/g, c => c.toUpperCase());
  const slug = (name?.[2] ?? title).toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
  const date = code.match(/\/\/\s*completed\s+(\d{1,2})\/(\d{1,2})\/(\d{4})/i);
  const completed = date ? `${date[3]}-${date[1].padStart(2, "0")}-${date[2].padStart(2, "0")}` : null;
  const topic = /TreeNode|binary.tree/i.test(code) ? "Trees"
    : /ListNode/.test(code) ? "Linked lists"
    : /VecDeque|sliding.window|moving.average|recent.calls/i.test(code) ? "Queues & windows"
    : /stack|parentheses|stock.span|temperatures|next.greater/i.test(code) ? "Stacks"
    : /HashMap|HashSet/.test(code) ? "Hashing"
    : /String|&str/.test(code) ? "Strings" : "Arrays";
  return { id, title, slug, filename, completed, topic, code, lines: code.trimEnd().split("\n").length };
}

export async function scanProblems(directory: string) {
  const names = (await readdir(directory)).filter(name => name.endsWith(".rs"));
  const problems = await Promise.all(names.map(async filename => parseProblem(filename, await readFile(join(directory, filename), "utf8"))));
  return problems.filter(p => p !== null).sort((a, b) => a.id - b.id || a.filename.localeCompare(b.filename));
}
