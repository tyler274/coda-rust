# Privacy and recovery

[ENG-161](https://linear.app/coda-health/issue/ENG-161) owns export, deletion and
session acceptance. Rust has its own privacy receipt and replay implementation;
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
connection, whose current semantics live in [ENG-70](https://linear.app/coda-health/issue/ENG-70)
and [accounts and practice connection](front-door.md).

[ENG-98](https://linear.app/coda-health/issue/ENG-98) owns real-service proof.
[Security boundaries](security-posture.md) maps identity and isolation;
[observability](observability.md) maps diagnostic content handling.
