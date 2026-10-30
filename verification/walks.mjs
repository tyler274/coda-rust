#!/usr/bin/env node
// Walks through a journey's TLA+ model, for its browser test to play against the real page.
//   node verification/walks.mjs SPEC.tla [--cover EXPR] [--walks N] [--depth D] [--seed S] [--skip EVENT]...
//   node verification/walks.mjs SPEC.tla --paths JSON
//
// TLC takes every step. The spec names each step in an `event` variable, a record with `name`
// and, for a tab's step, `tab`, and its config's VIEW leaves `event` out of the state. The first
// form asks TLC for the shortest walk to each value of EXPR, a TLA+ expression over the state
// after a step (by default `event`, so each event), then N random walks of D steps for the orders
// between them, all from the start and leaving out the events --skip names; the second
// asks for the walk through each list of events in JSON, such as a bug path, and fails when one
// can't happen. Either prints, as JSON,
// the start state and each walk's steps, each with its event ("open Coda in tab A") and the
// model's state after it, flattened to keys such as "tab.A.shows". TLC and Java are the ones the
// pinned statechart skill uses.
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";

const here = path.dirname(new URL(import.meta.url).pathname);

function tlc() {
  const { TLA2TOOLS, JAVA } = process.env;
  if (TLA2TOOLS) return { jar: TLA2TOOLS, java: JAVA || "java" };
  const run = fs.readFileSync(path.join(here, "../.agents/skills/statechart/scripts/run.ts"), "utf8");
  const pin = (name) => run.match(new RegExp(`const ${name} =\\s*"([^"]+)"`))?.[1];
  const [tool, java] = [pin("TLC_TOOL"), pin("JAVA_TOOL")];
  if (!tool || !java) throw new Error("the statechart skill's run.ts no longer names TLC_TOOL and JAVA_TOOL");
  const mise = (...args) => spawnSync("mise", args, { encoding: "utf8" });
  let where = mise("where", tool), which = mise("which", "java", "--tool", java);
  if (where.status || which.status) {
    mise("install", java, tool);
    where = mise("where", tool);
    which = mise("which", "java", "--tool", java);
  }
  if (where.status || which.status) throw new Error("Cannot find TLC: run the statechart skill once, or set TLA2TOOLS and JAVA");
  return { jar: path.join(where.stdout.trim(), "tla2tools.jar"), java: JAVA || which.stdout.trim() };
}

// TLC's values: strings, numbers, booleans, records, functions, sequences and sets.
function parse(text) {
  let i = 0;
  const space = () => { while (/\s/.test(text[i])) i++; };
  const take = (s) => { space(); if (!text.startsWith(s, i)) throw new Error(`expected ${s} at ${text.slice(i, i + 40)}`); i += s.length; };
  const peek = (s) => { space(); return text.startsWith(s, i); };
  const list = (end, item) => { const out = []; while (!peek(end)) { out.push(item()); if (peek(",")) take(","); } take(end); return out; };
  const value = () => {
    space();
    if (text[i] === '"') { const m = text.slice(i).match(/^"((?:[^"\\]|\\.)*)"/); i += m[0].length; return JSON.parse(`"${m[1]}"`); }
    if (peek("<<")) { take("<<"); return list(">>", value); }
    if (peek("[")) { take("["); return Object.fromEntries(list("]", () => { const m = text.slice(i).match(/^\s*(\w+)\s*\|->/); i += m[0].length; return [m[1], value()]; })); }
    if (peek("{")) { take("{"); return list("}", value); }
    if (peek("(")) {
      take("(");
      const out = {};
      for (;;) { const k = value(); take(":>"); out[k] = value(); if (peek("@@")) take("@@"); else break; }
      take(")");
      return out;
    }
    const m = text.slice(i).match(/^(TRUE|FALSE|-?\d+)/);
    if (!m) throw new Error(`can't read TLC value at ${text.slice(i, i + 40)}`);
    i += m[0].length;
    return m[1] === "TRUE" ? true : m[1] === "FALSE" ? false : Number(m[1]);
  };
  const v = value();
  space();
  if (i !== text.length) throw new Error(`unread TLC value: ${text.slice(i, i + 40)}`);
  return v;
}

// Each trace TLC printed, as states of variable → value.
function traces(output) {
  const out = [];
  for (const block of output.split(/^Error: The behavior up to this point is:$/m).slice(1)) {
    const states = [];
    for (const chunk of block.split(/^State \d+: .*$/m).slice(1)) {
      const body = chunk.split(/\n\s*\n/)[0];
      states.push(Object.fromEntries(body.split(/^\/\\ /m).slice(1).map((line) => {
        const [, name, value] = line.match(/^(\w+) = ([\s\S]*)$/);
        return [name, parse(value.trim())];
      })));
    }
    out.push(states);
  }
  return out;
}

const flat = (value, prefix, out = {}) => {
  if (value && typeof value === "object" && !Array.isArray(value))
    for (const [k, v] of Object.entries(value)) flat(v, prefix ? `${prefix}.${k}` : k, out);
  else out[prefix] = value;
  return out;
};
const stateOf = (s) => { const { event, step, ...rest } = s; return flat(rest, ""); };
const eventOf = (e) => (e.tab ? `${e.name} in tab ${e.tab}` : e.name);
const tla = (s) => JSON.stringify(s);

// TLC's exit code doesn't say whether a run finished, so read what it printed: it must finish,
// report no error but the walks' own invariant, and print what `complete` asks for.
function run(spec, wrapper, cfg, args, complete) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "walks-"));
  fs.writeFileSync(path.join(dir, "Walk.tla"), `---- MODULE Walk ----\nEXTENDS ${path.basename(spec, ".tla")}, Naturals, Sequences, TLC\n${wrapper}\n====\n`);
  fs.writeFileSync(path.join(dir, "Walk.cfg"), cfg);
  const { java, jar } = tlc();
  const r = spawnSync(java, [`-DTLA-Library=${path.dirname(path.resolve(spec))}`, "-XX:+UseParallelGC", "-cp", jar, "tlc2.TLC",
    "-workers", "1", "-metadir", path.join(dir, "meta"), "-config", "Walk.cfg", ...args, "Walk.tla"],
    { cwd: dir, encoding: "utf8", maxBuffer: 1 << 30 });
  const stray = (r.stdout.match(/^Error: .*$/gm) ?? [])
    .filter((e) => !/^Error: (Invariant \w+ is violated(\.| by the initial state:)|The behavior up to this point is:)$/.test(e));
  const found = traces(r.stdout);
  fs.writeFileSync(path.join(dir, "tlc.out"), r.stdout + r.stderr);
  const why = r.signal ? `TLC was stopped by ${r.signal}` : !/^Finished in /m.test(r.stdout) ? "TLC did not finish"
    : stray.length ? stray.join("\n") : !complete(r.stdout, found) ? `TLC finished without the walks asked for (${found.length} traces)` : "";
  if (why) throw new Error(`${why}; TLC's output is in ${path.join(dir, "tlc.out")}`);
  return found;
}

function main(argv) {
  const [spec, ...rest] = argv;
  const flag = (name, fallback) => { const k = rest.indexOf(name); return k < 0 ? fallback : rest[k + 1]; };
  if (!spec?.endsWith(".tla")) { console.error("Usage: node verification/walks.mjs SPEC.tla [--walks N] [--depth D] [--seed S] [--skip EVENT]... | --paths JSON"); return 2; }
  let walks;
  if (flag("--paths")) {
    walks = JSON.parse(flag("--paths")).map((events, n) => {
      const steps = events.map((e) => { const m = e.match(/^(.*) in tab (\w+)$/); return m ? { name: m[1], tab: m[2] } : { name: e, tab: "" }; });
      const path = `<<${steps.map((s) => `[name |-> ${tla(s.name)}, tab |-> ${tla(s.tab)}]`).join(", ")}>>`;
      const found = run(spec, [
        "VARIABLE step",
        `Path == ${path}`,
        "WalkInit == Init /\\ step = 0",
        "WalkNext == step < Len(Path) /\\ Next /\\ event' = Path[step + 1] /\\ step' = step + 1",
        "Unfinished == step < Len(Path)",
      ].join("\n"), "INIT WalkInit\nNEXT WalkNext\nINVARIANT Unfinished\n", ["-deadlock"],
      (_, found) => found.length < 2 && found.every((w) => w.length === steps.length + 1));
      if (!found.length) throw new Error(`path ${n + 1} can't happen in the model: ${events.join(" → ")}`);
      return found[0];
    });
  } else {
    const skip = `{${rest.flatMap((a, k) => (rest[k - 1] === "--skip" ? [tla(a)] : [])).join(", ")}}`;
    const depth = Number(flag("--depth", 12));
    const view = fs.readFileSync(spec.replace(/\.tla$/, ".cfg"), "utf8").match(/^VIEW\s+(\w+)/m)?.[1];
    if (!view) throw new Error("the spec's config needs a VIEW that leaves `event` out");
    // Breadth first, so the first step to each value of Cover ends the shortest walk to it. TLC
    // checks an invariant only in a state it hasn't seen, so the view tells states apart by Cover
    // too: a step back to a state already seen still counts.
    const first = run(spec, [
      "ASSUME TLCSet(1, {})",
      `FirstNext == Next /\\ event'.name \\notin ${skip}`,
      `Cover == ${flag("--cover", "event")}`,
      `CoverView == <<${view}, Cover>>`,
      "Seen == Cover \\in TLCGet(1) \\/ (TLCSet(1, TLCGet(1) \\cup {Cover}) /\\ FALSE)",
    ].join("\n"), "INIT Init\nNEXT FirstNext\nVIEW CoverView\nINVARIANT Seen\n", ["-continue"],
    (out) => /^\d+ states generated, \d+ distinct states found, 0 states left on queue\.$/m.test(out)).filter((w) => w.length > 1);
    walks = first.concat(run(spec, [
      "VARIABLE step",
      "WalkInit == Init /\\ step = 0",
      `WalkNext == Next /\\ event'.name \\notin ${skip} /\\ step' = step + 1`,
      `Short == step < ${depth}`,
    ].join("\n"), "INIT WalkInit\nNEXT WalkNext\nINVARIANT Short\n",
    ["-simulate", `num=${flag("--walks", 100)}`, "-depth", String(depth + 1), "-seed", flag("--seed", "1"), "-continue"],
    (_, found) => found.length === Number(flag("--walks", 100)) && found.every((w) => w.length === depth + 1)));
  }
  const start = stateOf(walks[0][0]);
  process.stdout.write(JSON.stringify({ start, walks: walks.map((w) => w.slice(1).map((s) => ({ event: eventOf(s.event), state: stateOf(s) }))) }));
  return 0;
}

process.exitCode = main(process.argv.slice(2));
