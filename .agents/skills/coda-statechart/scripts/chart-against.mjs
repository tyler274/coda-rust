// Check a detailed chart against the chart it details, through one map that labels
// the detailed chart in the agreed chart's words. Adapted from the --against comparison
// in cursor/plugins pstack (statechart from-spec); see PSTACK-LICENSE.
//
// The map is JSON:
//   { "states": { DETAIL STATE: { AGREED REGION: AGREED STATE, ... } },
//     "events": { DETAIL EVENT: AGREED EVENT | null },
//     "omits":  { AGREED EVENT: "why the detailed chart leaves it out" },
//     "outside": { DETAIL EVENT: "why the agreed chart does not model it" },
//     "steps":  { "FROM --> TO on EVENT": AGREED EVENT | null } }
// Every detailed state and event must be mapped. A null event is a hidden step: the
// detailed chart may add it as long as it keeps the agreed state. A step entry labels
// one detailed step, overriding its event's label there, as pstack labels a step from
// its state and action. Each detailed region is mapped on its own: the agreed regions
// its states name are its scope, and a region whose states map to {} only the detail
// has. A scoped part may be left out of a state's entry while its parent region is
// elsewhere, and reads as inactive there. An event
// listed in outside (such as a page reload the agreed chart abstracts away) is not
// compared; it is printed with its reason.
//
// Two directions, as in verification/README.md: every detailed step must be an agreed
// step (or hidden), and every agreed step in scope from a state the detailed chart
// reaches must be shown by it, unless the map says why it is omitted. Conditions an
// agreed step reads outside the scope are listed as what the detailed chart assumes.
// Starting states are not compared.
import { compile } from "./chart-verus.mjs";
import { limitWords } from "./chart-flat.mjs";

const quote = (s) => `"${s}"`;

export function readMap(json) {
  const map = JSON.parse(json);
  if (!map || typeof map !== "object" || Array.isArray(map)) throw new Error("the map must be a JSON object");
  for (const key of Object.keys(map))
    if (!["states", "events", "omits", "outside", "steps"].includes(key)) throw new Error(`the map has "${key}"; use only states, events, omits, outside and steps`);
  return { states: map.states ?? {}, events: map.events ?? {}, omits: map.omits ?? {}, outside: map.outside ?? {}, steps: map.steps ?? {} };
}

export function against(detailText, agreedText, map) {
  const detail = compile(detailText, "Detail").ir;
  const agreed = compile(agreedText, "Agreed").ir;
  const regions = new Map(agreed.regions.map((r) => [r.caption, r]));
  const agreedEvents = new Set(agreed.regions.flatMap((r) => r.transitions.map((t) => t.label)).filter(Boolean));
  const problems = [];

  // Validate the map against both charts.
  const home = new Map(); // detailed state -> its region
  for (const r of detail.regions)
    for (const v of r.values) {
      if (home.has(v)) problems.push(`the detailed state ${quote(v)} is in two regions; give it one caption per region`);
      home.set(v, r);
    }
  const detailStates = new Set(home.keys());
  const stepKey = (t) => `${t.from} --> ${t.to} on ${t.label}`;
  const detailSteps = new Set(detail.regions.flatMap((r) => r.transitions.map(stepKey)));
  for (const [k, a] of Object.entries(map.steps)) {
    if (!detailSteps.has(k)) problems.push(`the map labels the step ${quote(k)}, which the detailed chart does not have`);
    if (a !== null && !agreedEvents.has(a)) problems.push(`the map labels ${quote(k)} as ${quote(a)}, which no step of the agreed chart has`);
  }
  const detailEvents = new Set(detail.regions.flatMap((r) => r.transitions.map((t) => t.label)).filter(Boolean));
  for (const s of detailStates) if (!Object.hasOwn(map.states, s)) problems.push(`the map does not place the detailed state ${quote(s)}`);
  for (const s of Object.keys(map.states)) if (!detailStates.has(s)) problems.push(`the map places ${quote(s)}, which the detailed chart does not have`);
  for (const e of detailEvents) {
    const named = Object.hasOwn(map.events, e), out = Object.hasOwn(map.outside, e);
    if (!named && !out) problems.push(`the map does not name the detailed event ${quote(e)}; give an agreed event, null for a hidden step, or a reason under outside`);
    if (named && out) problems.push(`the map both names and puts outside the detailed event ${quote(e)}`);
  }
  for (const e of Object.keys(map.outside)) if (!detailEvents.has(e)) problems.push(`the map puts ${quote(e)} outside, which the detailed chart does not have`);
  for (const [e, a] of Object.entries(map.events)) {
    if (!detailEvents.has(e)) problems.push(`the map names ${quote(e)}, which the detailed chart does not have`);
    if (a !== null && !agreedEvents.has(a)) problems.push(`the map sends ${quote(e)} to ${quote(a)}, which no step of the agreed chart has`);
  }
  for (const e of Object.keys(map.omits)) if (!agreedEvents.has(e)) problems.push(`the map omits ${quote(e)}, which no step of the agreed chart has`);
  // Each detailed region's scope: the agreed regions its states name. One agreed
  // region is detailed by at most one detailed region.
  const scopes = new Map(detail.regions.map((r) => [r, new Set()]));
  for (const [s, image] of Object.entries(map.states))
    for (const [r, v] of Object.entries(image ?? {})) {
      const region = regions.get(r);
      if (!region) { problems.push(`the map places ${quote(s)} in ${quote(r)}, which is not a region of the agreed chart`); continue; }
      if (!region.values.includes(v)) problems.push(`the map places ${quote(s)} at ${quote(v)}, which ${quote(r)} does not have`);
      if (home.has(s)) scopes.get(home.get(s)).add(r);
    }
  const owner = new Map();
  for (const [d, list] of scopes)
    for (const r of list) {
      if (owner.has(r)) problems.push(`the detailed regions ${quote(owner.get(r).caption)} and ${quote(d.caption)} both map to ${quote(r)}; one detailed region details each agreed region`);
      owner.set(r, d);
    }
  if (problems.length) throw new Error(problems.join("\n"));

  // The agreed state of every scope region for a detailed state. A scoped part whose
  // parent is elsewhere is inactive; any other region the entry leaves out is an error.
  const image = new Map();
  for (const s of detailStates) {
    const full = {};
    for (const r of scopes.get(home.get(s))) {
      const region = regions.get(r);
      const given = map.states[s][r];
      const parent = region.scope && (map.states[s][region.scope.region] ?? null);
      if (given !== undefined) full[r] = given;
      else if (region.scope && parent !== null && parent !== region.scope.state) full[r] = region.inactive;
      else problems.push(`the map does not say which state of ${quote(r)} ${quote(s)} is in`);
    }
    image.set(s, full);
  }
  if (problems.length) throw new Error(problems.join("\n"));

  const holds = (condition, at, scope) => Object.entries(condition.states).every(([r, list]) =>
    !scope.has(r) || (list.includes(at[r]) !== Boolean(condition.not)));
  const outside = (condition, scope) => Object.keys(condition.states).some((r) => !scope.has(r));
  const conditions = (t) => [t.guard, t.needs, ...(t.within ?? [])].filter(Boolean);

  // Differences that repeat for several detailed states are printed once, naming them.
  const grouped = new Map();
  const note = (text, t) => {
    if (!grouped.has(text)) grouped.set(text, new Set());
    grouped.get(text).add(t.from === t.to ? quote(t.from) : `${quote(t.from)} --> ${quote(t.to)}`);
  };
  const assumed = new Set(), shown = new Set();
  for (const r of detail.regions)
    for (const t of r.transitions) {
      const scope = scopes.get(r);
      if (!scope.size || Object.hasOwn(map.outside, t.label)) continue;
      const from = image.get(t.from), to = image.get(t.to);
      const event = Object.hasOwn(map.steps, stepKey(t)) ? map.steps[stepKey(t)] : map.events[t.label];
      const on = `on ${quote(t.label)}`;
      if (event === null) {
        const moved = [...scope].filter((g) => from[g] !== to[g]);
        if (moved.length) note(`${quote(t.label)} is hidden but changes ${moved.map((g) => `${g} from ${quote(from[g])} to ${quote(to[g])}`).join(" and ")}`, t);
        continue;
      }
      // The agreed event must be able to happen here: some scope region takes it.
      let taken = false;
      const before = [...grouped.values()].reduce((n, set) => n + set.size, 0);
      for (const g of scope) {
        const region = regions.get(g);
        const [a, b] = [from[g], to[g]];
        if (region.scope) {
          // A parent step into this part's state (a self-loop included) enters the part
          // at the agreed step's entry target, or at its start.
          const parent = regions.get(region.scope.region);
          const entering = parent.transitions.filter((u) => u.label === event && u.to === region.scope.state
            && u.from === (from[region.scope.region] ?? u.from));
          if (entering.length && b !== region.inactive) {
            const allowed = [...new Set(entering.flatMap((u) => u.enter?.[g] ?? [...region.initial]))];
            if (!allowed.includes(b)) note(`${on} enters ${g} at ${quote(b)}; the agreed chart enters it at ${allowed.map(quote).join(" or ")}`, t);
            continue;
          }
          if (a === region.inactive || b === region.inactive) continue; // the parent leaves or stays out
        }
        const candidates = region.transitions.filter((u) => u.label === event && u.from === a);
        const match = candidates.filter((u) => u.to === b);
        if (!match.length) {
          if (a === b && !candidates.length) continue; // this region does not react
          note(a === b
            ? `${on} keeps ${g} at ${quote(a)}; on ${quote(event)} the agreed chart moves it to ${[...new Set(candidates.map((u) => quote(u.to)))].join(" or ")}`
            : `${on} moves ${g} from ${quote(a)} to ${quote(b)}; on ${quote(event)} the agreed chart ${candidates.length ? `moves it to ${[...new Set(candidates.map((u) => quote(u.to)))].join(" or ")}` : "has no step from there"}`, t);
          continue;
        }
        for (const u of match) {
          const blocked = conditions(u).filter((c) => !holds(c, from, scope));
          if (blocked.length) {
            note(`${on} moves ${g} as the agreed chart does only ${blocked.map(limitWords).join(" and ")}`, t);
            continue;
          }
          taken = true;
          shown.add(u);
          for (const c of conditions(u)) if (outside(c, scope)) assumed.add(`${quote(event)} (${g}: ${quote(u.from)} --> ${quote(u.to)}) ${limitWords(c)}`);
        }
      }
      if (taken || [...grouped.values()].reduce((n, set) => n + set.size, 0) > before) continue;
      if ([...scope].some((g) => regions.get(g).transitions.some((u) => u.label === event)))
        note(`${on} maps to ${quote(event)}, which no region in the map can take from there`, t);
      else assumed.add(`${quote(event)} moves only regions outside the map`);
    }

  // Agreed steps in scope, from a state the detailed chart can be in, that it never shows.
  const missing = new Map(), omitted = new Set();
  for (const [g, d] of owner) {
    const reached = new Set(d.values.map((v) => image.get(v)[g]));
    for (const u of regions.get(g).transitions) {
      if (!reached.has(u.from) || shown.has(u)) continue;
      if (Object.hasOwn(map.omits, u.label)) { omitted.add(`${quote(u.label)}: ${map.omits[u.label]}`); continue; }
      const condition = [u.guard && limitWords(u.guard), u.needs && `needs ${limitWords(u.needs)}`].filter(Boolean).join("; ");
      const key = `${g}: from ${quote(u.from)} on ${quote(u.label)}`;
      if (!missing.has(key)) missing.set(key, new Set());
      missing.get(key).add(`${quote(u.to)}${condition ? ` (${condition})` : ""}`);
    }
  }

  return {
    disallowed: [...grouped].map(([text, steps]) => `${text}: ${[...steps].sort().join(", ")}`).sort(),
    missing: [...missing].map(([key, to]) => `${key} to ${[...to].sort().join(" or ")}`).sort(),
    assumed: [...assumed].sort(),
    omitted: [...omitted, ...Object.entries(map.outside).map(([e, why]) => `${quote(e)} (detailed only): ${why}`)].sort(),
    layered: layered(detail, image, (t) => Object.hasOwn(map.steps, stepKey(t)) ? map.steps[stepKey(t)] : map.events[t.label]),
  };
}

// The detailed chart drawn inside the agreed states it maps to, as pstack's layered chart.
function layered(detail, image, events) {
  const boxes = new Map();
  for (const [s, i] of image) {
    const key = Object.entries(i).filter(([, v]) => !v.startsWith("(not in ")).map(([r, v]) => `${r}: ${v}`).join(" / ");
    if (!boxes.has(key)) boxes.set(key, []);
    boxes.get(key).push(s);
  }
  const id = new Map([...image.keys()].map((s, n) => [s, `d${n}`]));
  const text = (s) => s.replaceAll('"', "'");
  const lines = ["stateDiagram-v2", "%% Generated by statechart.mjs against: each box is an agreed state; the states inside detail it."];
  [...boxes].forEach(([key, list], n) => {
    lines.push(`state "${text(key)}" as a${n} {`);
    for (const s of list) lines.push(`  state "${text(s)}" as ${id.get(s)}`);
    lines.push("}");
  });
  for (const r of detail.regions) {
    for (const s of r.initial) lines.push(`[*] --> ${id.get(s)}`);
    for (const t of r.transitions) {
      const e = events(t);
      const note = e === undefined ? " (outside)" : e === null ? " (hidden)" : e === t.label ? "" : ` (${text(e)})`;
      lines.push(`${id.get(t.from)} --> ${id.get(t.to)} : ${text(t.label)}${note}`);
    }
  }
  return `${lines.join("\n")}\n`;
}

// Print the result; returns the exit code: 0 when both directions agree, 1 otherwise.
export function printAgainst({ disallowed, missing, assumed, omitted }) {
  const section = (title, list) => { if (list.length) { console.log(`${title}:`); for (const f of list) console.log(`  ${f}`); } };
  section("Detailed steps the agreed chart does not allow", disallowed);
  section("Agreed steps the detailed chart never shows", missing);
  section("Conditions outside the map that the detailed chart assumes", assumed);
  section("Left out by the map", omitted);
  const total = disallowed.length + missing.length;
  console.log(total ? `${total} difference${total === 1 ? "" : "s"}.` : "The detailed chart follows the agreed chart in both directions.");
  return total ? 1 : 0;
}
