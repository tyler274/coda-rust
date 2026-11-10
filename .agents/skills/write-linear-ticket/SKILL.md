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

## Shape the work

Find the existing outcome and responsible owners before proposing another
ticket. Recommend refining, adding, splitting, investigating, or leaving
the tracker unchanged according to the actual gap.

Preserve the complete intended outcome. Split work when ownership,
dependencies, or independently useful results justify it. Distinguish
what blocks starting from what blocks acceptance. Leave implementation
choices open unless already settled.

Use native issue relationships for organization and dependencies. Allow
independent progress where practical. When reshaping tickets, check that
required outcomes still have an owner, including verification of the complete workflow.

Put a blocking link on the issue whose acceptance requires the unfinished
deliverable. Name the deliverable and the acceptance criterion it blocks. If an
agreed interface or stub allows independent work, document that handoff and tie
any remaining dependency to the issue that owns the integration check.

## Write a usable handoff

Explain the problem, intended result, and scope. For each handoff, state what
the assignment consumes and produces, and who owns each side.
Include constraints, open decisions, and sources only where they affect the work.
Use only the structure needed. A small change may take a few sentences.

Remove repeated explanations and information that does not help someone
implement or accept the work. Link to the owning specification instead of
restating it.

Keep progress, commit snapshots, and PR or CI status out of the description.
Use issue fields, relationships, and comments for those facts.

Apply STE writing rules:

- Name the actor and use active verbs.
- Give each sentence one instruction. Keep each paragraph on one topic.
- Use the same term for the same thing. Preserve necessary technical terms.
- Aim for at most 20 words per instruction and 25 words per descriptive sentence.
- Preserve conditions, exceptions, uncertainty, and requirement strength.
  Keep a longer sentence when shortening it would change the meaning.

State acceptance as numbered criteria (AC1, AC2, ...). Each criterion names:

- The condition or action and its expected result.
- Where to observe that result.
- The inspectable evidence that would establish it.

Cover the agreed outcome, its necessary handoffs, and material failure cases.
Choose proof that matches the claim and its consequences. Do not require unrelated
checks or weaken acceptance to fit available tests.

Use the applicable [verify skill](../../../verification/README.md#verify-skills).
For behavior outside its coverage, specify the needed check and observable result.
"Tests pass" alone is not proof. A claim about a user workflow requires evidence
through that workflow.

Keep implementation, tests, deployed behavior, and domain judgment distinct.
State what the proposed evidence cannot establish. If work is split, identify
which ticket owns proof of the complete workflow.

Record actual results and evidence links in the implementation PR or issue
comments, tied to the revision checked.

For exploratory work, name the question and the decision the evidence will support.

Check that someone outside the conversation can understand the purpose and scope,
find the owners, distinguish requirements from suggestions, and assess completion.

## Deliver the result

Present the framing, consequential open choices, and complete draft in chat.
For approved tracker changes, preserve unrelated content and metadata and
read back the saved result. Use [implement-ticket](../implement-ticket/SKILL.md)
when the task proceeds to implementation.
