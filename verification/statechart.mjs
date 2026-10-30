#!/usr/bin/env node
// Check a journey's model and compile it to Verus.
//
//   node verification/statechart.mjs check MODEL.mjs
//   node verification/statechart.mjs verus MODEL.mjs --module NAME [--dir OUT] [--check]
//   node verification/statechart.mjs walks MODEL.mjs [--only REGEX | --paths JSON]
//
// A model (see model.mjs) holds the journey: everyone who acts in it, each one indivisible step in
// their code. `check` walks every reachable state, holds the model's `never` rules and `invariant`
// facts in each, finds each `reach` outcome, a list of states the model must pass through in
// order, and makes each control's change to show the rule it names can break. `verus` writes the same steps, rules and facts as a Verus model; the proof shows the facts
// hold at the start, survive every step and imply the rules, so it covers every state, not only the
// ones searched. `walks` prints, as JSON, walks from the start that between them take every case of
// every event (or those --only names), each step with the model's state after it, so a test can
// drive the code along them and compare; with --paths, a list of event lists, the walk through each,
// such as the bug paths to replay against the code.
import fs from "node:fs";
import path from "node:path";
import { createHash } from "node:crypto";
import { pathToFileURL } from "node:url";

async function load(file) {
  const text = fs.readFileSync(file, "utf8");
  const { machine } = await import(pathToFileURL(path.resolve(file)).href);
  if (!machine?.regions || !machine?.events) throw new Error(`${file} exports no model; build it with verification/model.mjs`);
  const regions = Object.entries(machine.regions).map(([caption, states]) => ({ caption, states, initial: states[0] }));
  const events = Object.keys(machine.events);
  return { text, machine, regions, events, rules: machine.never, facts: machine.invariant, reach: machine.reach };
}

// The same model with one control's change made to a copy of its events.
function changed(chart, control) {
  const events = structuredClone(chart.machine.events);
  control.change(events);
  return { ...chart, machine: { ...chart.machine, events } };
}

const words = (chart, v) => chart.regions.map((r, i) => `${r.caption}: ${r.states[v[i]]}`).join("; ");

// Every reachable state. On an event, the first case whose guard holds applies; the event happens
// when that case moves some region from where it is. A state is an array of state indexes, one per
// region, kept as bytes and named by one exact number.
// With `stop`, the search ends at the first state that meets it, and `stopped` names that state.
function explore(chart, only = () => true, stop) {
  const regions = chart.regions;
  const index = new Map(regions.map((r, i) => [r.caption, i]));
  const states = regions.map((r) => new Map(r.states.map((s, j) => [s, j])));
  const radix = regions.map((r) => r.states.length);
  if (radix.reduce((a, b) => a * b, 1) > Number.MAX_SAFE_INTEGER) throw new Error("the model has more states than one number can name");
  if (radix.some((n) => n > 255)) throw new Error("a region has more than 255 states");
  const encode = (v) => { let k = 0; for (let i = regions.length - 1; i >= 0; i--) k = k * radix[i] + v[i]; return k; };
  const compile = (g) => {
    if (!g) return () => true;
    if (g.in) { const i = index.get(g.in[0]), j = states[i].get(g.in[1]); return (v) => v[i] === j; }
    if (g.not) { const f = compile(g.not); return (v) => !f(v); }
    const fs = (g.and ?? g.or).map(compile);
    return g.and ? (v) => fs.every((f) => f(v)) : (v) => fs.some((f) => f(v));
  };
  const cases = chart.events.map((event) => chart.machine.events[event].cases.map((c) => ({
    when: compile(c.when),
    set: Object.entries(c.set).map(([caption, effect]) => {
      const i = index.get(caption);
      return [i, regions[i].states.map((from) => { const to = typeof effect === "string" ? effect : effect[from]; return to === undefined ? -1 : states[i].get(to); })];
    }),
  })));
  // The case an event takes, or -1 when it can't happen.
  const which = (v, e) => {
    for (let k = 0; k < cases[e].length; k++) {
      const c = cases[e][k];
      if (!c.when(v)) continue;
      return c.set.some(([i, to]) => to[v[i]] >= 0) ? k : -1;
    }
    return -1;
  };
  const step = (v, e) => {
    const k = which(v, e);
    if (k < 0) return null;
    const next = Array.from(v);
    for (const [i, to] of cases[e][k].set) if (to[v[i]] >= 0) next[i] = to[v[i]];
    return next;
  };
  const allowed = chart.events.map((event) => only(event));
  // Many maps, since one map holds at most 2^24 entries.
  const width = regions.length, CHUNK = 1 << 20;
  const chunks = [], found = Array.from({ length: 256 }, () => new Map()), parent = [], via = [];
  let count = 0, stopped = -1;
  const lookup = (k) => found[k % 256].get(k);
  const vector = (n) => chunks[Math.floor(n / CHUNK)].subarray((n % CHUNK) * width, (n % CHUNK + 1) * width);
  const add = (v, k, from, e) => {
    if (count % CHUNK === 0) chunks.push(new Uint8Array(CHUNK * width));
    chunks[chunks.length - 1].set(v, (count % CHUNK) * width);
    if (stopped < 0 && stop?.(v)) stopped = count;
    found[k % 256].set(k, count++); parent.push(from); via.push(e);
  };
  const start = regions.map((r, i) => states[i].get(r.initial));
  add(start, encode(start), -1, -1);
  const next = (n) => {
    const v = vector(n), out = [];
    for (let e = 0; e < chart.events.length; e++) {
      if (!allowed[e]) continue;
      const after = step(v, e);
      if (!after) continue;
      const k = encode(after);
      let m = lookup(k);
      if (m === undefined) { m = count; add(after, k, n, e); }
      out.push([e, m]);
    }
    return out;
  };
  for (let n = 0; n < count && stopped < 0; n++) {
    next(n);
    if (process.env.EXPLORE_PROGRESS && n % 500000 === 0 && n) console.error(`${n} explored, ${count} found`);
  }
  const path = (n) => { const steps = []; for (; n > 0; n = parent[n]) steps.unshift({ event: chart.events[via[n]], to: n }); return steps; };
  const holds = (g) => compile(g);
  const matches = (map) => {
    const tests = Object.entries(map).map(([region, list]) => { const i = index.get(region); const ok = new Set([list].flat().map((s) => states[i].get(s))); return (v) => ok.has(v[i]); });
    return (v) => tests.every((f) => f(v));
  };
  return { count, stopped, vector, next, path, matches, holds, step, which: (n, e) => which(vector(n), e) };
}

function check(chart) {
  const graph = explore(chart);
  const show = (steps) => steps.forEach((step, i) => console.log(`  ${i + 1}. ${step.event} → ${words(chart, graph.vector(step.to))}`));
  let failed = false;
  for (const rule of chart.rules) {
    const breaks = graph.matches(rule.states);
    let broken = -1;
    for (let n = 0; n < graph.count && broken < 0; n++) if (breaks(graph.vector(n))) broken = n;
    if (broken < 0) continue;
    failed = true;
    console.log(`BROKEN ${rule.name}: ${rule.sentence}`);
    show(graph.path(broken));
  }
  for (const fact of chart.facts) {
    const holds = graph.holds(fact.holds);
    let wrong = -1;
    for (let n = 0; n < graph.count && wrong < 0; n++) if (!holds(graph.vector(n))) wrong = n;
    if (wrong < 0) continue;
    failed = true;
    console.log(`FALSE ${fact.name}`);
    show(graph.path(wrong));
  }
  console.log(failed ? `${graph.count} reachable states.` : `Every rule and fact holds in all ${graph.count} reachable states. Data, progress and implementation remain separate.`);
  // Each outcome: a walk through its states in order, found breadth-first over (state, progress).
  let missed = 0;
  for (const outcome of chart.reach) {
    const marks = outcome.through.map(graph.matches);
    const advance = (n, j) => { const v = graph.vector(n); while (j < marks.length && marks[j](v)) j++; return j; };
    const size = marks.length + 1;
    const seen = new Set([advance(0, 0)]), queue = [...seen];
    let done = queue[0] === marks.length;
    for (let q = 0; q < queue.length && !done; q++) {
      const n = Math.floor(queue[q] / size), j = queue[q] % size;
      for (const [, m] of graph.next(n)) {
        const id = m * size + advance(m, j);
        if (seen.has(id)) continue;
        seen.add(id);
        queue.push(id);
        if (id % size === marks.length) { done = true; break; }
      }
    }
    if (!done) { missed++; console.log(`UNREACHED ${outcome.name}`); }
  }
  if (chart.reach.length && !missed) console.log(`${chart.reach.length} outcomes: the model reaches each.`);
  // Each control: its change must break a rule that says its sentence.
  let held = 0;
  for (const control of chart.machine.controls) {
    const broken = changed(chart, control);
    const marks = chart.rules.filter((rule) => rule.sentence === control.sentence).map((rule) => graph.matches(rule.states));
    const search = explore(broken, undefined, (v) => marks.some((breaks) => breaks(v)));
    if (search.stopped >= 0) continue;
    held++;
    console.log(`UNBROKEN "${control.sentence}" still holds when ${control.breaks}`);
  }
  if (chart.machine.controls.length && !held) console.log(`${chart.machine.controls.length} controls: each rule breaks when its control's change is made.`);
  return !failed && !missed && !held;
}

// Walks that take every case of every allowed event: for each, the shortest way to a state where
// it applies, then the event; walks that another walk begins with are left out.
function walks(chart, only) {
  const graph = explore(chart, only);
  const names = (v) => Object.fromEntries(chart.regions.map((r, i) => [r.caption, r.states[v[i]]]));
  const targets = new Map();
  for (let n = 0; n < graph.count; n++)
    for (let e = 0; e < chart.events.length; e++) {
      if (!only(chart.events[e])) continue;
      const k = graph.which(n, e);
      if (k >= 0 && !targets.has(`${e}:${k}`)) targets.set(`${e}:${k}`, [n, e]);
    }
  const list = [...targets.values()].map(([n, e]) => {
    const steps = graph.path(n).map((s) => ({ event: s.event, state: names(graph.vector(s.to)) }));
    const [, m] = graph.next(n).find(([x]) => x === e);
    return [...steps, { event: chart.events[e], state: names(graph.vector(m)) }];
  }).sort((a, b) => b.length - a.length);
  const key = (w) => w.map((s) => s.event).join("\n") + "\n";
  const kept = [];
  for (const w of list) if (!kept.some((k) => key(k).startsWith(key(w)))) kept.push(w);
  return { start: names(graph.vector(0)), cases: targets.size, walks: kept.reverse() };
}

// The one walk through `events` in order, each step with the model's state after it.
function walkThrough(chart, paths) {
  const graph = explore(chart, undefined, () => true);
  const names = (v) => Object.fromEntries(chart.regions.map((r, i) => [r.caption, r.states[v[i]]]));
  const walk = (events) => {
    let v = graph.vector(0);
    return events.map((event, n) => {
      const e = chart.events.indexOf(event);
      if (e < 0) throw new Error(`no event "${event}"`);
      const after = graph.step(v, e);
      if (!after) throw new Error(`step ${n + 1}, "${event}", can't happen there`);
      v = after;
      return { event, state: names(v) };
    });
  };
  return { start: names(graph.vector(0)), cases: 0, walks: paths.map(walk) };
}

// Rust names from the model's words.
function namer() {
  const taken = new Set();
  return (words, prefix, snake = false) => {
    const parts = words.normalize("NFKD").replace(/([a-z0-9])([A-Z])/g, "$1 $2")
      .replace(/[^A-Za-z0-9]+/g, " ").trim().split(/\s+/).filter(Boolean);
    const stem = snake ? parts.map((p) => p.toLowerCase()).join("_") : parts.map((p) => p[0].toUpperCase() + p.slice(1)).join("");
    let name = prefix + (stem || "Unnamed");
    for (let n = 2; taken.has(name); n++) name = `${prefix}${stem}_${n}`;
    taken.add(name);
    return name;
  };
}

const any = (xs) => (xs.length ? xs.map((x) => `(${x})`).join(" || ") : "false");
const all = (xs) => { xs = xs.filter((x) => x !== "true"); return xs.length ? xs.map((x) => `(${x})`).join(" && ") : "true"; };

function verus(chart, module) {
  const name = namer();
  const regions = chart.regions.map((r) => {
    const local = namer();
    return { ...r, field: name(r.caption, "region_", true), type: name(r.caption, "Region"),
      variants: new Map(r.states.map((s) => [s, local(s, "State")])) };
  });
  const byCaption = new Map(regions.map((r) => [r.caption, r]));
  const at = (r, state, v = "s") => `${v}.${r.field} == ${r.type}::${r.variants.get(state)}`;
  const guard = (g) => g === undefined ? "true"
    : g.in ? at(byCaption.get(g.in[0]), g.in[1])
      : g.not ? `!(${guard(g.not)})` : g.and ? all(g.and.map(guard)) : any(g.or.map(guard));
  // An event: its first case that holds, moving some region from where it is.
  const events = chart.events.map((label) => {
    const cases = chart.machine.events[label].cases;
    const options = cases.map((c) => {
      const effects = new Map(Object.entries(c.set).map(([caption, effect]) => {
        const r = byCaption.get(caption);
        return [caption, typeof effect === "string" ? r.states.map((from) => [from, effect]) : Object.entries(effect)];
      }));
      // The case happens when it moves some region from where it is.
      const moves = [...effects].some(([caption, pairs]) => pairs.length === byCaption.get(caption).states.length) ? "true"
        : any([...effects].map(([caption, pairs]) => any(pairs.map(([from]) => at(byCaption.get(caption), from)))));
      // The next state: each region the case sets, from where it is; every other region stays.
      const value = (r, state) => `${r.type}::${r.variants.get(state)}`;
      const fields = [...effects].map(([caption, pairs]) => {
        const r = byCaption.get(caption);
        if (pairs.length === r.states.length && new Set(pairs.map(([, to]) => to)).size === 1) return `${r.field}: ${value(r, pairs[0][1])}`;
        const arms = pairs.map(([from, to]) => `${value(r, from)} => ${value(r, to)}`);
        if (pairs.length < r.states.length) arms.push(`_ => s.${r.field}`);
        return `${r.field}: match s.${r.field} { ${arms.join(", ")} }`;
      });
      return { when: guard(c.when), then: `(${moves})\n        && t == (State { ${fields.join(",\n            ")}, ..s })` };
    });
    // The first case that holds decides, as an if-else chain over the cases' own functions.
    const fn = name(label, "event_", true);
    const named = options.map((o, i) => ({ ...o, when_fn: `${fn}_when_${i + 1}`, case_fn: `${fn}_case_${i + 1}`, passed_fn: `${fn}_passes_${i + 1}` }));
    const body = named.map((o, i) => `${i ? " else " : ""}if ${o.when_fn}(s) {\n        ${o.case_fn}(s, t)\n    }`).join("") + " else {\n        false\n    }";
    return { label, function: fn, cases: named, body };
  });
  const rules = chart.rules.map((rule) => ({ ...rule, function: name(rule.name, "rule_", true) }));
  const facts = chart.facts.map((fact) => ({ ...fact, function: name(fact.name, "fact_", true) }));
  const spec = (fn, params, body) => `pub open spec fn ${fn}(${params}) -> bool {\n    ${body}\n}`;
  const hash = (text) => createHash("sha256").update(text).digest("hex");
  const source = [
    "// @generated by verification/statechart.mjs; edit the model, not this file.",
    `// Model SHA256: ${hash(chart.text)}`,
    "// Abstract finite-state behavior only; implementation and progress need separate proof.",
    "use vstd::prelude::*;", "verus! {",
    ...regions.flatMap((r) => [`// Region: ${r.caption}`, `pub enum ${r.type} {`, ...r.states.map((s) => `    ${r.variants.get(s)}, // ${s}`), "}"]),
    "pub struct State {", ...regions.map((r) => `    pub ${r.field}: ${r.type},`), "}",
    spec("init", "s: State", all(regions.map((r) => at(r, r.initial)))),
    ...events.flatMap((e) => [`// Event: ${e.label}`,
      // Case i applies when no earlier case holds (passes) and its own `when` does.
      ...e.cases.flatMap((c, i) => [spec(c.when_fn, "s: State", c.when), spec(c.case_fn, "s: State, t: State", c.then),
        spec(c.passed_fn, "s: State", i ? `${e.cases[i - 1].passed_fn}(s) && !${e.cases[i - 1].when_fn}(s)` : "true")]),
      spec(e.function, "s: State, t: State", e.body)]),
    spec("next", "s: State, t: State", any(events.map((e) => `${e.function}(s, t)`))),
    spec("step", "s: State, t: State", "next(s, t) || s == t"),
    ...rules.flatMap((r) => [`// ${r.name}: ${r.sentence}`,
      spec(r.function, "s: State", `!(${all(Object.entries(r.states).map(([rg, sts]) => any([sts].flat().map((st) => at(byCaption.get(rg), st)))))})`)]),
    spec("safety", "s: State", all(rules.map((r) => `${r.function}(s)`))),
    ...facts.flatMap((f) => [`// ${f.name}`, spec(f.function, "s: State", guard(f.holds))]),
    spec("facts", "s: State", all(facts.map((f) => `${f.function}(s)`))),
    "}", "",
  ].join("\n");
  const steps = [
    "// @generated by verification/statechart.mjs; edit the model, not this file.",
    `// One preservation obligation per case of each event. Strengthen the model's facts when one fails.`,
    "use crate::chart::*;", "use crate::inductive;", "use vstd::prelude::*;", "verus! {",
    ...events.flatMap((e) => [
      ...e.cases.map((c) => `pub proof fn preserves_${c.case_fn}(s: State, t: State)\n    requires inductive(s), ${c.passed_fn}(s), ${c.when_fn}(s), ${c.case_fn}(s, t),\n    ensures inductive(t),\n{}`),
      `pub proof fn preserves_${e.function}(s: State, t: State)\n    requires inductive(s), ensures ${e.function}(s, t) ==> inductive(t),\n{\n    if ${e.function}(s, t) {\n        ${
        e.cases.map((c, i) => `${i ? " else " : ""}if ${c.when_fn}(s) {\n            preserves_${c.case_fn}(s, t);\n        }`).join("")}\n    }\n}`]),
    "pub proof fn every_event_preserves_inductive(s: State, t: State)",
    "    requires inductive(s), ensures step(s, t) ==> inductive(t),",
    "{", ...events.map((e) => `    preserves_${e.function}(s, t);`), "}", "}", "",
  ].join("\n");
  return { source, steps };
}

const PROOF = (module) => `// Editable proof. Generated behavior, rules and facts stay in ${module}.rs.
// Strengthen the model's facts when they are not inductive; do not weaken safety.
// No production code, temporal progress, or fairness is established here.
#[path = "${module}.rs"]
mod chart;
#[path = "${module}_steps.rs"]
mod steps;
use chart::*;
use vstd::prelude::*;
verus! {
pub open spec fn inductive(s: State) -> bool { facts(s) && safety(s) }

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
}
fn main() {}
`;

function writeVerus(chart, module, dir, checkOnly) {
  const { source, steps } = verus(chart, module);
  const files = [[path.join(dir, `${module}.rs`), source], [path.join(dir, `${module}_steps.rs`), steps]];
  if (checkOnly) {
    for (const [file, text] of files)
      if (!fs.existsSync(file) || fs.readFileSync(file, "utf8") !== text) throw new Error(`model and generated Verus differ: ${file}`);
    console.log(`Current: ${files[0][0]}`);
    return;
  }
  for (const [file] of files)
    if (fs.existsSync(file) && !fs.readFileSync(file, "utf8").startsWith("// @generated by verification/statechart.mjs"))
      throw new Error(`refusing to replace ${file}, which this script did not generate`);
  fs.mkdirSync(dir, { recursive: true });
  for (const [file, text] of files) fs.writeFileSync(file, text);
  const proof = path.join(dir, `${module}_proof.rs`);
  if (!fs.existsSync(proof)) fs.writeFileSync(proof, PROOF(module));
  console.log(`Generated ${files.map(([f]) => f).join(" and ")}; proof ${proof}`);
}

const run = async (command, file, rest) => {
  const flag = (name) => { const i = rest.indexOf(name); return i < 0 ? undefined : rest[i + 1]; };
  if (command === "check" && file) return check(await load(file)) ? 0 : 1;
  if (command === "walks" && file && flag("--paths")) {
    process.stdout.write(JSON.stringify(walkThrough(await load(file), JSON.parse(flag("--paths")))));
    return 0;
  }
  if (command === "walks" && file) {
    const pattern = flag("--only") && new RegExp(flag("--only"));
    process.stdout.write(JSON.stringify(walks(await load(file), (event) => !pattern || pattern.test(event))));
    return 0;
  }
  if (command === "verus" && file && /^[A-Za-z]\w*$/.test(flag("--module") ?? "")) {
    writeVerus(await load(file), flag("--module"), flag("--dir") ?? path.join(path.dirname(file), "verus"), rest.includes("--check"));
    return 0;
  }
  console.error("Usage: node verification/statechart.mjs check MODEL.mjs\n       node verification/statechart.mjs verus MODEL.mjs --module NAME [--dir OUT] [--check]\n       node verification/statechart.mjs walks MODEL.mjs [--only REGEX | --paths JSON]");
  return 2;
};

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  const [command, file, ...rest] = process.argv.slice(2);
  try {
    process.exitCode = await run(command, file, rest);
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
