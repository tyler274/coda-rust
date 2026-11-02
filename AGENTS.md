# Coda Rust operating contract

This repository is the Rust workspace for Duet and the physician client. The [intent specs](intent/README.md) own product intent: the principles, the
product specs and the open decisions, stated as current truth. Linear tracks scope, work and acceptance against them; this workspace owns Rust
implementation, operations and proof. Start with the [canonical map](book/src/reading.md).
The user's latest explicit decisions take precedence and land in the specs; pinned submodules are historical
reference, not a competing specification.

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

Use the relevant skills as the work calls for them; the user need not name
each one. The links below identify when to use each skill. Skills supply
methods and judgment within these shared development defaults.

- [write-linear-ticket](.agents/skills/write-linear-ticket/SKILL.md) writes and
  refines tickets, checks their framing, and prepares a useful handoff.
- [implement-ticket](.agents/skills/implement-ticket/SKILL.md) carries an agreed
  ticket through implementation, verification, local review, and PR preparation.
- [local-review](.agents/skills/local-review/SKILL.md) is part of local development,
  including work started without a ticket. After implementing and verifying a
  coherent change, obtain independent review, reconcile findings, and verify
  corrections before reporting it ready. No separate review request is needed.
- [capture-proof](.agents/skills/capture-proof/SKILL.md) records each acceptance
  criterion's evidence against its revision, checks it for secrets, publishes it,
  and renders the PR's Proof section.
- [write-pr](.agents/skills/write-pr/SKILL.md) prepares the PR explanation and
  refreshes it after material changes. [review-pr](.agents/skills/review-pr/SKILL.md)
  assesses existing PRs and handles review follow-ups. Carry prior findings and
  evidence forward; opening a PR does not by itself require a second review.
- [improve-review-pr](.agents/skills/improve-review-pr/SKILL.md) helps turn observed
  review failures and user feedback into improvements at the responsible owner.
- [coda-engineering-log](.agents/skills/coda-engineering-log/SKILL.md) drafts a
  requested daily update from the developer's available work and verified status.

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

`.agents/skills` owns the repository skills; `.claude/skills` links to them.
Resolve skill references from their real repository paths when using local
symlinks. Report the actual revision or local change, evidence, and remaining
limits; successful checks or a merge alone do not establish the intended outcome.
Evidence is an artifact a reader can inspect, such as a command with its output or
a screenshot, tied to the revision it ran against. The
[verify skills](verification/README.md#verify-skills) name the proof for each
feature they cover.

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

## Where work goes

| Work | Contract |
| --- | --- |
| Rust crates, the shared UI tokens, the physician Dioxus client, the sample practice server, and the skill loader | This file, then the crate you are changing |
| Public marketing site, its Rust server, and deployment artifacts | [Public site](book/src/public-site.md), then `crates/public-site`; `submodules/kytos` is optional read-only developer reference; builds/tests/CI use committed source snapshots, not private-repo access; staging owns `staging.codahealth.ai`; production serves the same tested image without sign-in from the same root, and its applies, DNS changes and Vercel retirement need the user's explicit authorization |
| Physician product, clinical runtime, and the clinical skill catalog | [Physician spec](intent/physician.md), [Rust implementation](book/src/physician-site.md), and [operating invariants](book/src/agent-contracts.md) |
| Clinical skill markdown | [skills](skills), the same catalog as coda-agents', read by `crates/agent-skills` for `crates/harness`; hosted physician chat (`physician-worker`) loads it at startup |
| Engineering workflow | [Development workflow](#development-workflow) routes shaping, implementation, local review, and PR work through the shared skills. [repo-graph-audit](.agents/skills/repo-graph-audit/SKILL.md) helps when repository ownership or handoffs need investigation. |
| Product-flow design and verification workflows | [Coda verification](verification/README.md); discuss intended flows with the user before formalizing them, and keep observed implementation separate |
| Clinical-corpus operations | [coda-agents skill](submodules/coda-agents/.agents/skills/coda-gcp-clinical-corpus-operations/SKILL.md) |
| Duet, the patient app | [Duet spec](intent/duet.md), [Rust backend](book/src/backend-architecture.md), [UI map](book/src/ui-surface-map.md), and [operating invariants](book/src/agent-contracts.md#duet-patient-app) |
| Gym cases, promotion, and runs | [Gym invariants](book/src/agent-contracts.md#coda-gym-evaluation), [coda-gym AGENTS.md](submodules/coda-gym/AGENTS.md) and the skills under [coda-gym/.claude/skills](submodules/coda-gym/.claude/skills) |
| Computer use | [Computer-use invariants](book/src/agent-contracts.md#computer-use) and [Coda-Computer-Use AGENTS.md](submodules/Coda-Computer-Use/AGENTS.md) |
| Synthetic patients: the patient bank tool and the patient set | [Synthetic patients](book/src/patient-bank.md), then [the patient-bank README](crates/patient-bank/README.md) |
| Dev container setup, ports, and storage | [Dev container](README.md#dev-container) |

## Product operating boundaries

[Operating invariants](book/src/agent-contracts.md) keeps the common authority,
clinical-intelligence and recovery rules. Product behavior lives in the intent
specs, and Linear tracks the work against them; the book maps them to actual
Rust owners.
Submodules stay read-only reference. Do not restore retired behavior merely
because it appears in an older contract or generated wire schema.

`eval/` inside coda-agents is sealed. Do not read it or place it in prompts, fixtures, or model-visible context.

Duet and the physician product do not share clinical types, accounts, servers, or durable application state. Shared crates are `ui`, `http`, `gmail`, `spruce`, `pubsub`, `telemetry`, and `harness`. `gmail` sends plain-text or HTML messages through the Gmail API or Workspace SMTP relay, and owns the push-wake and `history.list` cursor primitives for intake; it does not know either product. `spruce` is a read-only Spruce REST reader and change cursor; products own the cursor store, the effects, and patient linking. `pubsub` decodes push envelopes, checks loopback emulators and builds the official publisher; its `test-util` emulator is for `[dev-dependencies]` only. `harness` is product-neutral execution mechanics: it must not depend on either product or keep a second run ledger, and Duet must not import `local-product`. Each product keeps instructions, tool authority, clinical state, effects, and recovery.

Duet schema changes go in `crates/duet-api/migrations` and run through `duet-migrate` (`DUET_MIGRATION_DATABASE_URL`, `DUET_RUNTIME_ROLE`), not on API startup. Do not replay the coda-app ledger. Additive wire types go in `crates/duet-api/openapi/additions.json`; regenerate with `cargo run -p workspace-tools -- generate-duet-wire` and do not edit generated DTOs.

The physician site is a client of the physician projections. `crates/physician-sample` serves the page and a recorded projection of the synthetic practice on one origin. Unset `CODA_LOCAL_MODEL_EXECUTION` is `live`: physician chat runs through the Codex CLI subscription (`codex login`) and does not read an API key. `record` writes a cassette and `replay` answers from that cassette, also without a key. `api` answers physician chat through `crates/local-product` and OpenAI Responses; the key comes from the environment (`CODA_ALLOW_MODEL_CALLS=1` and `OPENAI_API_KEY`, loaded by Doppler in the nix shell) and is not stored in the page. The page mounts into `#main`. On the web build, an unset `PHYSICIAN_API_URL` uses that origin; `?api=` still points the same build at the Node server. A physician-site change is not done unless `cargo test -p physician-sample --test site -- --ignored` passed. That Chromium test fails when the document is blank.

The live Node projection server moves into `physician-sample` route family by route family. Duet stays in `duet-api` and `duet-app`. Gym and computer use get their own crates after those, and neither imports the other's clinical types.

## Persistence: ORM first

- New or changed ordinary CRUD uses the owning product's ORM entities and typed
  query APIs, not hand-authored SQL. `physician-auth` and `front-door` already use
  SeaORM 2 / SeaQuery; extend their existing owners. Use migration builders for
  schema changes where supported, preserving recorded migration history.
- Raw SQL is an exception for database-specific functionality unsupported by those
  APIs or a demonstrated performance need. Document the reason at the call site
  and test the behavior against Postgres. Bind all runtime values; never
  interpolate user values. Keep dynamic identifiers code-owned and allowlisted.
- Duet's `PgMoments` uses its own SeaORM entity over the existing SQLx pool.
  Preserve user-scoped list/delete, descending `(occurred_at, id)` order,
  unknown-kind errors, affected-row semantics and the privacy-journal deletion
  gate. Prove changes on disposable Postgres, including cross-account refusal.
- Duet's schema/account-transaction and other adapters still use SQLx. The
  remaining `src/sql/has_migration.sql` is a schema-readiness query; it is not a
  CRUD pattern. Keep migrations with `duet-migrate` and preserve the separate
  reference `schema_migrations` ledger. Do not import physician entities,
  combine product stores, or mass-convert persistence merely to comply with
  this preference.
- SQL formatting checks remain for legitimate authored-SQL exceptions; formatter
  coverage is not a reason to introduce or retain raw SQL. When an ORM conversion
  removes a query, update the formatter's required-file inventory and its tests
  together; never keep dummy SQL or report an empty scan as meaningful coverage.

## Dev shell

`nix develop` in this repository is the shell for the monorepo. It carries the Rust toolchain, the product tools, Doppler, and `act`.

For unattended work, use `nix develop .#agent --command <command>` for builds, tests and Git commits. This shares the default toolchain without Cursor or automatic Doppler credential loading. The execution environment supplies Nix; the repository setup hook only materializes the shell.

Doppler project `coda-rust` has three configs, `dev`, `stg` and `prd`. Log in once with `doppler login`, then prefix a local command with `doppler run --`. The shell loads `dev` when a login exists and continues when it does not. Production Rust processes that read secrets start with `doppler run --project coda-rust --config prd --` and still read environment variables. The public marketing site reads no secrets, so it runs without Doppler. Tests and the sample server do not need a Doppler account. Creating `prd` is an operator login step; this repo stores no token and no secret value.

Doppler holds every secret value. GCP Secret Manager holds only `DOPPLER_TOKEN`, a read-only service token scoped to one config: Cloud Run mounts it, and the process starts under `doppler run --`, which loads the rest. GitHub release jobs read the same token from an environment secret. Do not copy Doppler values into Secret Manager. `cargo run -p workspace-tools -- install-doppler OUTPUT_PATH` installs the pinned, checksum-verified CLI for images and runners.

`infra/gcp` holds the OpenTofu roots; [GCP deployment](book/src/gcp.md) lists each one and who applies it. This workspace owns the staging roots `public-site` and `rust-environments`, the mail-only production root `shared-mail`, and public-site production (the marketing site only). Apply each only from a reviewed saved plan with the operator's authorization. The other roots are references whose apply authority stays in the product repositories. These production exceptions grant no production authority over physician, Duet, clinical evidence or Gym application deployments.

`act` runs [`.github/workflows/ci.yml`](.github/workflows/ci.yml) with the host Docker daemon. `actionlint` checks that file. The workflow does not call Doppler.

Choose checks by affected contracts and dependencies; use hosted CI for shared
integration and environment confirmation. Report missing local capabilities
and remaining proof honestly.

CI uses [ci_scope.py](scripts/ci_scope.py) to select lightweight documentation and
engineering-skill checks or the full workspace suite. Runtime clinical `skills/`
remain product inputs. The `CI result` check requires every selected job to pass;
skills/docs-only main pushes do not deploy. Browser qualification remains explicit:
run ignored browser tests locally or dispatch CI manually for full qualification.
See [local CI checks](README.md#local-ci-checks).

CI runs on Blacksmith runners, which boot GitHub's Ubuntu 24.04 image and serve `actions/cache` from Blacksmith's own cache. The `ci` job uses `blacksmith-4vcpu-ubuntu-2404` and `cargo-deny` uses `blacksmith-2vcpu-ubuntu-2404`. `.actrc` maps both labels to the `act` image, and [`.github/actionlint.yaml`](.github/actionlint.yaml) declares them. To roll back, set `runs-on` to `ubuntu-latest`.
