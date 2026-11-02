# Accounts and practice connection

Duet owns standalone patient identity; the practice connection has a separate
invitation and consent boundary. A Duet account does not require an invitation.
An invitation attaches the practice to that account; existing accounts and data
are not migrated from the old app. The products keep separate data and authority.

## Current scope

The MVP allows independent connections to invited practices, fixed consent at join, sharing until
patient leave, practice discharge or account deletion, and re-invitation after
the connection ends. There is no separate stop/resume-sharing or re-consent
flow. Ending a connection preserves the Duet account and history; the last
received doctor's plan stays dated and is no longer updated. Ending one connection
leaves the others unaffected. Patient-facing clinician lookup is later work.

The connection contract covers consented evidence reaching the physician and
documented care instructions reaching Duet with provenance. Physicians do
not approve Duet's wording or manage the patient app. The patient-owned doctor
workspace—preparation, updates, review and saving—is in MVP. Sharing with physicians
outside Coda follows after MVP, using browser review with no PDF download.
The jointly maintained live shared space is also post-MVP; it is separate from
both the patient-owned workspace and the MVP practice connection. There is no
paid tier in the MVP; the
[Connected care project](https://linear.app/coda-health/project/connected-care-v1-9f745b9c6874)
owns scope and cadence decisions.

## Implementation

The patient-owned workspace remains in MVP, including explicitly sending a saved
version to a connected practice; saving alone shares nothing. External public-page
and link sharing is parked for post-MVP. See <a href="/intent/duet%2Emd">Duet intent</a> (D4)
and <a href="/intent/connected-care%2Emd">Connected care intent</a> (C1.6)
for the governing behavior.

| Rust owner | Implemented responsibility and boundary |
| --- | --- |
| [`duet-api::identity`](/crates/duet-api/src/identity.rs) | Duet identity store and authentication; standalone sign-up/session acceptance requires the whole journey |
| [`front-door`](/crates/front-door/src/lib.rs) | Existing account, invitation, connection, consent and premium adapter, with SeaORM/Postgres persistence; its account model is not a decision to make standalone Duet depend on practice admission |
| [`physician-sample::auth`](/crates/physician-sample/src/auth.rs) | Practice-authorized invitations and linked-patient reads; cross-practice access stays refused |
| [`duet-api`](/crates/duet-api/src/lib.rs) and [`duet-client`](/crates/duet-client/src/lib.rs) | Join, consent and premium router adapters and client calls; the Duet binary does not mount the adapters |
| [`duet-app::account`](/crates/duet-app/src/account.rs), [`duet-app`](/crates/duet-app/src/lib.rs) | Standalone account root owns private access. Join code and the sharing toggle are not an account prerequisite and do not define fixed consent |

The adapter's consent withdrawal/resume and paid-premium fields are not approved
MVP behavior.

[`duet-api::main`](/crates/duet-api/src/main.rs) mounts Duet-owned identity and
durable routes. Its optional front-door store is used by readiness; the separate
[`duet-privacy-delivery`](/crates/duet-api/src/bin/duet-privacy-delivery.rs)
operator also uses front-door journals. The physician auth
router mounts invitation and linked-patient operations.

`front-door-migrate` owns the adapter's schema; see
[Duet migrations](backend-architecture.md#persistence-and-migrations) and
[GCP migration jobs](gcp.md). The existing
[`front_door_postgres.rs`](/crates/physician-sample/tests/front_door_postgres.rs)
proves its invitation and link path. The [Connected care project](https://linear.app/coda-health/project/connected-care-v1-9f745b9c6874)
tracks these outcomes and their acceptance.
