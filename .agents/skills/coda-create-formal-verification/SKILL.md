---
name: coda-create-formal-verification
description: "Create Verus specifications and a repeatable Coda verification recipe from an agreed journey; connect existing Rust owners where feasible. Invoke explicitly."
disable-model-invocation: true
---

# Create formal verification

Follow the [shared principles](../../../verification/README.md#principles) and
[Verus boundary guide](references/verus.md).

## Formalize the agreement

1. Read the agreed journey, statecharts, promises and decision sources. Identify
   a consequential coherent scope, its neighboring owners and omitted behavior.
   Use [statechart design](../coda-statechart/SKILL.md) for unsettled product meaning.
2. Translate states, transitions, guards, effects and interactions. Use the
   [compiler](../coda-statechart/references/compiler.md) where supported; connect
   extra data and progress properties to named diagram promises. Review
   [correspondence](../../../verification/README.md#keep-the-diagram-and-model-in-lockstep)
   in both directions. A green selected subset does not formalize the whole journey.
3. Prove initialization and preservation across every modeled transition. Keep
   stale/invalid arrivals in the transition system so their rejection is proved.
   State abstraction, trusted inputs, bounds and any progress/fairness assumptions.
4. Run the pinned Verus command, investigate failed obligations and use chart
   reachability for concrete finite-state stories. Distinguish an established
   violation from mistranslation or insufficient inductive reasoning.

A design proof can finish before code exists. Do not create fake implementation
coverage to finish that stage.

## Connect the real Rust owner

Locate existing writers and atomic boundaries. Prefer verifying the responsible
production function directly; use the [concrete owner recipe](references/verus.md#verify-a-real-rust-owner).
Keep unsupported adapters and library behavior explicit. A copied function plus
matching tests is not implementation refinement. When direct proof is unavailable,
retain that gap and exercise the actual owner with meaningful tests.

Extend `.agents/skills/coda-verify-<app>/` or its existing feature recipe with:

- the plain-language promise and harm, linked diagram/model definitions and proofs;
- actual code owners, atomicity/refinement obligations and missing connections;
- exact commands, tool version, assumptions and last examined source revision;
- expected results, failure investigation and evidence location.

Keep product intent at its owner. Both this skill and
[app creation](../coda-create-verification/SKILL.md) extend the same recipe.

## Exercise the recipe

Run its instructions as the next agent would. Break a relevant modeled guarantee,
confirm the intended obligation fails, restore and confirm the pass. Verify the
changed producer too: an unchecked callee's postcondition can mask a mutation.
Syntax errors and unrelated failures are not successful negative controls.

Where real implementation exists, separately break its covered behavior in isolated
scratch source/build caches and confirm the implementation check rejects it.
Restore and rerun. Report design proof, implementation proof and runtime evidence
separately, including omissions and defects. A design-only run leaves app steps
unqualified. Maintain later drift with
[coda-maintain-formal-verification](../coda-maintain-formal-verification/SKILL.md).

Adapted from [PStack create-formal-verification-skill](https://github.com/cursor/plugins/blob/eb4db2b55b2b7dec3d9282754cddf0a56bf4ff10/pstack/skills/create-formal-verification-skill/SKILL.md).
