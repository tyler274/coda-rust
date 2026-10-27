# Coda product verification

Coda ports [PStack](#provenance) to Verus and connects agreed product journeys to
Rust owners and running-app evidence. The five general skills establish or repair
verification workflows and are explicit-only. App-specific recipes own their
concrete journeys, models and harnesses; ordinary feature work does not require
the full workflow.

## Principles

1. **Start from agreed intent.** Carry Dev's decisions forward. Design the complete
   journey before choosing proof cases. The chart is the product's contract and the
   harness serves it; never encode how the harness works in the chart. Existing code and historical failures can
   challenge it; missing code cannot limit it. A checked draft is still a draft.
2. **Keep one owner.** Journey, charts and formal specification express one behavior.
   One feature recipe connects their proofs, actual code and app checks. Reuse real
   state owners and harnesses; avoid copied implementations and parallel ledgers.
3. **Match evidence to the claim.** Verus is the formal verifier. Distinguish design
   proof, actual Rust proof, app readback, clinical calibration and deployed evidence.
   State abstraction, trusted inputs, missing connections and progress assumptions.
4. **Make failure meaningful.** Challenge promises with relevant broken variants.
   Check the changed producer and its proof dependencies; an unchecked callee's
   postcondition can mask a break. Syntax errors and unrelated failures do not count.
   Investigate failures without weakening the agreement to match current behavior.
5. **Keep the workflow usable.** Preserve PStack's useful discipline, shorten repeated
   guidance and update affected representations together. Run real entry points,
   inspect visual results and durable effects, and retain evidence after cleanup.
6. **Stay within authority.** Skills grant no permission to change product meaning,
   delegate, publish or send. Maintenance alone repairs verification; carry forward
   implementation authority already granted by the task. Use only the needed steps.

## The everyday workflow

Agree on the journey and its promises → formalize and check that design → build or
audit the real Rust owners → exercise the actual experience → maintain the connections.
A design proof may finish before implementation. App tests may run before refinement
proofs exist. Neither substitutes for the missing boundary.

When intent changes, update journey, diagrams, proofs and app expectations together.
When implementation changes, investigate conformance against unchanged intent.
Routine layout or copy edits do not require new formal machinery.

## Keep the diagram and model in lockstep

Map meaningful states to predicates, edges to guarded transitions and effects, and
cross-capability promises to composition obligations. Check both directions:
intended behavior must not disappear from the model; modeled behavior must have a
design meaning. Use the existing feature recipe and source links for correspondence,
including omissions. Check both valid publication and complete discovery: use full
collections, preserve every accepted batch member, derive history from its owner,
and separate safety from conditional progress. A capability overview plus selected
proofs is not full coverage.

The [compiler](../.agents/skills/coda-statechart/references/compiler.md) shares one
flattened representation between Verus generation and finite reachability search.
`%% when` encodes state guards; `%% never` encodes actual forbidden combinations.
`statechart.mjs trace` prints a shortest violating path and numbered Mermaid diagram. A completed
search covers only the finite chart's rules; limits or interruption are incomplete.
An inductive Verus obligation can fail without a reachable chart violation.

## Connect the harness through events

The harness stays out of the chart and meets it only at declared events (Dev
2026-10-26). An agent's event is the harness's output, and each tool it admits is
one such event: the product host checks that event's `needs` before the effect
commits, and the model decides when to use it. Retries, timeouts, cadence, memory
and prompts stay in the harness; `infrastructure-gaps.md` names the events each
runtime guarantee backs. The runtime records chart events where each fact's owner
changes it, so a recorded run can be checked against the flat model, and `against`
checks a more detailed model of the same behavior. The harness has no chart of
its own; it is just the harness.

Do not force every promise into finite states. Identity, revisions, collections,
atomic batches and progress need linked data proofs and explicit assumptions.
Charts without safety rules establish transition correspondence only. Keep generated
behavior separate from editable proofs; `statechart.mjs check` detects generated drift. Review
manual data refinements and implementation connections separately.

## Feature specs

| Folder | Spec |
| --- | --- |
| [physician-experience](physician-experience/README.md) | The physician web app's ideal behavior as one statechart, with its Verus model and proof. |
| [duet-experience](duet-experience/README.md) | The Duet app's ideal behavior as one statechart, with its Verus model and proof. |
| [connected-care](connected-care/README.md) | How Duet and the physician web app meet: one composition of both graphs through shared events, with the rules that span them. |

## How the skills fit

| Skill | Use |
| --- | --- |
| [coda-statechart](../.agents/skills/coda-statechart/SKILL.md) | Discuss or revise meaningful states, transitions and promises. |
| [coda-create-formal-verification](../.agents/skills/coda-create-formal-verification/SKILL.md) | Encode design in Verus and establish real Rust proof connections. |
| [coda-create-verification](../.agents/skills/coda-create-verification/SKILL.md) | Create launch, doctor, drive, evidence and cleanup instructions. |
| [coda-maintain-formal-verification](../.agents/skills/coda-maintain-formal-verification/SKILL.md) | Reconcile diagram, data model, proof and code drift. |
| [coda-maintain-verification](../.agents/skills/coda-maintain-verification/SKILL.md) | Reconcile feature coverage through source inspection and live driving. |

The creators extend the same `.agents/skills/coda-verify-<app>/` recipe. These
engineering skills are separate from the runtime clinical catalog in `skills/`.
Implementation remains the product task; verification is not implicit permission
to redesign it. Maintenance follows changes, without a separate schedule.

## Choosing scope

Keep the complete user outcome in view. Include neighboring journeys when their
shared state or handoffs can break its promise. Prove the connections between owners;
separate passing models do not establish composition. Reuse a proved implementation
across surfaces only where assumptions hold; exercise each remaining app boundary.
Add mechanisms for observed failures or necessary guarantees, not hypothetical races.

## Commands

Install the pinned, checksum-verified Verus under ignored `target/verification/tools`,
after `rustup toolchain install <rust_toolchain> --profile minimal` for the
toolchain [verus.json](verus.json) names:

```sh
python3 verification/install-verus.py
python3 verification/check.py formal --spec path/to/the/feature-proof.rs
```

There is no default proof for an unspecified feature. The extra toolchain does not
change the workspace default. See [verus.json](verus.json) and the
[real-owner recipe](../.agents/skills/coda-create-formal-verification/references/verus.md#verify-a-real-rust-owner).
The [compiler reference](../.agents/skills/coda-statechart/references/compiler.md)
provides a product-neutral chart example and contract-test command. App recipes
supply their own doctor, runtime, mutation and browser commands.

Checks print a new evidence directory under `target/verification/runs`; `--out`
selects another new directory. Logs survive scratch cleanup.

## Provenance

The baseline is [PStack at eb4db2b](https://github.com/cursor/plugins/tree/eb4db2b55b2b7dec3d9282754cddf0a56bf4ff10/pstack).
The [MIT license](PSTACK-LICENSE) retains Lauren Tan's attribution. Each general
skill links its pinned upstream source. Compare affected upstream instructions or
scripts before extending the port; retain useful behavior and explain adaptations.

| Preserved | Coda adaptation |
| --- | --- |
| App interview, launch/doctor/drive/evidence/cleanup, feature maps and dogfooding | Actual Rust harnesses, explicit visual readback, Coda naming and shared intent links. |
| Source inspection, live maintenance, triage and retained evidence | Task-scoped authority and one owner for shared harness repairs. |
| Consequential formal properties, atomicity mapping, negative controls and source revisions | Verus, design before code, explicit refinement boundaries and dependency-aware mutations. |
| Mermaid parser, hierarchy flattening, event/guard semantics and user-facing traces | Verus generation with editable induction proofs; shortest finite-chart search over the same representation. |
| Subcommand CLI with per-command help, ways-in, chart comparison and graph analysis (compared with upstream 00d5dec on 2026-10-23; through e73df47 on 2026-10-26, upstream changed no statechart or verification skill) | `statechart.mjs` with `verus`, `check`, `trace`, `rules`, `compare`, `flat`, `query`, `lint`, `against` and `coverage`; `rules` checks each rule on a sound cone of regions for charts too large to exhaust. |
| Top-level parallel regions only; upstream 00d5dec still refuses nested regions | Nested parallel regions with Harel entry, exit-wins and scope rules; templates with instances; declared events with cause, needs, families and groups; UML `/ effect` and optional `may` reactions; optional reasons on limits; compositions that join separate charts through shared events, with rules across them. All expand to the same flat model before any backend. |

Finite-chart search is not arbitrary Verus counterexample extraction, temporal model
checking or reverse translation of arbitrary code. The
[compiler reference](../.agents/skills/coda-statechart/references/compiler.md) defines
the supported boundary. No TLC or PDD preview kit is installed.
[Lauren's statechart post](https://x.com/poteto/status/2112736749281759566), the
[GitHub Next PDD announcement](https://x.com/GitHubNext/status/2112936525457740139)
and [Alex's explanation](https://x.com/alexlavaee/status/2112938102591553636) inform
this intent-to-specification-to-implementation workflow, without prescribing Coda's
product decomposition.
