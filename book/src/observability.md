# Observability

The observability stack has selectable **operations destinations** (traces,
logs, metrics, alerts) and **Langfuse** for agent runs. Destinations
are Datadog (APM, logs, Error Tracking, monitors), a self-hosted Grafana
stack (Tempo, Loki, Prometheus, Grafana). Both use the same OpenTelemetry
trace. The operations backend gets every span with agent content removed.
Langfuse gets only model generations, in the hosted project at
`https://hipaa.cloud.langfuse.com`, which Langfuse's BAA covers. The collector
strips content from the operations pipelines unconditionally, so no content
depends on an operations vendor's BAA scope.

This page describes configuration and verification, not live deployment status.
Infrastructure, Datadog and Grafana changes require separately authorized,
reviewed plans; the observability operator owns their application.

## Choosing a backend

`datadog` is the compatibility default. `both` runs Datadog and Grafana
concurrently with independent bounded exporter queues. Datadog owns operational
notifications; Grafana rules evaluate in shadow mode without service paging.
See <a href="/infra/gcp/observability/README%2Emd#concurrent-shadow-qualification">concurrent qualification</a>
for the default-off compatibility metrics source, separately gated synthetic
notification route, storage review and rollback requirements.

| | `datadog` | `grafana` |
| --- | --- | --- |
| Traces | Datadog APM | Tempo |
| Logs | Datadog Logs | Loki (OTLP ingestion) |
| Request metrics | APM trace metrics | Tempo metrics-generator span metrics, remote-written to Prometheus |
| Platform metrics | Datadog GCP integration (optional) | Cloud Monitoring datasource in Grafana (optional) |
| Alerts | Four `datadog_monitor`s to `alert_notify` | The same four rules, evaluated in shadow mode without notifications |
| Where it runs | Datadog SaaS, via the in-cluster Agent | StatefulSets in the observability cluster, ClusterIP only |

The choice lives in three places that must agree:

- **Collector.** One backend overlay, `collector.datadog.yaml`,
  `collector.grafana.yaml` or `collector.both.yaml`, is merged last and sets only the exporters of the
  `traces/ops` and `logs/ops` pipelines. Without an overlay those pipelines
  export to `nop`, so a missing choice drops data rather than sending it
  somewhere unintended. The privacy and caller processors are the same for
  both.
- **Services.** `CODA_TELEMETRY_BACKEND` (`datadog`, `grafana` or `both`) only changes
  the correlation keys in JSON log lines. OTLP export is identical.
- **Infrastructure.** `telemetry_backend` in both
  [`observability`](/infra/gcp/observability) and
  [`rust-environments`](/infra/gcp/rust-environments). The first deploys the
  Datadog Agent and monitors or the Grafana stack and picks the collector
  overlay; the second sets `CODA_TELEMETRY_BACKEND` on the product containers.
  The <a href="/infra/gcp/observability/README%2Emd#switching-the-backend">switch procedure</a>
  is in the root's README. `telemetry_content` works the same way: both roots
  set it, `rust-environments` puts it on the product containers as
  `CODA_TELEMETRY_CONTENT`, and `observability` refuses `full` without
  Langfuse.

## Data flow

```text
Cloud Run (staging): physician, duet, physician-worker    crates/telemetry (tracing + OTLP/HTTP)
        │  OTLP/HTTP localhost:4318 (spans and log records)
        ▼
otel-collector sidecar  (dev/observability/sidecar.yaml)
        │  OTLP/HTTP to the internal gateway, Google ID token (googleclientauth)
        │  Cloud Run Direct VPC egress → rust-private
        ▼
otel-gateway  internal passthrough LB, port 4318, source range = Cloud Run egress subnet
        ▼
otel-collector  GKE observability cluster
  │  collector.yaml + collector.gke.yaml [+ collector.gke-no-langfuse.yaml] + collector.<backend>.yaml (last)
  │  oidc: Google issuer + audience; caller allowlist of runtime service accounts
  ├─ traces/ops       … → transform/strip_content     → batch ─┬ datadog: Datadog Agent (OTLP 4317) → Datadog APM
  │                                                            └ grafana: Tempo (OTLP 4317) ── metrics-generator ─→ Prometheus
  ├─ traces/langfuse  … → filter/langfuse_generations → batch → Langfuse /api/public/otel (Basic auth)
  └─ logs/ops         … → transform/strip_log_content → batch ─┬ datadog: Datadog Agent (OTLP 4317) → Datadog Logs
                                                               └ grafana: Loki /otlp (OTLP/HTTP 3100)

Grafana (grafana or both) reads Tempo, Loki and Prometheus; operational rules remain shadow-only
Cloud Run stdout/stderr → Cloud Logging only (not forwarded to either backend)
Cloud Run and GKE platform metrics → Datadog GCP integration, or Grafana's Cloud Monitoring datasource (optional, project-scoped)
```

Locally, services export straight to the compose collector
(`dev/observability/collector.yaml` plus the debug, Datadog or Grafana
overlay) with no sidecar or gateway.

Langfuse can be switched off on its own (no `langfuse_host` in
`infra/gcp/observability`). The gateway then merges
`collector.gke-no-langfuse.yaml` before the backend overlay, and
`traces/langfuse` ends in `nop`. The ops pipelines and every privacy and
caller step are unchanged.

Per signal:

- **Traces.** Every span goes to the operations backend (Datadog APM or Tempo)
  with content deleted. Generation spans also go, with content as the service
  sent it, to the Langfuse project at `langfuse_host`.
- **Logs.** Services emit their log events as OTLP log records over the same
  sidecar path. Every HTTP response emits one `info` record with `status`,
  `latency_ms` and the request's trace id, so a request with a span and no log
  record means the log path is broken. The gateway deletes the same content keys from log record and
  resource attributes and sends them to the operations backend only (Datadog
  Logs or Loki); log records never go to Langfuse. Loki indexes only
  `service.name` and `deployment.environment` as labels (`service_name`,
  `deployment_environment`); `trace_id`, `coda.run_id` and the other
  attributes are structured metadata. Cloud Run's own stdout capture stays in
  Cloud Logging, so a service with export off still has its logs there.
- **Metrics.** Under `datadog`, request rate, error rate and latency per
  service come from APM's trace metrics. Under `grafana`, Tempo's
  metrics-generator derives span metrics (`traces_spanmetrics_calls_total`,
  `traces_spanmetrics_latency_bucket`) and a service graph and remote-writes
  them to Prometheus. Their only extra dimensions are `coda.job`,
  `gen_ai.system` and `deployment.environment`; run ids never become metric
  labels. Cloud Run and GKE platform metrics (instances, cold starts,
  container CPU and memory, requests answered before the product container)
  are optional under either backend and off by default: the Datadog GCP
  integration (`datadog_gcp_integration_enabled`), which Datadog reads by
  impersonating a read-only service account, or Grafana's Cloud Monitoring
  datasource (`grafana_cloud_monitoring_enabled`), which reads through
  Workload Identity as a `roles/monitoring.viewer` service account. Neither
  has a key.
- **Release.** `DD_ENV` comes from OpenTofu (`staging`) under both backends;
  it becomes `deployment.environment`. `DD_VERSION` is baked into each image
  at build time from its source revision, so a revision's spans and logs name
  the image that produced them and no apply can revert it.

The <a href="/infra/gcp/observability/README%2Emd#cloud-run-gateway"><code>infra/gcp/observability</code> README</a>
details the gateway's transport, authentication and rotation.

- **Spans.** [`crates/telemetry`](/crates/telemetry/src/lib.rs) installs one
  `tracing` subscriber. When `OTEL_EXPORTER_OTLP_ENDPOINT` is set, it exports
  OTLP/HTTP spans with `service.name`, `deployment.environment` (`DD_ENV`), and
  `service.version` (`DD_VERSION`). `http_layer` traces each request and
  continues an inbound W3C `traceparent`.
- **Generations.** `telemetry::genai::generation_span` opens a
  `gen_ai.generation` span in Langfuse's OTel mapping
  (`langfuse.observation.type=generation`, `gen_ai.system`,
  `gen_ai.request.model`, `gen_ai.usage.*`). It also carries the run
  contract: `coda.run_id`, `coda.job`, `coda.recipe_version`, `coda.origin`
  (`simulation` or `real`, also written to `langfuse.environment`),
  `coda.scenario_id`, `langfuse.session.id`, and `langfuse.trace.tags`.
- **Logs.** When an OTLP endpoint is set (or `CODA_LOG_FORMAT=json`), stdout
  logs are JSON lines with the W3C hex `trace_id` and `span_id`. Under
  `CODA_TELEMETRY_BACKEND=datadog` they also carry `dd.trace_id`,
  `dd.span_id`, `dd.service`, `dd.env` and `dd.version`; under `grafana` they
  carry no `dd.*` keys. Exported log records carry the trace and span ids
  under both, so a Datadog log or a Loki line opens its trace. A log event
  carries its own fields, never span fields, so content recorded on a
  generation span never reaches log storage.

### Sidecar lifecycle and outages

The sidecar is a second container in each Cloud Run service. The product
container does not depend on it: Cloud Run starts and health-checks the
product on its own probes, and the sidecar has its own startup probe on the
collector's `health_check` port with 1 vCPU and 256 MiB limits. Startup order
is independent, but revision readiness is not: a collector image-pull or startup
failure can prevent the new revision from serving.

- **Gateway or backend outage.** The sidecar keeps a bounded in-memory
  queue and retries for a bounded time, then drops. The product never blocks
  on export, and its requests keep succeeding.
- **Sidecar crash.** The product's exporter fails fast against localhost and
  drops; Cloud Run restarts the sidecar. Telemetry from that window is lost.
- **CPU and shutdown.** Enabling observability disables CPU throttling for
  the product and sidecar so export can drain between requests. Both containers
  receive 1 vCPU, increasing per-instance allocation and billing. This cost
  belongs in the reviewed deployment plan. On scale-in, the product makes a
  bounded flush through `TelemetryGuard` and the sidecar flushes on `SIGTERM`
  within Cloud Run's 10-second grace period; remaining queued telemetry is lost.

## Working-window timing

The shared harness retains the opening time in Unix milliseconds on each
`WindowOpened` event, alongside its reason and optional note reference.
`ProviderStarted` retains the request's admission time. Products store both
in their existing run journals. These wall-clock timestamps let inspection
relate a window opening to surrounding requests without a second timing store.
Cross-process comparisons depend on the hosts' clock synchronization. The
records contain model context and stay with the product; they are not exported
to Datadog as raw logs.

## Content policy

The policy applies identically to both backends: it lives in the shared
collector processors, and the backend overlay changes only the exporters.

| Where | Rule |
| --- | --- |
| Service | `CODA_TELEMETRY_CONTENT=none` (default) records no prompt or completion text. `full` attaches it as `langfuse.observation.input/output`. Simulation traffic may use `full` anywhere. Real traffic may use it only when spans go to the Langfuse project under the BAA. |
| Collector → ops traces | `transform/strip_content` on `traces/ops` keeps only an allowlist of operational attributes on resources, spans and span events, so prompt and completion text (`langfuse.observation.input/output` and any other key) never leaves for the operations backend. It also clears status messages and renames unknown span names. The policy setting does not change this. Model, usage, run, scenario and origin attributes stay. |
| Collector → Langfuse | `filter/langfuse_generations` keeps only spans with `langfuse.observation.type` or the `gen_ai.*` attributes the crate sets. HTTP and infrastructure spans stay in the operations backend. |
| Collector → ops logs | `transform/strip_log_content` on `logs/ops` applies the same kind of allowlist to log record and resource attributes and replaces every body. No log pipeline exports to Langfuse. |
| Datadog Agent (`datadog`) | Tails no container logs and excludes the `observability` namespace, because Langfuse can log payloads on errors. It receives logs only as OTLP from the gateway. |
| Tempo, Loki, Prometheus (`grafana`) | Only the gateway collector writes to Tempo and Loki; Prometheus receives only Tempo's span metrics. Nothing scrapes container logs. All ClusterIP only. |
| Gateway | Accepts only Google ID tokens with the gateway audience from the listed Cloud Run runtime service accounts, over the internal load balancer restricted to the Cloud Run egress range. |
| Langfuse | The hosted HIPAA cloud project at `langfuse_host`, reached over HTTPS with the project's key pair. People sign in at `https://hipaa.cloud.langfuse.com` as members of the Coda organization; nothing in the cluster serves Langfuse. Retention is the project's 90-day setting. |
| Grafana | ClusterIP, plus the same relay-only internal load balancer. In staging it serves under `https://staging.codahealth.ai/grafana/` and signs people in from the forwarded IAP assertion (`X-Coda-IAP-Assertion`) after checking its signature, issuer and the public-site audience; otherwise `kubectl port-forward`. Grafana-stack retention is 14 days of traces, 30 of logs and 30 of metrics by default. |

`tofu test` in `infra/gcp/observability` reads the deployed collector config
and asserts that every content key is deleted on both span contexts and that
model and run ids survive. The local debug overlay runs the same processor, so
what it prints is what either backend would receive.

The full path is tested end to end by an ignored test in
[`physician-sample`](/crates/physician-sample/tests/worker.rs),
`content_reaches_langfuse_and_never_the_operations_backend`. It runs
`physician-worker` with `CODA_TELEMETRY_CONTENT=full` on a rehearsal practice,
through `otelcol-contrib` on `collector.yaml`, to a real Langfuse in Docker
([`dev/observability/test`](/dev/observability/test/langfuse.yaml)). It reads
the generation back through the Langfuse API (environment `simulation`, the
chat as session, the run id, the prompt and reply), and asserts that the
operations pipelines delivered the same generation, with its model, to the
test's OTLP sink with neither text anywhere in what they sent:

```sh
PHYSICIAN_RUNTIME_TEST_DATABASE_URL=postgres://... \
  cargo test -p physician-sample --test worker content_reaches -- --ignored
```

## Environment variables

| Variable | Set on | Meaning |
| --- | --- | --- |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | services | Collector OTLP/HTTP base. Unset means no export (tests and the default sample server). Locally and on Cloud Run `http://localhost:4318` (the compose collector, or the service's sidecar). |
| `DD_ENV` | services | `local` or `staging`, exported as `deployment.environment` under either backend; on Cloud Run set by OpenTofu. Falls back to `CODA_ENV`. |
| `DD_VERSION` | services | Release version (`service.version`), baked into each image's environment at build time from its source revision. Never set by OpenTofu. |
| `CODA_TELEMETRY_CONTENT` | services | `none` (default) or `full`. |
| `CODA_TELEMETRY_BACKEND` | services | `datadog` (default), `grafana` or `both`. Under `datadog` and `both`, JSON log lines add the `dd.*` correlation keys. An unknown value warns once and falls back to `datadog`. On Cloud Run set from `rust-environments`' `telemetry_backend`. |
| `CODA_LOG_FORMAT` | services | `json` forces JSON logs without an OTLP endpoint. Cloud Run sets it. |
| `DATADOG_OTLP_ENDPOINT` | collector (`datadog`) | Datadog Agent OTLP gRPC `host:port`. |
| `TEMPO_OTLP_ENDPOINT` | collector (`grafana`) | Tempo OTLP gRPC `host:port`. |
| `LOKI_OTLP_ENDPOINT` | collector (`grafana`) | Loki OTLP/HTTP base ending in `/otlp`. |
| `PROMETHEUS_REMOTE_WRITE_URL` | Tempo | Where the metrics-generator remote-writes span metrics (Prometheus `/api/v1/write`). |
| `TEMPO_BLOCK_RETENTION` | Tempo | Trace retention, default `336h`; GKE sets it from `grafana_traces_retention_days`. |
| `LOKI_RETENTION_PERIOD` | Loki | Log retention, default `720h`; GKE sets it from `grafana_logs_retention_days`. Prometheus retention is a flag from `grafana_metrics_retention_days`. |
| `GRAFANA_SLACK_WEBHOOK_URL` | Grafana | Slack incoming webhook the provisioned contact point posts to. A Doppler secret on GKE; the local sink locally. |
| `OTEL_GATEWAY_AUDIENCE` | gateway collector, sidecar | The ID-token audience both sides use. |
| `OTEL_GATEWAY_ALLOWED_CALLERS` | gateway collector | Comma-separated runtime service-account emails whose data is kept. |
| `LANGFUSE_OTLP_ENDPOINT` | collector | Langfuse base ending in `/api/public/otel`. |
| `LANGFUSE_PUBLIC_KEY`, `LANGFUSE_SECRET_KEY` | collector | Langfuse project keys for Basic auth. In GKE they sync from Doppler. |

## Secrets

Doppler holds every observability secret: the Datadog API and app keys (under
`datadog`), `GRAFANA_ADMIN_PASSWORD` and `GRAFANA_SLACK_WEBHOOK_URL` (under
`grafana`), and the Langfuse project keys (`LANGFUSE_PUBLIC_KEY`,
`LANGFUSE_SECRET_KEY`, with `LANGFUSE_BASE_URL` beside them). Only the
selected backend's keys are synced, so the other backend's keys need not
exist. The Doppler Kubernetes Operator syncs them into the `observability`
namespace;
<a href="/infra/gcp/observability/README%2Emd"><code>infra/gcp/observability</code></a>
says how its token arrives and maps each key.

## Local runbook

```sh
nix develop
scripts/observability-up.sh            # Langfuse + collector, ops spans and logs to the debug exporter
scripts/observability-up.sh --datadog  # adds the Datadog Agent, DD_API_KEY from Doppler dev
scripts/observability-up.sh --grafana  # adds Tempo, Loki, Prometheus, Grafana and a webhook sink; no secrets

OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318 DD_ENV=local CODA_TELEMETRY_CONTENT=full \
  CODA_LOCAL_MODEL_EXECUTION=subscription scripts/local-practice.sh
```

With `--grafana`, also set `CODA_TELEMETRY_BACKEND=grafana` on the service.
Send one physician chat. The trace then appears in Langfuse at
<http://localhost:3000> (`dev@coda.local` / `coda-local-dev`, project
`coda-agents`). The content-free copy of the same spans prints in
`docker compose -f dev/observability/compose.yaml logs -f otel-collector`
(debug), shows in Datadog APM under `env:local` with `--datadog`, or opens in
Grafana Explore at <http://localhost:3300> (`admin` / `coda-local-dev`) with
`--grafana`. Prometheus is on <http://localhost:9091>. Grafana's Slack contact
point posts to a local sink instead of Slack; received alerts are listed at
<http://localhost:3301/received>.
`scripts/observability-up.sh down -v` removes the stack and its volumes. The
compose credentials are fixed local development values, and every port binds to
127.0.0.1. See <a href="/dev/observability/README%2Emd">dev/observability</a>.

## Downtime notification qualification

Datadog is the primary staging operational route to
`#engineering-observability`. An independent Cloud Monitoring watchdog covers
missing monitoring/probe heartbeat outside GKE. Concurrent Grafana service
alerts must stay shadow-only; its contact/evaluator qualification uses a
separately labeled test route, not duplicate operational pages.

The [local qualification rehearsal](/dev/observability/README%2Emd#offline-downtime-notification-rehearsal)
prints bounded synthetic failure/recovery, reminder, grouping and maintenance
steps and checks timestamped observations without making any external calls.
It extends the existing collector/sink tooling but does not substitute a local
webhook acknowledgment for a Slack receipt or a fixture for an actual evaluator.

Before live work, read back the exact account/contact/channel mapping and the
synthetic evaluator's signal selector and windows. Obtain explicit approval for
the labeled routing send, bounded synthetic failure/recovery and maintenance
window, and independent watchdog heartbeat-loss exercise. Include source and
deployed revisions, UTC start/end, maximum sends, dedicated target, abort and
cleanup steps. Infrastructure changes require their own reviewed saved plan and
authorization; approval for a Slack drill does not authorize an apply or IAM change.
Restore synthetic healthy signals and end only the drill's mutes at timeout or
abort, verify evaluator recovery, and retain evidence even when delivery fails.

Capture original signal, evaluator Pending/Firing (where exposed), provider
notification, human-confirmed Slack alert, evaluator recovery and Slack recovery
times. Record provider `last_notified` separately from the user's confirmed
Slack message timestamp/permalink. Measure detection and delivery intervals with
actual ingestion/evaluation delays. Prove absence of duplicate service paging,
maintenance silence and expiry behavior, and the independent watchdog route.
No downtime-to-Slack claim is valid until those live artifacts are inspected.

## Qualification and incident ownership

Datadog owns primary staging service notifications to the confirmed
`#engineering-observability` integration. Grafana evaluates service rules in
shadow mode; only the separately authorized, exactly labeled synthetic test
can notify. Cloud Monitoring owns the independent probe-heartbeat absence path
outside GKE and Grafana. Its heartbeat demonstrates probe execution and direct
publication, not collector health or clinical workflow success.

The product on-call owner handles service and overdue-work symptoms. The runtime
infrastructure operator handles probe authentication, scheduler execution and
state publication; the observability operator handles collection, backend
freshness, routing and storage. Use the affected service/environment and stable
resource identity in the alert, never patient, conversation or run IDs as metric
labels. Correlate a run only in the authorized trace/log investigation surface.

1. For readiness alerts, inspect the probe's fixed outcome and freshness first.
   Authentication failure or login redirect is not application success. Check
   IAP audience and caller grants without exposing tokens; do not bypass IAP or
   internal worker ingress to make a probe green. To check reachability on
   demand, run `workspace-tools observability-drill probe --output-dir DIR`. It
   executes one probe slot as the probe identity, the only rehearsal caller
   IAP admits, and fails unless every target's readiness sample and the
   heartbeat read back as successful. A workstation identity token carries
   gcloud's own audience, so IAP answers it with 401 by design.
2. For physician work, read the worker's queue gauges. Each `physician-worker`
   instance writes them every minute from the Postgres rows that own the work,
   on the `global` resource and labelled only with its instance id:
   `custom.googleapis.com/coda/physician/queue/oldest_due_seconds` (how long
   the longest-waiting due turn or reply revision has been runnable),
   `.../due` (how many are runnable now) and `.../failed` (how many ended
   failed and wait for the physician's Try again). `worker_oldest_due` alerts
   when due work has waited past 300 seconds throughout five minutes, and its
   absence means no worker is reporting; `worker_due` alerts when due work
   persists for 30 minutes; `worker_failed` alerts on any failed work. Compare
   them with the [worker processing/retry contract](physician-site.md) and the
   attempts saved on the rows before acting; do not retry work automatically.
   Investigate SQL and Redis using the configured exact resource scope and the
   cloud console.
3. For missing telemetry, inspect Cloud Scheduler/job execution and the direct
   Monitoring series separately from collector availability, exporter failures,
   queues and Datadog ingestion. Keep heartbeat absence distinct from a failing
   target. A cleared sparse counter or retired pod is not confirmed recovery.
4. For Grafana/collector loss or storage pressure, inspect stable workload
   readiness, restart/OOM evidence and mounted PVC used/capacity series. Do not
   resize from host-memory percentage alone or retain stale node groups as
   current workload failures. Use the <a href="/infra/gcp/observability/README%2Emd">private stack access procedure</a>
   and provisioned dashboards; preserve IAP and private access.

Follow the infrastructure README's <a href="/infra/gcp/observability/README%2Emd#maintenance-and-rollback-ownership">maintenance and rollback procedure</a>
for scoped, expiring mutes and reviewed saved-plan rollback. Keep the independent
watchdog active during collector or Grafana maintenance. Image-only releases
must preserve sidecars, environment, IAM and networking; an ordinary release's
before/after assertion is part of qualification, not an infrastructure override.

Periodic qualification and every material monitoring change need revision-bound
proof. Local mock plans, synthetic OTLP, isolated Grafana evaluation and offline
receipt validation prove regression contracts only. They do not prove cloud
IAM/token handling, Datadog query/ingestion, real Slack delivery, real dependency
failure, deployed volume retention or production evaluation delay. Record those
as separate gates and label each exercise **simulated signal** or **actual
component failure**. An authorized live drill must capture signal → pending →
firing → exact Slack receipt → recovery with UTC timestamps and cleanup, plus
the independent monitoring-loss route. Do not claim downtime notification
readiness until those observations exist. Production is outside this procedure.

## From alert to run

1. Open the primary Datadog notification from the confirmed `alert_notify`
   destination. Grafana evaluates equivalent operational rules in shadow mode;
   it does not send service notifications. Both evaluate the same four alerts (service 5xx rate, service p95 latency, physician chat
   generation errors, asynchronous failures) with the same thresholds.
   Check the environment, service, release version and incident time window.
   HTTP failures and latency cover service requests; asynchronous worker failures
   need a separate signal because the originating request can already have succeeded.
2. Follow the error log's trace link. The worker attempt and generation spans carry
   canonical correlation; a broker message ID identifies a delivery, not an attempt.
   Copy `coda.run_id`, `coda.work_id`, any predecessor ID, and the trace ID. Preserve
   the query window and release version with the evidence.
   Under `grafana`, reach Grafana with
   `kubectl -n observability port-forward svc/grafana 3000` and use Explore over
   the alert's window:
   - **Loki to Tempo.** Query the failing service, for example
     `{service_name="physician-worker"} | severity_number >= 17`. Expand the
     line and follow its `trace_id` field's "Open trace" link into Tempo.
   - **By run.** `{service_name="physician-worker"} | coda_run_id="<run id>"`
     finds a run's log lines that carry it (Loki turns dots in attribute
     names into underscores); in Tempo, TraceQL
     `{ span.coda.run_id = "<run id>" }` finds its spans.
   - **Tempo to Loki.** From a span, "Logs for this span" queries Loki on the
     same service and `trace_id`.
   - **Rates and latency.** Prometheus `traces_spanmetrics_*` series by
     `service`; exemplars open the matching traces.
3. An authorized product operator reads back the matching persisted attempt from
   physician Postgres, scoped to the intended practice and physician. Chat attempts
   are in the `state.attempts` collection of `physician_runtime_conversations`;
   current reply-revision attempts are in `physician_runtime_reply_drafts`, and
   earlier accepted revisions are archived in `physician_runtime_reply_revisions`.
   Match the exact attempt ID, its work and any persisted predecessor relationship,
   and its terminal status.
   Project only those operational fields into the evidence: do not export the
   surrounding conversation, chart, instructions or model records to Datadog or
   Grafana.
   Never use a retry's newer attempt as proof for the originally alerted attempt.
4. In Langfuse (`https://hipaa.cloud.langfuse.com`), find the generation with the same
   trace ID and `coda.run_id`. A trace may contain several attempts, so verify both
   identifiers. The environment is `coda.origin`, and tags include `coda.job`.
   Input/output exists only when content capture was explicitly enabled; absent
   content is expected under the default policy. A failure before a model call
   must not be presented as a completed model generation.
5. Restore healthy operation and confirm recovery at the notification destination.
   Retain the notification, trace/log query, scoped attempt readback and Langfuse
   observation against the tested release. Collector configuration or an empty
   vendor search is not evidence of delivery or privacy.

`workspace-tools observability-drill langfuse alert-to-run` walks steps 2 and 4
on a rehearsal turn it sends itself. The chat snapshot names the attempt that
answered (`runs` holds the run once a turn is answered, held or failed, and an
answered run's `attempt_id` names the attempt); the drill finds that attempt's
span in Datadog by `coda.run_id`, reads its trace's generation span, then the
Langfuse trace, and fails unless the
attempt id and the trace id (Datadog's 64-bit id is the low half of the
128-bit one) agree across all four.

## Langfuse drills

Staging physician chat sends its generations to Langfuse with content, because
its rehearsal traffic is synthetic. `workspace-tools observability-drill langfuse`
proves that on the deployed stack, one action per run, each writing
`langfuse-<action>-<UTC stamp>.json` with the deployed revisions it ran
against:

| Action | What must hold |
| --- | --- |
| `rehearse` | A canary turn sent as the rehearsal physician answers, and Langfuse holds its generation in environment `simulation`, with the scenario, the chat as session, `coda.run_id`, a 128-bit trace id and the canary in its input or output. |
| `alert-to-run` | The same turn found from Datadog: the snapshot's attempt id, the attempt span, the generation span and Langfuse agree on `coda.run_id` and the trace id, and the attempt span records `coda.trace_relation` `link`: the attempt started its own trace linked to the request that accepted the turn. |
| `isolation` | Langfuse holds the canary (the positive control) under the snapshot's attempt; the operations spans of the same trace exist in Datadog, and in Tempo under `grafana`, with model and usage, and no canary anywhere; Datadog Logs, and Loki under `grafana`, hold the physician logs of the same trace (their positive control) and no physician log event since the turn carries the canary in any field. |
| `chat-failure` | The physician chat failure monitor is OK, then the drill sends one more canary turn than its threshold, each marked `[fail:1]`. Every turn answers on a retry and the monitor fires. For one turn, the answering attempt's span names the failed attempt as its predecessor in the same `coda.work_id`; the failed attempt's span has `error.type` `rehearsed`; the monitor's own query, narrowed to that attempt, finds the error generation in the failed attempt's trace; and Langfuse holds that attempt's generation in the same trace at level `ERROR`, with the canary. Both attempts' spans record `coda.trace_relation` `link` to the accepting request. Datadog holds no canary in either attempt's trace, and the monitor recovers after its 10-minute window. |

The rehearsal physician is the `physician-rehearsal` service account. Its turns
belong to a practice in `physician_simulation_practices`
([`rust-environments`](/infra/gcp/rust-environments/rehearsal.tf)), so the
worker labels them `simulation`; a turn in any other practice stays `real`
whatever it sends. The operator signs in with gcloud as one of
`physician_rehearsal_signers`, which may mint the account's IAP token through
IAM `signJwt`; `POST /auth/rehearsal` then admits exactly that principal.
Every `kubectl` call names the staging context
`gke_coda-rust-staging_us-east1_rust-observability`, so an ambient context is
never used; the drill opens its own port-forwards to Tempo and Loki, reads
Langfuse at `LANGFUSE_BASE_URL` with the project keys, and reads those and the
Datadog keys from Doppler `coda-rust/stg` without printing them. Every receipt
records the Langfuse origin it read as `langfuse_host`.

Every wait runs under the exercise's 40-minute cap and the operator's Ctrl-C.
A readback request that times out or cannot connect, as a port-forward may,
counts as one of the wait's polls; an error status or a malformed answer ends
the run at once, and a wait whose every poll failed reports the last failure.
The Langfuse export monitors in
[`langfuse-monitors.tf`](/infra/gcp/observability/langfuse-monitors.tf), behind
`langfuse_monitors_enabled`, page when the collector cannot deliver to the
hosted project; no drill takes the hosted project down to fire them.

`chat-failure` pages `#engineering-observability`, so it needs `--confirm`,
and runs only with the operator's approval for that window. Its failure is the worker's own: a rehearsal turn marked `[fail:K]` fails
its first K attempts before any model call, with `error.type` `rehearsed`,
and the worker runs it again once its retry backoff is due. Only a practice in `physician_simulation_practices`
on the `staging-preview` profile honours the marker; a real practice's turn
answers whatever it says. The monitor is `chat_generation_errors` in
[`datadog.tf`](/infra/gcp/observability/datadog.tf), on in staging through
`chat_failure_monitor_enabled` while the other service monitors stay off. The
drill writes its own exercise, group `staging/physician-chat-failure`, to the
same `observations-langfuse-<stamp>.json` ledger: the first marked turn, the
monitor's firing, the restore right after it (every marked turn has answered
by then, so the failures have stopped), and the monitor's recovery; a person
adds the Slack alert and recovery.

## Validation

The `observability` job in [CI](/.github/workflows/ci.yml) validates, inside
`nix develop .#observability`, the collector and OpenTofu shell, the base
collector alone; the base with each local overlay (debug, Datadog, Grafana); the
authenticated GKE gateway with each backend, with and without Langfuse; and the
inline sidecar. It checks the compose file with each profile, verifies the Tempo,
Loki and Prometheus configs with their pinned images, and validates and runs
mocked OpenTofu tests for both [`observability`](/infra/gcp/observability) and
[`rust-environments`](/infra/gcp/rust-environments). Backend-free initialization
belongs in a fresh test checkout, never over an operator's initialized backend.

Run the synthetic collector behavior test separately:
```sh
nix develop .#agent --command python scripts/test_collector_privacy.py          # both backends
nix develop .#agent --command python scripts/test_collector_privacy.py grafana  # one backend
```

It runs the production file order with the `datadog` or `grafana` overlay last
and exercises content stripping with positive controls, authenticated caller
filtering, and bounded queues during exporter outages. The generated evidence
under `target/verification` is local proof, not evidence of deployed delivery.

The Grafana alert rules fix their thresholds in
`dev/observability/grafana/provisioning/alerting/rules.yaml`, because Grafana
provisioning has no defaults for unset variables. A precondition in
`infra/gcp/observability` fails the plan under `grafana` unless they equal the
threshold variables the Datadog monitors use, so change both together.
