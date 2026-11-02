# Coda Rust operating contract

This repository is the Rust workspace for Duet and the physician client. The [intent specs](intent/README.md) own product intent: the principles, the
product specs and the open decisions, stated as current truth. Linear tracks scope, work and acceptance against them; this workspace owns Rust
implementation, operations and proof. Start with the [canonical map](book/src/reading.md).
The user's latest explicit decisions take precedence and land in the specs. Submodules are read-only: behavior that appears only in a
submodule or a generated wire schema is not a specification, and is not restored merely because it is there.

## Development workflow

Iterate locally: implement, exercise the real workflow, review, and correct
before handing off a coherent change through a PR. Investigate failures and
verify corrections before pushing another revision. An early draft can help
when collaboration or remote evidence is needed; it should not replace the
local feedback loop.

Keep investigation and correction proportional to credible consequences.
Distinguish serious rare failures from speculative possibilities, and weigh
the complexity a proposed correction would introduce. Stop when the intended
outcome and appropriate proof are complete. Further review rounds need a
concrete change or unresolved concern, not a search for increasingly speculative
edge cases or a desire to obtain a clean verdict.

Continue implementation, verification, review, and accepted corrections within
the agreed task. Resolve routine choices; bring genuinely open product,
architecture, or scope decisions to the user. A request only to discuss, draft,
or review stays within that scope. Reviewers keep source code read-only; the
implementing agent owns corrections. Findings are evidence, not new requirements.
Follow the latest task scope: a later request to review first or wait suspends
earlier implementation authority.

Use the developer's configured connections and repository setup. Preserve
unrelated work and report access or proof gaps honestly. External publication,
messages, merges, deployment, and destructive actions retain the user's approval
requirements. A PR review request authorizes the independent reviewer's own
assessment and follow-up publications through the reviewer GitHub App on the
selected PRs, without per-comment approval, as defined by `review-pr`. This
exception does not cover author responses, other messages, or merges. Local
review before a PR exists remains internal.

Report the actual revision or local change, evidence, and remaining limits;
successful checks or a merge alone do not establish the intended outcome.
Evidence is an artifact a reader can inspect, such as a command with its output or
a screenshot, tied to the revision it ran against. The
[verify skills](verification/README.md#verify-skills) name the proof for each
feature they cover. A physician-site change is not done unless
`cargo test -p physician-sample --test site -- --ignored` passed; that Chromium
test fails when the document is blank.

Local review is part of every change, with or without a ticket: after
implementing and verifying a coherent change, obtain independent review,
reconcile findings, and verify corrections before reporting it ready. No
separate review request is needed. Opening a PR does not by itself require a
second review; carry prior findings and evidence forward.

The engineering skills live in [`.agents/skills`](.agents/skills), and each
skill's description says when to use it. Use them as the work calls for them;
the user need not name each one. Skills supply methods and judgment within these
shared development defaults. `.claude/skills` links to them; resolve skill
references from their real repository paths.

## Where work goes

Work at the owner. Before adding anything, find what already owns it and extend
that, rather than building a parallel one beside it. What more than one place
needs is shared, not copied. Add something new only when nothing existing fits,
and put it where the next agent would look for it. When something replaces an
old path, remove the old one. Stay on the task instead of chasing side problems.
The aim is a repository an agent can read cold, where everything has one obvious
home.

The [canonical map](book/src/reading.md) says where each area is documented, and
the [crate map](book/src/crate-map.md) lists every crate and what it owns.

## Documentation

Repository documentation, including the intent specs, states current truth and
direction, so an agent reading it cold understands what is true now and where
things are going. It reads as a description, never a record: no dated history,
change logs, evidence or decision trails, audit notes, or wording like
"previously" or "we changed X to Y." When something changes, edit the text so it
says what's true now; git history keeps how it got there. Say what something is
for and what must hold, and leave choices that belong to its owner open instead
of prescribing them.

Keep individual Linear ticket IDs, links and status mappings out of repository
documentation. Linear owns that tracking; docs use source and document links to
explain responsibilities, implemented behavior and operating procedures. Live
environment state, rollout notes, prices and implementation gaps stay out too;
they go in the PR, the ticket or the system that holds them.

## Product boundaries

Duet and the physician product are two products joined by
[connected care](intent/connected-care.md). Each keeps its own clinical types,
accounts and stored state, and data crosses only through the connection. They
share code only through the [shared crates](book/src/crate-map.md#shared), and the
[boundaries](crates/boundaries/src/lib.rs) tests keep either product from
importing the other's crates.
[The harness spec](intent/harness.md) says what the shared harness owns.
[Operating invariants](book/src/agent-contracts.md) keeps the common authority,
clinical-intelligence and recovery rules.

`eval/` inside `submodules/coda-agents` is sealed. Do not read it or place it in prompts, fixtures, or model-visible context.

## Persistence: ORM first

- New or changed ordinary CRUD uses the owning product's ORM entities and typed
  query APIs, not hand-authored SQL. `physician-auth` and `front-door` already use
  SeaORM 2 / SeaQuery; extend their existing owners. Use migration builders for
  schema changes where supported, preserving recorded migration history.
- Raw SQL is an exception for database-specific functionality unsupported by those
  APIs or a demonstrated performance need. Document the reason at the call site
  and test the behavior against Postgres. Bind all runtime values; never
  interpolate user values. Keep dynamic identifiers code-owned and allowlisted.
- Duet's schema/account transactions and other adapters use SQLx; its persistence
  owners are in [the Duet backend](book/src/backend-architecture.md#persistence-and-migrations).
  Do not import physician entities, combine product stores, or mass-convert
  persistence merely to comply with this preference.
- SQL formatting checks remain for legitimate authored-SQL exceptions; formatter
  coverage is not a reason to introduce or retain raw SQL. When an ORM conversion
  removes a query, update the formatter's required-file inventory and its tests
  together; never keep dummy SQL or report an empty scan as meaningful coverage.

## Dev shell

`nix develop` in this repository is the shell for the monorepo. It carries the Rust toolchain, the product tools, Doppler, and `act`.

For unattended work, use `nix develop .#agent --command <command>` for builds, tests and Git commits. This shares the default toolchain without Cursor or automatic Doppler credential loading. The execution environment supplies Nix; the repository setup hook only materializes the shell.

Doppler project `coda-rust` has three configs, `dev`, `stg` and `prd`. Log in once with `doppler login`, then prefix a local command with `doppler run --`. The shell loads `dev` when a login exists and continues when it does not. Production Rust processes that read secrets start with `doppler run --project coda-rust --config prd --` and still read environment variables. The public marketing site reads no secrets, so it runs without Doppler. Tests and the sample server do not need a Doppler account. Creating `prd` is an operator login step; this repo stores no token and no secret value.

Doppler holds every secret value. GCP Secret Manager holds only `DOPPLER_TOKEN`, a read-only service token scoped to one config: Cloud Run mounts it, and the process starts under `doppler run --`, which loads the rest. GitHub release jobs read the same token from an environment secret. Do not copy Doppler values into Secret Manager. `cargo run -p workspace-tools -- install-doppler OUTPUT_PATH` installs the pinned, checksum-verified CLI for images and runners.

## Infrastructure and CI

`infra/gcp` holds the OpenTofu roots; [GCP deployment](book/src/gcp.md) says which ones this workspace applies. Apply a root only from a reviewed saved plan with the operator's authorization; production applies and DNS changes need the user's explicit authorization. Applying a workspace root grants no production authority over the physician, Duet, clinical-evidence or Gym deployments.

Choose checks by affected contracts and dependencies; use hosted CI for shared
integration and environment confirmation. Report missing local capabilities
and remaining proof honestly.

CI uses [ci_scope.py](scripts/ci_scope.py) to select lightweight documentation and
engineering-skill checks or the full workspace suite. Runtime clinical `skills/`
remain product inputs. The `CI result` check requires every selected job to pass;
skills/docs-only main pushes do not deploy. Browser qualification remains explicit:
run ignored browser tests locally or dispatch CI manually for full qualification.
See [local CI checks](README.md#local-ci-checks) and [CI](book/src/release.md#continuous-integration).

`act` runs [`.github/workflows/ci.yml`](.github/workflows/ci.yml) with the host Docker daemon. `actionlint` checks that file. The workflow does not call Doppler.
