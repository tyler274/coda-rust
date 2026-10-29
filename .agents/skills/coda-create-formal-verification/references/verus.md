# Verus proof boundaries

Follow the [shared principles](../../../../verification/README.md#principles).
The journey supplies the promise. Verus proves the stated specification under its
assumptions; it does not approve product meaning or establish deployed behavior.

Use a state machine when event ordering matters. Prove initialization, preservation
and composition, keeping stale arrivals available so rejection is checked. Add
progress reasoning only for a promised outcome and state fairness/availability
assumptions. Mathematical integers do not establish machine-integer behavior.

Prefer proof of actual Rust logic to a copied implementation. Name trusted adapters,
unsupported libraries, persistence and transport boundaries. Tests of those owners
are valuable evidence, but are not mathematical refinement. Missing implementation
stays pending without limiting design scope.

## Verify a real Rust owner

Locate the actual pure logic and trusted boundary first. Verify its production
body directly where supported. If factoring is needed, move the existing predicate
without changing behavior or keeping two implementations. Connect its contract to
the intended model transition and inspect callers and atomic boundaries.

A small app-owned adapter may read the exact production function on each run and
insert ghost contracts without copying its executable body. Record the source hash,
use the pinned verifier, and refuse a changed signature until the adapter is reviewed.
Include both rejection of invalid inputs and acceptance of valid inputs, so an
always-refuse implementation cannot satisfy the contract. Use the production
integer types and name any unsupported library or environment assumptions.

Challenge the real source with meaningful scratch mutations, require the intended
proof failures, and recheck the original. Keep each app's adapter and commands in
its verification recipe; do not turn a small adapter into a source-rewriting framework.
Run the existing caller and integration checks as separate evidence. A pure-function
proof does not establish correct identity lookup, field mapping, persistence,
transaction boundaries or publication, nor complete refinement of the design model.

## Diagnose a failed check

Distinguish an unsatisfied obligation, a reachable model violation and a runtime
bug. `node verification/statechart.mjs check` prints the shortest path to a broken
chart rule, so it can explain finite-state violations but cannot extract arbitrary Verus traces.
Strengthen induction when an unreachable state breaks preservation; never weaken
agreed safety merely to obtain a pass.

For negative controls, verify the changed producer and its proof dependencies.
Verus may otherwise trust an unchecked callee's postcondition. Require the intended
proof failure, not a syntax error or unrelated obligation. Keep model, real-code
and runtime mutations distinct, then restore the original result.
