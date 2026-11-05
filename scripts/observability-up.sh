#!/usr/bin/env bash
# Start the local observability stack (dev/observability/compose.yaml):
# Langfuse v3, its stores, the shared OpenTelemetry Collector, and at most one
# operations backend.
#
#   scripts/observability-up.sh            # collector prints the ops-bound spans (debug exporter)
#   scripts/observability-up.sh --datadog  # also runs the Datadog Agent; needs DD_API_KEY
#   scripts/observability-up.sh --grafana  # also runs Tempo, Loki, Prometheus, Grafana and a webhook sink
#   scripts/observability-up.sh down       # stop every profile (add -v to drop volumes)
#
# Run it from the nix dev shell. When a Doppler login exists, compose runs
# under `doppler run --project coda-rust --config dev --` so DD_API_KEY comes
# from Doppler and never touches disk; without a login it runs plainly and the
# Datadog profile needs DD_API_KEY already in the environment. The Grafana
# profile needs no secret.
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
compose_file="$repo_root/dev/observability/compose.yaml"

runner=()
if command -v doppler >/dev/null 2>&1 && doppler me >/dev/null 2>&1; then
  runner=(doppler run --project coda-rust --config dev --)
fi

if [ "${1:-}" = "down" ]; then
  shift
  exec "${runner[@]}" docker compose -f "$compose_file" --profile datadog --profile grafana down "$@"
fi

profile_args=()
overlay=debug
case "${1:-}" in
  --datadog)
    shift
    profile_args=(--profile datadog)
    overlay=datadog
    if ! "${runner[@]}" sh -c '[ -n "${DD_API_KEY:-}" ]'; then
      echo "DD_API_KEY is not set: run \`doppler login\` (coda-rust/dev holds it) or export it." >&2
      exit 1
    fi
    ;;
  --grafana)
    shift
    profile_args=(--profile grafana)
    overlay=grafana
    ;;
esac

COLLECTOR_OVERLAY="$overlay" "${runner[@]}" \
  docker compose -f "$compose_file" "${profile_args[@]}" up -d --wait "$@"

cat <<EOF
Langfuse:   http://localhost:${LANGFUSE_PORT:-3000}  (dev@coda.local / coda-local-dev)
Collector:  OTLP http://localhost:4318, gRPC localhost:4317
Backend:    ${overlay}
EOF
if [ "$overlay" = grafana ]; then
  cat <<EOF
Grafana:    http://localhost:3300  (admin / coda-local-dev)
Tempo API:  http://localhost:3200   Loki API: http://localhost:3100   Prometheus: http://localhost:9091
Alerts:     Slack contact point posts to the local sink: http://localhost:3301/received
EOF
fi
backend_env=""
[ "$overlay" = grafana ] && backend_env="CODA_TELEMETRY_BACKEND=grafana "
cat <<EOF
Point a service at it:
  ${backend_env}OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318 DD_ENV=local CODA_TELEMETRY_CONTENT=full \\
    PHYSICIAN_DATABASE_URL=postgres://user@127.0.0.1:5432/physician scripts/local-practice.sh
Collector debug output: docker compose -f $compose_file logs -f otel-collector
EOF
