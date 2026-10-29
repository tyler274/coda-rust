---
name: review-pr
description: Review existing GitHub PRs or coherent batches through an independent Codex or Claude CLI reviewer. Start fresh and resume that exact reviewer for follow-ups. Assess intent, related open work, the combined solution, and shared causes independently of the coding agent.
---

# Review PR

The coordinating chat owns scope, interpretation, and authorized corrections.
An independent CLI reviewer keeps source access read-only and owns its review,
publication, and follow-up under the reviewer GitHub App identity. General
local-work validation remains separate. GitHub holds the shared review discussion;
local run artifacts retain execution details and the reviewer session needed for
continuation.

## Establish scope

A request to review a PR invokes this workflow and authorizes its CLI reviewer.
Use `$review-pr` in Codex or `/review-pr` in Claude Code; natural-language PR
review requests also apply. Discussing or editing the skill does not launch it.

**Review-only requests stop at the assessment.** Fixes require an explicit,
still-active implementation instruction. Earlier coding work or a review's
completion notification does not override a later instruction or promise to
review first or wait for the user.

Resolve the requested PRs. Group changes that implement one combined outcome or
depend on the same contract into one review; review independent changes
separately, sequentially unless parallel runs were requested. State the selected
targets and why they belong together. Use the current branch to find a PR only
when the user refers to its PR.

Read existing GitHub reviews, inline threads, author replies, and correction commits
before preparing the review. Identify which revisions each finding concerns.
Supply known context through `--context-file`: intended outcome, each PR's role,
evidenced dependencies or landing order, and relevant user decisions and prior
findings with sources. Label assumptions and keep the handoff neutral. The
original coding agent and conversation are optional; the reviewer independently
retrieves requirements, source, and relevant surrounding PR discussions.

## Launch the independent reviewer

Read [CLI execution](references/cli-execution.md), then use
[scripts/run_review.py](scripts/run_review.py). Supply every review target with
`--pr URL`; repeat the flag for a batch. PRs mentioned only as context are not
included in the launcher's revision checks. State the engine, model, and targets:

- Default: Codex, `gpt-6-astra`, effort `xhigh`.
- Claude: `--engine claude`, model `opus`, with Claude's default effort.
- Honor explicit model and effort choices; do not silently substitute an engine.

The first review starts a fresh, non-interactive CLI conversation, separate from
the coding session. For follow-ups, use `--resume-from /path/to/prior/run.json`
to resume that exact reviewer with the same target set. Use a new conversation
for an explicitly requested fresh opinion or changed target set. Do not use the
coding session, a native subagent, `--last`, or `--continue` as a substitute.

The launcher supplies [reviewer guidance](references/reviewer.md) on every turn.
Use the host's background execution and completion delivery described in the
execution reference.
Investigate launch failures; do not bypass restrictions or silently restart fresh.

## Reconcile before correcting

Read `run.json` and the complete `review.md`. A completed process is not a
favorable verdict; failed, stale, access-unverified, or freshness-unverified results
cannot establish current readiness. Present the useful result and exact reviewer
publication draft in chat.
Follow the publication procedure below; a local assessment is not a posted review.

Judge the whole report against intent and source evidence. Accept findings or
reject them with reasons, and identify unresolved decisions and evidence gaps.
Preserve material structural concerns as well as local defects.

Before authorized edits, briefly explain the intended behavior, shared causes,
responsible owner, affected paths, coherent correction, and necessary proof.
Trace relevant writers, readers, and lifecycle transitions so the correction
covers the contract beyond the reported example. Remove obsolete patches where
appropriate. Independent defects can stay local; preserve successful behavior
and unrelated work. Check the governing requirements and decisions: within
authorized scope, restore agreed behavior and resolve routine technical choices
yourself. If those sources leave a material behavior or architecture choice open,
ask the user with options and a recommendation before implementing it; continue
independent fixes. A review finding alone does not settle that choice.
This reconciliation is a few sentences in the existing chat, not a new report
or approval ceremony.

Implement that correction and verify the affected workflow. If a later finding
exposes the same contract through another path, revisit the owner and coverage
before adding a patch. Back-and-forth may clarify or challenge a finding without
requiring a code change.

Resume the reviewer for a concrete clarification, changed code, or new evidence,
carrying sourced dispositions and verification from the GitHub discussion, plus any
relevant unpublished evidence clearly labeled as such. Focus follow-ups on the correction,
unresolved findings, and affected contracts and consumers in the cumulative change.
Broaden investigation when new evidence warrants it; do not repeat unchanged broad
reviews to obtain a clean verdict. Finish when the requested outcome and appropriate
proof are complete; state unresolved concerns and final changes left unreviewed.

## Publish the review and follow-up on GitHub

The independent reviewer owns its GitHub review and follow-up. It prepares the exact
publication through its scoped local tool during the assessment turn. The coordinating
chat presents that draft without rewriting it and carries the user's approval back to
the same reviewer session. The reviewer then invokes its publisher, authenticated only
as the repository-scoped reviewer GitHub App. The coordinator never submits the
reviewer's assessment through its own GitHub identity or tool.

Prepare one coherent review per PR with the inspected commit, conclusion, material
findings, concrete human decisions, and evidence limits. For a batch, place findings
on their owning PR and link related findings across PRs. Present coordinator
disagreements separately, with evidence, rather than changing the reviewer draft.

Use inline threads for findings with verified diff locations. Keep cross-cutting
findings in the review body rather than attaching them to arbitrary lines. A clean
assessment still states its scope and limits. Default to a COMMENT review; approval
or request-changes status must be explicitly included in the approved publication.
If the reviewer App cannot submit the selected review, explain the limitation and keep
the draft unpublished; do not switch identities or fall back to a personal account.

Before posting, obtain the user's approval of the exact reviewer-authored content,
destination, and review event. Resume the run with `--approve-publication` naming that
saved draft. The launcher binds the approval to its digest, target, reviewed revision,
and prior reviewer session. The publishing tool re-reads the PR before writing and
reads back the saved author and body afterward. If the PR advanced, resume assessment
and obtain approval for a new draft. An ambiguous write requires GitHub reconciliation
before retrying. Pending approval or a failed write leaves the draft unpublished.

For authorized corrections, the author or coordinating chat prepares replies to each
material finding in its existing thread: corrected with commit and evidence, disagreed
with reasons, or awaiting a specific decision and owner. Those replies remain under the
author's identity and require their own exact-content approval. Group shared causes
before editing; replies must not turn review into isolated patches. The reviewer then
resumes to inspect the response and correction and owns only its resulting disposition.

Resume the same local reviewer for follow-up. Publish its verified disposition in the
existing discussion, linking the correction revision and relevant evidence. Keep new
findings distinct and avoid duplicate threads for an unchanged concern. Resolve a
thread only after the correction has been checked and resolution is authorized; author
claims, passing CI, or a merge alone do not establish that the concern was addressed.
Leave unresolved decisions visible. A final review conclusion names the assessed head
and any remaining limits; it never grants merge authority.

A review does not authorize publication, commits, pushes, merges, deployment, or
scheduling. External communication requires approval of the exact content and
destination. Use `improve-review-pr` when the user requests a retrospective or
guidance update; it does not run automatically.
