#!/usr/bin/env bash
# Run only against a disposable server: suites create/drop their own databases.
# Invoke from the Nix dev shell, exactly as the postgres CI job does.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
: "${MIGRATION_DATABASE_URL:?Set the disposable Postgres admin URL}"
: "${PHYSICIAN_DATABASE_URL:?Set the physician test database URL}"
: "${FRONT_DOOR_DATABASE_URL:?Set the front-door test database URL}"
: "${DUET_IDENTITY_TEST_DATABASE_URL:?Set the dedicated disposable Duet identity database URL}"
: "${PHYSICIAN_RUNTIME_TEST_DATABASE_URL:?Set the dedicated disposable physician runtime database URL}"
export CODA_REQUIRE_POSTGRES_TESTS=1

# This ignored test fails closed if its URL is missing. It tests each of the
# two workspace migration chains separately and together, with two reruns.
cargo test --locked -p physician-sample --test migrations_postgres -- --ignored
# Tests use unique per-test databases. Run suites in sequence so their admin
# work cannot race; the sample uses PHYSICIAN_DATABASE_URL for both stores.
cargo test --locked -p physician-auth --test postgres
cargo test --locked -p front-door --test postgres
cargo test --locked -p physician-sample --test front_door_postgres
# These identity targets must never silently return without exercising Postgres.
cargo test --locked -p front-door --test identity
cargo test --locked -p front-door --test luna_journal
# Admission capability exists only in cfg(test); real SQL proof is mandatory.
cargo test --locked -p front-door --lib conversation::tests::postgres_acceptance_isolation_dedup_restart_and_withdrawal -- --exact
cargo test --locked -p front-door --lib conversation::tests::postgres_operator_switch_allowance_and_context -- --exact
cargo test --locked -p front-door --lib conversation::tests::postgres_retry_context_reports_earlier_attempt -- --exact
cargo test --locked -p duet-api --lib conversation_execution::tests::postgres_dispatch_claim_stream_completion_and_recovery -- --exact
DUET_TEST_DATABASE_URL="$DUET_IDENTITY_TEST_DATABASE_URL" cargo test --locked -p duet-api --test database_foundation -- --exact fresh_and_upgrade_keep_identity_and_enforce_account_scope
# Typed moments retain account scoping, ordering and the privacy-journal gate.
DUET_TEST_DATABASE_URL="$DUET_IDENTITY_TEST_DATABASE_URL" cargo test --locked -p duet-api --test pg_moments
# Exercises fixed GCS transport against loopback HTTP plus the real journal owner.
cargo test --locked -p duet-api --lib privacy_archive::tests::gcs_transport_and_postgres_recovery_reconcile_same_receipt_after_restart -- --exact
# Explicitly select ignored real-database proofs; serial execution prevents
# migration and recovery scans racing other suites. Cluster crash/restart is
# deliberately excluded: it requires an independently owned disposable cluster.
cargo test --locked -p local-product --lib runtime::tests -- --ignored --test-threads=1
cargo test --locked -p local-product --test runtime_postgres -- --ignored --skip database_crash_restart --test-threads=1
cargo test --locked -p local-product --test runtime_publication -- --ignored --test-threads=1
cargo test --locked -p local-product --test runtime_recovery -- --ignored --test-threads=1
