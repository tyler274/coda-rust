# GCP deployment

Duet production is still the Python service released from coda-app; this workspace does not deploy it. The physician OpenTofu roots in this workspace are [`foundation`](/infra/gcp/foundation), <a href="/infra/gcp/staging/README%2Emd"><code>staging</code></a>, <a href="/infra/gcp/clinical-evidence/README%2Emd"><code>clinical-evidence</code></a>, and <a href="/infra/gcp/physician-runtime/README%2Emd"><code>physician-runtime</code></a>. The last one is the GKE Autopilot and Cloud SQL target for `physician-sample` and `physician-auth`. This workspace does not apply those reference roots. Public-site staging and production (the marketing site only) and the isolated Rust staging root are workspace-owned and require separately authorized saved-plan applies. [Build, CI, and release](release.md#retained-physician-infrastructure-references) says what each root owns.

The mail-only <a href="/infra/gcp/shared-mail/README%2Emd">shared-mail root</a> is owned here
for production SMTP in `coda-health-prod`, with separate state and explicit
saved-plan approval. This exception grants no authority over the referenced
production application or clinical-evidence roots.

Staging observability spans two workspace-owned roots, applied in order by an
operator from reviewed saved plans:

| Root | Owns | State |
| --- | --- | --- |
| [`rust-environments`](/infra/gcp/rust-environments) | Behind `observability_cluster_enabled`: the private observability Autopilot cluster on `rust-private`, its subnet and secondary ranges, and the reserved internal gateway address. Behind `observability_relay_image`: the IAP-protected `observability-relay` Cloud Run service, its `rust-relay` Direct VPC egress subnet and the reserved address of the Grafana internal load balancer. Behind `observability_enabled`: the collector sidecars on the Cloud Run services and their telemetry environment, including `CODA_TELEMETRY_BACKEND` from its `telemetry_backend`, which must match the `observability` root's. | `rust-environments/stg` |
| <a href="/infra/gcp/observability/README%2Emd"><code>observability</code></a> | Inside that cluster: the OpenTelemetry gateway collector and its internal load balancer, one operations backend chosen by `telemetry_backend` (the Datadog Agent and monitors with, off by default, the project-scoped Datadog GCP integration; or self-hosted Tempo, Loki, Prometheus and Grafana with, off by default, a Cloud Monitoring datasource), the `otlphttp/langfuse` exporter to the Langfuse HIPAA cloud project at `langfuse_host`, Grafana's internal load balancer, which admits only the relay subnet so people reach Grafana only at `staging.coda.dabney.moe/grafana/` through public-site, and the Doppler operator. It reads the cluster and network through variables fed from `rust-environments` outputs and manages neither. Staging only. | `observability/stg` in the same state bucket |

[Observability](observability.md) describes the per-signal paths and the
content policy.

Grafana shows telemetry that can carry PHI, so people reach it only through
Google IAP, and IAP admits only `group:gcp-engineering@dabney.moe` plus the
named service accounts a route needs to hop between services. It is not granted
to `allUsers`, `allAuthenticatedUsers`, a `domain:` or a single `user:`, and it
has no listener that skips IAP. Langfuse is the hosted HIPAA cloud project, under
Langfuse's BAA; access to it is membership of that Langfuse organization, not
IAP. Production keeps that rule:
no production observability exists yet, and the roots refuse a production
cluster, relay or observability environment until a production route is
designed under it. The <a href="/infra/gcp/observability/README%2Emd#boundary">observability boundary</a>
says how staging meets it.

Run every command on this page inside `nix develop` from the repository root. The
shell provides `gcloud`, `tofu`, `doppler`, `timeout` and `jq`; do not use host copies.
Commands that read Doppler secrets need the default shell after `doppler login`,
not `.#agent`, which loads no credentials.
## Public marketing site

The separate [`infra/gcp/public-site`](/infra/gcp/public-site) root runs the
stateless Rust public-site server on Cloud Run, while static showcase exports
(`ORIGIN=https://dabney.moe`, `BASE_PATH=/coda`) live under `dabney.moe/coda/`.
Live hosted backends target `coda.dabney.moe` (`staging.coda.dabney.moe` behind
direct Cloud Run IAP). The root shares no physician/Duet databases. CI deploys
staging from `main`; production receives staging-tested images through manual
promotion, and operator applies require explicit operator authorization. [Public
site](public-site.md) owns the architecture, IAP and cache controls, runtime
configuration, checks, and the
[staging](public-site.md#staging-state-and-deployment) and
[production](public-site.md#production-state-and-deployment) procedures.

## Rust container images

`nix/packages.nix` builds the Rust Cloud Run images from `Cargo.lock` and `flake.lock`. Each image tag is the git revision, and the build sets `created` to the epoch, so a revision always produces the same archive.

| Flake output | Image | Role | Entrypoint binary |
| --- | --- | --- | --- |
| `physician-sample-image` | `coda-physician-sample` | runtime | `physician-sample` serving `PHYSICIAN_SITE_DIR` (Dioxus WASM bundle) |
| `physician-worker-image` | `coda-physician-worker` | runtime | `physician-worker`, which claims chat turns and reply revisions from Postgres |
| `observability-relay-image` | `coda-observability-relay` | runtime | `observability-relay`, which forwards public-site's `/grafana/` requests to Grafana's internal load balancer. It reads no secrets and starts without Doppler |
| `duet-api-image` | `coda-duet-api` | runtime | `duet-api`, the mobile Duet API and auth service |
| `physician-migration-image` | `coda-physician-migrate` | migration | `physician-migrate`: physician-auth, front-door, then runtime chains; packaged synthetic charts for explicit `load-charts` |
| `duet-migration-image` | `coda-duet-migrate` | migration | `duet-migrate`: Duet-owned chain and runtime grants, then optional front-door chain |
| `duet-privacy-delivery-image` | `coda-duet-privacy-delivery` | job | `duet-privacy-delivery`, a bounded Luna receipt job |

- The runtime servers never migrate. They bind `0.0.0.0:$PORT`, where `PORT` defaults to 8080. `/health` reports liveness. Hosted startup accepts only the explicit `stg` / `staging-preview` profile with its required stores and identity configuration; other hosted profiles refuse to start. `/readyz` checks the configured schemas. The startup owners are [`physician-sample/src/hosting.rs`](/crates/physician-sample/src/hosting.rs) and [`duet-api/src/main.rs`](/crates/duet-api/src/main.rs).
- Each image contains the binary and the libraries it links, Doppler, CA roots, tzdata and a passwd entry for uid `65532`. The image has no shell or toolchain. It runs as `65532:65532` with working directory `/tmp`.
- Every entrypoint except the secret-free images (`secrets = false`, such as `observability-relay`) is `doppler run --no-fallback --preserve-env=PORT -- <binary>`. The Doppler CLI is the static release pinned in [`crates/workspace-tools/src/doppler.rs`](/crates/workspace-tools/src/doppler.rs). Nix reads the version and SHA-256 from that file, so the runners and the images run the same verified binary. Cloud Run mounts `DOPPLER_TOKEN` from Secret Manager. No secret is in a layer or in `Env`, and `--no-fallback` keeps secrets off the filesystem.
- `scripts/check-images.sh` (run with `nix develop .#release`) builds every image and inspects each archive. It loads them into Docker and checks rejection of missing credentials or an unconfigured hosted profile. Against disposable PostgreSQL it runs both migration images twice and explicitly imports the packaged physician chart bank twice, checking its manifest totals, zero failures and an unchanged reload. It then starts the servers to check `/health` 200, schema readiness 200, the WASM assets and a clean SIGTERM exit. The `images` CI job runs this script. Nothing is pushed.

## Rust staging bootstrap and deployment gate

The isolated project is `coda-rust-staging` in `us-east1`. It is separate from
`coda-health-staging`, which hosts the public marketing site. The root owns its
GCS state under `rust-environments/stg`; the reviewed nonsecret inputs are
[`env/staging.tfvars`](/infra/gcp/rust-environments/env/staging.tfvars) and
[`backend/staging.gcs.tfbackend`](/infra/gcp/rust-environments/backend/staging.gcs.tfbackend).

### Review and apply a staging plan

From an authorized engineering account, inside the Doppler-authorized default
`nix develop` shell at the repository root, with current Google application
default credentials (`gcloud auth application-default login`).
[`staging-reconcile`](/crates/workspace-tools/src/staging_reconcile.rs) reads the
four existing role passwords from Doppler and passes them only to the OpenTofu
child that needs them, never in arguments or output; it rotates no credentials.
It refuses a failed or empty lookup before OpenTofu runs.

```bash
dir=$(mktemp -u /tmp/staging-plan.XXXXXX)
cargo run -p workspace-tools -- staging-reconcile plan --output-dir "$dir"
less "$dir/review.txt"
```

`plan` creates the owner-only directory outside the repository, saves the plan,
its review text and a manifest binding the plan's digest to the root's
configuration digest and the fixed staging backend. The plan, review and
`diagnostics.log` can contain credentials: never publish or attach them.

**STOP:** review every change and obtain operator approval before continuing.
Unexpected deletes, replacements, IAM/network changes, image changes, or new
billable resources require investigation and explicit approval. Coordinate with
releases so no deployment races the saved plan. Do not use `-target` or
`-refresh-only` to hide differences. Image digests remain release-owned.

```bash
cargo run -p workspace-tools -- staging-reconcile apply --plan-dir "$dir" --authorize-staging-apply
cargo run -p workspace-tools -- staging-reconcile verify --plan-dir "$dir"
```

`apply` refuses a plan whose bytes, root inputs or backend changed since review,
and applies a plan at most once. A failed, timed-out or interrupted apply is
uncertain: inspect state, the lock and live resources rather than retrying.
`verify` runs a fresh plan that must report no changes, then requires the
physician service's scaling, CPU and egress settings from the reviewed plan, a
Ready latest revision and all traffic on it. It writes a nonsecret
`*-verification.json` receipt and exits 3 on any difference. Securely dispose of
the directory when no longer needed.

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

Do not patch Cloud Run flags by hand. Use [the saved-plan
procedure](#review-and-apply-a-staging-plan) for approved corrections; its
`verify` compares live physician settings with the reviewed plan, which follows
`env/staging.tfvars` and the [physician
scaling](#physician-scaling-redis-fan-out-and-the-chat-turn-worker) and [SMTP
relay](#staging-sign-in-accounts) contracts. Repeat `verify` after the next
separately approved release. If settings regress, fix the release owner rather
than reapplying in a loop.

A Ready revision is not database or journey qualification; use the existing
[rollout](#direct-staging-rollout), [scaling verification](#physician-scaling-redis-fan-out-and-the-chat-turn-worker)
and [isolated recovery](#disposable-database-recovery-rehearsal) procedures.
Correct and rerun failed browser qualification; an infra apply does not waive it.

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
PHYSICIAN_MAIL_FROM=signin@dabney.moe
PHYSICIAN_PUBLIC_ORIGIN=https://staging.coda.dabney.moe/physician
```

The sender is `Coda Health <signin@dabney.moe>`. It requires no mailbox, Gmail
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

The physician migration and site images carry the charts and manifest from the
[synthetic patient bank](patient-bank.md), built offline from accepted ledgers and
their seeds at the image's revision. `PHYSICIAN_SYNTHETIC_CHARTS` names the packaged
folder: `physician-migrate load-charts` imports the practice's initial records into
physician Postgres, and the staging site (`CODA_HOSTED_PROFILE=staging-preview`)
registers the synthetic adapter against that folder so live source checks and
reconnect commands run against the packaged bank and persist deliveries and source
standing in physician Postgres.

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

The harness cutover needs a
[one-time reset of the Chords runtime tables](physician-site.md#runtime-schema-cutover)
before its first release. The automatic deploy below does not perform it. After
the cutover, each physician runtime schema change is a new migration in the
runtime chain, and the migration job applies each one once.

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
2. checks the shape of the `physician` and `duet` services before any migration
   runs, then for `physician`, then `duet`: updates `<app>-migrate` to the migration
   digest and executes it to completion, then for physician updates `physician-worker` once that
   service exists, then updates the `<app>` service's product
   container to the runtime digest and requires its latest Ready revision to serve
   that digest with all traffic. The update is submitted with `--async` and
   readiness is polled every 5 s for at most 10 minutes, within the release's
   overall command budget. Polling stops early when the submit failed and two
   consecutive reads, 5 s apart, still show a different image in the service spec.
   The release then stops with an unconfirmed rollout rather than waiting out the
   poll bound;
3. updates `observability-relay` to its digest once the service exists, with the
   same checks. `physician-worker` moves before the physician site in step 2, so a
   site that announces turns in Postgres never serves ahead of a worker that
   listens for them. Until a service exists the release reports the digest to record as
   `images.physician.worker` or `observability_relay_image` for the infrastructure apply;
4. finalizes and uploads the structured `receipt.json`, tested-archive manifest
   and digest files as the `release-receipt-*` artifact, including failed runs.
   [`staging-release`](/crates/workspace-tools/src/staging_release.rs) initializes
   the receipt before publication and checkpoints before rollout mutations. It
   retains previous and observed final service states, migration execution IDs,
   explicit worker bootstrap skips and sanitized failure codes. Missing receipts
   and failed uploads fail visibly. Reconciliation is observational: it never
   reexecutes an uncertain migration or silently rolls traffic back.

The tested archive artifact is produced only while staging deployment is enabled
and retained for three days. A failed-job rerun uses
the original successful image job's artifact ID; rerunning the image job creates
a new artifact for that attempt. A missing, expired or invalid artifact stops the
release; rerun the image proof to produce a new handoff.

Cloud Run moves traffic only to a revision that passes its `/readyz` startup
probe. This protects an individual failed candidate, not the whole release:
the worker can already be serving a new revision when the physician site or Duet fails.
A timed-out update can also have completed remotely. Inspect each service's
observed traffic before deciding what to do; an unavailable read means unknown,
not unchanged. Fix forward by default. Returning traffic to a previous digest
requires proof that it can use the migrated schema; never reverse migrations
automatically. OpenTofu ignores the image fields, so a later saved-plan apply
does not roll the services back. Unsetting the variable stops staging deploys without editing
the workflow:

```bash
gh variable set RUST_STAGING_DEPLOY --body enabled   # on
gh variable delete RUST_STAGING_DEPLOY               # off
```

The release changes one field per service: the product container's image.
[`staging-release`](/crates/workspace-tools/src/staging_release/shape.rs) checks
each service's template around its `gcloud run services update`:

- **Before.** Before any migration runs, each existing service's template must
  have one unnamed container (`observability-relay`: one container named after
  the service), or exactly the product container (named after the service) and
  an `otel-collector` sidecar. With the sidecar, the product carries
  the telemetry environment (`DD_ENV`, `OTEL_EXPORTER_OTLP_ENDPOINT`,
  `CODA_LOG_FORMAT=json`, `CODA_TELEMETRY_CONTENT` of `none` or `full`,
  `CODA_TELEMETRY_BACKEND`), the sidecar carries its gateway configuration and
  takes no port. Any other shape stops the release. A two-container service is
  updated with `--container <service>`, so the image never lands on the sidecar,
  and the served image is read from the container named after the service.
- **After.** Once the new revision serves the image, the template must equal the
  old one except for that image and gcloud's client annotations and labels: same
  containers, environment, resources, CPU allocation, probes and scaling. A lost
  sidecar or any other moved field fails the service's rollout and stops the
  release before the next service.

Sidecars, telemetry environment, CPU and scaling belong to
[`rust-environments`](/infra/gcp/rust-environments) and change only through a
saved-plan apply. The release accepts either well-formed shape, so before that
apply it keeps releasing the single-container services unchanged.

### Telemetry sidecars and CPU allocation

With `observability_enabled`, each service runs an `otel-collector` sidecar
(1 vCPU, 256 MiB) that forwards to the observability gateway. The sidecar does
not change how CPU is allocated:

| Service | `cpu_idle` | Minimum instances | Why |
| --- | --- | --- | --- |
| `physician` | `false` | 0 | Its Postgres authority listener and Redis subscriber run between requests while an instance is up; the worker owns accepted turns, so none has to stay up |
| `duet` | `true` | 0 | All work happens inside a request |
| `physician-worker` | `false` | 1 | Its Postgres listener, 10 s poll and lease renewals run outside any request |

Telemetry is best effort, as the sidecar's own configuration states. Under
request-based CPU, a batch queued after the last response waits until the next
request or until shutdown, when Cloud Run gives the instance CPU again. At
shutdown, Cloud Run sends SIGTERM to the product and the sidecar together, so
the product's final flush can race the collector's own exit. The collector's
queue is bounded and drops on overflow, and its retries give up after 5 s.
Telemetry still queued in either process when the instance stops is lost.
Services never wait on it.

Cost at Cloud Run tier-1 prices ($0.000018 per vCPU-second, $0.000002 per
GiB-second, instance-based) for an instance kept warm for a month:

- `physician` with the sidecar: 2 vCPU and 1.25 GiB, about $101 a month instead
  of about $53 without it, for each instance kept up all month. It scales to
  zero, so it bills only while an instance is up, including up to 15 idle
  minutes after its last request.
- `physician-worker` has the same shape, instance-based CPU and its own minimum
  instance, so it is kept warm all month as well.
- `duet` stays on request-based billing and scales to zero, so idle time costs
  nothing. Instance-based CPU would have billed each warm instance for up to 15
  idle minutes after every request, up to about $101 a month at continuous low
  traffic.

### Database backup policy and RPO

[`database.tf`](/infra/gcp/rust-environments/database.tf) and
[`monitoring.tf`](/infra/gcp/rust-environments/monitoring.tf) enforce the
database backup and recovery-point policy across `stg` and `prd` (`prd` adds
`REGIONAL` high availability and requires explicit operator authorization plus
configured alert notification channels before databases can be planned):

| Policy item | Requirement | Enforcement in `rust-environments` |
| --- | --- | --- |
| Frequency and window | Automated daily full backup of both product databases (`rust-physician` and `rust-duet`) | `backup_configuration.enabled = true`, `start_time = "03:00"` (UTC) |
| Backup location | United States multi-region storage surviving a primary-region outage | `backup_configuration.location = "us"` (`sql_backup_location`), paired with a US-region precondition on `google_sql_database_instance.app` |
| Physician data RPO | Target **5 minutes**, never more than **15 minutes** | Continuous PostgreSQL Write-Ahead Log (WAL) archiving (`point_in_time_recovery_enabled = true`), not scheduled incremental backups |
| Retention | **7 retained daily full backups** and **7 days of transaction logs** | `retained_backups = 7` (`retention_unit = "COUNT"`), `transaction_log_retention_days = 7` |
| Scope | `rust-physician` owns durable physician clinical state; `rust-duet` pairs SQL backups with independent journal replay | Memorystore (`rust-physician-cache`) holds only ephemeral fan-out events, Langfuse holds disposable generation trace copies, and Gmail and Spruce retain their own external source records; Duet restores require [privacy recovery](privacy-recovery.md#isolated-restore-qualification) |

[`monitoring.tf`](/infra/gcp/rust-environments/monitoring.tf) routes Cloud SQL
backup and physician RPO alerts to `database_backup_alert_channels` (defaulting
to `availability_watchdog_channels`):

- **Daily backup older than 26 hours (`sql_backup_stale`).** Per-instance
  `coda/sql_backup_succeeded_<app>` counts `STATUS_SUCCEEDED` entries in
  `cloudaudit.googleapis.com/system_event`
  (`protoPayload.methodName="cloudsql.instances.automatedBackup"`), scoped by
  `resource.labels.database_id` so one product's daily backup cannot mask the
  other's. `sql_backup_stale` alerts when `absent_over_time(...[24h])` holds
  continuously for `7200s` (26 hours total, keeping the PromQL lookback plus
  `300s` evaluation interval inside Cloud Monitoring's 25-hour limit for user
  log-based metrics).
- **Daily backup failed or skipped (`sql_backup_failed`).** Per-instance
  `coda/sql_backup_failed_<app>` and `sql_backup_failed` notify immediately when
  `cloudsql.instances.automatedBackup` reports `STATUS_FAILED` or
  `STATUS_SKIPPED`, rather than waiting out the 26-hour absence window.
- **Physician WAL archiving and disk backpressure (`physician_sql_wal_archive_failed`, `physician_sql_disk_pressure`).**
  `coda/physician_sql_wal_archive_failed` matches `archive command failed` and
  `archiving write-ahead log file .* failed` in `rust-physician`'s
  `cloudsql.googleapis.com/postgres.log` and pages on the first failed archive
  cycle so low-traffic 5-minute `archive_timeout` cycles notify before 15
  minutes of committed data are unarchived. `physician_sql_disk_pressure` alerts
  when `rust-physician` exceeds `0.85` disk utilization for 10 minutes from
  local `pg_wal` accumulation.
- **Cloud SQL PITR lag metric boundary.** Google Cloud SQL
  (`cloudsql.googleapis.com`) exposes no Cloud Monitoring metric for
  point-in-time recovery-point age, latest recoverable timestamp, or silent
  asynchronous WAL offload lag into Google-managed Cloud Storage
  (`write_ahead_log/*_bytes_count` tracks local WAL flush and sits at zero when
  idle; `bytes_used_by_data_type` tracks stored bytes rather than lag;
  `replication/replica_byte_lag` exists only on read replicas). Automated alerts
  therefore catch daily backup failures or >26-hour absence, PostgreSQL
  `archive_command` failures, and `pg_wal` disk pressure, while actual
  recovery-point lag against the 5-minute target and 15-minute limit is measured
  only through isolated point-in-time clone rehearsals.

### Disposable database recovery rehearsal

[`workspace-tools staging-restore`](/crates/workspace-tools/src/staging_restore.rs)
is an operator command, not part of automatic deployment. It accepts only the
canonical staging physician or Duet source, a successful backup and a fresh
`rust-restore-<product>-<run>` target. Its default is read-only cloud preflight;
cloud writes require both explicit execution and authorization flags. The
operator must separately authorize the billable target and cleanup. The normal
release identity receives no SQL administration privileges.

Run `cargo run -p workspace-tools -- staging-restore --help` for the command
interface. Use a fresh receipt path for execution after preflight.
The command creates a private PostgreSQL target sized from the source's data disk
(preflight refuses sources outside 10-50 GB), records its creation identity,
restores the selected backup and waits for bounded operations. It refuses an
existing target rather than trusting its name or labels. Receipts retain operation
IDs, backup age, elapsed restore time and cleanup state; an interrupted create
may leave a billable resource whose identity requires manual investigation.
Never blindly retry or delete a target solely by its name.

When cleanup refuses because the receipt has no `target_created_at` (the create
failed or timed out before its identity was recorded), the authorizing operator
owns the investigation. Describe the receipt's `create_operation`, then describe
the instance and match its `coda-run` label to the receipt's `run_id`, and its
project, network and create time to the receipt. Delete it only under a separate
explicit authorization, and record the result beside the receipt.

Cloud restore completion is explicitly **unqualified**. Before a rehearsal:

1. Establish synthetic sentinel records through each product's existing owners
   and record their identities without clinical content in the evidence bundle.
   Select a successful backup containing them. For Duet, retain independent
   privacy receipts for a deletion/withdrawal accepted after that backup.
2. Review the target resources and operator access. Keep restored databases out
   of service routing, workers, subscriptions and outbound effects. Do not reuse
   a managed staging or production instance as a restore target.
3. Restore each product separately. Through an isolated validation process, check
   the owning migration ledger, synthetic saved-state readback and runtime versus
   migrator permissions. Run Duet's complete independent journal replay and check
   deletion/withdrawal refusal as described in [privacy recovery](privacy-recovery.md#isolated-restore-qualification).
4. Record validation commands/results beside the cloud receipt, tied to the
   target and revision. Record measured recovery duration and recovery-point age
   against the [backup policy](#database-backup-policy-and-rpo); retention
   settings alone do not establish measured RPO or RTO. For physician PITR
   measurement, clone `rust-physician` with `gcloud sql instances clone ... --point-in-time`
   into a fresh private `rust-restore-physician-pitr-<run>` instance that is
   never attached to serving, workers, queues or external channels, verify the
   sentinel record timestamp, and clear the clone's inherited deletion
   protection only under the separate cleanup authorization for that exact
   disposable target.
5. After separate cleanup authorization, use the receipt-bound cleanup command.
   It checks project, creation identity, private network and drill labels before
   deleting only its disposable target. Record deletion completion; unresolved
   or uncertain creation/deletion remains an operating obligation.

Neither local Postgres recovery tests nor mocked cloud operations replace an
actual authorized Cloud SQL restore and product readback. Keep those proof types
separate in the release handoff. Do not reopen serving based only on a successful
cloud operation or a manually asserted status.

### Operating limits and failure investigation

The [environment root](/infra/gcp/rust-environments) owns resource bounds, not an
aggregate spending ceiling. Its runtime and worker instance caps apply per
revision, so overlapping revisions can consume more than one revision's cap.
CPU, memory, request concurrency and timeouts bound individual executions;
migration jobs have one task, a 600-second timeout and no automatic retries.
SQL disk growth is capped and backups/PITR are retained for seven days. Redis,
SQL, minimum instances and networking can incur charges while no requests run.

On `coda.dabney.moe` and staging Cloud Run services, `physician-sample`, `duet-api`,
and `public-site` share one HTTP ingress abuse boundary in
[`ingress.rs`](/crates/http/src/ingress.rs) (`IngressGuard`). It enforces fixed-window
per-client IP rate limits (`429 Too Many Requests` with `Retry-After` and
`Cache-Control: no-store`) keyed from the rightmost trusted `X-Forwarded-For` hop,
strictly tightens quotas on unauthenticated auth/magic-link/OTP/invite/sync paths,
rejects oversized bodies fast (`413 Payload Too Large` + `DefaultBodyLimit`), bounds
concurrent in-flight non-probe requests (`503 Service Unavailable`), caps tracked
client buckets in memory, and exempts health/readiness probes (`/health`, `/healthz`,
`/readyz`, `/api/health`). Runtime knobs are `CODA_RATE_LIMIT_RPM`,
`CODA_AUTH_RATE_LIMIT_RPM`, `CODA_MAX_REQUEST_BODY_BYTES`,
`CODA_MAX_CONCURRENT_REQUESTS`, `CODA_TRUSTED_PROXY_HOPS`, and
`CODA_RATE_LIMIT_ENFORCE_LOOPBACK` (automatically enforced when `K_SERVICE`,
`CODA_HOSTED`, `APP_ENV`, or `CODA_ENVIRONMENT` is set, exempting local
loopback tests otherwise).

Before a rehearsal, compare the deployed Cloud Run services/jobs, SQL settings
and worker settings with the reviewed root inputs. Record identifiers and
nonsecret limits, not full service descriptions containing environment values.
Have the billing administrator verify the project budget's thresholds and
notification recipients, and the model-provider owner verify account limits.
A budget alert is not a hard cap; a per-turn bound is not an aggregate provider
limit. Record inaccessible settings as unverified rather than treating them as
absent or effective. Changes to budgets, IAM or capacity need their owners'
authorization.

For a failed release, start with its workflow run and release receipt. Use the
recorded execution name to find migration logs, and the recorded revision names
to find runtime/worker logs. Inspect current traffic independently before a
retry. Do not copy raw environment values, tokens or clinical content into a
receipt or public diagnostic. For worker failures also read the
attempts saved on the turn in `physician_runtime_conversations`: each failed
attempt is recorded there with its time, and a turn that failed five times stays
failed until the physician chooses Try again. A deployment
check and a ready process do not establish that a clinical journey succeeded.

### Physician scaling (Redis fan-out and the chat-turn worker)

`enable_physician_scale` lets the physician site run on more than one instance. It takes effect only
together with `enable_workloads`. Applying it writes project IAM bindings
that engineering can't write alone. Everything it creates belongs to physician; Duet gets no cache, worker,
grant or setting, and the root's tests check this. No Pub/Sub topic, subscription
or push identity exists, and the root does not enable the Pub/Sub API: turns are
announced and claimed in Postgres, as
[the physician site chapter](physician-site.md#scaling-across-instances) describes.

- [`cache.tf`](/infra/gcp/rust-environments/cache.tf) creates the Memorystore
  instance `rust-physician-cache`: Basic tier, 1 GB, Redis 7.2, AUTH and TLS
  (`SERVER_AUTHENTICATION`). It sits on the existing `rust-sql-private` peering,
  so it has no public endpoint. It carries `workspace_changed` and
  `workspace_settled` between instances. Clients use `fred` with TLS and AUTH
  (`PHYSICIAN_REDIS_URL`, `PHYSICIAN_REDIS_CA_PEM`). Lag or a reconnect is a
  `resync`, and the page refetches. Sign-in and panel notices stay on the
  Postgres `physician_authority` triggers.
- [`workloads.tf`](/infra/gcp/rust-environments/workloads.tf) adds the
  `physician-worker` Cloud Run service. It has internal ingress and no IAP, IAM
  invoker checks stay on, and only the availability probe may invoke it:
  nothing pushes work to it, and its only HTTP surface is the readiness probe.
  The worker runs as `physician-worker-runtime`, which may reach only the
  physician SQL instance. It reads the `physician-runtime-doppler-token` slot
  and runs on `stg_physician_runtime`, because it needs exactly those secrets;
  a second token on the same config would add a rotation step without narrowing
  anything. It has
  `cpu_idle = false` and one minimum instance, because no request carries its
  work. It listens on Postgres
  for accepted turns, reads the due work every 10 s, claims the leased turn,
  runs the model, commits, and publishes `workspace_changed`. A turn whose
  worker died is taken again when its lease runs out, so recovery belongs to the
  worker. Every minute each worker instance also writes the queue gauges the
  [worker monitors](observability.md) read, which is why its identity holds
  `roles/monitoring.metricWriter`.
- The physician site gets `PHYSICIAN_DURABLE_CHAT=1` and `cpu_idle = false`,
  and scales to zero like Duet. `PHYSICIAN_DURABLE_CHAT=1` is what hands turns
  to the worker. CPU stays allocated because its Postgres listener and Redis
  subscriber must keep running between requests. Its hourly receipt compaction
  sweeps once when each instance starts and needs no warm instance.
  Duet keeps `cpu_idle = true`. `runtime_environment` may not set any of the
  scaling keys, and Duet may not set any `PHYSICIAN_*` key.

`PHYSICIAN_REDIS_URL` embeds the AUTH string, so it lives only in Doppler, as
does `PHYSICIAN_REDIS_CA_PEM`. Neither value is committed.

#### Connection budget

The `physician` site and `physician-worker` both connect as the
`physician-runtime` role, whose `CONNECTION LIMIT` is 40
([`bootstrap-grants.sql`](/infra/gcp/rust-environments/bootstrap-grants.sql)).
Each process opens one Postgres pool,
[`local_product::runtime::Store::connect`](/crates/local-product/src/runtime.rs),
and every store in it reads through that pool: the runtime store, auth, the
front door and `/readyz`. The pool holds at most
`MAX_CONNECTIONS` = 3 connections, waits up to 10 s for one, and closes a
connection idle for 60 s. It keeps a returned connection only outside any
transaction, so a request cancelled mid-transaction leaves nothing open for
the next one. Each process's long-lived `LISTEN` (the site's
authority feed, the worker's wakeups) holds one of those 3 while it runs, so
two remain for requests and claims.

A rolling deploy runs the old and new revision side by side, so the peak is

```text
peak = services × max_instances × overlapping revisions × MAX_CONNECTIONS
     = 2 × 2 × 2 × 3 = 24 of 40
```

with `max_instances` at its default of 2. That leaves 16 for the migration job
and operators, and keeps the peak at or below 30. The highest `max_instances`
the role fits is `40 ÷ (2 × 2 × 3)`, rounded down: 3. Raising `max_instances`
past it, or raising `MAX_CONNECTIONS`, needs this budget redone and the role
limit changed with it.

To turn scaling on:

1. **Deploy the code first.** The site that queues turns and `physician-worker` must
   be on `main` with a successful `Deploy staging` job. Skills/docs-only pushes skip
   that job, so overall CI success is not deployment evidence. The plan refuses to
   run without a digest in `images.physician.worker`.
2. **Decide who applies.** The plan creates bindings `group:gcp-engineering@dabney.moe`
   cannot write: the worker's project-level Cloud SQL and Monitoring metric-writer
   roles. Apply as the project Owner, or grant engineering Project IAM Admin
   conditioned on
   `api.getAttribute('iam.googleapis.com/modifiedGrantsByRole', []).hasOnly(['roles/cloudsql.client', 'roles/cloudsql.instanceUser', 'roles/monitoring.metricWriter'])`,
   and remove it afterwards. Do not apply part of the plan.
3. **Plan, review, apply.** Use [the saved-plan procedure](#review-and-apply-a-staging-plan)
   right before applying, because a saved plan goes stale once `deploy-staging` or
   another apply touches state. A scaling-only plan must not change Duet or destroy
   anything. Apply exactly the reviewed file.
4. **Write Redis into Doppler and start new revisions,** with the commands below.
   OpenTofu doesn't write these two values. Without them each process keeps an
   in-process bus, so the worker's events never reach the site. Runtimes read Doppler
   only at process start. The AUTH string goes straight into Doppler and never reaches
   the screen, argv or a file.
5. **Verify.** Open site tabs until `labels.instanceId` in the `physician` stream logs
   shows two instances, send a turn from one tab, and check the reply appears in a tab
   on the other instance. The turn's row in `physician_runtime_conversations` should
   show one completed attempt.

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

To turn scaling off, set `enable_physician_scale = false` and apply a reviewed
saved plan. The plan removes the cache and the worker. The physician site
keeps
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
`MAIL_FROM=signin@dabney.moe`, `MAIL_NOTIFICATION_FROM=notifications@dabney.moe`,
and `MAIL_LISTEN_ADDR=0.0.0.0:8080`.

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
