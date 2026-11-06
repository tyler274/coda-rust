#!/usr/bin/env bash
# Roll one Cloud Run service onto a new product image, changing nothing else.
#
#   scripts/gcp/rollout-service.sh check SERVICE RECEIPT_DIR
#   scripts/gcp/rollout-service.sh roll  SERVICE IMAGE RECEIPT_DIR
#
# Needs PROJECT and REGION and an authenticated gcloud. The infrastructure
# root owns every other template field, including the otel-collector sidecar
# and the telemetry environment. `check` validates the running shape before
# any migration; `roll` names the product container in the update so the
# sidecar is never the target, then checks that only that image changed.
# A revision that never becomes Ready fails `roll` and keeps the old traffic.
set -euo pipefail
mode=$1 service=$2
here=$(cd "$(dirname "$0")" && pwd)
shape() { python3 "$here/release_shape.py" "$@"; }
run() { gcloud --project "$PROJECT" run "$@" --region "$REGION" --quiet; }
expected=$(shape expected "$here/../../infra/gcp/rust-environments/env/staging.tfvars")

case $mode in
  check)
    receipt=$3
    run services describe "$service" --format json >"$receipt/$service-before.json"
    shape target "$receipt/$service-before.json" "$service" "$expected" >/dev/null
    ;;
  roll)
    image=$3 receipt=$4
    run services describe "$service" --format json >"$receipt/$service-before.json"
    container=$(shape target "$receipt/$service-before.json" "$service" "$expected")
    if [ -n "$container" ]; then
      run services update "$service" --container "$container" --image "$image"
    else
      run services update "$service" --image "$image"
    fi
    run services describe "$service" --format json >"$receipt/$service-after.json"
    shape verify "$receipt/$service-before.json" "$receipt/$service-after.json" "$service" "$image"
    ready=$(jq -r '.status.latestReadyRevisionName' "$receipt/$service-after.json")
    created=$(jq -r '.status.latestCreatedRevisionName' "$receipt/$service-after.json")
    [ "$ready" = "$created" ] || { echo "::error::$service revision $created is not Ready ($ready is)"; exit 1; }
    jq -e '[.status.traffic[] | select(.latestRevision == true) | .percent] | add == 100' "$receipt/$service-after.json" >/dev/null \
      || { echo "::error::$service's latest revision does not serve all traffic"; exit 1; }
    echo "$ready" >"$receipt/$service-revision"
    sidecar=$(jq -r 'if [.spec.template.spec.containers[].name] | index("otel-collector") then "with" else "without" end' "$receipt/$service-after.json")
    echo "$service revision $ready serves $image $sidecar the otel-collector sidecar (source expects observability $expected)"
    ;;
  *)
    echo "usage: $0 check SERVICE RECEIPT_DIR | roll SERVICE IMAGE RECEIPT_DIR" >&2
    exit 2
    ;;
esac
