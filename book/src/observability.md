# Observability

The observability stack has selectable **operations destinations** (traces,
logs, metrics, alerts) and **self-hosted Langfuse** for agent runs. Destinations
are Datadog (APM, logs, Error Tracking, monitors), a self-hosted Grafana
stack (Tempo, Loki, Prometheus, Grafana). Both use the same OpenTelemetry
trace. The operations backend gets every span with agent content removed.
Langfuse gets only model generations and stays inside our GCP project. The
collector strips content from the operations pipelines unconditionally, so no
content depends on a vendor's BAA scope.

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

- **Collector.** One backend overlay, `collector.datadog.yaml` or
  `collector.grafana.yaml`, is merged last and sets only the exporters of the
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
  is in the root's README.

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

Langfuse can be switched off on its own (`langfuse_enabled = false` in
`infra/gcp/observability`). The gateway then merges
`collector.gke-no-langfuse.yaml` before the backend overlay, and
`traces/langfuse` ends in `nop`. The ops pipelines and every privacy and
caller step are unchanged.

Per signal:

- **Traces.** Every span goes to the operations backend (Datadog APM or Tempo)
  with content deleted. Generation spans also go, with content as the service
  sent it, to the self-hosted Langfuse in the same cluster.
- **Logs.** Services emit their log events as OTLP log records over the same
  sidecar path. The gateway deletes the same content keys from log record and
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

## Content policy

The policy applies identically to both backends: it lives in the shared
collector processors, and the backend overlay changes only the exporters.

| Where | Rule |
| --- | --- |
| Service | `CODA_TELEMETRY_CONTENT=none` (default) records no prompt or completion text. `full` attaches it as `langfuse.observation.input/output`. Simulation traffic may use `full` anywhere. Real traffic may use it only when spans go to the in-project Langfuse. |
| Collector → ops traces | `transform/strip_content` on `traces/ops` always deletes `gen_ai.prompt*`, `gen_ai.completion*`, `gen_ai.{input,output}.messages*`, and `langfuse.{observation,trace}.{input,output}*`, on span and span event attributes. The policy setting does not change this. Model, usage, and run ids stay. |
| Collector → Langfuse | `filter/langfuse_generations` keeps only spans with `langfuse.observation.type` or the `gen_ai.*` attributes the crate sets. HTTP and infrastructure spans stay in the operations backend. |
| Collector → ops logs | `transform/strip_log_content` on `logs/ops` deletes the same keys from log record and resource attributes. No log pipeline exports to Langfuse. |
| Datadog Agent (`datadog`) | Tails no container logs and excludes the `observability` namespace, because Langfuse can log payloads on errors. It receives logs only as OTLP from the gateway. |
| Tempo, Loki, Prometheus (`grafana`) | Only the gateway collector writes to Tempo and Loki; Prometheus receives only Tempo's span metrics. Nothing scrapes container logs. All ClusterIP only. |
| Gateway | Accepts only Google ID tokens with the gateway audience from the listed Cloud Run runtime service accounts, over the internal load balancer restricted to the Cloud Run egress range. |
| Langfuse, Grafana | ClusterIP only. People use `kubectl port-forward`; Langfuse also has an internal IAP gateway (`langfuse_iap_enabled`, default off). Langfuse retention defaults to 90 days; Grafana-stack retention is 14 days of traces, 30 of logs and 30 of metrics by default. |

`tofu test` in `infra/gcp/observability` reads the deployed collector config
and asserts that every content key is deleted on both span contexts and that
model and run ids survive. The local debug overlay runs the same processor, so
what it prints is what either backend would receive.

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
`grafana`), the Langfuse salt, NextAuth secret, encryption key, ClickHouse and
Valkey passwords, the GCS HMAC pair, and the Langfuse project keys. Only the
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
  CODA_LOCAL_MODEL_EXECUTION=replay CODA_CASSETTE_PATH=/absolute/path/cassette.json \
  PHYSICIAN_DATABASE_URL=postgres://user@127.0.0.1:5432/physician scripts/local-practice.sh
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
4. In Langfuse (port-forward or the IAP host), find the generation with the same
   trace ID and `coda.run_id`. A trace may contain several attempts, so verify both
   identifiers. The environment is `coda.origin`, and tags include `coda.job`.
   Input/output exists only when content capture was explicitly enabled; absent
   content is expected under the default policy. A failure before a model call
   must not be presented as a completed model generation.
5. Restore healthy operation and confirm recovery at the notification destination.
   Retain the notification, trace/log query, scoped attempt readback and Langfuse
   observation against the tested release. Collector configuration or an empty
   vendor search is not evidence of delivery or privacy.

## Validation

The `observability` job in [CI](/.github/workflows/ci.yml) validates, inside the
Nix shell, the base collector alone; the base with each local overlay (debug,
Datadog, Grafana); the authenticated GKE gateway with each backend, with and
without Langfuse; and the inline sidecar. It checks the compose file with each
profile, verifies the Tempo, Loki and Prometheus configs with their pinned
images, and validates and runs mocked OpenTofu tests for both
[`observability`](/infra/gcp/observability) and
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
