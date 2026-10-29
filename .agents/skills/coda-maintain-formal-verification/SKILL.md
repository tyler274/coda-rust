---
name: coda-maintain-formal-verification
description: "Reconcile drift among Coda journeys, Verus proofs and claimed Rust implementation connections. Invoke explicitly for a verification maintenance pass."
disable-model-invocation: true
---

# Maintain formal verification

Follow the [shared principles](../../../verification/README.md#principles).
Maintenance authorizes verification repairs; product fixes require implementation
scope already granted by the task. Preserve the agreed promise.

## Reconcile the changed boundary

1. Read the feature recipe, journey, charts, proofs, assumptions and code mapping.
   Run current checks once. An old pass can describe obsolete code or omit part of
   the intended journey; inspect both before judging it.
2. Compare changed states, transitions, data rules and neighboring capabilities.
   Check [correspondence](../../../verification/README.md#keep-the-diagram-and-model-in-lockstep)
   in both directions. Where code exists, diff from the recorded source revision
   and inspect every affected writer and atomic boundary.
3. Classify the difference: unaffected, translation/harness drift, missing design
   coverage, implementation regression, or proposed product change. Resolve new
   meaning through [statechart design](../coda-statechart/SKILL.md). Missing code
   does not prevent repairing or completing the design proof.
4. Repair the responsible representation. Regenerate compiled behavior from its
   chart; preserve editable proofs and run `statechart.mjs verus --check`. Reconcile extra data proofs,
   trusted boundaries and Rust refinements separately. Disclose weaker assumptions
   instead of silently relaxing the intended guarantee.
5. Run Verus and relevant chart searches. Investigate a failed obligation before
   calling it a bug. For an established violating sequence, show numbered product
   steps and replay against real code where available. An unreachable model trace
   may expose a bad abstraction; an unsatisfied proof alone is not a runtime bug.
6. Challenge the changed guarantee with a meaningful negative control at its
   claimed boundary, then restore the passing result. Check producer dependencies;
   reject syntax errors or unrelated failures as evidence. Isolate scratch caches.
7. Update the existing feature recipe, correspondence, omissions and last examined
   revision only after checking that connection. Coordinate affected app checks
   with [app maintenance](../coda-maintain-verification/SKILL.md).

## Report each check

- **in sync:** correspondence and claimed code connection remain accurate; it passes.
- **updated:** reconciled with agreed intent; it passes.
- **bug:** an established design or implementation behavior violates the promise.
- **blocked:** the check cannot run or the connection remains unresolved; explain why.

Updated and bug can coexist. Name the evidence boundary for every outcome, retain
missing implementation as pending, and report an old pass as stale when reconciling
its description exposes a failure. Do not manufacture a new PR or scheduled loop.

Adapted from [PStack maintain-formal-verification-skill](https://github.com/cursor/plugins/blob/eb4db2b55b2b7dec3d9282754cddf0a56bf4ff10/pstack/skills/maintain-formal-verification-skill/SKILL.md).
