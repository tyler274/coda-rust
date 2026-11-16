# Duet backend

[`duet-api`](/crates/duet-api/src/lib.rs) owns Duet's Rust service. It has identity,
evidence, doctor workspace, health-context and privacy owners. A module's presence
does not establish the complete real-service journey.

## Ownership

| Responsibility | Source owner |
| --- | --- |
| Startup, configured stores and hosted profile | [`main.rs`](/crates/duet-api/src/main.rs), [`hosting.rs`](/crates/duet-api/src/hosting.rs) |
| Duet accounts and authentication | [`identity.rs`](/crates/duet-api/src/identity.rs), [`session.rs`](/crates/duet-api/src/session.rs); [security boundaries](security-posture.md) |
| Evidence identity, ingestion authority, exact reads and paging | [`evidence.rs`](/crates/duet-api/src/evidence.rs), [`evidence_entity.rs`](/crates/duet-api/src/evidence_entity.rs), [`evidence_routes.rs`](/crates/duet-api/src/evidence_routes.rs), [`evidence_paging.rs`](/crates/duet-api/src/evidence_paging.rs) |
| Health context and producer grants | [`health_context.rs`](/crates/duet-api/src/health_context.rs), [`producer_routes.rs`](/crates/duet-api/src/producer_routes.rs), [`producer_database.rs`](/crates/duet-api/src/producer_database.rs) |
| Doctor preparation and publication implementation | [`workspace.rs`](/crates/duet-api/src/workspace.rs), [`doctor_routes.rs`](/crates/duet-api/src/doctor_routes.rs), [`publication.rs`](/crates/duet-api/src/publication.rs); preparation, review and saving are distinct from later publication to physicians outside Coda |
| Privacy receipts, replay and deletion archive | [Privacy recovery](privacy-recovery.md) |
| Schema and migrations | [`database.rs`](/crates/duet-api/src/database.rs), [`migrations/`](/crates/duet-api/migrations) |
| Practice invitations and connection adapter | [Accounts and practice connection](front-door.md) |

## Canonical evidence ingestion

`EvidenceStore` is the single mutation owner for canonical health evidence and
clinical notes. It applies single and batch mutations under an account-locked
transaction. The HTTP routes `POST /api/canonical-evidence` and
`POST /api/canonical-evidence/batch` always write with device authority. Device
authority may not create, correct, revive or remove the originals of the
server-owned portal sources (`epic`, `oracleHealth`, `healow`, `googleHealth`).
Those originals can be written only in process, through
`ingest_with_authority` and a matching connector source kind and connection.
Such refusals return the stable conflict code `source_authority_refused`. A
record's source kind, connection and note label are fixed when it is created.

- A batch holds 1–100 items, each a bounded source object up to 2 MB. Each item
  runs in its own savepoint and gets an `accepted` or typed `rejected` outcome,
  so no item is skipped silently. An infrastructure failure aborts the whole
  batch.
- Retrying an operation returns its first acknowledged address. A new operation
  identity must name the exact current revision.
- `measured_at` or `recorded_at` is stored only from an RFC 3339 instant the
  source declared. Otherwise the stored date is unknown (NULL); receipt time is
  never used, and the literal stays in the payload. Patient-authored notes
  (`Added in Duet`) must carry an explicit instant.
- `operation: "delete"` is an explicit removal by the owning source, fenced by
  exact revision and source generation. It keeps the base row and retained
  revisions, marks membership `deleted` and `source_removed`, and leaves earlier
  revisions readable as non-current citations. Privacy erasure
  ([privacy recovery](privacy-recovery.md)) deletes base rows and their
  retained revisions, and remains authoritative.

The <a href="/intent/duet%2Emd">Duet spec</a> (D4) owns For my doctor
and what it sends to a connected practice. The binary still mounts the publication
routes in `doctor_routes`, including HTML, PDF and SMART Health Link retrieval. A
mounted route doesn't put external sharing or PDF download in scope, and it keeps
its security findings until the route is removed or fixed.

Duet must not import the physician `local-product`; the
[boundaries](/crates/boundaries/src/lib.rs) tests define this boundary.

## API and clients

[`duet-client`](/crates/duet-client/src/lib.rs) adds Duet client behavior over
[`duet-wire`](/crates/duet-wire/src/lib.rs). Generate wire types with
`cargo run -p workspace-tools -- generate-duet-wire`; additive Rust operations
live in [`openapi/additions.json`](/crates/duet-api/openapi/additions.json).
[`workspace-tools::wire`](/crates/workspace-tools/src/wire.rs) orchestrates the
[`composer`](/scripts/duet-wire/compose.py), OpenAPI Generator and
[`finalizer`](/scripts/duet-wire/finalize.py); their compatibility templates live
beside those scripts. The <a href="/crates/duet-wire/README%2Emd">generated API index</a>
and model/operation pages are regenerated with the Rust sources.
`cargo run -p workspace-tools -- generate-duet-wire --check` compares sources,
the manifest and documentation against fresh output. Use the pinned toolchain
in `nix develop`.

The generator consumes the pinned OpenAPI source. The composer excludes the
retired `/api/sharing/` person-to-person routes and schemas used only by them.
Practice-connection and doctor-workspace contracts keep their own owners.
Regeneration clears generated source and documentation directories so removed
operations cannot survive as stale files.

## Persistence and migrations

The API never owns schema migration at startup. Run
[`duet-migrate`](/crates/duet-api/src/bin/duet-migrate.rs) with
`DUET_MIGRATION_DATABASE_URL` and `DUET_RUNTIME_ROLE`. The migration role and
pre-existing runtime login must be distinct; the runtime role must not be a
superuser, bypass RLS, create roles/databases, or inherit the migration role.
The binary applies the Duet-owned chain first and optionally the front-door
chain when `FRONT_DOOR_DATABASE_URL` is supplied. Do not replay the coda-app
migration ledger to initialize this service.

Queries go through the typed database API, as <a href="/AGENTS%2Emd#writing-code">AGENTS.md</a> asks.
Schema/account transactions use SQLx. `src/sql/has_migration.sql` is a
schema-readiness query, not a CRUD pattern, and `duet-migrate` keeps the separate
reference `schema_migrations` ledger. Ordinary CRUD belongs to the owning
product's typed entities; products do not share stores.

## Proof

[Release](release.md) and [GCP](gcp.md) explain the build and hosted proof
boundaries. Default tests may skip database-gated checks. The
[`check-postgres.sh`](/scripts/check-postgres.sh) suites require a disposable
PostgreSQL instance with pgvector and all five script URLs
(`MIGRATION_DATABASE_URL`, `PHYSICIAN_DATABASE_URL`, `FRONT_DOOR_DATABASE_URL`,
`DUET_IDENTITY_TEST_DATABASE_URL`, `PHYSICIAN_RUNTIME_TEST_DATABASE_URL`).
They create/drop databases. The Duet owner suites, such as `fresh_producers`,
`public_budget` and `evidence_cohort`, run against `DUET_TEST_DATABASE_URL`.
These suites and image smoke tests establish their
exercised persistence/startup paths, not sign-in, installed-client or clinical
qualification.
