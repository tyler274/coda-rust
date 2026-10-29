#!/usr/bin/env node
// Check a journey chart, an XState machine, and compile it to Verus.
//
//   node verification/statechart.mjs check CHART.mjs [JOURNEYS.md]
//   node verification/statechart.mjs verus CHART.mjs --module NAME [--check]
//
// XState's own graph search finds every reachable state; the chart's meta.never rules
// are checked on each, and journeys replay on the machine itself. `verus` writes the same
// behavior as a Verus model for the chart's proof. Charts are flat parallel regions whose
// guards are stateIn, not, and or or: what Sketch shows and Verus can state. load()
// refuses anything else, so both commands read exactly the behavior XState runs.
import fs from "node:fs";
import path from "node:path";
import { createHash } from "node:crypto";
import { pathToFileURL } from "node:url";
import { and, createActor, not, or, stateIn } from "xstate";
import { getShortestPaths } from "@xstate/graph";

const CHECKS = { stateIn: stateIn({}).check, not: not(() => true).check, and: and([]).check, or: or([]).check };

// The keys a chart may use at each level; any other key (always, after, entry, exit,
// invoke, context, history...) is behavior Verus would not see.
const ALLOWED = {
  root: ["id", "type", "states", "meta", "description", "schemas"],
  region: ["initial", "states", "meta", "description"],
  state: ["on", "meta", "description", "tags"],
};

async function load(file) {
  const text = fs.readFileSync(file, "utf8");
  const { machine } = await import(pathToFileURL(path.resolve(file)).href);
  if (!machine) throw new Error(`${file} exports no machine`);
  const root = machine.root;
  const refuse = (where, why) => { throw new Error(`${where}: ${why}; charts are flat parallel regions with stateIn, not, and or or guards`); };
  const only = (node, level, where) => {
    for (const key of Object.keys(node.config)) if (!ALLOWED[level].includes(key)) refuse(where, `"${key}" is not supported`);
  };
  only(root, "root", root.id);
  if (root.type !== "parallel") refuse(root.id, "the machine must be type: 'parallel', even with one region");
  const regions = Object.values(root.states).map((r) => {
    const caption = r.key;
    only(r, "region", caption);
    if (r.type !== "compound") refuse(caption, "a region holds one state at a time; draw parallel behavior as more regions");
    for (const s of Object.values(r.states)) {
      if (s.type !== "atomic") refuse(`${caption} / ${s.key}`, "a state has states of its own; draw them as another region");
      only(s, "state", `${caption} / ${s.key}`);
    }
    const start = r.initial;
    if (start.actions.length || start.target.length !== 1 || start.target[0].parent !== r) refuse(caption, "initial must name one of the region's states, with no actions");
    return { node: r, caption, states: Object.keys(r.states), initial: start.target[0].key };
  });
  const regionOf = (caption) => regions.find((r) => r.caption === caption);
  // A guard as data: { in: [region, state] }, { not: g }, { and: [g] } or { or: [g] }.
  const guard = (g, where) => {
    if (g === undefined) return undefined;
    if (typeof g === "string") g = machine.implementations.guards[g];
    else if (g.type && !g.check) g = machine.implementations.guards[g.type];
    if (g?.check === CHECKS.stateIn && g.stateValue && typeof g.stateValue === "object") {
      const pairs = Object.entries(g.stateValue).map(([region, state]) => {
        if (typeof state !== "string" || !regionOf(region)?.states.includes(state)) refuse(where, `stateIn names ${region}: ${JSON.stringify(state)}, which the chart does not have`);
        return { in: [region, state] };
      });
      return pairs.length === 1 ? pairs[0] : { and: pairs };
    }
    if (g?.check === CHECKS.not) return { not: guard(g.guards[0], where) };
    if (g?.check === CHECKS.and) return { and: g.guards.map((x) => guard(x, where)) };
    if (g?.check === CHECKS.or) return { or: g.guards.map((x) => guard(x, where)) };
    refuse(where, "a guard is not stateIn({ region: state }), not, and or or");
  };
  // Each state's transitions, in the order XState tries them.
  for (const r of regions)
    r.moves = Object.fromEntries(r.states.map((from) => {
      const moves = [];
      for (const [event, list] of r.node.states[from].transitions) {
        const where = `${r.caption} / ${from} / ${event}`;
        if (event.includes("*")) refuse(where, "wildcard events are not supported");
        for (const t of list) {
          if (t.actions.length) refuse(where, "a transition has actions");
          if (t.target && (t.target.length !== 1 || t.target[0].parent !== r.node)) refuse(where, "a transition leaves its region");
          moves.push({ event, to: t.target?.[0].key ?? from, guard: guard(t.guard, where) });
        }
      }
      return [from, moves];
    }));
  const meta = root.meta ?? {};
  // A rule names, per region, the state or states that must never hold together.
  const rules = (meta.never ?? []).map((rule) => {
    const states = Object.fromEntries(Object.entries(rule.states).map(([region, list]) => {
      list = [list].flat();
      for (const state of list)
        if (!regionOf(region)?.states.includes(state)) refuse(`rule ${rule.name}`, `${region}: ${state} is not in the chart`);
      return [region, list];
    }));
    return { ...rule, states };
  });
  return { text, machine, regions, events: root.events, rules, meta };
}

// Where a snapshot is: one state caption per region.
const where = (chart, snapshot) => chart.regions.map((r) => snapshot.value[r.caption]);
const words = (chart, at) => at.map((s, i) => `${chart.regions[i].caption}: ${s}`).join("; ");
const breaks = (chart, rule, at) => Object.entries(rule.states).every(([region, states]) =>
  states.includes(at[chart.regions.findIndex((r) => r.caption === region)]));

function check(chart) {
  const paths = getShortestPaths(chart.machine, { events: chart.events.map((type) => ({ type })) });
  let failed = false;
  for (const rule of chart.rules) {
    const broken = paths.filter((p) => breaks(chart, rule, where(chart, p.state)))
      .sort((a, b) => a.steps.length - b.steps.length)[0];
    if (!broken) continue;
    failed = true;
    console.log(`BROKEN ${rule.name}: ${rule.sentence}`);
    broken.steps.forEach((step, i) => console.log(`  ${i + 1}. ${i ? step.event.type : "Start"} → ${words(chart, where(chart, step.state))}`));
  }
  console.log(failed ? "" : `Every rule holds in all ${paths.length} reachable states. Data, progress and implementation remain separate.`);
  return !failed;
}

// Journeys: "## Name", an optional "From: Name", then "1. event → State; State".
function journeys(chart, text) {
  const list = [];
  for (const line of text.split("\n")) {
    const heading = /^## (.+)$/.exec(line);
    if (heading) { list.push({ name: heading[1].trim(), steps: [] }); continue; }
    const from = /^From: (.+)$/.exec(line);
    if (from && list.length) { list.at(-1).from = from[1].trim(); continue; }
    const step = /^\d+\. (.+?) → (.+)$/.exec(line);
    if (step && list.length) list.at(-1).steps.push({ event: step[1].trim(), expect: step[2].split(";").map((s) => s.trim()) });
  }
  const events = (name) => {
    const j = list.find((x) => x.name === name);
    if (!j) throw new Error(`no journey named ${name}`);
    return [...(j.from ? events(j.from) : []), ...j.steps.map((s) => s.event)];
  };
  let failed = false;
  for (const j of list) {
    const actor = createActor(chart.machine).start();
    for (const event of j.from ? events(j.from) : []) actor.send({ type: event });
    const stuck = j.steps.find((step) => {
      if (!actor.getSnapshot().can({ type: step.event })) return true;
      actor.send({ type: step.event });
      const at = where(chart, actor.getSnapshot());
      return !step.expect.every((caption) => at.includes(caption));
    });
    if (stuck) {
      failed = true;
      console.log(`STUCK ${j.name}: "${stuck.event}" does not reach ${stuck.expect.join("; ")} (at ${words(chart, where(chart, actor.getSnapshot()))})`);
    }
  }
  console.log(failed ? "" : `${list.length} journeys: each reaches its outcome.`);
  return !failed;
}

// Rust names from the chart's words, as the earlier Verus models named them.
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
  const at = (r, state, v = "s") => `${v}.${r.field} == ${r.type}::${r.variants.get(state)}`;
  const region = (caption) => regions.find((r) => r.caption === caption) ?? (() => { throw new Error(`no region ${caption}`); })();
  const guard = (g) => g === undefined ? "true"
    : g.in ? at(region(g.in[0]), g.in[1])
      : g.not ? `!(${guard(g.not)})` : g.and ? all(g.and.map(guard)) : any(g.or.map(guard));
  const events = chart.events.map((label) => {
    // In each region, the first enabled transition from the current state is taken.
    const moves = regions.map((r) => r.states.flatMap((from) =>
      r.moves[from].filter((m) => m.event === label).map((m, i, list) => ({
        when: all([at(r, from), ...list.slice(0, i).map((p) => `!(${guard(p.guard)})`), guard(m.guard)]),
        to: m.to,
      }))));
    const constraint = regions.map((r, ri) => moves[ri].length
      ? any([...moves[ri].map((m) => `${m.when} && ${at(r, m.to, "t")}`), `!(${any(moves[ri].map((m) => m.when))}) && t.${r.field} == s.${r.field}`])
      : `t.${r.field} == s.${r.field}`);
    return { label, function: name(label, "event_", true), enabled: any(moves.flat().map((m) => m.when)), constraint };
  });
  const rules = chart.rules.map((rule) => ({ ...rule, function: name(rule.name, "rule_", true) }));
  const spec = (fn, params, body) => `pub open spec fn ${fn}(${params}) -> bool {\n    ${body}\n}`;
  const hash = (text) => createHash("sha256").update(text).digest("hex");
  const source = [
    "// @generated by verification/statechart.mjs; edit the chart, not this file.",
    `// Chart SHA256: ${hash(chart.text)}`,
    "// Abstract finite-state behavior only; implementation and progress need separate proof.",
    "use vstd::prelude::*;", "verus! {",
    ...regions.flatMap((r) => [`// Region: ${r.caption}`, `pub enum ${r.type} {`, ...r.states.map((s) => `    ${r.variants.get(s)}, // ${s}`), "}"]),
    "pub struct State {", ...regions.map((r) => `    pub ${r.field}: ${r.type},`), "}",
    spec("init", "s: State", all(regions.map((r) => at(r, r.initial)))),
    ...events.flatMap((e) => [`// Event: ${e.label}`, spec(e.function, "s: State, t: State", [e.enabled, ...e.constraint].map((c) => `(${c})`).join("\n    && "))]),
    spec("next", "s: State, t: State", any(events.map((e) => `${e.function}(s, t)`))),
    spec("step", "s: State, t: State", "next(s, t) || s == t"),
    ...rules.flatMap((r) => [`// ${r.name}: ${r.sentence}`,
      spec(r.function, "s: State", `!(${all(Object.entries(r.states).map(([rg, sts]) => any(sts.map((st) => at(region(rg), st)))))})`)]),
    spec("safety", "s: State", all(rules.map((r) => `${r.function}(s)`))),
    "}", "",
  ].join("\n");
  const steps = [
    "// @generated by verification/statechart.mjs; edit the chart, not this file.",
    `// One preservation obligation per event. Strengthen inductive() in ${module}_proof.rs.`,
    "use crate::chart::*;", "use crate::inductive;", "use vstd::prelude::*;", "verus! {",
    ...events.map((e) => `pub proof fn preserves_${e.function}(s: State, t: State)\n    requires inductive(s), ensures ${e.function}(s, t) ==> inductive(t),\n{}`),
    "pub proof fn every_event_preserves_inductive(s: State, t: State)",
    "    requires inductive(s), ensures step(s, t) ==> inductive(t),",
    "{", ...events.map((e) => `    preserves_${e.function}(s, t);`), "}", "}", "",
  ].join("\n");
  return { source, steps };
}

const PROOF = (module) => `// Editable proof. Generated behavior stays in ${module}.rs.
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
}
fn main() {}
`;

function writeVerus(chart, module, dir, checkOnly) {
  const { source, steps } = verus(chart, module);
  const files = [[path.join(dir, `${module}.rs`), source], [path.join(dir, `${module}_steps.rs`), steps]];
  if (checkOnly) {
    for (const [file, text] of files)
      if (!fs.existsSync(file) || fs.readFileSync(file, "utf8") !== text) throw new Error(`chart and generated model differ: ${file}`);
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

const [command, file, ...rest] = process.argv.slice(2);
const flag = (name) => { const i = rest.indexOf(name); return i < 0 ? undefined : rest[i + 1]; };
try {
  if (command === "check" && file) {
    const chart = await load(file);
    const ok = check(chart);
    const journeyFile = rest.find((a) => !a.startsWith("--"));
    process.exitCode = ok && (!journeyFile || journeys(chart, fs.readFileSync(journeyFile, "utf8"))) ? 0 : 1;
  } else if (command === "verus" && file && /^[A-Za-z]\w*$/.test(flag("--module") ?? "")) {
    writeVerus(await load(file), flag("--module"), flag("--dir") ?? path.join(path.dirname(file), "verus"), rest.includes("--check"));
  } else {
    console.error("Usage: node verification/statechart.mjs check CHART.mjs [JOURNEYS.md]\n       node verification/statechart.mjs verus CHART.mjs --module NAME [--dir OUT] [--check]");
    process.exitCode = 2;
  }
} catch (error) {
  console.error(error.message);
  process.exitCode = 1;
}
