# Privacy and recovery

Rust has its own privacy receipt and replay implementation;
the original Python service is historical reference, not its runtime owner.

| Responsibility | Rust owner |
| --- | --- |
| Receipt types, journal interface, local file journal and replay | [`privacy.rs`](/crates/duet-api/src/privacy.rs) |
| Independent hosted journal | [`journal_gcs.rs`](/crates/duet-api/src/journal_gcs.rs) |
| Archive behavior for privacy operations | [`privacy_archive.rs`](/crates/duet-api/src/privacy_archive.rs) |
| Persistence and deletion integration | [`database.rs`](/crates/duet-api/src/database.rs), [`lib.rs`](/crates/duet-api/src/lib.rs) |
| Configured hosted stores and startup checks | [`main.rs`](/crates/duet-api/src/main.rs); [GCP](gcp.md) |

Inspect these owners and their tests together when changing deletion or replay.
A journal write, database mutation and restore replay are distinct observations;
prove the complete affected path before claiming that deleted information cannot
return. File-backed local proof does not establish independent hosted-journal
recovery. Keep patient account deletion separate from ending a practice
connection; see [accounts and practice connection](front-door.md).

## Isolated restore qualification

A database restore and a privacy replay are separate gates. Keep the restored
Duet database disconnected from serving, subscriptions and outbound effects until
both have passed. The operator selects a successful backup and retains the
independent journal through the recovery cutoff; restoring the journal to the
SQL backup's timestamp would discard restrictions accepted after that backup.

[`duet-privacy-replay`](/crates/duet-api/src/bin/duet-privacy-replay.rs) with no
receipt arguments calls the product's full `reconcile_inventory` path. Passing
selected receipt IDs is targeted repair, not full restore qualification. Hosted
replay requires the independent GCS journal and its authorized attached identity.
An empty replacement bucket or a SQL-derived list cannot establish coverage of
the independent inventory. Establish its provenance and completeness separately.

A synthetic rehearsal must retain a backup containing an account and its data,
then record deletion or withdrawal in the independent journal. Restore the older
SQL into an isolated target and replay the newer inventory. Check the stored
restriction and refusal through the product owner, including repeat replay and
missing-journal refusal, before any claim that deleted data cannot return. The
physician database has its own schema, principals and readback checks; it does
not import Duet's privacy types.

[GCP recovery](gcp.md) owns disposable cloud targets and operator approval. Record
backup identity, target, replay evidence and validation results together. A
successful SQL restore operation, a local file-journal test or a ready process
alone does not qualify hosted recovery.

[Security boundaries](security-posture.md) maps identity and isolation;
[observability](observability.md) maps diagnostic content handling.
