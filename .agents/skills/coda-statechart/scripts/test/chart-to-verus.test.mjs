// These tests use the CLI and real pinned Verus, not a JavaScript imitation of it.
import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";

const root = fileURLToPath(new URL("../../../../../", import.meta.url));
const compiler = fileURLToPath(new URL("../chart-to-verus.mjs", import.meta.url));
const example = fs.readFileSync(new URL("../../examples/sync.mmd", import.meta.url), "utf8");
const lookup = spawnSync("python3", ["-c", "import runpy,json; c=runpy.run_path('verification/check.py'); print(json.dumps([str(c['VERUS']), c['LOCK']['version']]))"], { cwd: root, encoding: "utf8" });
assert.equal(lookup.status, 0, lookup.stderr);
const [verus, version] = JSON.parse(lookup.stdout);
const versionResult = spawnSync(verus, ["--version"], { encoding: "utf8" });
assert.equal(versionResult.status, 0, "Install the pinned verifier with verification/install-verus.py");
assert.ok(versionResult.stdout.includes(version), "the verifier must match verification/verus.json");
const run = fs.mkdtempSync(path.join(root, "target/verification/runs/chart-compiler-"));
console.log(`Compiler evidence: ${run}`);

function fixture(text) {
  const dir = fs.mkdtempSync(path.join(run, "case-"));
  const input = path.join(dir, "chart.mmd");
  fs.writeFileSync(input, text);
  return { dir, input, model: path.join(dir, "Chart.rs"), proof: path.join(dir, "Chart_proof.rs"), map: path.join(dir, "Chart.map.json") };
}
function generate(f, extra = []) {
  const result = spawnSync(process.execPath, [compiler, f.input, "--module", "Chart", ...extra], { encoding: "utf8" });
  return { ...result, text: result.stdout + result.stderr };
}
function generated(text) {
  const f = fixture(text);
  const result = generate(f);
  assert.equal(result.status, 0, result.text);
  return f;
}
function verify(f, shouldPass = true, obligation) {
  const result = spawnSync(verus, [f.proof, "--no-cheating"], { encoding: "utf8", timeout: 30000 });
  const output = `${result.stdout ?? ""}${result.stderr ?? ""}`;
  fs.writeFileSync(path.join(f.dir, "verus.log"), output);
  assert.ifError(result.error);
  if (shouldPass) {
    assert.equal(result.status, 0, output);
    assert.match(output, /verification results:: [1-9][0-9]* verified, 0 errors/);
  } else {
    assert.notEqual(result.status, 0, "the contradictory chart must fail verification");
    assert.match(output, /postcondition not satisfied/, output);
    assert.match(output, /verification results:: \d+ verified, [1-9][0-9]* errors/);
    assert.ok(output.includes(obligation), output);
  }
}
// The correspondence map gives public generated names. The expected behavior
// below is written independently from the compiler, in the fixture's vocabulary.
function names(f) {
  const map = JSON.parse(fs.readFileSync(f.map, "utf8"));
  return {
    state(...captions) {
      return `State { ${map.regions.map((r, i) => {
        const state = r.states.find((s) => s.caption === captions[i]);
        assert.ok(state, `${captions[i]} absent from region ${r.caption}`);
        return `${r.field}: ${state.variant}`;
      }).join(", ")} }`;
    },
    event(label) {
      const event = map.events.find((e) => e.label === label);
      assert.ok(event, `${label} missing`);
      return event.function;
    },
  };
}
function addProof(f, body) {
  fs.appendFileSync(f.proof, `\nverus! {\npub proof fn translation_contract() {\n${body}\n}\n}\n`);
}

test("broadcasts, state guards, and finite-trace safety verify on the actual generated model", () => {
  const f = generated(example);
  const { state: s, event: e } = names(f);
  addProof(f, `
    assert forall|a: State, b: State| ${e("the network drops")}(a, b) == (
        (a == ${s("Online", "Synced")} && b == ${s("Offline", "Waiting to sync")})
        || (a == ${s("Online", "Waiting to sync")} && b == ${s("Offline", "Waiting to sync")})
        || (a == ${s("Offline", "Synced")} && b == ${s("Offline", "Waiting to sync")})
    ) by {};
    assert(init(${s("Online", "Synced")}));
    assert(${e("the network drops")}(${s("Online", "Synced")}, ${s("Offline", "Waiting to sync")}));
    assert(!${e("the network drops")}(${s("Online", "Synced")}, ${s("Offline", "Synced")}));
    assert(!${e("the server confirms")}(${s("Offline", "Waiting to sync")}, ${s("Offline", "Synced")}));
    assert(${e("the server confirms")}(${s("Online", "Waiting to sync")}, ${s("Online", "Synced")}));
    assert(${e("the network returns")}(${s("Offline", "Waiting to sync")}, ${s("Online", "Waiting to sync")}));
    assert(step(${s("Online", "Synced")}, ${s("Online", "Synced")}));
  `);
  verify(f);
});

test("removing the diagram guard fails the generated transition obligation, not compilation", () => {
  const f = generated(example.replace(" [while online]", "").replace("      %% when Online\n", ""));
  verify(f, false, "step_preserves_inductive");
  const trace = generate(f, ["--trace"]);
  assert.equal(trace.status, 1, trace.text);
  assert.match(trace.text, /Shortest path: 2 events/);
  assert.match(trace.text, /2\. the network drops/);
  assert.match(trace.text, /3\. the server confirms/);
  assert.match(trace.text, /flowchart LR/);
});

test("a forbidden initial configuration fails the initialization obligation", () => {
  const f = generated(example.replace("[*] --> Online", "[*] --> Offline"));
  verify(f, false, "initial_is_inductive");
  const trace = generate(f, ["--trace"]);
  assert.equal(trace.status, 1, trace.text);
  assert.match(trace.text, /Shortest path: 0 events/);
});

test("hierarchical entry, parent exits, and the deepest same-label transition survive translation", () => {
  const f = generated(`stateDiagram-v2
state Busy {
  [*] --> Sending
  Sending --> Checking : bytes arrive
  Checking --> Done : cancel
}
state Done
[*] --> Idle
Idle --> Busy : start
Busy --> Failed : disconnect
Busy --> Idle : cancel
Failed --> Busy : retry
`);
  const { state: s, event: e } = names(f);
  addProof(f, `
    assert(${e("start")}(${s("Idle")}, ${s("Busy.Sending")}));
    assert(!${e("start")}(${s("Idle")}, ${s("Busy.Checking")}));
    assert(${e("disconnect")}(${s("Busy.Sending")}, ${s("Failed")}));
    assert(${e("disconnect")}(${s("Busy.Checking")}, ${s("Failed")}));
    assert(${e("cancel")}(${s("Busy.Sending")}, ${s("Idle")}));
    assert(${e("cancel")}(${s("Busy.Checking")}, ${s("Done")}));
    assert(!${e("cancel")}(${s("Busy.Checking")}, ${s("Idle")}));
    assert(${e("retry")}(${s("Failed")}, ${s("Busy.Sending")}));
  `);
  verify(f);
});

test("shared events read pre-state guards and negative guards exclude every listed region", () => {
  const f = generated(`stateDiagram-v2
state App {
  state Power {
    [*] --> Off
    Off --> On : switch
  }
  --
  state Lamp {
    [*] --> Dark
    Dark --> Lit : switch [when off]
    %% when Off
    Lit --> Dark : reset [when neither power nor backup is on]
    %% when not On, BackupOn
    Dark --> Lit : activate [when both power and backup are on]
    %% when On, BackupOn
    Dark --> Lit : backup lighting [in either power state with backup on]
    %% when Off, On, BackupOn
  }
  --
  state Backup {
    [*] --> BackupOff
    BackupOff --> BackupOn : backup starts
  }
}
[*] --> App
`);
  const { state: s, event: e } = names(f);
  addProof(f, `
    assert(${e("switch")}(${s("Off", "Dark", "BackupOff")}, ${s("On", "Lit", "BackupOff")}));
    assert(!${e("switch")}(${s("Off", "Dark", "BackupOff")}, ${s("On", "Dark", "BackupOff")}));
    assert(${e("switch")}(${s("Off", "Lit", "BackupOff")}, ${s("On", "Lit", "BackupOff")}));
    assert(!${e("switch")}(${s("On", "Dark", "BackupOff")}, ${s("On", "Lit", "BackupOff")}));
    assert(${e("reset")}(${s("Off", "Lit", "BackupOff")}, ${s("Off", "Dark", "BackupOff")}));
    assert(!${e("reset")}(${s("On", "Lit", "BackupOff")}, ${s("On", "Dark", "BackupOff")}));
    assert(!${e("reset")}(${s("Off", "Lit", "BackupOn")}, ${s("Off", "Dark", "BackupOn")}));
    assert(${e("activate")}(${s("On", "Dark", "BackupOn")}, ${s("On", "Lit", "BackupOn")}));
    assert(!${e("activate")}(${s("Off", "Dark", "BackupOn")}, ${s("Off", "Lit", "BackupOn")}));
    assert(!${e("activate")}(${s("On", "Dark", "BackupOff")}, ${s("On", "Lit", "BackupOff")}));
    assert(${e("backup lighting")}(${s("Off", "Dark", "BackupOn")}, ${s("Off", "Lit", "BackupOn")}));
    assert(${e("backup lighting")}(${s("On", "Dark", "BackupOn")}, ${s("On", "Lit", "BackupOn")}));
  `);
  verify(f);
});

test("multiple starts, nondeterministic choices, unlabeled steps, and self loops remain available", () => {
  const f = generated(`stateDiagram-v2
[*] --> A
[*] --> B
A --> B : choose
A --> C : choose
B --> C
C --> C : refresh
`);
  const { state: s, event: e } = names(f);
  addProof(f, `
    assert(init(${s("A")})); assert(init(${s("B")})); assert(!init(${s("C")}));
    assert(${e("choose")}(${s("A")}, ${s("B")}));
    assert(${e("choose")}(${s("A")}, ${s("C")}));
    assert(!${e("choose")}(${s("B")}, ${s("C")}));
    assert(next(${s("B")}, ${s("C")}));
    assert(${e("refresh")}(${s("C")}, ${s("C")}));
    assert(!next(${s("C")}, ${s("A")}));
  `);
  verify(f);
});

test("a safe design with a noninductive safety rule can be proved by strengthening the editable invariant", () => {
  const f = generated(`stateDiagram-v2
state App {
  state Flow {
    [*] --> Ready
    Ready --> Bad : finish [when armed]
    %% when Armed
  }
  --
  state Gate {
    [*] --> Disarmed
    state Armed
  }
}
[*] --> App
%% never NoBad: Bad -- Bad is unreachable.
`);
  verify(f, false, "step_preserves_inductive");
  const trace = generate(f, ["--trace"]);
  assert.equal(trace.status, 0, trace.text);
  assert.match(trace.text, /Exhausted 1 reachable chart states/);
  const map = JSON.parse(fs.readFileSync(f.map, "utf8"));
  const r = map.regions.find((r) => r.caption === "Gate");
  const disarmed = r.states.find((s) => s.caption === "Disarmed");
  fs.writeFileSync(f.proof, fs.readFileSync(f.proof, "utf8").replace(
    "{ safety(s) }", `{ safety(s) && s.${r.field} == ${disarmed.variant} }`));
  verify(f);
});

test("a rule naming a composite forbids all of its nested states", () => {
  const f = generated(`stateDiagram-v2
[*] --> Idle
state Busy {
  [*] --> Sending
  Sending --> Checking : bytes arrive
}
%% never NoBusy: Busy -- This flow never starts work.
`);
  const { state: s } = names(f);
  addProof(f, `
    assert(safety(${s("Idle")}));
    assert(!safety(${s("Busy.Sending")}));
    assert(!safety(${s("Busy.Checking")}));
  `);
  verify(f);
});

test("chart drift is detected, regeneration preserves proof edits, and edited generated models are refused", () => {
  const f = generated(example);
  assert.equal(generate(f, ["--check"]).status, 0);
  fs.appendFileSync(f.proof, "\n// User-owned proof notes.\n");
  const proof = fs.readFileSync(f.proof, "utf8");
  const previous = fs.readFileSync(f.model, "utf8");
  fs.writeFileSync(f.input, example.replace("you edit a note", "you change a note"));
  assert.equal(generate(f, ["--check"]).status, 1);
  assert.equal(fs.readFileSync(f.model, "utf8"), previous, "checking must not regenerate");
  assert.equal(generate(f).status, 0);
  assert.equal(fs.readFileSync(f.proof, "utf8"), proof);
  assert.equal(generate(f, ["--check"]).status, 0);
  fs.appendFileSync(f.model, "\n// Manual refinement must be preserved.\n");
  const edited = fs.readFileSync(f.model, "utf8");
  assert.equal(generate(f).status, 1);
  assert.equal(fs.readFileSync(f.model, "utf8"), edited);
});

test("ambiguous or unsupported semantics fail before writing any model", () => {
  const cases = [
    "flowchart TD\nA --> B",
    "stateDiagram-v2\n[*] --> A\nA --> B : go [if ready]",
    "stateDiagram-v2\n[*] --> A\n%% never Broken",
    "stateDiagram-v2\n[*] --> A\nA --> [*] : close",
    "stateDiagram-v2\nstate Busy {\nA --> B : go\n}\n[*] --> Busy",
    "stateDiagram-v2\n[*] --> A\n%% never Bad: Missing",
    "stateDiagram-v2\n[*] --> A\nA --> B : go\n%% never Bad: A & B",
    "stateDiagram-v2\n[*] --> A\n%% never Bad: A\n%% never Bad: A",
    "stateDiagram-v2\n[*] --> A : start",
    "stateDiagram-v2\n[*] --> A\n--\n[*] --> B",
    `stateDiagram-v2
state App {
 state One {
  [*] --> A
 }
 --
 state Two {
  [*] --> B
  state A
 }
}
[*] --> App`,
  ];
  for (const text of cases) {
    const f = fixture(text);
    const result = generate(f);
    assert.equal(result.status, 1, `${text}\n${result.text}`);
    assert.ok(!fs.existsSync(f.model));
    assert.ok(!fs.existsSync(f.map));
    assert.ok(!fs.existsSync(f.proof));
  }
});

test("CLI reports invalid arguments without starting generation", () => {
  for (const args of [[], ["--module"], ["c.mmd", "--module", "../escape"], ["c.mmd", "--unknown"]]) {
    const result = spawnSync(process.execPath, [compiler, ...args], { encoding: "utf8" });
    assert.equal(result.status, 2, result.stderr);
  }
});


test("search is read-only, exhaustive when completed, and never passes at a state limit", () => {
  const f = fixture(example);
  const complete = generate(f, ["--trace"]);
  assert.equal(complete.status, 0, complete.text);
  assert.match(complete.text, /Exhausted 3 reachable chart states/);
  assert.deepEqual(fs.readdirSync(f.dir), ["chart.mmd"]);
  const limited = generate(f, ["--trace", "--max-states", "1"]);
  assert.equal(limited.status, 3, limited.text);
  assert.match(limited.text, /INCOMPLETE/);
  for (const limit of ["0", "-1", "1.5", "9007199254740992"]) {
    assert.equal(generate(f, ["--trace", "--max-states", limit]).status, 2);
  }
});

test("shortest paths cover alternative starts, nondeterminism and nested entry", () => {
  const f = generated(`stateDiagram-v2
[*] --> Far
[*] --> Near
Far --> Middle : walk
Middle --> Near : continue
Near --> Safe : choose
Near --> Busy : choose
state Busy {
 [*] --> Sending
 Sending --> Checking : inspect
}
%% never NoBusy: Busy -- Starting this work violates the promise.
`);
  const trace = generate(f, ["--trace"]);
  assert.equal(trace.status, 1, trace.text);
  assert.match(trace.text, /Shortest path: 1 events/);
  assert.match(trace.text, /1\. Start → .*Near/);
  assert.match(trace.text, /2\. choose → .*Busy.Sending/);
  verify(f, false, "step_preserves_inductive");
});

test("search guards read the pre-state and every enabled broadcast region moves", () => {
  const f = generated(`stateDiagram-v2
state App {
 state Gate {
  [*] --> Off
  Off --> On : switch
 }
 --
 state Light {
  [*] --> Dark
  Dark --> Lit : switch [while off]
  %% when Off
 }
}
[*] --> App
%% never NoHalfSwitch: On & Dark -- The event changes both enabled regions.
`);
  const trace = generate(f, ["--trace"]);
  assert.equal(trace.status, 0, trace.text);
  assert.match(trace.text, /Exhausted 2 reachable chart states/);
  verify(f);
});
