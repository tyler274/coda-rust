// Check each rule on its cone: the few regions the rule can depend on, searched with the
// same step semantics as `trace` (chart-search.mjs). Large charts whose full product is
// out of reach for `trace` can still have each rule settled on a small cone.
//
// Soundness. A rule's cone starts as its own regions and their enclosing regions. A
// region the cone's steps read (guards, needs, part conditions) but leaves out is free:
// at every step it may hold any of its values, one value per region per step read alike
// by every guard (a part whose composite is in the cone stays consistent with it). Every
// step of the whole chart restricted to the cone is then a cone step or leaves the cone
// unchanged, so every reachable chart state projects to a reachable cone state: HOLDS
// is sound. A violation path that never read a free region is real: each of its steps
// reads only the cone, and every other region always has an option (a move, or
// staying), so each step extends to a whole-chart step. Otherwise the free regions the
// path read join the cone (with their enclosing regions) and the search repeats, up to
// a size limit; a violation that still reads a free region is INCONCLUSIVE.
import { stepper } from "./chart-search.mjs";

const conditions = (m) => [m.guard, m.needs, ...(m.within ?? []), ...(m.unless ?? []).flat()].filter(Boolean);

export function ruleRegions(ir, rule) {
  return rule.kind === "scope" ? [rule.ri, ir.regions[rule.ri].parent] : rule.parts.map((p) => p.ri);
}

// Regions each region's steps read, and its enclosing region.
function dependencies(ir) {
  const byCaption = new Map(ir.regions.map((r, i) => [r.caption, i]));
  return ir.regions.map((r, ri) => {
    const found = new Set(r.scope ? [r.parent] : []);
    for (const t of r.transitions) for (const c of conditions(t)) for (const caption of Object.keys(c.states)) found.add(byCaption.get(caption));
    found.delete(ri);
    return found;
  });
}

// Add regions with their enclosing regions while the cone stays within max.
function grow(ir, members, candidates, max) {
  let added = 0;
  for (const ri of candidates) {
    const lineage = [];
    for (let x = ri; x !== undefined && !members.has(x); x = ir.regions[x].scope ? ir.regions[x].parent : undefined) lineage.push(x);
    if (!lineage.length || members.size + lineage.length > max) continue;
    for (const x of lineage) members.add(x);
    added += lineage.length;
  }
  return added;
}

export function cone(ir, members, deps = dependencies(ir)) {
  const free = new Set();
  for (const ri of members) for (const d of deps[ri]) if (!members.has(d)) free.add(d);
  return { regions: [...members].sort((a, b) => a - b), free: [...free].sort((a, b) => a - b) };
}

// Breadth-first search of one cone. Free regions take one representative per class of
// values that the cone's conditions on that event cannot tell apart.
export function checkCone(ir, rule, { regions: scope, free }, limit) {
  const { regions, actions } = ir;
  const { post, starts } = stepper(ir);
  const position = new Map(scope.map((ri, i) => [ri, i]));
  const byCaption = new Map(regions.map((r, i) => [r.caption, i]));
  const freeSet = new Set(free);
  const plan = [];
  for (const action of actions) {
    const moves = scope.flatMap((ri) => action.moves.get(ri) ?? []);
    if (!moves.length) continue;
    const sets = new Map(); // free region -> state lists the conditions name
    for (const m of moves) for (const c of conditions(m))
      for (const [caption, states] of Object.entries(c.states)) {
        const ri = byCaption.get(caption);
        if (freeSet.has(ri)) sets.set(ri, [...(sets.get(ri) ?? []), states]);
      }
    const choices = [...sets].map(([ri, lists]) => {
      const r = regions[ri];
      const classes = new Map();
      for (const v of r.values) {
        const signature = lists.map((l) => (l.includes(v) ? 1 : 0)).join("") + (r.scope && v === r.inactive ? "i" : "");
        if (!classes.has(signature)) classes.set(signature, v);
      }
      const bound = r.scope && position.has(r.parent) ? r : null;
      return { ri, values: [...classes.values()], bound };
    });
    plan.push({ action, choices });
  }
  const valuations = function* (choices, state) {
    const chosen = new Map();
    function* fill(i) {
      if (i === choices.length) { yield chosen; return; }
      const { ri, values, bound } = choices[i];
      for (const v of values) {
        if (bound && (v === bound.inactive) !== (state[position.get(bound.parent)] !== bound.scope.state)) continue;
        chosen.set(ri, v);
        yield* fill(i + 1);
      }
    }
    yield* fill(0);
  };
  const seen = new Map(), queue = [];
  const key = (s) => s.join("\u0000");
  const bad = (s) => rule.violated((ri) => s[position.get(ri)]);
  const found = (index) => {
    const steps = [];
    for (let i = index; i !== null; i = queue[i].parent) steps.push(queue[i]);
    return { status: "violation", visited: queue.length, steps: steps.reverse() };
  };
  const add = (state, parent, action, outside) => {
    const k = key(state);
    if (seen.has(k)) return null;
    if (queue.length === limit) return { status: "incomplete", visited: queue.length };
    seen.set(k, queue.length);
    queue.push({ state, parent, action, outside });
    return bad(state) ? found(queue.length - 1) : null;
  };
  for (const state of starts(scope)) {
    const stop = add(state, null, null, null);
    if (stop) return stop;
  }
  for (let index = 0; index < queue.length; index++) {
    const { state } = queue[index];
    for (const { action, choices } of plan)
      for (const values of valuations(choices, state)) {
        const pre = (ri) => (position.has(ri) ? state[position.get(ri)] : values.get(ri));
        for (const next of post(action, scope, pre)) {
          const stop = add(next, index, action, values.size ? new Map(values) : null);
          if (stop) return stop;
        }
      }
  }
  return { status: "holds", visited: queue.length };
}

export function checkRules(ir, { rule: only, maxCone = 12, maxStates = 100000 } = {}) {
  const rules = only === undefined ? ir.rules : ir.rules.filter((r) => r.name === only);
  if (only !== undefined && !rules.length) throw new Error(`no rule is named ${only}; the chart's rules are: ${ir.rules.map((r) => r.name).join(", ")}`);
  const deps = dependencies(ir);
  return rules.map((rule) => {
    const started = performance.now();
    const members = new Set();
    grow(ir, members, ruleRegions(ir, rule), Infinity);
    for (let rounds = 1; ; rounds++) {
      const c = cone(ir, members, deps);
      const result = checkCone(ir, rule, c, maxStates);
      const read = result.status === "violation"
        ? [...new Set(result.steps.flatMap((s) => (s.outside ? [...s.outside.keys()] : [])))] : [];
      const done = (status) => ({ rule, cone: c, rounds, ...result, status, exact: !c.free.length, ms: performance.now() - started });
      if (result.status !== "violation") return done(result.status);
      if (!read.length) return done("violation");
      if (!grow(ir, members, read, maxCone)) return done("inconclusive");
    }
  });
}

// Print the results; returns the exit code: 1 for any exact violation, else 3 when any
// rule is inconclusive or incomplete, else 0.
export function printRules(results, ir) {
  if (!ir.rules.length) {
    console.log("No chart safety rules; no safety claim checked.");
    return 0;
  }
  const words = (scope, step) => {
    const inside = scope.map((ri, i) => `${ir.regions[ri].caption}: ${step.state[i]}`);
    const outside = step.outside ? [...step.outside].map(([ri, v]) => `${ir.regions[ri].caption} (free): ${v}`) : [];
    return [...inside, ...outside].join("; ");
  };
  const path = (r) => r.steps.forEach((s, i) => console.log(`  ${i + 1}. ${s.action ? s.action.label || "unlabeled transition" : "Start"} → ${words(r.cone.regions, s)}`));
  const counts = { holds: 0, violation: 0, inconclusive: 0, incomplete: 0 };
  for (const r of results) {
    counts[r.status]++;
    const size = `cone of ${r.cone.regions.length} region${r.cone.regions.length === 1 ? "" : "s"}`;
    const shape = r.exact ? "nothing free" : `${r.cone.free.length} free`;
    const head = `${r.rule.name} (${size}, ${shape}; ${r.visited} states, ${r.rounds} round${r.rounds === 1 ? "" : "s"}, ${Math.round(r.ms)} ms)`;
    if (r.status === "holds") console.log(`HOLDS ${head}`);
    else if (r.status === "incomplete") console.log(`INCOMPLETE ${head}: stopped at --max-states; no pass established.`);
    else if (r.status === "violation") {
      console.log(`VIOLATION ${head}: ${r.rule.sentence}`);
      path(r);
    } else {
      console.log(`INCONCLUSIVE ${head}: a violation that reads free regions remains at --max-cone; raise it or run trace. Path with free values:`);
      path(r);
    }
  }
  console.log(`${counts.holds} hold, ${counts.violation} violated, ${counts.inconclusive} inconclusive, ${counts.incomplete} incomplete.`);
  return counts.violation ? 1 : counts.inconclusive || counts.incomplete ? 3 : 0;
}
