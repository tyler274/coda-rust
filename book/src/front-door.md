# Accounts and practice connection

[ENG-161](https://linear.app/coda-health/issue/ENG-161) owns standalone Duet
identity; [ENG-70](https://linear.app/coda-health/issue/ENG-70) owns the practice
connection. A Duet account does not require an invitation. An invitation attaches
the practice to that account; existing accounts and data are not migrated from
the old app. The products keep separate data and authority.

## Current scope

The MVP has one live invited practice, fixed consent at join, sharing until
patient leave, practice discharge or account deletion, and re-invitation after
the connection ends. There is no separate stop/resume-sharing or re-consent
flow. Ending a connection preserves the Duet account and history; the last
received doctor's plan stays dated and is no longer updated. Multiple live
practices and clinician lookup are later work.

[ENG-39](https://linear.app/coda-health/issue/ENG-39) owns consented evidence
reaching the physician. [ENG-45](https://linear.app/coda-health/issue/ENG-45)
owns documented care instructions reaching Duet with provenance. Physicians do
not approve Duet's wording or manage the patient app. The patient-owned doctor
workspace—preparation, updates, review and saving—is in MVP under
[ENG-169](https://linear.app/coda-health/issue/ENG-169). Sharing with physicians
outside Coda follows after MVP, using browser review with no PDF download.
The jointly maintained live shared space in [ENG-27](https://linear.app/coda-health/issue/ENG-27)
is also post-MVP; it is separate from both the patient-owned workspace and the
MVP practice connection. There is no paid tier in the MVP; the
[Connected care project](https://linear.app/coda-health/project/connected-care-v1-9f745b9c6874)
owns scope and cadence decisions.

## Implementation and remaining gaps

| Rust owner | Implemented responsibility and boundary |
| --- | --- |
| [`duet-api::identity`](/crates/duet-api/src/identity.rs) | Duet identity store and authentication; standalone sign-up/session acceptance remains ENG-161's whole journey |
| [`front-door`](/crates/front-door/src/lib.rs) | Existing account, invitation, connection, consent and premium adapter, with SeaORM/Postgres persistence; its account model is not a decision to make standalone Duet depend on practice admission |
| [`physician-sample::auth`](/crates/physician-sample/src/auth.rs) | Practice-authorized invitations and linked-patient reads; cross-practice access stays refused |
| [`duet-api`](/crates/duet-api/src/lib.rs) and [`duet-client`](/crates/duet-client/src/lib.rs) | Join, consent and premium endpoints and client calls |
| [`duet-app::account`](/crates/duet-app/src/account.rs), [`duet-app`](/crates/duet-app/src/lib.rs) | Standalone account root already owns private access; the old practice join gate is disabled. Retained join code and the sharing toggle still need reconciliation with ENG-70, not restoration as an account prerequisite |

The legacy consent withdrawal/resume and paid-premium fields still exist in the
adapter. Do not infer approved MVP behavior from them. Reconcile those paths
under ENG-70/ENG-161 and the connected-care outcomes, including account linking,
ended-connection behavior and both products' consumers. Documentation alignment
does not implement that migration.

`front-door-migrate` owns the adapter's schema; see
[Duet migrations](backend-architecture.md#persistence-and-migrations) and
[GCP migration jobs](gcp.md). The existing
[`front_door_postgres.rs`](/crates/physician-sample/tests/front_door_postgres.rs)
proves its scoped invitation/link path, not all current connection acceptance.
The [Bridge technical outline](https://linear.app/coda-health/document/bridge-technical-outline-6206e6c1af4c)
routes to these owners and the current outcomes.
