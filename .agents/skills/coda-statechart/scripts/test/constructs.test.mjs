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
  verify(f, false, "preserves_event_");
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

const office = `stateDiagram-v2
%% events declared
%% event sign in | by you | You sign in.
%% event sign out | by you | You sign out.
%% event someone else closes the item | by another physician | Another physician closes it first.
%% event close the item | by you | command on the item: Close, clicked while In | needs Open | reason: only a signed-in physician closes an open item | You close it.
%% event reopen the item | by you | command on the item: Reopen | needs In, Closed | reason: only a signed-in physician reopens a closed item | You reopen it.
state Desk {
 state Session {
  [*] --> Out
  Out --> In : sign in
  In --> Out : sign out
 }
 --
 state Item {
  [*] --> Open
  Open --> Closed : close the item
  Open --> Closed : someone else closes the item
  Closed --> Open : reopen the item
 }
 --
 state "Your command on the item" as Rc {
  %% receipt the item
 }
}
[*] --> Desk
`;

test("commands: the click moves only the receipt, the item changes on acceptance, refusal fires exactly when it is no longer allowed, and no outcome lands unasked", async () => {
  const f = generated(office);
  const { state, event: e } = names(f);
  const idle = "No command on the item in flight";
  const close = "Close on the item: sending; other commands on it wait";
  const stuck = "Couldn't confirm Close on the item, with Check again";
  const s = (a, b, c) => state(a, b, c);
  addProof(f, `
    assert forall|b: State| ${e("close the item")}(${s("In", "Open", idle)}, b) == (b == ${s("In", "Open", close)}) by {};
    assert forall|b: State| ${e("your Close on the item goes through")}(${s("In", "Open", close)}, b) == (b == ${s("In", "Closed", idle)}) by {};
    assert forall|b: State| !${e("your Close on the item is refused")}(${s("In", "Open", close)}, b) by {};
    assert forall|b: State| !${e("your Close on the item goes through")}(${s("In", "Closed", close)}, b) by {};
    assert forall|b: State| ${e("your Close on the item is refused")}(${s("In", "Closed", close)}, b) == (b == ${s("In", "Closed", idle)}) by {};
    assert forall|b: State| !${e("reopen the item")}(${s("In", "Closed", close)}, b) by {};
    assert forall|b: State| !${e("close the item")}(${s("Out", "Open", idle)}, b) by {};
    assert forall|b: State| ${e("your Close on the item goes through")}(${s("Out", "Open", close)}, b) == (b == ${s("Out", "Closed", idle)}) by {};
    assert forall|b: State| ${e("your Reopen on the item is refused")}(${s("Out", "Open", "Reopen on the item: sending; other commands on it wait")}, b)
      == (b == ${s("Out", "Open", idle)}) by {};
    assert forall|b: State| ${e("your Close on the item can't be confirmed")}(${s("In", "Open", close)}, b) == (b == ${s("In", "Open", stuck)}) by {};
    assert forall|b: State| ${e("check your Close on the item again")}(${s("In", "Open", stuck)}, b) == (b == ${s("In", "Open", close)}) by {};
    assert forall|b: State| !${e("your Close on the item goes through")}(${s("In", "Open", idle)}, b) by {};
    assert forall|b: State| !${e("your Close on the item is refused")}(${s("In", "Closed", idle)}, b) by {};
    assert forall|b: State| !${e("your Close on the item can't be confirmed")}(${s("In", "Open", idle)}, b) by {};
    assert forall|b: State| !${e("check your Close on the item again")}(${s("In", "Open", close)}, b) by {};
  `);
  assert.ok(await agreement(f) > 4);
  verify(f);
  // Every limit the tool generates states its reason.
  for (const cause of ["you", "the system", "the client"])
    assert.match(cmd("lint", f.input, "--cause", cause).stdout, /^No findings\.\n$/, cause);
});

const shifts = `stateDiagram-v2
%% events declared
%% event start the {work} | by Coda | Coda starts the {work}.
%% event stop the {work} | by you | command on the {work}: Stop | needs Run@{key} | reason: only running work stops | You stop the {work}.
state Desk {
 state "Work" as Work {
  %% template
  %% instance a: work = review
  %% instance b: work = brief
  state "{work}" as Life {
   [*] --> Idle
   state "{work} idle" as Idle
   state "{work} running" as Run
   Idle --> Run : start the {work}
   Run --> Idle : stop the {work}
  }
  --
  state "Your command on the {work}" as Rc {
   %% receipt the {work}
  }
 }
}
[*] --> Desk
`;

test("a receipt inside a template is one receipt per instance; search and Verus agree", async () => {
  const f = generated(shifts);
  const { state, event: e } = names(f);
  const idle = (w) => `No command on the ${w} in flight`, sending = (w) => `Stop on the ${w}: sending; other commands on it wait`;
  addProof(f, `
    assert forall|b: State| ${e("your Stop on the review goes through")}(${state("review running", sending("review"), "brief running", idle("brief"))}, b)
      == (b == ${state("review idle", idle("review"), "brief running", idle("brief"))}) by {};
    assert forall|b: State| !${e("your Stop on the review goes through")}(${state("review running", idle("review"), "brief running", sending("brief"))}, b) by {};
  `);
  assert.ok(await agreement(f) > 4);
  verify(f);
});

test("commands refuse a missing receipt, a receipt with no command or with other content, and a misread command field", () => {
  refused(office.replace(" state \"Your command on the item\" as Rc {\n  %% receipt the item\n }\n", " state Spare {\n  [*] --> S\n }\n"), /on the item, which no %% receipt composite fills/);
  refused(office.replace("%% receipt the item\n }\n", "%% receipt the item\n }\n --\n state Drawer {\n  %% receipt the drawer\n }\n"), /the %% receipt for the drawer has no command declared on it/);
  refused(office.replace("command on the item: Close", "command Close"), /a command reads/);
  refused(office.replace("%% receipt the item\n", "%% receipt the item\n  [*] --> Spare\n"), /the %% receipt composite for the item holds nothing else/);
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
  verify(f, false, "preserves_event_");
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

test("a template region whose states all drop for an instance is left out of that instance", () => {
  const desks = `stateDiagram-v2
  state "Desks" as Desks {
    state "Desk {key}" as Desk {
      %% template
      %% instance a: side = left
      %% instance b: side = right
      state "Lamp" as Lamp {
        [*] --> Off
        Off --> On : turn on lamp {key}
        On --> Off : turn off lamp {key}
      }
      --
      state "Drawer" as Drawer {
        %% only b
        [*] --> Shut
        Shut --> Open : open drawer {key}
        Open --> Shut : shut drawer {key}
      }
      --
      state "Chair" as Chair {
        [*] --> In
        In --> Out : pull out chair {key}
        Out --> In : push in chair {key}
      }
    }
  }
  [*] --> Desks
`;
  const { map } = names(generated(desks));
  assert.deepEqual(map.regions.map((r) => [r.caption, r.states.map((x) => x.caption)]), [
    ["Desk a/Lamp", ["Off", "On"]],
    ["Desk a/Chair", ["In", "Out"]],
    ["Desk b/Lamp", ["Off", "On"]],
    ["Desk b/Drawer", ["Shut", "Open"]],
    ["Desk b/Chair", ["In", "Out"]],
  ]);
  assert.equal(cmd("lint", fixture(desks).input).status, 0);
  refused(desks.replace("[*] --> In\n", "%% only b\n        [*] --> In\n"), /Desk keeps fewer than two parallel regions for instance a/);
});

const shapedDesk = here("./reads.mmd");

test("a shape written once fills composites anywhere; each use is bound, limited and extended like a template instance", async () => {
  const f = generated(shapedDesk);
  const { state: s, event: e, map } = names(f);
  const note = (x) => `Panel open.The note read.${x}`;
  assert.deepEqual(map.regions.map((r) => [r.caption, r.states.map((x) => x.caption)]), [
    ["The list read", ["Loading the list", "Showing the list", "Showing the list, with Retry", "Couldn't load the list, with Retry", "Switched off"]],
    ["Panel", ["Closed", note("Loading the note"), note("Showing the note"), note("Showing the note, with Retry"), note("Couldn't load the note, with Retry")]],
  ]);
  addProof(f, `
    assert forall|b: State| ${e("open the panel")}(${s("Showing the list", "Closed")}, b) == (b == ${s("Showing the list", note("Loading the note"))}) by {};
    assert forall|b: State| ${e("the note fails")}(${s("Showing the list", note("Showing the note"))}, b)
      == (b == ${s("Showing the list", note("Showing the note, with Retry"))}) by {};
    assert forall|b: State| ${e("the list is switched off")}(${s("Showing the list", "Closed")}, b) == (b == ${s("Switched off", "Closed")}) by {};
    assert forall|b: State| !${e("retry the note")}(${s("Showing the list", note("Showing the note"))}, b) by {};
    assert forall|b: State| !${e("retry the list")}(${s("Showing the list, with Retry", "Closed")}, b) by {};
    assert forall|b: State| ${e("retry the note")}(${s("Showing the list", note("Showing the note, with Retry"))}, b)
      == (b == ${s("Showing the list", note("Loading the note"))}) by {};
  `);
  assert.ok(await agreement(f) > 4);
  verify(f);
});

test("shapes refuse an unknown shape, a misplaced use, a repeated use name and a limit naming no use", () => {
  refused(shapedDesk.replace("%% use read as list", "%% use reed as list"), /names the shape reed, which no %% shape composite defines/);
  refused(shapedDesk.replace("    %% use read as note: thing = the note\n", "    [*] --> Spare\n    %% use read as note: thing = the note\n"), /%% use must be the first line inside the composite it fills/);
  refused(shapedDesk.replace("%% use read as note", "%% use read as list"), /two %% use lines of the shape read are named list/);
  refused(shapedDesk.replace("%% only note", "%% only nite"), /names nite, which is not a use of the shape read/);
});

test("negative control: without the instance guard both lockers fill", () => {
  const f = generated(lockers.replace(" [while locker {other} is empty]", "").replace("      %% when Empty@{other}\n", ""));
  verify(f, false, "preserves_event_");
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

test("a group named with a placeholder is one group per instance, bound like a family", () => {
  const family = lockerEvents
    .replace("%% events a collection = you collect from locker {key}", "%% events a collection = you collect from locker {key}\n  %% events locker {key} is touched = you collect from locker {key} | the door of locker {key} jams")
    .replace("      %% only b\n      %% never", "      %% only b\n      Jammed --> Full : locker {key} is touched\n      %% only b\n      %% never")
    .replace("Gone --> Driving : a collection", "Gone --> Driving : locker {key} is touched");
  const written = lockerEvents
    .replace("      %% only b\n      %% never", "      %% only b\n      Jammed --> Full : you collect from locker {key}\n      %% only b\n      Jammed --> Full : the door of locker {key} jams\n      %% only b\n      %% never")
    .replace("Gone --> Driving : a collection", ["a", "b"].flatMap((k) => [`Gone --> Driving : you collect from locker ${k}`, `Gone --> Driving : the door of locker ${k} jams`]).join("\n      "));
  const same = cmd("compare", fixture(family).input, fixture(written).input);
  assert.equal(same.stdout, "Same flat model.\n");
  assert.equal(cmd("lint", fixture(family).input).status, 0);
  refused(family.replace("%% events a collection = you collect from locker {key}", "%% events locker a is touched = you collect from locker a"), /the event group "locker a is touched" is declared twice/);
});

test("instances of different templates that produce the same event merge their values", () => {
  const room = `stateDiagram-v2
  %% events declared
  %% event press the {thing} | by you | needs {ready}@{key} | reason: only a powered {thing} switches | You press it.
  %% event unplug the {thing} | by you | You unplug it.
  %% event plug in the {thing} | by you | You plug it in.
  state "Room" as Room {
    state "{thing} switch" as Switch {
      %% template
      %% instance a: thing = lamp
      %% instance b: thing = fan
      [*] --> Off
      state "Off" as Off
      state "On" as On
      Off --> On : press the {thing}
      On --> Off : press the {thing}
    }
    --
    state "{thing} power" as Power {
      %% template
      %% instance a: thing = lamp, ready = Plugged
      %% instance b: thing = fan, ready = Plugged
      [*] --> Plugged
      state "Plugged in" as Plugged
      state "Unplugged" as Unplugged
      Plugged --> Unplugged : unplug the {thing}
      Unplugged --> Plugged : plug in the {thing}
    }
  }
  [*] --> Room
`;
  const { map } = names(generated(room));
  const { needs } = map.events.find((x) => x.label === "press the fan");
  assert.deepEqual([needs.not, needs.states], [false, { "fan power": ["Plugged in"] }]);
  refused(room.replace("%% instance b: thing = fan\n", "%% instance b: thing = fan, ready = Unplugged\n"), /"press the fan" is bound by instances that disagree on \{ready\}/);
});

test("negative control: without needs the courier fills a locker while the other is full", () => {
  const f = generated(lockerEvents.replace(" | needs Empty@{key}, Empty@{other}", ""));
  verify(f, false, "preserves_event_");
  const trace = cmd("trace", f.input);
  assert.equal(trace.status, 1, trace.text);
  assert.match(trace.text, /VIOLATION NotBothFull_a/);
});

test("a cause's limit holds for every event it causes, as its own condition nothing widens", () => {
  // The event's own needs name both courier states, yet the cause's limit still holds.
  const limited = lockerEvents.replace("  %% events declared\n", "  %% events declared\n  %% cause the courier | needs Driving | reason: the courier fills only on a round\n")
    .replace("needs Empty@{key}, Empty@{other}", "needs Empty@{key}, Empty@{other}, Driving, Gone");
  const f = generated(limited);
  const { state: s, event: e, map } = names(f);
  const fill = map.events.find((x) => x.label === "the courier fills locker a");
  assert.deepEqual(fill.limits, [{ not: false, states: { Courier: ["Driving"] }, reason: "the courier fills only on a round", cause: "the courier" }]);
  assert.equal(map.events.find((x) => x.label === "you collect from locker a").limits, undefined);
  const a = "Waiting for a small parcel", b = "Waiting for a large parcel";
  addProof(f, `
    assert forall|x: State| !${e("the courier fills locker a")}(${s(a, b, "Gone")}, x) by {};
    assert forall|x: State| ${e("the courier fills locker a")}(${s(a, b, "Driving")}, x) == (x == ${s("Full", b, "Gone")}) by {};
  `);
  verify(f);
  // A limit with placeholders binds per instance, like the event it limits.
  const bound = names(generated(lockerEvents.replace(" | needs Empty@{key}, Empty@{other}", " | needs Empty@{key}")
    .replace("  %% events declared\n", "  %% events declared\n  %% cause the courier | needs Empty@{other} | reason: one parcel at a time\n"))).map;
  assert.deepEqual(bound.events.find((x) => x.label === "the courier fills locker a").limits[0].states, { "Locker b (large)": ["Waiting for a large parcel"] });
  // Events a command generates keep their own cause's limit...
  const desk = names(generated(office.replace("%% events declared\n", "%% events declared\n%% cause the system | needs In | reason: the server answers only a signed-in session\n"))).map;
  const through = desk.events.find((x) => x.label === "your Close on the item goes through");
  assert.deepEqual(through.limits.map((l) => Object.values(l.states)), [[["In"]]]);
  assert.ok(through.transitions.every((t) => t.within?.some((w) => w.cause === "the system")));
  // ...and their click's template values.
  for (const cause of ["you", "the system"]) {
    const shift = names(generated(shifts.replace("%% events declared\n", `%% events declared\n%% cause ${cause} | needs Run@{key} | reason: only running work\n`))).map;
    const label = cause === "you" ? "check your Stop on the review again" : "your Stop on the review goes through";
    assert.deepEqual(Object.values(shift.events.find((x) => x.label === label).limits.at(-1).states), [["review running"]]);
  }
  generated(limited.replace("needs Driving |", "needs not Gone |"));
  refused(limited.replace("cause the courier", "cause the postman"), /no event is caused "by the postman"/);
  refused(limited.replace("needs Driving |", "needs Parked |"), /the cause "the courier" needs Parked, which is not a state id/);
  refused(limited.replace("  %% events declared\n", "  %% events declared\n  %% cause the courier | needs Gone\n"), /the cause "the courier" is declared twice/);
});

test("a command is accepted only while its click's limits still hold, and refused when they fail", () => {
  const f = generated(office.replace("%% events declared\n", "%% events declared\n%% cause the physician | needs In | reason: only a signed-in physician acts\n")
    .replace("%% event close the item | by you | command on the item: Close, clicked while In |", "%% event close the item | by the physician | command on the item: Close |"));
  const { state: s, event: e } = names(f);
  const idle = "No command on the item in flight", sending = "Close on the item: sending; other commands on it wait";
  addProof(f, `
    assert forall|x: State| !${e("your Close on the item goes through")}(${s("Out", "Open", sending)}, x) by {};
    assert forall|x: State| ${e("your Close on the item is refused")}(${s("Out", "Open", sending)}, x) == (x == ${s("Out", "Open", idle)}) by {};
    assert forall|x: State| ${e("your Close on the item goes through")}(${s("In", "Open", sending)}, x) == (x == ${s("In", "Closed", idle)}) by {};
    assert forall|x: State| !${e("your Close on the item is refused")}(${s("In", "Open", sending)}, x) by {};
  `);
  verify(f);
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

test("lint --repeats lists events written together on several moves that no group names yet", () => {
  const chart = (group) => `stateDiagram-v2
${group}state Desk {
 state Panel {
  [*] --> Open
  Open --> Shut : close it
  Open --> Shut : you leave
 }
 --
 state Drawer {
  [*] --> Out
  Out --> In : close it
  Out --> In : you leave
 }
 --
 state Lamp {
  [*] --> On
  On --> Off : close it
  On --> Off : you leave
  Off --> On : close it
 }
}
[*] --> Desk
`;
  const found = cmd("lint", fixture(chart("")).input, "--repeats");
  assert.match(found.stdout, /^repeated on 3 moves: "close it", "you leave"; one %% events group could name them once\n1 finding\.\n$/);
  assert.match(cmd("lint", fixture(chart("")).input).stdout, /^No findings\.\n$/);
  assert.match(cmd("lint", fixture(chart("%% events closing = close it | you leave\n")).input, "--repeats").stdout, /^No findings\.\n$/);
});

test("lint --unread lists states that nothing outside their own region reads", () => {
  const chart = (rule) => `stateDiagram-v2
state Room {
 state Door {
  [*] --> Shut
  Shut --> Open : open it
  Open --> Shut : close it
 }
 --
 state Lamp {
  [*] --> Off
  Off --> On : switch on [door open]
  %% when Open
  On --> Off : switch off
  ${rule}
 }
}
[*] --> Room
`;
  const found = cmd("lint", fixture(chart("")).input, "--unread");
  assert.match(found.stdout, /^unread region: Lamp: nothing outside it reads its states\n1 finding\.\n$/);
  const ruled = cmd("lint", fixture(chart("%% never DarkWhenShut: On & Shut -- The lamp is off while the door is shut.")).input, "--unread");
  assert.match(ruled.stdout, /^No findings\.\n$/);
  const third = cmd("lint", fixture(chart("").replace("  On --> Off : switch off", "  On --> Off : switch off\n  On --> Dim : dim it\n  Dim --> Off : switch off\n  %% never DarkWhenShut: On & Shut -- The lamp is off while the door is shut.")).input, "--unread");
  assert.match(third.stdout, /^unread: Lamp: Dim\n1 finding\.\n$/);
  assert.match(cmd("lint", fixture(chart("")).input).stdout, /^No findings\.\n$/);
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

test("against checks a detailed chart against the chart it details, in both directions, through one map", () => {
  const shop = `stateDiagram-v2
  state "Shop" as Shop {
    state "Checkout" as Checkout {
      [*] --> Cart
      state "Cart" as Cart
      state "Ordered" as Ordered {
        state "Shipping" as Shipping {
          [*] --> Packing
          state "Packing" as Packing
          state "Shipped" as Shipped
          Packing --> Shipped : the parcel leaves
        }
        --
        state "Invoice" as Invoice {
          [*] --> Due
          state "Due" as Due
        }
      }
      Cart --> Ordered : place the order [in stock]
      %% when InStock
      Ordered --> Cart : cancel the order
    }
    --
    state "Stock" as Stock {
      [*] --> InStock
      state "In stock" as InStock
      state "Sold out" as SoldOut
      InStock --> SoldOut : the last one sells
    }
  }
`;
  const pay = `stateDiagram-v2
  [*] --> Browsing
  state "Browsing" as Browsing
  state "Paying" as Paying
  state "Confirmed" as Confirmed
  Browsing --> Paying : tap Pay
  Paying --> Browsing : the payment is declined
  Paying --> Confirmed : the payment succeeds
`;
  const agreed = fixture(shop), detail = fixture(pay);
  const map = {
    states: { Browsing: { Checkout: "Cart" }, Paying: { Checkout: "Cart" },
      Confirmed: { Checkout: "Ordered", "Checkout/Ordered/Shipping": "Packing" } },
    events: { "tap Pay": null, "the payment is declined": null, "the payment succeeds": "place the order" },
  };
  const check = (m, chart = detail.input) => {
    const file = path.join(detail.dir, "map.json");
    fs.writeFileSync(file, JSON.stringify(m));
    return cmd("against", chart, agreed.input, "--map", file, "--out", path.join(detail.dir, "layered.mmd"));
  };

  // Agreed steps the detail never shows are differences until the map says why.
  const open = check(map);
  assert.equal(open.status, 1, open.text);
  assert.equal(open.stdout, [
    "Agreed steps the detailed chart never shows:",
    '  Checkout/Ordered/Shipping: from "Packing" on "the parcel leaves" to "Shipped"',
    '  Checkout: from "Ordered" on "cancel the order" to "Cart"',
    "Conditions outside the map that the detailed chart assumes:",
    '  "place the order" (Checkout: "Cart" --> "Ordered") when Stock is In stock',
    "2 differences.", ""].join("\n"));
  assert.equal(fs.readFileSync(path.join(detail.dir, "layered.mmd"), "utf8"), [
    "stateDiagram-v2",
    "%% Generated by statechart.mjs against: each box is an agreed state; the states inside detail it.",
    'state "Checkout: Cart" as a0 {', '  state "Browsing" as d0', '  state "Paying" as d1', "}",
    'state "Checkout: Ordered / Checkout/Ordered/Shipping: Packing" as a1 {', '  state "Confirmed" as d2', "}",
    "[*] --> d0", "d0 --> d1 : tap Pay (hidden)", "d1 --> d0 : the payment is declined (hidden)",
    "d1 --> d2 : the payment succeeds (place the order)", ""].join("\n"));
  const omits = { "the parcel leaves": "shipping is another chart's", "cancel the order": "cancelling is another chart's" };
  const agrees = check({ ...map, omits });
  assert.equal(agrees.status, 0, agrees.text);
  assert.match(agrees.stdout, /Left out by the map:\n  "cancel the order": cancelling is another chart's\n  "the parcel leaves": shipping is another chart's\nThe detailed chart follows the agreed chart in both directions\.\n$/);

  // Negative controls: each breaks one direction or one condition.
  const shipped = check({ ...map, omits, states: { ...map.states, Confirmed: { Checkout: "Ordered", "Checkout/Ordered/Shipping": "Shipped" } } });
  assert.equal(shipped.status, 1);
  assert.match(shipped.stdout, /on "the payment succeeds" enters Checkout\/Ordered\/Shipping at "Shipped"; the agreed chart enters it at "Packing": "Paying" --> "Confirmed"/);
  const refund = check({ ...map, omits, events: { ...map.events, refund: null } }, fixture(`${pay}  Confirmed --> Browsing : refund\n`).input);
  assert.equal(refund.status, 1);
  assert.match(refund.stdout, /"refund" is hidden but changes Checkout from "Ordered" to "Cart" and Checkout\/Ordered\/Shipping from "Packing" to "\(not in Ordered\)": "Confirmed" --> "Browsing"/);
  const soldOut = check({ ...map, omits, states: { ...map.states, Paying: { Checkout: "Cart", Stock: "Sold out" }, Browsing: { Checkout: "Cart", Stock: "Sold out" }, Confirmed: { ...map.states.Confirmed, Stock: "Sold out" } } });
  assert.equal(soldOut.status, 1);
  assert.match(soldOut.stdout, /on "the payment succeeds" moves Checkout as the agreed chart does only when Stock is In stock: "Paying" --> "Confirmed"/);
  const declined = check({ ...map, omits, events: { ...map.events, "the payment is declined": "cancel the order" } });
  assert.equal(declined.status, 1);
  assert.match(declined.stdout, /on "the payment is declined" maps to "cancel the order", which no region in the map can take from there: "Paying" --> "Browsing"/);
  const reload = check({ ...map, omits, outside: { "reload the page": "the agreed chart has no page" } }, fixture(`${pay}  Confirmed --> Browsing : reload the page\n`).input);
  assert.equal(reload.status, 0, reload.text);
  assert.match(reload.stdout, /"reload the page" \(detailed only\): the agreed chart has no page/);
  const both = check({ ...map, omits, outside: { "tap Pay": "x" } });
  assert.match(both.stderr, /the map both names and puts outside the detailed event "tap Pay"/);
  // A self-loop on the parent re-enters the part at its start.
  const reorder = fixture(shop.replace("Ordered --> Cart : cancel the order", "Ordered --> Cart : cancel the order\n      Ordered --> Ordered : reorder"));
  const again = fixture(`${pay}  state "Shipped" as Gone\n  Confirmed --> Gone : the parcel leaves\n  Gone --> Confirmed : reorder\n`);
  const file = path.join(again.dir, "map.json");
  fs.writeFileSync(file, JSON.stringify({ ...map, omits: { "cancel the order": "x" },
    states: { ...map.states, Shipped: { Checkout: "Ordered", "Checkout/Ordered/Shipping": "Shipped" } },
    events: { ...map.events, "the parcel leaves": "the parcel leaves", reorder: "reorder" } }));
  const reentered = cmd("against", again.input, reorder.input, "--map", file);
  assert.equal(reentered.status, 0, reentered.text);
  const stuck = fixture(`${pay}  state "Shipped" as Gone\n  Confirmed --> Gone : the parcel leaves\n  Gone --> Gone : reorder\n`);
  const kept = cmd("against", stuck.input, reorder.input, "--map", file);
  assert.equal(kept.status, 1, kept.text);
  assert.match(kept.stdout, /on "reorder" enters Checkout\/Ordered\/Shipping at "Shipped"; the agreed chart enters it at "Packing": "Shipped"/);
  // Each detailed region maps on its own: a button region only the detail has, and a
  // step label that gives one step its own agreed event.
  const twoRegions = fixture(`stateDiagram-v2
  state "Phone" as Phone {
    state "Order" as Order {
      [*] --> Browsing
      state "Browsing" as Browsing
      state "Paying" as Paying
      state "Confirmed" as Confirmed
      Browsing --> Paying : tap Pay
      Paying --> Browsing : the payment is declined
      Paying --> Confirmed : the payment succeeds
      Confirmed --> Confirmed : the payment succeeds
    }
    --
    state "Button" as Button {
      [*] --> Idle
      state "Idle" as Idle
      state "Spinning" as Spinning
      Idle --> Spinning : tap Pay
      Spinning --> Idle : the payment succeeds
      Spinning --> Idle : the payment is declined
    }
  }
`);
  const perRegion = { ...map, omits: { "cancel the order": "x", "the parcel leaves": "y" },
    states: { ...map.states, Idle: {}, Spinning: {} },
    steps: { "Confirmed --> Confirmed on the payment succeeds": "reorder" } };
  fs.writeFileSync(path.join(twoRegions.dir, "map.json"), JSON.stringify(perRegion));
  const regionsAgree = cmd("against", twoRegions.input, reorder.input, "--map", path.join(twoRegions.dir, "map.json"));
  assert.equal(regionsAgree.status, 0, regionsAgree.text);
  const { steps: _unused, ...unlabeled } = perRegion;
  fs.writeFileSync(path.join(twoRegions.dir, "map.json"), JSON.stringify(unlabeled));
  const regionsDiffer = cmd("against", twoRegions.input, reorder.input, "--map", path.join(twoRegions.dir, "map.json"));
  assert.equal(regionsDiffer.status, 1, regionsDiffer.text);
  assert.match(regionsDiffer.stdout, /on "the payment succeeds" maps to "place the order", which no region in the map can take from there: "Confirmed"/);
  fs.writeFileSync(path.join(twoRegions.dir, "map.json"), JSON.stringify({ ...perRegion, states: { ...perRegion.states, Idle: { Checkout: "Cart" } } }));
  assert.match(cmd("against", twoRegions.input, reorder.input, "--map", path.join(twoRegions.dir, "map.json")).stderr,
    /the detailed regions "(Phone\/)?Order" and "(Phone\/)?Button" both map to "Checkout"/);
  const unmapped = check({ ...map, states: { Browsing: map.states.Browsing, Paying: map.states.Paying } });
  assert.equal(unmapped.status, 1);
  assert.match(unmapped.stderr, /the map does not place the detailed state "Confirmed"/);
  const unknown = check({ ...map, omits: { "ship faster": "x" } });
  assert.match(unknown.stderr, /the map omits "ship faster", which no step of the agreed chart has/);
});

test("coverage checks a table that accounts for every state of a chart, region by region", () => {
  const chart = fixture(here("door.mmd")).input;
  const table = (body) => {
    const file = path.join(fs.mkdtempSync(path.join(run, "coverage-")), "coverage.md");
    fs.writeFileSync(file, body);
    return cmd("coverage", chart, file);
  };
  const section = (region, rows) => `## ${region}\n\n| State | Status | Where |\n| --- | --- | --- |\n${rows.map(([s, st]) => `| ${s} | ${st} | x |`).join("\n")}\n\n`;
  const full = "# Door app\n\nProse with | a pipe is ignored.\n\n"
    + section("Door", [["Open", "built"], ["Locked", "built"]])
    + section("Door/Locked/Alarm", [["Disarmed", "partial"], ["Armed", "missing"], ["Ringing", "missing"]])
    + section("Door/Locked/Keypad", [["Idle", "not shown"], ["Typing", "built"]])
    + section("Siren", [["Quiet", "not shown"], ["Loud", "built"]]);
  const ok = table(full);
  assert.equal(ok.status, 0, ok.text);
  assert.match(ok.stdout, /9 states: 4 built, 1 partial, 2 missing, 2 not shown\./);
  // A state the chart gains, a state it no longer has, an unknown status, a missing region.
  const drift = table(full.replace("| Ringing | missing | x |\n", "").replace("| Typing | built |", "| Typing | done |")
    .replace("| Loud | built | x |", "| Loud | built | x |\n| Muted | missing | x |").replace(/## Door\/Locked\/Keypad[\s\S]*?(?=## Siren)/, ""));
  assert.equal(drift.status, 1);
  assert.match(drift.stdout, /"Door\/Locked\/Alarm" does not list the state "Ringing"/);
  assert.match(drift.stdout, /"Siren" has no state "Muted"/);
  assert.match(drift.stdout, /the region "Door\/Locked\/Keypad" has no "## Door\/Locked\/Keypad" section/);
  const status = table(full.replace("| Typing | built |", "| Typing | done |"));
  assert.match(status.stdout, /"Typing" has the status "done"; use built, partial, missing, not shown/);
  const extra = table(`${full}## Garage\n\n| State | Status |\n| --- | --- |\n| Shut | built |\n`);
  assert.match(extra.stdout, /the chart has no region "Garage"/);
});

test("compose joins separate charts through shared events and checks rules across them", () => {
  const dir = fs.mkdtempSync(path.join(run, "compose-"));
  for (const name of ["compose-app.mmd", "compose-practice.mmd"]) fs.writeFileSync(path.join(dir, name), here(name));
  const write = (body) => {
    const file = path.join(dir, `composition-${fs.readdirSync(dir).length}.mmd`);
    fs.writeFileSync(file, body);
    return file;
  };
  const base = [
    "stateDiagram-v2",
    "%% compose app: compose-app.mmd",
    "%% compose practice: compose-practice.mmd",
    "%% shared app: you're invited = practice: invite the patient",
    "%% shared practice: the patient joins = app: join",
    "%% shared practice: a record arrives = app: save a note",
    "%% outside practice: by the patient through the app",
    "%% never SameStatus: app: Connected & practice: Not connected -- Both sides show one connection.",
    "%% never SameStatusBack: practice: Connected & app: Not connected -- Both sides show one connection.",
    "%% never JoinedBothSides: practice: Connected & app: Invited -- The practice connects only when the patient joins.",
  ];
  const both = write(base.join("\n"));
  const lint = cmd("lint", both);
  assert.equal(lint.status, 0, lint.text);
  const flat = cmd("flat", both).stdout;
  // Each chart's regions sit under its own name; its states stay its own.
  assert.match(flat, /state "Patient app\/Your practice"/);
  assert.match(flat, /state "Practice\/Connection"/);
  // A shared event is one event: the follower's step is drawn on the owner's label.
  const practice = /state "Practice\/Connection" as (r\d+) \{([\s\S]*?)\n  \}/.exec(flat)[2];
  assert.match(practice, /--> r\d+s\d+ : join/);
  assert.match(practice, /: save a note/);
  assert.doesNotMatch(flat, /a record arrives/);
  // The same words in two charts are two events unless the composition shares them.
  assert.match(flat, /: new records arrive \(practice\)/);
  assert.match(flat, /%% event new records arrive \(app\) \| by your records source/);
  // Rules: the chart's own keep holding, named by chart; composition rules name chart: caption.
  const rules = cmd("rules", both);
  assert.equal(rules.status, 0, rules.text);
  assert.match(rules.stdout, /HOLDS practice\.OwedOnlyFromSharing/);
  assert.match(rules.stdout, /HOLDS SameStatus /);
  assert.match(rules.stdout, /4 hold, 0 violated/);
  // A note saved before the practice connects reaches no work: the follower keeps its needs.
  const event = cmd("query", both, "--event", "save a note").stdout;
  assert.match(event, /Practice\/Work\n      Idle --> Owed \(needs here: when Practice\/Connection is Connected/);
  // A follower's step also waits for the owner event's needs: the app is invited only when the
  // practice can invite.
  const invite = cmd("query", both, "--event", "invite the patient").stdout;
  assert.match(invite, /Patient app\/Your practice\n      Not connected --> Invited \(needs here: when Patient app\/Your practice is Not connected[^)]*; the event's own needs: when Practice\/Connection is Not connected/);
  // A cause's limit survives composition: a follower's step waits for the owner's cause too...
  fs.writeFileSync(path.join(dir, "compose-app-limited.mmd"), here("compose-app.mmd")
    .replace("%% events declared\n", "%% events declared\n%% cause you | needs not NNone | reason: you act on a practice you know\n"));
  const owned = cmd("query", write(base.join("\n").replace("compose-app.mmd", "compose-app-limited.mmd")), "--event", "save a note");
  assert.equal(owned.status, 0, owned.text);
  assert.match(owned.stdout, /Practice\/Work\n      Idle --> Owed \([^)]*the event's cause, you, always needs: unless Patient app\/Your practice is Not connected/);
  // ...and keeps its own cause's limit once drawn on the owner's event.
  fs.writeFileSync(path.join(dir, "compose-practice-limited.mmd"), here("compose-practice.mmd")
    .replace("%% events declared\n", "%% events declared\n%% cause the patient through the app | needs not PNone | reason: the patient acts on a practice that invited them\n"));
  const kept = cmd("query", write(base.join("\n").replace("compose-practice.mmd", "compose-practice-limited.mmd")), "--event", "join");
  assert.equal(kept.status, 0, kept.text);
  assert.match(kept.stdout, /Practice\/Connection\n      Invited --> Connected \([^)]*this step's cause, the patient through the app, always needs: unless Practice\/Connection is Not connected/);
  // Owner and follower with one cause name but different limits: query shows both.
  fs.writeFileSync(path.join(dir, "compose-practice-you.mmd"), here("compose-practice.mmd")
    .replace("%% event a record arrives | by the patient through the app |", "%% event a record arrives | by you |")
    .replace("%% events declared\n", "%% events declared\n%% cause you | needs not PInvited | reason: for the test\n"));
  const both2 = cmd("query", write(base.join("\n").replace("compose-app.mmd", "compose-app-limited.mmd").replace("compose-practice.mmd", "compose-practice-you.mmd")
    .replace("%% outside practice: by the patient through the app", "%% outside practice: by the patient through the app")), "--event", "save a note");
  assert.equal(both2.status, 0, both2.text);
  assert.match(both2.stdout, /Practice\/Work\n      Idle --> Owed \([^)]*this step's cause, you, always needs: unless Practice\/Connection is Invited[^)]*the event's cause, you, always needs: unless Patient app\/Your practice is Not connected/);
  // Negative control: joining on the wrong event breaks the status rule, and trace shows how.
  const wrong = write(base.join("\n").replace("%% shared practice: the patient joins = app: join", "%% shared practice: the patient joins = app: save a note"));
  const broken = cmd("trace", wrong);
  assert.equal(broken.status, 1, broken.text);
  assert.match(broken.stdout, /JoinedBothSides/);
  // An owner event that is itself relabeled onto another chart's event is refused.
  const chained = cmd("lint", write(base.join("\n").replace("= app: join", "= app: you're invited")));
  assert.equal(chained.status, 1);
  assert.match(chained.stdout, /"you're invited" in app is itself shared as another chart's event/);
  // An event caused outside its chart must be shared.
  const unshared = cmd("lint", write(base.filter((l) => !l.includes("a record arrives =")).join("\n")));
  assert.equal(unshared.status, 1);
  assert.match(unshared.stdout, /"a record arrives" in practice is caused by the patient through the app, outside practice, but no %% shared line says which event it is/);
  // A shared line names events each chart carries.
  const missing = cmd("rules", write(base.join("\n").replace("app: save a note", "app: save a nate")));
  assert.equal(missing.status, 1);
  assert.match(missing.text, /%% shared names "save a nate", which app does not carry/);
  // Verus judges drift on one chart's text, so it refuses a composition.
  const model = cmd("verus", both, "--module", "Both");
  assert.equal(model.status, 1);
  assert.match(model.text, /verus and check take one chart/);
  // A composition names chart: caption, and a rule's caption is looked up in that chart only.
  const loose = cmd("rules", write([...base.slice(0, 7), "%% never Loose: Connected & practice: Not connected -- x"].join("\n")));
  assert.equal(loose.status, 1);
  assert.match(loose.text, /name a composed chart as KEY: \.\.\./);
  // A template's rules still find a caption repeated across its instances once composed.
  fs.writeFileSync(path.join(dir, "lockers.mmd"), here("lockers.mmd"));
  const templated = cmd("rules", write(["stateDiagram-v2", "%% compose lockers: lockers.mmd", "%% compose practice: compose-practice.mmd"].join("\n")));
  assert.equal(templated.status, 0, templated.text);
  assert.match(templated.stdout, /HOLDS lockers\.NotBothFull_a/);
});

test("a nested composed step yields to its parent only while its own event's needs hold", () => {
  const dir = fs.mkdtempSync(path.join(run, "compose-nested-"));
  const put = (name, lines) => fs.writeFileSync(path.join(dir, name), lines.join("\n"));
  put("room.mmd", [
    "stateDiagram-v2",
    "%% events declared",
    "%% event the bell rings | by the school | Class ends.",
    "%% event the child is called | by the school | needs Ready | reason: only a ready child is called | The child goes to the desk.",
    "%% event get ready | by you | The child gets ready.",
    "state \"Room\" as Room {",
    "  state \"Place\" as Place {",
    "    [*] --> Class",
    "    state \"Out of class\" as Out",
    "    state \"In class\" as Class {",
    "      [*] --> Seat",
    "      state \"At the seat\" as Seat",
    "      state \"At the desk\" as Desk",
    "      Seat --> Desk : the child is called",
    "    }",
    "    Class --> Out : the bell rings",
    "  }",
    "  --",
    "  state \"Readiness\" as Readiness {",
    "    [*] --> NotReady",
    "    state \"Not ready\" as NotReady",
    "    state \"Ready\" as Ready",
    "    NotReady --> Ready : get ready",
    "  }",
    "}",
  ]);
  put("clock.mmd", [
    "stateDiagram-v2",
    "%% events declared",
    "%% event tick | by the clock | The clock ticks.",
    "state \"Clock\" as Clock {",
    "  [*] --> Going",
    "  state \"Going\" as Going",
    "  Going --> Going : tick",
    "}",
  ]);
  const both = path.join(dir, "both.mmd");
  put("both.mmd", [
    "stateDiagram-v2",
    "%% compose room: room.mmd",
    "%% compose clock: clock.mmd",
    "%% shared room: the bell rings = clock: tick",
    "%% shared room: the child is called = clock: tick",
    "%% outside room: by the school",
    "%% never LeavesOnlyWhenReady: room: Out of class & room: Not ready -- (expected to break: the bell ends class for an unready child)",
    "%% never CalledOnlyWhenReady: room: In class.At the desk & room: Not ready -- Only a ready child is called.",
  ]);
  // Before the fix, the nested step on tick discarded the parent's, so an unready child never left.
  const trace = cmd("trace", both);
  assert.equal(trace.status, 1, trace.text);
  assert.match(trace.stdout, /LeavesOnlyWhenReady/);
  assert.doesNotMatch(trace.stdout, /CalledOnlyWhenReady/);
  // While the nested step can fire, it still wins: a ready child is called, not sent out.
  const flat = cmd("flat", both).stdout;
  assert.match(flat, /: tick\n    %% blocked while: when Room\/Readiness is Ready\n/);
});

test("a step shared onto a command's refusal happens only when the owner's refusal can", () => {
  const dir = fs.mkdtempSync(path.join(run, "compose-refused-"));
  const put = (name, lines) => fs.writeFileSync(path.join(dir, name), lines.join("\n"));
  put("desk.mmd", [
    "stateDiagram-v2",
    "%% events declared",
    "%% event someone else closes the item | by another physician | Another physician closes it first.",
    "%% event close the item | by you | command on the item: Close | needs Open | reason: only an open item closes | You close it.",
    "state \"Desk\" as Desk {",
    "  state \"Item\" as Item {",
    "    [*] --> Open",
    "    state \"Open\" as Open",
    "    state \"Closed\" as Closed",
    "    Open --> Closed : close the item",
    "    Open --> Closed : someone else closes the item",
    "  }",
    "  --",
    "  state \"Your command on the item\" as Rc {",
    "    %% receipt the item",
    "  }",
    "}",
  ]);
  put("watcher.mmd", [
    "stateDiagram-v2",
    "%% events declared",
    "%% event a refusal is seen | by the desk | The watcher sees a refusal.",
    "state \"Watcher\" as Watcher {",
    "  [*] --> Unseen",
    "  state \"No refusal seen\" as Unseen",
    "  state \"Refusal seen\" as Seen",
    "  Unseen --> Seen : a refusal is seen",
    "}",
  ]);
  const both = path.join(dir, "both.mmd");
  put("both.mmd", [
    "stateDiagram-v2",
    "%% compose desk: desk.mmd",
    "%% compose watcher: watcher.mmd",
    "%% shared watcher: a refusal is seen = desk: your Close on the item is refused",
    "%% outside watcher: by the desk",
    "%% never SeenOnlyWhenClosed: watcher: Refusal seen & desk: Open -- A refusal comes only once the item no longer allows Close.",
  ]);
  // Before the fix, the watcher saw a refusal the desk could not make while the item was still open.
  const rules = cmd("rules", both);
  assert.equal(rules.status, 0, rules.text);
  assert.match(rules.stdout, /HOLDS SeenOnlyWhenClosed/);
});

test("a nested composed refusal yields to its parent only while the command's needs fail", () => {
  const dir = fs.mkdtempSync(path.join(run, "compose-refusal-"));
  const put = (name, lines) => fs.writeFileSync(path.join(dir, name), lines.join("\n"));
  put("room.mmd", [
    "stateDiagram-v2",
    "%% events declared",
    "%% event the bell rings | by the school | Class ends.",
    "%% event get ready | by you | The child gets ready.",
    "%% event lose focus | by you | The child loses focus.",
    "%% event answer the question | by you | command on the question: Answer | needs Ready | reason: only a ready child answers | The child answers.",
    "state \"Room\" as Room {",
    "  state \"Place\" as Place {",
    "    [*] --> Class",
    "    state \"Out of class\" as Out",
    "    state \"In class\" as Class {",
    "      [*] --> Rc",
    "      state \"Your command on the question\" as Rc {",
    "        %% receipt the question",
    "      }",
    "    }",
    "    Class --> Out : the bell rings",
    "  }",
    "  --",
    "  state \"Readiness\" as Readiness {",
    "    [*] --> NotReady",
    "    state \"Not ready\" as NotReady",
    "    state \"Ready\" as Ready",
    "    NotReady --> Ready : get ready",
    "    Ready --> NotReady : lose focus",
    "    Ready --> Ready : answer the question",
    "  }",
    "}",
  ]);
  put("clock.mmd", [
    "stateDiagram-v2",
    "%% events declared",
    "%% event tick | by the clock | The clock ticks.",
    "state \"Clock\" as Clock {",
    "  [*] --> Going",
    "  state \"Going\" as Going",
    "  Going --> Going : tick",
    "}",
  ]);
  const both = path.join(dir, "both.mmd");
  put("both.mmd", [
    "stateDiagram-v2",
    "%% compose room: room.mmd",
    "%% compose clock: clock.mmd",
    "%% shared room: the bell rings = clock: tick",
    "%% shared room: your Answer on the question is refused = clock: tick",
    "%% outside room: by the school",
  ]);
  // Before the fix, a sending answer dropped the bell, so a ready child stayed in class.
  const flat = cmd("flat", both).stdout;
  assert.match(flat, /: tick\n    %% blocked while: unless Room\/Readiness is Ready\n/);
});
