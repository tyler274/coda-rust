# Compile a diagram into Verus

Use [statechart.mjs](../scripts/statechart.mjs) for a diagram written in the
supported pstack `stateDiagram-v2` subset. It reuses pstack's Mermaid parser and
hierarchy flattening, with a Verus backend for the same transition semantics.
No TLA+ or TLC installation is used. See [provenance](../../../../verification/README.md#provenance)
and the retained [MIT license](../../../../verification/PSTACK-LICENSE).
`node statechart.mjs --help` lists the commands; `COMMAND --help` explains each.
[chart-to-verus.mjs](../scripts/chart-to-verus.mjs) keeps its original arguments.

From the repository root, this example generates and checks a two-region chart:

```sh
node .agents/skills/coda-statechart/scripts/statechart.mjs verus \
  .agents/skills/coda-statechart/examples/sync.mmd \
  --module Sync --dir target/verification/sync-chart
python3 verification/check.py formal --spec target/verification/sync-chart/Sync_proof.rs
node .agents/skills/coda-statechart/scripts/statechart.mjs check \
  .agents/skills/coda-statechart/examples/sync.mmd \
  --module Sync --dir target/verification/sync-chart
```

The existing [pinned Verus setup](../../../../verification/README.md#commands)
supplies the verifier. Generation requires Node.js and no npm packages.
Use the feature's own directory and module name for maintained models.

## What the files own

- `Sync.rs` is generated behavior: finite region enums, initial-state predicate,
  one relation per event, the combined transition relation, and safety predicates.
  Change that behavior in the diagram, then regenerate.
- `Sync.map.json` connects diagram captions, transitions and rules to generated
  symbols and records the diagram and generated-source hashes. It also records
  scoped regions, template instances, declared events and stated reasons. It is a
  generated correspondence artifact, not a second design document.
- `Sync_proof.rs` is the editable proof starter. It proves initialization,
  preservation of an inductive invariant, implication of the chart's safety
  rules, and safety along every finite initialized trace. It is created once and
  preserved during regeneration. Strengthen its invariant or add proof reasoning
  here; do not weaken the chart's safety rules to make it pass.

`check` writes nothing. It detects a changed diagram, missing generated files,
or edits to the generated model/map. It checks correspondence, not the proof.
Regeneration refuses a model whose contents no longer match its generation hash,
so hand-written refinements are not silently overwritten. Preserve such work in a
separate module before regenerating. Re-run Verus after every behavioral update.

Keep additional data specifications and implementation refinements separate and
connected to the generated predicates. The compiler does not check that additional
Rust models or product code implement the chart; those connections remain explicit
proof and maintenance work. An editable proof entrypoint must still establish the
initialization, transition and safety obligations for the current generated model.

## Draw behavior, not taste

- Draw what an observer of the system can see happen. Guidance, tone and
  preference belong in the product's prompts, skills and evals, not in the chart.
- An actor's choices are nondeterministic, often `may` reactions. Keep hard limits
  few, and give each a reason that names what it protects.
- Every event declares who causes it. An event caused by an agent is an output
  of the system that runs that agent.
- Separate charts compose through shared declared events, not copied states.

## Supported meaning

- A region is in one of its leaf states. Multiple initial arrows describe
  alternative starting states. Nested composites flatten into qualified captions;
  entry uses the composite's initial state, and parent exits cover its children.
  For an equally labeled child/parent edge, pstack's deepest source wins.
- Parallel regions use a composite split by `--`, at the top level or nested (see
  below). Declare state identities globally without reusing them across regions.
- Equally labeled transitions are one broadcast event. Every region with an
  enabled edge participates; regions with no enabled edge stay unchanged. Multiple
  enabled destinations remain nondeterministic. Unlabeled edges are separate events.
  Self-loops remain transitions, and the trace semantics also allow stuttering.
- `%% when StateId, OtherId` immediately after an edge guards it on other regions.
  IDs in one region are alternatives; different regions must all match.
  `%% when not StateId, OtherId` requires none of those matches. All guards read
  the state before the event, including events shared by several regions. Include
  readable `[guard words]` in the label. Bracketed prose without `%% when` is refused.
- `%% never Name: Caption & OtherCaption -- explanation` forbids that combination.
  It uses display captions, including qualified nested captions, rather than IDs.
  A composite caption covers its descendants. Conditions must identify distinct
  regions. Rules may also forbid one state. Duplicate rule names are refused.
- Draw app closing or completion as transitions to the resulting visible state.
  Final `[*]` targets, unsupported Mermaid constructs, malformed formal annotations,
  guarded initial arrows, and ambiguous identities are errors.

Ordinary comments, display styling, and the upstream reserved `legend` element do
not add behavior. This is a defined Mermaid subset, not a general Mermaid or UML
execution engine. Event labels are names: words such as “after 5 seconds” do not
create a clock, and explanatory prose does not create data guards or side effects.
Model those semantics explicitly in the chart or its linked formal detail.

## Authoring constructs

Each construct expands into the flat model above before any backend reads it.
`flat CHART.mmd` prints the expansion. Fixtures under `scripts/test/` show each one.

**Nested parallel regions.** A state split by `--` anywhere in the chart makes each
part a region active only while its parent is in that state; otherwise the part is
in a generated `(not in P)` state, and a generated scope rule says so. Entering P
enters every part at its start; leaving P closes them all, and the exit wins over
any move inside. An edge into a part's state enters P there and the other parts at
their starts. `[*] --> S : label` inside a part is a labeled start: entering P on
that event starts the part at S. Regions are named `Region/State/Part`.

```mermaid
state Door {
  [*] --> Open
  Open --> Locked : you lock the door
  Locked --> Open : you unlock the door
  state Locked {
    state Alarm {
      [*] --> Disarmed
      [*] --> Armed : the timer locks the door
      Armed --> Ringing : someone forces the door
    }
    --
    state Keypad { ... }
  }
}
```

**Templates.** `%% template` as the first line of a composite that is alone in its
part makes it one region written once. Each `%% instance KEY: name = value, ...`
line makes one copy with `{name}` and `{key}` substituted in captions, labels,
guards, effects, `%% when` ids and `%% never` lines. Inside, ids are local;
elsewhere name one copy as `Id@KEY`. `%% only k1, k2` or `%% except k` after a
state or edge keeps it in just those instances. A `%% never` inside a template
holds for every instance, named `Name_KEY`.

```mermaid
state "Locker {key} ({size})" as Locker {
  %% template
  %% instance a: size = small, other = b
  %% instance b: size = large, other = a
  [*] --> Empty
  Empty --> Full : the courier fills locker {key} [while locker {other} is empty]
  %% when Empty@{other}
  %% never NotBothFull: Full & Full@{other} -- At most one locker holds a parcel.
}
```

**Declared events.** `%% event LABEL | by CAUSE | needs [not] Ids | INTENT` declares
who causes an event and what it is for; `needs` (optional) is a `%% when` added to
every edge carrying the event. A label with `{placeholders}` declares a family,
one event per template instance that binds them. `%% events NAME = a | b` names a
group: an edge labeled NAME stands for one edge per member. `%% events declared`
makes the chart strict: every labeled edge must carry a declared event, and
unlabeled steps are refused.

```mermaid
%% event the courier fills locker {key} | by the courier | needs Empty@{key}, Empty@{other} | The courier fills an empty locker while the other one is empty.
%% events a collection = you collect from locker {key}
```

**Effects.** UML `event [guard words] / effect` keeps `event` as the event name and
records the effect as the edge's intent. `/ may effect` makes the reaction
optional: when the edge is enabled, the region may also keep its state.
The same `may` words on one event in several regions are one choice: in a step,
every region with such an enabled edge takes it, or none does. Use this when one
decision changes several owners at once.

```mermaid
Idle --> Ringing : the bell is pressed / may ring the chime
Today --> Now : the review finishes / may raise the urgency
Opened --> New : the review finishes / may raise the urgency
```

**Reasons.** Any limit can say what it protects with free text after `| reason:`,
on `%% when Ids | reason: ...`, after a declaration's `needs Ids | reason: ...`, or
at the end of a `%% never` line. Reasons are recorded and printed; they change no
behavior. `lint --cause TEXT` lists every guard and needs on events caused by TEXT
that states no reason.

### Semantic choices

These follow Harel and UML where the brief did not decide:

- Exit wins: a move that leaves P closes every part, whatever the parts do on that
  event; a self-loop on P re-enters every part at its start.
- Deepest source wins among exits: an edge drawn from a part's state out of P
  takes priority over a same-label edge from P itself.
- Priority compares enabled steps only. Within a region, a step drawn from a
  deeper state takes priority over a same-label step from an enclosing state only
  while its own `%% when` holds; otherwise the enclosing step fires. `flat` lists
  the yielding step's condition as a `%% yields to a deeper step` comment.
- An edge may target a state one level inside a composite with parts; deeper
  targets, edges between parts, and labeled starts outside parts are refused.
- Initial arrows refuse guards and effects; labeled starts take only a label.
- `may` is a choice, made once per step for each distinct wording on the event. A
  chosen effect is taken wherever enabled and wins over a required move in that
  region; an unchosen one is absent. A region may keep its state when an own `may`
  edge is enabled and no required move or chosen effect is.
- A `%%` line that starts with a construct keyword but lacks its shape stays a
  comment; `lint` lists it.
- A `%% never` inside a template names its own instance's states first.

## What a result establishes

Successful generation establishes only that the diagram was translated. With no
`%% never` rules, the starter has no behavioral safety promise; the CLI says so.
A Verus pass establishes the generated safety obligations under the modeled
transition semantics. It does not establish eventual completion, fairness,
implementation atomicity, UI behavior or production conformance.

Unlike TLC's reachability search, the starter asks Verus for an inductive proof.
A safety rule can hold for every reachable state yet fail induction from another
unreachable state. Strengthen the editable inductive invariant when needed and
prove its initialization and preservation. Investigate an unsatisfied obligation
before reporting a design contradiction. The compiler does not extract a concrete
failure trace from Verus or reverse-translate arbitrary Rust into a diagram.

## Read a concrete chart failure

Search without generating or modifying files:

```sh
node .agents/skills/coda-statechart/scripts/statechart.mjs trace \
  .agents/skills/coda-statechart/examples/sync.mmd
```

Breadth-first search uses the same flattened regions, grouped events, resolved
state guards and forbidden combinations as generation. It prints the shortest
violating path as numbered user steps and a Mermaid path diagram. Alternative
initial states and nondeterministic destinations are explored. Stuttering cannot
shorten a safety violation. Exit 1 means a violation (or invalid input); 0 means
completed search, with an explicit notice if there were no rules. `--max-states N`
(default 100000) returns exit 3 and **incomplete** when it prevents exhaustion.
Interruption is also incomplete. Neither result proves progress or data properties.

The finite search can find a reachable chart bug after an induction failure, or
exhaust reachable states without finding one. In the latter case, strengthen the
inductive proof or investigate the abstraction. It does not extract counterexamples
from arbitrary Verus obligations. Keep identity, revision and collection rules in
linked data proofs rather than inventing chart states to manufacture safety rules.

### Rules on large charts

When `trace` is incomplete, `rules` checks each rule on its cone with the same step
semantics. The cone starts as the rule's regions and their enclosing regions. A
region its steps' guards, needs or part conditions read, but which is outside the
cone, is free: at every step it may hold any value, one per region per step, read
alike by every guard (a part whose composite is in the cone stays consistent with
it). Every whole-chart step restricted to the cone is then a cone step or leaves
the cone unchanged, so every reachable chart state projects to a reachable cone
state and **HOLDS is sound**. A violation path that read no free region is real:
every other region always has an option (a move or staying), so each step extends
to a whole-chart step. Otherwise the free regions that path read join the cone and
the search repeats, up to `--max-cone` regions; a violation that still reads a free
region is **inconclusive**. Exit 0 all hold, 1 a violation, 3 inconclusive or
incomplete. HOLDS covers chart rules only, like an exhausted trace.

## Inspect a chart

- `query` lists the ways in and out of a state, who reacts to an event (cause,
  needs, intent), or a region's summary; with no flag, the chart's size.
- `lint` lists states unreachable ignoring guards, dead edges, declaration
  problems and keyword lines read as comments; `--cause` adds unreasoned limits.
- `compare FIRST SECOND [--rename MAP.json]` diffs two flat models: regions,
  states, steps with their conditions, declared events and rules.
- `flat` prints the expansion; without parallel parts it reads back to the same
  flat model, with rules as comments.

Run the compiler's contract checks with the real pinned verifier:

```sh
node --test .agents/skills/coda-statechart/scripts/test/
```

They exercise generated behavior, forbidden transitions and initial states,
nondeterminism, hierarchy, guard timing, proof strengthening, regeneration, drift
detection, each authoring construct with a negative control, agreement between the
search and Verus on every reachable step, and the inspection commands. Artifacts
and verifier logs stay under `target/verification/runs`.
