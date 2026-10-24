// Compare two charts by their flat models: regions, states, starts, steps (with guards,
// needs, part conditions, entry targets and optionality), declared events and rules.
// A rename map carries the first chart's names into the second's vocabulary first:
// { "regions": { old: new }, "states": { old: new }, "events": { old: new } }.
// State renames apply in every region. Intent text is compared only for declared events;
// the reason every limit states is compared everywhere.
import { compile } from "./chart-verus.mjs";
import { limitWords, reasonSuffix } from "./chart-flat.mjs";

const KINDS = ["regions", "states", "events"];

export function readRenames(json) {
  const map = JSON.parse(json);
  if (!map || typeof map !== "object" || Array.isArray(map)) throw new Error("the rename map must be a JSON object");
  for (const key of Object.keys(map))
    if (!KINDS.includes(key)) throw new Error(`the rename map has "${key}"; use only ${KINDS.join(", ")}`);
  for (const kind of KINDS) {
    const table = map[kind] ?? {};
    if (typeof table !== "object" || Array.isArray(table) || Object.values(table).some((v) => typeof v !== "string"))
      throw new Error(`the rename map's "${kind}" must map names to names`);
  }
  return { regions: map.regions ?? {}, states: map.states ?? {}, events: map.events ?? {} };
}

// The comparable model: sets of sentences, one per fact, in the second chart's names.
function model(text, renames = { regions: {}, states: {}, events: {} }) {
  const { ir, parsed } = compile(text, "Chart");
  const region = (c) => renames.regions[c] ?? c;
  const state = (s) => renames.states[s] ?? s;
  const event = (l) => renames.events[l] ?? l;
  const states = (list) => [...list].map(state).sort();
  const condition = (g) => g && limitWords({ ...g, states: Object.fromEntries(Object.entries(g.states).map(([r, s]) => [region(r), states(s)]).sort()) });
  const facts = { regions: new Set(), states: new Set(), steps: new Set(), events: new Set(), rules: new Set() };
  for (const r of ir.regions) {
    const name = region(r.caption);
    facts.regions.add(`${name}${r.scope ? ` (active while ${region(r.scope.region)} is ${state(r.scope.state)})` : ""}`);
    for (const s of r.values) facts.states.add(`${name}: ${state(s)}${r.initial.has(s) ? " (start)" : ""}`);
    for (const [label, list] of r.starts ?? []) facts.states.add(`${name}: entered on "${event(label)}" at ${states(list).join(" or ")}`);
    for (const t of r.transitions) {
      const extra = [condition(t.guard), t.needs && `needs ${condition(t.needs)}`,
        ...(t.within ?? []).map((w) => `within ${condition(w)}`),
        ...(t.unless ?? []).map((c) => `blocked while ${c.map(condition).join(" and ")}`),
        ...Object.entries(t.enter ?? {}).map(([p, list]) => `enters ${region(p)} at ${states(list).join(" or ")}`),
        t.optional && "may"].filter(Boolean);
      facts.steps.add(`${name}: ${state(t.from)} --> ${state(t.to)} on ${t.label ? `"${event(t.label)}"` : "an unlabeled step"}${extra.length ? ` (${extra.join("; ")})` : ""}`);
    }
  }
  for (const [label, e] of parsed.events ?? [])
    facts.events.add(`"${event(label)}" by ${e.cause}${e.needs ? `, needs ${condition(e.needs)}` : ""}: ${e.intent}`);
  // Scope rules follow from the regions' activity, listed above.
  for (const rule of ir.rules) if (rule.kind !== "scope")
    facts.rules.add(`never ${rule.parts.map((p) => `${region(ir.regions[p.ri].caption)} is ${states(p.states).join(" or ")}`).join(" & ")}${reasonSuffix(rule)}`);
  return facts;
}

export function compareCharts(a, b, renames) {
  const left = model(a, renames), right = model(b);
  return Object.keys(left).map((kind) => ({
    kind,
    removed: [...left[kind]].filter((f) => !right[kind].has(f)).sort(),
    added: [...right[kind]].filter((f) => !left[kind].has(f)).sort(),
  }));
}

// Print the differences; returns the exit code: 0 same flat model, 1 differences.
export function printComparison(sections) {
  let total = 0;
  for (const { kind, removed, added } of sections) {
    if (!removed.length && !added.length) continue;
    console.log(`${kind}:`);
    for (const f of removed) console.log(`  - ${f}`);
    for (const f of added) console.log(`  + ${f}`);
    total += removed.length + added.length;
  }
  console.log(total ? `${total} difference${total === 1 ? "" : "s"} (- only in the first chart, + only in the second).` : "Same flat model.");
  return total ? 1 : 0;
}
