#!/usr/bin/env bash
# Applies the coda-app backend migration chain to the dev container's
# PostgreSQL, in sorted order, idempotently. The chain's owner is coda-app's
# backend/migrations; this script is a mechanical applier, not a schema owner:
# it never edits or reorders the files, and it fails closed on any error.
#
# Usage:
#   bash scripts/devcontainer-apply-migrations.sh [psql-function...]
# With no arguments it runs `psql -v ON_ERROR_STOP=1 -h 127.0.0.1 -p 5433
# -U coda -d duet` directly (host-side use).
set -euo pipefail

workspace=$(cd "$(dirname "$0")/.." && pwd)
migrations_dir="$workspace/submodules/coda-app/backend/migrations"

if [ $# -gt 0 ]; then
  # shellcheck disable=SC2124
  psql_cmd=("$@")
else
  psql_cmd=(psql -v ON_ERROR_STOP=1 -h 127.0.0.1 -p 5433 -U coda -d duet)
fi

if [ ! -d "$migrations_dir" ]; then
  echo "migrations directory missing: $migrations_dir" >&2
  echo "initialize the coda-app submodule first" >&2
  exit 1
fi

applied=0
skipped=0
for file in "$migrations_dir"/*.sql; do
  version=$(basename "$file" .sql)
  # 0001 creates public.schema_migrations itself, so a failed existence probe
  # on an empty database is expected and means "not applied".
  if "${psql_cmd[@]}" -tAc "select 1 from public.schema_migrations where version = '$version'" 2>/dev/null | grep -q 1; then
    skipped=$((skipped + 1))
    continue
  fi
  echo "applying $version"
  "${psql_cmd[@]}" -f "$file"
  applied=$((applied + 1))
done

echo "migrations: $applied applied, $skipped already present"
