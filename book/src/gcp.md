# GCP deployment

Duet production is still the Python service released from coda-app; this workspace does not deploy it. The physician OpenTofu roots in this workspace are [`foundation`](/infra/gcp/foundation), <a href="/infra/gcp/staging/README%2Emd"><code>staging</code></a>, <a href="/infra/gcp/clinical-evidence/README%2Emd"><code>clinical-evidence</code></a>, and <a href="/infra/gcp/physician-runtime/README%2Emd"><code>physician-runtime</code></a>. The last one is the GKE Autopilot and Cloud SQL target for `physician-sample` and `physician-auth`. This workspace does not apply those reference roots. Public-site staging and production (the marketing site only) and the isolated Rust staging root are workspace-owned and require separately authorized saved-plan applies. [Build, CI, and release](release.md#retained-physician-infrastructure-references) says what each root owns.

The mail-only <a href="/infra/gcp/shared-mail/README%2Emd">shared-mail root</a> is owned here
for production SMTP in `coda-health-prod`, with separate state and explicit
saved-plan approval. This exception grants no authority over the referenced
production application or clinical-evidence roots.

## Public marketing site

The separate [`infra/gcp/public-site`](/infra/gcp/public-site) root runs the
stateless Rust marketing server on Cloud Run. The site belongs to the company,
not to a product, so both environments live in the Coda Health projects:
`staging.codahealth.ai` behind direct Cloud Run IAP in `coda-health-staging`, and
`codahealth.ai` and `www.codahealth.ai` behind a load balancer and Cloud CDN in
`coda-health-prod`, apart from the physician and Duet projects such as
`coda-rust-staging`. The root shares no physician/Duet databases. CI deploys
staging from `main`; production receives staging-tested images through manual
promotion, and operator applies need Dev's explicit authorization. [Public
site](public-site.md) owns the architecture, IAP and cache controls, runtime
configuration, checks, and the
[staging](public-site.md#staging-state-and-deployment) and
[production](public-site.md#production-state-and-deployment) procedures.

## Rust container images

`nix/packages.nix` builds the Rust Cloud Run images from `Cargo.lock` and `flake.lock`. Each image tag is the git revision, and the build sets `created` to the epoch, so a revision always produces the same archive.

| Flake output | Image | Role | Entrypoint binary |
| --- | --- | --- | --- |
| `physician-sample-image` | `coda-physician-sample` | runtime | `physician-sample` serving `PHYSICIAN_SITE_DIR` (Dioxus WASM bundle) |
| `physician-worker-image` | `coda-physician-worker` | runtime | `physician-worker`, the Pub/Sub chat-turn worker, loading the skill catalog at `PHYSICIAN_SKILLS_DIR` |
| `duet-api-image` | `coda-duet-api` | runtime | `duet-api` serving `DUET_SITE_DIR` (Dioxus WASM bundle) |
| `physician-migration-image` | `coda-physician-migrate` | migration | `physician-migrate`: physician-auth, front-door, then runtime chains; packaged synthetic charts for explicit `load-charts` |
| `duet-migration-image` | `coda-duet-migrate` | migration | `duet-migrate`: Duet-owned chain and runtime grants, then optional front-door chain |
| `duet-privacy-delivery-image` | `coda-duet-privacy-delivery` | job | `duet-privacy-delivery`, a bounded Luna receipt job |

- The runtime servers never migrate. They bind `0.0.0.0:$PORT`, where `PORT` defaults to 8080. `/health` reports liveness. Hosted startup accepts only the explicit `stg` / `staging-preview` profile with its required stores and identity configuration; other hosted profiles refuse to start. `/readyz` checks the configured schemas. The startup owners are [`physician-sample/src/hosting.rs`](/crates/physician-sample/src/hosting.rs) and [`duet-api/src/main.rs`](/crates/duet-api/src/main.rs).
- Each image contains the binary and the libraries it links, Doppler, CA roots, tzdata and a passwd entry for uid `65532`. The image has no shell or toolchain. It runs as `65532:65532` with working directory `/tmp`.
- Every entrypoint is `doppler run --no-fallback --preserve-env=PORT -- <binary>`. The Doppler CLI is the static release pinned in [`crates/workspace-tools/src/doppler.rs`](/crates/workspace-tools/src/doppler.rs). Nix reads the version and SHA-256 from that file, so the runners and the images run the same verified binary. Cloud Run mounts `DOPPLER_TOKEN` from Secret Manager. No secret is in a layer or in `Env`, and `--no-fallback` keeps secrets off the filesystem.
- `scripts/check-images.sh` (run with `nix develop .#release`) builds every image and inspects each archive. It loads them into Docker and checks rejection of missing credentials or an unconfigured hosted profile. Against disposable PostgreSQL it runs both migration images twice and explicitly imports the packaged physician chart bank twice, checking its manifest totals, zero failures and an unchanged reload. It then starts the servers to check `/health` 200, schema readiness 200, the WASM assets and a clean SIGTERM exit. The `images` CI job runs this script. Nothing is pushed.

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
hosted sign-in setup. This root does not create domain mappings.

The foundation comprises the private network and SQL peering, five service
accounts, GitHub WIF, immutable application registry, four empty Doppler bootstrap
secret slots with separate readers, CI grants and 30-day logs. Budgets sit outside
this state so an engineering plan does not need billing-account access. The
committed staging inputs enable a staging-preview: two `db-f1-micro` SQL
instances (`rust-physician`, `rust-duet`, private IP only), both migration jobs
and the `physician` and `duet` Cloud Run services behind direct IAP for
`iap_group`. Each service reaches Ready only through its `/readyz` startup probe
against its own database.

A new environment needs these operator steps, in order:

1. Create the per-role Doppler configs `stg_physician_runtime`,
   `stg_physician_migration`, `stg_duet_runtime`, `stg_duet_migration` and
   `stg_sql_admin`, each with a generated `DB_PASSWORD`. Plans read the four
   application passwords into `TF_VAR_database_passwords`; no value is committed.
   Database-enabled plans reject missing or blank values. Foundation-only plans
   need no database credentials. Password rotation is explicit through
   `database_password_versions`.
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

### Reconcile staging configuration

Use the committed `env/staging.tfvars` as the desired configuration; do not patch
Cloud Run flags by hand. With scaling enabled, physician needs minimum instances
`1`, maximum `2`, and CPU throttling `false`. With SMTP relay enabled, its VPC
egress must remain `all-traffic`. Image digests remain release-owned.

From the repository root, use the default dev shell with operator credentials.
Keep the plan and its output private: they can contain database passwords.

```bash
nix develop
# Run the remaining commands inside that shell; never enable shell tracing.
set -euo pipefail
umask 077
root=infra/gcp/rust-environments
plan_dir=$(mktemp -d)
plan="$plan_dir/staging.tfplan"
tofu -chdir="$root" init -input=false -backend-config=backend/staging.gcs.tfbackend
(
  # Fail immediately if any secret lookup fails; do not print the values.
  pr=$(doppler secrets get DB_PASSWORD --plain --project coda-rust --config stg_physician_runtime)
  pm=$(doppler secrets get DB_PASSWORD --plain --project coda-rust --config stg_physician_migration)
  dr=$(doppler secrets get DB_PASSWORD --plain --project coda-rust --config stg_duet_runtime)
  dm=$(doppler secrets get DB_PASSWORD --plain --project coda-rust --config stg_duet_migration)
  export pr pm dr dm TF_VAR_database_passwords
  TF_VAR_database_passwords=$(jq -cn \
    '{"physician-runtime":env.pr,"physician-migration":env.pm,"duet-runtime":env.dr,"duet-migration":env.dm}')
  unset pr pm dr dm
  tofu -chdir="$root" plan -input=false -var-file=env/staging.tfvars -out="$plan"
)
tofu -chdir="$root" show -no-color "$plan"
```

**STOP:** review every change and obtain operator approval before continuing.
Unexpected deletes, replacements, IAM/network changes, image changes, or new
billable resources require investigation and explicit approval. Coordinate with
releases so no deployment races the saved plan. Do not use `-target` or
`-refresh-only` to hide differences.

```bash
tofu -chdir="$root" apply "$plan"
gcloud run services describe physician --project=coda-rust-staging --region=us-east1 \
  --format='json(spec.template.metadata.annotations,status.conditions,status.latestCreatedRevisionName,status.latestReadyRevisionName,status.traffic)'
```

Require the expected settings above, Ready status, matching latest-created and
latest-ready revisions, and 100% intended traffic. Repeat the plan block: it must
report no changes. Securely dispose of the private plan when no longer needed.
After a separately approved merge and successful staging release, repeat the
readback; if settings regress, fix the release owner rather than reapplying in a
loop. A Ready revision is not database or journey qualification: verify both
migration executions and signed-in product readback; isolated restore and privacy
replay retain their separate authorization and proof requirements. A failed
browser qualification must be corrected and rerun, not waived by an infra apply.

### Staging sign-in accounts

`physician-migrate` ensures the accounts listed in `PHYSICIAN_STAGING_ACCOUNTS`
(`email|Display Name|admin` or `…|physician`, comma-separated) exist in
`roger-synthetic-practice`. Each email gets its own physician id. The list and
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
   apply authorization. It adds one regional IPv4 address, a Cloud Router and Cloud
   NAT on `rust-run`, and moves only the physician runtime to `ALL_TRAFFIC`. The
   release contract carries that choice so later image releases preserve it.
2. In Workspace Admin, add an SMTP relay rule for domain-owned addresses,
   authenticated by only the exact `physician_mail_egress_ip` output, with TLS
   required. Do not enable SMTP authentication or add a Workspace user.
3. Deploy the relay-capable image through the staging release path, then set the
   three Doppler values above and start a revision that reads them. The site builds
   its mail transport at startup.
4. Send an explicitly approved test message and verify recipient delivery, the
   branded sender, sign-in, reload, sign-out, and rejection of a reused link.

### Staging synthetic charts

The physician migration image carries the charts and manifest from the
[synthetic patient bank](patient-bank.md), built offline from accepted ledgers and
their seeds at the image's revision. `PHYSICIAN_SYNTHETIC_CHARTS` names the packaged
folder. Runtime images read the imported records from physician Postgres.

After a successful schema migration and account bootstrap, a staging operator with
`run.jobs.runWithOverrides` loads the charts explicitly. The CI release role does
not have this permission.

```bash
gcloud run jobs execute physician-migrate --project coda-rust-staging \
  --region us-east1 --args=load-charts --task-timeout=30m --wait
```

Read the summary in Cloud Logging using the execution name returned by that command:

```bash
gcloud logging read \
  'resource.type="cloud_run_job" AND resource.labels.job_name="physician-migrate" AND labels."run.googleapis.com/execution_name"="EXECUTION_NAME"' \
  --project coda-rust-staging --format='value(textPayload)'
```

The longer deadline applies only to this full-bank import execution; the job's
default schema-migration deadline stays unchanged. The execution must report the
packaged bank's patient and record totals and zero
failures. After a completed import, repeating the same image's import must report
zero changed records. A failed or timed-out import can be retried: completed
patients remain stored, and a retry can still report changes for the rest.
The command requires the `staging-preview` profile and preserves panel choices.
The execution override leaves the job's default schema-migration command
unchanged; ordinary deploys do not load charts. Verify imported patients and their
charts through the normal signed-in physician roster before treating the staging
journey as qualified.

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
3. updates `physician-worker` to its digest once that service exists, with the same
   Ready-revision check. Until then it reports the digest to record as
   `images.physician.worker` for the scaling apply;
4. uploads a receipt (tested-archive manifest, digests, executions, revisions) as the
   `release-receipt-*` artifact.

The tested archive artifact is produced only while staging deployment is enabled
and retained for three days. A failed-job rerun uses
the original successful image job's artifact ID; rerunning the image job creates
a new artifact for that attempt. A missing, expired or invalid artifact stops the
release; rerun the image proof to produce a new handoff.

Cloud Run moves traffic only to a revision that passes its `/readyz` startup
probe, so a failed rollout leaves the previous revision serving; fix forward.
OpenTofu ignores the image fields, so a later saved-plan apply does not roll
the services back. Unsetting the variable stops staging deploys without editing
the workflow:

```bash
gh variable set RUST_STAGING_DEPLOY --body enabled   # on
gh variable delete RUST_STAGING_DEPLOY               # off
```

### Physician scaling (Redis fan-out and Pub/Sub chat turns)

`enable_physician_scale` lets the physician site run on more than one instance. It takes effect only
together with `enable_workloads`. Applying it writes project and Pub/Sub IAM bindings
that engineering can't write alone. Everything it creates belongs to physician; Duet gets no cache, topic, worker,
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

To turn scaling on:

1. **Deploy the code first.** The site that queues turns and `physician-worker` must
   be on `main` with a successful `Deploy staging` job. Skills/docs-only pushes skip
   that job, so overall CI success is not deployment evidence. The plan refuses to
   run without a digest in `images.physician.worker`.
2. **Decide who applies.** The plan creates bindings `group:gcp-engineering@codahealth.ai`
   cannot write: the worker's project-level Cloud SQL roles and the IAM on the topics
   and subscriptions. Apply as the project Owner, or grant engineering Pub/Sub Admin
   plus Project IAM Admin conditioned on
   `api.getAttribute('iam.googleapis.com/modifiedGrantsByRole', []).hasOnly(['roles/cloudsql.client', 'roles/cloudsql.instanceUser'])`,
   and remove both afterwards. Do not apply part of the plan: the physician service
   would get `PHYSICIAN_CHAT_TOPIC` without permission to publish.
3. **Plan, review, apply.** Plan right before applying, because a saved plan goes
   stale once `deploy-staging` or another apply touches state. The plan must not name
   Duet or destroy anything. Apply exactly that file.
4. **Write Redis into Doppler and start new revisions,** with the commands below.
   OpenTofu doesn't write these two values. Without them each process keeps an
   in-process bus, so the worker's events never reach the site. Runtimes read Doppler
   only at process start. The AUTH string goes straight into Doppler and never reaches
   the screen, argv or a file.
5. **Verify.** Open site tabs until `labels.instanceId` in the `physician` stream logs
   shows two instances, send a turn from one tab, and check the reply appears in a tab
   on the other instance. On `physician-chat-turns-worker`, `subscription/push_request_count`
   should show successes, `subscription/oldest_unacked_message_age` stay near zero and
   `subscription/dead_letter_message_count` stay at zero.

Replacing the Memorystore instance changes its host, AUTH string and CA, so run step 4
again.

```bash
root=infra/gcp/rust-environments
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

To inspect dead letters without acknowledging them, run
`gcloud pubsub subscriptions pull physician-chat-turns-dead-letter --project coda-rust-staging --limit 10`.

To turn scaling off, set `enable_physician_scale = false` and apply a reviewed
saved plan. The plan removes the cache, topics, subscriptions, and worker.
The physician site goes back to a minimum of zero instances. It keeps
`cpu_idle = false`, because its Postgres authority
listener still runs between requests. Then delete the two Doppler keys and
redeploy.

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
apply require the normal release approvals. Configure its reviewed image digest
explicitly.

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
Callers cannot supply an arbitrary From address. Omitted purpose uses the sign-in
sender; unknown purposes are refused before SMTP.
HTTP 204 means the SMTP server accepted the message, not that a mailbox received it.
HTTP 422 means invalid headers or an explicit SMTP refusal. HTTP 503 means acceptance
could not be established; a caller must not assume it is safe to retry. No message
content, code, bearer token or SMTP response text is returned or logged by the adapter.
`GET /readyz` returning 204 checks only process readiness, not SMTP or delivery.
Gym owns its cutover and cleanup steps in `authoring/web/infra/README.md` in the coda-gym
repository.
