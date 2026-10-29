// Export a chart's whole reachable graph for the HTML explorer, using the same step
// semantics as `trace`, `journey` and the Verus model, so the page can't drift from them.
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { compile } from "./chart-verus.mjs";
import { stepper } from "./chart-search.mjs";
import { readJourneys } from "./chart-journey.mjs";

const LIMIT = 20000;
const TEMPLATE = path.join(path.dirname(fileURLToPath(import.meta.url)), "explore-template.html");

export function explore(chartText, { title, journeyText } = {}) {
  const { ir, parsed } = compile(chartText, "Chart");
  const { regions, actions, rules } = ir;
  const { post, starts } = stepper(ir);
  const scope = regions.map((_, i) => i);
  const states = [], index = new Map(), edges = [];
  const add = (s) => {
    const k = JSON.stringify(s);
    if (!index.has(k)) {
      if (states.length === LIMIT) throw new Error(`more than ${LIMIT} reachable states; the explorer needs a smaller chart`);
      index.set(k, states.length);
      states.push(s);
    }
    return index.get(k);
  };
  const start = [...starts(scope)].map(add);
  for (let i = 0; i < states.length; i++) {
    const s = states[i];
    for (const action of actions) {
      if (!action.label) continue;
      const seen = new Set();
      for (const t of post(action, scope, (ri) => s[ri])) {
        const to = add(t);
        if (seen.has(to)) continue;
        seen.add(to);
        edges.push({ from: i, to, event: action.label });
      }
    }
  }
  const events = Object.fromEntries((parsed.declared ?? []).map((d) => [d.label, { cause: d.cause ?? "", intent: d.intent ?? "" }]));
  const ruleList = rules.map((r) => ({
    name: r.name,
    sentence: r.sentence,
    broken: states.flatMap((s, i) => (r.violated((ri) => s[ri]) ? [i] : [])),
  }));
  // Journeys replay as one concrete path over the exported graph, so the page can show
  // each step: an edge carrying the step's event to a state where every named state
  // holds. `journey` remains the check, over every state a step can reach.
  const holdsAll = (s, expect) => expect.every((caption) => s.includes(caption));
  const ended = new Map();
  const journeys = (journeyText ? readJourneys(journeyText).journeys : []).map((j) => {
    let at = j.from ? ended.get(j.from) : start[0];
    const steps = [];
    if (at === undefined) return { name: j.name, from: j.from, steps, broken: `From: ${j.from}` };
    for (const step of j.steps) {
      const edge = edges.find((e) => e.from === at && e.event === step.event && holdsAll(states[e.to], step.expect));
      if (!edge) return { name: j.name, from: j.from ?? null, steps, broken: step.event };
      steps.push({ event: step.event, expect: step.expect, state: edge.to });
      at = edge.to;
    }
    ended.set(j.name, at);
    return { name: j.name, from: j.from ?? null, steps };
  });
  return {
    title,
    regions: regions.map((r) => ({ caption: r.caption, states: [...r.states], initial: [...r.initial] })),
    states, start, edges, events, rules: ruleList, journeys,
    source: chartText,
  };
}

export function writeExplorer(model, out) {
  const json = JSON.stringify(model).replace(/</g, "\\u003c");
  const html = fs.readFileSync(TEMPLATE, "utf8")
    .replace("__TITLE__", (model.title ?? "Chart explorer").replace(/[<&]/g, ""))
    .replace("__MODEL__", () => json);
  fs.writeFileSync(out, html);
  return { states: model.states.length, edges: model.edges.length };
}
