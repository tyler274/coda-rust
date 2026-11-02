# Duet backend

[`duet-api`](/crates/duet-api/src/lib.rs) owns Duet's Rust service. It has identity,
evidence, conversation execution, saved work and privacy owners beyond the
original moments slice. A module's presence does not establish the complete
real-service journey.

## Ownership

| Responsibility | Source owner |
| --- | --- |
| Startup, configured stores and hosted profile | [`main.rs`](/crates/duet-api/src/main.rs), [`hosting.rs`](/crates/duet-api/src/hosting.rs) |
| Duet accounts and authentication | [`identity.rs`](/crates/duet-api/src/identity.rs), [`session.rs`](/crates/duet-api/src/session.rs); [security boundaries](security-posture.md) |
| Evidence identity, ingestion authority, exact reads and paging | [`evidence.rs`](/crates/duet-api/src/evidence.rs), [`evidence_entity.rs`](/crates/duet-api/src/evidence_entity.rs), [`evidence_routes.rs`](/crates/duet-api/src/evidence_routes.rs), [`evidence_paging.rs`](/crates/duet-api/src/evidence_paging.rs) |
| Mounted conversation acceptance and execution | [`conversation.rs`](/crates/duet-api/src/conversation.rs), [`conversation_routes.rs`](/crates/duet-api/src/conversation_routes.rs) |
| Mounted conversation instructions, context and admitted skills | [`conversation.rs`](/crates/duet-api/src/conversation.rs), [`conversation_context.rs`](/crates/duet-api/src/conversation_context.rs), [`skills.rs`](/crates/duet-api/src/skills.rs) |
| Saved care, follow-up and Today | [`care.rs`](/crates/duet-api/src/care.rs), [`follow_up.rs`](/crates/duet-api/src/follow_up.rs), [`today.rs`](/crates/duet-api/src/today.rs), [`producer_database.rs`](/crates/duet-api/src/producer_database.rs) |
| Doctor preparation and publication implementation | [`workspace.rs`](/crates/duet-api/src/workspace.rs), [`doctor_routes.rs`](/crates/duet-api/src/doctor_routes.rs), [`publication.rs`](/crates/duet-api/src/publication.rs); preparation, review and saving are distinct from later publication to physicians outside Coda |
| Privacy receipts, replay and deletion archive | [Privacy recovery](privacy-recovery.md) |
| Schema and moments persistence | [`database.rs`](/crates/duet-api/src/database.rs), [`migrations/`](/crates/duet-api/migrations) |
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

The patient-owned doctor workspace and explicit sending to a connected practice
are MVP scope; [Accounts and practice connection](front-door.md#implementation)
owns that implementation boundary. External public-page and link sharing is
parked for post-MVP. The current binary still mounts the publication routes in
`doctor_routes`, including HTML, PDF and SMART Health Link retrieval. Retaining
those routes neither qualifies external sharing for MVP nor restores PDF download
to product scope. Parked product scope does not disable a route or resolve its
security findings.

Duet uses the product-neutral [`harness`](/crates/harness/src/lib.rs). It retains
its own context policy, authority, durable state, effects and recovery. It must
not import the physician `local-product`; [operating invariants](agent-contracts.md)
define this boundary.

The shared model → tool → model orchestration lives in `harness::engine`.
The mounted conversation owner uses `harness::execution` as a compatibility
facade, preserving saved `Request`, `Response`, `Record` and replay identities.
Restored call/token counters belong to the accepted command, not a fresh budget
on recovery. Recovery reconciles retained tool calls without another provider
exchange; it does not start a second loop. Provisional text and late provider
usage remain separate from permission to commit. Duet still owns routine
idempotency, visible-prose finalization and its completion transaction; physician
terminal-tool continuation does not impose physician saving policy on Duet.
Cancellation allows a bounded one-second transport settlement window, followed
by at most one additional second for final evidence acknowledgments. The latter
window is shared by the final records, not restarted for each write; it permits
asynchronous database retention after a noncooperative transport times out.
Neither window permits continued presentation or effects. A callback that does
not acknowledge within its window leaves its write uncertain; the product
must reconcile it rather than treating cancellation as rollback. Recovery
validates retained call IDs against the original dispatched context before
executing any sibling, and partial usage snapshots preserve earlier known
fields so live and restored token accounting agree.

The binary mounts `conversation_routes` over `ConversationStore` and starts its
pending-work recovery. The separate
[`conversation_execution.rs`](/crates/duet-api/src/conversation_execution.rs)
front-door-authorized path and its [`luna.rs`](/crates/duet-api/src/luna.rs)
instruction composer are not wired into that startup. Their presence is not
evidence of a second mounted conversation service.

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

The generator consumes the pinned OpenAPI source. Its DTOs are compatibility
artifacts, not a promise of person-to-person sharing. Retiring a wire surface
requires checking its actual callers and services.

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
`PgMoments` uses a Duet-owned SeaORM entity over the SQLx pool. Schema/account
transactions and other adapters use SQLx. Ordinary CRUD belongs to the owning
product's typed entities; products do not share stores.

## Proof

[Release](release.md) and [GCP](gcp.md) explain the build and hosted proof
boundaries. Default tests may skip database-gated checks. The
[`check-postgres.sh`](/scripts/check-postgres.sh) suites require a disposable
PostgreSQL instance with pgvector and all five script URLs
(`MIGRATION_DATABASE_URL`, `PHYSICIAN_DATABASE_URL`, `FRONT_DOOR_DATABASE_URL`,
`DUET_IDENTITY_TEST_DATABASE_URL`, `PHYSICIAN_RUNTIME_TEST_DATABASE_URL`).
They create/drop databases. The shared-loop regression gate also runs the real
Duet owner suites `conversation_proof`, `provisional_stream`, `skills_accounting`,
`fresh_producers`, `today_producer` and `follow_up_producer` against
`DUET_TEST_DATABASE_URL`; the unmounted adapter proof is not a substitute.
Loop consolidation preserves the current admitted capabilities, not completion
of the saved-routine journey or full D1 acceptance. These suites and image smoke tests establish their
exercised persistence/startup paths, not sign-in, installed-client or clinical
qualification.
