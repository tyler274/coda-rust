#!/usr/bin/env bash
# Build-and-run proof for the Nix images: no registry, deployment, Doppler
# token, or cloud credential. Run in `nix develop .#release` (nix, docker
# client, python3, postgres) with a reachable Docker daemon.
#
#  1. nix build every image and inspect the archives (check-images.py).
#  2. docker load, then prove fail-closed starts: the Doppler entrypoint
#     refuses to start without a token, servers refuse Cloud Run hosting
#     (K_SERVICE) and jobs refuse missing database configuration.
#  3. Start a disposable PostgreSQL (initdb in mktemp, loopback only), run
#     both migration images twice, import the packaged physician bank twice,
#     and start the servers with PORT set:
#     /health and /readyz must answer 200. physician-worker starts hosted
#     on the image's own PHYSICIAN_AUTH and packaged skill catalog, and
#     refuses to start when its catalog directory is missing.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

images=(physician-sample-image physician-worker-image duet-api-image physician-migration-image duet-migration-image duet-privacy-delivery-image)
nix build --no-link --print-out-paths "${images[@]/#/.#}" > image-paths
mapfile -t archives < image-paths
python3 scripts/check-images.py "${archives[@]}"

declare -A tag_of
for archive in "${archives[@]}"; do
  docker load --quiet --input "$archive" >/dev/null
  tag=$(python3 -c 'import json,sys,tarfile; t=tarfile.open(sys.argv[1]); print(json.load(t.extractfile("manifest.json"))[0]["RepoTags"][0])' "$archive")
  tag_of[${tag%%:*}]=$tag
  printf '%s  %s  %s\n' "$tag" "$(du -h --apparent-size "$archive" | cut -f1)" "$archive"
done

# Cloud Run hardening analogue: read-only root, writable /tmp, no capabilities.
locked=(--read-only --tmpfs /tmp:mode=1777 --cap-drop ALL --security-opt no-new-privileges)
containers=()
# `docker run --rm` can report a daemon cleanup race (exit 125, e.g. a busy
# ZFS dataset) as the job's failure. Run attached, then report the
# container's own exit code and remove it separately.
#
# expect_failure calls run in a command substitution, so containers+= does not
# reach this shell there; $BASHPID keeps each subshell's name unique so a
# removal the daemon has not finished cannot collide with the next run.
run() {
  local container="coda-image-smoke-$$-$BASHPID-${#containers[@]}"
  containers+=("$container")
  docker run --name "$container" "$@" || true
  local code
  code=$(docker inspect -f '{{.State.ExitCode}}' "$container" 2>/dev/null) || code=125
  docker rm -f "$container" >/dev/null 2>&1 || true
  return "$code"
}
binary_of() { docker image inspect --format '{{index .Config.Labels "com.coda.binary"}}' "$1"; }
expect_failure() {
  local name=$1 pattern=$2; shift 2
  local output
  if output=$("$@" 2>&1); then
    printf '%s: unexpectedly succeeded\n%s\n' "$name" "$output" >&2
    exit 1
  fi
  if ! grep -q -- "$pattern" <<<"$output"; then
    printf '%s: wrong failure\n%s\n' "$name" "$output" >&2
    exit 1
  fi
  printf '%s: fails closed (%s)\n' "$name" "$pattern"
}

for tag in "${tag_of[@]}"; do
  # The real entrypoint: no DOPPLER_TOKEN means no secrets and no process.
  expect_failure "$tag doppler entrypoint" "you must provide a token" \
    run "${locked[@]}" --network none "$tag"
done
for name in coda-physician-sample coda-physician-worker coda-duet-api; do
  expect_failure "${tag_of[$name]} hosted start" "hosted activation is not qualified" \
    run "${locked[@]}" --network none -e K_SERVICE=qualification-smoke \
      --entrypoint "$(binary_of "${tag_of[$name]}")" "${tag_of[$name]}"
done
expect_failure "${tag_of[coda-physician-migrate]} without database" "PHYSICIAN_DATABASE_URL is required" \
  run "${locked[@]}" --network none --entrypoint "$(binary_of "${tag_of[coda-physician-migrate]}")" "${tag_of[coda-physician-migrate]}"
expect_failure "${tag_of[coda-duet-migrate]} without database" "DUET_MIGRATION_DATABASE_URL and DUET_RUNTIME_ROLE are required" \
  run "${locked[@]}" --network none --entrypoint "$(binary_of "${tag_of[coda-duet-migrate]}")" "${tag_of[coda-duet-migrate]}"
expect_failure "${tag_of[coda-duet-privacy-delivery]} without configuration" "privacy_journal_delivery_unavailable" \
  run "${locked[@]}" --network none --entrypoint "$(binary_of "${tag_of[coda-duet-privacy-delivery]}")" "${tag_of[coda-duet-privacy-delivery]}"

# Disposable PostgreSQL on loopback. Containers share the host network so the
# servers' PORT binding is reachable from this script as well.
free_port() { python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1",0)); print(s.getsockname()[1])'; }
pgdata=$(mktemp -d)
pgport=$(free_port)
cleanup() {
  for c in "${containers[@]}"; do docker rm -f "$c" >/dev/null 2>&1 || true; done
  pg_ctl -D "$pgdata/data" -m immediate stop >/dev/null 2>&1 || true
  rm -rf "$pgdata"
}
trap cleanup EXIT
initdb -D "$pgdata/data" -U postgres --auth=trust >/dev/null
pg_ctl -D "$pgdata/data" -l "$pgdata/log" -w \
  -o "-c listen_addresses=127.0.0.1 -p $pgport -k $pgdata -c fsync=off" start >/dev/null
admin="postgres://postgres@127.0.0.1:$pgport/postgres"
psql -q "$admin" \
  -c 'create role physician_migration login nosuperuser nobypassrls nocreatedb nocreaterole connection limit 4' \
  -c 'create database physician owner physician_migration' -c 'create database duet'
physician_db="postgres://physician_migration@127.0.0.1:$pgport/physician"
physician_runtime_db="postgres://postgres@127.0.0.1:$pgport/physician"
duet_db="postgres://postgres@127.0.0.1:$pgport/duet"
run_job() {
  local tag=$1; shift
  run "${locked[@]}" --network host --entrypoint "$(binary_of "$tag")" "$@" "$tag"
}

schema_state() {
  psql -At "$1" -c "select string_agg(table_name, ',' order by table_name) from information_schema.tables where table_schema = 'public'" \
    -c "select 'physician-auth=' || count(*) from public.physician_auth_migrations" \
    -c "select 'front-door=' || count(*) from public.front_door_migrations"
}
states=()
for attempt in 1 2; do
  run_job "${tag_of[coda-physician-migrate]}" -e PHYSICIAN_DATABASE_URL="$physician_db" \
    -e CODA_HOSTED_PROFILE=staging-preview \
    -e 'PHYSICIAN_STAGING_ACCOUNTS=dev@codahealth.test|Dev|admin,tyler@codahealth.test|Tyler|admin,physician@codahealth.test|Staging Physician|admin,ops@codahealth.test|Ops|admin,eli@codahealth.test|Eli|admin'
  states+=("$(schema_state "$physician_db")")
  printf 'physician migration run %s: ok (%s)\n' "$attempt" "$(tail -n2 <<<"${states[-1]}" | paste -sd' ')"
done
[ "${states[0]}" = "${states[1]}" ] || { echo "physician rerun changed the schema" >&2; exit 1; }
grep -q physician_runtime_commands <<<"${states[0]}" || { echo "runtime chain missing" >&2; exit 1; }
echo "physician migration rerun is idempotent"

# The explicit import must work from the image alone, without a source mount or
# a manually supplied chart directory. Ordinary schema migrations above do not
# import charts. The second load must preserve the stored records unchanged.
charts=$(nix build --no-link --print-out-paths .#physician-synthetic-charts)
expected=$(python3 - "$charts/manifest.json" <<'PY'
import json, sys
bank = json.load(open(sys.argv[1]))
assert bank['patients'], 'the packaged bank is empty'
print(f"{len(bank['patients'])} patients, {sum(bank['totals'].values())} records")
PY
)
[ "$(psql -At "$physician_db" -c 'select count(*) from physician_runtime_sources')" = 0 ] || {
  echo "ordinary migrations imported charts" >&2; exit 1;
}
chart_summaries=()
for attempt in 1 2; do
  if ! output=$(run "${locked[@]}" --network host \
    --entrypoint "$(binary_of "${tag_of[coda-physician-migrate]}")" \
    -e PHYSICIAN_DATABASE_URL="$physician_db" \
    -e CODA_HOSTED_PROFILE=staging-preview \
    "${tag_of[coda-physician-migrate]}" load-charts); then
    printf '%s\n' "$output" >&2
    exit 1
  fi
  printf '%s\n' "$output"
  summary=$(grep '^synthetic charts:' <<<"$output")
  [[ "$summary" == "synthetic charts: $expected, "*", 0 failed" ]] || {
    echo "packaged chart import did not match its manifest without failures" >&2; exit 1;
  }
  chart_summaries+=("$summary")
done
[[ "${chart_summaries[1]}" == *", 0 changed, 0 failed" ]] || {
  echo "packaged chart reload changed stored records" >&2; exit 1;
}
echo "packaged chart import and reload are idempotent"

# duet-migrate applies the doctor-page chain as the unprivileged runtime role,
# then the front-door chain. Both runs must record the same ledger.
psql -q "$admin" -c "create role duet_runtime login nosuperuser nobypassrls nocreatedb nocreaterole"
psql -q "$duet_db" -c "revoke create on schema public from public"
duet_states=()
for attempt in 1 2; do
  run_job "${tag_of[coda-duet-migrate]}" \
    -e DUET_MIGRATION_DATABASE_URL="$duet_db" \
    -e DUET_RUNTIME_ROLE=duet_runtime \
    -e FRONT_DOOR_DATABASE_URL="$duet_db"
  duet_states+=("$(psql -At "$duet_db" -c 'select count(*) from public.duet_schema_migrations')")
  printf 'duet migration run %s: ok (%s schema versions)\n' "$attempt" "${duet_states[-1]}"
done
[ "${duet_states[0]}" = "${duet_states[1]}" ] || { echo "duet rerun changed the schema" >&2; exit 1; }
[ "${duet_states[0]}" -gt 0 ] || { echo "duet schema chain was not applied" >&2; exit 1; }
echo "duet migration rerun is idempotent"

serve() {
  local name=$1 port=$2; shift 2
  local container="coda-image-smoke-$name-$$"
  containers+=("$container")
  servers+=("$container")
  docker run -d --name "$container" --read-only --tmpfs /tmp:mode=1777 --cap-drop ALL \
    --security-opt no-new-privileges --network host -e PORT="$port" \
    --entrypoint "$(binary_of "${tag_of[$name]}")" "$@" "${tag_of[$name]}" >/dev/null
  for _ in $(seq 1 100); do
    if python3 - "$port" <<'PY' 2>/dev/null; then break; fi
import socket, sys
socket.create_connection(("127.0.0.1", int(sys.argv[1])), 0.2).close()
PY
    if [ "$(docker inspect -f '{{.State.Running}}' "$container")" != true ]; then
      docker logs "$container" >&2
      echo "$name exited before listening" >&2
      exit 1
    fi
    sleep 0.1
  done
}
status() { python3 - "$1" <<'PY'
import sys, urllib.request, urllib.error
try:
    print(urllib.request.urlopen(sys.argv[1], timeout=5).status)
except urllib.error.HTTPError as error:
    print(error.code)
PY
}
expect_status() {
  local url=$1 want=$2 got
  got=$(status "$url")
  [ "$got" = "$want" ] || { echo "$url returned $got, expected $want" >&2; exit 1; }
  printf '%s -> %s\n' "$url" "$got"
}

servers=()
physician_port=$(free_port)
# API mode needs an explicit allowance and a key value to construct the
# router; this placeholder is never sent anywhere because no chat is submitted.
serve coda-physician-sample "$physician_port" \
  -e PHYSICIAN_DATABASE_URL="$physician_runtime_db" -e PHYSICIAN_AUTH=required \
  -e CODA_ENVIRONMENT=stg -e CODA_HOSTED_PROFILE=staging-preview \
  -e CODA_ALLOW_MODEL_CALLS=1 -e OPENAI_API_KEY=image-smoke-placeholder
expect_status "http://127.0.0.1:$physician_port/health" 200
expect_status "http://127.0.0.1:$physician_port/readyz" 200
expect_status "http://127.0.0.1:$physician_port/" 200
expect_status "http://127.0.0.1:$physician_port/physician-app_bg.wasm" 200

# The worker as staging runs it: hosted, with no PHYSICIAN_AUTH passed in, so
# the image's own setting must satisfy hosted startup. The placeholder key is
# never used because no push is sent; the audience and push account only
# configure token verification.
worker_port=$(free_port)
serve coda-physician-worker "$worker_port" \
  -e CODA_HOSTED=1 -e PHYSICIAN_DATABASE_URL="$physician_runtime_db" \
  -e CODA_ENVIRONMENT=stg -e CODA_HOSTED_PROFILE=staging-preview \
  -e CODA_ALLOW_MODEL_CALLS=1 -e OPENAI_API_KEY=image-smoke-placeholder \
  -e PHYSICIAN_WORKER_AUDIENCE=https://physician-worker.image-smoke.invalid \
  -e PHYSICIAN_PUSH_SERVICE_ACCOUNT=physician-pubsub-push@image-smoke.iam.gserviceaccount.com
expect_status "http://127.0.0.1:$worker_port/health" 200
expect_status "http://127.0.0.1:$worker_port/readyz" 200
# Listening at all proves hosted startup loaded the image's packaged catalog:
# the worker loads PHYSICIAN_SKILLS_DIR before binding, outside any source
# checkout. Pointed at a directory with no catalog, it must refuse to start.
expect_failure "${tag_of[coda-physician-worker]} without its skill catalog" "physician skill catalog at /tmp/no-skills is unavailable" \
  run "${locked[@]}" --network host \
    --entrypoint "$(binary_of "${tag_of[coda-physician-worker]}")" \
    -e CODA_HOSTED=1 -e PHYSICIAN_DATABASE_URL="$physician_db" \
    -e CODA_ENVIRONMENT=stg -e CODA_HOSTED_PROFILE=staging-preview \
    -e CODA_ALLOW_MODEL_CALLS=1 -e OPENAI_API_KEY=image-smoke-placeholder \
    -e PHYSICIAN_WORKER_AUDIENCE=https://physician-worker.image-smoke.invalid \
    -e PHYSICIAN_PUSH_SERVICE_ACCOUNT=physician-pubsub-push@image-smoke.iam.gserviceaccount.com \
    -e PHYSICIAN_SKILLS_DIR=/tmp/no-skills -e PORT="$(free_port)" \
    "${tag_of[coda-physician-worker]}"

duet_port=$(free_port)
publication_key=$(python3 -c 'import base64; print(base64.urlsafe_b64encode(b"0123456789abcdef0123456789abcdef").decode().rstrip("="))')
serve coda-duet-api "$duet_port" \
  -e DATABASE_URL="postgres://duet_runtime@127.0.0.1:$pgport/duet" \
  -e FRONT_DOOR_DATABASE_URL="$duet_db" \
  -e CODA_ENVIRONMENT=stg -e CODA_HOSTED_PROFILE=staging-preview \
  -e IDENTITY_PLATFORM_PROJECT_ID=image-smoke \
  -e IDENTITY_PLATFORM_API_KEY=image-smoke \
  -e DUET_PRIVACY_JOURNAL_DIRECTORY=/tmp/duet-journal \
  -e DUET_PUBLICATION_MASTER_KEY="$publication_key" \
  -e DUET_PUBLIC_ORIGIN=http://127.0.0.1 \
  -e DUET_SHL_VIEWER_URL=http://127.0.0.1/shl
expect_status "http://127.0.0.1:$duet_port/health" 200
expect_status "http://127.0.0.1:$duet_port/readyz" 200
expect_status "http://127.0.0.1:$duet_port/" 200
expect_status "http://127.0.0.1:$duet_port/duet-app_bg.wasm" 200

# Graceful stop on SIGTERM, as Cloud Run sends before scale-down.
for container in "${servers[@]}"; do
  docker stop --time 10 "$container" >/dev/null
  code=$(docker inspect -f '{{.State.ExitCode}}' "$container" 2>/dev/null) || code=125
  [ "$code" = 0 ] || { docker logs "$container" >&2; echo "$container did not stop cleanly on SIGTERM (exit $code)" >&2; exit 1; }
  printf '%s drained and exited 0 on SIGTERM\n' "$container"
done
echo "image proof complete"
