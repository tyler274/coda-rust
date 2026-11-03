# Observability

The observability stack uses Datadog for
operations (APM, logs, Error Tracking, monitors, paging) and **self-hosted
Langfuse** for agent runs. The two use the same OpenTelemetry trace. Datadog
gets every span with agent content removed. Langfuse gets only model
generations and stays inside our GCP project. The collector strips content from
Datadog unconditionally, so no content depends on Datadog's BAA scope.

This page covers implementation and validation only. No Datadog org, monitor, or
Langfuse deployment is changed from this repository.

## Data flow

```text
physician-sample, duet-api            crates/telemetry (tracing + OTLP/HTTP)
        │  OTLP 4318
        ▼
otel-collector  (dev/observability/collector.yaml, one file everywhere)
  ├─ traces/datadog   memory_limiter → transform/strip_content → batch → Datadog Agent (OTLP 4317) → Datadog
  └─ traces/langfuse  memory_limiter → filter/langfuse_generations → batch → Langfuse /api/public/otel (Basic auth)

stdout JSON logs (dd.trace_id, dd.span_id) → Datadog Agent log collection → Datadog Logs
```

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
- **Logs.** When an OTLP endpoint is set (or `CODA_LOG_FORMAT=json`), logs are
  JSON lines with `dd.trace_id` and `dd.span_id`, so a Datadog log opens its
  trace. A log line carries its event's fields, never span fields, so content
  recorded on a generation span never reaches log storage.

## Content policy

| Where | Rule |
| --- | --- |
| Service | `CODA_TELEMETRY_CONTENT=none` (default) records no prompt or completion text. `full` attaches it as `langfuse.observation.input/output`. Simulation traffic may use `full` anywhere. Real traffic may use it only when spans go to the in-project Langfuse. |
| Collector → Datadog | `transform/strip_content` always deletes `gen_ai.prompt*`, `gen_ai.completion*`, `gen_ai.{input,output}.messages*`, and `langfuse.{observation,trace}.{input,output}*`, on span and span event attributes. The policy setting does not change this. Model, usage, and run ids stay. |
| Collector → Langfuse | `filter/langfuse_generations` keeps only spans with `langfuse.observation.type` or the `gen_ai.*` attributes the crate sets. HTTP and infrastructure spans stay in Datadog. |
| Datadog logs | The Agent excludes container logs from the `observability` namespace, because Langfuse can log payloads on errors. |
| Langfuse | ClusterIP only. People use `kubectl port-forward`, or an internal IAP gateway (`langfuse_iap_enabled`, default off). Retention defaults to 90 days. |

`tofu test` in `infra/gcp/observability` reads the deployed collector config
and asserts that every content key is deleted on both span contexts and that model and run ids
survive. The local debug overlay runs the same processor, so what it prints is
what Datadog would receive.

## Environment variables

| Variable | Set on | Meaning |
| --- | --- | --- |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | services | Collector OTLP/HTTP base. Unset means no span export (tests and the default sample server). Locally `http://localhost:4318`; in GKE `http://otel-collector.observability.svc.cluster.local:4318`. |
| `DD_ENV` | services | `local`, `staging`, or `production`. Falls back to `CODA_ENV`. |
| `DD_VERSION` | services | Release version. `physician-runtime` uses the image digest prefix. |
| `CODA_TELEMETRY_CONTENT` | services | `none` (default) or `full`. |
| `CODA_LOG_FORMAT` | services | `json` forces JSON logs without an OTLP endpoint. |
| `DATADOG_OTLP_ENDPOINT` | collector | Datadog Agent OTLP gRPC `host:port`. |
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

1. A Datadog monitor pages `alert_notify`. The three monitors are
   physician-site 5xx rate, physician chat generation errors (error logs
   emitted inside the generation span, `@span:gen_ai.generation`), and
   physician-site p95 latency.
2. Open the failing trace from the monitor, or open an error log, which links to its trace through
   `dd.trace_id`. The `gen_ai.generation` span shows the model, token usage,
   duration, and error status, but no content.
3. Copy `coda.run_id` (or the trace id) from that span.
4. In Langfuse (port-forward or the IAP host), filter traces by
   `coda.run_id` metadata or open the trace id directly. The environment is
   `coda.origin`, and tags include `coda.job`. The generation shows input and
   output only when the service ran with `CODA_TELEMETRY_CONTENT=full`.
5. Compare `coda.recipe_version` across runs to see whether a prompt or recipe
   change caused the regression.

## Validation

CI (`.github/workflows/ci.yml`, job `observability`) runs these inside
`nix develop`:

```sh
LANGFUSE_PUBLIC_KEY=x LANGFUSE_SECRET_KEY=y otelcol-contrib validate --config dev/observability/collector.yaml
LANGFUSE_PUBLIC_KEY=x LANGFUSE_SECRET_KEY=y otelcol-contrib validate \
  --config dev/observability/collector.yaml --config dev/observability/collector.debug.yaml
tofu -chdir=infra/gcp/observability init -backend=false && tofu -chdir=infra/gcp/observability validate && tofu -chdir=infra/gcp/observability test
tofu -chdir=infra/gcp/physician-runtime init -backend=false && tofu -chdir=infra/gcp/physician-runtime validate && tofu -chdir=infra/gcp/physician-runtime test
```
