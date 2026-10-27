// Compose separate charts into one model through shared events, never copied states.
//
// A composition file is a stateDiagram-v2 whose %% lines name the charts and how they meet:
//
//   %% compose duet: ../duet/duet.mmd
//   %% compose physician: ../physician/physician.mmd
//   %% shared physician: the patient joins = duet: your Join on your practice goes through
//   %% outside physician: by the patient through Duet
//   %% never SameStatus: duet: Connected & physician: Not connected -- Both sides agree.
//
// Each chart is expanded on its own (templates, events, commands), then:
// - its state ids get its key as a prefix, so charts never share a state;
// - its events keep their labels, except an unshared label another chart also uses, which
//   gets " (KEY)" so the two stay separate events;
// - "%% shared F: L = O: L1 | O: L2" makes the follower F's event L the owners' events: every
//   edge F draws on L is drawn on each owner label instead. It keeps F's own needs and also
//   waits for the owner event's needs, so F reacts only when the owner's event can happen.
//   The owner's declaration (cause, needs, intent) stands; F's declaration of L goes;
// - "%% outside F: by CAUSE" says every event F declares with that cause happens in another
//   chart, so each must be shared;
// - each chart is one parallel part, and its regions are named under its top caption;
// - its own %% never rules keep holding, named KEY.Name; composition rules name a state as
//   KEY: caption.
import fs from "node:fs";
import path from "node:path";
import { readChart } from "./chart-parse.mjs";
import { expand } from "./chart-expand.mjs";

export const COMPOSES = /^\s*%%\s*compose\s+[A-Za-z][A-Za-z0-9_]*\s*:/m;

// Rewrite each "%% compose KEY: PATH" to an absolute path, so the composition reads the same
// charts wherever its text is compiled.
export function resolveComposition(text, file) {
  return text.replace(/^(\s*%%\s*compose\s+[A-Za-z][A-Za-z0-9_]*\s*:\s*)(.+?)\s*$/gm,
    (whole, head, target) => `${head}${path.resolve(path.dirname(file), target)}`);
}

// What a model generated from a composition derives from: the composition's own text and
// each composed chart's text, in composition order. Paths stay as written, so the identity
// is the same on every machine; each chart is also hashed alone for the map.
export function compositionSources(text, file) {
  return readComposition(text).charts.map(({ key, file: written }) => {
    const target = path.resolve(path.dirname(file), written);
    try { return { key, file: written, text: fs.readFileSync(target, "utf8") }; }
    catch { throw new Error(`cannot read the composed chart ${key}: ${target}`); }
  });
}

export function readComposition(text) {
  const charts = [], shared = [], outside = [], nevers = [];
  const keys = () => charts.map((c) => c.key);
  const ref = (s, line) => {
    const m = /^([A-Za-z][A-Za-z0-9_]*)\s*:\s*(.+)$/.exec(s.trim());
    if (!m || !keys().includes(m[1])) throw new Error(`name a composed chart as KEY: ... in: ${line}`);
    return { key: m[1], label: m[2].trim() };
  };
  if (!/^\s*stateDiagram-v2\s*$/m.test(text)) throw new Error("expected a stateDiagram-v2 composition");
  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (!line || /^stateDiagram/.test(line)) continue;
    if (!line.startsWith("%%")) throw new Error(`a composition holds only %% lines: ${line}`);
    let m = /^%%\s*compose\s+([A-Za-z][A-Za-z0-9_]*)\s*:\s*(.+)$/.exec(line);
    if (m) {
      if (keys().includes(m[1])) throw new Error(`the chart ${m[1]} is composed twice`);
      charts.push({ key: m[1], file: m[2].trim() });
      continue;
    }
    m = /^%%\s*shared\s+(.+?)\s*=\s*(.+)$/.exec(line);
    if (m) {
      const follower = ref(m[1], line);
      const owners = m[2].split("|").map((s) => ref(s, line));
      if (owners.some((o) => o.key === follower.key)) throw new Error(`a shared event joins different charts: ${line}`);
      if (shared.some((s) => s.follower.key === follower.key && s.follower.label === follower.label))
        throw new Error(`"${follower.label}" in ${follower.key} is shared twice`);
      shared.push({ follower, owners });
      continue;
    }
    m = /^%%\s*outside\s+([A-Za-z][A-Za-z0-9_]*)\s*:\s*by\s+(.+)$/.exec(line);
    if (m) {
      if (!keys().includes(m[1])) throw new Error(`%% outside names ${m[1]}, which is not composed: ${line}`);
      outside.push({ key: m[1], cause: m[2].trim() });
      continue;
    }
    const [rule, reason] = /^(.*?)\s*\|\s*reason:\s*(.+)$/.exec(line)?.slice(1) ?? [line, null];
    m = /^%%\s*never\s+([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(.+?)(?:\s+--\s+(.*))?$/.exec(rule);
    if (m) {
      const conditions = m[2].split("&").map((s) => ref(s, line));
      nevers.push({ name: m[1], states: conditions.map((c) => c.label), scopes: conditions.map((c) => c.key),
        sentence: m[3] ?? "", ...(reason ? { reason } : {}) });
      continue;
    }
    if (/^%%\s*(compose|shared|outside|never)\b/.test(line)) throw new Error(`malformed composition line: ${line}`);
  }
  if (charts.length < 2) throw new Error("a composition names at least two charts with %% compose");
  return { charts, shared, outside, nevers };
}

const walk = (q, visit) => {
  for (const p of q.parts) {
    visit.part?.(p);
    for (const t of p.transitions) visit.step?.(t);
    for (const c of p.children) { visit.node?.(c); if (!c.leaf) walk(c, visit); }
  }
};

// The composed syntax tree, ready to flatten like one expanded chart.
export function compose(text, options = {}) {
  const { charts, shared, outside, nevers: own } = readComposition(text);
  const problems = [];
  const problem = (message) => { if (!options.lenient) throw new Error(message); problems.push(message); };
  const expanded = charts.map(({ key, file }) => {
    let source;
    try { source = fs.readFileSync(file, "utf8"); }
    catch { throw new Error(`cannot read the composed chart ${key}: ${file}`); }
    if (COMPOSES.test(source)) throw new Error(`the chart ${key} is itself a composition; compose charts, not compositions`);
    return { key, x: expand(readChart(source), options) };
  });
  // Every label each chart carries, before renaming.
  const labelsOf = (x) => {
    const labels = new Set(x.catalog.keys());
    walk(x.root, { step: (t) => { if (t.label) labels.add(t.label); } });
    return labels;
  };
  const labels = new Map(expanded.map(({ key, x }) => [key, labelsOf(x)]));
  for (const { follower, owners } of shared) {
    if (!labels.get(follower.key).has(follower.label)) problem(`%% shared names "${follower.label}", which ${follower.key} does not carry`);
    for (const o of owners) if (!labels.get(o.key).has(o.label)) problem(`%% shared names "${o.label}", which ${o.key} does not carry`);
  }
  for (const { owners } of shared)
    for (const o of owners)
      if (shared.some((s) => s.follower.key === o.key && s.follower.label === o.label))
        problem(`"${o.label}" in ${o.key} is itself shared as another chart's event; name that event instead`);
  for (const { key, cause } of outside)
    for (const [label, { decl, values }] of expanded.find((e) => e.key === key).x.catalog)
      if (decl.cause.replace(/\{([A-Za-z_]\w*)\}/g, (w, n) => values[n] ?? w) === cause
        && !shared.some((s) => s.follower.key === key && s.follower.label === label))
        problem(`"${label}" in ${key} is caused by ${cause}, outside ${key}, but no %% shared line says which event it is`);
  // An owner keeps its label; any other chart's unshared event of the same name is renamed.
  const owns = new Set(shared.flatMap((s) => s.owners.map((o) => `${o.key}\n${o.label}`)));
  const followed = (key, label) => shared.find((s) => s.follower.key === key && s.follower.label === label);
  const clash = (key, label) => !owns.has(`${key}\n${label}`)
    && [...labels].some(([k, set]) => k !== key && set.has(label) && !followed(k, label));

  const names = new Map(), catalog = new Map(), groups = new Map(), declared = [], instances = [], ignored = [];
  const nevers = [];
  const parts = [];
  // Each owner event's needs and limits, in the owner's prefixed ids, for the followers' steps
  // on it. A command's refusal also carries what the command needs, its click's limits
  // included; one of them must fail for it to happen.
  const ownerNeeds = new Map(), ownerRefuses = new Map();
  for (const { key, x } of expanded)
    for (const [l, { decl, values, limits = [] }] of x.catalog) {
      if (!owns.has(`${key}\n${l}`)) continue;
      const prefixed = (g) => ({ not: g.not, ids: g.ids.map((i) => `${key}_${i.replace(/\{([A-Za-z_]\w*)\}/g, (w, n) => values[n] ?? w)}`),
        ...(g.reason ? { reason: g.reason } : {}), ...(g.cause ? { cause: g.cause } : {}) });
      const all = [decl.needs, ...limits].filter(Boolean).map(prefixed);
      if (all.length) ownerNeeds.set(l, all);
      if (decl.refusal)
        ownerRefuses.set(l, { refusal: true, ...(decl.refuses ? { refuseIds: prefixed(decl.refuses) } : {}),
          ...(decl.refuseLimits ? { refuseLimits: decl.refuseLimits.map(prefixed) } : {}) });
    }
  for (const { key, x } of expanded) {
    const id = (s) => (s === "[*]" ? s : `${key}_${s}`);
    const ids = (g) => g && { ...g, ids: g.ids.map(id) };
    const label = (l) => (l && !followed(key, l) && clash(key, l) ? `${l} (${key})` : l);
    for (const [k, v] of x.names) names.set(id(k), v);
    if (x.root.parts.length !== 1) throw new Error(`the chart ${key} needs one top-level composite`);
    const [top] = x.root.parts;
    const tops = top.children.filter((c) => !c.leaf);
    if (tops.length !== 1 || top.children.length !== 1)
      throw new Error(`the chart ${key} needs exactly one top-level composite to compose`);
    walk(x.root, {
      node: (c) => {
        c.name = id(c.name);
        // A template instance keeps naming its template, so its rules find their states in it.
        if (c.instance) c.instance = { ...c.instance, template: id(c.instance.template) };
      },
      step: (t) => {
        t.from = id(t.from);
        t.to = id(t.to);
        for (const f of ["when", "needsIds", "refuseIds"]) if (t[f]) t[f] = ids(t[f]);
        for (const f of ["causeIds", "refuseLimits"]) if (t[f]) t[f] = t[f].map(ids);
      },
    });
    // A follower's step on a shared event is drawn on each owner label instead.
    walk(x.root, {
      part: (p) => {
        p.transitions = p.transitions.flatMap((t) => {
          const s = t.label && followed(key, t.label);
          if (!s) return [{ ...t, label: label(t.label), text: t.label ? t.text.replace(t.label, label(t.label)) : t.text }];
          return s.owners.map((o) => {
            if (t.refusal && ownerRefuses.has(o.label))
              throw new Error(`${key}'s refusal "${t.label}" can't be shared onto the refusal "${o.label}"; share it onto an event that is not a refusal`);
            return { ...t, label: o.label, text: t.text.replace(t.label, o.label), shared: true,
              ...(ownerNeeds.has(o.label) ? { ownerIds: ownerNeeds.get(o.label) } : {}),
              ...(ownerRefuses.get(o.label) ?? {}) };
          });
        });
      },
    });
    for (const [l, entry] of x.catalog) {
      if (followed(key, l)) continue;
      const decl = { ...entry.decl,
        ...(entry.decl.needs ? { needs: ids(entry.decl.needs) } : {}),
        ...(entry.decl.refuseLimits ? { refuseLimits: entry.decl.refuseLimits.map(ids) } : {}),
        ...(entry.decl.accepts ? { accepts: label(entry.decl.accepts) } : {}),
        ...(entry.decl.command?.client ? { command: { ...entry.decl.command, client: entry.decl.command.client.map(id) } } : {}) };
      const name = label(l);
      if (catalog.has(name)) throw new Error(`the event "${name}" is declared by two composed charts; share it or rename one`);
      catalog.set(name, { ...entry, decl, ...(entry.limits ? { limits: entry.limits.map(ids) } : {}) });
    }
    for (const d of x.events ?? []) declared.push({ ...d, label: followed(key, d.label) ? d.label : label(d.label) });
    for (const [g, members] of x.groups ?? []) groups.set(groups.has(g) ? `${g} (${key})` : g, members.map(label));
    for (const i of x.instances) instances.push({ ...i, template: id(i.template) });
    ignored.push(...(x.ignored ?? []));
    problems.push(...(x.problems ?? []));
    const chart = names.get(tops[0].name) ?? tops[0].name;
    for (const n of x.nevers)
      nevers.push({ ...n, name: `${key}.${n.name}`, scopes: n.states.map(() => chart),
        ...(n.home ? { home: { ...n.home, template: id(n.home.template) } } : {}) });
    parts.push({ children: [tops[0]], transitions: [{ from: "[*]", to: tops[0].name, text: "", label: "" }] });
  }
  const scopeOf = new Map(expanded.map(({ x }, i) => [charts[i].key, names.get(parts[i].children[0].name)]));
  for (const n of own) {
    if (nevers.some((m) => m.name === n.name)) throw new Error(`duplicate never name ${n.name}`);
    nevers.push({ ...n, scopes: n.scopes.map((k) => scopeOf.get(k)) });
  }
  names.set("composition", "Composition");
  const root = { name: "", parts: [{ children: [{ name: "composition", parts }], transitions: [] }] };
  return { root, names, nevers, instances, ignored, catalog, events: declared, groups,
    strict: expanded.every(({ x }) => x.strict), problems, keys: scopeOf };
}
