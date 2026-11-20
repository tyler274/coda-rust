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
$P run --claim AC1 --label "before the fix" --expect-fail --expect 'right: "rejected"' \
  -- cargo test -p physician-sample --test try_again -- second_resume_is_rejected
$P run --claim AC1 --label "after the fix" -- cargo test -p physician-sample --test try_again
$P run --claim AC2 --expect 'failed\|5\|5' -- psql "$URL" -At -c "select ..."
$P add --claim AC3 --revision HEAD --label "Try again, site A" "$RUN/couldnt-finish-try-again/site-a-failed.png"
$P add --claim AC3 --revision HEAD --label "scale stack run" "$RUN/couldnt-finish-try-again"
$P screenshot --claim AC4 --url http://127.0.0.1:8080/ --label "empty state"
$P gap --claim AC5 --reason "needs hosted staging; not run"
$P status
$P keep 1 --reason "deliberately before the fix"
$P keep 4 5 --reason "later commit changed only docs"
$P supersede 7 --reason "AC5 since proven on staging"
```

- `run` passes the command's output through and exits nonzero when the entry is not
  what was asked: a failure, an expected failure that passed, or missing `--expect`
  output. `--expect` matters where success is not the exit status, such as a
  `rejected` answer with HTTP 200.
- For a defect a test can show, write the test first and record it failing before
  you write the fix. Select that test, and give `--expect-fail` an `--expect` naming
  the assertion the defect breaks, so a build error or an unrelated setup panic does
  not count. The entry predates the fix on purpose: `keep` it rather than re-run it.
- Prefer the feature's [verify skill](../../../verification/README.md#verify-skills).
  Add its evidence folder or the specific files its recipe names rather than
  capturing the same behavior again. `add` cannot see when an artifact was made:
  give `--revision` the revision the binaries and page were built from, or the
  entry stays marked as of unknown revision. A browser step needs its own screenshot; an
  HTTP pass does not stand in for it.
- Look at every screenshot before recording it. A page can contain the right text
  while the panel is clipped, blank or off-screen.
- Record a criterion you cannot prove here as a gap with the reason. Do not weaken
  the criterion to fit the proof available.
- Scale evidence to the change. A wording or skill change can rest on the checks CI
  runs for it and, for a skill, one dry run of the changed instructions; it does not
  need a running stack.

`status` lists failing and stale entries first and exits 1 when any remain. An entry
is stale when it ran against another revision or with uncommitted changes to tracked
files. It does not see untracked files: add a new file to Git before recording
evidence that depends on it, or the entry reads as current when it is not. Re-run
it and `supersede` the old entry, or `keep` it at this head with the reason the later
change cannot affect it. Superseding also retires a failure that a correction fixed
or a gap that later evidence filled; the manifest keeps the history, and the Proof
section shows only current entries. Several entries can share a criterion, such as
the failure before a fix and the pass after it.

## Publish and render

Use synthetic data only. `check` refuses tokens, sessions, keys and other credential
shapes, including Coda's `codas-`/`codap-`/`codaf-` tokens and credentials in
arguments and URLs, in the artifacts and the manifest. It also refuses files or
bundles over the size limits and warns about email addresses outside synthetic
domains. `run` records `--session`, `--token` and similar argument values as
`<redacted>`, but a command that prints a credential still puts it in its log.
Screenshots are not scanned: look at them. Remove or regenerate a refused artifact rather than
editing the check.

```bash
$P check
$P publish                      # refs/proof/<branch> on origin
$P render > target/verification/proof/pr-section.md
```

`publish` pushes the bundle as a commit on `refs/proof/<branch>`, outside
`refs/heads`, so ordinary clones and branch lists do not fetch it; mirror clones do.
Each publish adds a commit whose parent is the previous one, so links to earlier
revisions keep working while the ref exists. Deleting the ref
(`git push origin :refs/proof/<branch>`) lets GitHub collect the commits, after which
their links stop working. Links use the repository's own access controls. Publishing
is part of the PR's publication and needs the same approval. Pass a remote URL as
`--remote` when the configured remote is unreachable.

`render` prints the Proof section: a link to the whole bundle, one row per current
entry with its criterion, evidence and result, the last lines of each run's output
(redacted), and embedded screenshots. Links are to the published commit while it
matches the current revision and bundle; after a new commit or new evidence they fall
back to local paths, which only suit a draft returned in chat. Publish again so the
section describes the PR head, then open its links in the saved PR.
