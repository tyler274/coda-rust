---
name: local-review
description: Independently review local code, artifacts, or combined changes before handoff. Use when the repository's development workflow calls for independent review or the user requests it.
---

# Local Review

Use this skill when the shared
[development workflow](../../../AGENTS.md#development-workflow) calls for independent
review, or when the user requests it. Assess the complete local result through a
fresh reviewer, separate from the coding conversation. The parent owns intent,
corrections, and final judgment.

## Prepare the review

Read the relevant conversation, governing guidance, and current owners. Supply:

- The intended outcome, scope, completion bar, and relevant original user
  decisions and corrections, with accessible sources or bounded excerpts.
  Distinguish settled decisions, assumptions, and open questions.
- Exact target paths, HEADs, comparison bases, complete staged and unstaged
  changes, and relevant untracked artifacts. Freeze or fingerprint the target
  so concurrent edits cannot silently change what was reviewed.
- Affected workflows and consumers, dependencies between pieces, prior findings
  and dispositions, and inspected verification with its limits: each acceptance
  criterion with its evidence artifacts and the revision they ran against.

Give enough context for independent judgment without prescribing a verdict or
limiting investigation to known concerns. Keep unrelated private material and
credentials out of the handoff. Retrieve missing intent or name its effect on
confidence; ordinary correctness defects can still be established.

## Run the reviewer

From Codex, use [Claude reviewer execution](references/claude-reviewer.md).
From Claude Code, use [Codex reviewer execution](references/codex-reviewer.md).
From another editor, terminal, or agent host, use either available CLI route.
Both use the invoking developer's installation and authentication, with paths
resolved from their checkout; no personal installation or connector is required.
Honor an explicitly selected engine or model. These references own commands,
background completion, result validation, and exact-session continuation.

Supply the review standard and the context on every turn. Start fresh for an
initial review and resume that reviewer for corrections. Use
[review-pr](../review-pr/SKILL.md) for a requested PR review; local work does not need to
be published to become reviewable.

## Reviewer's assignment

The reviewer judges the change by the
[review standard](references/standard.md), the same one a PR reviewer follows.
Include that file's text, unchanged, at the start of every turn's request, then
the context, and end with what to review; don't summarize it. The reviewer stays
read-only and returns its findings to the parent.

## Reconcile and finish

Read the whole report and confirm target freshness. Accept findings, reject them
with evidence, or bring an unresolved decision to the user. Before corrections,
explain the shared cause, responsible owner, affected paths, coherent repair,
and necessary proof. Preserve successful behavior and unrelated work.

Continue accepted corrections within the implementation task. Verify them and
resume the same reviewer with dispositions and new evidence. Focus follow-ups on
the correction, unresolved findings, and affected contracts in the cumulative
result; broaden when new evidence warrants it. Further rounds need a concrete
change or unresolved concern, not a desire to obtain a clean verdict.

Report the final target, assessment, evidence, remaining concerns, and changes
left unreviewed. Carry this context into [write-pr](../write-pr/SKILL.md) and any
subsequent PR review so the next reader can understand the outcome.
