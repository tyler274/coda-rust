# Observability

The observability stack uses Datadog for
operations (APM, logs, Error Tracking, monitors, paging) and **self-hosted
Langfuse** for agent runs. The two use the same OpenTelemetry trace. Datadog
gets every span with agent content removed. Langfuse gets only model
generations and stays inside our GCP project. The collector strips content from
Datadog unconditionally, so no content depends on Datadog's BAA scope.

This page describes configuration and verification, not live deployment status.
Infrastructure and Datadog changes require separately authorized, reviewed plans;
the observability operator owns their application.

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
otel-collector  GKE observability cluster (collector.yaml + collector.gke.yaml)
  │  oidc: Google issuer + audience; caller allowlist of runtime service accounts
  ├─ traces/datadog   … → transform/strip_content     → batch → Datadog Agent (OTLP 4317) → Datadog APM
  ├─ traces/langfuse  … → filter/langfuse_generations → batch → Langfuse /api/public/otel (Basic auth)
  └─ logs/datadog     … → transform/strip_log_content → batch → Datadog Agent (OTLP 4317) → Datadog Logs

Cloud Run stdout/stderr → Cloud Logging only (not forwarded to Datadog)
Cloud Run and GKE platform metrics → Datadog GCP integration (optional, project-scoped)
```

Locally, services export straight to the compose collector
(`dev/observability/collector.yaml` plus a debug or Datadog overlay) with no
sidecar or gateway.

Langfuse can be switched off on its own (`langfuse_enabled = false` in
`infra/gcp/observability`). The gateway then merges
`collector.gke-no-langfuse.yaml` last, and `traces/langfuse` ends in `nop`.
The Datadog pipelines and every privacy and caller step are unchanged.

Per signal:

- **Traces.** Every span goes to Datadog APM with content deleted. Generation
  spans also go, with content as the service sent it, to the self-hosted
  Langfuse in the same cluster.
- **Logs.** Services emit their log events as OTLP log records over the same
  sidecar path. The gateway deletes the same content keys from log record and
  resource attributes and sends them to Datadog only; log records never go to
  Langfuse. Cloud Run's own stdout capture stays in Cloud Logging, so a
  service with export off still has its logs there.
- **Metrics.** Request rate, error rate and latency per service come from
  Datadog APM's trace metrics. Cloud Run and GKE platform metrics (instances,
  cold starts, container CPU and memory, requests answered before the product
  container) come from the staging-scoped Datadog GCP integration
  (`datadog_gcp_integration_enabled`, off by default), which Datadog reads by
  impersonating a read-only service account; no key exists.
- **Release.** `DD_ENV` comes from OpenTofu (`staging`). `DD_VERSION` is baked
  into each image at build time from its source revision, so a revision's
  spans and logs name the image that produced them and no apply can revert it.

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
  logs are JSON lines with `dd.trace_id` and `dd.span_id`, and exported log
  records carry the trace and span ids, so a Datadog log opens its trace. A
  log event carries its own fields, never span fields, so content recorded on
  a generation span never reaches log storage.

### Sidecar lifecycle and outages

The sidecar is a second container in each Cloud Run service. The product
container does not depend on it: Cloud Run starts and health-checks the
product on its own probes, and the sidecar has its own startup probe on the
collector's `health_check` port with 1 vCPU and 256 MiB limits. Startup order
is independent, but revision readiness is not: a collector image-pull or startup
failure can prevent the new revision from serving.

- **Gateway or Datadog outage.** The sidecar keeps a bounded in-memory
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

| Where | Rule |
| --- | --- |
| Service | `CODA_TELEMETRY_CONTENT=none` (default) records no prompt or completion text. `full` attaches it as `langfuse.observation.input/output`. Simulation traffic may use `full` anywhere. Real traffic may use it only when spans go to the in-project Langfuse. |
| Collector → Datadog | `transform/strip_content` always deletes `gen_ai.prompt*`, `gen_ai.completion*`, `gen_ai.{input,output}.messages*`, and `langfuse.{observation,trace}.{input,output}*`, on span and span event attributes. The policy setting does not change this. Model, usage, and run ids stay. |
| Collector → Langfuse | `filter/langfuse_generations` keeps only spans with `langfuse.observation.type` or the `gen_ai.*` attributes the crate sets. HTTP and infrastructure spans stay in Datadog. |
| Collector → Datadog logs | `transform/strip_log_content` deletes the same keys from log record and resource attributes. No log pipeline exports to Langfuse. |
| Datadog Agent | Tails no container logs and excludes the `observability` namespace, because Langfuse can log payloads on errors. It receives logs only as OTLP from the gateway. |
| Gateway | Accepts only Google ID tokens with the gateway audience from the listed Cloud Run runtime service accounts, over the internal load balancer restricted to the Cloud Run egress range. |
| Langfuse | ClusterIP only. People use `kubectl port-forward`, or an internal IAP gateway (`langfuse_iap_enabled`, default off). Retention defaults to 90 days. |

`tofu test` in `infra/gcp/observability` reads the deployed collector config
and asserts that every content key is deleted on both span contexts and that model and run ids
survive. The local debug overlay runs the same processor, so what it prints is
what Datadog would receive.

## Environment variables

| Variable | Set on | Meaning |
| --- | --- | --- |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | services | Collector OTLP/HTTP base. Unset means no export (tests and the default sample server). Locally and on Cloud Run `http://localhost:4318` (the compose collector, or the service's sidecar). |
| `DD_ENV` | services | `local` or `staging`; on Cloud Run set by OpenTofu. Falls back to `CODA_ENV`. |
| `DD_VERSION` | services | Release version, baked into each image's environment at build time from its source revision. Never set by OpenTofu. |
| `CODA_TELEMETRY_CONTENT` | services | `none` (default) or `full`. |
| `CODA_LOG_FORMAT` | services | `json` forces JSON logs without an OTLP endpoint. Cloud Run sets it. |
| `DATADOG_OTLP_ENDPOINT` | collector | Datadog Agent OTLP gRPC `host:port`. |
| `OTEL_GATEWAY_AUDIENCE` | gateway collector, sidecar | The ID-token audience both sides use. |
| `OTEL_GATEWAY_ALLOWED_CALLERS` | gateway collector | Comma-separated runtime service-account emails whose data is kept. |
| `LANGFUSE_OTLP_ENDPOINT` | collector | Langfuse base ending in `/api/public/otel`. |
| `LANGFUSE_PUBLIC_KEY`, `LANGFUSE_SECRET_KEY` | collector | Langfuse project keys for Basic auth. In GKE they sync from Doppler. |

## Secrets

Doppler holds every observability secret: the Datadog API and app keys, the
Langfuse salt, NextAuth secret, encryption key, ClickHouse and Valkey passwords,
the GCS HMAC pair, and the Langfuse project keys. The Doppler Kubernetes Operator
syncs them into the `observability` namespace;
<a href="/infra/gcp/observability/README%2Emd"><code>infra/gcp/observability</code></a>
says how its token arrives and maps each key.

## Local runbook

```sh
nix develop
scripts/observability-up.sh            # Langfuse + collector, Datadog spans to the debug exporter
scripts/observability-up.sh --datadog  # adds the Datadog Agent, DD_API_KEY from Doppler dev

OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318 DD_ENV=local CODA_TELEMETRY_CONTENT=full \
  CODA_LOCAL_MODEL_EXECUTION=replay CODA_CASSETTE_PATH=/absolute/path/cassette.json \
  PHYSICIAN_DATABASE_URL=postgres://user@127.0.0.1:5432/physician scripts/local-practice.sh
```

Send one physician chat. The trace then appears in Langfuse at
<http://localhost:3000> (`dev@coda.local` / `coda-local-dev`, project
`coda-agents`). The content-free copy of the same spans prints in
`docker compose -f dev/observability/compose.yaml logs -f otel-collector`
(debug), or shows in Datadog APM under `env:local` with `--datadog`.
`scripts/observability-up.sh down -v` removes the stack and its volumes. The
compose credentials are fixed local development values, and every port binds to
127.0.0.1. See <a href="/dev/observability/README%2Emd">dev/observability</a>.

## From alert to run

1. Open the notification delivered to the configured `alert_notify` destination
   (staging: Slack `#engineering-observability`).
   Check the environment, service, release version and incident time window.
   HTTP failures and latency cover service requests; asynchronous worker failures
   need a separate signal because the originating request can already have succeeded.
2. Follow the error log's trace link. The worker attempt and generation spans carry
   canonical correlation; a broker message ID identifies a delivery, not an attempt.
   Copy `coda.run_id`, `coda.work_id`, any predecessor ID, and the trace ID. Preserve
   the query window and release version with the evidence.
3. An authorized product operator reads back the matching persisted attempt from
   physician Postgres, scoped to the intended practice and physician. Chat attempts
   are in the `state.attempts` collection of `physician_runtime_conversations`;
   current reply-revision attempts are in `physician_runtime_reply_drafts`, and
   earlier accepted revisions are archived in `physician_runtime_reply_revisions`.
   Match the exact attempt ID, its work and any persisted predecessor relationship,
   and its terminal status.
   Project only those operational fields into the evidence: do not export the
   surrounding conversation, chart, instructions or model records to Datadog.
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

The `observability` job in [CI](/.github/workflows/ci.yml) validates the shared,
debug, Datadog, authenticated GKE, GKE-without-Langfuse, and inline sidecar
configurations inside the Nix shell. It validates and runs mocked OpenTofu tests
for both [`observability`](/infra/gcp/observability) and
[`rust-environments`](/infra/gcp/rust-environments). Backend-free initialization
belongs in a fresh test checkout, never over an operator's initialized backend.

Run the synthetic collector behavior test separately:

```sh
nix develop .#agent --command python scripts/test_collector_privacy.py
```

It exercises content stripping with positive controls, authenticated caller
filtering, and bounded queues during exporter outages. The generated evidence
under `target/verification` is local proof, not evidence of deployed delivery.
