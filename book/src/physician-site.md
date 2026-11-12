# The Chords physician site and its harness

The Rust page is the Dioxus client in [`physician-app`](/crates/physician-app/src/lib.rs), mounting into `#main`, consuming projections through [`physician-wire`](/crates/physician-wire). [`physician-sample`](/crates/physician-sample/src/lib.rs) serves the page and the physician routes over the synthetic practice, with chat through the model execution described next; `PHYSICIAN_UPSTREAM` forwards physician routes to the Node server. Model execution is owned by [`local-product`](/crates/local-product/src/lib.rs); `api` mode is opt-in and never runs without an authorized key. [`agent-skills`](/crates/agent-skills/src/lib.rs) reads the clinical skill catalog in [`skills/`](/skills) for [`harness`](/crates/harness/src/lib.rs); physician chat loads it (see [Skills in physician chat](#skills-in-physician-chat)); the [`encoder`](/crates/encoder/src/lib.rs) runs the offline patient-text inference, and [`patient-work`](/crates/patient-work/src/lib.rs) models the dispatcher's grouping window.

The web build calls the API at a loopback `?api=` URL when one is given, otherwise at `PHYSICIAN_API_URL` set at build time, otherwise at its own origin (under `/physician` when it is mounted there).

## Product direction and proof

The <a href="/intent/physician%2Emd">physician spec</a> owns product intent; each
feature's TLA+ model lives under <a href="/verification/README%2Emd">verification</a>.

## Runtime schema cutover

The harness cutover changes the shape of the Chords runtime tables. It is a
clean cutover: nothing reads earlier runtime data, so these tables are reset
once. Rust staging holds only synthetic physician runtime data. Ordinary
releases never reset data, and neither startup nor `Store::migrate` performs
this reset.

This section states what the cutover needs. How staging stops writers, reaches
the database, releases images and routes traffic is the infrastructure owner's
procedure and is not described here.

Before the site and the worker start, the settings in
[Model settings](#model-settings) exist for both. Each refuses to start without
them.

The reset discards chats, drafts, revision history, response feedback,
attempts, command receipts, frozen execution inputs, provider cooldown and
retained synthetic chart observations. It also drops
`physician_runtime_migrations`, the runtime chain's ledger, so a reset leaves
no ledger behind. It leaves account, session, practice, panel and front-door
tables in place.

```sql
DROP TABLE IF EXISTS
    physician_runtime_chat_attempts,
    physician_runtime_chat_drafts,
    physician_runtime_commands,
    physician_runtime_conversations,
    physician_runtime_events,
    physician_runtime_migrations,
    physician_runtime_provider_quota,
    physician_runtime_recipes,
    physician_runtime_skill_packages,
    physician_runtime_reply_drafts,
    physician_runtime_reply_revisions,
    physician_runtime_response_feedback,
    physician_runtime_revision_attempts,
    physician_runtime_source_pulls,
    physician_runtime_source_versions,
    physician_runtime_sources;
```

After the drop:

- The cutover's migration creates the tables. A migration image from before the
  cutover creates the earlier tables again.
- The synthetic charts are loaded again with `physician-migrate load-charts`,
  because the reset drops the source tables that hold them. See
  [Staging synthetic charts](gcp.md#staging-synthetic-charts).
- A notification sent before the reset names work that no longer exists. The
  worker's claim fails with `error.type` `claim` and the notification is gone;
  nothing sends it again, and the due-work read finds nothing of it.
- The earlier site and worker do not work with the new tables, and the new ones
  do not work with the earlier tables.

An automatic deploy of the cutover commit does not perform the reset. If the
reset is skipped, the runtime migration fails: its first migration creates each
runtime table and stops at the first one that already exists. It runs in one
transaction, so it leaves the runtime tables as they were. The
`physician-migrate` job then fails, and the release stops at that failed
migration.

[`runtime.sql`](/crates/local-product/src/runtime.sql) is the first migration of
the [runtime chain](/crates/local-product/src/runtime/migration/mod.rs). The
chain records each applied migration in `physician_runtime_migrations`, like the
physician-auth, front-door and clinical-library chains, and `Store::migrate`
applies only the migrations not recorded there. A later schema change is a new
migration in the chain. Existing permanent records are not rewritten to add
fields. Once a practice holds real data, storage changes must neither refuse to
start nor wipe it, as the
<a href="/intent/physician%2Emd">physician intent</a> requires.

## Current execution paths

The page, identity, clinical execution and delivery each have an owner:

| Responsibility | Owner |
| --- | --- |
| Physician screens, navigation and command feedback | [`physician-app`](/crates/physician-app/src/lib.rs), through [`physician-client`](/crates/physician-client/src/lib.rs) and [`physician-wire`](/crates/physician-wire) |
| Sign-in, practice membership and panel access | [`physician-auth`](/crates/physician-auth/src/lib.rs), enforced at the site routes |
| Serving projections and accepting chat commands | [`physician-sample`](/crates/physician-sample/src/lib.rs) and its [`live`](/crates/physician-sample/src/live.rs) adapter |
| Physician model execution and conversation state | [`local-product`](/crates/local-product/src/lib.rs): Postgres `runtime::Store` for every accepted job |
| Waking accepted queued turns and executing them | [`durable`](/crates/physician-sample/src/durable.rs) and [`physician-worker`](/crates/physician-sample/src/bin/physician-worker.rs), through Postgres `NOTIFY` and the worker's due-work read |
| Telling open pages to reread changed state | [`physician-bus`](/crates/physician-bus/src/lib.rs), using a local bus or Redis fan-out |

Chat, saved drafts and reply revisions use the Postgres
[`runtime::Store`](/crates/local-product/src/runtime.rs) and the same worker locally
and when hosted. The site requires `PHYSICIAN_DURABLE_CHAT=1`, `PHYSICIAN_DATABASE_URL`
and `PHYSICIAN_AUTH=required` for this path. With `PHYSICIAN_DURABLE_CHAT` unset, a
set `PHYSICIAN_CHAT_TOPIC` also selects durable chat: a deployed environment may carry
only the topic until the `rust-environments` root is applied, and that root sets
`PHYSICIAN_DURABLE_CHAT=1`. The topic is not otherwise read. A hosted site refuses to
start without durable chat. With neither set, or `PHYSICIAN_DURABLE_CHAT=0`, a local site
can render fixtures or proxy another server, but cannot execute chat. The worker selects API-key or local developer-plan
Responses transport; credentials are worker configuration, outside the durable job.
[Local chat setup](/README%2Emd#local-physician-chat) describes the plan grant, which
refreshes before each request. [Scaling across instances](#scaling-across-instances)
describes acceptance, recovery and fan-out.

### Patients and the local practice

[`chart_load.rs`](/crates/physician-sample/src/chart_load.rs) is test tooling that
loads the synthetic charts through their adapter, and `physician-migrate load-charts`
runs it apart from any deploy. [`patients.rs`](/crates/physician-sample/src/patients.rs)
is how a patient is read: the auth store says who belongs to the practice and whose
panel they are on, and each source's last pull says who they are and when their
records were refreshed. With no database the practice holds no patients.
[`scripts/local-practice.sh`](/scripts/local-practice.sh) runs the practice on a local
Postgres database with its worker and Valkey. The database persists
between runs; the worker uses the same notes, windows, history and chart tools as staging.

### Choose the proof for the claim

| Check | What it can establish | Boundary |
| --- | --- | --- |
| Feature model checked by TLC | Every state the modeled steps reach keeps the agreed rules. | Covers only the finite model; does not execute the app, its Rust owners or a clinical model. |
| `cargo test -p physician-sample --test site` | The exercised Chromium journeys render and behave correctly against their test setup. | Does not qualify every graph state or hosted dependency. |
| <a href="/.agents/skills/coda-verify-physician/SKILL%2Emd">Local physician scale recipe</a> | Real site/worker binaries, Postgres and Valkey support the exercised cross-instance chat and recovery journeys. | Fixed provider, proof sign-in and a local Postgres; not real model quality, mail delivery, Cloud Run or Cloud SQL behaviour. |
## Physician sign-in and practice access

[`physician-auth`](/crates/physician-auth/src/lib.rs) is one durable owner for who a clinic user or physician is, which practice they belong to, who holds panel-assignment permission, and which patients are on each physician's panel. Sign-in is a per-user email magic token: the server issues a one-time token (24-hour TTL, stored `sha256`-hashed), and its verification mints one bearer session (idle for 12 hours, and never past 7 days after sign-in) with a revocation fence (`sessions_ended_before`) so deactivating an account ends every session it holds.

Access follows three concentric boundaries:
1. **Practice boundary (cross-clinic isolation):** Any patient or workspace outside the signed-in user's `practice_id` answers 404 — never 403 — and never appears in roster search.
2. **Clinic demographic boundary (same clinic, not on the physician's panel):** Physicians and staff in the clinic can search the clinic's patient roster and read basic demographic information (`display_name`, `birth_date`, `age_years`, `sex_or_gender`, `medical_record_numbers`) on `GET /patient-workspaces/{id}` for other patients in their clinic, while clinical synopses, citations, chats, follow-ups, Today items, drafts, and commands remain omitted/404.
3. **Panel clinical boundary (`Membership::Allowed`):** Full clinical workspace reads, Today items, chat snapshots, drafts, and commands require the patient to be assigned to the physician's panel. Clinic admins, secretaries, and physicians holding the `admin` permission can assign or remove clinic patients on a physician's panel via `PUT /auth/panels/{physician_id}/{patient_workspace_id}` (`AuthStore::assign_panel`).

The store is the [`AuthStore`](/crates/physician-auth/src/lib.rs) trait over SeaORM and PostgreSQL: `physician-sample` holds `Arc<dyn AuthStore>` and picks the Postgres store (connecting without applying migrations; the migration job owns schema changes) when `PHYSICIAN_DATABASE_URL` is set, process memory otherwise. With `PHYSICIAN_AUTH=required`, one Axum middleware guard resolves the bearer session before any physician read and enforces the practice, clinic-demographic, and panel boundaries before any patient read or command dispatch; the auth routes and `/health` stay open. The physician client in [`physician-app`](/crates/physician-app/src/lib.rs) carries the bearer session on every read through [`physician-client`](/crates/physician-client/src/lib.rs). The session belongs to the browser, in `localStorage`, so every tab and a reopened Coda share it while it lasts. The client shows an email form followed by “Check your email” before any physician data renders. Clicking the email’s Sign in button removes the token from the URL, redeems it automatically, and opens Today without a code field or second confirmation. Used or expired links offer a new link; verification transport failures remain distinguishable from an invalid link. Local proof delivery redeems its returned token through the same client path without exposing it in the page.

The retained GKE deployment reference is <a href="/infra/gcp/physician-runtime/README%2Emd"><code>infra/gcp/physician-runtime</code></a>: one private GKE Autopilot cluster, one private Cloud SQL Postgres 16 instance with IAM database authentication (no database password), separate keyless site and migration identities, and `PHYSICIAN_AUTH=required` riding the site's environment as configuration intent. Rust staging runs on Cloud Run under [GCP deployment](gcp.md#rust-staging-bootstrap-and-deployment-gate).

## Scaling across instances

A turn that finishes on one `physician-sample` instance has to show up on a stream another instance is holding open. [`physician-bus`](/crates/physician-bus/src/lib.rs) is the physician product's fan-out. It carries `workspace_changed`, `workspace_settled`, and `resync`. `resync` is what a subscriber gets when it lags or the connection drops, and the Dioxus client refetches on that event. Sign-in and panel changes stay on the existing Postgres `NOTIFY` feed, which also fires for the migration job and for `psql`.

`RedisBus` is the `fred` client: TLS plus AUTH, `PHYSICIAN_REDIS_URL` as a `rediss://` URL and `PHYSICIAN_REDIS_CA_PEM` as the Memorystore server CA. With `PHYSICIAN_REDIS_URL` unset, `LocalBus` keeps those events inside one process. That is the local and test path.

Hosted `message.submit` accepts the turn in Postgres. The transaction that saves it also sends `pg_notify` on the channel `physician_work` with `{scope, input_id}`, and nothing else. Postgres delivers a notification only when its transaction commits, so there is no accepted turn whose announcement failed. Try again, Resume, an accepted reply revision and a claim a stopping worker hands back are announced the same way ([`runtime/delivery.rs`](/crates/local-product/src/runtime/delivery.rs)).

`physician-worker` holds one `LISTEN` connection and starts announced work at once ([`pull.rs`](/crates/physician-sample/src/bin/physician-worker/pull.rs)). A notification sent while that connection is down is lost, so the worker also reads the due work every 10 seconds, and again each time it resubscribes: pending work whose retry delay has passed, and attempts whose lease ran out. A notification only names work. The claim in Postgres decides who runs it, so work heard twice, or by two workers, runs once, and work that is committed, stopped, replaced by a newer message or held under another live claim is not claimed. A worker runs at most four attempts at a time. Its HTTP listener serves `/health` and `/readyz` only; there is no route that delivers work.

When a newer message steers an executing answer, running calls finish
within their 300-second deadline before settlement releases the old claim.
Stop does not wait for running calls. Settlement makes the waiting
message due at once and announces it in the same transaction, so a listening
worker starts the answer without waiting for its next due-work read.

The worker publishes `workspace_changed` after each committed reply and each failed attempt, including a provider that cannot start and a claim that finds the turn already failed. That last case covers a turn whose worker leases all ran out: the claim records the fifth failure and nothing is left to run. A held page then refetches and shows "Coda couldn't finish". Accepted work lives only in Postgres, so a turn accepted by a site instance that died, or announced while no worker was listening, is found by the next due-work read of whichever worker is up. [`tests/instance_loss.rs`](/crates/physician-sample/tests/instance_loss.rs) proves both against Valkey and Postgres.

An intentional Stop is `work.stop`. It holds that turn until `work.resume` or a newer message, including across a worker restart, and the site publishes `workspace_changed` so another open tab refetches. The snapshot shows Stop while the turn is still pending or active, and Resume once it is held. Each Stop press is its own command, so Stop after Resume holds the resumed run again, even before a worker claims it; each hold names its own run for Resume. Another press on work that is already held is answered as accepted and changes nothing. A newer message releases the hold and is answered, and the stopped run loses its Resume. A late model result cannot publish over Stop or over the newer message. An infrastructure failure does not end the turn by itself: the worker settles it using the harness failure classification, and a crash before that write leaves the attempt leased so expiry starts a successor. A run that marks no step for 600 seconds is recovered the same way: the worker cancels it, stops renewing its lease and settles nothing. The harness owns the five-failure ceiling. Reached limits and intentional interruptions never consume that allowance. At the run limit, the final request may leave a partial answer. Try again continues the work with a fresh allowance. A structured job keeps its handoff in the run record until its own finish operation publishes. [`tests/stop.rs`](/crates/physician-sample/tests/stop.rs) proves Stop, the held readback after a worker restart, a second Stop after Resume, and one answer to a message sent while held.

Each published reply has a response id: the id its assistant message is saved with in the commit that publishes it, which names the attempt that answered. It stays the same across restarts, so it is the reply's durable identity, the one feedback and evaluation refer to, and the team matches it to the attempt's telemetry. `Conversation::responses` reads the published replies with their ids. A partial answer kept when a run reaches its limit is not a response and has none. In the chat snapshot each published model turn carries `response_id`, while physician, mechanical and partial lines carry none. `runs` names the latest run of the turn in progress: `held` for Resume after Stop, `failed` for Try again, and `completed` once it was answered. A repeated `message.submit` under the same key is answered as accepted and runs nothing again; a different command under that key is rejected. Receipts belong to one physician, so another physician's command under the same key is theirs alone. The session's practice and physician scope every read and command, and the panel check answers 404 for a workspace the physician cannot see. A colleague with the same patient on their panel reads and changes only their own chats. [`tests/durable_access.rs`](/crates/physician-sample/tests/durable_access.rs) proves a repeated submit runs one turn whose reply carries its response id, and that a colleague, an off-panel physician and another practice can neither read the chat nor Stop, Resume or write into it.

The worker listens on `PHYSICIAN_WORKER_LISTEN_ADDR` when it is set, otherwise on `0.0.0.0:$PORT` when `PORT` is set, otherwise on `127.0.0.1:8791`.

Chat acceptance requires durable chat, the database and required sign-in configuration together.
A hosted site without durable chat refuses to start; local fixture pages
remain display-only.

Reply revision runs in the worker. Drafts live in `physician_runtime_reply_drafts`, one JSONB row per practice, physician, workspace and reply, which [`Store::migrate`](/crates/local-product/src/runtime.rs) creates from the fixed runtime baseline. `GET` and `PUT /replies/{id}` read and save that row; a save must name the current revision, and a `PUT` never creates a draft. `reply.revise` checks the session and workspace, then `Store::accept_revision` records the instruction, the saved text it applies to and the workspace's current source revisions with their version, read in its own transaction, and the same transaction announces a `reply_revision` delivery on `physician_work`. The same command id returns the same acceptance and announces nothing new. A draft that is missing, belongs to another physician, has moved past the command's expected revision, or already has a revision in progress refuses with 409.

The worker claims the draft and runs [`revision.rs`](/crates/physician-sample/src/bin/physician-worker/revision.rs) through the shared harness recovery and execution path with the physician product's frozen recipe. Revisions offer the shared working-note, fresh-window, history, skill and retained-result paging capabilities. The accepted draft owns its skill package and events; a successor keeps both. Notes are private work records, never saved reply text. History lists and reads this draft's accepted physician instructions and completed model revisions under current authority. `read_patient_sources` answers from the attempt's pinned sources, and only `finish_reply_revision` can publish the revised draft. Each model exchange is saved before the next one. A retry reuses the same instruction and base text, pins the newest sources when claimed, and restores the request count and usage already spent, so the shared backstops cover the whole revision.

Anything Coda is still working on takes in what arrives before it finishes, so nothing it produces rests on an older reading. A reply under revision is still a draft; its history is fixed only once something is said or sent. `Store::commit_revision` therefore compares the attempt's source version with the workspace's current one in the transaction that would publish, under the draft lock, the auth owner's locks and a shared lock on the source version row. If they differ, nothing is published. The attempt records a `SourceRefresh` (from and to version, added, corrected with the earlier revision kept, and withdrawn sources), the draft re-pins, and the model receives a `sources_changed` result naming those references. It reads them and finishes again in the same run, under the same request bound; a catch-up is not a failed attempt. A finish whose context does not yet hold an exact read of every added or corrected source, at its current revision, gets `sources_unread` instead. If the chart keeps changing until the bound runs out, the revision stays held with its handoff in the run record and the saved draft is unchanged. The attempt's `sources` stays what it claimed and its `refreshes` show what it could read afterwards, so the record lists only sources Coda could actually read.

Hosted sources live in `physician_runtime_sources` and `physician_runtime_source_versions` ([`runtime/sources.rs`](/crates/local-product/src/runtime/sources.rs)). Charts arrive through `Store::record_chart` and `Store::record_chart_with_receipt`, which land a patient's delivery in one transaction, as one version, keeping saved records that a later pull omits and recording their candidate IDs in `missing_candidates`; explicit withdrawal goes through `Store::withdraw_source`. A delivery says what the source holds now, so an earlier revision delivered again becomes current. A site also imports the `CODA_PHYSICIAN_SOURCE_SNAPSHOT` file at startup through `Store::import_sources` when one is set; a correction or withdrawal is a new row, never a rewrite, and the workspace's version moves only when what can be read changes. A revision identity is immutable within its workspace and candidate: different content or provenance under an already recorded identity is refused, including after correction or withdrawal. Replaying any identical historical revision is a no-op, so an older instance's startup snapshot cannot roll back a correction or resurrect a withdrawal. Restoring a withdrawn candidate requires a previously unseen revision identity; opaque revision strings are not sorted to infer age. Catch-up checks compare the exact native content, hash and provenance as well as revision identity, not just a matching revision label. `Store::record_source` owns individual immutable revisions and is also used by the startup import. Projection prose and model output never become sources. Completion commits under the physician's current session authority and the draft revision accepted with the command. If the physician saved a newer draft in between, the late completion is refused and their text stays. A failed attempt returns the revision to pending, due again after its retry backoff; the fifth failure ends the revision as failed. The draft `GET` returns `revision_run`, the latest accepted command id and its status (pending, active, completed, failed or superseded), so a failed revision is visible, not only stored. Accepting another revision moves the previous one, with its sources and every attempt's records, into `physician_runtime_reply_revisions` in the same transaction, one row per earlier revision. The draft row holds only the current revision, so its size does not grow with history. A history row changes only when a superseded attempt's worker records a late exchange, observation or failure; that record is appended to the archived attempt and cannot publish or alter the current draft. Each row carries a per-draft ordinal assigned under the draft lock, and `Store::earlier_revisions` reads one physician's draft oldest first by that ordinal. The worker's due-work read finds due revisions and expired revision leases just as it does turns.

The composer saves the physician's unsent chat text through `GET` and `PUT /physician-state/drafts/{workspace}/{chat}`. With no chat open the page saves under the chat id `draft`. [`chat_draft.rs`](/crates/physician-sample/src/chat_draft.rs) serves both. The draft is a row of `physician_runtime_chat_drafts`, keyed by practice, physician, workspace and chat, with typed `text` and `revision` columns, which `Store::migrate` creates from the fixed runtime baseline. [`Store::save_chat_draft`](/crates/local-product/src/runtime/chat_draft.rs) checks the physician's session and panel in the save's own transaction. It inserts revision 1 only when the page names revision 0, and otherwise updates only the revision the page names. A stale save writes nothing and answers 409 with `{outcome: "conflict", current}`, so one instance's save is read and fenced by every other instance. A missing session answers 401. A patient outside the physician's panel answers 404, and another physician on the same patient has a separate draft. The fixture router answers 409 because its projections are read-only.

Each of Coda's responses in a hosted chat carries a thumbs up and down, and once rated an "Add details" dialog with four reasons, a text box and its own Save. Feedback names a response by its response id, the `response_id` the snapshot carries on each of Coda's published turns, so a rating stays on the reply it was given to. [`feedback.rs`](/crates/physician-sample/src/feedback.rs) serves one route, `PUT /physician-state/feedback/{workspace}/{chat}/{response}`, whose body is the rating with optional details; it replaces both, so details never exist without a rating. Each rated response is a row of `physician_runtime_response_feedback`, created by the runtime chain's second migration and keyed by practice, physician, workspace, chat and response, holding the rating, the details, the time of the latest save and a revision: 1 on the first save, one more on each later one. [`Store::save_feedback`](/crates/local-product/src/runtime/feedback.rs) refuses an id that names no published reply in that chat, including a partial answer (404) and details over the length limit (422). There is no un-rate. The save checks the physician's session and panel in its own transaction and answers 409 when the auth owner refuses. A missing session answers 401 and a patient outside the panel 404. The chat snapshot carries the signed-in physician's saved feedback, with its revision, on each of Coda's turns they rated, so a physician sees only their own feedback. A failed feedback read never withholds the conversation: the site logs the cause and the snapshot carries every turn with `"feedback": "unavailable"` and no turn's feedback, and each response the page has neither saved nor read feedback on shows disabled thumbs saying "Couldn't load your rating" rather than showing as unrated, while the rest of the chat works. Feedback an earlier snapshot read still shows, at its revision, and an open details dialog stays open with its unsaved words. A failed conversation read answers 503, with its cause logged, and the page keeps showing that chat's last conversation, marked as not refreshed, until a reread succeeds. The fixture site has no feedback route and its turns carry no response id, so it shows no controls. [`physician-app/src/feedback.rs`](/crates/physician-app/src/feedback.rs) keeps one state per response, across chats: saved, saving or failed. It shows the highest revision it has seen, from any snapshot or a save's answer, so an older answer never replaces a newer one. A thumb sends the new rating with the saved details, so a rating change keeps them; after a failed details save it sends those unsaved details instead, since they are what the physician last wrote. The dialog's Save sends the details with the shown rating. A failed save says "Couldn't save", shows the saved rating again and keeps its draft to retry. A response takes one save at a time, which keeps going when the physician closes the dialog or leaves the chat.

Coda staff read feedback for evaluation with `physician-migrate feedback`, which prints one JSON line per rated response, in each chat's order: the practice, physician, workspace and chat it belongs to, the response id, its rating, reasons, details, the time of the latest save, its revision and the rated response's text, from the database at `PHYSICIAN_DATABASE_URL`. The response id names the attempt that answered, the link to that reply's telemetry. It only reads. The output is clinical text, so it runs on an operator's terminal and refuses to run as a Cloud Run job, where stdout would reach Cloud Logging. No HTTP route serves feedback to anyone but its author. [`tests/feedback.rs`](/crates/physician-sample/tests/feedback.rs) proves both: the author's saves, snapshot readback and refusals over HTTP, a snapshot that keeps the conversation when a feedback row can't be read, and the staff read's rows, its refusal as a Cloud Run job and the cause it prints when the database is unreachable.

[`composer.rs`](/crates/physician-app/src/composer.rs) holds the page's draft state as pure, unit-tested transitions: autosave, one save in flight per draft, and the conflict choices, where the physician either retries over exactly the revision shown or takes the newer text. A bare or malformed 409 is an ordinary failure and never authorizes an overwrite.

The Send button and Cmd/Ctrl+Enter share one path. Acceptance removes only the submitted text, and the saved draft is cleared only over the revision that save produced, so newer text from another tab stays.

[`local_draft.rs`](/crates/physician-app/src/local_draft.rs) keeps unsent text across a reload of the same tab, in `sessionStorage` only.

`PHYSICIAN_SYNTHETIC_REPLY=<patient workspace id>` turns on a synthetic reply source in [`synthetic_reply.rs`](/crates/physician-sample/src/synthetic_reply.rs). It is off by default and applies only to a signed-in physician of the synthetic practice (`roger-synthetic-practice`) on that patient. The workspace read then carries one fixed-ready reply with a stable server-owned id and non-PHI wording. If the physician has no draft for it, the site opens revision 1: through `Store::open_draft_authorized`, which holds the physician's session, account and panel locks in the insert's own transaction. Opening again, from any instance and concurrently, keeps the saved text and revision history. `PUT /replies/{id}` and `reply.revise` still refuse a reply that has no opened draft, and nothing is created from a browser payload. The reply card shows the saved text, its exact `draft_revision` and the latest revision run's status: pending, active, completed, failed (the saved draft is unchanged) or superseded (a newer saved edit won). Ask Coda to revise names the revision on screen. The hosted worker announces `workspace_changed` after completing or failing an attempt, and the page rereads the workspace; a missed notice is recovered by the existing resync after the bus reconnects. The card does not offer Send to the patient, and closing it is not supported in this mode. With the switch off, no reply appears.

`GET /patient-workspaces/{id}` lists the signed-in physician's runtime chats for that workspace, newest activity first, each titled by its first physician message (cut to about 60 characters). Durable chats come from `Store::chats` over `physician_runtime_conversations`, scoped by practice, physician and workspace. Postgres orders them by latest activity, then chat id, before taking the newest 200, so a chat that just got a message lists however many older chats the workspace has. The page's `New conversation` button, in the Conversations sidebar and the empty Conversation region, opens a new `chat-…` id. The navigation owner keeps `?patient=` and `?chat=` aligned with the selected conversation, including sidebar selection and patient changes, and removes both when leaving the patient page. A chat is listed after a reload once its first message is accepted.

The physician site sets `cpu_idle = false` so the Redis subscriber and the Postgres listener keep running between requests. The worker also sets `cpu_idle = false` and keeps one instance up, because its listener, its poll and its lease renewals run outside any request. Duet has no Redis instance and no worker.

`nix develop` provides `valkey` and Postgres, which is all the physician scale tests need.

## Skills in physician chat

Chat in `physician-worker` offers `skill_view`, working notes, fresh windows, history and result paging, together with `search_patient_information` and `read_patient_sources`. Discovery searches retained source content, decoded text and metadata within the authorized workspace. A blank query browses records. The model receives references and dates on demand and opens a specific revision for evidence. A changed or withdrawn revision requires a new search. Large reads return exact UTF-8 passages with a continuation into the same revision, including its decoded text. Local developer-plan chart calls are restricted to the synthetic bank. The physician site loads its [skill catalog](/skills) from `PHYSICIAN_SKILLS_DIR` at startup. A local site defaults to the workspace catalog. Hosted admission requires a packaged catalog. The site saves the package with accepted work so each attempt or replacement worker opens exactly the bodies and resources named in that work’s index.

Each model request carries the physician-owned system prompt followed by the catalog metadata: names, descriptions and resource paths, but no bodies. Every Chords job lists the whole package. The accepted work retains that exact package and recipe. The model opens an approved skill by name. An optional `file_path` opens a registered supporting resource. The exact body is retained; large results use the shared preview and range reader. An unknown tool or malformed arguments come back to the model as `{"executed": false, "error": ...}` without running; an unknown skill comes back `unavailable`. An unknown skill name also returns the names of the skills the package has. A bad call does not stop a valid read in the same turn. A skill is guidance only; it grants no tool, patient read or action. The Responses request carries an opaque cache key scoped to the conversation.

Chords uses the shared <a href="/intent/harness%2Emd#how-it-is-built-today">run budgets</a>. Each work retains one frozen recipe. A provider dispatch references saved items, records its new input or send-time projections and carries the IDs of newly included physician messages. Recovery reconstructs the exact request from those parts; a transport retry records a separate attempt to send it unchanged. New steering or source feedback makes a new request. A large source result stays with the work and can be read in pages.

Chat and revision events are appended to `physician_runtime_events`, scoped to
their product owner and attempt. Status, revision, accepted session and attempt
leases use typed control columns. Appending execution evidence does not rewrite
authored history; status transitions update their own control state. Readback and
recovery join the event rows to their attempts;
archiving a revision keeps those same rows, including evidence that arrives late.

The worker streams provider events through the shared Responses adapter. The run
record retains usage and completed messages; reply fragments use the live-text
callback and Chords neither saves nor displays them. Chords publishes the assembled
whole answer. The provider's silence timeout
measures gaps between complete events, so a stream that stops making progress
can retry within the run before an answer is published.

Hosted chat supplies its retained authored messages to the harness's shared history
tools. The model can list in either order, search exact text and read a message in
character chunks. List and search previews leave the stored originals unchanged;
each chunk carries its original message identity, offset and full length. Result
paging also applies to history reads, under the same current physician authority.

### Model settings

Every enabled Chords runtime requires `CODA_<ROLE>_MODEL`, `CODA_<ROLE>_EFFORT`,
`CODA_<ROLE>_CONTEXT_WINDOW_TOKENS` and `CODA_<ROLE>_INPUT_LIMIT_TOKENS` for `CHAT` and
`BACKGROUND`. Missing or invalid settings refuse startup.
Hosted revisions select the background role. The site freezes each recipe when
it accepts the work: model settings, system prompt, tools and skills.
Workers read that saved recipe through retries, steers and recovery.

Local process settings and the product's hosted Doppler configuration own the
values. The synthetic stack declares its settings in
[`scripts/physician-scale-stack.py`](../../scripts/physician-scale-stack.py).

The shared token backstop counts output plus input not reported as cached. Missing
counts use the existing context and content estimate; recorded provider usage
stays distinct from those estimates. Billing belongs to the provider's usage
records. The budget survives recovery through the product's retained run events.

The working window is separate from the provider input limit. The harness estimates the first request and each fresh window, then uses reported input and output usage plus an estimate for added content. It requests a handoff near its named threshold, reserving room for the reply. Chords sets no output-token cap. Dispatch events retain those counts. When context is full, or the provider refuses it as too long, the harness opens a fresh window from exact accepted inputs and pending results. The fresh window carries the exact text of the newest note saved in this work, says when work done after that note is not in it, and points to earlier notes in the order saved. A refused fresh window gets one retry with smaller pending results, then the run is held at a limit. The shared response transport bound is separate from context pressure.

Large tool results appear as short previews of their start and end. The exact
original stays in the product's run record. `read_tool_result` reads a byte range
or finds a literal match under current authority. Previously sent results stay
unchanged within their window, and a partial read never establishes full source
coverage. The harness owns presentation bounds; those bounds do not reject writes.

Chords publishes the text from the final answer after checking the saved candidate, current lease, input revision and physician authority. Earlier tool narration stays in the run record. An empty ordinary model turn gets feedback. At a run limit, Chords keeps the handoff text as a partial reply and offers Try again. A revision that reaches its limit stays held with its handoff in the run record. Only its explicit finish operation can update the draft.

## Retention

The physician runtime keeps clinical history and the evidence Coda can learn from; only disposable copies expire, after 90 days. HIPAA sets no single record period, so clinical history follows the practice's own record schedule. No automatic deletion runs until that schedule exists. The rows below are the hosted Postgres copies, plus the copies kept outside the runtime.

| Copy | Owner | Purpose | Rule |
| --- | --- | --- | --- |
| Conversations (`physician_runtime_conversations`) | [`runtime::Store`](/crates/local-product/src/runtime.rs) | Clinical history: what the physician asked and what Coda answered | Kept under the practice's record schedule. No automatic deletion. |
| Chat drafts (`physician_runtime_chat_drafts`) and reply drafts (`physician_runtime_reply_drafts`) | [`chat_draft.rs`](/crates/local-product/src/runtime/chat_draft.rs), [`reply.rs`](/crates/local-product/src/runtime/reply.rs) | The physician's unsent text; for replies, the current revision with its instruction, base text, pinned sources and attempts with their source refreshes | Kept under the practice's record schedule, including a reply's last revision when no later one replaces it. |
| Response feedback (`physician_runtime_response_feedback`) | [`feedback.rs`](/crates/local-product/src/runtime/feedback.rs) | Evaluation evidence: each physician's rating and details on one of Coda's responses | Kept with clinical history. No automatic deletion. |
| Earlier reply revisions (`physician_runtime_reply_revisions`) | [`reply.rs`](/crates/local-product/src/runtime/reply.rs) | Learning evidence: what Coda could read, what it read and produced, and what the physician changed | Append-only: a superseded attempt's late records are added, nothing is rewritten; kept with clinical history until the de-identified learning export exists. |
| Patient sources (`physician_runtime_sources`) and their versions (`physician_runtime_source_versions`) | [`sources.rs`](/crates/local-product/src/runtime/sources.rs) | Every revision of each source a reply revision could read, and each workspace's observation version | Append-only: corrections and withdrawals are new rows. Kept with clinical history, since pinned and refreshed revisions on drafts and earlier revisions refer to them. |
| Each source's last pull (`physician_runtime_source_pulls`) | [`sources.rs`](/crates/local-product/src/runtime/sources.rs) | Patient directory identity per source, when a source last delivered a complete chart (or null when only partial records have landed), which "Records as of" reads, plus candidate IDs omitted on that pull | One row per patient and source; partial pulls preserve any prior complete timestamp and a later complete pull replaces it. |
| Execution records (attempt records on conversations and drafts) | [`runtime::Store`](/crates/local-product/src/runtime.rs) | Model requests and responses, tool reads, failures, and the evidence retries and recovery replay | Kept with the record they belong to. |
| Command receipts (`physician_runtime_commands`) | [`receipt.rs`](/crates/local-product/src/runtime/receipt.rs) | Idempotency: a repeated command id returns the same answer; a different command under that id is refused | Full command for 90 days from `accepted_at`, then reduced to a salted digest. |
| Langfuse traces | [Observability](observability.md) | Debugging model runs | The Langfuse HIPAA cloud project keeps them for 90 days. |

Command receipts duplicate input the runtime already keeps: a chat message lives in its conversation, and a reply instruction lives in its draft or archived revision. After 90 days, `Store::compact_receipts` replaces the stored command with `{"salt", "sha256"}`: a random salt kept on the row, and the SHA-256 of that salt followed by the command's canonical JSON (keys sorted). A repeated command still gets its original answer, and a different command under the same key is still refused. The hosted site's recovery sweep runs compaction on start and then hourly, in batches of 100 receipts, at most 2,000 per run.

Sources carry their own limits. Under Google's Workspace user-data policy, Gmail-derived content cannot train shared models, even when it is de-identified or rewritten. Duet's data stays with Duet's owners; its separate consent rules apply, and nothing here reads it.
