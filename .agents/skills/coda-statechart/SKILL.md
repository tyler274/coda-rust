---
name: coda-statechart
description: "Design or revise an agreed Coda journey in Mermaid and connect its promises to Verus. Invoke explicitly for journey design or diagram/model reconciliation."
disable-model-invocation: true
---

# Design the intended journey

Follow the [shared principles](../../../verification/README.md#principles).

## Discuss the experience

1. Read Dev's decisions and the responsible product contract. Identify actors,
   goals, responsibilities and open questions. Existing code supplies evidence
   about implementation; missing code does not narrow the design.
2. Draw meaningful states, actions, external events, guards and effects in Mermaid.
   Use user-facing labels and independent regions for cooperating responsibilities.
   Include hidden distinctions only when they change a promised outcome.
3. Walk ordinary continuation, alternatives, completion and recovery from each
   state, including cross-feature interactions and reopening where relevant.
   Challenge the complete journey first; known failures add further challenges.
4. Draw what an observer sees happen; guidance and taste belong in prompts, skills
   and evals. Actors choose nondeterministically (`may`); keep hard limits few,
   each with a reason naming what it protects. Every event declares its cause; an
   agent's event is an output of the system running it. Charts compose through
   shared declared events, not copied states.
5. State each promise and its decision source beside the diagram. Revise words
   and diagrams together. Resolve new product meaning with Dev; checking a draft
   does not approve it. Save beside existing verification when edits are authorized.

## Check the agreed design

Use the [compiler](references/compiler.md) for its supported `stateDiagram-v2`
subset: nested parallel regions, templates, declared events and `/ effect` labels
expand into one flat model. Encode real finite-state exclusions with `%% never` and
state guards with `%% when`. Link identity, revision, collection and progress promises to named data
proofs; do not invent artificial states to make every chart have a safety rule.
A chart without rules establishes transition correspondence only.

Generate the model with `statechart.mjs verus`, run its Verus proof and `check`,
and use `trace` to find shortest reachable chart violations (`rules` when the chart
is too large to exhaust). Use `lint`, `query`, `flat` and `compare` to read a chart. Show an established violation as numbered
user steps and its generated path diagram. Search exhaustion is limited to the
finite chart; an interrupted or bounded search is incomplete. A failed Verus data
obligation need not have a reachable counterexample.

Use [formal creation](../coda-create-formal-verification/SKILL.md) to encode linked
data and composition promises. Keep generated behavior separate from editable
proof reasoning. Follow [correspondence](../../../verification/README.md#keep-the-diagram-and-model-in-lockstep)
in both directions; name omitted behavior even when existing proofs pass.

Build or audit the real Rust owner against that design when implementation exists.
Use [app creation](../coda-create-verification/SKILL.md) for its driving/readback
recipe. Investigate failures before claiming a bug: repair mistranslation while
preserving the promise, and return proposed behavior changes to Dev.

Adapted from [PStack statechart](https://github.com/cursor/plugins/blob/eb4db2b55b2b7dec3d9282754cddf0a56bf4ff10/pstack/skills/statechart/SKILL.md).
