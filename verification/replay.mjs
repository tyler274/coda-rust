#!/usr/bin/env node
// Replays paths through a feature's TLA+ model, for its browser test to play against the real page.
//   node verification/replay.mjs SPEC.tla PATHS
//
// PATHS is JSON, a list of paths, each a list of events such as "open Coda in tab A". The spec
// names each step in an `event` variable, a record with `name` and, for a tab's step, `tab`. TLC
// takes each path's steps and fails when one can't happen in the model. Prints, as JSON, the start
// state and each path's steps, each with its event and the model's state after it, flattened to
// keys such as "tab.A.shows", each value as text. TLC and Java are the ones the pinned statechart skill uses.
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
  if (where.status !== 0 || which.status !== 0) {
    mise("install", java, tool);
    where = mise("where", tool);
    which = mise("which", "java", "--tool", java);
  }
  if (where.status !== 0 || which.status !== 0) throw new Error("Cannot find TLC: run the statechart skill once, or set TLA2TOOLS and JAVA");
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
  else out[prefix] = typeof value === "string" ? value : JSON.stringify(value);
  return out;
};
const stateOf = (s) => { const { event, step, ...rest } = s; return flat(rest, ""); };
const eventOf = (e) => (e.tab ? `${e.name} in tab ${e.tab}` : e.name);
const tla = (s) => JSON.stringify(s);

// The states along one path, from TLC. TLC's exit code doesn't say whether a run finished, so read
// what it printed: it must finish, report no error but the path's own invariant, and print at most
// one trace, the whole path. None means the path can't happen in the model.
function replay(spec, events, n) {
  const steps = events.map((e) => { const m = e.match(/^(.*) in tab (\w+)$/); return m ? { name: m[1], tab: m[2] } : { name: e, tab: "" }; });
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "replay-"));
  fs.writeFileSync(path.join(dir, "Path.tla"), [
    "---- MODULE Path ----",
    `EXTENDS ${path.basename(spec, ".tla")}, Naturals, Sequences, TLC`,
    "VARIABLE step",
    `Steps == <<${steps.map((s) => `[name |-> ${tla(s.name)}, tab |-> ${tla(s.tab)}]`).join(", ")}>>`,
    "PathInit == Init /\\ step = 0",
    "PathNext == step < Len(Steps) /\\ Next /\\ event' = Steps[step + 1] /\\ step' = step + 1",
    "Unfinished == step < Len(Steps)",
    "====",
  ].join("\n"));
  // The feature's own config sets the model's scope.
  const scope = fs.readFileSync(spec.replace(/\.tla$/, ".cfg"), "utf8")
    .match(/^CONSTANTS?\b[\s\S]*?(?=^[A-Z_]+\b(?<!CONSTANTS?)|(?![\s\S]))/m)?.[0] ?? "";
  fs.writeFileSync(path.join(dir, "Path.cfg"), `${scope}INIT PathInit\nNEXT PathNext\nINVARIANT Unfinished\n`);
  const { java, jar } = tlc();
  const r = spawnSync(java, [`-DTLA-Library=${path.dirname(path.resolve(spec))}`, "-XX:+UseParallelGC", "-cp", jar, "tlc2.TLC",
    "-workers", "1", "-metadir", path.join(dir, "meta"), "-config", "Path.cfg", "-deadlock", "Path.tla"],
    { cwd: dir, encoding: "utf8", maxBuffer: 1 << 30 });
  fs.writeFileSync(path.join(dir, "tlc.out"), r.stdout + r.stderr);
  const stray = (r.stdout.match(/^Error: .*$/gm) ?? [])
    .filter((e) => !/^Error: (Invariant Unfinished is violated\.|The behavior up to this point is:)$/.test(e));
  const found = traces(r.stdout);
  const why = r.signal ? `TLC was stopped by ${r.signal}` : !/^Finished in /m.test(r.stdout) ? "TLC did not finish"
    : stray.length ? stray.join("\n")
    : found.length > 1 || found.some((t) => t.length !== steps.length + 1) ? `TLC printed ${found.length} traces, not the path`
    : !found.length ? `path ${n + 1} can't happen in the model: ${events.join(" → ")}` : "";
  if (why) throw new Error(`${why}; TLC's output is in ${path.join(dir, "tlc.out")}`);
  return found[0];
}

function main(argv) {
  const [spec, json] = argv;
  if (!spec?.endsWith(".tla") || !json) { console.error("Usage: node verification/replay.mjs SPEC.tla PATHS"); return 2; }
  const paths = JSON.parse(json).map((events, n) => replay(spec, events, n));
  const start = stateOf(paths[0][0]);
  process.stdout.write(JSON.stringify({ start, paths: paths.map((p) => p.slice(1).map((s) => ({ event: eventOf(s.event), state: stateOf(s) }))) }));
  return 0;
}

process.exitCode = main(process.argv.slice(2));
