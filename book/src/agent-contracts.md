# Agent contracts, hoisted

The four product repositories keep their full contracts; this page hoists their
operative rules into this workspace so an agent working on a Rust crate can
follow them without opening a submodule. Submodules are read-only reference
(see the <a href="/.cursor/rules/rust-repo-first%2Emdc">rust-repo-first</a> rule). When a
product-semantic rule here and a submodule source disagree, the submodule source
wins and this page is a bug. Rust crate sharing follows the explicitly
user-approved boundary in <a href="/AGENTS%2Emd">AGENTS.md</a>; reference-repository
ownership does not override that approval.

## Coda Agents (physician product, harness)

Source: <a href="/submodules/coda-agents/AGENTS%2Emd">coda-agents AGENTS.md</a>.

- **Authority order.** Dev's latest explicit direction, then canonical docs,
  then source. `USER_SETTLED`, `PROJECT_SETTLED`, `CODEX_ASSUMPTION`, and
  `OPEN` record decision provenance; new evidence reopens them. `eval/` is
  sealed — never read it or place it in model-visible context.
- **Authority limits.** A new product layer, lifecycle, schema family, or
  authority rule is `OPEN` until Dev approves. Changes to externally
  observable protocol semantics require explicit authorization.
- **Invariants before mechanisms.** Repair the owner that already records the
  truth; never add a parallel receipt, store, parser, or policy engine.
  Standards-defined wire decoding is mechanical. Code never guesses, grades,
  repairs, ranks, or suppresses clinical cognition. Model-visible evidence
  uses source-owned immutable identities, never prose or line ranges.
- **One owner per fact.** Every mechanism is justified by an observed failure
  or a necessary guarantee; subtraction is a fix. Before adding a fallback or
  flag, walk the whole workflow and find the shared cause.
- **Model capability.** Models select evidence, interpret records, and compose
  responses. Deterministic code enforces access, isolation, identity, exact
  evidence, and authority for external actions. When a model underperforms,
  look for a tool that starves it before adding a rule. Every proposed
  guardrail names a concrete failure, its consequence, and why code must own it.
- **Failure and recovery.** A failure affects the smallest necessary unit.
  Preserve successful work, make missing information explicit, retry with
  bounded backoff, and never let a retry limit mean forgetting the work or a
  skip mean declaring it processed. Recovery rides existing owners — no
  recovery framework, no catch blocks that hide errors.
- **Introducing a capability.** Ground in main as it runs, map what it rides,
  write journeys as deltas, budget words and states, uplevel the owners it
  rides, align the layers, then order, prove, fold.
- **Build for every practice.** What differs by practice is configuration or a
  declared adapter, never a constant or a branch named for one practice.
- **Search and indexing.** Originals are the truth; indexes are rebuildable
  projections. A ranking score never establishes sufficiency; citations
  require an exact read. Foreground work never queues behind background work.

## Duet (patient app)

Source: <a href="/submodules/coda-app/AGENTS%2Emd">coda-app AGENTS.md</a>;
North Star owns product, privacy, consent, and architecture.

- **Boundary.** Preserve patient-owned evidence, source provenance, consent,
  and the separate physician-side boundary. Do not infer new clinical behavior
  or release authority from a successful test.
- **Intelligence boundary.** Models own evidence selection, significance, and
  composition. Before adding a filter or threshold, name the mechanical
  invariant it protects; code must not decide that valid evidence is too
  sparse, noisy, or uninteresting to reach the model. Prefer deleting a
  judgment rule to building another mechanism.
- **Failure handling.** Same doctrine as Coda Agents above: isolate failures
  to the smallest unit, reconcile uncertain writes before retrying, and
  review every failure path with the four recovery questions.

## Coda Gym (evaluation)

Source: <a href="/submodules/coda-gym/AGENTS%2Emd">coda-gym AGENTS.md</a>.

- **Clinical authority.** Physician-authored source is clinical authority;
  promotion translates it, code never infers clinical meaning or rubric
  structure from prose.
- **One run contract.** The task package is canonical. No second package
  format, run identity, answer contract, or score authority. Code owns shape,
  identity, provenance, and fail-closed infrastructure; LLM judges own
  clinical and semantic equivalence. Hidden chain-of-thought is never score
  authority; clinical penalties carry the burden of proof and uncertainty is
  non-penalizing.
- **Gates.** Publication, upload, PR creation, and benchmark claims require
  explicit authorization after proof and review. Architecture-sensitive fixes
  name the duplicate owner they remove or explain why none exists.

## Computer use

Source: <a href="/submodules/Coda-Computer-Use/AGENTS%2Emd">Coda-Computer-Use AGENTS.md</a>.

- **Division of work.** Models choose actions; code preserves authority and
  execution integrity. Never execute raw model-generated code, prescribe
  patient routes, or filter candidates using private evaluator truth.
- **Writes are physician-approved write-backs.** Do exactly that change, write
  nothing else, and return proof of what was sent and what the chart now
  shows. The model picks every target; mismatches return to the model as
  feedback. Action failures are model-visible feedback — no automatic
  fallback or repeated input after uncertain execution.
- **Synthetic only.** Local EHRGym and Docker OpenEMR on loopback with
  synthetic patients; every other origin stays blocked. Private graders and
  answer keys never enter model context.

## How this maps to the Rust workspace

When porting a submodule behavior into a crate:

- The Coda Agents invariants and failure doctrine govern the physician owners:
  `patient-work`, `local-product`, `physician-auth`, and `physician-app`.
- The Duet intelligence boundary governs `duet-api` and `duet-app`.
- `harness` and its neutral supporting libraries, including the `agent-skills`
  loader, share execution mechanics under the approved
  [ENG-159 boundary](https://linear.app/coda-health/issue/ENG-159#comment-0bb61a83-8e56-40d9-aa2c-f8b9960393eb).
  Common execution-integrity and failure-isolation invariants apply to both
  consumers; physician clinical policy does not become Duet policy. Each product
  supplies its own skills, context, admitted tools and authority checks.
- Products retain accounts/sessions, clinical types, durable state and scheduling,
  effects, commit/publication and recovery. The harness emits neutral events to
  each product's existing durable owner and awaits required acknowledgments;
  it owns neither a shared clinical journal nor a product terminal-state machine.
- Preserve the distinct original semantics: physician revision forbids memory
  writes/delegation, and interrupted work continues through successor attempts;
  Duet may save routines before the reply and retries its accepted run. Product
  context-preservation policies, Stop/steering/consent fences and completion
  transactions remain distinct. P1/D1 must prove restart, idempotency and no
  duplicate effects through their real owners before qualification is claimed.
- Gym and computer-use crates (future) keep their clinical types separate from
  the physician and Duet crates, per the top-level boundary.
- One owner per fact maps directly to crate/module ownership: the crate that
  owns a type is the only writer of that type's truth.
