# CLI execution

The launcher starts one non-interactive reviewer process for a PR or coherent
batch. Any editor, terminal, or agent host can select either installed CLI.
The first conversation is
fresh; follow-ups resume its exact reviewer, never the coding session.

## Setup

Use the repository's `.agents/skills/review-pr` directory. The `.claude/skills`
entry links to that same skill; a separate personal copy may be older. Run it in
the [dev shell](../../../../AGENTS.md#dev-shell), which supplies Python, OpenSSL
and both reviewer CLIs (`codex` and `claude`). The GitHub CLI (`gh`) must also be
on PATH; report it as missing rather than installing it on the host. The live
workflow was verified with
Codex CLI 0.163.0 and Claude Code 2.1.310; an older CLI must support the flags
used by the launcher.

Authenticate with your own accounts: `gh auth login`, then `codex login` or
`claude auth login` for the selected reviewer engine. Your GitHub account needs
read access to `tyler274/coda-rust`.

This repository carries no reviewer GitHub App: there is no `--github-app-config`
to pass and no secret to fetch. The reviewer's `publish` tool therefore refuses,
and a run finishes as `publication_unverified` with its draft retained under the
run's `publications/`. That is the expected finish; the operator reads the draft
and `review.md` back from there.

Use the invocation below with `--dry-run` to check PR access and prepare the
reviewer command without starting a model. Omit `--dry-run` only when requesting
an actual review.

## Invoke

Use absolute paths to this repository's script and a trusted workspace. Replace
`/absolute/path/to/coda-rust` with the checkout containing this skill:

```bash
python3 /absolute/path/to/coda-rust/.agents/skills/review-pr/scripts/run_review.py \
  --pr https://github.com/OWNER/REPO/pull/123 \
  --workspace /absolute/path/to/trusted/workspace \
  --engine codex --model gpt-6-astra --effort xhigh
```

Select `--engine claude --model opus` for Claude. Repeat `--pr URL` for every
batch target, including cross-repository targets when relevant. Use the existing
`--context-file /absolute/path/to/context.md` for sourced intent, roles,
dependencies, decisions, and earlier findings. The reviewer retrieves source
and surveys related open work; the launcher does not infer dependencies.

After completion, resume the saved reviewer:

```bash
python3 /absolute/path/to/coda-rust/.agents/skills/review-pr/scripts/run_review.py \
  --resume-from /absolute/path/to/prior/run.json \
  --context-file /absolute/path/to/followup-context.md
```

This inherits all targets, workspace, engine, model, and effort. Explicit model,
effort, or trusted-workspace overrides remain available; the engine and target
set must match. Use a saved launcher run, not a raw session ID or coding
transcript. Older single-target run records remain valid. Wait for a run to
finish before resuming it. A changed target set or deliberately fresh opinion
starts without `--resume-from`.

The workspace need not contain the author's checkout. Use a trusted scratch
workspace when needed; do not launch in a downloaded contributor-controlled
folder merely to load its settings or hooks. Both engines retain their normal
authentication and applicable configuration. CLI isolation and permissions are
not identical. Both receive the same local reviewer MCP server for direct
target-repository reads and publication drafting. `--github-app-config` exists
for a repository with a reviewer App; this one passes none, so the server's
`publish` refuses and the draft stays in the run. Do not put credential values
in run artifacts or model prompts.

`--dry-run` reads the PRs and prepares the request without starting a model.
Python 3.10+, authenticated `gh`, and the selected CLI must be on PATH.
`--output-dir` accepts a new scratch directory; otherwise results go under
`work/pr-review-*` in the workspace.
Existing run files are never overwritten.

## Execute and receive

Codex uses `exec` with a read-only shell sandbox, approval policy `never`, explicit
model/effort, JSON events, and a final-message file. Follow-ups add
`resume <saved-id>`. Claude loads only the supplied MCP config for this invocation,
refreshes its system prompt/tool descriptions on each invocation while preserving
the saved conversation, uses JSON output and disabled permission prompts, and exposes
only Read/Glob/Grep plus the reviewer MCP tools; follow-ups add
`--resume <saved-id>`. Source remains read-only. Within the scoped reviewer MCP
server, `reviewer.publish` is its only remote write capability, and without App
credentials it refuses; the reviewer's saved draft is the deliverable. Codex
retains its applicable configuration and is instructed to use no other connector
for publication; its read-only shell sandbox does not by itself establish global
tool isolation.

Run through the host's background execution and consume completion delivery.
A terminal user can run directly and collect the result when the process exits.
In Claude Code, use the parent's Bash tool with `run_in_background: true`, not
a subagent or `claude --bg`. The launcher does not daemonize or impose a model
timeout. Do not add polling, sleep loops, or scheduling.

Read `run.json` for targets, selected settings, session identity, and status;
read `review.md` for the complete assessment. `request.md` and native stdout/stderr
logs support diagnosis. These are scratch transport files; report in chat.

All targets must be readable before the reviewer starts. After it finishes, the
launcher rechecks every target's head, base, base branch, and open/closed/merged
state, recording changed targets and readback errors. Context-only PRs are outside
that automated check. A resume must return the requested session ID. Missing or
failed results never become completed reviews, and failures do not trigger an
automatic fresh-session fallback.

- Exit 0: prepared in dry-run mode, or completed; this is not an approval.
- Exit 1: failed; inspect the recorded error and logs before retrying. A failed
  run cannot be resumed, even when its reviewer session started: `--resume-from`
  refuses it. Resolve the cause, then resume the last finished run of that
  reviewer, or start a fresh review with the earlier findings in its context when
  no finished run exists.
- Exit 3: a report exists, but direct reviewer access was not demonstrated
  (`access_unverified`), a target changed (`stale`), or its current state could not
  be established (`freshness_unverified`), or a target lacks a verified publication
  (`publication_unverified`, the normal finish in this repository). These finished
  runs can be resumed.

Launcher tests with CLI stand-ins verify transport, not model judgment or live
session restoration. Keep those proof boundaries explicit.

## Sources

- [Codex non-interactive mode](https://learn.chatgpt.com/docs/non-interactive-mode)
- [Claude programmatic execution](https://code.claude.com/docs/en/headless)
- [Claude permissions](https://code.claude.com/docs/en/permissions)

## Drafts

The reviewer saves its assessment with `prepare_publication` as a draft under the
current run's `publications/`, bound to the run and the reviewed revision. The
`publish` tool accepts only such a draft with a matching content digest and
refuses a changed or closed PR, a changed draft, an out-of-scope target, or
missing App credentials; this repository has no App, so it refuses, and the draft
and `review.md` are what the operator reads back. A verified readback, where an
App exists, is recorded in `github_publications`; a draft or attempted write is
not a receipt.

Resume the same reviewer with `--resume-from` for corrections and follow-ups; it
re-reads the target and prepares a new draft itself.
