// Read a compiled chart as a graph: summaries, ways in and out of a state, who reacts
// to an event, region summaries, and static lint. Everything here reads the same flat
// model the Verus backend and the search use; nothing is inferred from prose.
// Promoted from the scratch graph tools (metrics, list states, events, reset counts,
// static analysis) and PStack's ways-in, at upstream 00d5dec.
import { guardWords, limitWords } from "./chart-flat.mjs";
import { placeholders } from "./chart-expand.mjs";

const quoted = (s) => `"${s}"`;
const plural = (n, word) => `${n} ${word}${n === 1 ? "" : "s"}`;

function graph({ ir, parsed }) {
  const { regions, actions } = ir;
  const index = new Map(regions.map((r, i) => [r.caption, i]));
  const children = regions.map((_, i) => regions.flatMap((r, ki) => (r.scope && r.parent === i ? [ki] : [])));
  const edges = regions.flatMap((r, ri) => r.transitions.map((t) => ({ ...t, ri })));
  const info = (label) => parsed.events?.get(label);
  // States a region can be entered at: its starts, labeled starts and step targets inside it.
  const entries = (ri) => {
    const r = regions[ri];
    const found = new Set(r.initial);
    for (const states of r.starts?.values() ?? []) for (const s of states) found.add(s);
    if (r.scope) for (const t of regions[r.parent].transitions) for (const s of t.enter?.[r.caption] ?? []) found.add(s);
    return found;
  };
  // Reachability inside one region, ignoring guards and other regions (the upstream idea
  // "states the chart's own steps never reach", made structural for large charts).
  const unreachable = (ri) => {
    const r = regions[ri];
    const seen = entries(ri);
    if (r.scope) seen.add(r.inactive);
    const queue = [...seen];
    while (queue.length) {
      const s = queue.shift();
      for (const t of r.transitions) if (t.from === s && !seen.has(t.to)) { seen.add(t.to); queue.push(t.to); }
    }
    return r.values.filter((s) => !seen.has(s));
  };
  const reads = (ri) => new Set(regions[ri].transitions.flatMap((t) =>
    [t.guard, t.needs, ...(t.within ?? []), ...(t.unless ?? []).flat()].filter(Boolean).flatMap((g) => Object.keys(g.states).map((c) => index.get(c)))));
  return { regions, actions, index, children, edges, info, entries, unreachable, reads };
}

function describe(t, g) {
  const parts = [];
  if (t.guard) parts.push(`guard: ${limitWords(t.guard)}`);
  for (const w of t.within ?? []) parts.push(`part condition: ${guardWords(w)}`);
  for (const c of t.unless ?? []) parts.push(`yields to a deeper step ${c.map(guardWords).join(" and ")}`);
  const e = t.label ? g.info(t.label) : null;
  if (e) {
    parts.push(`cause: ${e.cause}`);
    if (e.needs) parts.push(`needs: ${limitWords(e.needs)}`);
  }
  if (t.optional) parts.push("optional");
  if (t.intent) parts.push(`intent: ${t.intent}`);
  else if (e) parts.push(`event intent: ${e.intent}`);
  if (t.enter) for (const [part, states] of Object.entries(t.enter)) parts.push(`enters ${part} at ${states.join(" or ")}`);
  return parts.length ? ` (${parts.join("; ")})` : "";
}
const on = (t) => (t.label ? `on ${quoted(t.label)}` : "on an unlabeled step");

export function summary(compiled) {
  const g = graph(compiled);
  const { regions, actions, edges } = g;
  const lines = [];
  const scoped = regions.filter((r) => r.scope).length;
  const templated = regions.filter((r) => r.instance).length;
  const states = regions.reduce((n, r) => n + r.values.length, 0);
  const loops = edges.filter((t) => t.from === t.to).length;
  const declared = compiled.parsed.events?.size ?? 0;
  lines.push(`${plural(regions.length, "region")} (${scoped} scoped, ${templated} from templates), ${plural(states, "state")}, ${plural(edges.length, "edge")} (${loops} self-loops), ${plural(actions.length, "event")} (${declared} declared${compiled.parsed.strict ? ", strict" : ""}), ${plural(compiled.ir.rules.length, "rule")}`);
  const largest = regions.map((r) => [r.transitions.length, r.caption]).sort((a, b) => b[0] - a[0]).slice(0, 10);
  lines.push(`Largest regions by edges: ${largest.map(([n, c]) => `${c} (${n})`).join(", ")}`);
  const wide = actions.map((a) => [a.moves.size, a.label || "(unlabeled)"]).sort((a, b) => b[0] - a[0]).slice(0, 10);
  lines.push("Widest events (regions reacting):");
  for (const [n, l] of wide) lines.push(`  ${n}  ${l}`);
  const resets = new Map();
  for (const t of edges) {
    const r = regions[t.ri];
    if (t.from === t.to || !r.initial.has(t.to)) continue;
    resets.set(t.label, (resets.get(t.label) ?? 0) + 1);
  }
  lines.push(`Edges returning a region to its start: ${[...resets.values()].reduce((a, b) => a + b, 0)}`);
  for (const [l, n] of [...resets].sort((a, b) => b[1] - a[1]).slice(0, 10)) lines.push(`  ${n}  ${l || "(unlabeled)"}`);
  return lines.join("\n");
}

export function queryState(compiled, text) {
  const g = graph(compiled);
  const { regions, edges } = g;
  const owner = compiled.parsed.owners.get(text);
  const hits = owner
    ? [{ ri: g.index.get(owner.name), states: owner.all }]
    : regions.flatMap((r, ri) => {
      const states = r.states.has(text) ? [text] : r.values.filter((s) => s.startsWith(`${text}.`));
      return states.length ? [{ ri, states }] : [];
    });
  if (!hits.length) throw new Error(`no state has the id or caption ${quoted(text)}`);
  const lines = [];
  for (const { ri, states } of hits) {
    const r = regions[ri];
    const set = new Set(states);
    lines.push(`${quoted(text)} in region ${quoted(r.caption)}${states.length > 1 ? ` (${states.length} nested states)` : ""}${r.instance ? ` (template ${r.instance.template}, instance ${r.instance.key})` : ""}`);
    const ins = [], outs = [];
    for (const s of states) if (r.initial.has(s)) ins.push(`  start${r.scope ? ` when ${r.scope.region} enters ${r.scope.state}` : ""}: ${s}`);
    for (const [label, ss] of r.starts ?? []) for (const s of ss) if (set.has(s)) ins.push(`  labeled start when ${r.scope.region} enters ${r.scope.state} ${on({ label })}: ${s}`);
    if (r.scope) for (const t of regions[r.parent].transitions) for (const s of t.enter?.[r.caption] ?? [])
      if (set.has(s)) ins.push(`  ${r.scope.region}: ${t.from} --> ${t.to} ${on(t)} enters here at ${s}`);
    for (const t of edges) if (t.ri === ri && set.has(t.to) && !set.has(t.from)) ins.push(`  from ${quoted(t.from)} ${on(t)}${describe(t, g)}`);
    for (const t of edges) if (t.ri === ri && set.has(t.from) && !set.has(t.to)) outs.push(`  to ${quoted(t.to)} ${on(t)}${describe(t, g)}`);
    // Steps drawn from inside a part to outside its composite are moves of the parent.
    const leaves = edges.filter((t) => r.scope && t.ri === r.parent && t.within?.some((w) => !w.not && w.states[r.caption]?.some((s) => set.has(s))));
    for (const t of leaves) outs.push(`  leaves with ${r.scope.region}: ${t.from} --> ${t.to} ${on(t)}${describe(t, g)}`);
    if (r.scope) outs.push(`  closes (to ${r.inactive}) whenever ${r.scope.region} leaves ${r.scope.state}`);
    const loops = edges.filter((t) => t.ri === ri && set.has(t.from) && set.has(t.to));
    lines.push(`Ways in (${ins.length}):`, ...ins, `Ways out (${outs.length}):`, ...outs);
    if (loops.length) {
      lines.push(`Within (${loops.length}):`);
      for (const t of loops) lines.push(`  ${quoted(t.from)} to ${quoted(t.to)} ${on(t)}${describe(t, g)}`);
    }
    const readers = edges.filter((t) => [t.guard, t.needs, ...(t.within ?? []), ...(t.unless ?? []).flat()].some((x) => x?.states[r.caption]?.some((s) => set.has(s))));
    if (readers.length) {
      lines.push(`Read by (${readers.length}):`);
      for (const t of readers) lines.push(`  ${regions[t.ri].caption}: ${t.from} --> ${t.to} ${on(t)}`);
    }
  }
  return lines.join("\n");
}

export function queryEvent(compiled, text) {
  const g = graph(compiled);
  const { parsed } = compiled;
  const lines = [];
  let labels = [text];
  if (parsed.groups?.has(text)) {
    labels = g.actions.map((a) => a.label).filter((l) => parsed.groups.get(text).some((m) => m === l || parsed.events.get(l)?.family === m));
    lines.push(`Group ${quoted(text)} = ${parsed.groups.get(text).join(" | ")}; members carried: ${labels.length}`);
  } else if (placeholders(text).length) {
    labels = g.actions.map((a) => a.label).filter((l) => parsed.events?.get(l)?.family === text);
    lines.push(`Family ${quoted(text)}; members carried: ${labels.length}`);
  }
  for (const label of labels) {
    const action = g.actions.find((a) => a.label === label);
    const e = g.info(label);
    if (!action && !e) throw new Error(`no event, family or group is labeled ${quoted(text)}`);
    lines.push(`Event ${quoted(label)}`);
    if (e) {
      lines.push(`  cause: ${e.cause}`);
      lines.push(`  needs: ${e.needs ? limitWords(e.needs) : "nothing"}`);
      lines.push(`  intent: ${e.intent}`);
      if (e.family) lines.push(`  family: ${e.family}`);
    } else lines.push("  not declared");
    const moves = action ? [...action.moves] : [];
    lines.push(`  reacting regions (${moves.length}):`);
    for (const [ri, ms] of moves) {
      lines.push(`    ${g.regions[ri].caption}`);
      for (const m of ms) lines.push(`      ${m.from} --> ${m.to}${describe({ ...m, label: "" }, g)}`);
    }
    for (const [ri, r] of g.regions.entries())
      if (r.starts?.has(label)) lines.push(`  labeled start: ${r.caption} enters at ${r.starts.get(label).join(" or ")} when ${r.scope.region} enters ${r.scope.state}`);
  }
  return lines.join("\n");
}

export function queryRegion(compiled, text) {
  const g = graph(compiled);
  const ri = g.index.get(text);
  if (ri === undefined) throw new Error(`no region is named ${quoted(text)}`);
  const r = g.regions[ri];
  const lines = [`Region ${quoted(r.caption)}`];
  if (r.scope) lines.push(`  scoped: active only while ${r.scope.region} is in ${r.scope.state}`);
  if (r.instance) lines.push(`  template ${r.instance.template}, instance ${r.instance.key}`);
  lines.push(`  states (${r.values.length}): ${r.values.join(" / ")}`);
  lines.push(`  starts: ${[...r.initial].join(" / ")}`);
  for (const [label, states] of r.starts ?? []) lines.push(`  labeled start on ${quoted(label)}: ${states.join(" / ")}`);
  const events = new Set(r.transitions.map((t) => t.label));
  lines.push(`  ${plural(r.transitions.length, "edge")}, ${plural(events.size, "event")}`);
  const kids = g.children[ri].map((k) => g.regions[k].caption);
  if (kids.length) lines.push(`  parts: ${kids.join(", ")}`);
  const reads = [...g.reads(ri)].filter((x) => x !== ri).map((x) => g.regions[x].caption);
  lines.push(`  reads: ${reads.length ? reads.join(", ") : "no other region"}`);
  const readBy = g.regions.flatMap((x, xi) => (xi !== ri && g.reads(xi).has(ri) ? [x.caption] : []));
  lines.push(`  read by: ${readBy.length ? readBy.join(", ") : "no other region"}`);
  const rules = compiled.ir.rules.filter((rule) => rule.kind === "scope" ? rule.ri === ri : rule.parts.some((p) => p.ri === ri));
  if (rules.length) lines.push(`  rules: ${rules.map((x) => x.name).join(", ")}`);
  const unreachable = g.unreachable(ri);
  if (unreachable.length) lines.push(`  unreachable (ignoring guards): ${unreachable.join(" / ")}`);
  const sinks = r.values.filter((s) => s !== r.inactive && !r.transitions.some((t) => t.from === s && t.to !== s));
  if (sinks.length) lines.push(`  no way out (within the region): ${sinks.join(" / ")}`);
  return lines.join("\n");
}

// Static findings. Each is a fact about the flat model, not a judgement of design.
// With cause, also list every limit (a guard on a move, or the event's needs) on an event
// declared with that cause that states no reason.
export function lint(compiled, { cause } = {}) {
  const g = graph(compiled);
  const { regions, edges } = g;
  const { parsed } = compiled;
  const findings = [];
  for (const [ri, r] of regions.entries()) {
    const unreachable = g.unreachable(ri);
    if (unreachable.length) findings.push(`unreachable (ignoring guards): ${r.caption}: ${unreachable.join(" / ")}`);
  }
  // Dead edges (a): an ancestor always leaves or re-enters the scoping state on this event.
  const always = (ri, label) => {
    for (let x = ri; regions[x].scope; x = regions[x].parent) {
      const { region, state } = regions[x].scope;
      const pi = regions[x].parent;
      const exit = regions[pi].transitions.find((t) => t.label === label && t.from === state
        && !t.guard && !t.within?.length && !t.unless?.length && !t.optional);
      if (exit) return `${region} always ${exit.to === state ? "re-enters" : "leaves"} ${state} on this event`;
    }
    return null;
  };
  // Dead edges (b): conditions that name states which cannot be active together.
  const impossible = (t) => {
    const allowed = new Map();
    const restrict = (ri, states) => {
      const cur = allowed.get(ri);
      allowed.set(ri, new Set(cur ? [...cur].filter((s) => states.includes(s)) : states));
    };
    restrict(t.ri, [t.from]);
    const conditions = [t.guard, t.needs, ...(t.within ?? [])].filter(Boolean);
    for (const c of conditions) if (!c.not) for (const [region, states] of Object.entries(c.states)) restrict(g.index.get(region), states);
    for (let changed = true; changed;) {
      changed = false;
      for (const [ri, set] of [...allowed]) {
        const r = regions[ri];
        if (!r.scope || !set.size || set.has(r.inactive)) continue;
        const before = allowed.get(r.parent)?.size;
        restrict(r.parent, [r.scope.state]);
        if (allowed.get(r.parent).size !== before) changed = true;
      }
    }
    for (const [ri, set] of allowed) if (!set.size) return `${regions[ri].caption} would have to be in two states at once`;
    for (const c of conditions) if (c.not)
      for (const [region, states] of Object.entries(c.states)) {
        const set = allowed.get(g.index.get(region));
        if (set && [...set].every((s) => states.includes(s))) return `it needs ${region} not in ${states.join(" or ")}, which its other conditions require`;
      }
    return null;
  };
  for (const t of edges) {
    const why = (t.label && regions[t.ri].scope ? always(t.ri, t.label) : null) ?? impossible(t);
    if (why) findings.push(`dead edge: ${regions[t.ri].caption}: ${t.from} --> ${t.to} ${on(t)}: ${why}`);
  }
  for (const p of parsed.problems ?? []) findings.push(p);
  const carried = new Set(g.actions.map((a) => a.label));
  for (const [label] of parsed.events ?? []) if (!carried.has(label)) findings.push(`declared event nothing carries: ${quoted(label)}`);
  for (const d of parsed.declared ?? [])
    if (placeholders(d.label).length && ![...parsed.events.values()].some((e) => e.family === d.label))
      findings.push(`declared family no template instance binds: ${quoted(d.label)}`);
  const known = (m) => parsed.declared.some((d) => d.label === m) || carried.has(m)
    || [...(parsed.events?.values() ?? [])].some((e) => e.family === m);
  for (const [name, members] of parsed.groups ?? [])
    for (const m of members) if (!known(m)) findings.push(`group ${quoted(name)}: member ${quoted(m)} is neither a declared event nor a step label`);
  for (const line of parsed.ignored ?? []) findings.push(`read as a plain comment (not the construct's shape): ${line}`);
  if (cause !== undefined) {
    const wanted = cause.trim().toLowerCase();
    for (const a of g.actions) {
      const e = g.info(a.label);
      if (!e || e.cause.trim().toLowerCase() !== wanted) continue;
      const what = `restricts without a stated reason: ${quoted(a.label)}`;
      if (e.needs && !e.needs.reason) findings.push(`${what} needs: ${guardWords(e.needs)}`);
      for (const [ri, moves] of a.moves)
        for (const m of moves) if (m.guard && !m.guard.reason)
          findings.push(`${what}: ${regions[ri].caption}: ${m.from} --> ${m.to} ${guardWords(m.guard)}`);
    }
  }
  return findings;
}
