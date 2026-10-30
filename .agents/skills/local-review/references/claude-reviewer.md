# Claude reviewer execution

Use a fresh, non-interactive Claude CLI session with `claude-opus-5-5` and Claude's
default effort unless the user explicitly selects another effort. Do not silently
substitute the moving `opus` alias, a native subagent, or an interactive coding
session. Report unavailable tools or models without bypassing permissions or
changing global settings. Follow the shared local-review method and repository
development defaults.

Pass the shared [review method](../SKILL.md) and sourced context on stdin from a
trusted workspace. The `claude` CLI must be installed, authenticated, and
available on PATH. Use the host's background process facility; in a terminal,
run directly and collect the result when the process exits:

```text
claude -p --model claude-opus-5-5 --output-format json \
  --permission-mode plan --permission-prompts none --tools Read,Glob,Grep,Bash
```

Add only necessary `--add-dir` paths for frozen repositories and evidence. For
corrections, add `--resume SAVED_SESSION_ID` and reapply model, permissions, and
directory access. Never use `--continue`, `--last`, or the coding session. Start
fresh only for a changed target set or an explicitly requested fresh opinion. A neutral
prompt does not enforce read-only access; keep the permission settings every turn.

For explicitly selected PR targets, switch to the
[PR workflow](../../review-pr/SKILL.md), including its shared App configuration
and reviewer-owned publication. Its
[execution reference](../../review-pr/references/cli-execution.md) owns the
launcher flags, exact-session resume, and publication receipts.

Keep a task-local scratch receipt of command, target fingerprints, guidance/context
paths, output paths, and process identity. Capture stdout/stderr and the full report.
Require a successful JSON result, a session ID, and a nonempty report; corrections
must return the saved ID. Verify the model in model-usage metadata when available
and state any limit. Recheck frozen targets before accepting the report. A clean
exit or transport receipt does not establish review quality or readiness. Do not
add another durable launcher or status system.

## Completion delivery

Use the host's completion delivery or a wait on the launched process; continue
independent work when the host supports background execution.
Do not add active polling, log-tail loops, or an automatic scheduled monitor.
If completion delivery is unavailable, preserve the receipt and report the pending
run and that limit; do not promise automatic continuation. Scheduling requires a
separate user request and the host's supported tools, not a personal skill dependency.

On completion, read the whole report and recheck the frozen targets. Reconcile the
assessment in the owning chat and continue only authorized work. For corrections,
resume the saved reviewer with the updated context and retain the prior evidence.
