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

Read the ticket, discussion, governing specs, dependencies, and current code.
Establish what already works, what remains, and what would prove completion.
Reconcile material contradictions; use
[linear-github-issues](../linear-github-issues/SKILL.md) when scope needs shaping.
Explain the plan, responsible owners, affected workflows, and verification.

## Implement, verify, and review

Implement the coherent outcome at its owners. Exercise affected consumers,
side effects, and saved results using the repository's verification practices.
Investigate failures before retrying. Keep controlled tests, running-product
behavior, deployed qualification, and domain judgment distinct.

Use [local-review](../local-review/SKILL.md) with the original intent, exact
change, and inspected evidence. Reconcile its findings, correct accepted defects,
and verify the result through that skill's correction workflow. Work through
that feedback locally before PR handoff, following the shared defaults for early
collaboration. Missing proof
remains visible rather than reducing the ticket's acceptance criteria.

## Prepare the handoff

Use [write-pr](../write-pr/SKILL.md) to explain the final change, linking the
ticket and relevant sources. Carry local review dispositions, verification,
open decisions, and remaining acceptance work into the handoff. Refresh the
explanation after material changes.

Use [review-pr](../review-pr/SKILL.md) for a requested PR review or its
follow-ups, preserving the earlier findings and evidence. Report the
actual revision, outcome, and limits, and prepare any needed ticket update.
Follow the shared defaults for publication and completion.
