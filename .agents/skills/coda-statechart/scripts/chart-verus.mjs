// Verus backend adapted from pstack's chart-to-tla.mjs at eb4db2b.
// See verification/PSTACK-LICENSE. The chart owns generated behavior; proofs are editable.
import fs from "node:fs";
import path from "node:path";
import { createHash } from "node:crypto";
import { parseMermaid } from "./mermaid.mjs";
import { sharedEffects } from "./chart-search.mjs";
import { COMPOSES } from "./chart-compose.mjs";

export const hash = (text) => createHash("sha256").update(text).digest("hex");
// rustc parses a flat chain of N operators as a tree N deep; long chains are grouped into
// a balanced tree so a large chart cannot overflow the compiler's stack.
const join = (xs, operator, empty) => {
  if (xs.length === 0) return empty;
  if (xs.length === 1) return xs[0];
  if (xs.length <= 16) return xs.map((x) => `(${x})`).join(` ${operator} `);
  const half = Math.ceil(xs.length / 2);
  return `(${join(xs.slice(0, half), operator, empty)}) ${operator} (${join(xs.slice(half), operator, empty)})`;
};
const and = (xs) => join(xs.filter((x) => x !== "true"), "&&", "true");
const or = (xs) => join(xs, "||", "false");
// Simplifying forms, used only by constructs that older charts cannot contain.
const all = (xs) => xs.includes("false") ? "false" : and(xs);
const any = (xs) => xs.includes("true") ? "true" : or(xs.filter((x) => x !== "false"));
const comment = (text) => text.replace(/[\r\n\u2028\u2029]/g, " ");

// Like pstack's backend, derive reviewable identifiers from the chart's words.
// Prefixes avoid Rust keywords; disambiguate captions that normalize alike.
function ident(words, prefix, taken, snake = false) {
  const parts = words.normalize("NFKD").replace(/([a-z0-9])([A-Z])/g, "$1 $2")
    .replace(/[^A-Za-z0-9]+/g, " ").trim().split(/\s+/).filter(Boolean);
  const stem = snake ? parts.map((p) => p.toLowerCase()).join("_")
    : parts.map((p) => p[0].toUpperCase() + p.slice(1)).join("");
  const base = prefix + (stem || "Unnamed");
  let name = base;
  for (let n = 2; taken.has(name); n++) name = `${base}_${n}`;
  taken.add(name);
  return name;
}

const definition = (name, parameters, clauses) =>
  `pub open spec fn ${name}(${parameters}) -> bool {\n    ${clauses.length <= 16
    ? clauses.map((c) => `(${c})`).join("\n    && ") : join(clauses, "&&", "true")}\n}`;

const SEMANTICS = "pstack broadcast events; pre-state guards; nondeterministic choices; stuttering allowed";

export function compile(text, module, options = {}) {
  const parsed = parseMermaid(text, options);
  const types = new Set(), fields = new Set(), functions = new Set();
  const regions = Object.entries(parsed.regions).map(([caption, r]) => {
    if (!r.states.size || !r.initial.size) throw new Error(`region ${caption} needs states and [*] --> start`);
    const variants = new Map(), names = new Set();
    for (const state of r.states) variants.set(state, ident(state, "State", names));
    return { caption, field: ident(caption, "region_", fields, true),
      type: ident(caption, "Region", types), ...r, values: [...r.states], variants };
  });
  if (!regions.length) throw new Error("the chart has no regions");
  const index = new Map(regions.map((r, i) => [r.caption, i]));
  // A scoped region's parent always precedes it, so parents are resolved first.
  for (const r of regions) if (r.scope) r.parent = index.get(r.scope.region);
  const at = (r, state, variable = "s") => `${variable}.${r.field} == ${r.type}::${r.variants.get(state)}`;
  // Named predicates keep the model small enough for rustc: each long formula is written
  // once and called wherever it recurs. Identical formulas share one definition.
  const helpers = [], named = new Map();
  const helper = (words, parameters, body) => {
    if (body.length <= 80) return body;
    const key = `${parameters}|${body}`;
    if (!named.has(key)) {
      const name = ident(words, "", functions, true);
      helpers.push(definition(name, parameters, [body]));
      named.set(key, `${name}(${parameters.split(", ").map((p) => p.split(":")[0]).join(", ")})`);
    }
    return named.get(key);
  };
  const guard = (g) => {
    if (!g) return "true";
    const hits = Object.entries(g.states).map(([name, states]) => {
      const r = regions[index.get(name)];
      return helper(`in ${r.field}`, "s: State", or(states.map((state) => at(r, state))));
    });
    return g.not ? `!(${or(hits)})` : and(hits);
  };
  const actions = [];
  const byLabel = new Map();
  const same = (a, b) => JSON.stringify(a ?? null) === JSON.stringify(b ?? null);
  for (const [ri, r] of regions.entries()) for (const t of r.transitions) {
    let action = t.label ? byLabel.get(t.label) : null;
    if (!action) {
      action = { label: t.label, function: ident(t.label || `${t.from} to ${t.to}`, "event_", functions, true), moves: new Map() };
      actions.push(action);
      if (t.label) byLabel.set(t.label, action);
    }
    if (!action.moves.has(ri)) action.moves.set(ri, []);
    const move = { ...t, enabled: helper(`${action.function} when`, "s: State", and([at(r, t.from), guard(t.guard), guard(t.needs),
      ...(t.within ?? []).map(guard), ...(t.unless ?? []).map((c) => `!(${and(c.map(guard))})`)])) };
    const moves = action.moves.get(ri);
    if (!moves.some((m) => m.from === move.from && m.to === move.to && m.enabled === move.enabled
      && Boolean(m.optional) === Boolean(move.optional) && same(m.enter, move.enter))) moves.push(move);
  }
  const ruleNames = new Set();
  const rules = parsed.nevers.map((n) => {
    if (ruleNames.has(n.name)) throw new Error(`duplicate never name ${n.name}`);
    ruleNames.add(n.name);
    const parts = n.states.map((caption, ci) => {
      // A composed chart's rule names states of that chart only.
      const scope = n.scopes?.[ci];
      const within = (r) => !scope || r.caption === scope || r.caption.startsWith(`${scope}/`);
      const find = (words) => regions.flatMap((r, ri) => {
        if (!within(r)) return [];
        const states = r.states.has(words) ? [words] : r.values.filter((s) => s.startsWith(`${words}.`));
        return states.length ? [{ ri, states }] : [];
      });
      let hits = find(caption);
      // Templates: "Caption@KEY" names a state of that instance. A rule written inside a
      // template that names one of the template's states means its own instance's state.
      const named = /^(.*)@([A-Za-z0-9_]+)$/.exec(caption);
      if (!hits.length && named) hits = find(named[1]).filter((h) => regions[h.ri].instance?.key === named[2]);
      else if (n.home && hits.some((h) => regions[h.ri].instance?.template === n.home.template))
        hits = hits.filter((h) => regions[h.ri].instance?.template === n.home.template && regions[h.ri].instance.key === n.home.key);
      if (hits.length !== 1) throw new Error(`never ${n.name}: "${caption}" must identify one state or composite in one region`);
      return hits[0];
    });
    if (new Set(parts.map((p) => p.ri)).size !== parts.length)
      throw new Error(`never ${n.name} names two states of one region`);
    return { ...n, function: ident(n.name, "rule_", functions, true), parts,
      violated: (value) => parts.every((p) => p.states.includes(value(p.ri))) };
  });
  // Each scoped region is inactive exactly when its parent is not in the scoping state.
  for (const [ri, r] of regions.entries()) if (r.scope)
    rules.push({ name: `${r.caption} scope`, kind: "scope", ri,
      sentence: `${r.caption} is ${r.inactive} exactly when ${r.scope.region} is not in ${r.scope.state}.`,
      function: ident(`scope ${r.caption}`, "rule_", functions, true),
      violated: (value) => (value(ri) === r.inactive) === (value(r.parent) === r.scope.state) });

  const self = (r) => `t.${r.field} == s.${r.field}`;
  // A region's moves under one assignment `chosen` of the event's shared effects (see
  // sharedEffects): a chosen one is taken wherever enabled, an unchosen one is absent, and a
  // required move yields to a chosen effect.
  const split = (moves, action, chosen) => {
    const shared = sharedEffects(action);
    const open = moves.filter((m) => !m.optional || !shared.has(m.intent) || chosen.has(m.intent));
    const taken = moves.filter((m) => m.optional && shared.has(m.intent) && chosen.has(m.intent));
    const own = moves.filter((m) => m.optional && !shared.has(m.intent));
    const required = moves.filter((m) => !m.optional);
    const none = (ms) => (ms.length ? `!(${or(ms.map((m) => m.enabled))})` : "true");
    // Keeping the state for an enabled own "may" needs no required move or chosen effect enabled.
    const keep = (m) => and([m.enabled, none(required), none(taken)]);
    return { open, own, keep, guard: (m) => (m.optional ? "true" : none(taken)), none };
  };
  const ordinary = (r, ri, action, chosen) => {
    const moves = action.moves.get(ri);
    if (!moves) return self(r);
    const { open, own, keep, guard, none } = split(moves, action, chosen);
    const choices = open.map((m) => and([m.enabled, guard(m), at(r, m.to, "t")]));
    // As in chart-to-tla: each enabled region participates in a shared event;
    // a region with no enabled move stays. All guards read s, never t.
    if (action.moves.size > 1) choices.push(and([none(open), self(r)]));
    // An optional ("may") reaction also lets its enabled region keep its state.
    if (own.length) choices.push(and([or(own.map(keep)), self(r)]));
    return or(choices);
  };
  // Scoped regions follow their parent's choice on the event. The parent's choice is
  // one disjunct, and its parts are conjoined inside it, so sibling parts always agree:
  // a move into the scoping state (or the parent's own entry there) (re)enters the
  // parts; keeping that state without moving lets them react; anything else closes
  // them. Exit wins. Regions without parts keep the original per-region formula.
  const children = regions.map((_, i) => regions.flatMap((r, ki) => r.scope && r.parent === i ? [ki] : []));
  const subtree = (xi) => [xi, ...children[xi].flatMap(subtree)];
  const entries = (r, action) => r.starts?.get(action.label) ?? [...r.initial];
  const closed = (ki) => helper(`closed ${regions[ki].field}`, "t: State",
    all([at(regions[ki], regions[ki].inactive, "t"), ...children[ki].map(closed)]));
  const untouched = (xi, action) => !subtree(xi).some((x) => action.moves.has(x));
  const unchangedTree = (xi) => helper(`same ${regions[xi].field}`, "s: State, t: State", and(subtree(xi).map((x) => self(regions[x]))));
  const enter = (ki, lands, action) => any(lands.map((state) =>
    all([at(regions[ki], state, "t"), ...children[ki].map((c) => (regions[c].scope.state === state ? enter(c, entries(regions[c], action), action) : closed(c)))])));
  const react = (xi, action, chosen) => {
    const x = regions[xi], moves = action.moves.get(xi) ?? [];
    if (!children[xi].length) return ordinary(x, xi, action, chosen);
    if (untouched(xi, action)) return unchangedTree(xi);
    const kept = (ki) => {
      const here = at(x, regions[ki].scope.state);
      return helper(`kept ${regions[ki].field}`, "s: State, t: State",
        any([all([here, react(ki, action, chosen)]), all([`!(${here})`, closed(ki)])]));
    };
    const { open, own, keep, guard, none } = split(moves, action, chosen);
    const options = open.map((m) => all([m.enabled, guard(m), at(x, m.to, "t"), ...children[xi].map((ki) => {
      const k = regions[ki];
      return m.to === k.scope.state ? enter(ki, m.enter?.[k.caption] ?? entries(k, action), action) : closed(ki);
    })]));
    const stay = moves.length ? any([none(open), ...own.map(keep)]) : "true";
    options.push(all([stay, self(x), ...children[xi].map(kept)]));
    return any(options);
  };
  const effect = (r, ri, action, chosen) => {
    if (!children[ri].length) return ordinary(r, ri, action, chosen);
    return react(ri, action, chosen);
  };
  // Regions naming a shared effect are written once per assignment of the event's shared
  // effects; every other region's constraint is the same under all of them.
  const effects = (action) => {
    const shared = [...sharedEffects(action)];
    const touches = (ri) => subtree(ri).some((x) => (action.moves.get(x) ?? []).some((m) => m.optional && shared.includes(m.intent)));
    const top = regions.flatMap((r, ri) => (r.scope ? [] : [ri])).filter((ri) => !untouched(ri, action));
    // Like TLA's UNCHANGED: every region the event cannot touch keeps its pre-state.
    const moved = top.flatMap(subtree);
    const frame = moved.length === regions.length ? []
      : [helper("unchanged", "s: State, t: State",
        `t == (State { ${moved.map((x) => `${regions[x].field}: t.${regions[x].field}`).join(", ")}${moved.length ? ", " : ""}..s })`)];
    const fixed = top.filter((ri) => !touches(ri)).map((ri) => effect(regions[ri], ri, action, new Set()));
    const varying = top.filter(touches);
    if (!varying.length) return [...fixed, ...frame];
    const assignments = [...Array(2 ** shared.length).keys()].map((bits) => new Set(shared.filter((_, i) => bits & (2 ** i))));
    return [...fixed, or(assignments.map((chosen) => and(varying.map((ri) => effect(regions[ri], ri, action, chosen))))), ...frame];
  };
  const init = (r) => {
    const start = or([...r.initial].map((s) => at(r, s)));
    if (!r.scope) return start;
    const here = at(regions[r.parent], r.scope.state);
    return or([and([here, start]), and([`!(${here})`, at(r, r.inactive)])]);
  };

  const out = [
    "// @generated by chart-to-verus.mjs; edit the diagram, not this file.",
    `// Chart SHA256: ${hash(text)}`,
    "// Abstract finite-state behavior only; implementation and progress need separate proof.",
    "use vstd::prelude::*;", "verus! {",
  ];
  for (const r of regions) {
    out.push(`// Region: ${comment(r.caption)}`, `pub enum ${r.type} {`);
    r.values.forEach((s) => out.push(`    ${r.variants.get(s)}, // ${comment(s)}`));
    out.push("}");
  }
  out.push("pub struct State {", ...regions.map((r) => `    pub ${r.field}: ${r.type},`), "}");
  const initDefinition = definition("init", "s: State", regions.map(init));
  const events = [];
  for (const action of actions) {
    const enabled = [...action.moves.values()].flatMap((moves) => moves.map((m) => m.enabled));
    // A scoped region's constraint is part of its outermost parent's formula.
    events.push(`// Event: ${comment(action.label || "unlabeled transition")}`,
      definition(action.function, "s: State, t: State", [or(enabled), ...effects(action)]));
  }
  if (helpers.length) out.push("// Named predicates shared by the definitions below.", ...helpers);
  out.push(initDefinition, ...events);
  out.push(`pub open spec fn next(s: State, t: State) -> bool {\n    ${or(actions.map((a) => `${a.function}(s, t)`))}\n}`);
  // TLA's [][Next]_vars also allows stuttering, independently of enabled actions.
  out.push(`pub open spec fn step(s: State, t: State) -> bool {\n    next(s, t) || s == t\n}`);
  for (const rule of rules) {
    out.push(`// ${comment(`${rule.name}: ${rule.sentence}`)}`);
    if (rule.kind === "scope") {
      const r = regions[rule.ri];
      out.push(`pub open spec fn ${rule.function}(s: State) -> bool {\n    (${at(r, r.inactive)}) <==> !(${at(regions[r.parent], r.scope.state)})\n}`);
    } else out.push(`pub open spec fn ${rule.function}(s: State) -> bool {\n    !(${and(rule.parts.map((p) => or(p.states.map((s) => at(regions[p.ri], s)))))})\n}`);
  }
  out.push(`pub open spec fn safety(s: State) -> bool {\n    ${and(rules.map((r) => `${r.function}(s)`))}\n}`, "}", "");
  const source = out.join("\n");
  const steps = stepsSource(module, actions);
  const scoped = regions.some((r) => r.scope);
  const mapping = {
    format: 1, module, chart_sha256: hash(text), generated_sha256: hash(source), steps_sha256: hash(steps),
    semantics: scoped ? `${SEMANTICS}; parallel parts follow their parent's post-state, and exit wins` : SEMANTICS,
    regions: regions.map((r) => ({ caption: r.caption, field: r.field, type: r.type,
      ...(r.instance ? { instance: r.instance } : {}),
      ...(r.scope ? { scope: r.scope, inactive: r.inactive, ...(r.starts ? { starts: Object.fromEntries(r.starts) } : {}) } : {}),
      states: r.values.map((caption) => ({ caption, variant: `${r.type}::${r.variants.get(caption)}`, initial: r.initial.has(caption) })) })),
    events: actions.map((a) => ({ label: a.label, function: a.function,
      ...(parsed.events.has(a.label) ? parsed.events.get(a.label) : {}),
      ...(sharedEffects(a).size ? { shared_effects: [...sharedEffects(a)] } : {}),
      transitions: [...a.moves].flatMap(([ri, moves]) => moves.map(({ from, to, guard, within, unless, enter, optional, intent }) => ({
        region: regions[ri].caption, from, to, guard: guard ?? null,
        ...(within ? { within } : {}), ...(unless ? { unless } : {}), ...(enter ? { enter } : {}),
        ...(optional ? { optional } : {}), ...(intent ? { intent } : {}) }))) })),
    rules: rules.map(({ name, sentence, states, function: fn, kind, ri, reason }) => kind === "scope"
      ? { name, sentence, kind, region: regions[ri].caption, scope: regions[ri].scope, function: fn }
      : { name, sentence, states, function: fn, ...(reason ? { reason } : {}) }),
  };
  return { source, steps, mapping, ir: { regions, actions, rules }, parsed };
}

// Like the #[inductive] lemmas of Verus's state_machine!: one small obligation per event, so
// no single query has to reason about every event at once. inductive() stays in the editable
// proof file; this file is generated and regenerated with the model.
function stepsSource(module, actions) {
  const lemma = (a) => `preserves_${a.function}`;
  return [
    "// @generated by chart-to-verus.mjs; edit the diagram, not this file.",
    `// One preservation obligation per event. Strengthen inductive() in ${module}_proof.rs.`,
    "use crate::chart::*;", "use crate::inductive;", "use vstd::prelude::*;", "verus! {",
    ...actions.map((a) => `pub proof fn ${lemma(a)}(s: State, t: State)\n    requires inductive(s), ensures ${a.function}(s, t) ==> inductive(t),\n{}`),
    // Each event's reasoning stays in its own lemma. Hiding the events here would make the
    // solver's encoding far costlier, so they stay open.
    "pub proof fn every_event_preserves_inductive(s: State, t: State)",
    "    requires inductive(s), ensures step(s, t) ==> inductive(t),",
    "{",
    ...actions.map((a) => `    ${lemma(a)}(s, t);`),
    "}", "}", "",
  ].join("\n");
}

export function proofStarter(module) {
  return `// Editable proof starter. Generated behavior stays in ${module}.rs.
// Strengthen inductive() when safety alone is not inductive; do not weaken safety.
// No production code, temporal progress, or fairness is established here.
#[path = "${module}.rs"]
mod chart;
#[path = "${module}_steps.rs"]
mod steps;
use chart::*;
use vstd::prelude::*;
verus! {
pub open spec fn inductive(s: State) -> bool { safety(s) }

pub proof fn initial_is_inductive(s: State)
    requires init(s), ensures inductive(s),
{}
pub proof fn step_preserves_inductive(s: State, t: State)
    requires inductive(s), step(s, t), ensures inductive(t),
{
    steps::every_event_preserves_inductive(s, t);
}
pub proof fn inductive_implies_safety(s: State)
    requires inductive(s), ensures safety(s),
{}

pub open spec fn path_from(s: State, rest: Seq<State>) -> bool
    decreases rest.len(),
{
    rest.len() == 0 || (step(s, rest.first()) && path_from(rest.first(), rest.drop_first()))
}
pub proof fn all_reached_states_are_safe(s: State, rest: Seq<State>)
    requires inductive(s), path_from(s, rest),
    ensures safety(s), forall|i: int| 0 <= i < rest.len() ==> safety(rest[i]),
    decreases rest.len(),
{
    // Each step is discharged by step_preserves_inductive; unfolding it here only costs time.
    hide(step);
    inductive_implies_safety(s);
    if rest.len() > 0 {
        step_preserves_inductive(s, rest.first());
        all_reached_states_are_safe(rest.first(), rest.drop_first());
        assert forall|i: int| 0 <= i < rest.len() implies safety(rest[i]) by {
            if i > 0 { assert(rest[i] == rest.drop_first()[i - 1]); }
        }
    }
}
pub proof fn every_initialized_finite_trace_is_safe(s: State, rest: Seq<State>)
    requires init(s), path_from(s, rest),
    ensures safety(s), forall|i: int| 0 <= i < rest.len() ==> safety(rest[i]),
{
    hide(step);
    initial_is_inductive(s);
    all_reached_states_are_safe(s, rest);
}
}
fn main() {}
`;
}

// Generate (or with check: true, compare without writing) NAME.rs, NAME_steps.rs, NAME.map.json and NAME_proof.rs.
export function writeModel({ chart, module, dir, check }) {
  const text = fs.readFileSync(chart, "utf8");
  // Drift is judged on the chart's own text, which a composition does not hold.
  if (COMPOSES.test(text)) throw new Error("verus and check take one chart; check a composition with rules, trace, lint and query");
  const { source, steps, mapping } = compile(text, module);
  dir = dir ?? path.dirname(chart);
  const modelPath = path.join(dir, `${module}.rs`);
  const mapPath = path.join(dir, `${module}.map.json`);
  const proofPath = path.join(dir, `${module}_proof.rs`);
  const stepsPath = path.join(dir, `${module}_steps.rs`);
  const mapText = JSON.stringify(mapping, null, 2) + "\n";
  if (check) {
    for (const [file, wanted] of [[modelPath, source], [stepsPath, steps], [mapPath, mapText]])
      if (!fs.existsSync(file) || fs.readFileSync(file, "utf8") !== wanted) throw new Error(`diagram/generated drift: ${file}`);
    if (!fs.existsSync(proofPath)) throw new Error(`missing proof entrypoint: ${proofPath}`);
    console.log(`Current: ${modelPath} (correspondence only; run Verus separately)`);
    return;
  }
  // Preserve hand edits and avoid silently replacing another model with the same name.
  if (fs.existsSync(modelPath)) {
    if (!fs.existsSync(mapPath)) throw new Error(`refusing to replace ${modelPath} without its generation map`);
    const prior = JSON.parse(fs.readFileSync(mapPath, "utf8"));
    if (hash(fs.readFileSync(modelPath, "utf8")) !== prior.generated_sha256)
      throw new Error(`hand-edited generated model: ${modelPath}; preserve edits in a separate refinement before regenerating`);
    if (fs.existsSync(stepsPath) && prior.steps_sha256 && hash(fs.readFileSync(stepsPath, "utf8")) !== prior.steps_sha256)
      throw new Error(`hand-edited generated steps: ${stepsPath}; strengthen inductive() in ${proofPath} instead`);
  } else if (fs.existsSync(mapPath) || fs.existsSync(proofPath)) {
    throw new Error(`incomplete existing output for ${module}; choose a new output directory`);
  }
  fs.mkdirSync(dir, { recursive: true });
  fs.writeFileSync(modelPath, source);
  fs.writeFileSync(stepsPath, steps);
  fs.writeFileSync(mapPath, mapText);
  if (!fs.existsSync(proofPath)) fs.writeFileSync(proofPath, proofStarter(module), { flag: "wx" });
  console.log(`Generated ${modelPath}, ${stepsPath} and ${mapPath}; proof entrypoint ${proofPath}`);
  console.log(`${mapping.regions.length} regions, ${mapping.events.length} events, ${mapping.rules.length} safety rules. Run Verus to check the proof.`);
  if (!mapping.rules.length) console.log("No %% never rules: this skeleton does not yet state a behavioral safety promise.");
}
