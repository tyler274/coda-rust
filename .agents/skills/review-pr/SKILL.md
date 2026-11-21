---
name: review-pr
description: Independently assess existing PRs or coherent batches against intent, related work, and evidence. Use for PR review and correction follow-ups; resume the same reviewer as the change evolves.
---

# Review PR

Use an independent CLI reviewer for an existing PR or coherent batch. Source
access stays read-only. The reviewer returns its assessment and a saved draft to
the operator; the swarm's reviewers own the review of record on GitHub, and
posting the draft there is the user's decision. Local run artifacts retain
reviewer execution and session identity. Follow the shared
[development workflow](../../../AGENTS.md#development-workflow).
Use [local-review](../local-review/SKILL.md) for unpublished local changes.

## Establish the targets

Start a PR review on the user's request, and continue its follow-ups.
Implementation review uses `local-review`.

Resolve the PRs, current revisions, bases, and existing review discussions. Group
changes that implement one combined outcome or depend on the same contract;
review unrelated outcomes separately. State the targets and grouping rationale.

Supply sourced context through `--context-file`: intended outcome, each PR's role,
dependencies, relevant user decisions, acceptance criteria with their evidence,
and prior findings and dispositions, including local review. Distinguish assumptions from decisions. The reviewer also
retrieves requirements, source, and relevant surrounding PR discussions; the
original coding conversation is useful context, not a mandatory dependency.

## Run the reviewer

Read [CLI execution](references/cli-execution.md) and use
[run_review.py](scripts/run_review.py), passing every target with `--pr URL`.
This repository carries no reviewer App configuration: run without
`--github-app-config`, using the invoking developer's own tools.
Related PRs remain context unless selected as targets. Defaults are Codex
`gpt-6-astra` at `xhigh`, or Claude `opus` with its default effort when selected.
Honor the user's engine and model choices.

Start a fresh reviewer conversation, separate from the coding session. Resume
that exact reviewer with `--resume-from` for corrections or clarification; use a
fresh review for a changed target set or a deliberate new opinion. The launcher supplies the
[review standard](../local-review/references/standard.md) and the
[PR reviewer guidance](references/reviewer.md) each turn and checks target freshness.
Use background completion delivery and inspect `run.json`, the full `review.md`
and the retained draft. Failed, stale or access-unverified runs do not establish
completion; `publication_unverified` is the expected finish in this repository.

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

## Return the assessment

The reviewer prepares one coherent review per PR with the inspected revision,
conclusion, material findings, human decisions, and evidence limits, and saves
it as a draft bound to its run and the reviewed revision. Default to COMMENT; the
reviewer may choose APPROVE or REQUEST_CHANGES when warranted. A review
disposition never grants merge authority. Review-only requests leave code
unchanged.

The coordinator returns the assessment, the draft's location and any reasoned
disagreement in chat; it does not rewrite the reviewer's assessment. The swarm's
reviewers own the review of record on GitHub. Posting the draft on the PR is an
external publication that needs the user's approval; when approved, post it as
the `review-pr` reviewer's assessment at the inspected revision. The
[execution reference](references/cli-execution.md) owns the draft mechanics.

The author or coordinating agent prepares responses to findings with correction
commits and proof, reasoned disagreement, or the remaining decision. Resume the
reviewer to assess corrections. Keep new findings distinct. A clean verdict or
merge alone does not settle a finding or prove deployment.

Use [write-pr](../write-pr/SKILL.md) to refresh the explanation after material
changes, and [improve-review-pr](../improve-review-pr/SKILL.md) when observed
feedback warrants examining the workflow itself.
