// Check a coverage table that accounts for every state of a chart: what an
// implementation shows for each state, kept beside the chart it covers.
//
// The table is Markdown: one "## REGION" heading per region of the expanded chart,
// in its caption, then a table whose first column is the state and whose second is
// its status. Other columns are free text (where the implementation shows it, what
// it still needs). Only level-two headings name regions: put prose under a "#"
// heading, which no table follows into. A region's "(not in ...)" value is not a
// state and is not listed.
import { compile } from "./chart-verus.mjs";

export const STATUSES = ["built", "partial", "missing", "not shown"];
const quote = (s) => `"${s}"`;

export function readCoverage(text) {
  const regions = new Map();
  const problems = [];
  let region;
  text.split("\n").forEach((line, index) => {
    const at = `line ${index + 1}`;
    const heading = /^##\s+(.+?)\s*$/.exec(line);
    if (heading) {
      region = heading[1];
      if (regions.has(region)) problems.push(`${at}: the region ${quote(region)} is listed twice`);
      else regions.set(region, new Map());
      return;
    }
    if (/^#\s/.test(line)) { region = undefined; return; }
    if (!line.trim().startsWith("|") || !region) return;
    const cells = line.trim().replace(/^\||\|$/g, "").split("|").map((c) => c.trim());
    if (cells.every((c) => /^:?-+:?$/.test(c))) return;
    const [state, status] = cells;
    if (state === "State" && status === "Status") return;
    const states = regions.get(region);
    if (states.has(state)) problems.push(`${at}: ${quote(state)} is listed twice in ${quote(region)}`);
    states.set(state, { status, at });
  });
  return { regions, problems };
}

export function coverage(chartText, coverageText) {
  const { ir } = compile(chartText, "Chart");
  const { regions, problems } = readCoverage(coverageText);
  const counts = Object.fromEntries(STATUSES.map((s) => [s, 0]));
  for (const r of ir.regions) {
    const listed = regions.get(r.caption);
    const states = r.values.filter((v) => v !== r.inactive);
    if (!listed) { problems.push(`the region ${quote(r.caption)} has no "## ${r.caption}" section`); continue; }
    for (const s of states) {
      const row = listed.get(s);
      if (!row) { problems.push(`${quote(r.caption)} does not list the state ${quote(s)}`); continue; }
      if (!STATUSES.includes(row.status)) problems.push(`${row.at}: ${quote(s)} has the status ${quote(row.status ?? "")}; use ${STATUSES.join(", ")}`);
      else counts[row.status]++;
    }
    for (const [s, row] of listed) if (!states.includes(s)) problems.push(`${row.at}: ${quote(r.caption)} has no state ${quote(s)}`);
  }
  const known = new Set(ir.regions.map((r) => r.caption));
  for (const r of regions.keys()) if (!known.has(r)) problems.push(`the chart has no region ${quote(r)}`);
  return { problems, counts };
}

export function printCoverage({ problems, counts }) {
  for (const p of problems) console.log(p);
  const total = Object.values(counts).reduce((a, b) => a + b, 0);
  console.log(`${total} states: ${STATUSES.map((s) => `${counts[s]} ${s}`).join(", ")}.`);
  if (problems.length) console.log(`${problems.length} problem${problems.length === 1 ? "" : "s"}: the table does not match the chart.`);
  else console.log("The table accounts for every state of the chart.");
  return problems.length ? 1 : 0;
}
