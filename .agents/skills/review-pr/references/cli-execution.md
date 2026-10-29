# CLI execution

The launcher starts one non-interactive reviewer process for a PR or coherent
batch. Either interactive host can select either CLI. The first conversation is
fresh; follow-ups resume its exact reviewer, never the coding session.

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
not identical; respect restrictions and do not alter global settings to bypass them.

`--dry-run` reads the PRs and prepares the request without starting a model.
Python 3.10+, authenticated `gh`, and the selected CLI must be on PATH.
`--output-dir` accepts a new scratch directory; otherwise results go under
`work/pr-review-*` in the workspace.
Existing run files are never overwritten.

## Execute and receive

Codex uses `exec` with a read-only shell sandbox, approval policy `never`, explicit
model/effort, JSON events, and a final-message file. Follow-ups add
`resume <saved-id>`. Claude uses `-p` with JSON output, plan permissions,
permission prompts disabled, and Read/Glob/Grep/Bash tools; follow-ups add
`--resume <saved-id>`. Read-only settings are reapplied on every turn.

Run through the host's background execution and consume completion delivery.
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
- Exit 1: failed; inspect the recorded error and logs before retrying.
- Exit 3: a report exists, but a target changed (`stale`) or its current state
  could not be established (`freshness_unverified`).

Launcher tests with CLI stand-ins verify transport, not model judgment or live
session restoration. Keep those proof boundaries explicit.

## Sources

- [Codex non-interactive mode](https://learn.chatgpt.com/docs/non-interactive-mode)
- [Claude programmatic execution](https://code.claude.com/docs/en/headless)
- [Claude permissions](https://code.claude.com/docs/en/permissions)

## Connect local runs to the GitHub discussion

After the coordinator publishes and reads back a review or follow-up, add a
`github_publications` array to that run's existing `run.json`. Each entry records
`pr_url`, `url` (the verified review or comment URL), and `reviewed_head_sha`.
Append only confirmed publications; drafts and attempted writes are not receipts.
Preserve all existing run fields. These links connect the local reviewer session
to GitHub; GitHub owns the discussion and no comment bodies or disposition ledger
need to be copied into run metadata. Existing runs without this field remain valid.
Use `--resume-from` exactly as before. Publishing is performed by the coordinator,
not the launcher, and does not change reviewer permissions or invocation.
