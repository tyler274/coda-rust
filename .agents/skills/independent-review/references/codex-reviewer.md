# Codex reviewer execution from Claude Code

Use a fresh, non-interactive Codex CLI session with `gpt-6-astra` and `xhigh`
effort unless the user explicitly selects another configuration. The reviewer
stays read-only. Dispatch requires the independent-review request described in the
shared method; this reference does not add automatic pre-commit review.

Prepare the shared [review method](../SKILL.md) and sourced context as a task-local
request file. Run from a trusted workspace through the parent's Bash tool with
`run_in_background: true`, never in a subagent or blocking foreground call:

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

For explicitly selected PR targets, use the existing
[PR launcher](../../review-pr/scripts/run_review.py) with `--engine codex`,
`--model gpt-6-astra`, `--effort xhigh`, and `--context-file`; resume with
`--resume-from`. Its [execution reference](../../review-pr/references/cli-execution.md)
owns the flags and PR freshness checks. Leave the launcher unchanged.

Use background completion delivery. Do not add active polling, log-tail loops,
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
