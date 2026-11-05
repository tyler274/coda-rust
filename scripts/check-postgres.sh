#!/usr/bin/env bash
# Run only against a disposable server: suites create/drop their own databases.
# Invoke from the Nix dev shell, exactly as the postgres CI job does.
# CI runs it as two shards (POSTGRES_SHARD=1 and 2) that together run every
# suite once; unset, it runs both in sequence.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
: "${MIGRATION_DATABASE_URL:?Set the disposable Postgres admin URL}"
: "${PHYSICIAN_DATABASE_URL:?Set the physician test database URL}"
: "${FRONT_DOOR_DATABASE_URL:?Set the front-door test database URL}"
: "${DUET_IDENTITY_TEST_DATABASE_URL:?Set the dedicated disposable Duet identity database URL}"
: "${PHYSICIAN_RUNTIME_TEST_DATABASE_URL:?Set the dedicated disposable physician runtime database URL}"
shard="${POSTGRES_SHARD:-all}"
case "$shard" in 1 | 2 | all) ;; *) echo "POSTGRES_SHARD must be 1, 2 or unset" >&2; exit 2 ;; esac
export CODA_REQUIRE_POSTGRES_TESTS=1

# Every call selects the same packages, so Cargo resolves features once and
# builds each crate once. A different -p per call re-resolves dev-dependency
# features and rebuilds the shared crates. A target name selects only the
# packages that have it (`--test postgres` runs physician-auth's and
# front-door's suites, as two separate calls did).
packages=(-p physician-sample -p physician-auth -p front-door -p duet-api
  -p local-product -p physician-bus -p pubsub -p gmail)
pg_test() { cargo test --locked "${packages[@]}" "$@"; }

# Shard 1 builds most of the workspace, so shard 2 takes the larger share of
# test time: four slow physician scale suites and the Pub/Sub emulator suites.
shard_1_targets=(--lib --test migrations_postgres --test postgres
  --test front_door_postgres --test identity --test luna_journal
  --test database_foundation --test pg_moments --test steering
  --test runtime_postgres --test runtime_publication --test runtime_recovery
  --test redis --test fanout --test worker --test recovery --test try_again)
shard_2_targets=(--test stop --test chat_draft --test reply_revision
  --test instance_loss --test emulator --test push_wake)

shard_1() {
  # This ignored test fails closed if its URL is missing. It tests each of the
  # two workspace migration chains separately and together, with two reruns.
  pg_test --test migrations_postgres -- --ignored
  # Tests use unique per-test databases. Run suites in sequence so their admin
  # work cannot race; the sample uses PHYSICIAN_DATABASE_URL for both stores.
  pg_test --test postgres
  pg_test --test front_door_postgres
  # These identity targets must never silently return without exercising Postgres.
  pg_test --test identity
  pg_test --test luna_journal
  # Admission capability exists only in cfg(test); real SQL proof is mandatory.
  pg_test --lib conversation::tests::postgres_acceptance_isolation_dedup_restart_and_withdrawal -- --exact
  pg_test --lib conversation::tests::postgres_operator_switch_allowance_and_context -- --exact
  pg_test --lib conversation::tests::postgres_retry_context_reports_earlier_attempt -- --exact
  pg_test --lib conversation_execution::tests::postgres_dispatch_claim_stream_completion_and_recovery -- --exact
  DUET_TEST_DATABASE_URL="$DUET_IDENTITY_TEST_DATABASE_URL" pg_test --test database_foundation -- --exact fresh_and_upgrade_keep_identity_and_enforce_account_scope
  # Typed moments retain account scoping, ordering and the privacy-journal gate.
  DUET_TEST_DATABASE_URL="$DUET_IDENTITY_TEST_DATABASE_URL" pg_test --test pg_moments
  # ENG-200: a same-conversation message steers the reply in progress; one paid
  # run, idempotent retries, restart survival and the ENG-191 refusals.
  DUET_TEST_DATABASE_URL="$DUET_IDENTITY_TEST_DATABASE_URL" pg_test --test steering
  # Exercises fixed GCS transport against loopback HTTP plus the real journal owner.
  pg_test --lib privacy_archive::tests::gcs_transport_and_postgres_recovery_reconcile_same_receipt_after_restart -- --exact
  # Explicitly select ignored real-database proofs; serial execution prevents
  # migration and recovery scans racing other suites. Cluster crash/restart is
  # deliberately excluded: it requires an independently owned disposable cluster.
  pg_test --lib runtime::tests -- --ignored --test-threads=1
  pg_test --test runtime_postgres -- --ignored --skip database_crash_restart --test-threads=1
  pg_test --test runtime_publication -- --ignored --test-threads=1
  pg_test --test runtime_recovery -- --ignored --test-threads=1
  # Physician scale proofs. Valkey and the Pub/Sub emulator come from the nix
  # dev shell; each test creates its own database on PHYSICIAN_DATABASE_URL.
  # The bus suite needs Valkey only. The workspace CI job has no Nix shell, so
  # it skips; this job fail-closes when valkey-server is missing.
  pg_test --test redis -- --test-threads=1
  pg_test --test fanout -- --test-threads=1
  pg_test --test worker -- --test-threads=1
  # A failed publish is republished by the recovery sweep; a dead-lettered turn
  # shows "couldn't finish" and Try again (work.resume) gets the reply.
  pg_test --test recovery -- --test-threads=1
  pg_test --test try_again -- --test-threads=1
}

shard_2() {
  # Stop holds the turn across a worker restart; a second Stop holds the resumed
  # run; a message sent while held is answered without either stopped reply.
  pg_test --test stop -- --test-threads=1
  # Composer chat drafts: cross-site reads, stale-revision conflicts and physician
  # ownership, without a worker. The runtime draft-fence proof runs in shard 1.
  pg_test --test chat_draft -- --test-threads=1
  # Hosted reply revisions: duplicate acceptance, retries re-pinning the newest
  # sources, a source arriving mid-revision read before publishing, a chart that
  # keeps changing exhausting the call bound, cross-site reads, physician-save
  # fencing, missing/foreign draft isolation, earlier revisions' evidence kept in
  # their own history table, and a failed revision visible over HTTP.
  # Receipt compaction and the pre-accepted_at upgrade proof run with runtime::tests.
  pg_test --test reply_revision -- --test-threads=1
  # A site instance that dies before its publish lands: another instance's sweep
  # finishes the turn. Exhausted worker leases fail the turn, announce it and
  # dead-letter its wakeup.
  pg_test --test instance_loss -- --test-threads=1
  # Gmail push wake on the Pub/Sub emulator: the shared harness, then wake-only,
  # duplicates, redelivery, dead letters, ordering keys and cursor expiry.
  pg_test --test emulator --test push_wake -- --test-threads=1
}

# Build only the targets this shard runs; the packages' other test binaries
# would add link time on a 2 vCPU runner for nothing.
case "$shard" in
  1) pg_test --no-run "${shard_1_targets[@]}"; shard_1 ;;
  2) pg_test --no-run "${shard_2_targets[@]}"; shard_2 ;;
  all) pg_test --no-run "${shard_1_targets[@]}" "${shard_2_targets[@]}"; shard_1; shard_2 ;;
esac
