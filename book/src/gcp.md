# GCP deployment

Duet production composition stays in <a href="/submodules/coda-app/infra/gcp/README%2Emd"><code>infra/gcp/README.md</code></a>. The physician OpenTofu roots in this workspace are [`foundation`](/infra/gcp/foundation), <a href="/infra/gcp/staging/README%2Emd"><code>staging</code></a>, <a href="/infra/gcp/clinical-evidence/README%2Emd"><code>clinical-evidence</code></a>, and <a href="/infra/gcp/physician-runtime/README%2Emd"><code>physician-runtime</code></a>. The last one is the GKE Autopilot and Cloud SQL target for `physician-sample` and `physician-auth`. This workspace does not apply those reference roots. Public-site staging and the isolated Rust staging root are workspace-owned and require separately authorized saved-plan applies. [Build, CI, and release](release.md#retained-physician-infrastructure-references) says what each root owns.

The mail-only <a href="/infra/gcp/shared-mail/README%2Emd">shared-mail root</a> is owned here
for production SMTP in `coda-health-prod`, with separate state and explicit
saved-plan approval. This exception grants no authority over the referenced
production application or clinical-evidence roots.

## Public marketing site preparation

The separate [`infra/gcp/public-site`](/infra/gcp/public-site) root prepares the
stateless Rust marketing server on Cloud Run. Staging runs in the existing
`coda-health-staging` project with direct Cloud Run IAP and no load balancer or
CDN. Access is a service-scoped `roles/iap.httpsResourceAccessor` binding for
`group:gcp-engineering@codahealth.ai`, and the IAP service agent is the only
invoker. The origin also verifies the signed IAP assertion. Production stays behind
a global external HTTPS load balancer with Cloud CDN. There is no Cloudflare
dependency. Unlike the physician roots, this root owns its **staging** state
(GCS prefix `public-site/staging` in the existing state bucket) and may be planned
and applied for staging from this workspace. It is never planned or applied for
production here. Staging is deployed at `staging.codahealth.ai` behind IAP (see
[deployment status](public-site.md#staging-deployment-status-2026-10-21)). The
root does not share physician/Duet databases or authorize a product cutover. See
[Public site](public-site.md) for the architecture, IAP and cache controls, runtime
configuration, image build/smoke procedure, verification evidence and the
[staging state and deployment](public-site.md#staging-state-and-deployment)
procedure.

[PR #36](https://github.com/Coda-Health/coda-rust/pull/36) merged the implementation.
Dev's staging review and production approval, hostname/IAP/analytics
decisions, verified cutover/recovery procedures and the agreed recovery window
precede removal of website-only Vercel resources and credentials. Neither this
chapter nor passing local/CI validation authorizes deployment, DNS changes or
Vercel retirement.

## Rust container images

`nix/packages.nix` builds the Rust Cloud Run images from `Cargo.lock` and `flake.lock`. Each image tag is the git revision, and the build sets `created` to the epoch, so a revision always produces the same archive.

| Flake output | Image | Role | Entrypoint binary |
| --- | --- | --- | --- |
| `physician-sample-image` | `coda-physician-sample` | runtime | `physician-sample` serving `PHYSICIAN_SITE_DIR` (Dioxus WASM bundle) |
| `physician-worker-image` | `coda-physician-worker` | runtime | `physician-worker`, the Pub/Sub chat-turn worker, loading the skill catalog at `PHYSICIAN_SKILLS_DIR` |
| `duet-api-image` | `coda-duet-api` | runtime | `duet-api` serving `DUET_SITE_DIR` (Dioxus WASM bundle) |
| `physician-migration-image` | `coda-physician-migrate` | migration | `physician-migrate`: physician-auth, front-door, then runtime chains |
| `duet-migration-image` | `coda-duet-migrate` | migration | `duet-migrate`: Duet-owned chain and runtime grants, then optional front-door chain |
| `duet-privacy-delivery-image` | `coda-duet-privacy-delivery` | job | `duet-privacy-delivery`, a bounded Luna receipt job |

- The runtime servers never migrate. They bind `0.0.0.0:$PORT`, where `PORT` defaults to 8080. `/health` reports liveness. Hosted startup accepts only the explicit `stg` / `staging-preview` profile with its required stores and identity configuration; other hosted profiles refuse to start. `/readyz` checks the configured schemas. This preview profile and a successful schema check do not establish hosted sign-in, live-model acceptance or production readiness. The startup owners are [`physician-sample/src/hosting.rs`](/crates/physician-sample/src/hosting.rs) and [`duet-api/src/main.rs`](/crates/duet-api/src/main.rs).
- Each image contains the binary and the libraries it links, Doppler, CA roots, tzdata and a passwd entry for uid `65532`. The image has no shell or toolchain. It runs as `65532:65532` with working directory `/tmp`.
- Every entrypoint is `doppler run --no-fallback --preserve-env=PORT -- <binary>`. The Doppler CLI is the static release pinned in [`crates/workspace-tools/src/doppler.rs`](/crates/workspace-tools/src/doppler.rs). Nix reads the version and SHA-256 from that file, so the runners and the images run the same verified binary. Cloud Run mounts `DOPPLER_TOKEN` from Secret Manager. No secret is in a layer or in `Env`, and `--no-fallback` keeps secrets off the filesystem.
- `scripts/check-images.sh` (run with `nix develop .#release`) builds every image and inspects each archive. It loads them into Docker and checks rejection of missing credentials or an unconfigured hosted profile. Against disposable PostgreSQL it runs both migration images twice, then starts both servers to check `/health` 200, schema readiness 200, the WASM assets and a clean SIGTERM exit. The local Duet smoke uses a file journal and placeholder Identity Platform configuration; it does not exercise Cloud Run's required independent GCS journal or real sign-in. The `images` CI job runs this script. Nothing is pushed.

## Rust staging bootstrap and deployment gate

The isolated project is `coda-rust-staging` in `us-east1`. It is separate from
`coda-health-staging`, which hosts the public marketing site. The root owns its
GCS state under `rust-environments/stg`; the reviewed nonsecret inputs are
[`env/staging.tfvars`](/infra/gcp/rust-environments/env/staging.tfvars) and
[`backend/staging.gcs.tfbackend`](/infra/gcp/rust-environments/backend/staging.gcs.tfbackend).

From an authorized engineering account:

```bash
root=infra/gcp/rust-environments
nix develop .#release --command tofu -chdir="$root" init -input=false \
  -backend-config=backend/staging.gcs.tfbackend
nix develop .#release --command tofu -chdir="$root" plan -input=false \
  -var-file=env/staging.tfvars -out=staging.tfplan
```

Review the saved plan before any separately authorized apply. Engineering-group
access owns routine infrastructure administration and state access. Project IAM,
custom-role changes and billing administration remain with Dev. The administrator
bootstrap installs conditional database IAM grants before databases exist, so
later database activation does not itself need another project-IAM grant.

Engineering-group access includes Identity Platform administration for Duet's
hosted sign-in setup. Tyler's domain-mapping access concern remains unverified;
this root does not create domain mappings. The foundation plan was verified under
Dev's account; an infrastructure plan and deployment from Tyler's account remain
unverified.

The foundation comprises the private network and SQL peering, five service
accounts, GitHub WIF, immutable application registry, four empty Doppler bootstrap
secret slots with separate readers, CI grants and 30-day logs. Dev owns the
separate `coda-rust-stg` $10 monthly budget alert in Cloud Billing, using the
existing billing recipients; it is not a spending cap. Budgets are deliberately
outside this state so an engineering plan does not need billing-account access.
The committed staging inputs enable a staging-preview: two `db-f1-micro` SQL
instances (`rust-physician`, `rust-duet`, private IP only), both migration jobs
and the `physician` and `duet` Cloud Run services behind direct IAP for
`iap_group`. Each service reaches Ready only through its `/readyz` startup probe
against its own database.

Activating it took these operator steps, in order:

1. Create the per-role Doppler configs `stg_physician_runtime`,
   `stg_physician_migration`, `stg_duet_runtime`, `stg_duet_migration` and
   `stg_sql_admin`, each with a generated `DB_PASSWORD`. Plans read the four
   application passwords into `TF_VAR_database_passwords`; no value is committed.
   Database-enabled plans reject missing, blank or former placeholder values.
   Foundation-only plans still need no database credentials. Password rotation
   remains explicit through `database_password_versions`; removing the fallback
   does not rotate an already deployed password.
2. Apply the databases, then set the `postgres` password from `stg_sql_admin`.
3. Run [`bootstrap-grants.sql`](/infra/gcp/rust-environments/bootstrap-grants.sql)
   once per database from a one-off `postgres:17` Cloud Run job on `rust-private`,
   with the admin password mounted from a temporary secret. Before the script,
   revoke `cloudsqlsuperuser` from both application users (Cloud SQL grants it to
   API-created users) and grant the migration user to the administrator so it
   can transfer schema ownership. Delete the job and temporary secret afterwards.
4. Write the database URLs into the four role configs, mint a read-only Doppler
   service token per config into the matching `*-doppler-token` secret, and
   record the versions in `doppler_token_versions`.
5. Apply the migration jobs, execute both (a second execution is a no-op), then
   apply the services.

The initial preview uses proof-token delivery until physician mail is configured
as below. Staging physician email sign-in was qualified on October 24, 2026;
Duet identity and the full P1/D1 journeys remain outside that result, so this is
not complete hosted acceptance.

### Staging sign-in accounts

`physician-migrate` ensures the accounts listed in `PHYSICIAN_STAGING_ACCOUNTS`
(`email|Display Name|admin` or `…|physician`, comma-separated) exist in
`roger-synthetic-practice` with `physician_id = staging-physician`, the panel the
server seeds with the five synthetic workspaces on startup. The list and
`CODA_HOSTED_PROFILE=staging-preview` come from `migration_environment` in
[`env/staging.tfvars`](/infra/gcp/rust-environments/env/staging.tfvars); any
other profile refuses the list. A rerun never reactivates or edits an existing
account, so an administrator's deactivation sticks. This is the staging
physician-account bootstrap. `/auth/invitations` invites roster patients to
Duet; it does not create physician accounts.

Sign-in and invitation mail uses the shared [`gmail`](/crates/gmail/src/lib.rs)
transport. The Workspace relay option uses these values in
`stg_physician_runtime`:

```text
PHYSICIAN_MAIL_TRANSPORT=smtp-relay
PHYSICIAN_MAIL_FROM=signin@codahealth.ai
PHYSICIAN_PUBLIC_ORIGIN=https://staging.codahealth.ai/physician
```

The sender is `Coda Health <signin@codahealth.ai>`. It requires no mailbox, Gmail
settings change, OAuth grant, or SMTP password. Delivery connects to
`smtp-relay.gmail.com:587` with mandatory STARTTLS. A configured but invalid
transport stays in mailed mode: the sign-in API does not return the token.
The existing Gmail API option remains available through `PHYSICIAN_GMAIL_FROM`
and its credentials. With neither transport configured, local proof mode still
returns the code.

Enable the relay in this order:

1. Review a saved plan with `enable_physician_smtp_relay=true` and obtain separate
   apply authorization. It reserves one regional IPv4 address and creates a
   Cloud Router and Cloud NAT on `rust-run`. Only the physician runtime changes
   to `ALL_TRAFFIC`; other workloads keep private-range egress. The release
   contract carries that choice so later image releases preserve it.
2. In Workspace Admin, add an SMTP relay rule for domain-owned addresses,
   authenticated by only the exact `physician_mail_egress_ip` output, with TLS
   required. Do not enable SMTP authentication or add a Workspace user.
3. Deploy the relay-capable image through the reviewed staging release path,
   then set the three Doppler values above and start a revision that reads them.
4. Send an explicitly approved test message and verify recipient delivery, the
   branded sender, sign-in, reload, sign-out, and rejection of a reused link.

Sign-in links open the mounted physician root with the email and code. The
client removes both from the address bar, redeems the single-use token
automatically, and opens Today after the server verifies it. The email contains
one Sign in button and an equivalent plain-text link; there is no code field or
second confirmation. Used or expired links offer a new link. The
[sign-in graph](/verification/physician-experience/sign-in.mmd) maps entry and
recovery; in-workspace session expiry remains a separate behavior.

Local browser tests exercise the actual HTML email button through the
public-site proxy using a stand-in mail service. Live staging qualification
separately checks delivery to the approved mailbox, automatic sign-in, reload,
sign-out and used-link refusal. Invitation recipient navigation has separate
front-door acceptance work. IAP still limits the site to `iap_group`.

Cloud NAT adds an IP-hour charge, gateway usage, data processing, and applicable
network egress; see [Google's current pricing](https://cloud.google.com/nat/pricing).
The relay is disabled by default and this configuration does not itself prove
staging activation.

### Direct staging rollout

The `deploy-staging` job in [`.github/workflows/ci.yml`](/.github/workflows/ci.yml)
runs on pushes to `main`, after every check job, when the repository variable
`RUST_STAGING_DEPLOY` is `enabled`. It authenticates through the GitHub
`staging` environment's `GCP_WORKLOAD_IDENTITY_PROVIDER` and
`GCP_DEPLOYER_SERVICE_ACCOUNT` (with `GCP_PROJECT_ID` and `GCP_REGION`), then:

1. downloads the successful `images` job's tested archives by immutable artifact
   ID from the same workflow run. [`staging_images.py`](/scripts/staging_images.py)
   checks the complete manifest, source revision and archive checksums before
   cloud authentication. It pushes those archives to `apps/` tagged with the
   commit, checks that each published image's configuration digest (including
   its uncompressed layer identities) matches the tested image, and records the
   registry digest. It never rebuilds an image;
2. for `physician`, then `duet`: updates `<app>-migrate` to the migration digest
   and executes it to completion, then updates the `<app>` service to the runtime
   digest and requires its latest Ready revision to serve that digest with all
   traffic;
3. updates `physician-worker` to its digest, with the same Ready-revision check,
   once that service exists. The first push, before the scaling plan creates the
   service, still publishes the digest for `images.physician.worker` and leaves
   the service alone;
4. uploads a receipt (tested-archive manifest, digests, executions, revisions) as the
   `release-receipt-*` artifact.

The tested archive artifact is produced only while staging deployment is enabled
and retained for three days. A failed-job rerun uses
the original successful image job's artifact ID; rerunning the image job creates
a new artifact for that attempt. A missing, expired or invalid artifact stops the
release; rerun the image proof to produce a new handoff. PR image proofs and their
separate operator qualification artifacts retain their existing behavior.

Cloud Run moves traffic only to a revision that passes its `/readyz` startup
probe, so a failed rollout leaves the previous revision serving; fix forward.
OpenTofu ignores the image fields, so a later saved-plan apply does not roll
the services back. Unsetting the variable stops staging deploys without editing
the workflow:

```bash
gh variable set RUST_STAGING_DEPLOY --body enabled   # on
gh variable delete RUST_STAGING_DEPLOY               # off
```

This direct rollout stands in for the
[`release.py`](/scripts/gcp/release.py) evidence engine, which replaces it once
the producers below exist. It does not establish complete hosted acceptance;
the work below remains outstanding:

- **Hosted identity:** physician email sign-in through the public `/physician/`
  mount has live mailbox, reload, sign-out and used-link evidence. Invitation
  recipient acceptance and in-workspace session-expiry recovery remain separate.
  Duet web requires a registered Apple
  Services ID/return URL, its SDK, Identity Platform configuration and the public
  build-time settings consumed by [`browser_identity.rs`](/crates/duet-app/src/browser_identity.rs)
  and [`account.rs`](/crates/duet-app/src/account.rs); the current Nix image builder
  does not supply those settings.
- **Runtime and data acceptance:** the SQL instances, grants, migrations, Duet
  privacy journal and four Doppler configs/tokens are in place for the preview.
  Still verify private TLS, account isolation, retained conversations and the
  runtime's actual model/pool limits before accepting the activation gate.
- **Release evidence:** the workflow does not produce the immutable
  build/runtime/compatibility manifests required by
  [`release_commands.py`](/scripts/gcp/release_commands.py), nor configure its
  deployer/verifier/smoke identities. Its region/token environment and `ci` job
  lookup also need correction. The runner requires previous-release evidence,
  so the first-release bootstrap must be explicitly implemented and verified;
  do not fabricate a previous release or weaken the evidence checks.

These partial checks do not establish complete hosted acceptance. The
preview services have IAP-gated `run.app` URLs only; no custom domain points at
them.

### Physician scaling (Redis fan-out and Pub/Sub chat turns)

`enable_physician_scale` lets the physician site run on more than one instance. It takes effect only
together with `enable_workloads`, and the committed staging inputs turn it on.
Everything it creates belongs to physician; Duet gets no cache, topic, worker,
grant or setting, and the root's tests check this.

- [`cache.tf`](/infra/gcp/rust-environments/cache.tf) creates the Memorystore
  instance `rust-physician-cache`: Basic tier, 1 GB, Redis 7.2, AUTH and TLS
  (`SERVER_AUTHENTICATION`). It sits on the existing `rust-sql-private` peering,
  so it has no public endpoint. It carries `workspace_changed` and
  `workspace_settled` between instances. Clients use `fred` with TLS and AUTH
  (`PHYSICIAN_REDIS_URL`, `PHYSICIAN_REDIS_CA_PEM`). Lag or a reconnect is a
  `resync`, and the page refetches. Sign-in and panel notices stay on the
  Postgres `physician_authority` triggers.
- [`queue.tf`](/infra/gcp/rust-environments/queue.tf) creates the topics
  `physician-chat-turns` and `physician-chat-turns-dead-letter`. Each message is
  `{scope, input_id}`. A push subscription delivers each turn to the worker
  with an OIDC token for the `physician-pubsub-push` service account. The ack
  deadline is 600 s and retries back off from 10 s to 600 s. After 5 failed
  deliveries a message goes to the
  dead-letter topic, which has a pull subscription of the same name for
  inspection. Only the physician runtime identity may publish turns. The
  Pub/Sub service agent may dead-letter, and it may mint tokens for the push
  account and no other.
- [`workloads.tf`](/infra/gcp/rust-environments/workloads.tf) adds the
  `physician-worker` Cloud Run service. It has internal ingress and no IAP, IAM
  invoker checks stay on, and `physician-pubsub-push` is its only invoker.
  Pub/Sub counts as internal traffic when it calls the default `run.app` URL from
  the same project
  ([Cloud Run ingress](https://cloud.google.com/run/docs/securing/ingress)).
  The worker runs as `physician-worker-runtime`, which may reach only the
  physician SQL instance. It reads the `physician-runtime-doppler-token` slot
  and runs on `stg_physician_runtime`, because it needs exactly those secrets;
  a second token on the same config would add a rotation step without narrowing
  anything. It is told its OIDC audience (the deterministic
  `https://physician-worker-PROJECT_NUMBER.REGION.run.app`) and the push account
  as `PHYSICIAN_WORKER_AUDIENCE` and `PHYSICIAN_PUSH_SERVICE_ACCOUNT`. It keeps
  request-based CPU, because every turn runs inside the push request it
  answers. On each push it checks the token's audience and the
  `physician-pubsub-push` email, claims the leased turn in Postgres, runs the
  model, commits, and publishes `workspace_changed`.
- The physician site gets `PHYSICIAN_CHAT_TOPIC`, `cpu_idle = false` and one
  minimum instance. CPU stays allocated because
  its Postgres listener and Redis subscriber must keep running between requests.
  The minimum instance keeps the recovery sweep running: it republishes, every
  10 s, an accepted turn whose Pub/Sub publish failed. At zero instances that
  turn would wait until unrelated traffic started the site again.
  Duet keeps `cpu_idle = true`. `runtime_environment` may not set any of the
  scaling keys, and Duet may not set any `PHYSICIAN_*` key.

`PHYSICIAN_REDIS_URL` embeds the AUTH string, so it lives only in Doppler, as
does `PHYSICIAN_REDIS_CA_PEM`. Neither value is committed.

Merge the code before applying the plan. That order is safe: without
`PHYSICIAN_CHAT_TOPIC` the site starts, logs a warning and runs chat inline, as
it did before the worker. Without `PHYSICIAN_REDIS_URL` it keeps the in-process
bus. Applying the plan sets the topic on the service, and from then on the site
queues turns and reply revisions for the worker. Reply drafts live in
Postgres; see [Physician site](physician-site.md) for how a revision is
accepted, retried and fenced.

Activation, in order, from an authorized engineering account. Engineering
cannot apply this plan alone; the exact commands Dev runs, including the IAM
and budget steps, are in [Apply commands for Dev](#apply-commands-for-dev).

1. **Publish the worker image.** The initial `coda-physician-worker` digest is
   already pinned as `images.physician.worker` in
   [`env/staging.tfvars`](/infra/gcp/rust-environments/env/staging.tfvars). The
   plan refuses to run without a digest in that field. Later `deploy-staging`
   pushes replace the running image; OpenTofu ignores that field after the
   service exists.
2. **Plan, review, apply.** Run the saved plan from
   [the bootstrap section](#rust-staging-bootstrap-and-deployment-gate) and
   review it. It should create the cache, both topics, both subscriptions, the
   push and worker identities with their grants, and the worker. It should also
   update the physician service in place (`cpu_idle`, `min_instance_count = 1`,
   `PHYSICIAN_CHAT_TOPIC`),
   add the worker to the `physician-runtime` token reader and enable
   `redis.googleapis.com` and `pubsub.googleapis.com`. It must not touch Duet.
   Apply exactly that file:

   ```bash
   nix develop .#release --command tofu -chdir="$root" apply staging.tfplan
   ```

   Creating Memorystore takes several minutes. Until step 4, the site already
   queues turns, but the worker's events reach other instances only through
   Redis.
3. **Write Redis into Doppler.** Read the AUTH string straight into Doppler; it
   never appears on screen, in argv or in a file. The CA bundle is public.

   ```bash
   tf() { nix develop .#release --command tofu -chdir="$root" "$@"; }
   host=$(tf output -json physician_scale | jq -r .redis_host)
   port=$(tf output -json physician_scale | jq -r .redis_port)
   { printf 'rediss://:'; tf output -raw physician_redis_auth_string; printf '@%s:%s' "$host" "$port"; } |
     doppler secrets set PHYSICIAN_REDIS_URL --project coda-rust --config stg_physician_runtime --silent
   tf output -raw physician_redis_ca_pem |
     doppler secrets set PHYSICIAN_REDIS_CA_PEM --project coda-rust --config stg_physician_runtime --silent
   ```

   The worker runs on the same config and token, so it needs no new Doppler
   token and `doppler_token_versions` does not change. Run step 3 again if the
   instance is ever replaced, because its host, AUTH string and CA change.
4. **Deploy.** The runtimes read Doppler only at process start. Re-run
   `deploy-staging` on `main`, or push to `main`, so the site and the worker
   start revisions with the new values. From then on, the release updates the
   worker image the same way it updates the other services.
5. **Verify.**
   - Open the site in two tabs, and keep opening tabs until the
     `labels.instanceId` of the stream requests in the `physician` logs shows two
     instances. Each SSE stream holds one of an instance's 8 request slots.
     Send a turn from one tab; the reply must appear in a tab on the other
     instance.
   - In Cloud Monitoring, check these metrics:
     - `physician-chat-turns-worker`: `subscription/push_request_count` grouped
       by `response_class` (successes),
       `subscription/oldest_unacked_message_age` (near zero) and
       `subscription/dead_letter_message_count` (zero).
     - `rust-physician-cache`: `redis.googleapis.com/clients/connected`, one
       subscriber per site instance plus the workers.
     - `run.googleapis.com/container/instance_count` for `physician` and
       `physician-worker`.
   - To inspect dead letters without acknowledging them, run
     `gcloud pubsub subscriptions pull physician-chat-turns-dead-letter --project coda-rust-staging --limit 10`.

Workstations use `valkey` and the Pub/Sub emulator from `nix develop`. Unset
`PHYSICIAN_REDIS_URL` keeps the in-process bus. Tests start the emulator through
the shared harness in [`crates/pubsub`](/crates/pubsub/src/testing.rs)
(`test-util` feature), which reuses a loopback `PUBSUB_EMULATOR_HOST` when one is
set. [`crates/gmail/tests/push_wake.rs`](/crates/gmail/tests/push_wake.rs) uses it to
prove the Gmail push-wake contract: a push only wakes `history.list` from the
stored cursor, under duplicates, reordering, redelivery, dead letters and
ordering keys. Push OIDC and real Gmail stay unproven there.

To turn scaling off, set `enable_physician_scale = false` and apply a reviewed
saved plan. The plan removes the cache, topics, subscriptions, and worker.
The physician site goes back to a minimum of zero instances. It keeps
`cpu_idle = false`, because its Postgres authority
listener still runs between requests. Then delete the two Doppler keys and
redeploy.

#### Apply commands for Dev

**Status, October 26, 2026.** Dev provisioned the infrastructure as the project
Owner while leaving the physician site's queue activation pending. Redis, both
topics and subscriptions, and `physician-worker` exist; the Redis URL and CA are in Doppler `stg_physician_runtime`; the worker is Ready;
the budget alert is $200/month. The remaining activation is:

- The physician site keeps one instance while scaling is on
  (`min_instance_count` 0 to 1), so its recovery sweep always runs. After the
  stack merges and `deploy-staging` is green, run steps 4 to 6 once more. The
  saved plan should update `google_cloud_run_v2_service.app["physician"]`
  in place: `min_instance_count` 0 to 1, `cpu_idle` true to false, and add
  `PHYSICIAN_CHAT_TOPIC`. The readback at 05:09 UTC still had minimum zero,
  CPU throttling enabled, and no topic. Review every changed field in a fresh
  saved plan; expect nothing added or destroyed and no Duet changes. The minimum
  instance costs about $48/month after the free tier (see [Cost](#cost)).
- The worker image now sets `PHYSICIAN_AUTH=required`. The same key in
  `stg_physician_runtime` becomes redundant and harmless; it can stay.

The first-time steps below are kept for a fresh project.

The plan creates IAM bindings that `group:gcp-engineering@codahealth.ai`
cannot write on `coda-rust-staging`. Checked with `testIamPermissions` on
October 25, 2026, engineering lacks:

| Permission | Needed for |
| --- | --- |
| `resourcemanager.projects.setIamPolicy` | `google_project_iam_member.physician_worker_sql`: `roles/cloudsql.client` and `roles/cloudsql.instanceUser` for `physician-worker-runtime`, each conditioned to the `rust-physician` instance |
| `pubsub.topics.getIamPolicy`, `pubsub.topics.setIamPolicy` | the publisher bindings on `physician-chat-turns` (physician runtime) and `physician-chat-turns-dead-letter` (Pub/Sub service agent) |
| `pubsub.subscriptions.getIamPolicy`, `pubsub.subscriptions.setIamPolicy` | the service agent's subscriber binding on `physician-chat-turns-worker`, which dead-lettering needs |

Everything else in the plan (Memorystore, topics, subscriptions, service
accounts, the worker service and its invoker binding, the Doppler token
reader, the two APIs) is within engineering's roles. Do not apply part of it:
the physician service would get `PHYSICIAN_CHAT_TOPIC` without permission to
publish, and Memorystore would be billed. The budget alert also needs Cloud
Billing access that engineering does not have.

Run everything below in `nix develop` from an up-to-date `main` checkout. That
shell has `gcloud`, `tofu`, `doppler` and `jq`.

1. **Confirm the code is live.** The site that queues turns and
   `physician-worker` must be on `main`, and the latest `deploy-staging` run on
   `main` must be green. Applying before that sets the topic on a site that does
   not read it.

   ```bash
   gh run list --workflow ci.yml --branch main --event push --limit 1 \
     --json headSha,conclusion,url
   ```

2. **Raise the budget alert** (billing administrator). Expected fixed spend
   after this plan is about $55/month, plus up to about $105/month if both
   `physician` instances stay up (see [Cost](#cost)). Pick the new amount; 150 is
   an example.

   ```bash
   billing=01691B-49BADA-F3D54D
   gcloud billing budgets list --billing-account="$billing" \
     --format='table(name.basename(),displayName,amount.specifiedAmount.units)'
   gcloud billing budgets update BUDGET_ID --billing-account="$billing" \
     --budget-amount=150.00
   ```

3. **Decide who applies.** Either Dev applies as `dev@codahealth.ai`, the
   project Owner, which needs no new grants (skip to step 4), or Dev grants
   engineering the missing permissions and engineering applies. The narrow grant
   is Pub/Sub Admin plus Project IAM Admin that may only add or remove the two
   Cloud SQL roles:

   ```bash
   project=coda-rust-staging
   group=group:gcp-engineering@codahealth.ai
   cat > /tmp/only-cloudsql-worker-grants.yaml <<'YAML'
   title: only-cloudsql-worker-grants
   description: May add or remove only the worker's Cloud SQL roles
   expression: >-
     api.getAttribute('iam.googleapis.com/modifiedGrantsByRole', [])
     .hasOnly(['roles/cloudsql.client', 'roles/cloudsql.instanceUser'])
   YAML
   gcloud projects add-iam-policy-binding "$project" --member="$group" \
     --role=roles/pubsub.admin --condition=None
   gcloud projects add-iam-policy-binding "$project" --member="$group" \
     --role=roles/resourcemanager.projectIamAdmin \
     --condition-from-file=/tmp/only-cloudsql-worker-grants.yaml
   ```

   Remove both after the apply (step 8) unless they should stay.

4. **Plan.** Sign in as the account that applies. The database passwords are
   write-only inputs read from Doppler. The saved plan goes stale once
   `deploy-staging` or another apply touches state, so plan right before
   applying.

   ```bash
   gcloud auth login   # dev@codahealth.ai, or the engineering account after step 3
   root=infra/gcp/rust-environments
   export GOOGLE_OAUTH_ACCESS_TOKEN=$(gcloud auth print-access-token)
   pw() { doppler secrets get DB_PASSWORD --project coda-rust --config "stg_$1" --plain; }
   export TF_VAR_database_passwords=$(jq -nc \
     --arg a "$(pw physician_runtime)" --arg b "$(pw physician_migration)" \
     --arg c "$(pw duet_runtime)" --arg d "$(pw duet_migration)" \
     '{"physician-runtime":$a,"physician-migration":$b,"duet-runtime":$c,"duet-migration":$d}')
   tofu -chdir="$root" init -input=false -reconfigure \
     -backend-config=backend/staging.gcs.tfbackend
   tofu -chdir="$root" plan -input=false -var-file=env/staging.tfvars \
     -out=staging.tfplan
   ```

5. **Review the saved plan.** On October 25, 2026 it was 20 to add, 2 to
   change, 0 to destroy. The additions are the cache; the two topics, the two
   subscriptions and their three bindings; the push and worker service accounts,
   the push token grant and the worker's two Cloud SQL grants; the worker, its
   invoker and its two release grants; the Pub/Sub service identity; and the
   `redis` and `pubsub` APIs. The two in-place changes are the
   `physician` service (`cpu_idle` true to false, `min_instance_count` 0 to 1,
   adds `PHYSICIAN_CHAT_TOPIC`, image unchanged) and the `physician-runtime`
   Doppler-token reader (adds
   `physician-worker-runtime`). Nothing may name Duet and nothing may be
   destroyed:

   ```bash
   tofu -chdir="$root" show -json staging.tfplan | jq -r '.resource_changes[]
     | select(.change.actions != ["no-op"])
     | "\(.change.actions | join(","))\t\(.address)"'
   tofu -chdir="$root" show -json staging.tfplan | jq -e '[.resource_changes[]
     | select(.change.actions != ["no-op"])
     | select((.address | test("duet")) or (.change.actions | index("delete")))]
     | length == 0'
   ```

6. **Apply exactly that file.** Memorystore takes several minutes. The access
   token lasts an hour; if the apply outlives it, export a fresh one and
   re-plan.

   ```bash
   tofu -chdir="$root" apply staging.tfplan
   ```

7. **Write Redis into Doppler and restart.** The AUTH string goes straight from
   `tofu output` into Doppler and never reaches the screen, argv or a file.
   Runtimes read Doppler only at process start, so start new revisions.
   `--revision-suffix` changes only the revision name, which OpenTofu ignores
   on both services.

   ```bash
   tf() { tofu -chdir="$root" "$@"; }
   host=$(tf output -json physician_scale | jq -r .redis_host)
   port=$(tf output -json physician_scale | jq -r .redis_port)
   { printf 'rediss://:'; tf output -raw physician_redis_auth_string; printf '@%s:%s' "$host" "$port"; } |
     doppler secrets set PHYSICIAN_REDIS_URL --project coda-rust --config stg_physician_runtime --silent
   tf output -raw physician_redis_ca_pem |
     doppler secrets set PHYSICIAN_REDIS_CA_PEM --project coda-rust --config stg_physician_runtime --silent
   suffix=redis-$(date +%s)
   for service in physician physician-worker; do
     gcloud run services update "$service" --project coda-rust-staging \
       --region us-east1 --revision-suffix="$suffix" --quiet
   done
   ```

8. **Check, then remove any temporary grants.** Each describe must show a
   Ready revision ending in the suffix from step 7; the subscription must push
   to the worker with 5 delivery attempts; the cache must be `READY` with AUTH
   and `SERVER_AUTHENTICATION`.

   ```bash
   p=(--project coda-rust-staging)
   for service in physician physician-worker; do
     gcloud run services describe "$service" "${p[@]}" --region us-east1 \
       --format='value(metadata.name,status.latestReadyRevisionName)'
   done
   gcloud pubsub subscriptions describe physician-chat-turns-worker "${p[@]}" \
     --format='value(pushConfig.pushEndpoint,deadLetterPolicy.maxDeliveryAttempts)'
   gcloud redis instances describe rust-physician-cache "${p[@]}" --region us-east1 \
     --format='value(state,authEnabled,transitEncryptionMode)'
   # Only if step 3 granted engineering:
   gcloud projects remove-iam-policy-binding coda-rust-staging \
     --member=group:gcp-engineering@codahealth.ai --role=roles/pubsub.admin --condition=None
   gcloud projects remove-iam-policy-binding coda-rust-staging \
     --member=group:gcp-engineering@codahealth.ai --role=roles/resourcemanager.projectIamAdmin \
     --condition-from-file=/tmp/only-cloudsql-worker-grants.yaml
   ```

   Then run the two-tab and Cloud Monitoring checks in step 5 of the activation
   list above.

#### Cost

These are list prices for `us-east1` in October 2026. The `coda-rust-stg` budget
is an alert, not a spending cap. Dev raised it from $10 to $200/month on
October 25, 2026, which covers this gate. Raise
that alert before applying. Nothing is applied without a reviewed saved plan.

| Item | Billing | Estimate |
| --- | --- | --- |
| Memorystore Basic M1, 1 GB | $0.049 per GB-hour, always billed while the instance exists ([pricing](https://cloud.google.com/memorystore/docs/redis/pricing)) | about **$35/month** |
| Pub/Sub topics and subscriptions | Throughput billing with a monthly free tier; each turn message is a few hundred bytes (`{scope, input_id}`) | negligible, $0 at staging volume |
| `physician` with `cpu_idle = false` (1 vCPU, 1 GiB, min 1, max 2) | Instance-based: $0.000018 per vCPU-second plus $0.000002 per GiB-second for the whole time an instance is up ([pricing](https://cloud.google.com/run/pricing)) | $0.072 per instance-hour. The minimum instance is up all the time: about **$53/month**, less the free tier of 240,000 vCPU-seconds and 450,000 GiB-seconds per billing account (about $5). A second instance at the cap of 2 adds the same rate while it is up. |
| `physician-worker` (`cpu_idle = true`) | Request-based, billed only while a turn runs | cents per month at staging volume; model tokens dominate |

The two `db-f1-micro` SQL instances already cost about $20/month. With
Memorystore and the minimum physician instance, staging's fixed spend is about
$105/month before the worker and any second site instance.

## Release procedure

- <a href="/submodules/coda-app/infra/gcp/README%2Emd#environments">Environments</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#branch-rehearsal">Branch rehearsal</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#staging-window">Staging window</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#release-runner">Release runner</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#one-time-staging-provider-admission">One-time staging provider admission</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#compatibility-checked-rollback">Compatibility-checked rollback</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#staging-identity-setup-and-deletion-proof">Staging identity setup and deletion proof</a>

## Operations

- <a href="/submodules/coda-app/infra/gcp/README%2Emd#monitoring">Monitoring</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#required-services">Required services</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#current-launch-guardrails">Current launch guardrails</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#doctor-page-cutover">Doctor-page cutover</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#worker-queue-capacity">Worker queue capacity</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#continuation-preparation-queue">Continuation preparation queue</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#scheduled-conversation-queue">Scheduled conversation queue</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#follow-up-and-review-wake-job">Follow-up and review wake job</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#acquisition-worker-operations">Acquisition worker operations</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#secrets-and-runtime-settings">Secrets and runtime settings</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#minimum-supported-iphone-build">Minimum supported iPhone build</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#sign-out-everywhere-readback">Sign out everywhere readback</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#model-cost-controls">Model cost controls</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#order">Order</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#removed-runtime-decommission">Removed runtime decommission</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#access-and-security-operations">Access and security operations</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#cloud-sql-restore-drill">Cloud SQL restore drill</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#privacy-reconciliation-before-recovery-serves-traffic">Privacy reconciliation before recovery serves traffic</a>

## Dated release and security evidence

Source: <a href="/submodules/coda-app/infra/gcp/README%2Emd#dated-release-and-security-evidence"><code>Dated release and security evidence</code></a>.

- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-5-2026-spend-records-and-reviewed-doctor-page-release">October 5, 2026 spend records and reviewed doctor page release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-5-2026-structural-cleanup-release">October 5, 2026 structural cleanup release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-3-2026-wave-1-and-genome-removal-release">October 3, 2026 Wave 1 and Genome removal release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-2-2026-patient-isolation-and-graceful-runs-release">October 2, 2026 patient isolation and graceful runs release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-1-2026-security-and-build-133-release">October 1, 2026 security and build 133 release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#october-1-2026-doctors--hospitals-release">October 1, 2026 Doctors &amp; hospitals release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#september-30-2026-minimum-build-131-release">September 30, 2026 minimum build 131 release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#september-30-2026-companion-release">September 30, 2026 companion release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#september-27-2026-first-session-recovery-release">September 27, 2026 first-session recovery release</a>
- <a href="/submodules/coda-app/infra/gcp/README%2Emd#earlier-releases">Earlier releases</a>


## Shared mail service

Products hosted outside the Rust VPC can use the `mail-relay` binary in `crates/gmail`.
The mail-only <a href="/infra/gcp/shared-mail/README%2Emd">production root</a> owns its separate
deployment in `coda-health-prod`; `rust-environments` continues to own staging.
It wraps the existing `SmtpRelay`; it does not own product sign-in, message templates,
recipients, send records or retry decisions. The physician server continues to use
that same transport directly. There is one SMTP implementation and no additional
mail vendor, mailbox credential, queue or persistence layer.

Build `crates/gmail/Dockerfile` from the workspace root for `linux/amd64`. It follows
the existing private, digest-pinned base-image convention and builds only
`cargo build --locked --release -p gmail --bin mail-relay`. Build/push and infrastructure
apply require the normal release approvals. This addition is not wired into the
physician release pipeline; configure its reviewed image digest explicitly.

In `infra/gcp/rust-environments`, `mail_relay_image` defaults to null (no service).
Setting a reviewed digest and nonempty `mail_relay_invokers` creates an IAM-protected
Cloud Run service, a runtime identity with no application-data roles, and a
service-scoped invoker binding. `enable_physician_smtp_relay` must already be enabled:
the adapter uses that existing network, subnet and NAT address, preserving the
Workspace IP allowlist. It needs no Doppler token. Runtime settings are
`MAIL_FROM=signin@codahealth.ai`, `MAIL_NOTIFICATION_FROM=notifications@codahealth.ai`
(the binary default), and `MAIL_LISTEN_ADDR=0.0.0.0:8080`.

For Gym staging, the named caller is
`serviceAccount:workspace-web-staging@coda-gym.iam.gserviceaccount.com`.
Production uses `serviceAccount:workspace-web@coda-gym.iam.gserviceaccount.com`
on its own relay and authorized egress, managed by `infra/gcp/shared-mail` in a
separate state. Production must not be planned or applied from the Rust staging
root. Neither root grants a public invoker or routes production through staging.
The `mail_relay_url` output is both the caller's
base URL and ID-token audience. Cloud Run IAM checks identity; the adapter does
not add a second token verifier.

`POST /send` accepts JSON `{purpose, to, subject, text, html?}`. `purpose` is
`signin` or `notification`; the service maps it to its two configured identities.
Callers cannot supply an arbitrary From address. Omitted purpose retains the sign-in
sender for older callers during relay-first deployment; unknown purposes are refused
before SMTP. Qualify a new image on staging before separately approving production.
HTTP 204 means the SMTP server accepted the message, not that a mailbox received it.
HTTP 422 means invalid headers or an explicit SMTP refusal. HTTP 503 means acceptance
could not be established; a caller must not assume it is safe to retry. No message
content, code, bearer token or SMTP response text is returned or logged by the adapter.
`GET /readyz` returning 204 checks only process readiness, not SMTP or delivery.

Before cutover, read back the image digest, IAM enforcement, named invoker, runtime
identity, ALL_TRAFFIC routing and the existing NAT address. Check authenticated
readiness from the actual calling runtime and denial without a token. Then, under
explicit approval of the exact email and recipient, verify receipt and sender in the
mailbox and the owning product's completed workflow. Preserve the old credential
until that proof is complete; deployment alone is not delivery proof. Gym owns its
cutover and cleanup steps in `authoring/web/infra/README.md` in the coda-gym repository.
