# Security boundaries

This is an implementation map, not a security certification or a release gate
readback. [ENG-161](https://linear.app/coda-health/issue/ENG-161) owns Duet account
and privacy acceptance; [ENG-167](https://linear.app/coda-health/issue/ENG-167)
and the [platform project](https://linear.app/coda-health/project/coda-rust-platform-6ca86460e169)
own hosted and real-patient readiness.

- Duet identity and authentication live in [`identity.rs`](/crates/duet-api/src/identity.rs) and [`session.rs`](/crates/duet-api/src/session.rs). Inspect the configured store and route admission in [`main.rs`](/crates/duet-api/src/main.rs) and [`lib.rs`](/crates/duet-api/src/lib.rs); do not equate an available identity type with a qualified provider sign-in journey.
- Physician identity, practice membership and panel access have a separate [`physician-auth`](/crates/physician-auth/src/lib.rs) owner. [Physician site](physician-site.md#physician-sign-in-and-practice-access) describes its access boundaries.
- [Practice connection](front-door.md) maps consent and invitation implementation to ENG-70. The existing withdrawal toggle does not define the current fixed-consent contract.
- Runtime database roles and migration identities are separate. [Backend migrations](backend-architecture.md#persistence-and-migrations) and [GCP](gcp.md) identify their owners and configuration checks.
- [Privacy recovery](privacy-recovery.md) owns receipts, deletion and replay. [Observability](observability.md) owns telemetry content handling. Neither is a substitute for access control.
- Native device/session storage must be checked through the installed client and its platform bindings. Historical SwiftUI security findings in the [original Duet docs](https://github.com/Coda-Health/coda-app/blob/5a1dc47b8feacb8db3fcc71522982f76822c0b01/docs/security-posture.md) are source evidence, not proof of either a Rust vulnerability or a completed Rust fix.

The hosted staging profile, `/readyz`, an IAP redirect, and a passing local test
each prove only their boundary. Real authenticated member/nonmember access,
sign-in, account isolation and privacy recovery need their own observed journeys.
