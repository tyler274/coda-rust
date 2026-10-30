---
name: coda-create-formal-verification
description: "Create Verus specifications and a repeatable Coda verification recipe from an agreed journey; connect existing Rust owners where feasible. Invoke explicitly."
disable-model-invocation: true
---

# Create formal verification

Follow the [shared principles](../../../verification/README.md#principles) and
[Verus boundary guide](references/verus.md).

## Formalize the agreement

1. Read the agreed journey, its model, promises and decision sources. Choose the
   promise where a break harms the person most and tests rarely reach the case that
   breaks it, not the one easiest to prove; name its neighboring owners and omitted
   behavior. Use [the journey model](../coda-statechart/SKILL.md) for unsettled
   product meaning.
2. Let the question choose the check. Whether steps can happen in a bad order is a
   question for the journey's model: put the code's own steps in it, as
   [coda-statechart](../coda-statechart/SKILL.md) step 3 says, search it, and prove
   it with `node verification/statechart.mjs verus`. Whether code behaves for every
   input is a question for Verus on the real Rust owner, below. Connect extra data
   and progress properties to named promises. Review
   [correspondence](../../../verification/README.md#keep-the-model-the-proof-and-the-code-in-step)
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

Run its instructions as the next agent would. Model the code as it was before a
real fix, or break a step the way a plausible change would, confirm the intended
rule and obligation fail at the step you expect, restore and confirm the pass. Verify the
changed producer too: an unchecked callee's postcondition can mask a mutation.
Syntax errors and unrelated failures are not successful negative controls.

Where real implementation exists, separately break its covered behavior in isolated
scratch source/build caches and confirm the implementation check rejects it.
Restore and rerun. Report design proof, implementation proof and runtime evidence
separately, including omissions and defects. A design-only run leaves app steps
unqualified. Maintain later drift with
[coda-maintain-formal-verification](../coda-maintain-formal-verification/SKILL.md).

Adapted from [PStack create-formal-verification-skill](https://github.com/cursor/plugins/blob/519d36aa98955bf680db01bcde5fdfba686dc901/pstack/skills/create-formal-verification-skill/SKILL.md).
