// Help text for statechart.mjs, one entry per subcommand (the PStack CLI layout).
export const USAGE = {
  verus: "node statechart.mjs verus CHART.mmd --module NAME [--dir OUT]",
  check: "node statechart.mjs check CHART.mmd --module NAME [--dir OUT]",
  trace: "node statechart.mjs trace CHART.mmd [--max-states N]",
  rules: "node statechart.mjs rules CHART.mmd [--rule NAME] [--max-cone N] [--max-states N]",
  compare: "node statechart.mjs compare FIRST.mmd SECOND.mmd [--rename MAP.json]",
  against: "node statechart.mjs against DETAIL.mmd AGREED.mmd --map MAP.json [--out LAYERED.mmd]",
  coverage: "node statechart.mjs coverage CHART.mmd COVERAGE.md",
  flat: "node statechart.mjs flat CHART.mmd",
  query: "node statechart.mjs query CHART.mmd [--state ID|CAPTION | --event LABEL | --region NAME]",
  lint: "node statechart.mjs lint CHART.mmd [--cause TEXT] [--repeats] [--unread]",
};
export const usage = (lines) => `Usage: ${lines.join("\n       ")}`;
export const more = (command) =>
  `Run node statechart.mjs ${command ? `${command} ` : ""}--help for details.`;

const SEMANTICS = `Regions are finite state variables. Equally labeled edges form one broadcast event:
every region with an enabled edge moves; other regions remain unchanged. Choices
remain nondeterministic. Guards read the pre-event state. Unlabeled edges are
separate events. %% when [not] StateId[, StateId] guards the preceding edge;
its label must include readable [guard words]. %% never Name: Caption & Caption
-- sentence forbids a combination of states (or composite state captions).
Parallel parts of a composite are regions active only while it is; templates,
declared events and "/ effect" labels expand before any backend reads the chart.
See references/compiler.md for the full syntax.`;

const VERUS = `verus CHART.mmd --module NAME [--dir OUT]
  Compile the chart to Verus. Writes NAME.rs (generated behavior), NAME_steps.rs
  (one generated preservation lemma per event), NAME.map.json (diagram
  correspondence) and NAME_proof.rs (editable proof starter, created only when
  absent) in OUT (default: the chart's folder). Existing generated files may be
  refreshed; hand-edited generated files are refused.

  The starter checks initialization, inductive safety, and finite traces. It may
  need a stronger inductive invariant. A failed obligation is not automatically a
  reachable design bug. This does not infer data guards, timers, fairness, runtime
  atomicity, or production-code conformance. No verifier is run by this command.

${SEMANTICS}`;

const CHECK = `check CHART.mmd --module NAME [--dir OUT]
  Write nothing; fail if NAME.rs, NAME_steps.rs or NAME.map.json differ from what the chart
  generates, or NAME_proof.rs is missing. Checks correspondence, not the proof.`;

const TRACE = `trace CHART.mmd [--max-states N]
  Breadth-first search of the same finite chart, writing no files. Prints the
  shortest path to a %% never violation (or a generated scope rule) as numbered
  steps and a Mermaid diagram. --max-states defaults to 100000; reaching it is
  INCOMPLETE (exit 3), never a pass. An exhausted search covers only chart rules,
  not data proofs or progress.`;

const RULES = `rules CHART.mmd [--rule NAME] [--max-cone N] [--max-states N]
  Check each rule (or only NAME) on its cone, a small set of regions, for charts
  too large for trace. Writes no files. Per rule: HOLDS, VIOLATION with the
  shortest path, INCONCLUSIVE, or INCOMPLETE (--max-states, default 100000).

  The cone starts as the rule's regions and their enclosing regions. A region its
  steps' guards, needs or part conditions read but that is not in the cone is
  free: at every step it may hold any value, one value per region per step read
  alike by every guard (a part whose composite is in the cone stays consistent
  with it). Every whole-chart step restricted to the cone is a cone step or
  leaves the cone unchanged, so every reachable chart state projects to a
  reachable cone state: HOLDS is sound. A violation path that read no free region
  is real, since every other region always has an option (a move or staying).
  Otherwise the free regions that path read join the cone (with their enclosing
  regions) and the search repeats, up to --max-cone regions (default 12); a
  violation still reading free regions is INCONCLUSIVE: raise it or run trace.

  Exit: 0 every rule holds; 1 a violation; 3 inconclusive or incomplete.`;

const COMPARE = `compare FIRST.mmd SECOND.mmd [--rename MAP.json]
  Compare two charts by their flat models, after templates, families, groups and
  effects expand: regions (and their activity), states and starts, steps with
  their guards, needs, part conditions, entry targets and "may", declared events
  (cause, needs, intent) and %% never rules (by the states they forbid, not their
  names). Lines starting "-" are only in FIRST, "+" only in SECOND.
  --rename reads {"regions": {...}, "states": {...}, "events": {...}}, each
  mapping a FIRST name to its SECOND name; state renames apply in every region.
  Exit: 0 the same flat model; 1 differences or an invalid chart.`;

const COVERAGE = `coverage CHART.mmd COVERAGE.md
  Check a Markdown coverage table that accounts for every state of the chart, such
  as what an app shows for each. One "## REGION" heading per region of the
  expanded chart, in its caption, each followed by a table whose first column is
  the state and whose second is its status: built, partial, missing or not shown.
  Other columns are free text; prose goes under "#" headings. A region's
  "(not in ...)" value is not listed. Reports missing, unknown or repeated regions
  and states and unknown statuses, then the count for each status.
  Exit: 0 the table matches the chart; 1 problems or an invalid chart.`;

const AGAINST = `against DETAIL.mmd AGREED.mmd --map MAP.json [--out LAYERED.mmd]
  Check a chart that details part of another against it, through one map in the
  agreed chart's words (adapted from pstack's --against):
    {"states": {DETAIL STATE: {AGREED REGION: AGREED STATE, ...}},
     "events": {DETAIL EVENT: AGREED EVENT or null},
     "omits":  {AGREED EVENT: "why the detail leaves it out"},
     "outside": {DETAIL EVENT: "why the agreed chart does not model it"},
     "steps":   {"FROM --> TO on EVENT": AGREED EVENT or null}}
  Every detailed state and event must be mapped; null marks a hidden step that
  only the detail has, which must keep the agreed state. A steps entry labels one
  detailed step, overriding its event's label there. Each detailed region maps on
  its own: the agreed regions its states name are its scope (one detailed region
  per agreed region), and a region whose states map to {} only the detail has. A
  scoped part left out of an entry is inactive while its parent is elsewhere. An event under outside, such as a page reload that
  the agreed chart abstracts away, is not compared and is printed with its
  reason. Both directions are checked:
  - each detailed step must be an agreed step in every scope region on the mapped
    event (a region the agreed event does not move may stay), and must not need a
    condition the mapped state rules out; an entered part must land where the
    agreed step enters it;
  - each agreed step in scope, from a state the detail can be in, must be shown by
    a detailed step, unless omits names its event with a reason.
  Conditions the matched agreed steps read outside the scope are listed as what
  the detail assumes. Starting states are not compared. --out writes the layered
  chart: each agreed state as a box holding the detailed states mapped to it.
  Exit: 0 both directions agree; 1 differences or an invalid map or chart.`;

const FLAT = `flat CHART.mmd
  Print the fully expanded chart as plain Mermaid: one region per holder, leaf
  states with qualified captions, concrete event labels (templates, families and
  groups expanded), guards as %% when lines and needs as %% event lines. A chart
  without parallel parts reads back to the same flat model. Scoped activity,
  part conditions, entry targets and rules are listed as comments.`;

const QUERY = `query CHART.mmd [--state ID|CAPTION | --event LABEL | --region NAME]
  --state   every way in and out of a state (by id, or caption; a composite
            caption covers its nested states), with event, guard, cause, needs
            and intent; entries from a parent, closing, and edges that read it.
  --event   an event's cause, needs and intent, and every reacting region and
            edge. A family pattern or group name lists its carried members.
  --region  a summary: scope, instance, states, starts, edges, parts, the regions
            it reads and that read it, rules, unreachable states and dead ends.
  Without a flag: chart size, largest regions, widest events, and edges that
  return a region to its start.`;

const LINT = `lint CHART.mmd [--cause TEXT] [--repeats] [--unread]
  Static findings over the flat model (exit 1 when there are any):
  - states unreachable from a region's starts, ignoring guards and other regions;
  - dead edges: a part's edge on an event that always closes or re-enters an
    enclosing composite, or conditions that name states which cannot be active
    together (a part's state while its composite is elsewhere);
  - undeclared or unlabeled steps under %% events declared;
  - declared events nothing carries, and families no instance binds;
  - group members that are neither declared events nor step labels;
  - %% lines that start with a construct keyword but were read as comments;
  - with --cause TEXT: every limit on an event declared "by TEXT" (case and
    surrounding spaces ignored) that states no reason: a guard on one of its
    moves, or its needs. Tag a limit with "| reason: free text" after the ids of
    %% when, after the ids of needs, or at the end of a %% never line;
  - with --repeats: each set of events that make the same move together on three or
    more moves of the flat model and that no one %% events group already names;
  - with --unread: each state other than a region's start that nothing outside its
    own region reads (no %% never rule, no guard or needs on another region's move,
    no part scoped to it), and each region none of whose states is read. Keep one
    only when the reader sees it; otherwise it is mechanism for the implementation,
    not the chart. Run it on a composition too, whose rules read across charts.
  Unreachable-ignoring-guards is structural: a state it lists is never reached;
  one it does not list may still be unreachable once guards are considered.`;

export const HELP = {
  "": `${usage(Object.values(USAGE))}

Commands:
  verus   compile the chart to a Verus model, map and proof starter
  check   detect drift between the chart and its generated files
  trace   shortest reachable violation of the chart's rules
  rules   check each rule on a small cone of regions (large charts)
  compare differences between two charts' flat models
  against check a detailed chart against the chart it details, through a map
  coverage check a table that accounts for every state of the chart
  flat    print the expanded chart as plain flat Mermaid
  query   ways in and out of a state, who reacts to an event, a region summary
  lint    unreachable states, dead edges, and event-declaration problems

CHART may be a composition: %% compose lines join separate charts through shared
events (references/compiler.md, Compositions). verus and check judge drift on the
composition and every chart it composes.

${more()}

Exit: 0 success, completed search or no findings; 1 violation, findings, invalid
chart, drift or I/O error; 2 invalid arguments; 3 incomplete search.`,
  verus: VERUS,
  check: CHECK,
  trace: TRACE,
  rules: RULES,
  compare: COMPARE,
  against: AGAINST,
  coverage: COVERAGE,
  flat: FLAT,
  query: QUERY,
  lint: LINT,
};
