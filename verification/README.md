# Coda product verification

> **Journey models are built one at a time from the [intent specs](../intent/README.md)**, under
> `physician/`, `duet/` and `connected-care/`, starting with
> [physician sign-in](physician/sign-in/README.md). The specs govern every journey not yet modeled.

Coda ports [PStack](#provenance) to Verus and connects agreed product journeys to
Rust owners and running-app evidence. The five general skills establish or repair
verification workflows and are explicit-only. App-specific recipes own their
concrete journeys, models and harnesses; ordinary feature work does not require
the full workflow.

## Principles

1. **Start from the intent specs.** They hold the product decisions. Once a
   journey's model is agreed, its folder is that journey's spec and the product spec links to it.
   One model answers one journey question. Its rules are the agreed sentences, about what the
   reader sees. Design the complete journey before choosing proof cases. The model is the
   design: it also holds what the rules depend on, such as a stored value, a request in flight
   or another tab, and lets what we don't control happen in any order. Code carries the model
   out and is judged against it, never the reverse. The agent harness stays out of it. Existing code and historical failures can challenge the model;
   missing code cannot limit it. A checked draft is still a draft.
2. **Keep one owner.** Journey, model and formal specification express one behavior.
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

## Keep the model, the proof and the code in step

Map meaningful states to predicates, edges to guarded transitions and effects, and
cross-capability promises to composition obligations. Check both directions:
intended behavior must not disappear from the model; modeled behavior must have a
design meaning. Use the existing feature recipe and source links for correspondence,
including omissions. Check both valid publication and complete discovery: use full
collections, preserve every accepted batch member, derive history from its owner,
and separate safety from conditional progress. A capability overview plus selected
proofs is not full coverage.

Each journey's model is plain data built with [model.mjs](model.mjs): regions that each hold one
state, and events whose ordered cases read like the code's own steps, guarded with `is`, `not`,
`all` and `any`. Whatever one actor can do (a tab, a device) is generated from one design, so the
search tries every order. `never` holds the agreed rules, `invariant` the facts that explain why
they hold, `reach` the outcomes the spec walks through, and `controls` one plausible bug per rule
sentence. [statechart.mjs](statechart.mjs) `check` reaches every state, prints the shortest path to
any broken rule or false fact, finds a path to each outcome, and makes each control's change to
show its rule can break; a completed search covers only the finite model. `verus` writes the same
cases as a Verus model; the proof shows the facts and rules hold at the start and survive every
case of every event, so they hold in every state, whatever the search covered. `walks` covers every
case of every event in a few short walks, which the journey's browser test plays in the product;
`walks --paths` gives the walk through each bug path, which the journey's bug-path test replays.

Each event's `code` note says where the code carries that step out. The code follows the model:
where it differs, write a test that fails on the code, then change the code. A difference the
model can't settle is a design question for the person you are working with. When the code a
model's notes point to changes, re-read it against the model and run the journey's browser walks.

## Connect the harness through events

The agent harness stays out of the model and meets it only at declared events. An agent's
event is the harness's output, and each tool it admits is one such event: the product host checks that event's `needs` before the effect
commits, and the model decides when to use it. Retries, timeouts, cadence, memory
and prompts stay in the harness, whose guarantees the
[harness spec](../intent/harness.md) states. The runtime records model events where
each fact's owner changes it, so a recorded run can be replayed on the model. The
harness has no model of its own; it is just the harness. An app's own session and request
handling is not the harness: when a rule depends on it, it belongs in the model.

Do not force every promise into finite states. A promise that rests on records the
model can't hold (identity, revisions, collections, atomic batches) is listed in the
journey's README under its rules, naming the test or proof that holds it, or saying
none does yet. Progress needs explicit assumptions.
Models without safety rules establish transition correspondence only. Keep generated
behavior separate from editable proofs; `statechart.mjs verus --check` detects generated drift. Review
manual data refinements and implementation connections separately.

## Feature specs

| Folder | Spec |
| --- | --- |
| [physician/sign-in](physician/sign-in/README.md) | How a physician signs in: the spec in words, its rules, model and Verus proof. |

## How the skills fit

| Skill | Use |
| --- | --- |
| [coda-statechart](../.agents/skills/coda-statechart/SKILL.md) | Design one journey as a model from the intent specs; check it, prove it with Verus, and hold the code to it through its browser walks. |
| [coda-create-formal-verification](../.agents/skills/coda-create-formal-verification/SKILL.md) | Encode design in Verus and establish real Rust proof connections. |
| [coda-create-verification](../.agents/skills/coda-create-verification/SKILL.md) | Create launch, doctor, drive, evidence and cleanup instructions. |
| [coda-maintain-formal-verification](../.agents/skills/coda-maintain-formal-verification/SKILL.md) | Reconcile model, proof, record promises and code drift. |
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
node verification/statechart.mjs check path/to/the/journey.mjs
node verification/statechart.mjs verus path/to/the/journey.mjs --module NAME --check
node verification/statechart.mjs walks path/to/the/journey.mjs [--only REGEX | --paths JSON]
```

There is no default proof for an unspecified feature. The extra toolchain does not
change the workspace default. See [verus.json](verus.json) and the
[real-owner recipe](../.agents/skills/coda-create-formal-verification/references/verus.md#verify-a-real-rust-owner).
App recipes supply their own doctor, runtime, mutation and browser commands.

Checks print a new evidence directory under `target/verification/runs`; `--out`
selects another new directory. Logs survive scratch cleanup.

## Provenance

The baseline is [PStack at eb4db2b](https://github.com/cursor/plugins/tree/eb4db2b55b2b7dec3d9282754cddf0a56bf4ff10/pstack),
with the stable one-model [statechart skill at 519d36a](https://github.com/cursor/plugins/blob/519d36aa98955bf680db01bcde5fdfba686dc901/pstack/skills/statechart/SKILL.md).
The [MIT license](PSTACK-LICENSE) retains Lauren Tan's attribution. Each general
skill links its pinned upstream source. Compare affected upstream instructions or
scripts before extending the port; retain useful behavior and explain adaptations.

| Preserved | Coda adaptation |
| --- | --- |
| App interview, launch/doctor/drive/evidence/cleanup, feature maps and dogfooding | Actual Rust harnesses, explicit visual readback, Coda naming and shared intent links. |
| Source inspection, live maintenance, triage and retained evidence | Task-scoped authority and one owner for shared harness repairs. |
| Consequential formal properties, atomicity mapping, negative controls and source revisions | Verus, design before code, explicit refinement boundaries and dependency-aware mutations. |
| One model checked against the reader's rules, a chart drawn from it in the reader's words, and each bug path replayed against code | Plain per-event cases and their own explicit-state search instead of TLA+ and TLC; Verus proves the model's facts inductive and safe; covering walks, played in the browser, hold the code to the model, and each bug path stays in a browser test that failed on the code. |
| A check never seen to fail proves nothing; never weaken a property | Each rule sentence has a control, a plausible bug `check` must see break it; a rule the model can't say gets a region that only records what it names. |
| Per-journey models labeled as the screen, one never-rule per agreed sentence | Models are reviewed as their cases and rules; every model also gets a Verus proof. |
| Top-level parallel regions only | Top-level parallel regions with flat states, as Verus can state them; events declared with cause, intent and code. |

Finite-model search is not arbitrary Verus counterexample extraction, temporal model
checking or reverse translation of arbitrary code. [statechart.mjs](statechart.mjs)
refuses models outside the supported boundary. No TLC or PDD preview kit is installed.
Lauren's statechart posts ([first](https://x.com/poteto/status/2112736749281759566),
[stable](https://x.com/poteto/status/2115880102601101414)), the
[GitHub Next PDD announcement](https://x.com/GitHubNext/status/2112936525457740139)
and [Alex's explanation](https://x.com/alexlavaee/status/2112938102591553636) inform
this intent-to-specification-to-implementation workflow, without prescribing Coda's
product decomposition.
