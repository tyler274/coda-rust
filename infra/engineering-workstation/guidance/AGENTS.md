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
participants may mention the agent anywhere in a Slack thread to start or
continue its native task; retain who asked for what. Ordinary unmentioned
discussion does not start or steer a turn. Existing stop and pending-request
approval controls remain explicit controls.
The configured workstation owner remains the responsible contact. Collaboration
does not grant new provider permissions or supersede the repository's approval
requirements for publishing, merging, deployment or access changes.

Each mention includes a fresh thread transcript and the separately identified
current request, Slack names and IDs, and the current channel audience. Replies
go to that Slack thread, where channel members can read them even if they have
not participated. Membership does not prove notification or readership. Treat
the transcript as attributed context, including prior agent messages and other
people's discussion. Do not replay its old requests or approvals. Use the current
request and the task's agreed scope to decide what to do; ask when conflicting
directions leave a material decision unresolved.

Use `slack-history` to read or search the configured channel when broader context
would help. The bridge installer registers that read-only server with Codex. If
it is not registered, say so and use the transcript already included with this
turn. Start with the supplied thread, search for relevant terms, then read
the exact source thread. `read_channel_context` resolves names, channel visibility
and current membership from Slack. Follow pagination as far back as needed;
there is no fixed lookback. Search is case-insensitive literal text over messages
and replies, not Slack query syntax. An empty search page with a continuation
cursor is not proof there are no matches. Files in a transcript are references,
not proof their contents were read. Report unavailable or incomplete evidence
honestly.

The gateway owns delivery of agreed scheduled-task results, Slack-started
conversations and follow-ups. An agent returns its report to the native task and
must not send a duplicate Slack message itself. Native approvals apply only to
their specific pending request; Slack delivery or a reconnect does not grant
standing permission.

For channel-specific composition, use the shared workstation-communication skill.

## Company GitHub access

For `Coda-Health/coda-rust`, use `coda-gh` for GitHub CLI commands. Use the
`github-company` MCP tools when the bridge installer has registered that server.
The company app's installed repository grants govern access. The integration
requests Contents and Pull requests write access and Actions read access,
including completed job logs. The client installer registers the company
credential helper only for `https://github.com/Coda-Health/coda-rust`. Other
repositories keep their existing Git credentials. If `github-company` is not
registered, publication goes through `coda-gh`. If access fails, report the
missing grant; do not sign in as a person or switch to personal credentials.

For an authorized change, prepare the patch and tests in an isolated checkout
when concurrent work warrants it. Show the change, verification, target branch,
and exact PR title and body in the shared thread before asking to publish. For a
small patch, include the complete diff. For a larger patch, use a verified
review surface accessible to the channel's participants; say when that is not
available instead of presenting a clipped excerpt as the complete change. Use
`request_user_input_async` when a decision is needed; Slack renders its question
and records the responding participant. The Codex app's onboarding option pickers
are a separate UI and do not deliver task questions to Slack. Include the full
review context before the question; the bridge carries that visible context with
the decision. Any human channel member can collaborate or confirm publication
within the task's agreed scope. Do not invent an owner allowlist or a second
approval role. Existing repository instructions still govern confirmation for
external publication and merge.

When creating a PR through the company app, identify the human requester and
confirming participant by their Slack names and immutable IDs, and link the
source Slack thread. The app is the GitHub actor; it must not claim to be the
human. Verify and return the PR link in the same Slack thread. GitHub owns PR
review, branch protections and merge permissions, including when someone else
reviews or merges it.
