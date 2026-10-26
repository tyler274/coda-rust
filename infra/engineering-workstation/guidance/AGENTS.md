# Engineering workstation guidance

These preferences apply to local Codex and Claude Code work on this workstation.
Follow the task's authorization and the responsible repository's instructions.

## Communication

- Do not use em dashes in authored prose. Use a period, comma, colon, or parentheses.
- Lead with the result or decision. Use plain language and short paragraphs.
- Leave a blank line between paragraphs, before lists, and between distinct findings.
- Use short headings only when they help navigation. Use bold sparingly for outcomes
  and decisions. Avoid dense walls of text and unnecessary nested lists.
- Keep each Slack message readable on a phone. Give the conclusion first, then the
  few facts or actions the reader needs. Keep long evidence in its owning task.
- Use descriptive link labels and link directly to the supporting source. Check
  the actual destination. Do not invent a share URL, assume a local file link works
  on another device, or publish a conversation snapshot just to provide a link.
- Keep internal thread IDs, machine paths, tool syntax, JSON and diagnostic detail
  out of ordinary updates. Include exact technical details when they help someone
  inspect evidence, approve an action, or resolve a problem.
- Distinguish completed work, work in progress, blocked work and unverified claims.
  A saved configuration is not proof of a future scheduled event.
- Keep report titles and prose free of decorative emojis. Prefer a single native
  Slack reaction for useful status, managed by the bridge. Do not repeat that
  reaction in the message text or decorate findings and headings. A completion
  reaction means the response was delivered, not that an audit found no issues.
- Preserve exact code, commands, identifiers, quotations and source facts when
  their spelling matters. Style edits must not change their meaning.

## Verification cost

- Do not launch expensive model runs, full security or CI reviews, repeated live
  agent conversations, or broad builds merely to test wiring or presentation.
- Use saved outputs, synthetic inputs, syntax checks, focused tests, direct API
  reads and native UI readback where they establish the needed fact.
- Run the smallest meaningful check for the change. Reuse completed evidence and
  stop when the relevant proof is complete. Do not repeat a whole workflow solely
  because another check or UI path is available.
- If an expensive live check is necessary and not already explicitly authorized,
  explain the remaining uncertainty and expected cost before asking the owner to approve
  that specific run. Existing scheduled jobs retain their agreed scope and cadence.
- Preserve useful build caches during temporary-file cleanup unless reclaiming
  that space is requested. Avoid making the next real job rebuild unnecessarily.

## Slack ownership

The configured private Slack channel is a shared engineering conversation. Human
participants may contribute to the same native task; retain who asked for what.
The configured workstation owner remains the responsible contact. Collaboration
does not grant new provider permissions or supersede the repository's approval
requirements for publishing, merging, deployment or access changes.

The gateway owns delivery of agreed scheduled-task results, Slack-started
conversations and follow-ups. An
agent returns its report to the native task and must not send a duplicate Slack
message itself. Native approvals apply only to their specific pending request;
Slack delivery or a reconnect does not grant standing permission.

For channel-specific composition, use the shared workstation-communication skill.

## Company GitHub access

For `Coda-Health/coda-rust`, use the `github-company` MCP tools or `coda-gh`
for GitHub CLI commands. The company app permits repository/PR reads and Actions
reads, including completed job logs. Normal HTTPS Git fetch uses the installed
company credential helper. If access fails, report the missing coverage; do not
sign in as a person or switch to personal credentials. Publishing still requires
an approved write-capable path, which this read-only integration does not supply.
