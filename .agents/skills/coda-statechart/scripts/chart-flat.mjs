// Flatten a chart tree into the flat model shared by every backend:
// regions of leaf states, initial states, and labeled transitions with pre-state guards.
// Adapted from cursor/plugins pstack at eb4db2b (hierarchy flattening); see PSTACK-LICENSE.
//
// Coda adds nested parallel regions. A composite split by -- becomes one leaf of its
// own region, and each part becomes a region scoped to that leaf. A scoped region has a
// synthetic "(not in P)" state and records { scope: { region, state }, inactive, starts }.
// Its activity is derived from the parent's post-state by the backends; edges that cross
// a part's boundary are parent-region moves that carry `within` conditions (the part's
// state before the event) and `enter` targets (the part's state after entry).
import { LEGEND_ID } from "./chart-parse.mjs";
import { substitute } from "./chart-expand.mjs";

const INITIAL_LABEL = "an initial arrow cannot have a guard or event label";

export function flatten({ root, names, nevers, instances = [], ignored = [], catalog = new Map(),
  events: declared = [], groups = new Map(), strict = false, problems = [] }) {
  const display = (id) => names.get(id) ?? id;

  // Build the region for one part. `scope` is null for an always-active region, else
  // { region, state }. `outer` holds the ids of enclosing regions; edges that reach
  // them are returned as `outgoing` for the enclosing region to resolve.
  // Returns { list: [{ name, out, info }, ...descendants], info, outgoing }.
  // Ids are global, as in Mermaid: a transition may name a state declared inside another composite.
  function region(p, name, scope = null, outer = null, instance = null) {
    const out = { states: new Set(), initial: new Set(), transitions: [] };
    // A region expanded from a template instance (and every part inside it) records it.
    if (instance) out.instance = instance;
    if (scope) {
      out.scope = scope;
      out.inactive = `(not in ${scope.state})`;
      out.states.add(out.inactive);
    }
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
    const orthogonal = [];
    const lookup = (id) => {
      const found = info.get(id);
      if (!found) throw new Error(`transition names ${id}, which is not in this region`);
      return found;
    };
    function states(q, prefix, depth) {
      const local = q.children.filter((c) => c.name !== LEGEND_ID);
      for (const t of q.transitions) {
        for (const end of [t.from, t.to]) {
          if (end === "[*]" || declared.has(end) || outer?.has(end)) continue;
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
        if (c.leaf || c.parts.length > 1) {
          if (out.states.has(full))
            throw new Error(`two states have the caption "${full}"; give each its own caption`);
          out.states.add(full);
          info.set(c.name, { all: [full], initial: [full], depth });
          all.push(full);
          // A composite split by -- is one state here; its parts become scoped regions.
          if (!c.leaf) orthogonal.push({ node: c, caption: full, depth });
          continue;
        }
        if (c.parts[0].transitions.some((t) => t.from === "[*]" && t.label)) throw new Error(INITIAL_LABEL);
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

    // Parallel parts: build each scoped region, then make its ids resolvable here.
    const list = [{ name, out, info }];
    const enclosing = new Set([...(outer ?? []), ...declared]);
    for (const o of orthogonal) {
      const kids = parts(o.node.parts, `${name}/${o.caption}`, { region: name, state: o.caption }, enclosing, instance);
      for (const kid of kids) {
        list.push(...kid.list);
        for (const [id, entry] of kid.info)
          info.set(id, {
            all: [o.caption], initial: [o.caption], depth: o.depth + 1 + entry.depth, orthogonal: o,
            part: kid.list[0].name,
            inside: entry.inside
              ? { ...entry.inside, direct: false }
              : { region: kid.list[0].name, all: entry.all, initial: entry.initial, direct: true },
          });
        transitions.push(...kid.outgoing);
      }
    }
    for (const t of p.transitions) {
      if (t.from !== "[*]") continue;
      const target = lookup(t.to);
      if (target.inside)
        throw new Error(`[*] --> ${t.to} starts inside a parallel part; start at ${display(target.orthogonal.node.name)}`);
      if (!t.label) for (const s of target.initial) out.initial.add(s);
      else if (!scope) throw new Error(INITIAL_LABEL);
      else {
        // A labeled start: when the parent is entered by this event, start here instead.
        out.starts ??= new Map();
        out.starts.set(t.label, [...new Set([...(out.starts.get(t.label) ?? []), ...target.initial])]);
      }
    }
    const outgoing = [];
    const local = transitions.filter((t) => {
      if (info.has(t.from) && info.has(t.to)) return true;
      for (const end of [t.from, t.to]) if (!info.has(end) && !outer?.has(end)) lookup(end);
      outgoing.push(t);
      return false;
    });
    local.sort((a, b) => lookup(b.from).depth - lookup(a.from).depth);
    // Within a region the deepest source wins for an equally labeled step while it is
    // enabled. A source inside a parallel part wins only while that part is in it, and a
    // guarded step only while its guard holds, so shallower steps get the negation. A
    // composed step drawn from another event keeps that event's needs, which its
    // neighbours on the label don't share, so it wins only while they hold too.
    const handled = new Set(), negated = new Map(), unless = new Map();
    let pending = [], pendingNegated = [], pendingUnless = [];
    let depth = null;
    for (const t of local) {
      const from = lookup(t.from), to = lookup(t.to);
      if (from.depth !== depth) {
        for (const h of pending) handled.add(h);
        for (const [key, g] of pendingNegated) negated.set(key, [...(negated.get(key) ?? []), g]);
        for (const [key, c] of pendingUnless) unless.set(key, [...(unless.get(key) ?? []), c]);
        pending = [];
        pendingNegated = [];
        pendingUnless = [];
        depth = from.depth;
      }
      if (from.inside && to.inside && from.orthogonal === to.orthogonal) {
        if (from.part !== to.part)
          throw new Error(`${t.from} --> ${t.to} joins two parallel parts of ${display(from.orthogonal.node.name)}; a step stays in one part or leaves the composite`);
        throw new Error(`${t.from} --> ${t.to} stays inside the parallel part ${from.part}; write it inside that part`);
      }
      if (to.inside && !to.inside.direct)
        throw new Error(`${t.from} --> ${t.to} enters a state two parallel levels down; target an enclosing composite and use a labeled start`);
      const source = from.inside ? { not: false, states: { [from.inside.region]: [...from.inside.all].sort() } } : null;
      const enter = to.inside ? { [to.inside.region]: to.inside.initial } : null;
      for (const f of from.all) {
        const key = `${f}\u0000${t.label}`;
        if (t.label && handled.has(key)) continue;
        const within = [...(source ? [source] : []), ...(t.label ? negated.get(key) ?? [] : [])];
        const blockers = t.label ? unless.get(key) : null;
        for (const target of to.initial)
          out.transitions.push({
            from: f,
            to: target,
            label: t.label,
            ...(t.when ? { when: t.when } : {}),
            ...(t.needsIds ? { needsIds: t.needsIds } : {}),
            ...(t.ownerIds ? { ownerIds: t.ownerIds } : {}),
            ...(within.length ? { within } : {}),
            ...(blockers ? { unless: blockers } : {}),
            ...(t.refuseIds ? { refuseIds: t.refuseIds } : {}),
            ...(enter ? { enter } : {}),
            ...(t.optional ? { optional: true } : {}),
            ...(t.intent ? { intent: t.intent } : {}),
          });
        const own = t.shared ? { needsIds: t.needsIds, ownerIds: t.ownerIds } : {};
        if (t.label && (t.when || own.needsIds || own.ownerIds || t.refuseIds))
          pendingUnless.push([key, { within: source, when: t.when, ...own, refuseIds: t.refuseIds }]);
        else if (t.label && source) pendingNegated.push([key, { ...source, not: true }]);
        else if (t.label) pending.push(key);
      }
    }
    return { list, info, outgoing };
  }

  // Regions for the parts of a composite split by --. A part holding exactly one composite
  // (entered only by [*]) takes that composite's caption; a holder that itself splits
  // contributes its parts under its caption.
  function parts(list, prefix, scope, outer, instance) {
    const results = [];
    list.forEach((p, i) => {
      const [only] = p.children;
      const holder =
        p.children.length === 1 &&
        !only.leaf &&
        p.transitions.every((t) => t.from === "[*]" && t.to === only.name)
          ? only
          : null;
      const name = `${prefix}/${holder ? display(holder.name) : `Region${i + 1}`}`;
      const from = holder?.instance ?? instance;
      if (holder && holder.parts.length > 1) results.push(...parts(holder.parts, name, scope, outer, from));
      else results.push(region(holder ? holder.parts[0] : p, name, scope, outer, from));
    });
    return results;
  }

  const owners = new Map();
  const regions = Object.create(null);
  const register = ({ list }) => {
    for (const { name, out, info } of list) {
      if (Object.hasOwn(regions, name)) throw new Error(`duplicate region caption ${name}`);
      regions[name] = out;
      for (const [id, { all, inside }] of info) {
        if (inside) continue;
        if (owners.has(id)) throw new Error(`state id ${id} is declared in multiple regions`);
        owners.set(id, { name, all });
      }
    }
  };
  const top = root.parts[0];
  const machine = top.children.filter((c) => c.name !== LEGEND_ID);
  const parallel =
    machine.length === 1 &&
    !machine[0].leaf &&
    machine[0].parts.length > 1 &&
    top.transitions.every((t) => t.from === "[*]");
  if (parallel) {
    machine[0].parts.forEach((p, index) => {
      const [only] = p.children;
      const holder =
        p.children.length === 1 &&
        !only.leaf &&
        p.transitions.every((t) => t.from === "[*]" && t.to === only.name)
          ? only
          : null;
      const name = holder ? display(holder.name) : `Region${index + 1}`;
      // A top-level region is always active, so the parts of a splitting holder are too.
      if (holder && holder.parts.length > 1) for (const r of parts(holder.parts, name, null, null, holder.instance)) register(r);
      else register(region(holder ? holder.parts[0] : p, name, null, null, holder?.instance));
    });
  } else {
    const { list } = region(top, "Main");
    const [{ out, info }] = list;
    regions.Main = out;
    for (const [id, { all, inside }] of info) if (!inside) owners.set(id, { name: "Main", all });
    if (list.length > 1) register({ list: list.slice(1) });
  }
  // An event's needs reads the pre-state like %% when, and may name any region.
  const needs = ({ not, ids, reason }, label) => {
    const states = Object.create(null);
    for (const id of ids) {
      const at = owners.get(id);
      if (!at) throw new Error(`the event "${label}" needs ${id}, which is not a state id in the chart`);
      states[at.name] = [...new Set([...(states[at.name] ?? []), ...at.all])].sort();
    }
    return { not, states, ...(reason ? { reason } : {}) };
  };
  const single = Object.keys(regions).length === 1;
  const guard = (when, name) => {
    if (single)
      throw new Error("a %% when line guards a step on other regions, and the chart has none");
    const states = Object.create(null);
    for (const id of when.ids) {
      const at = owners.get(id);
      if (!at)
        throw new Error(`a %% when line names ${id}, which is not a state id in the chart`);
      if (at.name === name)
        throw new Error(
          `a %% when line names ${display(id)}, which is in the step's own region; a guard reads other regions`,
        );
      states[at.name] = [...new Set([...(states[at.name] ?? []), ...at.all])].sort();
    }
    return { not: when.not, states, ...(when.reason ? { reason: when.reason } : {}) };
  };
  for (const [name, r] of Object.entries(regions))
    for (const t of r.transitions) {
      if (t.needsIds) {
        t.needs = needs(t.needsIds, t.label);
        delete t.needsIds;
      }
      // A composed chart's step on another chart's event also waits for that event's own needs.
      if (t.ownerIds) {
        t.within = [...(t.within ?? []), { ...needs(t.ownerIds, t.label), owner: true }];
        delete t.ownerIds;
      }
      // A shallower step yields to each deeper one exactly while that one is enabled: its part,
      // guard and needs hold, and a refusal's command needs fail somewhere (one blocker per region).
      if (t.unless) t.unless = t.unless.flatMap((c) => {
        const all = [...(c.within ? [c.within] : []),
          ...(c.when ? [guard(c.when, name)] : []),
          ...(c.needsIds ? [needs(c.needsIds, t.label)] : []),
          ...(c.ownerIds ? [{ ...needs(c.ownerIds, t.label), owner: true }] : [])];
        if (!c.refuseIds) return [all];
        return Object.entries(needs(c.refuseIds, t.label).states)
          .map(([region, states]) => [...all, { not: true, states: { [region]: states } }]);
      });
      // A command is refused exactly while its needs fail.
      if (t.refuseIds) {
        t.unless = [...(t.unless ?? []), [needs(t.refuseIds, t.label)]];
        delete t.refuseIds;
      }
      if (!t.when) continue;
      t.guard = guard(t.when, name);
      delete t.when;
    }
  // Declared events by concrete label: who causes each, what it needs, and its intent.
  const events = new Map();
  for (const [label, { decl, values }] of catalog)
    events.set(label, {
      cause: substitute(decl.cause, values), intent: substitute(decl.intent, values),
      needs: decl.needs ? needs({ not: decl.needs.not, ids: decl.needs.ids.map((id) => substitute(id, values)),
        reason: decl.needs.reason && substitute(decl.needs.reason, values) }, label) : null,
      ...(decl.label !== label ? { family: decl.label } : {}),
      ...(decl.accepts ? { accepts: decl.accepts } : {}),
    });
  return { regions, nevers, owners, instances, ignored, events, declared, groups, strict, problems };
}

// Words for a resolved guard, in the chart's captions.
export function guardWords(g) {
  if (!g) return "";
  const parts = Object.entries(g.states).map(([region, states]) => `${region} is ${states.join(" or ")}`);
  return `${g.not ? "unless" : "when"} ${parts.join("; ")}`;
}

// The reason a limit states, as a suffix: " | reason: TEXT", or nothing.
export const reasonSuffix = (limit) => (limit?.reason ? ` | reason: ${limit.reason}` : "");

// A limit in words with the reason it states: the one wording query and compare read.
export const limitWords = (g) => `${guardWords(g)}${reasonSuffix(g)}`;

// Print the flat model as plain Mermaid: one holder composite per region, leaf states
// only, concrete event labels, guards as %% when lines and needs as %% event lines.
// Without parallel parts or guarded steps that shadow shallower ones, it reads back to
// the same flat model. Scoped activity, part conditions, blocking conditions and
// rules are listed as comments: they are not plain-Mermaid behavior.
export function printFlat(flat, rules = []) {
  const quote = (s) => s.replace(/"/g, "#quot;");
  const words = (s) => s.replace(/\[/g, "(").replace(/\]/g, ")");
  const entries = Object.entries(flat.regions);
  const ids = new Map();
  entries.forEach(([name, r], ri) => [...r.states].forEach((s, si) => ids.set(`${name}\u0000${s}`, `r${ri}s${si}`)));
  const id = (region, state) => ids.get(`${region}\u0000${state}`);
  const idsOf = (g) => Object.entries(g.states).flatMap(([region, states]) => states.map((s) => id(region, s)));
  const edges = entries.reduce((n, [, r]) => n + r.transitions.length, 0);
  const out = ["stateDiagram-v2",
    `%% Flat expansion: ${entries.length} regions, ${edges} edges. Generated by statechart.mjs flat for inspection.`];
  if (flat.strict) out.push("%% events declared");
  const labels = new Set(entries.flatMap(([, r]) => r.transitions.map((t) => t.label)));
  for (const [label, e] of flat.events ?? []) {
    if (!labels.has(label)) continue;
    const needs = e.needs ? ` | needs ${e.needs.not ? "not " : ""}${idsOf(e.needs).join(", ")}${reasonSuffix(e.needs)}` : "";
    out.push(`%% event ${label} | by ${e.cause}${needs} | ${e.intent}`);
  }
  const single = entries.length === 1 && entries[0][0] === "Main";
  const pad = single ? "" : "    ";
  if (!single) out.push('state "Chart" as chart {');
  entries.forEach(([name, r], ri) => {
    if (!single) {
      if (ri) out.push("  --");
      out.push(`  state "${quote(name)}" as r${ri} {`);
    }
    if (r.instance) out.push(`${pad}%% from template ${r.instance.template}, instance ${r.instance.key}`);
    if (r.scope) {
      out.push(`${pad}%% scoped: active only while ${r.scope.region} is in ${r.scope.state}; ${r.inactive} otherwise`);
      for (const [label, states] of r.starts ?? []) out.push(`${pad}%% labeled start: entered on "${label}" at ${states.join(" or ")}`);
    }
    for (const s of r.initial) out.push(`${pad}[*] --> ${id(name, s)}`);
    for (const s of r.states) if (!r.scope || s !== r.inactive) out.push(`${pad}state "${quote(s)}" as ${id(name, s)}`);
    for (const t of r.transitions) {
      const guard = t.guard ? ` [${words(guardWords(t.guard))}]` : "";
      const effect = t.intent ? ` / ${t.intent}` : "";
      out.push(`${pad}${id(name, t.from)} --> ${id(name, t.to)}${t.label || guard || effect ? ` : ${t.label}${guard}${effect}` : ""}`);
      if (t.guard) out.push(`${pad}%% when ${t.guard.not ? "not " : ""}${idsOf(t.guard).join(", ")}${reasonSuffix(t.guard)}`);
      for (const w of t.within ?? []) out.push(`${pad}%% within: ${guardWords(w)}`);
      for (const c of t.unless ?? []) out.push(`${pad}%% blocked while: ${c.map(guardWords).join(" and ")}`);
      if (t.enter) for (const [part, states] of Object.entries(t.enter)) out.push(`${pad}%% enters ${part} at ${states.join(" or ")}`);
    }
    if (!single) out.push("  }");
  });
  if (!single) out.push("}", "[*] --> chart");
  for (const rule of rules) {
    const what = rule.kind === "scope" ? rule.sentence
      : `${rule.parts.map((p) => `${entries[p.ri][0]}: ${p.states.join(" or ")}`).join(" & ")} -- ${rule.sentence}`;
    out.push(`%% rule ${rule.name}: ${what}${reasonSuffix(rule)}`);
  }
  return out.join("\n") + "\n";
}
