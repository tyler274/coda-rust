// Search the exact flattened regions, grouped events and resolved rules used by the Verus backend.
// Stuttering and duplicate self-loops cannot shorten a safety counterexample.

// A "may" effect worded alike in two or more regions on one event is one choice: in a step,
// either every region with such an enabled edge takes it, or none does. An effect only one
// region names stays that region's own choice.
export function sharedEffects(action) {
  if (action.shared) return action.shared;
  const owners = new Map();
  for (const [ri, moves] of action.moves) for (const m of moves) if (m.optional) {
    if (!owners.has(m.intent)) owners.set(m.intent, new Set());
    owners.get(m.intent).add(ri);
  }
  return (action.shared = new Set([...owners].filter(([, rs]) => rs.size > 1).map(([intent]) => intent)));
}

// The step semantics, shared by `trace` (all regions) and `rules` (a cone of regions).
// `pre(ri)` reads any region's pre-state, so guards may read regions outside `scope`.
// Scoped regions are resolved after their parent, from the parent's chosen option:
// "move" (it took a move into the scoping state) and "enter" (it was itself entered)
// make the part (re)enter; "stay" lets the part react ordinarily; elsewhere it is off.
export function stepper(ir) {
  const { regions } = ir;
  const byCaption = new Map(regions.map((r, i) => [r.caption, i]));
  const kids = new Set(regions.filter((r) => r.scope).map((r) => r.parent));
  const holds = (g, pre) => {
    if (!g) return true;
    const hits = Object.entries(g.states).map(([region, states]) => states.includes(pre(byCaption.get(region))));
    return g.not ? !hits.some(Boolean) : hits.every(Boolean);
  };
  const enabled = (m, ri, pre) => m.from === pre(ri) && holds(m.guard, pre) && holds(m.needs, pre)
    && (m.within ?? []).every((g) => holds(g, pre))
    && (m.unless ?? []).every((c) => !c.every((g) => holds(g, pre)));
  // A chosen shared effect is taken wherever it is enabled; an unchosen one is absent. A
  // region with neither a chosen effect nor a required move enabled may stay.
  const ordinary = (ri, action, pre, choice) => {
    const open = (action.moves.get(ri) ?? []).filter((m) => enabled(m, ri, pre));
    if (!open.length) return [{ value: pre(ri), how: "stay" }];
    const shared = sharedEffects(action);
    const chosen = open.filter((m) => m.optional && shared.has(m.intent) && choice.has(m.intent));
    const own = open.filter((m) => m.optional && !shared.has(m.intent));
    const required = open.filter((m) => !m.optional);
    const options = [], seen = new Set();
    const add = (o) => {
      const key = kids.has(ri) ? `${o.how}\u0000${o.value}\u0000${JSON.stringify(o.move?.enter ?? null)}` : o.value;
      if (!seen.has(key)) { seen.add(key); options.push(o); }
    };
    for (const m of [...chosen, ...own]) add({ value: m.to, how: "move", move: m });
    if (!chosen.length) {
      for (const m of required) add({ value: m.to, how: "move", move: m });
      if (!required.length) add({ value: pre(ri), how: "stay" });
    }
    return options;
  };
  const options = (ri, action, pre, parent, choice) => {
    const r = regions[ri];
    if (!r.scope) return ordinary(ri, action, pre, choice);
    if (parent.value !== r.scope.state) return [{ value: r.inactive, how: "off" }];
    if (parent.how === "stay") return ordinary(ri, action, pre, choice);
    const lands = (parent.how === "move" && parent.move.enter?.[r.caption]) || r.starts?.get(action.label) || [...r.initial];
    return lands.map((value) => ({ value, how: "enter" }));
  };
  // Every post-state of `action` for the regions in `scope` (ascending indices), or none
  // when no region in scope has an enabled move.
  function* post(action, scope, pre) {
    if (!scope.some((ri) => (action.moves.get(ri) ?? []).some((m) => enabled(m, ri, pre)))) return;
    const position = new Map(scope.map((ri, i) => [ri, i]));
    const chosen = new Array(scope.length);
    // Only shared effects enabled somewhere in scope change what scope can do.
    const live = [...sharedEffects(action)].filter((intent) => scope.some((ri) =>
      (action.moves.get(ri) ?? []).some((m) => m.optional && m.intent === intent && enabled(m, ri, pre))));
    for (let bits = 0; bits < 2 ** live.length; bits++) {
      const choice = new Set(live.filter((_, i) => bits & (2 ** i)));
      const memo = scope.map(() => new Map());
      function* fill(i) {
        if (i === scope.length) { yield chosen.map((o) => o.value); return; }
        const ri = scope[i], r = regions[ri];
        const parent = r.scope ? chosen[position.get(r.parent)] : null;
        let list = memo[i].get(parent);
        if (!list) memo[i].set(parent, list = options(ri, action, pre, parent, choice));
        for (const o of list) { chosen[i] = o; yield* fill(i + 1); }
      }
      yield* fill(0);
    }
  }
  // Initial states for `scope`; a scoped region starts only where its parent does.
  function* starts(scope) {
    const position = new Map(scope.map((ri, i) => [ri, i]));
    const chosen = new Array(scope.length);
    function* fill(i) {
      if (i === scope.length) { yield [...chosen]; return; }
      const r = regions[scope[i]];
      const values = !r.scope ? [...r.initial]
        : chosen[position.get(r.parent)] === r.scope.state ? [...r.initial] : [r.inactive];
      for (const v of values) { chosen[i] = v; yield* fill(i + 1); }
    }
    yield* fill(0);
  }
  return { post, starts, holds, enabled };
}

export function search(ir, limit) {
  const { regions, actions, rules } = ir;
  if (!rules.length) return { status: "no-rules", visited: 0 };
  const { post, starts } = stepper(ir);
  const scope = regions.map((_, i) => i);
  const seen = new Map(), queue = [];
  const key = (state) => JSON.stringify(state);
  const add = (state, parent, action) => {
    const k = key(state);
    if (seen.has(k)) return true;
    if (queue.length === limit) return false;
    seen.set(k, queue.length);
    queue.push({ state, parent, action });
    return true;
  };
  const violation = (state) => rules.find((r) => r.violated((ri) => state[ri]));
  const found = (index, rule) => {
    const steps = [];
    for (let i = index; i !== null; i = queue[i].parent) steps.push(queue[i]);
    return { status: "violation", visited: queue.length, rule, steps: steps.reverse() };
  };
  for (const state of starts(scope)) {
    if (!add(state, null, null)) return { status: "incomplete", visited: queue.length };
    const bad = violation(state);
    if (bad) return found(seen.get(key(state)), bad);
  }
  for (let index = 0; index < queue.length; index++) {
    const state = queue[index].state;
    const pre = (ri) => state[ri];
    for (const action of actions) {
      for (const next of post(action, scope, pre)) {
        if (!add(next, index, action)) return { status: "incomplete", visited: queue.length };
        const bad = violation(next);
        if (bad) return found(seen.get(key(next)), bad);
      }
    }
  }
  return { status: "exhausted", visited: queue.length };
}

// Print a search result; returns the process exit code.
export function printSearch(result, ir) {
  if (result.status === "no-rules") {
    console.log("No chart safety rules; no safety claim checked. Use linked data proofs.");
    return 0;
  }
  if (result.status === "incomplete") {
    console.log(`INCOMPLETE after ${result.visited} states: the limit prevented exhaustive search. No pass established.`);
    return 3;
  }
  if (result.status === "exhausted") {
    console.log(`Exhausted ${result.visited} reachable chart states; no chart-rule violation. Data, progress and implementation remain separate.`);
    return 0;
  }
  const stateWords = (s) => s.map((v, i) => `${ir.regions[i].caption}: ${v}`).join("; ");
  const actionWords = (step) => step.action?.label || (step.action ? "unlabeled transition" : "Start");
  console.log(`VIOLATION ${result.rule.name}: ${result.rule.sentence}`);
  console.log(`Shortest path: ${result.steps.length - 1} events.`);
  result.steps.forEach((s, i) => console.log(`${i + 1}. ${actionWords(s)} → ${stateWords(s.state)}`));
  // Encode chart text as labels, never Mermaid syntax.
  const escape = (s) => s.replace(/&/g, "&amp;").replace(/"/g, "&quot;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/[\r\n]/g, " ");
  console.log("```mermaid\nflowchart LR");
  result.steps.forEach((s, i) => {
    console.log(`  S${i}["${i + 1}. ${escape(stateWords(s.state))}"]`);
    if (i) console.log(`  S${i - 1} -->|"${i}. ${escape(actionWords(s))}"| S${i}`);
  });
  console.log("```");
  return 1;
}
