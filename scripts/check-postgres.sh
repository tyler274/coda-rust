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
  -p local-product -p physician-bus)
pg_test() { cargo test --locked "${packages[@]}" "$@"; }

# Shard 1 builds most of the workspace, so shard 2 takes the larger share of
# test time: the four slow physician scale suites.
shard_1_targets=(--lib --test migrations --test migrations_postgres --test postgres
  --test front_door_postgres --test identity --test luna_journal
  --test database_foundation
  --test runtime_postgres --test runtime_publication --test runtime_recovery --test runtime_quota
  --test chat_finish --test cancelled_read --test concerns_postgres --test practice_jobs_dispatch
  --test source_refresh_postgres --test practice_schedule_postgres --test physician_run_recovery
  --test patient_reviews_postgres --test visit_briefings_postgres --test today_updates_postgres
  --test spruce_matching --test reply_proposals --test follow_ups_postgres
  --test withdrawal_postgres --test attachments
  --test staff_practice_accounts --test source_check_and_reconnect
  --test redis --test fanout --test worker --test try_again --test pool_budget
  --bin physician-worker)
shard_2_targets=(--test stop --test chat_draft --test feedback --test reply_revision
  --test instance_loss --test physician_chat_recipe --test physician_chat_mid_answer_arrivals
  --test initial_understanding_postgres)

shard_1() {
  # This ignored test fails closed if its URL is missing. It tests each of the
  # two workspace migration chains separately and together, with two reruns.
  PHYSICIAN_TEST_DATABASE_URL="${PHYSICIAN_TEST_DATABASE_URL:-${PHYSICIAN_DATABASE_URL/postgres:\/\//postgresql://}}" \
    FRONT_DOOR_TEST_DATABASE_URL="${FRONT_DOOR_TEST_DATABASE_URL:-${FRONT_DOOR_DATABASE_URL/postgres:\/\//postgresql://}}" \
    pg_test --test migrations -- --test-threads=1
  pg_test --test migrations_postgres -- --ignored
  # Tests use unique per-test databases. Run suites in sequence so their admin
  # work cannot race; the sample uses PHYSICIAN_DATABASE_URL for both stores.
  pg_test --test postgres
  pg_test --test front_door_postgres
  # These identity targets must never silently return without exercising Postgres.
  pg_test --test identity
  pg_test --test luna_journal
  DUET_TEST_DATABASE_URL="$DUET_IDENTITY_TEST_DATABASE_URL" pg_test --test database_foundation -- --exact fresh_and_upgrade_keep_identity_and_enforce_account_scope
  # Exercises fixed GCS transport against loopback HTTP plus the real journal owner.
  pg_test --lib privacy_archive::tests::gcs_transport_and_postgres_recovery_reconcile_same_receipt_after_restart -- --exact
  # Explicitly select ignored real-database proofs; serial execution prevents
  # migration and recovery scans racing other suites. Crash/restart runs below
  # against its own disposable cluster, separate from these shared databases.
  pg_test --lib runtime::tests -- --ignored --test-threads=1
  pg_test --test runtime_postgres -- --ignored --skip database_crash_restart --test-threads=1
  local restart_binary
  restart_binary="$(pg_test --test runtime_postgres --no-run --message-format=json |
    python3 -c 'import json,sys; rows=(json.loads(line) for line in sys.stdin); binaries=[row["executable"] for row in rows if row.get("executable") and row["target"]["name"]=="runtime_postgres"]; [binary]=binaries; print(binary)')"
  python3 scripts/check-crash-restart.py "$restart_binary"
  pg_test --test runtime_publication -- --ignored --test-threads=1
  pg_test --test runtime_recovery -- --ignored --test-threads=1
  pg_test --test runtime_quota -- --ignored --test-threads=1
  pg_test --test chat_finish -- --ignored --test-threads=1
  # A display read dropped while its BEGIN waits on the server leaves the
  # next reads and writes on that pool outside any transaction.
  pg_test --test cancelled_read -- --ignored --test-threads=1
  pg_test --test concerns_postgres -- --ignored --test-threads=1
  pg_test --test practice_jobs_dispatch -- --test-threads=1
  pg_test --test source_refresh_postgres -- --test-threads=1
  pg_test --test practice_schedule_postgres -- --test-threads=1
  pg_test --test physician_run_recovery -- --test-threads=1
  pg_test --test patient_reviews_postgres -- --test-threads=1
  pg_test --test visit_briefings_postgres -- --test-threads=1
  pg_test --test today_updates_postgres -- --test-threads=1
  pg_test --test spruce_matching -- --test-threads=1
  pg_test --test reply_proposals -- --ignored --test-threads=1
  pg_test --test follow_ups_postgres -- --ignored --test-threads=1
  pg_test --test withdrawal_postgres -- --ignored --test-threads=1
  pg_test --test attachments -- --ignored --test-threads=1
  pg_test --test staff_practice_accounts -- --test-threads=1
  pg_test --test source_check_and_reconnect -- --test-threads=1
  # The worker's source boundary: a developer plan refuses non-synthetic sources.
  pg_test --bin physician-worker -- --ignored --test-threads=1
  # Physician scale proofs. Valkey comes from the nix dev shell; each test
  # creates its own database on PHYSICIAN_DATABASE_URL.
  # The bus suite needs Valkey only. The workspace CI job has no Nix shell, so
  # it skips; this job fail-closes when valkey-server is missing.
  pg_test --test redis -- --test-threads=1
  pg_test --test fanout -- --test-threads=1
  pg_test --test worker -- --test-threads=1
  # The worker hears of accepted work from Postgres and polls for what it did
  # not hear of; failures wait longer each time. A turn that ran out of
  # attempts shows "couldn't finish" and Try again (work.resume) gets the reply.
  pg_test --test try_again -- --test-threads=1
  # The connection budget: each process holds one pool of at most
  # MAX_CONNECTIONS, and a worker's failed due read says whether the pool ran
  # out or Postgres refused the query.
  pg_test --test pool_budget -- --test-threads=1
}

shard_2() {
  # Stop holds the turn across a worker restart; a second Stop holds the resumed
  # run; a message sent while held is answered without either stopped reply.
  pg_test --test stop -- --test-threads=1
  # Composer chat drafts: cross-site reads, stale-revision conflicts and physician
  # ownership, without a worker. The runtime draft-fence proof runs in shard 1.
  pg_test --test chat_draft -- --test-threads=1
  # Response feedback: ratings on Coda's responses by response id, details,
  # and author-only reads through two sites. The store proof runs in shard 1.
  pg_test --test feedback -- --test-threads=1
  # Hosted reply revisions: duplicate acceptance, frozen recipes, retries
  # re-pinning the newest sources, physician-save fencing, draft isolation,
  # synthetic opening and failed/held Try again over HTTP. Source currency
  # and revision history run at their storage owners in runtime::tests.
  pg_test --test reply_revision -- --test-threads=1
  # A turn accepted with no worker running is finished by a later one. A worker
  # told to stop hands its turn over at once; a killed worker's turn is taken
  # when its lease runs out. Exhausted leases fail the turn and announce it.
  pg_test --test instance_loss -- --test-threads=1
  pg_test --test physician_chat_recipe -- --ignored --test-threads=1
  pg_test --test physician_chat_mid_answer_arrivals -- --ignored --test-threads=1
  pg_test --test initial_understanding_postgres -- --test-threads=1
}

# Build only the targets this shard runs; the packages' other test binaries
# would add link time on a 2 vCPU runner for nothing.
case "$shard" in
  1) pg_test --no-run "${shard_1_targets[@]}"; shard_1 ;;
  2) pg_test --no-run "${shard_2_targets[@]}"; shard_2 ;;
  all) pg_test --no-run "${shard_1_targets[@]}" "${shard_2_targets[@]}"; shard_1; shard_2 ;;
esac
