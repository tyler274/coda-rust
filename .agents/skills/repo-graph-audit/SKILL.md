---
name: repo-graph-audit
description: "Map a repository's structural dependencies and workflow handoffs to audit ownership, boundaries, documentation/code drift, and proof gaps. Use for a full node-and-edge map, cross-component coherence audit, or source-backed reconciliation plan. A focused bug investigation or ordinary repo orientation usually does not need this full pass."
---

# Repo Graph Audit

Audit how the repository's instructions, implementation, artifacts, and evidence
fit together. Produce two connected layers: extracted structural relationships
and source-backed judgments about workflow handoffs. The latter requires reading
the responsible owners; an import graph cannot supply it.

## Pin the target and the meaning of coverage

Honor the requested repository, revision, branch, or local changes. Record its
root, revision, dirty state, and relevant base. Follow its guidance on isolation;
read-only inspection does not by itself require a new worktree. If the user names
no revision, use the checkout they are asking about and state that choice. Do not
silently substitute a default branch or mix different revisions' files.

Read applicable instructions, the repository front door, and the owning product
and architecture sources. Distinguish user decisions, documented contracts,
observed implementation, and your assumptions. Conflicts between them are audit
subjects; code is not automatically the intended behavior.

Define the finite universe before using words such as **full** or **every**:

- A full repository map inventories every tracked entry at the selected target.
  A working-tree audit also accounts for relevant changed and untracked files.
- Classify generated outputs, vendor code, data, historical material, submodules,
  and external dependencies explicitly. Inventory can be complete while their
  contents remain outside semantic review. Do not silently drop them.
- Name the structural relationship families the extraction covers and the
  workflow boundaries being judged. File coverage is not a complete function
  call graph, dynamic execution graph, or domain-content audit.

An audit remains read-only unless implementation was requested. Parsing source
does not require running its scripts. Use existing authority for any checks,
external access, spend, or delegation; this skill grants none. Keep transient
analysis outside the repository and findings in chat unless another deliverable
was requested. A useful map need not become a maintained repository artifact.

## Build the structural layer

Inventory first, then extract relationships using the languages and conventions
actually present. Read [Extraction and evidence](references/extraction-and-evidence.md)
when selecting collectors or preparing machine-readable graph data.

Useful families include imports, document links, CLI registrations and mentions,
build/package inclusion, configuration selection, routes, persistence references,
CI commands, source pins, and symlinks. Keep their meanings separate: a command
mention is not an invocation, and a CI command is not a successful test result.

Retain source locations and unresolved targets. Separate declared external
dependencies from failed internal resolution. Account for language aliases,
workspace packages, build variants, and runtime registration before calling an
unresolved edge broken. A regex match is a candidate, not semantic evidence.

Use existing repo tooling when it provides the needed relationships. Small,
task-local collectors are appropriate; do not create a universal parser, daemon,
or graph database to perform this audit.

When tracing a workflow through branch-heavy code, optionally run Lizard on the
relevant file or directory at the selected revision. For example, from the
Coda Rust checkout:

```sh
uvx --from lizard==1.17.31 lizard -l rust crates/physician-app/src/lib.rs
```

The `CCN` column estimates cyclomatic complexity per function. Lizard can exit 1
after printing a report with warnings. Use the report to pick functions to
read. When a function mixes responsibilities, ask whether an existing framework
pattern would let someone change one behavior without having to understand the
others.

## Build the owner and workflow layer

Group files around responsibilities found in the repository, not a predefined
architecture. An owner may be a module, document, skill, shared library, service,
artifact contract, or human/external decision boundary. It need not be a team.
Give each inventoried file a primary map membership for coverage; represent
shared use as edges. Unclear membership stays explicit rather than being hidden
in an unexplained miscellaneous node.

For each owner, establish its responsibility, authoritative inputs/outputs,
source references, and what it does not decide. Read the actual entrypoints,
callers, downstream consumers, and proof relevant to that responsibility.

Trace each in-scope workflow from its initiating action through configuration,
input admission, execution, persisted result, presentation/export, and readback.
Follow the return paths too: feedback, source corrections, retries, partial
failure, saved-result recovery, version changes, and retirement. Include only
paths the repository supports or promises; do not invent hypothetical machinery.

For each handoff record:

- direction and relation, such as declares, invokes, reads, writes, consumes,
  validates, hands back, recovers, or publishes;
- what crosses the boundary, its identity/version, and who owns its meaning;
- the relevant source locations on both sides;
- disposition, rationale, and the evidence available or missing.

Mark a promised but absent connection as **expected/missing**. Do not present it
as an observed implementation edge. A declared prohibition is a boundary edge,
not a missing capability to implement.

Use these dispositions consistently:

| Disposition | Meaning |
| --- | --- |
| Aligned in source | The inspected owners agree about this handoff; runtime success is not implied. |
| Bounded | An intentional limit or exclusion is supported by the owning contract. |
| Reconcile | Evidence shows contradictory ownership, behavior, procedure, or capability. |
| Unresolved | Evidence is missing, ambiguous, or insufficiently inspected. State what would resolve it. |

Keep evidence level separate from disposition: source inspection, synthetic
execution, local integration, hosted readback, and domain judgment establish
different things. Do not label an uninspected edge Bounded to close the inventory.

## Reconcile through the existing owners

Look for the same fact being independently selected, inferred, copied, or
reinterpreted across boundaries. Compare operator instructions with executable
commands, developer installations with shipped artifacts, current paths with
historical compatibility, and feedback routes with the actual source owner.

Challenge agreement as well as disagreement. Code, tests, and docs can preserve
the same mistaken premise. Judge the workflow against the user's intent, the
participants' actual needs, and independent evidence where available.

For each finding, identify the concrete trigger or contradiction, consequence,
source evidence, responsible owner, bounded correction, and verification needed.
Group related edges by their shared cause. Prefer correcting an existing owner,
removing a duplicate rule, fixing a caller, or narrowing an unsupported promise
before proposing another subsystem. Separate an implementation defect from an
open product decision or missing runtime evidence.

A cycle, high fan-in, wrapper, copied file, or old name is an investigation lead,
not a defect by itself. Check whether it is intentional composition, a generated
projection, a versioned snapshot, or active compatibility. Only call a path dead
after checking registration, packaging, callers, and applicable configurations.

## Verify the map and its conclusions

Check graph integrity: unique identities, valid endpoints, source references at
the selected revision, accounted-for files, dispositions for every workflow
edge, and an explicit reason for isolated nodes. Counts must reconcile with the
inventory. Check evidence on claimed contradictions directly; a file header is
not a substitute for the relevant declaration or behavior.

When execution is needed and authorized, choose the smallest meaningful probe
at the affected boundary. A documented installed command is tested in that
installation; a configuration claim is traced through serialization and its
reader; a recovery claim includes the saved inputs, side effects, and result.
Name the boundary when only source proof is available. The graph does not make
model calls, deployment, publication, or domain validation necessary by itself.

For a full-map request, continue until every in-scope node and workflow edge has
an inspectable disposition, including Unresolved. If a real blocker prevents
coverage, report the partial result explicitly; do not present a sampled overview
as exhaustive. Before finishing, recheck the source state and identify any changes
since the pinned snapshot.

## Deliver

Lead with what needs reconciling and why. Include the baseline, coverage and
exclusions, source-backed findings, bounded recommendations, and remaining proof
or decision gaps. Distinguish inventory totals from inspected workflow totals.

For a large requested map, make the detail inspectable: owner selection,
incoming/outgoing handoffs, disposition filters, underlying files, and source
links. Show findings and an owner neighborhood before an unreadable whole-repo
graph. Use the available visualization skill when creating an interactive map;
otherwise tables or a small diagram are sufficient. Keep source data separate
from presentation so layout cannot change findings. Rendering tests should check
filters, source links, pagination, clipping, and console errors, not just that
the page opens.

A later implementation request should consume the findings through existing
owners. Use [codebase-cleanup-mapping](../codebase-cleanup-mapping/SKILL.md) to
decide whether a proposed removal is safe. Preserve the original map's revision
or explicitly update its evidence. Keep historical audits labeled with the
revision they cover.
