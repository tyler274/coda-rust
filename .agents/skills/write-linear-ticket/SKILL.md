---
name: write-linear-ticket
description: Write and refine Linear tickets, GitHub issues, and project overviews. Turn ideas or observed problems into grounded outcomes, check the proposed scope, and fit the work into existing plans.
---

# Write Linear Ticket

Help clarify what work should happen, why it matters, where it belongs,
and what would establish success. Scale the investigation to the request.
Follow the shared [development workflow](../../../AGENTS.md#development-workflow).

## What a good ticket does

A ticket manages work for an engineer and their agent. Hold these ideas above
the guidance that follows. Where the two disagree, follow the idea.

- **The outcome comes first.** The first sentence says what will be true when
  the work is done, in the words of the person who asked for it.
- **It points; it does not copy.** The specification owns the rules and the code
  owns the behavior. The ticket says which decisions are already made and who
  owns them, and leaves the rest with its source.
- **It stays true while things move.** Tickets, tools and code change every day.
  Name things and their owners, not ticket numbers, functions or today's state.
- **It is salient.** Keep what changes what gets built or where an engineer
  looks. A longer ticket is not a safer one.
- **It reads as a decision.** Write a build ticket only for work whose shape is
  decided. Keep undecided ideas together in one ticket marked as ideas.
- **It leaves the engineer room.** Say what must be true and how someone will
  see it. Leave how to build it open unless that is already settled.

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

Use native issue relationships to connect tickets, and default to a related
link. Use a blocking link only when the work cannot start without the other
deliverable; an agreed interface or stub removes that need. Status, milestone
and cycle order the work, not ticket text. When reshaping tickets, check that
required outcomes still have an owner, including verification of the complete workflow.

## Write a usable handoff

After the outcome, give the scope. Keep three things apart: what is decided,
ideas that are not requirements, and what is open for the engineer.

An undecided point lives in the ticket that needs it, whether it is about the
product or about the work. When a point about the product is answered, the
answer goes into the specification.

Say where to start: the spec section that owns the rules, any decided input
the work must use, and the area of code that owns the behavior, named by crate
or feature. The reader checks the code at head.

For each handoff, state what the assignment consumes and produces, and who owns
each side. Name the thing in prose ("the Today read model") and let the issue
relation carry the ticket. Use only the structure needed. A small change may
take a few sentences.

Link to the specification instead of restating it, and do not match it line for
line. A short fixed line that says what kind of ticket this is may repeat across
a family of tickets.

Keep progress, commit snapshots, and PR or CI status out of the description.
Use issue fields, relationships, and comments for those facts.

Apply STE writing rules:

- Name the actor and use active verbs.
- Give each sentence one instruction. Keep each paragraph on one topic.
- Use the same term for the same thing. Preserve necessary technical terms.
- Use the word the team says. If the specification uses another word for the
  same thing, raise it so the specification changes.
- Aim for at most 20 words per instruction and 25 words per descriptive sentence.
- Preserve conditions, exceptions, uncertainty, and requirement strength.
  Keep a longer sentence when shortening it would change the meaning.

State acceptance as numbered criteria (AC1, AC2, ...). Each criterion names:

- The condition or action and its expected result.
- Where to observe that result.
- The inspectable evidence that would establish it.

Write each criterion as something a person wants to be able to do or see, not
as a check on mechanics. Keep the list short: cover the agreed outcome and the
failure cases that matter, and leave another ticket's behavior to that ticket.
Choose proof that matches the claim and its consequences. Do not require unrelated
checks or weaken acceptance to fit available tests.

Use the applicable [verify skill](../../../verification/README.md#verify-skills).
For behavior outside its coverage, specify the needed check and observable result.
"Tests pass" alone is not proof. A claim about a user workflow requires evidence
through that workflow.

Keep implementation, tests, deployed behavior, and domain judgment distinct.
If work is split, identify which ticket owns proof of the complete workflow.
Rules for recording evidence belong to the verify skill; do not paste them into
each ticket.

Record actual results and evidence links in the implementation PR or issue
comments, tied to the revision checked.

For exploratory work, name the question and the decision the evidence will support.

Check that someone outside the conversation can understand the purpose and scope,
find the owners, distinguish requirements from suggestions, and assess completion.

## Deliver the result

Present the framing, consequential open choices, and one whole draft in chat,
not pieces across messages. For approved tracker changes, preserve unrelated
content and metadata and read back the saved result.

Leave a ticket that is assigned or in progress as it is. Draft a comment for
its owner instead. Write a comment or message about a ticket as situation,
background, assessment and recommendation, in a few sentences.

Use [implement-ticket](../implement-ticket/SKILL.md) when the task proceeds to
implementation.
