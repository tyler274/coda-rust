---
name: review-pr
description: Independently assess existing PRs or coherent batches against intent, related work, and evidence. Use for PR review and correction follow-ups; resume the same reviewer as the change evolves.
---

# Review PR

Use an independent CLI reviewer for an existing PR or coherent batch. Source
access stays read-only; the reviewer owns publication and follow-up through the
GitHub App. GitHub holds the shared discussion; local run artifacts retain reviewer execution
and session identity. Follow the shared
[development workflow](../../../AGENTS.md#development-workflow).
Use [local-review](../local-review/SKILL.md) for unpublished local changes.

## Establish the targets

Start a PR review on the user's request, and continue its authorized follow-ups.
Implementation review uses `local-review`; it does not itself authorize a
publishing PR review.

Resolve the PRs, current revisions, bases, and existing review discussions. Group
changes that implement one combined outcome or depend on the same contract;
review unrelated outcomes separately. State the targets and grouping rationale.

Supply sourced context through `--context-file`: intended outcome, each PR's role,
dependencies, relevant user decisions, and prior findings and dispositions,
including local review. Distinguish assumptions from decisions. The reviewer also
retrieves requirements, source, and relevant surrounding PR discussions; the
original coding conversation is useful context, not a mandatory dependency.

## Run the reviewer

Read [CLI execution](references/cli-execution.md) and use
[run_review.py](scripts/run_review.py), passing every target with `--pr URL`.
For Coda Health, pass the shared [reviewer-app.json](reviewer-app.json) with
`--github-app-config`, using the invoking developer's own tools and Doppler access.
Related PRs remain context unless selected as targets. Defaults are Codex
`gpt-6-astra` at `xhigh`, or Claude `opus` with its default effort when selected.
Honor the user's engine and model choices.

Start a fresh reviewer conversation, separate from the coding session. Resume
that exact reviewer with `--resume-from` for corrections or clarification; use a
fresh review for a changed target set or a deliberate new opinion. The launcher supplies
[reviewer guidance](references/reviewer.md) each turn and checks target freshness.
Use background completion delivery and inspect `run.json` and the full `review.md`.
Read back verified publication links as well as the assessment. Failed, stale,
access-unverified, or publication-unverified runs do not establish completion.

## Reconcile and correct

Judge the whole report against intent and source evidence. Accept findings, reject
them with reasons, or identify an open decision. Before corrections, explain the
shared cause, responsible owner, affected paths, coherent repair, and necessary
proof. Continue accepted fixes within the implementation task; a review-only task
returns the assessment.

Verify the corrected workflow and resume the same reviewer with dispositions and
new evidence. Focus on corrections, unresolved findings, and affected contracts;
broaden when new evidence warrants it. Repeated symptoms of one cause call for
revisiting the owner. Stop when the outcome and its appropriate proof are complete,
stating unresolved concerns and any final changes left unreviewed.

## Publish and maintain the GitHub discussion

A PR review request authorizes the reviewer to publish its own assessment, inline
findings, and follow-up replies on the selected PRs through the reviewer GitHub
App, without per-comment approval. This standing exception does not cover author
responses, unrelated messages, PR-description edits, thread resolution, or merges.
Review-only requests include this publication and leave code unchanged.

The reviewer prepares and publishes its own exact assessment in the same turn
through the supplied tools. The coordinator returns the verified links and any
reasoned disagreement separately; it does not rewrite or post the reviewer's
assessment. Use the App identity only, with no personal-account fallback.

Prepare one coherent review per PR with the inspected revision, conclusion,
material findings, human decisions, and evidence limits. Use verified diff
locations for inline findings and the body for cross-cutting concerns. Default
to COMMENT; the reviewer may choose APPROVE or REQUEST_CHANGES when warranted.
A review disposition never grants merge authority.

The existing publisher binds a draft to its run and reviewed revision, checks
the target before writing, and verifies the saved identity, content, and location.
Report a retained draft or failed publication honestly. Reconcile uncertain writes
before retrying; resume the same reviewer after the cause is resolved. The
[execution reference](references/cli-execution.md) owns these mechanics and receipts.

The author or coordinating agent prepares responses in the existing threads with
correction commits and proof, reasoned disagreement, or the remaining decision.
Author responses retain their own approval requirement. Resume the reviewer to
assess corrections and publish its dispositions in that discussion. Keep new
findings distinct; resolve threads only after verification and authorization.
A clean verdict or merge alone does not settle a finding or prove deployment.

Use [write-pr](../write-pr/SKILL.md) to refresh the explanation after material
changes, and [improve-review-pr](../improve-review-pr/SKILL.md) when observed
feedback warrants examining the workflow itself.
