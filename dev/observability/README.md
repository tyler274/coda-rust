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
  cargo run -p physician-sample
```

| Surface | Address |
| --- | --- |
| Langfuse UI | <http://localhost:3000> (`LANGFUSE_PORT` overrides the host port), `dev@coda.local` / `coda-local-dev` |
| Collector OTLP | `http://localhost:4318` (HTTP), `localhost:4317` (gRPC) |
| Collector health | <http://localhost:13133> |
| Datadog-bound spans | `docker compose -f dev/observability/compose.yaml logs -f otel-collector` |

## Files

- `collector.yaml` is the one collector config. `infra/gcp/observability`
  mounts the same file, so a change here is a change to the deployed pipeline.
  Endpoints and Langfuse keys are `${env:...}` references.
- `collector.debug.yaml` and `collector.datadog.yaml` are local overlays.
  Compose passes exactly one of them after the base file (`COLLECTOR_OVERLAY`),
  choosing the exporter of the Datadog pipeline. The content-stripping
  processor runs in both, so the debug output is exactly what Datadog would get.
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
docker compose -f dev/observability/compose.yaml config -q
```
