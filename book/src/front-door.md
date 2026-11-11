# Accounts and practice connection

Duet owns standalone patient identity; the practice connection has a separate
invitation and consent boundary. A Duet account does not require an invitation.
An invitation attaches the practice to that account; existing accounts and data
are not migrated from the old app. The products keep separate data and authority.

## Scope

The <a href="/intent/connected-care%2Emd">connected-care spec</a> owns what a practice
connection does, from invitation and fixed consent to leaving, discharge and
re-invitation. The <a href="/intent/duet%2Emd">Duet spec</a> (D4) owns the patient's
For my doctor, including sending marked records and notes to a connected practice.

## Implementation

| Rust owner | Implemented responsibility and boundary |
| --- | --- |
| [`duet-api::identity`](/crates/duet-api/src/identity.rs) | Duet identity store and authentication; standalone sign-up/session acceptance requires the whole journey |
| [`front-door`](/crates/front-door/src/lib.rs) | Existing account, invitation, connection, consent and premium adapter, with SeaORM/Postgres persistence; its account model is not a decision to make standalone Duet depend on practice admission |
| [`physician-sample::auth`](/crates/physician-sample/src/auth.rs) | Practice-authorized invitations; cross-practice access stays refused |
| [`duet-api`](/crates/duet-api/src/lib.rs) and [`duet-client`](/crates/duet-client/src/lib.rs) | Join, consent and premium router adapters and client calls; the Duet binary does not mount the adapters |
| [`duet-app::account`](/crates/duet-app/src/account.rs), [`duet-app`](/crates/duet-app/src/lib.rs) | The standalone account root owns private access. It offers neither invitation enrollment nor a physician sharing control and does not request practice consent status |

The adapter's consent withdrawal/resume and paid-premium fields are not approved
MVP behavior.

[`duet-api::main`](/crates/duet-api/src/main.rs) mounts Duet-owned identity and
durable routes. Its optional front-door store is used by readiness; the separate
[`duet-privacy-delivery`](/crates/duet-api/src/bin/duet-privacy-delivery.rs)
operator also uses front-door journals. The physician auth
router mounts invitation operations.

`front-door-migrate` owns the adapter's schema; see
[Duet migrations](backend-architecture.md#persistence-and-migrations) and
[GCP migration jobs](gcp.md). The existing
[`front_door_postgres.rs`](/crates/physician-sample/tests/front_door_postgres.rs)
proves its invitation and link path.
