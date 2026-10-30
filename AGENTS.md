# Coda Rust operating contract

This repository is the Rust workspace for Duet and the physician client. The [intent specs](intent/README.md) own product intent: the principles, the
product specs and the open decisions, stated as current truth. Linear tracks scope, work and acceptance against them; this workspace owns Rust
implementation, operations and proof. Start with the [canonical map](book/src/reading.md).
Dev's latest explicit decisions take precedence and land in the specs; pinned submodules are historical
reference, not a competing specification.

Keep individual Linear ticket IDs, links and status mappings out of repository
documentation. Linear owns that tracking; docs use source and document links to
explain responsibilities, implemented behavior and operating procedures.

## Where work goes

| Work | Contract |
| --- | --- |
| Rust crates, the shared UI tokens, the physician Dioxus client, the sample practice server, and the skill loader | This file, then the crate you are changing |
| Public marketing site, its Rust server, and deployment artifacts | [Public site](book/src/public-site.md), then `crates/public-site`; `submodules/kytos` is optional read-only developer reference; builds/tests/CI use committed source snapshots, not private-repo access; staging owns `staging.codahealth.ai`; production serves the same tested image without sign-in from the same root, and its applies, DNS changes and Vercel retirement need Dev's explicit authorization |
| Physician product, clinical runtime, and the clinical skill catalog | [Operating invariants](book/src/agent-contracts.md) and [coda-agents AGENTS.md](submodules/coda-agents/AGENTS.md) |
| Clinical skill markdown | [skills](skills), the same catalog as coda-agents', read by `crates/agent-skills` for `crates/harness`; hosted physician chat (`physician-worker`) loads it at startup |
| PR authoring, review and repository audits | [write-pr](.agents/skills/write-pr/SKILL.md) before creating or materially rewriting a PR; [review-pr](.agents/skills/review-pr/SKILL.md) for independent PR or coherent-batch assessment; [improve-review-pr](.agents/skills/improve-review-pr/SKILL.md) for requested retrospectives and guidance updates; [independent-review](.agents/skills/independent-review/SKILL.md) for local artifacts or combined outcomes; [repo-graph-audit](.agents/skills/repo-graph-audit/SKILL.md) for optional repository ownership and handoff audits. `.agents/skills` owns every repository skill; `.claude/skills` links to each one. Ordinary changes do not require a full graph audit. |
| Product-flow design and verification workflows | [Coda verification](verification/README.md), then the adopted engineering skills in [.agents/skills](.agents/skills); load [coda-statechart](.agents/skills/coda-statechart/SKILL.md) in every session before editing any journey model under `verification/`; discuss intended flows with Dev before formalizing them, and keep observed implementation separate |
| Clinical-corpus operations | [coda-agents skill](submodules/coda-agents/.agents/skills/coda-gcp-clinical-corpus-operations/SKILL.md) |
| Duet, the patient app | [Duet invariants](book/src/agent-contracts.md#duet-patient-app), [coda-app AGENTS.md](submodules/coda-app/AGENTS.md) and [Duet UI](submodules/coda-app/.agents/skills/duet-ui-ux/SKILL.md) |
| Gym cases, promotion, and runs | [Gym invariants](book/src/agent-contracts.md#coda-gym-evaluation), [coda-gym AGENTS.md](submodules/coda-gym/AGENTS.md) and the skills under [coda-gym/.claude/skills](submodules/coda-gym/.claude/skills) |
| Computer use | [Computer-use invariants](book/src/agent-contracts.md#computer-use) and [Coda-Computer-Use AGENTS.md](submodules/Coda-Computer-Use/AGENTS.md) |
| Synthetic patients: the patient bank tool and the patient set | [Synthetic patients](book/src/patient-bank.md), then [the patient-bank README](crates/patient-bank/README.md) |
| Dev container setup, ports, and storage | [Dev container](README.md#dev-container) |

[Operating invariants](book/src/agent-contracts.md) keeps the common authority,
clinical-intelligence and recovery rules. Product behavior lives in the intent
specs, and Linear tracks the work against them; the book maps them to actual
Rust owners and names remaining gaps.
Submodules stay read-only reference. Do not restore retired behavior merely
because it appears in an older contract or generated wire schema.

`eval/` inside coda-agents is sealed. Do not read it or place it in prompts, fixtures, or model-visible context.

Duet and the physician product do not share clinical types, accounts, servers, or durable application state. Shared crates are `ui`, `http`, `gmail`, `pubsub`, and `harness`. `gmail` sends plain-text or HTML messages through the Gmail API or Workspace SMTP relay, and owns the push-wake and `history.list` cursor primitives for intake; it does not know either product. `pubsub` decodes push envelopes, checks loopback emulators and builds the official publisher; its `test-util` emulator is for `[dev-dependencies]` only. `harness` is product-neutral execution mechanics: it must not depend on either product or keep a second run ledger, and Duet must not import `local-product`. Each product keeps instructions, tool authority, clinical state, effects, and recovery.

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

Routine CI runs unit, service, storage and HTTP checks across the workspace. Browser qualification is explicit: run the ignored browser tests locally, or dispatch the CI workflow manually. A green routine CI run does not establish browser qualification.

CI runs on Blacksmith runners, which boot GitHub's Ubuntu 24.04 image and serve `actions/cache` from Blacksmith's own cache. The `ci` job uses `blacksmith-4vcpu-ubuntu-2404` and `cargo-deny` uses `blacksmith-2vcpu-ubuntu-2404`. `.actrc` maps both labels to the `act` image, and [`.github/actionlint.yaml`](.github/actionlint.yaml) declares them. To roll back, set `runs-on` to `ubuntu-latest`.

React and SwiftUI remain the shipping clients until a journey matches them. Do not claim a production cutover, a TestFlight release, or a physician production deploy from this repository.
