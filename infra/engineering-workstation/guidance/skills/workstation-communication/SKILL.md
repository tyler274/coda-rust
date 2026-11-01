---
name: workstation-communication
description: Write clear progress updates, engineering reports, and Slack thread messages for a personal Linux workstation. Use for agent-authored updates and scheduled-task results.
---

# Workstation communication

Read [the shared workstation guidance](../../AGENTS.md). It owns writing style,
verification cost and Slack authority for both Codex and Claude Code.

Make the result easy to read on a phone:

- Start with the outcome and any decision the owner needs to make.
- Keep titles, headings and prose plain. The bridge owns native Slack status
  reactions; do not add inline status emojis or duplicate reactions yourself.
  A finished response can contain unresolved findings; completion does not mean
  success or safety.
- Separate the summary, distinct findings and next action with blank lines.
- Keep a finding's impact, evidence link and qualification together.
- Prefer short bullets to wide tables in Slack. Use a table only when comparing
  the columns is essential and the result remains readable at phone width.
- Use named links such as "CI run" or "Relevant source". A link to Codex must
  reach the original conversation on the reader's actual device and host; if
  that path is unverified, explain the limit instead of presenting a working link.
- Do not paste native inbox directives, internal IDs or raw protocol messages into
  an ordinary update. The bridge renders native display cards for Slack.
- Approval requests must retain the exact proposed action, target and consequences.
  Concision must not hide what the user is being asked to authorize.

Do not force every update into a template. For a completed review, a useful shape
is a short conclusion, a few supported findings, and a clear next action when one
exists. Do not invent follow-up work to fill the last section.
