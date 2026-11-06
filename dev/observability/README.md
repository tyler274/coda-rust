# Local observability stack

Self-hosted Langfuse v3 (web, worker, Postgres, ClickHouse, Valkey, MinIO),
the shared OpenTelemetry Collector, and at most one operations backend: the
Datadog Agent or the Grafana stack (Tempo, Loki, Prometheus, Grafana). The
full data flow and content policy are in [book/src/observability.md](../../book/src/observability.md).

```sh
scripts/observability-up.sh            # Langfuse + collector; ops spans and logs go to the debug exporter
scripts/observability-up.sh --datadog  # also the Datadog Agent, DD_API_KEY from Doppler `dev`
scripts/observability-up.sh --grafana  # also Tempo, Loki, Prometheus, Grafana and a webhook sink; no secrets
scripts/observability-up.sh down -v    # stop every profile and drop volumes
```

Then point a service at the collector (add `CODA_TELEMETRY_BACKEND=grafana`
with `--grafana`):

```sh
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318 DD_ENV=local CODA_TELEMETRY_CONTENT=full \
  PHYSICIAN_DATABASE_URL=postgres://user@127.0.0.1:5432/physician scripts/local-practice.sh
```

| Surface | Address |
| --- | --- |
| Langfuse UI | <http://localhost:3000> (`LANGFUSE_PORT` overrides the host port; 3030 belongs to the worker), `dev@coda.local` / `coda-local-dev` |
| Collector OTLP | `http://localhost:4318` (HTTP), `localhost:4317` (gRPC) |
| Collector health | <http://localhost:13133> |
| Ops-bound spans and logs (debug) | `docker compose -f dev/observability/compose.yaml logs -f otel-collector` |
| Grafana (`--grafana`) | <http://localhost:3300>, `admin` / `coda-local-dev` |
| Tempo, Loki, Prometheus APIs (`--grafana`) | <http://localhost:3200>, <http://localhost:3100>, <http://localhost:9091> |
| Alerts received (`--grafana`) | <http://localhost:3301/received>, or `docker compose -f dev/observability/compose.yaml logs webhook-sink` |

## Files

- `collector.yaml` is the one collector config. `infra/gcp/observability`
  mounts the same file, so a change here changes that deployment definition. An actual rollout
  and readback are separate.
  Endpoints and Langfuse keys are `${env:...}` references. It carries traces
  to the operations pipeline `traces/ops` and to Langfuse, and logs to
  `logs/ops` only. Both ops pipelines export to `nop` until a backend overlay
  replaces their exporters, so a missing overlay drops data.
- `collector.datadog.yaml`, `collector.grafana.yaml` and `collector.debug.yaml`
  are the backend overlays, merged **last**. Each sets only the ops pipelines'
  exporters: `otlp/datadog` (`DATADOG_OTLP_ENDPOINT`); `otlp/tempo`
  (`TEMPO_OTLP_ENDPOINT`) and `otlphttp/loki` (`LOKI_OTLP_ENDPOINT`, Loki's
  `/otlp`); or `debug` (local only). The content-stripping processors run in
  all three, so the debug output is exactly what either backend would get.
  Compose passes exactly one after the base file (`COLLECTOR_OVERLAY`); the
  GKE gateway passes `collector.<telemetry_backend>.yaml`.
- `collector.gke.yaml` is merged after the base file only on the GKE gateway.
  It requires a Google ID token on both OTLP receivers and keeps only spans
  and log records from the service accounts in `OTEL_GATEWAY_ALLOWED_CALLERS`.
- `collector.gke-no-langfuse.yaml` is merged after `collector.gke.yaml`, before
  the backend overlay, while `infra/gcp/observability` has
  `langfuse_enabled = false`. It sends the generation pipeline to `nop`, so
  the gateway needs no Langfuse endpoint or keys; the ops pipelines are
  unchanged.
- `sidecar.yaml` is the Cloud Run sidecar collector. It forwards the service's
  localhost OTLP to the gateway with the runtime service account's ID token.
- `grafana/` holds the Grafana stack configs, read unchanged by the compose
  `grafana` profile and by `infra/gcp/observability/grafana.tf`:
  - `tempo.yaml`: monolithic Tempo on local storage, retention
    `TEMPO_BLOCK_RETENTION` (default 14 days). Its metrics-generator writes
    span metrics and a service graph to `PROMETHEUS_REMOTE_WRITE_URL`, with
    only `coda.job`, `gen_ai.system` and `deployment.environment` as extra
    dimensions; run ids never become labels.
  - `loki.yaml`: single-binary Loki with OTLP ingestion, retention
    `LOKI_RETENTION_PERIOD` (default 30 days). `service.name` and
    `deployment.environment` are labels; `trace_id`, `coda.run_id` and the
    other attributes are structured metadata.
  - `prometheus.yaml`: receives remote write only; retention is a flag.
  - `provisioning/datasources.yaml`: datasources with fixed UIDs `tempo`,
    `loki` and `prometheus`, linked trace to logs on `trace_id`, log to trace
    through the `trace_id` field, and exemplars to traces.
  - `provisioning/alerting/rules.yaml`: the four alerts of
    `infra/gcp/observability/datadog.tf` with the same thresholds, fixed in the
    file. A tofu precondition keeps them equal to that root's threshold
    variables.
  - `provisioning/alerting/contact.yaml`: the `coda-slack` contact point,
    posting to `GRAFANA_SLACK_WEBHOOK_URL`.
  - `webhook-sink.py`: the local stand-in for Slack. Compose points
    `GRAFANA_SLACK_WEBHOOK_URL` at it, so local alerts never reach Slack.
- `compose.yaml` pins every image. Its credentials are fixed **local
  development values**, not secrets: the ports bind to 127.0.0.1 and the
  stack holds synthetic data. `LANGFUSE_INIT_*` creates the org, project,
  user, and the project keys `pk-lf-coda-local-dev` / `sk-lf-coda-local-dev`
  on first boot; the Grafana admin login is likewise a local constant.
  `DD_API_KEY` is the only real secret and is read from the environment
  (Doppler) at start.

## Offline downtime-notification rehearsal

`qualify-alerts.py` is a local rehearsal/evidence checker, not a notification
sender. It cannot contact providers, submit metrics, inject failures or apply
configuration; `--live` always fails before reading inputs. It emits the actual
evaluator steps to review with an operator, with separate Datadog primary,
Cloud Monitoring watchdog and Grafana test routes. These steps do not establish
that an evaluator or contact exists.

```sh
nix develop .#agent --command python dev/observability/qualify-alerts.py
nix develop .#agent --command python scripts/test_alert_qualification.py -v
```

Copy `qualification.json` outside tracked source for a qualification's verified
input contract. Null fields are deliberate: supply the actual dedicated
synthetic evaluator/policy ID, signal descriptor/selector, exact query and
thresholds, sampling/alignment/pending/recovery timing, notification grouping,
reminder/maintenance settings, Slack account and channel ID, contact ID, and
readback evidence references. Do not put webhook URLs or keys in this contract.
Use Stage2's actual probe/Cloud Monitoring outputs; never fabricate a watchdog
policy, metric name, target or channel. Datadog's primary route is
`#engineering-observability`. The watchdog must evaluate and notify independently
of GKE, Grafana and the watched collector. Grafana operational rules must remain
shadow-only; qualification uses a separately labeled test rule and route.
The shared Grafana provisioning currently routes every rule to its contact;
this rehearsal does not change that configuration or qualify concurrent shadow
operation. Inspect the intended shadow/test provisioning before any live test.

The printed policy is a **proposal to review**, not deployed configuration:
one stable environment/target/symptom group, 15-minute reminders capped at two,
40-minute maximum exercise, recovery on the same route, and time-bounded
synthetic-only maintenance. Read back actual provider behavior, including
recovery and maintenance-expiry semantics, before approving a drill. Provider
features differ; a reminder cap that a provider cannot express needs an explicit
bounded operator stop/cleanup procedure, not a claim that the provider enforces it.
The 3-failure/2-success minute cadence and 5-minute heartbeat absence are intended
readiness criteria; configured evaluator windows and ingestion delays must be
measured rather than inferred from that cadence.

After an approved exercise, `--observations /path/to/observations.json` checks a
local JSON array, one route/group/exercise per ledger. Each record has `route`
(`datadog`, `cloud-monitoring`, `grafana-test`), `kind`, original UTC `at`
(RFC3339 with up to nine fractional digits), stable `group`, `source`, and
an inspectable `evidence` reference. Keep source/deployed revisions and the
approved time window with those artifacts. Record `signal_failure`, `pending`
(if exposed), `firing`, `provider_notified`, `slack_alert`, `reminder`,
`signal_restored`, `recovered`, `slack_recovery`, `maintenance_start` and
`maintenance_end` as observed, not as predicted timestamps. For the watchdog,
`signal_failure` is the first withheld heartbeat's scheduled time; preserve
the last received heartbeat and absence-window calculation in its evidence.
If a provider does not expose Pending, note that fact in its evaluator evidence.

Transitions use `source: evaluator-history`; `provider_notified` uses
`source: provider-metadata` and preserves the provider's original `last_notified`
value in the referenced artifact. Slack receipts require
`source: human-confirmed-slack`, `confirmed_by`, and `message_ref` (message
permalink or exact channel/message timestamp). `at` is the Slack message time,
not when an operator later noticed it; retain the confirmation time separately
in the evidence. A provider send timestamp, HTTP success, or this stack's
`local-webhook-sink` receipt is **not** a Slack receipt. The sink adds its exact
UTC receipt timestamp while preserving its payload and `/received` interface.

The checker preserves records and computes decimal failure→firing,
failure→Slack, firing→Slack, restore→recovery and recovery→Slack intervals.
It flags missing observations, unstable grouping, excessive/early reminders,
notifications during maintenance, unfinished maintenance and out-of-order times.
Even clean input is only an offline consistency check: it always returns
`live_qualified: false`. Inspect the referenced artifacts yourself; no parser can
authenticate a human assertion. Separate live proof is required for routing,
no notifications through the complete maintenance/delivery window, re-alerting
after expiry, watchdog independence and absence of duplicate Grafana pages.
An evidence check exits nonzero when a supplied route has consistency gaps;
absent routes are listed as unassessed, not failed exercises. Empty ledgers are
rejected. Supplied Pending transitions must fall between failure and Firing.
The default rehearsal exits zero while listing unconfigured inputs. Neither
exit status is a live acceptance verdict.

## Validate

```sh
LANGFUSE_PUBLIC_KEY=x LANGFUSE_SECRET_KEY=y \
  otelcol-contrib validate --config dev/observability/collector.yaml
for backend in debug datadog grafana; do
  LANGFUSE_PUBLIC_KEY=x LANGFUSE_SECRET_KEY=y \
    otelcol-contrib validate --config dev/observability/collector.yaml --config dev/observability/collector.$backend.yaml
done
for backend in datadog grafana; do
  LANGFUSE_PUBLIC_KEY=x LANGFUSE_SECRET_KEY=y OTEL_GATEWAY_AUDIENCE=aud OTEL_GATEWAY_ALLOWED_CALLERS=a@p.iam.gserviceaccount.com \
    otelcol-contrib validate --config dev/observability/collector.yaml --config dev/observability/collector.gke.yaml \
    --config dev/observability/collector.$backend.yaml
  OTEL_GATEWAY_AUDIENCE=aud OTEL_GATEWAY_ALLOWED_CALLERS=a@p.iam.gserviceaccount.com \
    otelcol-contrib validate --config dev/observability/collector.yaml --config dev/observability/collector.gke.yaml \
    --config dev/observability/collector.gke-no-langfuse.yaml --config dev/observability/collector.$backend.yaml
done
OTEL_SIDECAR_CONFIG="$(cat dev/observability/sidecar.yaml)" OTEL_GATEWAY_ENDPOINT=http://10.0.0.1:4318 OTEL_GATEWAY_AUDIENCE=aud \
  otelcol-contrib validate --config=env:OTEL_SIDECAR_CONFIG
docker compose -f dev/observability/compose.yaml --profile datadog --profile grafana config -q
```

CI also verifies `grafana/tempo.yaml`, `grafana/loki.yaml` and
`grafana/prometheus.yaml` with their pinned images; see the `observability`
job in [ci.yml](../../.github/workflows/ci.yml).
