# The physician site and the Coda Agents harness

The shipping physician client is the React site in `submodules/coda-agents/web`. The Rust page is the Dioxus client in [`physician-app`](/crates/physician-app/src/lib.rs), mounting into `#main`, speaking the same projections through [`physician-wire`](/crates/physician-wire). [`physician-sample`](/crates/physician-sample/src/lib.rs) serves the page and the physician routes over the synthetic practice, with chat through the model execution described next; tests also use its recorded projection, and `PHYSICIAN_UPSTREAM` forwards physician routes to the Node server. Model execution is owned by [`local-product`](/crates/local-product/src/lib.rs); `api` mode is opt-in and never runs without an authorized key. [`agent-skills`](/crates/agent-skills/src/lib.rs) reads the clinical skill catalog in [`skills/`](/skills) for [`harness`](/crates/harness/src/lib.rs), though no product runtime loads it yet; the [`encoder`](/crates/encoder/src/lib.rs) runs the offline patient-text inference, and [`patient-work`](/crates/patient-work/src/lib.rs) models the dispatcher's grouping window.

## Product direction and proof

The [Coda Agents project](https://linear.app/coda-health/project/coda-agents-d7d16847168e)
owns current product intent. The
<a href="/verification/physician-experience/README%2Emd">physician statechart</a>
is the behavior spec, accompanied by its design decisions, its Verus proof and
<a href="/verification/physician-experience/coverage%2Emd">coverage.md</a>, which says
what the Rust app shows for each of its states. <a href="/verification/README%2Emd">Verification</a>
keeps design artifacts separate from runtime and clinical qualification. The
original repository links below are comparison evidence, not a competing
specification or a claim of current Rust completeness.

## Current execution paths

[`physician-sample::live`](/crates/physician-sample/src/live.rs) currently opens
[`PhysicianSession`](/crates/local-product/src/execution.rs) for mounted chat
and reply work, using the local session file outside test fixtures. The separate
Postgres [`runtime::Store`](/crates/local-product/src/runtime.rs) owns durable
acceptance, leases and recovery primitives; configuring the physician auth
database does not connect chat routes to that store.

## Physician sign-in and practice access

[`physician-auth`](/crates/physician-auth/src/lib.rs) is one durable owner for who a clinic user or physician is, which practice they belong to, who holds panel-assignment permission, and which patients are on each physician's panel. Sign-in is a per-user email magic token: the server issues a one-time token (24-hour TTL, stored `sha256`-hashed), and its verification mints one bearer session (30-day TTL) with a revocation fence (`sessions_ended_before`) so deactivating an account ends every session it holds.

Access follows three concentric boundaries:
1. **Practice boundary (cross-clinic isolation):** Any patient or workspace outside the signed-in user's `practice_id` answers 404 — never 403 — and never appears in roster search.
2. **Clinic demographic boundary (same clinic, not on the physician's panel):** Physicians and staff in the clinic can search the clinic's patient roster and read basic demographic information (`display_name`, `birth_date`, `age_years`, `sex_or_gender`, `medical_record_numbers`) on `GET /patient-workspaces/{id}` for other patients in their clinic, while clinical synopses, citations, chats, follow-ups, Today items, drafts, and commands remain omitted/404.
3. **Panel clinical boundary (`Membership::Allowed`):** Full clinical workspace reads, Today items, chat snapshots, drafts, and commands require the patient to be assigned to the physician's panel. Clinic admins, secretaries, and physicians holding the `admin` permission can assign or remove clinic patients on a physician's panel via `PUT /auth/panels/{physician_id}/{patient_workspace_id}` (`AuthStore::assign_panel`).

The store is the [`AuthStore`](/crates/physician-auth/src/lib.rs) trait over SeaORM and PostgreSQL: `physician-sample` holds `Arc<dyn AuthStore>` and picks the Postgres store (connecting without applying migrations; the migration job owns schema changes) when `PHYSICIAN_DATABASE_URL` is set, process memory otherwise. With `PHYSICIAN_AUTH=required`, one Axum middleware guard resolves the bearer session before any physician read and enforces the practice, clinic-demographic, and panel boundaries before any patient read or command dispatch; the auth routes and `/health` stay open. The physician client in [`physician-app`](/crates/physician-app/src/lib.rs) carries the bearer session on every read through [`physician-client`](/crates/physician-client/src/lib.rs) and shows an email form followed by “Check your email” before any physician data renders. Clicking the email’s Sign in button removes the token from the URL, redeems it automatically, and opens Today without a code field or second confirmation. Used or expired links offer a new link; verification transport failures remain distinguishable from an invalid link. Local proof delivery redeems its returned token through the same client path without exposing it in the page.

The retained GKE deployment reference is <a href="/infra/gcp/physician-runtime/README%2Emd"><code>infra/gcp/physician-runtime</code></a>: one private GKE Autopilot cluster, one private Cloud SQL Postgres 16 instance with IAM database authentication (no database password), separate keyless site and migration identities, and `PHYSICIAN_AUTH=required` riding the site's environment as configuration intent. The current isolated Rust staging path is Cloud Run under [GCP deployment](gcp.md#rust-staging-bootstrap-and-deployment-gate); configuration is not hosted access proof.

## Scaling across instances

A turn that finishes on one `physician-sample` instance has to show up on a stream another instance is holding open. [`physician-bus`](/crates/physician-bus/src/lib.rs) is the physician product's fan-out. It carries `workspace_changed`, `workspace_settled`, and `resync`. `resync` is what a subscriber gets when it lags or the connection drops, and the Dioxus client refetches on that event. Sign-in and panel changes stay on the existing Postgres `NOTIFY` feed, which also fires for the migration job and for `psql`.

`RedisBus` is the `fred` client: TLS plus AUTH, `PHYSICIAN_REDIS_URL` as a `rediss://` URL and `PHYSICIAN_REDIS_CA_PEM` as the Memorystore server CA. With `PHYSICIAN_REDIS_URL` unset, `LocalBus` keeps those events inside one process. That is the local and test path.

Hosted `message.submit` accepts the turn in Postgres and publishes `{scope, input_id}` on the Pub/Sub topic `physician-chat-turns`. The topic carries only that pair. `physician-worker` checks the push token's audience and the `physician-pubsub-push` email, claims the leased turn, runs the model, commits, and publishes `workspace_changed`. A non-2xx response is a redelivery. After 5 attempts the message goes to the dead-letter topic. The subscription's ack deadline is 600 seconds, and retries back off from 10 seconds to 600 seconds.

The site queues turns only when `PHYSICIAN_CHAT_TOPIC` is set. Without it, hosted or not, chat runs inline in the site process as it did before the worker, and a hosted start logs a warning. `PHYSICIAN_CHAT_TOPIC` without `PHYSICIAN_DATABASE_URL` refuses to start, because the turn is accepted in Postgres.

| `PHYSICIAN_CHAT_TOPIC` | `PHYSICIAN_DATABASE_URL` | Startup |
| --- | --- | --- |
| unset | unset | Chat runs inline. Hosted logs a warning. |
| unset | set | Chat runs inline. Hosted logs a warning. |
| set | unset | Refuses to start. |
| set | set | Turns go to Postgres and the topic. The worker runs the model. |

With the topic set, `reply.revise` answers 503 "Hosted reply revision is unavailable." Reply revision runs the model in the site process, and with the topic set the site no longer runs the model. It works again once revision moves to the worker.

The physician site sets `cpu_idle = false` so the Redis subscriber and the Postgres listener keep running between requests. The worker stays on request-based CPU: the push request is the turn. Duet has no Redis instance and no chat-turn topic.

Memorystore Basic 1 GB is about $35/month, against the $10 `coda-rust-stg` budget alert. Raise that alert before applying. Nothing is applied without a reviewed saved plan. An operator writes `PHYSICIAN_REDIS_URL` into the Doppler config `stg_physician_runtime`. The activation steps, including the CA bundle and the cost table, are [Physician scaling](gcp.md#physician-scaling-redis-fan-out-and-pubsub-chat-turns).

`nix develop` provides `valkey` and the Pub/Sub emulator (`google-cloud-sdk` with the `pubsub-emulator` component, and `jre_headless` to run it).

- <a href="/submodules/coda-agents/docs/architecture%2Emd#system-boundary">Architecture: the system boundary and model roles</a>
- <a href="/submodules/coda-agents/docs/architecture%2Emd#external-clinical-evidence">Architecture: external clinical evidence</a>
- <a href="/submodules/coda-agents/docs/runtime-protocol%2Emd">Runtime protocol</a>
- <a href="/submodules/coda-agents/docs/physician-experience%2Emd#physician-product-direction">Physician experience</a>
- <a href="/submodules/coda-agents/docs/ai-care-team-operating-contract%2Emd">Care-team operating contract</a>
- <a href="/submodules/coda-agents/docs/patient-messaging%2Emd">Messaging contract</a>
- <a href="/submodules/coda-agents/docs/readiness%2Emd#current-local-product-work">Readiness: current work</a>
- <a href="/submodules/coda-agents/docs/testing%2Emd#proof-by-boundary">Testing: proof by boundary</a>
- <a href="/submodules/coda-agents/README%2Emd#run-and-verify">Run and verify</a>

## Remaining gaps

- **Hosted reply revision.** With `PHYSICIAN_CHAT_TOPIC` set, `reply.revise` answers 503. Follow-up: accept the revision in Postgres, publish it on the chat-turn topic, and have `physician-worker` run it. Then remove the 503 in [`live.rs`](/crates/physician-sample/src/live.rs).
