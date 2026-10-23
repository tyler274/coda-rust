# Compile a diagram into Verus

Use [chart-to-verus.mjs](../scripts/chart-to-verus.mjs) for a diagram written in the
supported pstack `stateDiagram-v2` subset. It reuses pstack's Mermaid parser and
hierarchy flattening, with a Verus backend for the same transition semantics.
No TLA+ or TLC installation is used. See [provenance](../../../../verification/README.md#provenance)
and the retained [MIT license](../../../../verification/PSTACK-LICENSE).

From the repository root, this example generates and checks a two-region chart:

```sh
node .agents/skills/coda-statechart/scripts/chart-to-verus.mjs \
  .agents/skills/coda-statechart/examples/sync.mmd \
  --module Sync --dir target/verification/sync-chart
python3 verification/check.py formal --spec target/verification/sync-chart/Sync_proof.rs
node .agents/skills/coda-statechart/scripts/chart-to-verus.mjs \
  .agents/skills/coda-statechart/examples/sync.mmd \
  --module Sync --dir target/verification/sync-chart --check
```

The existing [pinned Verus setup](../../../../verification/README.md#commands)
supplies the verifier. Generation requires Node.js and no npm packages.
Use the feature's own directory and module name for maintained models.

## What the files own

- `Sync.rs` is generated behavior: finite region enums, initial-state predicate,
  one relation per event, the combined transition relation, and safety predicates.
  Change that behavior in the diagram, then regenerate.
- `Sync.map.json` connects diagram captions, transitions and rules to generated
  symbols and records the diagram and generated-source hashes. It is a generated
  correspondence artifact, not a second design document.
- `Sync_proof.rs` is the editable proof starter. It proves initialization,
  preservation of an inductive invariant, implication of the chart's safety
  rules, and safety along every finite initialized trace. It is created once and
  preserved during regeneration. Strengthen its invariant or add proof reasoning
  here; do not weaken the chart's safety rules to make it pass.

`--check` writes nothing. It detects a changed diagram, missing generated files,
or edits to the generated model/map. It checks correspondence, not the proof.
Regeneration refuses a model whose contents no longer match its generation hash,
so hand-written refinements are not silently overwritten. Preserve such work in a
separate module before regenerating. Re-run Verus after every behavioral update.

Keep additional data specifications and implementation refinements separate and
connected to the generated predicates. The compiler does not check that additional
Rust models or product code implement the chart; those connections remain explicit
proof and maintenance work. An editable proof entrypoint must still establish the
initialization, transition and safety obligations for the current generated model.

## Supported meaning

- A region is in one of its leaf states. Multiple initial arrows describe
  alternative starting states. Nested composites flatten into qualified captions;
  entry uses the composite's initial state, and parent exits cover its children.
  For an equally labeled child/parent edge, pstack's deepest source wins.
- Parallel regions use one top-level composite split by `--`. Nested parallelism
  is refused. Declare state identities globally without reusing them across regions.
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
node .agents/skills/coda-statechart/scripts/chart-to-verus.mjs \
  .agents/skills/coda-statechart/examples/sync.mmd --module Sync --trace
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

Run the compiler's contract checks with the real pinned verifier:

```sh
node --test .agents/skills/coda-statechart/scripts/test/chart-to-verus.test.mjs
```

They exercise generated behavior, forbidden transitions and initial states,
nondeterminism, hierarchy, guard timing, proof strengthening, regeneration, and
drift detection. Artifacts and verifier logs stay under `target/verification/runs`.
