# Codex reviewer execution

Use a fresh, non-interactive Codex CLI session with `gpt-6-astra` and `xhigh`
effort unless the user explicitly selects another configuration. The reviewer
stays read-only. Follow the shared local-review method and repository development
defaults.

Prepare the sourced context and the full text of the
[review standard](standard.md) as a task-local request file. The `codex` CLI must be installed, authenticated, and available on
PATH. Replace the example paths with this developer's workspace and scratch
files. Run through the host's background process facility; in Claude Code, use
the parent's Bash tool with `run_in_background: true`. In a terminal, run directly
and collect the result when the process exits:

```bash
codex exec -C /absolute/trusted/workspace --skip-git-repo-check \
  --sandbox read-only -c 'approval_policy="never"' \
  --model gpt-6-astra -c 'model_reasoning_effort="xhigh"' \
  --json --output-last-message /absolute/run/review.md \
  - < /absolute/run/request.md > /absolute/run/stdout.log 2> /absolute/run/stderr.log
```

For a correction, use the same invocation with `resume SAVED_SESSION_ID -`
in place of the final `-`. Keep the execution options before `resume`, reapplying
the model, read-only permissions, and output settings. Use a new output directory
and the updated request; never overwrite the initial receipt. Do not use `--last`,
the coding session, or a fresh reviewer as a substitute for that exact session.

For explicitly selected PR targets, switch to the
[PR workflow](../../review-pr/SKILL.md), including its shared App configuration
and reviewer-owned publication. Its
[execution reference](../../review-pr/references/cli-execution.md) owns the
launcher flags, exact-session resume, and publication receipts.

Use completion delivery or a wait on the launched process. Do not add active
polling, log-tail loops,
or a scheduled monitor to this route. Preserve the command, target fingerprints,
output paths, and process identity in task-local scratch. When completion arrives,
inspect all JSON events: require `turn.completed`, no failed turn, a `thread_id`
from `thread.started`, and a nonempty final report. A correction must return the
saved session ID. Record requested model and confirm returned identity when
available; state any verification limit.

Recheck frozen targets before accepting the report. Present its complete useful
assessment in chat, distinguish reviewer findings from parent judgment, and apply
the shared reconciliation before any edits. Failed, stale, or unreadable output
does not establish readiness; investigate before retrying or changing routes.
