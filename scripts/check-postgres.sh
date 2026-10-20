#!/usr/bin/env bash
# Run only against a disposable server: suites create/drop their own databases.
# Invoke from the Nix dev shell, exactly as the postgres CI job does.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
: "${MIGRATION_DATABASE_URL:?Set the disposable Postgres admin URL}"
: "${PHYSICIAN_DATABASE_URL:?Set the physician test database URL}"
: "${FRONT_DOOR_DATABASE_URL:?Set the front-door test database URL}"
export CODA_REQUIRE_POSTGRES_TESTS=1

# This ignored test fails closed if its URL is missing. It tests each of the
# two workspace migration chains separately and together, with two reruns.
cargo test --locked -p physician-sample --test migrations_postgres -- --ignored
# Tests use unique per-test databases. Run suites in sequence so their admin
# work cannot race; the sample uses PHYSICIAN_DATABASE_URL for both stores.
cargo test --locked -p physician-auth --test postgres
cargo test --locked -p front-door --test postgres
cargo test --locked -p physician-sample --test front_door_postgres
