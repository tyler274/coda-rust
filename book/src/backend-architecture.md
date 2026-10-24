# Duet backend

[`duet-api`](/crates/duet-api/src/lib.rs) owns Duet's Rust service. It has identity,
evidence, conversation execution, saved work and privacy owners beyond the
original moments slice. [ENG-161](https://linear.app/coda-health/issue/ENG-161),
[ENG-26](https://linear.app/coda-health/issue/ENG-26),
[ENG-162](https://linear.app/coda-health/issue/ENG-162) and
[ENG-34](https://linear.app/coda-health/issue/ENG-34) define acceptance. A module's
presence does not establish the complete real-service journey.

## Ownership

| Responsibility | Source owner |
| --- | --- |
| Startup, configured stores and hosted profile | [`main.rs`](/crates/duet-api/src/main.rs), [`hosting.rs`](/crates/duet-api/src/hosting.rs) |
| Duet accounts and authentication | [`identity.rs`](/crates/duet-api/src/identity.rs), [`session.rs`](/crates/duet-api/src/session.rs); [security boundaries](security-posture.md) |
| Evidence identity, exact reads and paging | [`evidence.rs`](/crates/duet-api/src/evidence.rs), [`evidence_routes.rs`](/crates/duet-api/src/evidence_routes.rs), [`evidence_paging.rs`](/crates/duet-api/src/evidence_paging.rs) |
| Accepted conversation and execution | [`conversation.rs`](/crates/duet-api/src/conversation.rs), [`conversation_execution.rs`](/crates/duet-api/src/conversation_execution.rs), [`conversation_routes.rs`](/crates/duet-api/src/conversation_routes.rs) |
| Duet instructions and admitted skills | [`luna.rs`](/crates/duet-api/src/luna.rs), [`skills.rs`](/crates/duet-api/src/skills.rs) |
| Saved care, follow-up and Today | [`care.rs`](/crates/duet-api/src/care.rs), [`follow_up.rs`](/crates/duet-api/src/follow_up.rs), [`today.rs`](/crates/duet-api/src/today.rs), [`producer_database.rs`](/crates/duet-api/src/producer_database.rs) |
| Doctor preparation and publication implementation | [`workspace.rs`](/crates/duet-api/src/workspace.rs), [`doctor_routes.rs`](/crates/duet-api/src/doctor_routes.rs), [`publication.rs`](/crates/duet-api/src/publication.rs); [ENG-169](https://linear.app/coda-health/issue/ENG-169) puts preparation/review/saving in MVP, with publication to physicians outside Coda after MVP |
| Privacy receipts, replay and deletion archive | [Privacy recovery](privacy-recovery.md) |
| Schema and moments persistence | [`database.rs`](/crates/duet-api/src/database.rs), [`migrations/`](/crates/duet-api/migrations) |
| Practice invitations and connection adapter | [Accounts and practice connection](front-door.md) |

Duet uses the product-neutral [`harness`](/crates/harness/src/lib.rs). It retains
its own context policy, authority, durable state, effects and recovery. It must
not import the physician `local-product`; [operating invariants](agent-contracts.md)
and [ENG-159](https://linear.app/coda-health/issue/ENG-159) define this boundary.

## API and clients

[`duet-client`](/crates/duet-client/src/lib.rs) adds Duet client behavior over
[`duet-wire`](/crates/duet-wire/src/lib.rs). Generate wire types with
`cargo run -p workspace-tools -- generate-duet-wire`; additive Rust operations
live in [`openapi/additions.json`](/crates/duet-api/openapi/additions.json).
The generator still consumes the pinned legacy OpenAPI source. Retained DTOs
are compatibility artifacts, not a promise to restore retired person-to-person
sharing. [ENG-141](https://linear.app/coda-health/issue/ENG-141) owns removal
through actual callers and services.

## Persistence and migrations

The API never owns schema migration at startup. Run
[`duet-migrate`](/crates/duet-api/src/bin/duet-migrate.rs) with
`DUET_MIGRATION_DATABASE_URL` and `DUET_RUNTIME_ROLE`. The migration role and
pre-existing runtime login must be distinct; the runtime role must not be a
superuser, bypass RLS, create roles/databases, or inherit the migration role.
The binary applies the Duet-owned chain first and optionally the front-door
chain when `FRONT_DOOR_DATABASE_URL` is supplied. Do not replay the coda-app
migration ledger to initialize this service.

Follow the ORM rules in <a href="/AGENTS%2Emd#persistence-orm-first">AGENTS.md</a>.
`PgMoments` already uses a Duet-owned SeaORM entity over the existing SQLx
pool. Schema/account transactions and other adapters still use SQLx; do not
confuse that remaining work with an unimplemented moments conversion. Ordinary
CRUD belongs to the owning product's typed entities; products do not share stores.

## Proof

[ENG-98](https://linear.app/coda-health/issue/ENG-98) owns real-service proof;
[release](release.md) and [GCP](gcp.md) explain the build and hosted boundaries.
Default tests may skip database-gated checks. The
[`check-postgres.sh`](/scripts/check-postgres.sh) suites require a disposable
PostgreSQL 16 instance with pgvector and all three test URLs
(`MIGRATION_DATABASE_URL`, `PHYSICIAN_DATABASE_URL`, `FRONT_DOOR_DATABASE_URL`).
They create/drop databases. These suites and image smoke tests establish their
exercised persistence/startup paths, not sign-in, installed-client or clinical
qualification. Historical dependency-upgrade proof remains in
[PR #36](https://github.com/Coda-Health/coda-rust/pull/36) and
[ENG-163](https://linear.app/coda-health/issue/ENG-163).
