---
name: codebase-cleanup-mapping
description: Decide whether a proposed removal is safe. Use for dead-code cleanup, retiring a surface, or removing OpenTofu configuration.
---

# Codebase Cleanup Mapping

Take one removal decision at a time. A widget, the native files it alone uses,
and its entry in the docs can be one decision. Follow the repository's
[development workflow](../../../AGENTS.md#development-workflow) and
[guidance on retiring a surface](../../../book/src/reading.md#retiring-a-surface).

## Key lessons

### Broken is not dead

Find what a check was meant to protect and where its assertions run. A test
that returns early in CI can still enforce a rule locally. If a rule can no
longer match, trace what changed before deciding whether to repair or retire it.

### Unused is not unneeded

Code with no callers today can still be part of work in progress. An active
ticket can show why it needs to stay before there's a branch to inspect.

## What uses it?

Use name searches to find leads, then check callers, registration, build
variants, packaging, and configuration. For native files, check the relevant
build scripts, manifests, entitlements, and extension targets. A build can
pass with unused public APIs or Cargo dependencies, items marked
`#[allow(dead_code)]`, and code called only by its own tests.

Reuse [repo-graph-audit](../repo-graph-audit/SKILL.md) evidence when it covers
the item at the revision you're reviewing.

## Who still needs it?

Read the relevant [intent spec](../../../intent) and look for related work in
the tracker, branches, and PRs. Use history to see why code or docs are there
when their purpose is unclear. Ask the owner when their answer could change
the decision.

## What could break if it goes?

Follow the deletion through its helpers, dependencies, fixtures, and docs.
Search the docs for the behavior it promised as well as its name.

Run the checks the repository requires and choose additional checks based on
what could break. For example, a downstream crate's tests can verify the
affected behavior even when it never calls the removed item directly. Run the
relevant mobile build if the deletion could affect what ships. Say what test
coverage goes away and which relevant checks could not run.

## Infrastructure needs a different proof

OpenTofu roots need their own checks. Find which affected roots CI validates
and tests, and report any gaps.

1. **Confirm who can apply it.** The backend block identifies the state the
   root targets. Start with the [GCP deployment guide](../../../book/src/gcp.md)
   and confirm apply authority with the responsible operator before removing
   the root or anything it defines.
2. **If the root stays, compare checks before and after.** For the changed
   root and any roots that use its outputs, run `tofu fmt -check`,
   `tofu init -backend=false`, `tofu validate`, and `tofu test` with mocked
   providers. Report what changes in the mock plans.
3. **If the whole root goes, check its resources and consumers.** Removing the
   files leaves live resources in place. Before removing the root, confirm
   with the operator that its resources have been retired or are managed
   elsewhere, and run the root's checks. Afterward, verify that workflows and
   scripts no longer use the root and rerun checks in roots that used its
   outputs.

Mock checks use no live state. Get explicit authorization before planning
against live state or applying changes.

## Record the decision

Say what went away, why it was safe, and link to the evidence. A short PR
note can cover a straightforward removal. Use
[capture-proof](../capture-proof/SKILL.md) when you need to collect evidence.
If the decision depends on an owner's answer or a missing check, leave the
item in place and say what would settle it.
