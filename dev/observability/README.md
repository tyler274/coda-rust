# Local observability stack

Self-hosted Langfuse v3 (web, worker, Postgres, ClickHouse, Valkey, MinIO),
the shared OpenTelemetry Collector, and an optional Datadog Agent. The full
data flow and content policy are in [book/src/observability.md](../../book/src/observability.md).

```sh
scripts/observability-up.sh            # Langfuse + collector; Datadog spans go to the debug exporter
scripts/observability-up.sh --datadog  # also the Datadog Agent, DD_API_KEY from Doppler `dev`
scripts/observability-up.sh down -v    # stop and drop volumes
```

Then point a service at the collector:

```sh
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318 DD_ENV=local CODA_TELEMETRY_CONTENT=full \
  PHYSICIAN_DATABASE_URL=postgres://user@127.0.0.1:5432/physician scripts/local-practice.sh
```

| Surface | Address |
| --- | --- |
| Langfuse UI | <http://localhost:3000> (`LANGFUSE_PORT` overrides the host port; 3030 belongs to the worker), `dev@coda.local` / `coda-local-dev` |
| Collector OTLP | `http://localhost:4318` (HTTP), `localhost:4317` (gRPC) |
| Collector health | <http://localhost:13133> |
| Datadog-bound spans | `docker compose -f dev/observability/compose.yaml logs -f otel-collector` |

## Files

- `collector.yaml` is the one collector config. `infra/gcp/observability`
  mounts the same file, so a change here changes that deployment definition. An actual rollout
  and readback are separate.
  Endpoints and Langfuse keys are `${env:...}` references. It carries traces
  to Datadog and Langfuse and logs to Datadog only.
- `collector.gke.yaml` is merged after the base file only on the GKE gateway.
  It requires a Google ID token on both OTLP receivers and keeps only spans
  and log records from the service accounts in `OTEL_GATEWAY_ALLOWED_CALLERS`.
- `collector.gke-no-langfuse.yaml` is merged last on the gateway while
  `infra/gcp/observability` has `langfuse_enabled = false`. It sends the
  generation pipeline to `nop`, so the gateway needs no Langfuse endpoint or
  keys; the Datadog pipelines are unchanged.
- `sidecar.yaml` is the Cloud Run sidecar collector. It forwards the service's
  localhost OTLP to the gateway with the runtime service account's ID token.
- `collector.debug.yaml` and `collector.datadog.yaml` are local overlays.
  Compose passes exactly one of them after the base file (`COLLECTOR_OVERLAY`),
  choosing the exporter of the Datadog pipelines. The content-stripping
  processors run in both, so the debug output is exactly what Datadog would get.
- `compose.yaml` pins every image. Its credentials are fixed **local
  development values**, not secrets: the ports bind to 127.0.0.1 and the
  stack holds synthetic data. `LANGFUSE_INIT_*` creates the org, project,
  user, and the project keys `pk-lf-coda-local-dev` / `sk-lf-coda-local-dev`
  on first boot. `DD_API_KEY` is the only real secret and is read from the
  environment (Doppler) at start.

## Validate

```sh
LANGFUSE_PUBLIC_KEY=x LANGFUSE_SECRET_KEY=y \
  otelcol-contrib validate --config dev/observability/collector.yaml
LANGFUSE_PUBLIC_KEY=x LANGFUSE_SECRET_KEY=y \
  otelcol-contrib validate --config dev/observability/collector.yaml --config dev/observability/collector.debug.yaml
LANGFUSE_PUBLIC_KEY=x LANGFUSE_SECRET_KEY=y OTEL_GATEWAY_AUDIENCE=aud OTEL_GATEWAY_ALLOWED_CALLERS=a@p.iam.gserviceaccount.com \
  otelcol-contrib validate --config dev/observability/collector.yaml --config dev/observability/collector.gke.yaml
OTEL_GATEWAY_AUDIENCE=aud OTEL_GATEWAY_ALLOWED_CALLERS=a@p.iam.gserviceaccount.com \
  otelcol-contrib validate --config dev/observability/collector.yaml --config dev/observability/collector.gke.yaml \
  --config dev/observability/collector.gke-no-langfuse.yaml
OTEL_SIDECAR_CONFIG="$(cat dev/observability/sidecar.yaml)" OTEL_GATEWAY_ENDPOINT=http://10.0.0.1:4318 OTEL_GATEWAY_AUDIENCE=aud \
  otelcol-contrib validate --config=env:OTEL_SIDECAR_CONFIG
docker compose -f dev/observability/compose.yaml config -q
```
