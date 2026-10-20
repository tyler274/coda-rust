# The patient front door

[`front-door`](/crates/front-door/src/lib.rs) is one durable owner, outside Duet and Agents, for who a patient is, which practice's roster they belong to, whether they consented to share with their physician, and whether they pay for premium. Identity is never guessed from names, phones, or clinical text: the practice invites the patient explicitly, and the accepted invitation writes the Duet-account-to-roster-patient link. Agents reads only a per-patient cadence setting and never knows who pays; Duet reads only which premium features are on; neither stores billing.

## Accounts, invitations, and links

An invitation is a fourteen-day, single-use offer a practice issues to one roster patient's email. Tokens are stored `sha256`-hashed, so the presented token is the only secret; redemption creates the account (or re-links an existing email), writes the roster link, and mints the first bearer session (30-day TTL, `sessions_ended_before` revocation fence). Sign-in for an existing linked account re-verifies through a one-day email token without touching the link. The store is the [`FrontDoorStore`](/crates/front-door/src/lib.rs) trait over SeaORM and PostgreSQL — `MemoryFrontDoorStore` for tests, `PostgresFrontDoorStore` (applying the [migration chain](/crates/front-door/src/migration)) when `FRONT_DOOR_DATABASE_URL` is set — and the axum router under `/front-door/*` answers invitations, redeem, sign-in, account, link, consent, and premium reads, with redeem and sign-in open (the token is the proof) and session reads behind the bearer.

## The practice side (ENG-48, ENG-44)

[`physician-sample`](/crates/physician-sample/src/lib.rs) holds the front door beside its auth store and offers two admin-gated routes behind the same panel gate as the panel changes — a clinic admin, secretary, or panel-assignment physician:

- `POST /auth/invitations` invites one patient on their own practice's roster by email; a cross-practice or unknown workspace answers 404, a non-admin answers 403, an email that already holds a front-door account answers 409, and the raw redeem token rides the response for proof setups.
- `GET /auth/front-door/{patient_workspace_id}` reads one linked patient's account (id, email, display name, active) beside their premium flag and the cadence Agents runs — `weekly`, or `weekly-daily-check` when the patient pays. An unlinked or cross-practice patient answers 404 and nothing here stores billing.

The Postgres journey (invite → redeem → link → premium read) is proved by [`physician-sample/tests/front_door_postgres.rs`](/crates/physician-sample/tests/front_door_postgres.rs), gated on `PHYSICIAN_DATABASE_URL`.

## The Duet side (ENG-47)

[`duet-api`](/crates/duet-api/src/lib.rs) embeds the front door beside its moments routes on one origin: `POST /api/join` redeems the practice's invitation (open by design — the one-time token is the proof) and the minted session becomes Duet's bearer; `GET/POST/DELETE /api/consent` reads, grants, and withdraws sharing in one step; `GET /api/premium` reads only which premium features are on. `main.rs` connects `PostgresFrontDoorStore` from `FRONT_DOOR_DATABASE_URL`, falling back to `DATABASE_URL`. [`duet-client`](/crates/duet-client/src/lib.rs) carries the join, consent, and premium calls with `with_bearer`, and [`duet-app`](/crates/duet-app/src/lib.rs) gates its tabs behind a join screen and a "share with my physician" toggle — unchecking withdraws in one step, and withdrawn consent keeps the granted-at date as dated evidence.

Identity is email-based for now. Whether Duet accounts keep a shared email magic-link provider or move to Sign in with Apple is an open Discuss-with-Dev item.

## Where the product rules live

This page covers the Rust implementation only. The product rules this slice touches belong to the product repositories, and they win over this page:

- Duet North Star: <a href="/submodules/coda-app/docs/north-star%2Emd#backend-and-ownership">Backend and ownership</a>, <a href="/submodules/coda-app/docs/north-star%2Emd#privacy-consent-and-learning">Privacy, consent, and learning</a>, <a href="/submodules/coda-app/docs/north-star%2Emd#identity-and-setup">Identity and setup</a>, and <a href="/submodules/coda-app/docs/north-star%2Emd#coda-agents-boundary">Coda Agents boundary</a>.
- Coda Agents: the deferred Apple Health and Duet data line in <a href="/submodules/coda-agents/docs/readiness%2Emd#patient-messaging-design">readiness</a>, and the <a href="/submodules/coda-agents/docs/ai-care-team-operating-contract%2Emd#intended-initial-weekly-cadence">intended initial weekly cadence</a>.

The [Bridge technical outline](https://linear.app/coda-health/document/bridge-technical-outline-6206e6c1af4c) in Linear lists the upstream rows that this slice changes. Those rows land through each product repository's own change process, not as copies here.
