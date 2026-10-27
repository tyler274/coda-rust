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

When a push cannot claim its turn, the worker answers from the saved conversation. It acknowledges a turn that is committed, stopped, replaced by a newer message, or running under another live claim. It answers 500 for a turn that is still waiting and for one that recovery gave up on. The second case is what lets Pub/Sub dead-letter the message: it counts each message's own deliveries, so a message the sweep republished under a new ID still reaches the dead-letter topic.

The worker publishes `workspace_changed` after each committed reply and each failed attempt, including a provider that cannot start and a push that finds the turn already failed. That last case covers a turn whose worker leases all ran out: the claim records the fifth failure and nothing is left to run. A held page then refetches and shows "Coda couldn't finish". The site's recovery sweep republishes accepted turns whose wakeup never reached the topic, so a turn accepted by an instance that died is finished by whichever instance is still up. [`tests/instance_loss.rs`](/crates/physician-sample/tests/instance_loss.rs) proves both against the emulator, Valkey and Postgres.

Outside hosting, `PUBSUB_EMULATOR_HOST` turns off push-token checks, because the emulator sends no token. The worker then starts only on a loopback listener (`127.0.0.1` or `[::1]`). `PORT` alone binds `0.0.0.0`, so it, a wildcard, or any other address refuses to start. Hosted runs refuse the emulator setting altogether.

The site queues turns only when `PHYSICIAN_CHAT_TOPIC` is set. Without it, hosted or not, chat runs inline in the site process as it did before the worker, and a hosted start logs a warning. `PHYSICIAN_CHAT_TOPIC` without `PHYSICIAN_DATABASE_URL` refuses to start, because the turn is accepted in Postgres.

| `PHYSICIAN_CHAT_TOPIC` | `PHYSICIAN_DATABASE_URL` | Startup |
| --- | --- | --- |
| unset | unset | Chat runs inline. Hosted logs a warning. |
| unset | set | Chat runs inline. Hosted logs a warning. |
| set | unset | Refuses to start. |
| set | set | Turns go to Postgres and the topic. The worker runs the model. |

With the topic set, reply revision also runs in the worker. Drafts live in `physician_runtime_reply_drafts`, one JSONB row per practice, physician, workspace and reply, which [`Store::migrate`](/crates/local-product/src/runtime.rs) creates from its SeaORM entity. `GET` and `PUT /replies/{id}` read and save that row; a save must name the current revision, and a `PUT` never creates a draft. `reply.revise` checks the session and workspace, then `Store::accept_revision` records the instruction, the saved text it applies to and the workspace's pinned source revisions, and the site publishes a `reply_revision` delivery on the chat-turn topic. The same command id returns the same acceptance and publishes nothing new. A draft that is missing, belongs to another physician, has moved past the command's expected revision, or already has a revision in progress refuses with 409.

The worker claims the draft and runs [`revision.rs`](/crates/physician-sample/src/bin/physician-worker/revision.rs) through `harness::execution::run`. It has two tools: `read_patient_sources`, which answers only from the pinned sources, and `finish_reply_revision`. Each model exchange is saved on the draft before the next one. A retry reuses the same instruction, base text and sources, and starts from the calls and tokens its revision's earlier attempts already spent, so the eight-call bound covers the whole revision. Completion commits under the physician's current session authority and the draft revision accepted with the command. If the physician saved a newer draft in between, the late completion is refused and their text stays. A failed attempt returns the revision to pending and answers 500, so Pub/Sub keeps counting toward the dead-letter topic; the fifth failure ends the revision as failed. The draft `GET` returns `revision_run`, the latest accepted command id and its status (pending, active, completed, failed or superseded), so a failed revision is visible, not only stored. Accepting another revision moves the previous one, with its sources and every attempt's records, into `physician_runtime_reply_revisions` in the same transaction, one row per earlier revision. The draft row holds only the current revision, so its size does not grow with history. A history row changes only when a superseded attempt's worker records a late exchange, observation or failure; that record is appended to the archived attempt and cannot publish or alter the current draft. Each row carries a per-draft ordinal assigned under the draft lock, and `Store::earlier_revisions` reads one physician's draft oldest first by that ordinal. The recovery sweep republishes due revisions just as it does turns.

`GET /patient-workspaces/{id}` lists the signed-in physician's runtime chats for that workspace, newest activity first, each titled by its first physician message (cut to about 60 characters). Durable chats come from `Store::chats` over `physician_runtime_conversations`, scoped by practice, physician and workspace. Postgres orders them by latest activity, then chat id, before taking the newest 200, so a chat that just got a message lists however many older chats the workspace has. Inline chat submissions, lists, snapshots and completions use the same practice and physician identity in `PhysicianSession`. Existing unscoped session records remain local anonymous data; signing in does not assign them to that physician. Durable conversations record `opened_at` and `last_activity_at` in their JSONB state, so no migration is needed. The page's `New conversation` button, in the Conversations sidebar and the empty Conversation region, opens a new `chat-…` id. The navigation owner keeps `?patient=` and `?chat=` aligned with the selected conversation, including sidebar selection and patient changes, and removes both when leaving the patient page. A chat is listed after a reload once its first message is accepted.

The physician site sets `cpu_idle = false` so the Redis subscriber and the Postgres listener keep running between requests. The worker stays on request-based CPU: the push request is the turn. Duet has no Redis instance and no chat-turn topic.

Memorystore Basic 1 GB is about $35/month and the site's minimum instance about $48/month after the free tier. Dev raised the `coda-rust-stg` budget alert from $10 to $200/month before applying. Nothing is applied without a reviewed saved plan. An operator writes `PHYSICIAN_REDIS_URL` into the Doppler config `stg_physician_runtime`. The activation steps, including the CA bundle and the cost table, are [Physician scaling](gcp.md#physician-scaling-redis-fan-out-and-pubsub-chat-turns).

`nix develop` provides `valkey` and the Pub/Sub emulator (`google-cloud-sdk` with the `pubsub-emulator` component, and `jre_headless` to run it).

The bus proofs in [`tests/redis.rs`](/crates/physician-bus/tests/redis.rs) do not race production timing. `RedisBus::connect_with` takes a test `Timing` (heartbeat, command timeout, reconnect backoff and jitter); `connect` keeps the production values. The fast-restart proof routes the bus through a TCP proxy that the test closes and reopens, with a 60 s heartbeat and no jitter, so only the reconnect path can resync. Each test waits for the bus to be live and quiet instead of sleeping a fixed time.

The local worker can fail chosen messages. With `PHYSICIAN_WORKER_FIXED_TEXT` set, `PHYSICIAN_WORKER_FIXED_FAIL_MARKER=1` makes a message containing `[fail:K]` fail its first K attempts, counted from the attempts Postgres saved on that message, so a worker restart or a second instance keeps the count. Try again keeps the message, so `[fail:5]` dead-letters and its Try again answers. Hosted execution refuses the variable, like the other fixed-provider variables.

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

- **Opening the first reply draft.** Neither the inline session nor the Postgres draft store creates a draft from a site request; the physician reply owner opens it. Today only tests call `Store::open_draft`, so a hosted physician can revise only a draft that owner has already opened. The page has no hosted revision UI change in this slice.
