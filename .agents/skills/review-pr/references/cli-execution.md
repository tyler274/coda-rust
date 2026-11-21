# CLI execution

The launcher starts one non-interactive reviewer process for a PR or coherent
batch. Any editor, terminal, or agent host can select either installed CLI.
The first conversation is
fresh; follow-ups resume its exact reviewer, never the coding session.

## Setup for Dabney.moe

Use the repository's `.agents/skills/review-pr` directory. The `.claude/skills`
entry links to that same skill; a separate personal copy may be older. Install
Python 3.10+, GitHub CLI (`gh`), Doppler CLI, OpenSSL, and the reviewer CLI you
intend to use (`codex` or `claude`), with their executables on PATH. The live
workflow was verified with Codex CLI 0.163.0 and Claude Code 2.1.310; an older CLI
must support the flags used by the launcher.

Authenticate with your own accounts:

```bash
gh auth login
doppler login
```

Sign in to the selected reviewer engine with `codex login` or `claude auth login`.
Your GitHub account needs read access to `tyler274/coda-rust`. Your Doppler
account needs access to `CODA_PR_REVIEWER_PRIVATE_KEY` in project `coda-rust`,
config `dev`; request that access from a Doppler administrator if it is missing.
Having repository access alone does not grant secret access.

The shared [reviewer-app.json](../reviewer-app.json) contains the Coda PR Reviewer
App ID, installation ID, and the command that retrieves its existing key from
Doppler. These are non-secret settings. Use this file directly; do not create a
personal copy or put a private key or token in the checkout. Publication uses
`coda-pr-reviewer[bot]`, independently of the GitHub account used for reads. The
App must be installed on `tyler274/coda-rust` for publication to succeed; the
recorded installation came from Coda Health and has not been verified against
this repository.

Verify key retrieval without displaying or saving the key. Replace the checkout
path below with your own:

```bash
python3 - /absolute/path/to/coda-rust/.agents/skills/review-pr/reviewer-app.json <<'PYTHON'
import json
from pathlib import Path
import subprocess
import sys

config = json.loads(Path(sys.argv[1]).read_text())
result = subprocess.run(config["private_key_command"], capture_output=True)
if result.returncode or not result.stdout.strip():
    raise SystemExit("Reviewer key unavailable; check Doppler login and access.")
print("Reviewer key is accessible; no secret value was displayed or saved.")
PYTHON
```

Then use the invocation below with `--dry-run` to check PR access and prepare the
reviewer command without starting a model or posting anything. A dry-run does not
verify model login or live publication. Omit `--dry-run` only when requesting an
actual review, which includes reviewer-bot publication.

## Invoke

Use absolute paths to this repository's script and a trusted workspace. Replace
`/absolute/path/to/coda-rust` with the checkout containing this skill:

```bash
python3 /absolute/path/to/coda-rust/.agents/skills/review-pr/scripts/run_review.py \
  --pr https://github.com/OWNER/REPO/pull/123 \
  --workspace /absolute/path/to/trusted/workspace \
  --github-app-config /absolute/path/to/coda-rust/.agents/skills/review-pr/reviewer-app.json \
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
target-repository reads, publication drafting, and reviewer-owned bot publication. The
config file contains App and installation IDs plus a command that retrieves the
private key; it contains no private key or token. Its path is inherited by
follow-ups. Do not put credential values in run artifacts or model prompts.

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
server, `reviewer.publish` is its only remote write capability. It publishes the
reviewer's own saved draft on a selected PR under the user's standing review
authorization; there is no separate approval turn. Codex retains its applicable
configuration and is instructed to use no other connector for publication; its
read-only shell sandbox does not by itself establish global tool isolation.

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
- Exit 1: failed; inspect the recorded error and logs before retrying.
- Exit 3: a report exists, but direct reviewer access was not demonstrated
  (`access_unverified`), a target changed (`stale`), or its current state could not
  be established (`freshness_unverified`), or a target lacks a verified publication
  (`publication_unverified`). These finished runs can be resumed after resolving
  the reported gap.

Launcher tests with CLI stand-ins verify transport, not model judgment or live
session restoration. Keep those proof boundaries explicit.

## Sources

- [Codex non-interactive mode](https://learn.chatgpt.com/docs/non-interactive-mode)
- [Claude programmatic execution](https://code.claude.com/docs/en/headless)
- [Claude permissions](https://code.claude.com/docs/en/permissions)

## Reviewer-owned publication

The user's PR review request authorizes the reviewer to publish its own reviews,
inline findings, and follow-up replies on the selected PRs without per-comment
approval. The reviewer calls `prepare_publication`, then `publish` in the same turn.
The launcher supplies this standing exception on fresh and resumed turns; it does
not authorize author responses, unrelated messages, thread resolution, or merges.

Drafts live under the current run's `publications/`. The publisher accepts only a
saved draft from that run with a matching content digest. It refuses a changed or
closed PR, a changed draft, an out-of-scope target, or missing App credentials.
A verified readback is recorded in `github_publications`; a draft or attempted write
is not a receipt. The receipt links to its owning `run.json`, which records the
returned native session ID when the CLI finishes. On a resumed run, the receipt
also includes the already-known reviewer session ID.

An uncertain attempt stops before another write so GitHub state can be reconciled.
Do not start a new run or prepare a replacement to bypass that uncertainty. After
resolving a publication failure, resume the same reviewer using `--resume-from` and
supply the concrete reconciliation evidence. The reviewer re-reads the target and
prepares any needed publication itself. Later code and discussion follow-ups use
that same session as well.
