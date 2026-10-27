// Replay constructive journeys on a chart: the useful outcomes a person reaches, shown
// to be reachable by the same step semantics `trace` and the Verus backend use.
//
// The journeys are Markdown: one "## NAME" heading per journey, then numbered steps,
// each an event label, optionally followed by "→" and the states that must hold after it,
// separated by ";". Other lines are prose. A journey starts where the chart starts, or
// where an earlier journey ended when its first line is "From: NAME". In a composition a
// state is named as KEY: caption, as in its rules.
//
//   ## You ask and Duet answers
//   1. send your message → Sending
//   2. Duet receives your message → Sent
//
// Each step keeps every state the event can lead to (choices stay open) and then only
// those where its states hold; a journey fails at the first step nothing satisfies.
import { compile, locate } from "./chart-verus.mjs";
import { stepper } from "./chart-search.mjs";
import { guardWords } from "./chart-flat.mjs";

const quote = (s) => `"${s}"`;
const LIMIT = 50000;

export function readJourneys(text) {
  const journeys = [];
  const problems = [];
  let journey;
  text.split("\n").forEach((line, index) => {
    const at = `line ${index + 1}`;
    const heading = /^##\s+(.+?)\s*$/.exec(line);
    if (heading) {
      if (journeys.some((j) => j.name === heading[1])) problems.push(`${at}: the journey ${quote(heading[1])} is listed twice`);
      journeys.push(journey = { name: heading[1], from: null, steps: [], at });
      return;
    }
    if (/^#\s/.test(line)) { journey = undefined; return; }
    if (!journey) return;
    const from = /^From:\s*(.+?)\s*$/.exec(line);
    if (from) {
      if (journey.steps.length || journey.from) problems.push(`${at}: "From:" comes once, before the steps`);
      journey.from = from[1];
      return;
    }
    const step = /^\s*\d+\.\s+(.+?)\s*$/.exec(line);
    if (!step) return;
    const [event, expect] = step[1].split(/\s*(?:→|->)\s*/, 2);
    journey.steps.push({ event, expect: expect ? expect.split(";").map((s) => s.trim()).filter(Boolean) : [], at });
  });
  return { journeys, problems };
}

export function journeys(chartText, journeyText) {
  const { ir, parsed } = compile(chartText, "Chart");
  const { regions, actions } = ir;
  const { post, starts, holds } = stepper(ir);
  const scope = regions.map((_, i) => i);
  const byLabel = new Map(actions.filter((a) => a.label).map((a) => [a.label, a]));
  const { journeys: list, problems } = readJourneys(journeyText);
  const where = (s) => scope.map((ri) => `${regions[ri].caption}: ${s[ri]}`);
  // A state as written: "KEY: caption" in a composition, else its caption.
  const state = (words) => {
    const keyed = /^([A-Za-z][A-Za-z0-9_]*):\s*(.+)$/.exec(words);
    const chart = keyed && parsed.keys.get(keyed[1]);
    const hits = chart ? locate(regions, keyed[2], chart) : locate(regions, words);
    if (hits.length !== 1) throw new Error(`${quote(words)} must identify one state or composite in one region`);
    return hits[0];
  };
  const ended = new Map();
  const results = [];
  for (const j of list) {
    const fail = (why) => results.push({ name: j.name, failed: why });
    let frontier;
    if (j.from) {
      frontier = ended.get(j.from);
      if (!frontier) { fail(`${j.at}: "From:" names ${quote(j.from)}, which isn't an earlier journey that reached its end`); continue; }
    } else frontier = [...starts(scope)];
    if (!j.steps.length) { fail(`${j.at}: the journey has no steps`); continue; }
    let failed = null;
    for (const [n, step] of j.steps.entries()) {
      const action = byLabel.get(step.event);
      if (!action) { failed = `step ${n + 1} (${step.at}): the chart has no event ${quote(step.event)}`; break; }
      let wanted;
      try { wanted = step.expect.map(state); }
      catch (error) { failed = `step ${n + 1} (${step.at}): ${error.message}`; break; }
      const seen = new Set(), next = [], reached = [];
      scan: for (const s of frontier) for (const t of post(action, scope, (ri) => s[ri])) {
        const key = JSON.stringify(t);
        if (seen.has(key)) continue;
        seen.add(key);
        reached.push(t);
        if (wanted.every((w) => w.states.includes(t[w.ri]))) next.push(t);
        if (next.length > LIMIT) break scan;
      }
      if (next.length > LIMIT) { failed = `step ${n + 1} (${step.at}): more than ${LIMIT} states remain open; name states to narrow it`; break; }
      if (!reached.length) {
        // Why not, in the first state reached: each region the event moves, and for a
        // step from where it is, the conditions that don't hold.
        const s = frontier[0], pre = (ri) => s[ri];
        const moved = [...action.moves].map(([ri, moves]) => {
          const here = moves.filter((m) => m.from === s[ri]);
          if (!here.length) return `  ${regions[ri].caption} is ${s[ri]}; the event moves it only from ${[...new Set(moves.map((m) => m.from))].join(", ")}`;
          const unmet = here.flatMap((m) => [m.guard, m.needs, ...(m.within ?? [])].filter((g) => g && !holds(g, pre)).map(guardWords)
            .concat((m.unless ?? []).filter((c) => c.every((g) => holds(g, pre))).map((c) => `not while ${c.map(guardWords).join(" and ")}`)));
          return `  ${regions[ri].caption} is ${s[ri]}; its step from there needs: ${[...new Set(unmet)].join("; ")}`;
        });
        failed = [`step ${n + 1} (${step.at}): ${quote(step.event)} can't happen here. In the first state reached so far:`, ...moved.slice(0, 12)].join("\n");
        break;
      }
      if (!next.length) {
        const seenIn = wanted.map((w) => `  ${regions[w.ri].caption} reaches ${[...new Set(reached.map((t) => t[w.ri]))].join(", ")}`);
        failed = [`step ${n + 1} (${step.at}): after ${quote(step.event)}, ${step.expect.join("; ")} never holds:`, ...seenIn].join("\n");
        break;
      }
      frontier = next;
    }
    if (failed) { fail(failed); continue; }
    ended.set(j.name, frontier);
    results.push({ name: j.name, steps: j.steps.length, end: where(frontier[0]) });
  }
  return { problems, results };
}

export function printJourneys({ problems, results }) {
  for (const p of problems) console.log(p);
  for (const r of results) console.log(r.failed ? `FAILS ${r.name}\n${r.failed}` : `reaches ${r.name} (${r.steps} steps)`);
  const failed = results.filter((r) => r.failed).length;
  if (!results.length && !problems.length) console.log("No journeys: nothing shows a useful outcome is reachable.");
  else if (failed || problems.length) console.log(`${failed} of ${results.length} journeys fail${problems.length ? `, ${problems.length} problem${problems.length === 1 ? "" : "s"}` : ""}.`);
  else console.log(`${results.length} journeys: each reaches its outcome.`);
  return failed || problems.length || !results.length ? 1 : 0;
}
