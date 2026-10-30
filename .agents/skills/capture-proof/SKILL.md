---
name: capture-proof
description: Record each acceptance criterion's evidence as inspectable artifacts tied to a revision, check them for secrets, publish them, and render a PR's Proof section. Use while verifying a change and when writing its PR.
---

# Capture Proof

Evidence is what a reader outside the coding conversation can inspect: the command
with its exit status and output, a response body, a log, a screenshot. A statement
that a check passed is not evidence. Follow the shared
[development workflow](../../../AGENTS.md#development-workflow);
[implement-ticket](../implement-ticket/SKILL.md) chooses each criterion's evidence
and [write-pr](../write-pr/SKILL.md) presents it.

## Record

[proof.py](scripts/proof.py) keeps the change's bundle in
`target/verification/proof/<branch>/`, with `manifest.json` beside the artifacts.
Each entry names its criterion and the revision it ran against. Run it from the
checkout, inside the dev shell when the command needs it:

```bash
P=.agents/skills/capture-proof/scripts/proof.py
$P run --claim AC1 --label "try again" -- cargo test -p physician-sample --test try_again
$P run --claim AC1 --label "before the fix" --expect-fail --at origin/main \
  --with crates/physician-sample/tests/try_again.rs -- cargo test -p physician-sample --test try_again
$P run --claim AC2 --expect 'failed\|5\|5' -- psql "$URL" -At -c "select ..."
$P add --claim AC3 --label "Try again, site A" "$RUN/couldnt-finish-try-again/site-a-failed.png"
$P add --claim AC3 --label "scale stack run" "$RUN/couldnt-finish-try-again"
$P screenshot --claim AC4 --url http://127.0.0.1:8080/ --label "empty state"
$P gap --claim AC5 --reason "needs hosted staging; not run"
$P status
```

- `run` passes the command's output through and exits nonzero when the entry is not
  what was asked: a failure, an expected failure that passed, or missing `--expect`
  output. `--expect` matters where success is not the exit status, such as a
  `rejected` answer with HTTP 200.
- A defect's proof includes the failure before the fix: `--at` runs in a temporary
  worktree at that revision, and `--with` copies the new test into it.
- Prefer the feature's [verify skill](../../../verification/README.md#verify-skills).
  Add its evidence folder or the specific files its recipe names rather than
  capturing the same behavior again. A browser step needs its own screenshot; an
  HTTP pass does not stand in for it.
- Look at every screenshot before recording it. A page can contain the right text
  while the panel is clipped, blank or off-screen.
- Record a criterion you cannot prove here as a gap with the reason. Do not weaken
  the criterion to fit the proof available.
- Scale evidence to the change. A wording or skill change can rest on the checks CI
  runs for it and, for a skill, one dry run of the changed instructions; it does not
  need a running stack.

`status` lists failing and stale entries first and exits 1 when any remain. An entry
is stale when it ran against another revision or with uncommitted changes. Re-run
it, or keep it with a stated reason that the later change cannot affect it.

## Publish and render

Use synthetic data only. `check` refuses tokens, sessions, keys and other credential
shapes, and files or bundles over the size limits; it warns about email addresses
outside synthetic domains. Remove or regenerate a refused artifact rather than
editing the check.

```bash
$P check
$P publish                      # refs/proof/<branch> on origin
$P render > target/verification/proof/pr-section.md
```

`publish` pushes the bundle as a commit on `refs/proof/<branch>`, outside
`refs/heads`, so clones and branch lists do not fetch it. Each publish adds a commit
whose parent is the previous one, so links to earlier revisions keep working. Links
point at that commit through the repository's own access controls, and images render
for signed-in readers. Publishing is part of the PR's publication and needs the same
approval. Pass a remote URL as `--remote` when the configured remote is unreachable.

`render` prints the Proof section: one row per entry with its criterion, evidence and
result, the last lines of each run's output, and embedded screenshots. Links are to the
published commit when it matches the current revision, otherwise to local paths, which
only suit a draft returned in chat. Publish again after new commits so the section
describes the PR head.
