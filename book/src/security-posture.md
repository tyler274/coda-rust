# Security boundaries

This is an implementation map, not a security certification or a release gate
readback. Account, privacy and hosted readiness require evidence from the
complete affected journeys.

- Duet identity and authentication live in [`identity.rs`](/crates/duet-api/src/identity.rs). Inspect the configured store and route admission in [`main.rs`](/crates/duet-api/src/main.rs) and [`durable.rs`](/crates/duet-api/src/durable.rs); do not equate an available identity type with a qualified provider sign-in journey.
- Physician identity, practice membership and panel access have a separate [`physician-auth`](/crates/physician-auth/src/lib.rs) owner. [Physician site](physician-site.md#physician-sign-in-and-practice-access) describes its access boundaries.
- [Practice connection](front-door.md) maps consent and invitation implementation. The existing withdrawal toggle does not define the current fixed-consent contract.
- Runtime database roles and migration identities are separate. [Backend migrations](backend-architecture.md#persistence-and-migrations) and [GCP](gcp.md) identify their owners and configuration checks.
- [Privacy recovery](privacy-recovery.md) owns receipts, deletion and replay. [Observability](observability.md) owns telemetry content handling. Neither is a substitute for access control.
- Native device/session storage must be checked through the installed client and its platform bindings.

The hosted staging profile, `/readyz`, an IAP redirect, and a passing local test
each prove only their boundary. Real authenticated member/nonmember access,
sign-in, account isolation and privacy recovery need their own observed journeys.
