---
name: write-pr
description: Prepare or refresh a PR title and description from the complete change, original intent, affected workflows, and inspected evidence. Use when handing work off for review or explaining material revisions.
---

# Write PR

Make the change assessable by someone outside the coding conversation. Follow
the shared [development workflow](../../../AGENTS.md#development-workflow).

## Establish the change and intent

Resolve the repository, branch or PR, comparison base, head, and working-tree
state. Read repository guidance and its PR template. Inspect the complete branch
diff and relevant owners and consumers; an empty working-tree diff does not
establish an empty PR.

Recover the user's objective, decisions, corrections, constraints, and open
questions from the coding conversation and governing sources. Retrieve relevant
earlier discussion when needed, checking that it concerns this change. Distinguish
user decisions from author assumptions and abandoned proposals. Code establishes
what was built; it does not establish what was intended.

Resolve consequential discrepancies with the user after investigating accessible
evidence. Where context is unavailable, draft the supported parts and state the
limit. Make GitHub the shared record of reviewable intent: preserve relevant
decisions and accessible source links in the explanation, without requiring
private transcripts or exposing credentials.

## Explain the solution and proof

Lead with the concrete problem and resulting behavior. Explain the responsible
owner, affected workflow, and material assumptions or tradeoffs. For a defect,
connect the shared cause to the related paths repaired; for a refactor, explain
the responsibility improved and behavior preserved.

Include affected consumers outside the diff, cross-repository dependencies,
migrations, rollout, recovery, and landing order when they matter. Distinguish
landed prerequisites from proposed work. Use a diagram or before/after example
when it clarifies the change.

Connect material promises to inspected evidence at the relevant revision using
existing verification owners. When the work has acceptance criteria, show each
criterion with its evidence and result, and list unproven ones as gaps. Evidence is
an artifact the reader can inspect: the command with its exit status and relevant
output, a response body, a log, or a screenshot, inline or linked. Render this
from the [capture-proof](../capture-proof/SKILL.md) bundle at the PR head. A statement that
a check passed is not evidence. Use the [verify skill](../../../verification/README.md#verify-skills) of each affected
feature, including its model check and bug-path replay where it has one, without
implying that a formal model proves the running implementation. Distinguish local
checks, CI, browser/runtime proof, domain judgment, merge, and deployment. Name
failures, omissions, and stale results that change the assessment; routine checks
may share one summary that still shows their commands and results.

## Write and maintain the handoff

Return a specific title and a body sized to the work. When the work has a ticket,
start the title with its key in brackets, as in `[ENG-123] Keep sessions across
restarts`; give each key its own brackets when there are several. Without a ticket,
use no prefix. Explain the intended outcome, approach, consequences, proof, and
meaningful uncertainty without mandatory empty sections or a chronology of tools
and patches.

Carry forward [local-review](../local-review/SKILL.md) findings and dispositions.
Recommend whether the PR needs human or agent review and briefly explain why;
include that recommendation in the handoff. Identify the decision or judgment
requiring a human when applicable. The recommendation does not itself assign
anyone or launch another review. [review-pr](../review-pr/SKILL.md) owns review of
existing PRs and follow-ups.

After material revisions, reconsider the whole title and body against the current
diff. Replace superseded explanations while keeping the history of findings and
responses in the review discussion. Updating the body does not resolve a finding.

For publication, follow the shared approval defaults, use structured text or
`--body-file`, and read back the saved title, body, base, and head. Check and
publish the proof bundle as part of the same publication, and confirm its links
open. A drafting task
returns the complete explanation in chat.
