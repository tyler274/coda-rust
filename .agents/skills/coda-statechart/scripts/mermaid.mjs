// Adapted from cursor/plugins pstack at eb4db2b55b2b7dec3d9282754cddf0a56bf4ff10.
// Mermaid parsing/flattening is shared with its chart-to-tla implementation.
// MIT license: ../../../../verification/PSTACK-LICENSE (relative to this directory).
// Coda rejects guard text without a machine guard, malformed rule annotations,
// guarded initial arrows, and ambiguous region/state identities instead of dropping them.
const unwrap = (s) =>
  s
    .replace(/<br\s*\/?>|#160;/g, " ")
    .replace(/#(quot|34);/g, '"')
    .replace(/\s+/g, " ")
    .trim();

export const LEGEND_ID = "legend";

export function parseMermaid(text) {
  const names = new Map();
  const nevers = [];
  let last = null;
  const newPart = () => ({ children: [], transitions: [] });
  const root = { name: "", parts: [newPart()] };
  const stack = [root];
  const part = () => stack[stack.length - 1].parts.at(-1);
  const display = (id) => names.get(id) ?? id;
  const implicit = new Map();
  const declare = (id) => {
    const prev = implicit.get(id);
    if (prev)
      prev.children.splice(
        prev.children.findIndex((c) => c.name === id),
        1,
      );
    implicit.set(id, null);
  };
  const mention = (id) => {
    if (id === "[*]" || implicit.has(id)) return;
    part().children.push({ name: id, leaf: true });
    implicit.set(id, part());
  };
  const body = text.replace(/^\s*---[ \t]*\r?\n[^]*?\r?\n---[ \t]*(\r?\n|$)/, "");
  if (!/^\s*stateDiagram-v2\s*$/m.test(body))
    throw new Error("expected a stateDiagram-v2 chart (not a flowchart or Markdown fence)");
  for (const raw of body.split("\n")) {
    const line = raw.trim().replace(/(\[\*\]|[A-Za-z_]\w*):::[\w-]+/g, "$1");
    const when = /^%%\s*when\s+(not\s+)?(.+)$/.exec(line);
    if (when) {
      if (!last) throw new Error(`a %% when line must come right after a transition: ${line}`);
      if (!last.words)
        throw new Error(
          `the guarded step "${last.label}" must end its label with the guard's words in [ ]`,
        );
      last.when = { not: Boolean(when[1]), ids: when[2].split(",").map((s) => s.trim()) };
      last = null;
      continue;
    }
    last = null;
    const never = /^%%\s*never\s+([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(.+?)(?:\s+--\s+(.*))?$/.exec(line);
    if (never) {
      nevers.push({
        name: never[1],
        states: never[2].split("&").map((s) => s.trim()),
        sentence: never[3] ?? "",
      });
      continue;
    }
    if (/^%%\s*(when|never)\b/.test(line))
      throw new Error(`malformed formal annotation: ${line}`);
    if (
      !line ||
      line.startsWith("%%") ||
      /^(stateDiagram|direction|classDef|class |note)/.test(line)
    )
      continue;
    let m = /^state\s+"([^"]*)"\s+as\s+([A-Za-z_]\w*)\s*(\{)?\s*$/.exec(line);
    if (!m) {
      const plain = /^state\s+([A-Za-z_]\w*)\s*(\{)?\s*$/.exec(line);
      if (plain) m = [line, null, plain[1], plain[2]];
    }
    if (m) {
      if (m[1] !== null && !names.has(m[2])) names.set(m[2], unwrap(m[1]));
      declare(m[2]);
      if (m[3]) {
        const node = { name: m[2], parts: [newPart()] };
        part().children.push(node);
        stack.push(node);
      } else part().children.push({ name: m[2], leaf: true });
      continue;
    }
    if (line === "}") {
      if (stack.length === 1) throw new Error("unmatched } in the chart");
      stack.pop();
      continue;
    }
    if (line === "--") {
      stack[stack.length - 1].parts.push(newPart());
      continue;
    }
    m = /^(\[\*\]|[A-Za-z_]\w*)\s*-->\s*(\[\*\]|[A-Za-z_]\w*)(?:\s*:\s*(.*))?$/.exec(line);
    if (m) {
      if (m[2] === "[*]" && m[1] !== "[*]")
        throw new Error(
          `cannot read final state in "${line}"; draw the step to the state the reader sees next`,
        );
      mention(m[1]);
      mention(m[2]);
      const said = m[3] ? unwrap(m[3]) : "";
      const label = said.replace(/\s*\[[^\]]*\]/g, "");
      last = { from: m[1], to: m[2], label, words: label !== said };
      part().transitions.push(last);
      continue;
    }
    m = /^([A-Za-z_]\w*)\s*:\s*(.*)$/.exec(line);
    if (m) {
      if (!names.has(m[1])) names.set(m[1], unwrap(m[2]));
      if (implicit.get(m[1]) === null) continue;
      declare(m[1]);
      part().children.push({ name: m[1], leaf: true });
      continue;
    }
    throw new Error(`cannot read chart line: ${line}`);
  }
  if (stack.length !== 1) throw new Error("the chart has an unclosed {");
  if (root.parts.length > 1)
    throw new Error(
      "the chart has -- outside a composite; to make parallel regions, split one top-level composite with --",
    );
  (function validate(q) {
    for (const p of q.parts) {
      for (const t of p.transitions) {
        if (t.words && !t.when)
          throw new Error(`guard words on "${t.label}" need a %% when annotation; prose is not an executable guard`);
        if (t.from === "[*]" && (t.when || t.label))
          throw new Error("an initial arrow cannot have a guard or event label");
      }
      for (const c of p.children) if (!c.leaf) validate(c);
    }
  })(root);

  // Ids are global, as in Mermaid: a transition may name a state declared inside another composite.
  function region(p) {
    const out = { states: new Set(), initial: new Set(), transitions: [] };
    const info = new Map();
    const declared = new Set();
    (function collect(q) {
      for (const c of q.children) {
        if (c.name === LEGEND_ID) continue;
        declared.add(c.name);
        if (!c.leaf) for (const sub of c.parts) collect(sub);
      }
    })(p);
    const transitions = [];
    const lookup = (id) => {
      const found = info.get(id);
      if (!found) throw new Error(`transition names ${id}, which is not in this region`);
      return found;
    };
    function states(q, prefix, depth) {
      const local = q.children.filter((c) => c.name !== LEGEND_ID);
      for (const t of q.transitions) {
        for (const end of [t.from, t.to]) {
          if (end === "[*]" || declared.has(end)) continue;
          declared.add(end);
          local.push({ name: end, leaf: true });
        }
      }
      const all = [];
      for (const c of local) {
        if (info.has(c.name)) {
          if (local.indexOf(c) !== local.findIndex((x) => x.name === c.name)) continue;
          throw new Error(`state id ${c.name} is declared in two places`);
        }
        const full = `${prefix}${display(c.name)}`;
        if (c.leaf) {
          if (out.states.has(full))
            throw new Error(`two states have the caption "${full}"; give each its own caption`);
          out.states.add(full);
          info.set(c.name, { all: [full], initial: [full], depth });
          all.push(full);
          continue;
        }
        if (c.parts.length > 1)
          throw new Error(
            `${display(c.name)} splits into regions below the top level; make each region a top-level region`,
          );
        const inner = states(c.parts[0], `${full}.`, depth + 1);
        const initial = c.parts[0].transitions
          .filter((t) => t.from === "[*]")
          .flatMap((t) => lookup(t.to).initial);
        if (initial.length === 0)
          throw new Error(
            `${display(c.name)} has no [*] --> start, so a transition into it has nowhere to land`,
          );
        info.set(c.name, { all: inner, initial, depth });
        all.push(...inner);
      }
      for (const t of q.transitions) if (t.from !== "[*]" && t.to !== "[*]") transitions.push(t);
      return all;
    }
    states(p, "", 0);
    for (const t of p.transitions)
      if (t.from === "[*]") for (const s of lookup(t.to).initial) out.initial.add(s);
    transitions.sort((a, b) => lookup(b.from).depth - lookup(a.from).depth);
    const handled = new Set();
    let pending = [];
    let depth = null;
    for (const t of transitions) {
      const from = lookup(t.from);
      if (from.depth !== depth) {
        for (const h of pending) handled.add(h);
        pending = [];
        depth = from.depth;
      }
      for (const f of from.all) {
        const key = `${f}\u0000${t.label}`;
        if (t.label && handled.has(key)) continue;
        for (const to of lookup(t.to).initial)
          out.transitions.push({
            from: f,
            to,
            label: t.label,
            ...(t.when ? { when: t.when } : {}),
          });
        if (t.label) pending.push(key);
      }
    }
    return [out, info];
  }

  const owners = new Map();
  const regions = Object.create(null);
  const top = root.parts[0];
  const machine = top.children.filter((c) => c.name !== LEGEND_ID);
  const parallel =
    machine.length === 1 &&
    !machine[0].leaf &&
    machine[0].parts.length > 1 &&
    top.transitions.every((t) => t.from === "[*]");
  if (parallel) {
    for (const p of machine[0].parts) {
      const [only] = p.children;
      const holder =
        p.children.length === 1 &&
        !only.leaf &&
        p.transitions.every((t) => t.from === "[*]" && t.to === only.name)
          ? only
          : null;
      if (holder && holder.parts.length > 1)
        throw new Error(`${display(holder.name)} splits into regions below the top level`);
      const name = holder ? display(holder.name) : `Region${Object.keys(regions).length + 1}`;
      const [r, ids] = region(holder ? holder.parts[0] : p);
      if (Object.hasOwn(regions, name)) throw new Error(`duplicate region caption ${name}`);
      regions[name] = r;
      for (const [id, { all }] of ids) {
        if (owners.has(id)) throw new Error(`state id ${id} is declared in multiple regions`);
        owners.set(id, { name, all });
      }
    }
  } else {
    const [r, ids] = region(top);
    regions.Main = r;
    for (const [id, { all }] of ids) owners.set(id, { name: "Main", all });
  }
  for (const [name, r] of Object.entries(regions))
    for (const t of r.transitions) {
      if (!t.when) continue;
      if (!parallel)
        throw new Error("a %% when line guards a step on other regions, and the chart has none");
      const states = Object.create(null);
      for (const id of t.when.ids) {
        const at = owners.get(id);
        if (!at)
          throw new Error(`a %% when line names ${id}, which is not a state id in the chart`);
        if (at.name === name)
          throw new Error(
            `a %% when line names ${display(id)}, which is in the step's own region; a guard reads other regions`,
          );
        states[at.name] = [...new Set([...(states[at.name] ?? []), ...at.all])].sort();
      }
      t.guard = { not: t.when.not, states };
      delete t.when;
    }
  return { regions, nevers };
}
