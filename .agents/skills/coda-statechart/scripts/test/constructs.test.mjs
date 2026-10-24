// Authoring constructs (nested regions, templates, declared events, effects) and the graph
// tools, checked through the CLI and the real pinned Verus, like chart-to-verus.test.mjs.
// Expected behavior is written independently from the compiler, in each fixture's vocabulary.
import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";

const root = fileURLToPath(new URL("../../../../../", import.meta.url));
const cli = fileURLToPath(new URL("../statechart.mjs", import.meta.url));
const here = (name) => fs.readFileSync(new URL(name, import.meta.url), "utf8");
const lookup = spawnSync("python3", ["-c", "import runpy,json; c=runpy.run_path('verification/check.py'); print(json.dumps([str(c['VERUS']), c['LOCK']['version']]))"], { cwd: root, encoding: "utf8" });
assert.equal(lookup.status, 0, lookup.stderr);
const [verus, version] = JSON.parse(lookup.stdout);
const versionResult = spawnSync(verus, ["--version"], { encoding: "utf8" });
assert.equal(versionResult.status, 0, "Install the pinned verifier with verification/install-verus.py");
assert.ok(versionResult.stdout.includes(version), "the verifier must match verification/verus.json");
fs.mkdirSync(path.join(root, "target/verification/runs"), { recursive: true });
const run = fs.mkdtempSync(path.join(root, "target/verification/runs/chart-constructs-"));
console.log(`Constructs evidence: ${run}`);

function fixture(text) {
  const dir = fs.mkdtempSync(path.join(run, "case-"));
  const input = path.join(dir, "chart.mmd");
  fs.writeFileSync(input, text);
  return { dir, input, proof: path.join(dir, "Chart_proof.rs"), map: path.join(dir, "Chart.map.json"), model: path.join(dir, "Chart.rs") };
}
function cmd(...args) {
  const result = spawnSync(process.execPath, [cli, ...args], { encoding: "utf8" });
  return { ...result, text: result.stdout + result.stderr };
}
function generated(text) {
  const f = fixture(text);
  const result = cmd("verus", f.input, "--module", "Chart");
  assert.equal(result.status, 0, result.text);
  return f;
}
function verify(f, shouldPass = true, obligation) {
  const result = spawnSync(verus, [f.proof, "--no-cheating"], { encoding: "utf8", timeout: 120000 });
  const output = `${result.stdout ?? ""}${result.stderr ?? ""}`;
  fs.writeFileSync(path.join(f.dir, "verus.log"), output);
  assert.ifError(result.error);
  if (shouldPass) {
    assert.equal(result.status, 0, output);
    assert.match(output, /verification results:: [1-9][0-9]* verified, 0 errors/);
  } else {
    assert.notEqual(result.status, 0, "the broken variant must fail verification");
    assert.match(output, /verification results:: \d+ verified, [1-9][0-9]* errors/);
    assert.ok(output.includes(obligation), output);
  }
}
// Public generated names from the map; a state is given by captions in region order.
function names(f) {
  const map = JSON.parse(fs.readFileSync(f.map, "utf8"));
  return {
    map,
    state(...captions) {
      assert.equal(captions.length, map.regions.length, `regions: ${map.regions.map((r) => r.caption).join(" | ")}`);
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
function addProof(f, body, name = "translation_contract") {
  fs.appendFileSync(f.proof, `\nverus! {\npub proof fn ${name}() {\n${body}\n}\n}\n`);
}
// Cross-check the two backends: for every state the search reaches and every event, Verus
// must prove that the generated relation allows exactly the successors the search enumerates.
async function agreement(f) {
  const { compile } = await import("../chart-verus.mjs");
  const { stepper } = await import("../chart-search.mjs");
  const { ir } = compile(fs.readFileSync(f.input, "utf8"), "Chart");
  const { post, starts } = stepper(ir);
  const { state } = names(f);
  const scope = ir.regions.map((_, i) => i);
  const seen = new Map();
  const queue = [...starts(scope)];
  for (const s of queue) seen.set(JSON.stringify(s), true);
  const lines = [];
  for (let i = 0; i < queue.length; i++) {
    const s = queue[i];
    for (const action of ir.actions) {
      const nexts = [...post(action, scope, (ri) => s[ri])];
      for (const n of nexts) if (!seen.has(JSON.stringify(n))) { seen.set(JSON.stringify(n), true); queue.push(n); }
      const unique = [...new Set(nexts.map((n) => state(...n)))];
      lines.push(`    assert forall|b: State| ${action.function}(${state(...s)}, b) == (${unique.map((u) => `b == ${u}`).join(" || ") || "false"}) by {};`);
    }
  }
  addProof(f, lines.join("\n"), "search_and_verus_agree");
  return queue.length;
}
function refused(text, pattern) {
  const f = fixture(text);
  const result = cmd("verus", f.input, "--module", "Chart");
  assert.equal(result.status, 1, `${text}\n${result.text}`);
  assert.match(result.text, pattern);
  assert.ok(!fs.existsSync(f.model) && !fs.existsSync(f.map) && !fs.existsSync(f.proof));
}

const door = here("./door.mmd");

test("nested parallel regions: entry, labeled start, deep target, exit wins, re-entry and exits from a part", () => {
  const f = generated(door);
  const { state, event: e, map } = names(f);
  assert.deepEqual(map.regions.map((r) => r.caption), ["Door", "Door/Locked/Alarm", "Door/Locked/Keypad", "Siren"]);
  const off = "(not in Locked)";
  const s = (door, alarm, keypad, siren) => state(door, alarm, keypad, siren);
  addProof(f, `
    assert(init(${s("Open", off, off, "Quiet")}));
    assert(!init(${s("Open", "Disarmed", off, "Quiet")}));
    assert forall|b: State| ${e("you lock the door")}(${s("Open", off, off, "Quiet")}, b) == (b == ${s("Locked", "Disarmed", "Idle", "Quiet")}) by {};
    assert forall|b: State| ${e("the timer locks the door")}(${s("Open", off, off, "Quiet")}, b) == (b == ${s("Locked", "Armed", "Idle", "Quiet")}) by {};
    assert forall|b: State| ${e("you lock and arm")}(${s("Open", off, off, "Quiet")}, b) == (b == ${s("Locked", "Armed", "Idle", "Quiet")}) by {};
    assert(${e("you press a key")}(${s("Locked", "Disarmed", "Idle", "Quiet")}, ${s("Locked", "Disarmed", "Typing", "Quiet")}));
    assert forall|b: State| ${e("you unlock the door")}(${s("Locked", "Disarmed", "Typing", "Quiet")}, b) == (b == ${s("Open", off, off, "Quiet")}) by {};
    assert forall|b: State| ${e("you reset the lock")}(${s("Locked", "Armed", "Typing", "Loud")}, b) == (b == ${s("Locked", "Disarmed", "Idle", "Quiet")}) by {};
    assert(${e("someone forces the door")}(${s("Locked", "Armed", "Idle", "Quiet")}, ${s("Locked", "Ringing", "Idle", "Loud")}));
    assert(${e("someone opens the door")}(${s("Locked", "Ringing", "Typing", "Loud")}, ${s("Open", off, off, "Loud")}));
    assert(!${e("someone opens the door")}(${s("Locked", "Armed", "Idle", "Quiet")}, ${s("Open", off, off, "Quiet")}));
    assert(safety(${s("Locked", "Ringing", "Idle", "Loud")}));
    assert(!safety(${s("Open", "Ringing", "Idle", "Loud")}));
    assert(!safety(${s("Open", "Disarmed", off, "Quiet")}));
  `);
  verify(f);
  const trace = cmd("trace", f.input);
  assert.equal(trace.status, 0, trace.text);
  assert.match(trace.text, /Exhausted 12 reachable chart states/);
  const scope = map.rules.filter((r) => r.kind === "scope").map((r) => r.region);
  assert.deepEqual(scope, ["Door/Locked/Alarm", "Door/Locked/Keypad"]);
});

test("negative control: the same alarm drawn as an independent region rings while the door is open", () => {
  const flat = `stateDiagram-v2
state App {
 state Door {
  [*] --> Open
  Open --> Locked : you lock the door
  Locked --> Open : you unlock the door
 }
 --
 state Alarm {
  [*] --> Disarmed
  Disarmed --> Armed : you arm the alarm
  Armed --> Ringing : someone forces the door
 }
}
[*] --> App
%% never NoRingingWhileOpen: Open & Ringing -- The alarm never rings while the door stands open.
`;
  const f = generated(flat);
  verify(f, false, "step_preserves_inductive");
  const trace = cmd("trace", f.input);
  assert.equal(trace.status, 1, trace.text);
  assert.match(trace.text, /VIOLATION NoRingingWhileOpen/);
  assert.match(trace.text, /Shortest path: 2 events/);
});

const lamp = `stateDiagram-v2
[*] --> Off
Off --> On : power on
On --> Off : power off
state On {
 state Lamp {
  [*] --> Warm
  Warm --> Cold : flip
  Cold --> Warm : flip
  state Warm {
   state Glow {
    [*] --> Dim
    Dim --> Bright : brighten
   }
   --
   state Fan {
    [*] --> Slow
   }
  }
 }
 --
 state Timer {
  [*] --> Waiting
 }
}
%% never NoBrightWhenOff: Off & Bright -- Nothing glows while the power is off.
`;

test("three levels of parallel nesting inside a single-region chart re-enter and close together", () => {
  const f = generated(lamp);
  const { state: s, event: e, map } = names(f);
  assert.deepEqual(map.regions.map((r) => r.caption), ["Main", "Main/On/Lamp", "Main/On/Lamp/Warm/Glow", "Main/On/Lamp/Warm/Fan", "Main/On/Timer"]);
  const on = "(not in On)", warm = "(not in Warm)";
  addProof(f, `
    assert(init(${s("Off", on, warm, warm, on)}));
    assert forall|b: State| ${e("power on")}(${s("Off", on, warm, warm, on)}, b) == (b == ${s("On", "Warm", "Dim", "Slow", "Waiting")}) by {};
    assert(${e("brighten")}(${s("On", "Warm", "Dim", "Slow", "Waiting")}, ${s("On", "Warm", "Bright", "Slow", "Waiting")}));
    assert forall|b: State| ${e("flip")}(${s("On", "Warm", "Bright", "Slow", "Waiting")}, b) == (b == ${s("On", "Cold", warm, warm, "Waiting")}) by {};
    assert forall|b: State| ${e("flip")}(${s("On", "Cold", warm, warm, "Waiting")}, b) == (b == ${s("On", "Warm", "Dim", "Slow", "Waiting")}) by {};
    assert forall|b: State| ${e("power off")}(${s("On", "Warm", "Bright", "Slow", "Waiting")}, b) == (b == ${s("Off", on, warm, warm, on)}) by {};
  `);
  verify(f);
  const trace = cmd("trace", f.input);
  assert.equal(trace.status, 0, trace.text);
  assert.match(trace.text, /Exhausted 4 reachable chart states/);
});

test("nesting refuses steps that join parts, skip levels, or start inside a part", () => {
  const nested = (inner, outer = "") => `stateDiagram-v2
[*] --> Off
Off --> On : power on
On --> Off : power off
state On {
 state Left {
  [*] --> A
  ${inner}
 }
 --
 state Right {
  [*] --> B
  state B
  state C
 }
}
${outer}`;
  refused(nested("A --> B : jump"), /joins two parallel parts of On/);
  refused(nested("", "B --> C : step"), /stays inside the parallel part Main\/On\/Right; write it inside that part/);
  refused(nested(`state "(not in On)" as Shadow`), /two states have the caption "\(not in On\)"/);
  refused(nested("state D {\n state D1 {\n [*] --> Deep\n }\n --\n state D2 {\n [*] --> Other\n }\n }", "Off --> Deep : dive"), /two parallel levels down/);
  refused("stateDiagram-v2\n[*] --> A\nstate B {\n[*] --> X : go\n}\nA --> B : enter", /an initial arrow cannot have a guard or event label/);
  refused(nested("", "[*] --> Off : boot"), /an initial arrow cannot have a guard or event label/);
  refused(nested("state A", "[*] --> A"), /starts inside a parallel part; start at On/);
});

const parcel = `stateDiagram-v2
state Delivery {
 state Parcel {
  [*] --> Waiting
  Waiting --> Delivered : the courier rings [while the courier is out] / hand the parcel over
  %% when Out
  Waiting --> Porch : the courier leaves a note / leave the parcel on the porch
 }
 --
 state Courier {
  [*] --> Out
  Out --> Visited : the courier rings
  Out --> Visited : the courier leaves a note
 }
 --
 state You {
  [*] --> Home
  Home --> Away : you go out / may change your mind
 }
}
[*] --> Delivery
%% never NoVisitWithoutParcel: Visited & Waiting -- After a visit the parcel is never still waiting.
`;

test("effects: the event name drops guard and effect words, intent is recorded, and may keeps the state", () => {
  const f = generated(parcel);
  const { state: s, event: e, map } = names(f);
  assert.deepEqual(map.events.map((x) => x.label), ["the courier rings", "the courier leaves a note", "you go out"]);
  const rings = map.events[0].transitions.find((t) => t.region === "Parcel");
  assert.equal(rings.intent, "hand the parcel over");
  assert.equal(rings.optional, undefined);
  const out = map.events[2].transitions[0];
  assert.deepEqual([out.intent, out.optional], ["may change your mind", true]);
  addProof(f, `
    assert forall|b: State| ${e("the courier leaves a note")}(${s("Waiting", "Out", "Home")}, b) == (b == ${s("Porch", "Visited", "Home")}) by {};
    assert forall|b: State| ${e("the courier rings")}(${s("Waiting", "Out", "Home")}, b) == (b == ${s("Delivered", "Visited", "Home")}) by {};
    assert(${e("you go out")}(${s("Waiting", "Out", "Home")}, ${s("Waiting", "Out", "Away")}));
    assert(${e("you go out")}(${s("Waiting", "Out", "Home")}, ${s("Waiting", "Out", "Home")}));
    assert(!${e("you go out")}(${s("Waiting", "Out", "Away")}, ${s("Waiting", "Out", "Away")}));
  `);
  verify(f);
  assert.equal(cmd("trace", f.input).status, 0);
});

const restart = `stateDiagram-v2
[*] --> Idle
Idle --> Busy : start
Busy --> Busy : refresh / may restart the work
Busy --> Idle : stop / may keep working
state Busy {
 state Work {
  [*] --> Step1
  [*] --> Step2 : start
  Step1 --> Step2 : refresh
  Step2 --> Step3 : advance
  Step3 --> Idle : finish
 }
 --
 state Log {
  [*] --> Empty
  Empty --> Written : refresh
  Written --> Empty : stop
 }
}
`;

test("search and Verus agree on every reachable step of nested, optional, template and event charts", async () => {
  for (const text of [door, parcel, restart, lamp, here("./lockers.mmd"), here("./lockers-events.mmd")]) {
    const f = generated(text);
    assert.ok(await agreement(f) > 2);
    verify(f);
  }
});

const desk = `stateDiagram-v2
state Desk {
 state Urgency {
  [*] --> Today
  Today --> Now : the review finishes / may raise the urgency
  Now --> Today : the review finishes / may lower the urgency
 }
 --
 state Seen {
  [*] --> New
  New --> Opened : you open it
  Opened --> New : the review finishes / may raise the urgency
 }
 --
 state Log {
  [*] --> Quiet
  Quiet --> Noted : the review finishes
  Noted --> Noted : the review finishes / may raise the urgency
 }
}
[*] --> Desk
`;

test("a may effect worded alike in several regions is one choice: all take it or none do", async () => {
  const f = generated(desk);
  const { state: s, event: e, map } = names(f);
  assert.deepEqual(map.events.find((x) => x.label === "the review finishes").shared_effects, ["may raise the urgency"]);
  addProof(f, `
    assert forall|b: State| ${e("the review finishes")}(${s("Today", "Opened", "Quiet")}, b)
      == (b == ${s("Today", "Opened", "Noted")} || b == ${s("Now", "New", "Noted")}) by {};
    assert forall|b: State| ${e("the review finishes")}(${s("Now", "Opened", "Noted")}, b)
      == (b == ${s("Now", "Opened", "Noted")} || b == ${s("Today", "Opened", "Noted")}
        || b == ${s("Now", "New", "Noted")} || b == ${s("Today", "New", "Noted")}) by {};
  `);
  assert.ok(await agreement(f) > 2);
  verify(f);
});

const plan = `stateDiagram-v2
state Plan {
 state Choice {
  [*] --> Undecided
  Undecided --> Change : you pick
  Undecided --> Cancel : you pick
  Undecided --> Both : you pick
 }
 --
 state Item {
  [*] --> Active
  state Active {
   [*] --> Due
   Due --> Waiting : it goes through [change]
   %% when Change, Both
   Waiting --> Due : it comes due
  }
  Active --> Cancelled : it goes through [cancel]
  %% when Cancel, Both
 }
}
[*] --> Plan
`;

test("a deeper step wins only while its guard holds; otherwise the enclosing step on the event fires", async () => {
  const f = generated(plan);
  const { state: s, event: e } = names(f);
  addProof(f, `
    assert forall|b: State| ${e("it goes through")}(${s("Cancel", "Active.Due")}, b) == (b == ${s("Cancel", "Cancelled")}) by {};
    assert forall|b: State| ${e("it goes through")}(${s("Change", "Active.Due")}, b) == (b == ${s("Change", "Active.Waiting")}) by {};
    assert forall|b: State| ${e("it goes through")}(${s("Both", "Active.Due")}, b) == (b == ${s("Both", "Active.Waiting")}) by {};
    assert forall|b: State| !${e("it goes through")}(${s("Change", "Active.Waiting")}, b) by {};
    assert forall|b: State| ${e("it goes through")}(${s("Both", "Active.Waiting")}, b) == (b == ${s("Both", "Cancelled")}) by {};
  `);
  assert.ok(await agreement(f) > 2);
  verify(f);
});

test("negative control: differently worded effects stay separate choices", () => {
  const f = generated(desk.replace("Opened --> New : the review finishes / may raise the urgency", "Opened --> New : the review finishes / may mark it new"));
  const { state: s, event: e, map } = names(f);
  assert.deepEqual(map.events.find((x) => x.label === "the review finishes").shared_effects, ["may raise the urgency"]);
  addProof(f, `
    assert(${e("the review finishes")}(${s("Today", "Opened", "Quiet")}, ${s("Now", "Opened", "Noted")}));
  `);
  verify(f);
});

test("negative control: an optional reaction breaks the rule the mandatory one keeps", () => {
  const f = generated(parcel.replace("/ leave the parcel", "/ may leave the parcel"));
  const { state: s, event: e } = names(f);
  addProof(f, `
    assert forall|b: State| ${e("the courier leaves a note")}(${s("Waiting", "Out", "Home")}, b)
      == (b == ${s("Porch", "Visited", "Home")} || b == ${s("Waiting", "Visited", "Home")}) by {};
  `);
  verify(f, false, "step_preserves_inductive");
  const trace = cmd("trace", f.input);
  assert.equal(trace.status, 1, trace.text);
  assert.match(trace.text, /2\. the courier leaves a note → Parcel: Waiting; Courier: Visited/);
  refused("stateDiagram-v2\nstate A {\n state B {\n [*] --> X : go / now\n }\n --\n state C {\n [*] --> Y\n }\n}\n[*] --> Z\nZ --> A : go", /an initial arrow cannot have an effect/);
});

const lockers = here("./lockers.mmd");

test("templates expand one region per instance with substitutions, instance references and limits", () => {
  const f = generated(lockers);
  const { state: s, event: e, map } = names(f);
  assert.deepEqual(map.regions.map((r) => [r.caption, r.instance?.key, r.states.map((x) => x.caption)]), [
    ["Locker a (small)", "a", ["Waiting for a small parcel", "Full"]],
    ["Locker b (large)", "b", ["Waiting for a large parcel", "Full", "Jammed"]],
    ["Courier", undefined, ["Driving", "Gone"]],
  ]);
  assert.deepEqual(map.rules.map((r) => [r.name, r.states]), [["NotBothFull_a", ["Full", "Full@b"]], ["NotBothFull_b", ["Full", "Full@a"]]]);
  const a = "Waiting for a small parcel", b = "Waiting for a large parcel";
  addProof(f, `
    assert forall|x: State| ${e("the courier fills locker a")}(${s(a, b, "Driving")}, x) == (x == ${s("Full", b, "Gone")}) by {};
    assert forall|x: State| ${e("the courier fills locker b")}(${s("Full", b, "Driving")}, x) == (x == ${s("Full", b, "Gone")}) by {};
    assert(${e("the door of locker b jams")}(${s(a, "Full", "Gone")}, ${s(a, "Jammed", "Gone")}));
    assert(!${e("you collect from locker a")}(${s("Full", "Full", "Gone")}, ${s(a, "Full", "Driving")}));
    assert(${e("you collect from locker a")}(${s("Full", b, "Gone")}, ${s(a, b, "Driving")}));
    assert(!safety(${s("Full", "Full", "Gone")}));
  `);
  verify(f);
  const trace = cmd("trace", f.input);
  assert.equal(trace.status, 0, trace.text);
  assert.match(trace.text, /Exhausted 5 reachable chart states/);
  const except = generated(lockers.replaceAll("%% only b", "%% except a"));
  assert.deepEqual(JSON.parse(fs.readFileSync(except.map, "utf8")).events, map.events);
});

test("negative control: without the instance guard both lockers fill", () => {
  const f = generated(lockers.replace(" [while locker {other} is empty]", "").replace("      %% when Empty@{other}\n", ""));
  verify(f, false, "step_preserves_inductive");
  const trace = cmd("trace", f.input);
  assert.equal(trace.status, 1, trace.text);
  assert.match(trace.text, /VIOLATION NotBothFull_a/);
  assert.match(trace.text, /Shortest path: 2 events/);
});

test("templates refuse unqualified outside references, unknown instances and misplaced annotations", () => {
  refused(lockers.replace("%% when Empty@b", "%% when Empty"), /names Empty, which is inside template Locker; name one instance as Empty@KEY/);
  refused(lockers.replace("Driving --> Gone : the courier fills locker a", "Driving --> Full : the courier fills locker a"), /Full is inside template Locker/);
  refused(lockers.replace("%% only b\n      Full", "%% only c\n      Full"), /names c, which is not an instance of template Locker/);
  refused(lockers.replace("as Full\n", "as Full\n      %% only a\n").replace("%% never NotBothFull: Full & Full@{other}", "%% never NotBothFull: Full & Jammed@b"), /never NotBothFull_b: "Full" must identify one state/);
  refused(lockers.replace("({size})", "({colour})"), /placeholder its instance does not bind/);
  refused(lockers.replace("      %% instance b: size = large, other = a\n", "").replace("      %% instance a: size = small, other = b\n", ""), /has no %% instance lines/);
  refused(lockers.replace("%% template", "%% template\n      %% only a"), /must come right after a state or transition/);
  refused(lockers.replace("Driving --> Gone : the courier fills locker b", "Driving --> Gone : the courier fills locker b\n      %% only a"), /outside any template/);
  refused(lockers.replace("    --\n    state \"Courier\"", "      [*] --> Elsewhere\n    --\n    state \"Courier\""), /must be the only state in its part/);
  refused(lockers.replace("%% instance a:", "%% instance a: key = z,"), /\{key\} is the instance key/);
  refused("stateDiagram-v2\nstate \"T {key}\" as T {\n%% template\n%% instance a:\n[*] --> X\n}\n[*] --> T", /template T must be a region/);
});

const lockerEvents = here("./lockers-events.mmd");

test("declared events: families, groups and needs make every reacting region wait for the event's owner", () => {
  const f = generated(lockerEvents);
  const { state: s, event: e, map } = names(f);
  const fill = map.events.find((x) => x.label === "the courier fills locker a");
  assert.deepEqual([fill.cause, fill.family, fill.intent], ["the courier", "the courier fills locker {key}", "The courier fills an empty locker while the other one is empty."]);
  assert.deepEqual(fill.needs, { not: false, states: { "Locker a (small)": ["Waiting for a small parcel"], "Locker b (large)": ["Waiting for a large parcel"] } });
  assert.deepEqual(fill.transitions.map((t) => t.region), ["Locker a (small)", "Courier"]);
  const collect = map.events.find((x) => x.label === "you collect from locker b");
  assert.deepEqual([collect.cause, collect.needs, collect.transitions.map((t) => t.region)], ["you", null, ["Locker b (large)", "Courier"]]);
  const a = "Waiting for a small parcel", b = "Waiting for a large parcel";
  addProof(f, `
    assert forall|x: State| ${e("the courier fills locker a")}(${s(a, b, "Driving")}, x) == (x == ${s("Full", b, "Gone")}) by {};
    assert forall|x: State| !${e("the courier fills locker b")}(${s("Full", b, "Driving")}, x) by {};
    assert forall|x: State| ${e("you collect from locker a")}(${s("Full", b, "Gone")}, x) == (x == ${s(a, b, "Driving")}) by {};
  `);
  verify(f);
  const trace = cmd("trace", f.input);
  assert.equal(trace.status, 0, trace.text);
  assert.match(trace.text, /Exhausted 7 reachable chart states/);
});

test("a group edge binds each member's values into its effect and %% when ids", () => {
  const f = generated(lockerEvents.replace("Gone --> Driving : a collection", "Gone --> Driving : a collection [while that locker is full] / log locker {key}\n      %% when Full@{key}"));
  const { map } = names(f);
  const courier = (label) => map.events.find((x) => x.label === label).transitions.find((t) => t.region === "Courier");
  assert.equal(courier("you collect from locker a").intent, "log locker a");
  assert.deepEqual(courier("you collect from locker b").guard, { not: false, states: { "Locker b (large)": ["Full"] } });
});

test("negative control: without needs the courier fills a locker while the other is full", () => {
  const f = generated(lockerEvents.replace(" | needs Empty@{key}, Empty@{other}", ""));
  verify(f, false, "step_preserves_inductive");
  const trace = cmd("trace", f.input);
  assert.equal(trace.status, 1, trace.text);
  assert.match(trace.text, /VIOLATION NotBothFull_a/);
});

test("strict events refuse undeclared and unlabeled steps; declarations are checked", () => {
  refused(lockerEvents.replace("Gone --> Driving : a collection", "Gone --> Driving : you wave"), /strict events: "you wave" is not declared/);
  refused(lockerEvents.replace("Gone --> Driving : a collection", "Gone --> Driving"), /strict events: the step Gone --> Driving has no event label/);
  refused(lockerEvents.replace("locker {key} | by you |", "locker {key} [now] | by you |"), /an event line reads/);
  refused(lockerEvents.replace("| by you |", "| you |"), /strict events: "you collect from locker a" is not declared/);
  refused(lockerEvents.replace(" | You take your parcel out of the locker.", ""), /needs a one-sentence intent/);
  refused(lockerEvents.replace("needs Empty@{key}", "needs Spare@{key}"), /the event "the courier fills locker a" needs Spare@a, which is not a state id/);
  refused(lockerEvents.replace("%% events a collection", "%% events the door of locker {key} jams"), /is both an event and an event group/);
  refused(`${lockerEvents}\n%% event you collect from locker a | by you | Again.`, /declared twice/);
  refused(lockerEvents.replace("needs Empty@{key}, Empty@{other}", "needs Empty"), /the event "the courier fills locker a" needs Empty, which is inside template Locker; name one instance as Empty@KEY/);
  // A placeholder nothing binds is refused in any chart, not read as a one-off event.
  const typo = lockerEvents.replace("  %% events declared\n", "").replace("Driving --> Gone : the courier fills locker {key}", "Driving --> Gone : the courier fills locker {kye}");
  refused(typo, /the step Driving --> Gone on "the courier fills locker \{kye\}" leaves \{kye\} unbound/);
  assert.match(cmd("lint", fixture(typo).input).stdout, /leaves \{kye\} unbound/);
  // Without %% events declared, undeclared events keep today's meaning.
  const lenient = generated(lockerEvents.replace("  %% events declared\n", "").replace("Gone --> Driving : a collection", "Gone --> Driving : you wave"));
  assert.ok(JSON.parse(fs.readFileSync(lenient.map, "utf8")).events.some((x) => x.label === "you wave" && x.cause === undefined));
});

test("flat prints the expanded chart, which reads back to the same flat model without parallel parts", async () => {
  const { parseMermaid } = await import("../mermaid.mjs");
  const model = (text) => {
    const p = parseMermaid(text);
    return {
      regions: Object.entries(p.regions).map(([name, r]) => [name, [...r.states], [...r.initial],
        r.transitions.map(({ from, to, label, guard, needs, optional, intent }) => ({ from, to, label, guard, needs, optional, intent }))]),
      events: [...p.events].map(([label, { cause, intent, needs }]) => [label, cause, intent, needs]),
    };
  };
  for (const text of [here("./lockers-events.mmd"), here("./lockers.mmd"), parcel, here("../../examples/sync.mmd")]) {
    const f = fixture(text);
    const result = cmd("flat", f.input);
    assert.equal(result.status, 0, result.text);
    assert.doesNotMatch(result.stdout, /\{[a-z]+\}|%% template|%% instance|%% only/);
    const carried = new Set(Object.values(parseMermaid(text).regions).flatMap((r) => r.transitions.map((t) => t.label)));
    const expected = model(text);
    expected.events = expected.events.filter(([label]) => carried.has(label));
    assert.deepEqual(model(result.stdout), expected);
  }
  const nested = cmd("flat", fixture(door).input);
  assert.match(nested.stdout, /%% scoped: active only while Door is in Locked; \(not in Locked\) otherwise/);
  assert.match(nested.stdout, /%% labeled start: entered on "the timer locks the door" at Armed/);
  assert.match(nested.stdout, /%% within: when Door\/Locked\/Alarm is Ringing/);
  assert.match(nested.stdout, /%% rule Door\/Locked\/Alarm scope:/);
});

test("lint reports unreachable states, dead edges and event-declaration problems", () => {
  assert.equal(cmd("lint", fixture(here("../../examples/sync.mmd")).input).status, 0);
  const doorLint = cmd("lint", fixture(door).input);
  assert.equal(doorLint.status, 1);
  assert.equal(doorLint.stdout, 'dead edge: Door/Locked/Keypad: Typing --> Idle on "you unlock the door": Door always leaves Locked on this event\n1 finding.\n');
  const findings = cmd("lint", fixture(`stateDiagram-v2
%% events declared
%% event you press | by you | You press the button.
%% event you rest | by you | Nothing carries this.
%% events presses = you press | you knock
%% event this line has no cause | you
[*] --> Off
Off --> On : you press
On --> Off : you press
Off --> Off : you wait
Lost --> Off : you press
state On {
 state Lamp {
  [*] --> Dim
  Dim --> Bright : you press [while off]
  %% when Off
 }
 --
 state Fan {
  [*] --> Slow
  Slow --> Fast : you press
 }
}
`).input);
  assert.equal(findings.status, 1, findings.text);
  for (const line of [
    "unreachable (ignoring guards): Main: Lost",
    'dead edge: Main/On/Lamp: Dim --> Bright on "you press": Main always leaves On on this event',
    'dead edge: Main/On/Fan: Slow --> Fast on "you press": Main always leaves On on this event',
    'strict events: "you wait" is not declared with %% event',
    'declared event nothing carries: "you rest"',
    'group "presses": member "you knock" is neither a declared event nor a step label',
    "read as a plain comment (not the construct's shape): %% event this line has no cause | you",
  ]) assert.ok(findings.stdout.includes(`${line}\n`), `${line}\n---\n${findings.stdout}`);
  const cooccur = cmd("lint", fixture(`stateDiagram-v2
[*] --> Off
Off --> On : power on
On --> Off : power off
Off --> Off : tick [while the lamp is bright]
%% when Bright
state On {
 state Lamp {
  [*] --> Dim
  Dim --> Bright : brighten
 }
 --
 state Fan {
  [*] --> Slow
 }
}
`).input);
  assert.match(cooccur.stdout, /dead edge: Main: Off --> Off on "tick": Main would have to be in two states at once/);
});

test("query lists ways in and out with cause, needs and intent, reacting regions, and region summaries", () => {
  const f = fixture(door);
  const armed = cmd("query", f.input, "--state", "Armed");
  assert.equal(armed.status, 0, armed.text);
  for (const line of [
    'labeled start when Door enters Locked on "the timer locks the door": Armed',
    'Door: Open --> Locked on "you lock and arm" enters here at Armed',
    'from "Disarmed" on "you arm the alarm"',
    'to "Ringing" on "someone forces the door"',
    "closes (to (not in Locked)) whenever Door leaves Locked",
    'Siren: Quiet --> Loud on "someone forces the door"',
  ]) assert.ok(armed.stdout.includes(line), `${line}\n---\n${armed.stdout}`);
  assert.match(cmd("query", f.input, "--state", "Ringing").stdout, /leaves with Door: Locked --> Open on "someone opens the door"/);
  const events = fixture(here("./lockers-events.mmd"));
  const fill = cmd("query", events.input, "--event", "the courier fills locker b");
  for (const line of ["cause: the courier", "needs: when Locker b (large) is Waiting for a large parcel; Locker a (small) is Waiting for a small parcel",
    "intent: The courier fills an empty locker while the other one is empty.", "reacting regions (2):", "Driving --> Gone"])
    assert.ok(fill.stdout.includes(line), `${line}\n---\n${fill.stdout}`);
  assert.match(cmd("query", events.input, "--event", "a collection").stdout, /members carried: 2/);
  assert.match(cmd("query", events.input, "--event", "you collect from locker {key}").stdout, /Family .*members carried: 2/);
  const region = cmd("query", f.input, "--region", "Door/Locked/Alarm").stdout;
  assert.match(region, /scoped: active only while Door is in Locked/);
  assert.match(region, /read by: Door, Siren/);
  assert.match(cmd("query", f.input).stdout, /^4 regions \(2 scoped, 0 from templates\), 11 states, 13 edges/);
  assert.equal(cmd("query", f.input, "--state", "Nowhere").status, 1);
  assert.equal(cmd("query", f.input, "--state", "Open", "--event", "x").status, 2);
});

test("reasons on limits are recorded, printed, and lint --cause lists the limits that state none", async () => {
  const lock = `stateDiagram-v2
%% event the lock relocks | by the lock | needs Closed | reason: a relock on an open door jams the bolt | The lock throws the bolt again.
%% event the lock opens itself | by the lock | needs Closed | The lock releases the bolt for a known key.
%% event you turn the key | by you | needs Closed | You turn the key by hand.
state "Smart lock" as Lock {
 state Bolt {
  [*] --> Thrown
  Thrown --> Drawn : the lock opens itself [while the battery is fine]
  %% when Fine
  Drawn --> Thrown : the lock relocks [while the battery is fine]
  %% when Fine | reason: a weak battery cannot drive the bolt home
  Thrown --> Drawn : you turn the key
 }
 --
 state Door {
  [*] --> Closed
  Closed --> Ajar : you push the door
  Ajar --> Closed : you pull the door
 }
 --
 state Battery {
  [*] --> Fine
  Fine --> Weak : the battery runs down
 }
}
[*] --> Lock
%% never BoltOut: Drawn & Ajar -- The bolt is never drawn while the door swings. | reason: a swinging door scrapes the bolt
`;
  const f = generated(lock);
  const map = JSON.parse(fs.readFileSync(f.map, "utf8"));
  const relock = map.events.find((e) => e.label === "the lock relocks");
  assert.equal(relock.needs.reason, "a relock on an open door jams the bolt");
  assert.equal(relock.transitions[0].guard.reason, "a weak battery cannot drive the bolt home");
  assert.equal(map.events.find((e) => e.label === "the lock opens itself").transitions[0].guard.reason, undefined);
  assert.equal(map.rules[0].reason, "a swinging door scrapes the bolt");
  assert.equal(map.rules[0].sentence, "The bolt is never drawn while the door swings.");

  const plain = cmd("lint", f.input);
  assert.equal(plain.status, 0, plain.text);
  const byLock = cmd("lint", f.input, "--cause", " The Lock ");
  assert.equal(byLock.status, 1);
  assert.equal(byLock.stdout, [
    'restricts without a stated reason: "the lock opens itself" needs: when Door is Closed',
    'restricts without a stated reason: "the lock opens itself": Bolt: Thrown --> Drawn when Battery is Fine',
    "2 findings.", ""].join("\n"));
  assert.match(cmd("lint", f.input, "--cause", "you").stdout, /^restricts without a stated reason: "you turn the key" needs: when Door is Closed\n1 finding\.\n$/);
  assert.match(cmd("query", f.input, "--event", "the lock relocks").stdout,
    /needs: when Door is Closed \| reason: a relock on an open door jams the bolt/);

  const flat = cmd("flat", f.input).stdout;
  assert.match(flat, /%% when r2s0 \| reason: a weak battery cannot drive the bolt home/);
  assert.match(flat, /\| needs r1s0 \| reason: a relock on an open door jams the bolt \|/);
  assert.match(flat, /%% rule BoltOut: .* \| reason: a swinging door scrapes the bolt/);
  const { parseMermaid } = await import("../mermaid.mjs");
  const guards = (text) => Object.values(parseMermaid(text).regions).flatMap((r) => r.transitions.map((t) => t.guard ?? null));
  assert.deepEqual(guards(flat), guards(lock));
  const needs = (text) => [...parseMermaid(text).events.values()].map((e) => e.needs);
  assert.deepEqual(needs(flat), needs(lock));

  const bad = cmd("lint", fixture("stateDiagram-v2\n%% event x | by you | reason: no needs | Intent.\n[*] --> A\nA --> B : x\n").input);
  assert.equal(bad.status, 1);
  assert.match(bad.text, /reason/);
});

const latch = (sensorGuard) => `stateDiagram-v2
state "Parcel hatch" as Hatch {
 state Door {
  [*] --> Shut
  Shut --> Open : you open the hatch [while the latch is released]
  %% when Released
  Open --> Shut : you close the hatch
 }
 --
 state Latch {
  [*] --> Locked
  Locked --> Released : the latch releases [while the chute is clear]
  %% when Clear
  Released --> Locked : the latch locks [while the hatch is shut]
  %% when Shut
 }
 --
 state Chute {
  [*] --> Clear
  Clear --> Blocked : a parcel slides in${sensorGuard ? " [while the latch is locked]\n  %% when Locked" : ""}
  Blocked --> Clear : the parcel drops
 }
}
[*] --> Hatch
%% never NoParcelAtOpenHatch: Open & Blocked -- A parcel never sits in the chute while the hatch stands open.
`;

test("rules settles each rule on a cone: refines through free regions, finds real violations, agrees with trace", async () => {
  const guarded = fixture(latch(true));
  const holds = cmd("rules", guarded.input);
  assert.equal(holds.status, 0, holds.text);
  assert.match(holds.stdout, /^HOLDS NoParcelAtOpenHatch \(cone of 3 regions, nothing free; \d+ states, 2 rounds, \d+ ms\)\n1 hold, 0 violated, 0 inconclusive, 0 incomplete\.\n$/);
  const capped = cmd("rules", guarded.input, "--max-cone", "2");
  assert.equal(capped.status, 3, capped.text);
  assert.match(capped.stdout, /^INCONCLUSIVE NoParcelAtOpenHatch \(cone of 2 regions, 1 free;/);
  assert.match(capped.stdout, /Latch \(free\): Released/);

  const open = cmd("rules", fixture(latch(false)).input);
  assert.equal(open.status, 1, open.text);
  assert.match(open.stdout, /^VIOLATION NoParcelAtOpenHatch \(cone of 3 regions, nothing free;/);
  for (const step of ["1. Start", "the latch releases", "you open the hatch", "a parcel slides in → Door: Open; Latch: Released; Chute: Blocked"])
    assert.ok(open.stdout.includes(step), `${step}\n---\n${open.stdout}`);
  assert.equal(cmd("rules", guarded.input, "--rule", "Nowhere").status, 1);
  assert.equal(cmd("rules", fixture(here("../../examples/sync.mmd")).input).status, 0);

  // Per rule, a cone's verdict matches an exhaustive search of the whole chart for that rule.
  const { compile } = await import("../chart-verus.mjs");
  const { search } = await import("../chart-search.mjs");
  const { checkRules } = await import("../chart-rules.mjs");
  for (const text of [door, parcel, restart, lamp, here("./lockers.mmd"), here("./lockers-events.mmd"), latch(true), latch(false)]) {
    const { ir } = compile(text, "Chart");
    for (const maxCone of [1, 12])
      for (const result of checkRules(ir, { maxCone })) {
        const truth = search({ ...ir, rules: [result.rule] }, 1000000).status;
        if (result.status === "holds") assert.equal(truth, "exhausted", result.rule.name);
        else if (result.status === "violation") assert.equal(truth, "violation", result.rule.name);
        else assert.equal(result.status, "inconclusive", result.rule.name);
        if (maxCone === 12) assert.notEqual(result.status, "inconclusive", result.rule.name);
      }
  }
});

test("compare reports flat-model differences, after renames, and nothing for a chart and its flat print", () => {
  const sync = here("../../examples/sync.mmd");
  const a = fixture(sync);
  const same = cmd("compare", a.input, a.input);
  assert.equal(same.status, 0, same.text);
  assert.equal(same.stdout, "Same flat model.\n");
  const edited = sync.replace('"Waiting to sync" as Pending', '"Not yet sent" as Pending')
    .replace("the server confirms [while online]\n      %% when Online", "the server confirms")
    .replace('"Connection" as Connection', '"Network" as Connection');
  const b = fixture(edited);
  const renames = path.join(b.dir, "renames.json");
  fs.writeFileSync(renames, JSON.stringify({ regions: { Connection: "Network" }, states: { "Waiting to sync": "Not yet sent" } }));
  const diff = cmd("compare", a.input, b.input, "--rename", renames);
  assert.equal(diff.status, 1, diff.text);
  assert.equal(diff.stdout, [
    "steps:",
    '  - Sync badge: Not yet sent --> Synced on "the server confirms" (when Network is Online)',
    '  + Sync badge: Not yet sent --> Synced on "the server confirms"',
    "2 differences (- only in the first chart, + only in the second).", ""].join("\n"));
  const unrenamed = cmd("compare", a.input, b.input).stdout;
  assert.match(unrenamed, /regions:\n  - Connection\n  \+ Network\n/);
  assert.match(unrenamed, /rules:\n  - never Connection is Offline & Sync badge is Synced\n  \+ never Network is Offline & Sync badge is Synced\n/);
  // What a limit protects is part of the model: dropping a reason is a difference.
  const reasoned = sync.replace("%% when Online", "%% when Online | reason: only a live connection confirms");
  const r = cmd("compare", fixture(reasoned).input, a.input);
  assert.equal(r.status, 1, r.text);
  assert.match(r.stdout, /- .*\(when Connection is Online \| reason: only a live connection confirms\)/);
  fs.writeFileSync(renames, JSON.stringify({ places: {} }));
  assert.equal(cmd("compare", a.input, b.input, "--rename", renames).status, 1);
  assert.equal(cmd("compare", a.input).status, 2);

  // Without parallel parts, a chart and its flat print differ only in rules (printed as
  // comments) and declared events no step carries (left out of the print).
  const events = fixture(here("./lockers-events.mmd"));
  const printed = path.join(events.dir, "flat.mmd");
  fs.writeFileSync(printed, cmd("flat", events.input).stdout);
  assert.equal(cmd("compare", events.input, printed).stdout, [
    "events:",
    '  - "the door of locker a jams" by the locker: The locker door sticks shut.',
    "rules:",
    "  - never Locker a (small) is Full & Locker b (large) is Full",
    "  - never Locker b (large) is Full & Locker a (small) is Full",
    "3 differences (- only in the first chart, + only in the second).", ""].join("\n"));
});
