#!/usr/bin/env node
// Verus backend adapted from pstack's chart-to-tla.mjs at eb4db2b.
// See verification/PSTACK-LICENSE. The chart owns generated behavior; proofs are editable.
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { createHash } from "node:crypto";
import { parseMermaid } from "./mermaid.mjs";

const HELP = `Usage: node chart-to-verus.mjs CHART.mmd --module NAME [--dir OUT] [--check | --trace] [--max-states N]

Compile pstack's supported Mermaid stateDiagram-v2 subset to Verus.
Writes NAME.rs (generated behavior), NAME.map.json (diagram correspondence),
and NAME_proof.rs (editable proof starter, created only when absent).
Existing generated files may be refreshed; hand-edited generated files are refused.
--check writes nothing and fails if generated files differ from the chart.
--trace searches this same finite chart without writing files; prints the shortest
violating path and a numbered Mermaid diagram. --max-states defaults to 100000.
An exhausted search covers only chart rules, not data proofs or progress.

Regions are finite state variables. Equally labeled edges form one broadcast event:
every region with an enabled edge moves; other regions remain unchanged. Choices
remain nondeterministic. Guards read the pre-event state. Unlabeled edges are
separate events. %% when [not] StateId[, StateId] guards the preceding edge;
its label must include readable [guard words]. %% never Name: Caption & Caption
-- sentence forbids a combination of states (or composite state captions).

The starter checks initialization, inductive safety, and finite traces. It may
need a stronger inductive invariant. A failed obligation is not automatically a
reachable design bug. This does not infer data guards, timers, fairness, runtime
atomicity, or production-code conformance. No verifier is run by this command.

Exit: 0 generated/current or completed chart search; 1 violation, invalid chart,
drift or I/O error; 2 invalid arguments; 3 incomplete search.
`;

function argumentsOf(argv) {
  const args = {};
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === "--help" || arg === "-h") return { help: true };
    if (arg === "--check") { args.check = true; continue; }
    if (arg === "--trace") { args.trace = true; continue; }
    if (arg === "--module" || arg === "--dir" || arg === "--max-states") {
      if (!argv[i + 1] || argv[i + 1].startsWith("-")) throw new Error(`${arg} needs a value`);
      const key = arg.slice(2);
      if (args[key]) throw new Error(`${arg} supplied twice`);
      args[key] = argv[++i];
    } else if (!arg.startsWith("-") && !args.chart) args.chart = arg;
    else throw new Error(`unknown argument ${arg}`);
  }
  if (!args.chart || !/^[A-Za-z][A-Za-z0-9_]*$/.test(args.module ?? ""))
    throw new Error("provide CHART.mmd and --module NAME (letters, digits, underscores)");
  if (args.check && args.trace) throw new Error("choose --check or --trace");
  if (args["max-states"] !== undefined && (!args.trace || !/^[1-9][0-9]*$/.test(args["max-states"]) || !Number.isSafeInteger(Number(args["max-states"]))))
    throw new Error("--max-states needs --trace and a positive safe integer");
  return args;
}

const hash = (text) => createHash("sha256").update(text).digest("hex");
const join = (xs, operator, empty) => xs.length === 0 ? empty : xs.length === 1 ? xs[0] : xs.map((x) => `(${x})`).join(` ${operator} `);
const and = (xs) => join(xs.filter((x) => x !== "true"), "&&", "true");
const or = (xs) => join(xs, "||", "false");
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
  `pub open spec fn ${name}(${parameters}) -> bool {\n    ${clauses.map((c) => `(${c})`).join("\n    && ")}\n}`;

function compile(text, module) {
  const parsed = parseMermaid(text);
  const types = new Set(), fields = new Set(), functions = new Set();
  const regions = Object.entries(parsed.regions).map(([caption, r]) => {
    if (!r.states.size || !r.initial.size) throw new Error(`region ${caption} needs states and [*] --> start`);
    const variants = new Map(), names = new Set();
    for (const state of r.states) variants.set(state, ident(state, "State", names));
    return { caption, field: ident(caption, "region_", fields, true),
      type: ident(caption, "Region", types), ...r, values: [...r.states], variants };
  });
  if (!regions.length) throw new Error("the chart has no regions");
  const at = (r, state, variable = "s") => `${variable}.${r.field} == ${r.type}::${r.variants.get(state)}`;
  const guard = (g) => {
    if (!g) return "true";
    const hits = Object.entries(g.states).map(([name, states]) => {
      const r = regions.find((r) => r.caption === name);
      return or(states.map((state) => at(r, state)));
    });
    return g.not ? `!(${or(hits)})` : and(hits);
  };
  const actions = [];
  const byLabel = new Map();
  for (const [ri, r] of regions.entries()) for (const t of r.transitions) {
    let action = t.label ? byLabel.get(t.label) : null;
    if (!action) {
      action = { label: t.label, function: ident(t.label || `${t.from} to ${t.to}`, "event_", functions, true), moves: new Map() };
      actions.push(action);
      if (t.label) byLabel.set(t.label, action);
    }
    if (!action.moves.has(ri)) action.moves.set(ri, []);
    const move = { ...t, enabled: and([at(r, t.from), guard(t.guard)]) };
    const moves = action.moves.get(ri);
    if (!moves.some((m) => m.from === move.from && m.to === move.to && m.enabled === move.enabled)) moves.push(move);
  }
  const ruleNames = new Set();
  const rules = parsed.nevers.map((n) => {
    if (ruleNames.has(n.name)) throw new Error(`duplicate never name ${n.name}`);
    ruleNames.add(n.name);
    const parts = n.states.map((caption) => {
      const hits = regions.flatMap((r, ri) => {
        const states = r.states.has(caption) ? [caption] : r.values.filter((s) => s.startsWith(`${caption}.`));
        return states.length ? [{ ri, states }] : [];
      });
      if (hits.length !== 1) throw new Error(`never ${n.name}: "${caption}" must identify one state or composite in one region`);
      return hits[0];
    });
    if (new Set(parts.map((p) => p.ri)).size !== parts.length)
      throw new Error(`never ${n.name} names two states of one region`);
    return { ...n, function: ident(n.name, "rule_", functions, true), parts };
  });
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
  out.push(definition("init", "s: State", regions.map((r) => or([...r.initial].map((s) => at(r, s))))));
  for (const action of actions) {
    const enabled = [...action.moves.values()].flatMap((moves) => moves.map((m) => m.enabled));
    const effects = regions.map((r, ri) => {
      const moves = action.moves.get(ri);
      if (!moves) return `t.${r.field} == s.${r.field}`;
      const choices = moves.map((m) => and([m.enabled, at(r, m.to, "t")]));
      // As in chart-to-tla: each enabled region participates in a shared event;
      // a region with no enabled move stays. All guards read s, never t.
      if (action.moves.size > 1) choices.push(and([`!(${or(moves.map((m) => m.enabled))})`, `t.${r.field} == s.${r.field}`]));
      return or(choices);
    });
    out.push(`// Event: ${comment(action.label || "unlabeled transition")}`,
      definition(action.function, "s: State, t: State", [or(enabled), ...effects]));
  }
  out.push(`pub open spec fn next(s: State, t: State) -> bool {\n    ${or(actions.map((a) => `${a.function}(s, t)`))}\n}`);
  // TLA's [][Next]_vars also allows stuttering, independently of enabled actions.
  out.push(`pub open spec fn step(s: State, t: State) -> bool {\n    next(s, t) || s == t\n}`);
  for (const rule of rules) out.push(`// ${comment(`${rule.name}: ${rule.sentence}`)}`,
    `pub open spec fn ${rule.function}(s: State) -> bool {\n    !(${and(rule.parts.map((p) => or(p.states.map((s) => at(regions[p.ri], s)))))})\n}`);
  out.push(`pub open spec fn safety(s: State) -> bool {\n    ${and(rules.map((r) => `${r.function}(s)`))}\n}`, "}", "");
  const source = out.join("\n");
  const mapping = {
    format: 1, module, chart_sha256: hash(text), generated_sha256: hash(source),
    semantics: "pstack broadcast events; pre-state guards; nondeterministic choices; stuttering allowed",
    regions: regions.map((r) => ({ caption: r.caption, field: r.field, type: r.type,
      states: r.values.map((caption) => ({ caption, variant: `${r.type}::${r.variants.get(caption)}`, initial: r.initial.has(caption) })) })),
    events: actions.map((a) => ({ label: a.label, function: a.function,
      transitions: [...a.moves].flatMap(([ri, moves]) => moves.map(({ from, to, guard }) => ({ region: regions[ri].caption, from, to, guard: guard ?? null }))) })),
    rules: rules.map(({ name, sentence, states, function: fn }) => ({ name, sentence, states, function: fn })),
  };
  return { source, mapping, ir: { regions, actions, rules } };
}

// Search the exact flattened regions, grouped events and resolved rules used above.
// Stuttering and duplicate self-loops cannot shorten a safety counterexample.
function* product(choices, prefix = [], index = 0) {
  if (index === choices.length) { yield prefix; return; }
  for (const value of choices[index]) yield* product(choices, [...prefix, value], index + 1);
}
function search(ir, limit) {
  const { regions, actions, rules } = ir;
  if (!rules.length) return { status: "no-rules", visited: 0 };
  const byCaption = new Map(regions.map((r, i) => [r.caption, i]));
  const holds = (g, state) => {
    if (!g) return true;
    const hits = Object.entries(g.states).map(([region, states]) => states.includes(state[byCaption.get(region)]));
    return g.not ? !hits.some(Boolean) : hits.every(Boolean);
  };
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
  const violation = (state) => rules.find((r) => r.parts.every((p) => p.states.includes(state[p.ri])));
  const found = (index, rule) => {
    const steps = [];
    for (let i = index; i !== null; i = queue[i].parent) steps.push(queue[i]);
    return { status: "violation", visited: queue.length, rule, steps: steps.reverse() };
  };
  for (const state of product(regions.map((r) => [...r.initial]))) {
    if (!add(state, null, null)) return { status: "incomplete", visited: queue.length };
    const bad = violation(state);
    if (bad) return found(seen.get(key(state)), bad);
  }
  for (let index = 0; index < queue.length; index++) {
    const state = queue[index].state;
    for (const action of actions) {
      let any = false;
      const choices = regions.map((r, ri) => {
        const enabled = (action.moves.get(ri) ?? []).filter((m) => m.from === state[ri] && holds(m.guard, state));
        if (enabled.length) any = true;
        return enabled.length ? [...new Set(enabled.map((m) => m.to))] : [state[ri]];
      });
      if (!any) continue;
      for (const next of product(choices)) {
        if (!add(next, index, action)) return { status: "incomplete", visited: queue.length };
        const bad = violation(next);
        if (bad) return found(seen.get(key(next)), bad);
      }
    }
  }
  return { status: "exhausted", visited: queue.length };
}
function printSearch(result, ir) {
  if (result.status === "no-rules") {
    console.log("No chart safety rules; no safety claim checked. Use linked data proofs.");
    return;
  }
  if (result.status === "incomplete") {
    console.log(`INCOMPLETE after ${result.visited} states: the limit prevented exhaustive search. No pass established.`);
    process.exitCode = 3;
    return;
  }
  if (result.status === "exhausted") {
    console.log(`Exhausted ${result.visited} reachable chart states; no chart-rule violation. Data, progress and implementation remain separate.`);
    return;
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
  process.exitCode = 1;
}

function proofStarter(module) {
  return `// Editable proof starter. Generated behavior stays in ${module}.rs.
// Strengthen inductive() when safety alone is not inductive; do not weaken safety.
// No production code, temporal progress, or fairness is established here.
#[path = "${module}.rs"]
mod chart;
use chart::*;
use vstd::prelude::*;
verus! {
pub open spec fn inductive(s: State) -> bool { safety(s) }

pub proof fn initial_is_inductive(s: State)
    requires init(s), ensures inductive(s),
{}
pub proof fn step_preserves_inductive(s: State, t: State)
    requires inductive(s), step(s, t), ensures inductive(t),
{}
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
    initial_is_inductive(s);
    all_reached_states_are_safe(s, rest);
}
}
fn main() {}
`;
}

let args;
try { args = argumentsOf(process.argv.slice(2)); }
catch (error) { console.error(`${error.message}\n${HELP.split("\n")[0]}`); process.exit(2); }
if (args.help) { console.log(HELP); process.exit(0); }
try {
  const text = fs.readFileSync(args.chart, "utf8");
  const { source, mapping, ir } = compile(text, args.module);
  if (args.trace) { printSearch(search(ir, Number(args["max-states"] ?? 100000)), ir); } else {
  const dir = args.dir ?? path.dirname(args.chart);
  const modelPath = path.join(dir, `${args.module}.rs`);
  const mapPath = path.join(dir, `${args.module}.map.json`);
  const proofPath = path.join(dir, `${args.module}_proof.rs`);
  const mapText = JSON.stringify(mapping, null, 2) + "\n";
  if (args.check) {
    for (const [file, wanted] of [[modelPath, source], [mapPath, mapText]])
      if (!fs.existsSync(file) || fs.readFileSync(file, "utf8") !== wanted) throw new Error(`diagram/generated drift: ${file}`);
    if (!fs.existsSync(proofPath)) throw new Error(`missing proof entrypoint: ${proofPath}`);
    console.log(`Current: ${modelPath} (correspondence only; run Verus separately)`);
  } else {
    // Preserve hand edits and avoid silently replacing another model with the same name.
    if (fs.existsSync(modelPath)) {
      if (!fs.existsSync(mapPath)) throw new Error(`refusing to replace ${modelPath} without its generation map`);
      const prior = JSON.parse(fs.readFileSync(mapPath, "utf8"));
      if (hash(fs.readFileSync(modelPath, "utf8")) !== prior.generated_sha256)
        throw new Error(`hand-edited generated model: ${modelPath}; preserve edits in a separate refinement before regenerating`);
    } else if (fs.existsSync(mapPath) || fs.existsSync(proofPath)) {
      throw new Error(`incomplete existing output for ${args.module}; choose a new output directory`);
    }
    fs.mkdirSync(dir, { recursive: true });
    fs.writeFileSync(modelPath, source);
    fs.writeFileSync(mapPath, mapText);
    if (!fs.existsSync(proofPath)) fs.writeFileSync(proofPath, proofStarter(args.module), { flag: "wx" });
    console.log(`Generated ${modelPath} and ${mapPath}; proof entrypoint ${proofPath}`);
    console.log(`${mapping.regions.length} regions, ${mapping.events.length} events, ${mapping.rules.length} safety rules. Run Verus to check the proof.`);
    if (!mapping.rules.length) console.log("No %% never rules: this skeleton does not yet state a behavioral safety promise.");
  }
  }
} catch (error) { console.error(error.message); process.exitCode = 1; }
