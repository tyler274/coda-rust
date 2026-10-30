---
name: write-linear-ticket
description: Write and refine Linear tickets, GitHub issues, and project overviews. Turn ideas or observed problems into grounded outcomes, check the proposed scope, and fit the work into existing plans.
---

# Write Linear Ticket

Help clarify what work should happen, why it matters, where it belongs,
and what would establish success. Scale the investigation to the request.
Follow the shared [development workflow](../../../AGENTS.md#development-workflow).

## Understand and check the framing

Read the user's discussion, relevant project guidance, and existing work.
Inspect implementation or runtime evidence when the scope depends on
what currently works or fails. Distinguish decisions, observations,
proposals, and open questions; follow the established source owners.

Separate the desired outcome from the proposed solution. Check consequential
assumptions: does this address the underlying problem, is it already covered,
and would changing or simplifying an existing path serve the goal better?
Challenge with evidence; do not manufacture objections or reopen settled
choices without reason.

Resolve routine details yourself. Bring unresolved product, architecture,
scope, or priority choices back to the user with a recommendation. Keep
supported work moving without presenting assumptions as decisions.

## Shape the work

Find the existing outcome and responsible owners before proposing another
ticket. Recommend refining, adding, splitting, investigating, or leaving
the tracker unchanged according to the actual gap.

Preserve the complete intended outcome. Split work when ownership,
dependencies, or independently useful results justify it. Distinguish
what blocks starting from what blocks acceptance. Leave implementation
choices open unless already settled.

## Write a usable handoff

Explain the problem, intended result, material constraints, relevant
sources, open decisions, and meaningful completion. Use only the structure
needed; a small change may take a few sentences.

Define observable success and appropriate evidence across the affected
workflow. Keep implementation, tests, deployed behavior, and domain
judgment distinct. Exploratory work should name the question and the
decision its evidence will support.

Check that someone outside the conversation can understand the purpose,
find the owners, distinguish requirements from suggestions, and assess
completion. Follow project conventions without duplicating specifications
or progress tracking.

## Deliver the result

Present the framing, consequential open choices, and complete draft in chat.
For approved tracker changes, preserve unrelated content and metadata and
read back the saved result. Use [implement-ticket](../implement-ticket/SKILL.md)
when the task proceeds to implementation.
