---
name: implement-ticket
description: Carry an agreed ticket through implementation, verification, local review, and PR preparation. Use when implementing an issue or coherent set of related issues.
---

# Implement a Ticket

Carry the intended outcome and its evidence through the shared
[development workflow](../../../AGENTS.md#development-workflow).

## Understand and plan

Use the developer's configured Linear and GitHub connections or authenticated
tools. Resolve the ticket, workspace, repository, and checkout; follow repository
setup guidance. Name missing access and continue work supported by available
evidence rather than inventing ticket contents or a separate integration.

When the task is to implement the work, claim each Linear ticket before investing
in it, checking every ticket in a set before changing any. For a Done, Canceled,
or Duplicate ticket, change nothing: name the status and any assignee, and ask the
user before any work. Approval starts the work and leaves the ticket as it is. If
someone else holds any other ticket, change nothing and stop: name the assignee
and status, and ask plainly whether to take it from them, as in "Should we hijack
this ticket?" Approval assigns it to the developer; without approval, leave it and
do not start. If the user declines a ticket in a set, ask whether the rest should
go ahead before changing any of them. Assign an unassigned ticket to the developer,
and move a Backlog or Todo ticket to In Progress once it is the developer's; an In
Review ticket keeps its status. If Linear is unavailable or the write fails, report
the ticket as unclaimed.

Read the ticket, discussion, governing specs, dependencies, and current code.
Establish what already works, what remains, and what would prove completion.
Restate acceptance as numbered criteria (AC1, AC2, ...), drafting them when the
work has none and confirming consequential ones with the user. Choose each
criterion's evidence: the [verify skill](../../../verification/README.md#verify-skills) feature that covers the affected
code, a focused test, or the observation to capture. For a defect a test can
show, write the test first and record its failure, for the reason the defect
gives, before the fix.
Reconcile material contradictions; use
[write-linear-ticket](../write-linear-ticket/SKILL.md) when scope needs shaping.
Explain the plan, responsible owners, affected workflows, criteria, and evidence.

## Implement, verify, and review

Implement the coherent outcome at its owners. Exercise affected consumers,
side effects, and saved results through the chosen evidence. Investigate failures
before retrying. Keep controlled tests, running-product behavior, deployed
qualification, and domain judgment distinct.

Keep each criterion's evidence as artifacts a reader can inspect: the command
with its exit status and output, response bodies, logs, and screenshots you have
looked at, each recorded with the revision it ran against through
[capture-proof](../capture-proof/SKILL.md). Re-run evidence that a
later correction makes stale. A criterion without evidence stays listed as a gap.

Use [local-review](../local-review/SKILL.md) with the original intent, exact
change, criteria, and their evidence. Reconcile its findings, correct accepted defects,
and verify the result through that skill's correction workflow. Work through
that feedback locally before PR handoff, following the shared defaults for early
collaboration. Missing proof
remains visible rather than reducing the ticket's acceptance criteria.

## Prepare the handoff

Use [write-pr](../write-pr/SKILL.md) to explain the final change, linking the
ticket and relevant sources. Carry each criterion with its evidence and result,
local review dispositions, open decisions, and remaining acceptance work into the
handoff. Refresh the
explanation after material changes.

Use [review-pr](../review-pr/SKILL.md) for a requested PR review or its
follow-ups, preserving the earlier findings and evidence. Report the
actual revision, outcome, and limits, and prepare any needed ticket update.
Follow the shared defaults for publication and completion.
