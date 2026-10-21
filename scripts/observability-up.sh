#!/usr/bin/env bash
# Start the local observability stack (dev/observability/compose.yaml):
# Langfuse v3, its stores, and the shared OpenTelemetry Collector.
#
#   scripts/observability-up.sh            # collector prints the Datadog-bound span (debug exporter)
#   scripts/observability-up.sh --datadog  # also runs the Datadog Agent; needs DD_API_KEY
#   scripts/observability-up.sh down       # stop (add -v to drop volumes)
#
# Run it from the nix dev shell. When a Doppler login exists, compose runs
# under `doppler run --project coda-rust --config dev --` so DD_API_KEY comes
# from Doppler and never touches disk; without a login it runs plainly and the
# Datadog profile needs DD_API_KEY already in the environment.
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
compose_file="$repo_root/dev/observability/compose.yaml"

runner=()
if command -v doppler >/dev/null 2>&1 && doppler me >/dev/null 2>&1; then
  runner=(doppler run --project coda-rust --config dev --)
fi

if [ "${1:-}" = "down" ]; then
  shift
  exec "${runner[@]}" docker compose -f "$compose_file" --profile datadog down "$@"
fi

profile_args=()
overlay=debug
if [ "${1:-}" = "--datadog" ]; then
  shift
  profile_args=(--profile datadog)
  overlay=datadog
  if ! "${runner[@]}" sh -c '[ -n "${DD_API_KEY:-}" ]'; then
    echo "DD_API_KEY is not set: run \`doppler login\` (coda-rust/dev holds it) or export it." >&2
    exit 1
  fi
fi

COLLECTOR_OVERLAY="$overlay" "${runner[@]}" \
  docker compose -f "$compose_file" "${profile_args[@]}" up -d --wait "$@"

cat <<EOF
Langfuse:   http://localhost:${LANGFUSE_PORT:-3000}  (dev@coda.local / coda-local-dev)
Collector:  OTLP http://localhost:4318, gRPC localhost:4317
Datadog:    ${overlay}
Point a service at it:
  OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318 DD_ENV=local CODA_TELEMETRY_CONTENT=full \\
    cargo run -p physician-sample
Collector debug output: docker compose -f $compose_file logs -f otel-collector
EOF
