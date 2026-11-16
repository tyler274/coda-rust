#!/usr/bin/env bash
# Captured inputs, existing app ingestion, disposable Postgres and retained reads.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
: "${PHYSICIAN_RUNTIME_TEST_DATABASE_URL:?Set a disposable Postgres URL}"
: "${SOURCE_REPLAY_ROOT:?Set the archived Hint and MIMIC selection directory}"
: "${HINT_CAPTURE_ROOT:?Set the paired Hint capture directory}"
mkdir -p target/verification
export ADAPTER_APP_INPUTS
ADAPTER_APP_INPUTS=$(mktemp -d "$PWD/target/verification/record-adapter-inputs.XXXXXX")
python3 crates/physician-sample/tests/support/prepare_adapter_app.py \
  --hint-captures "$HINT_CAPTURE_ROOT" --selection "$SOURCE_REPLAY_ROOT" --output "$ADAPTER_APP_INPUTS"
export HINT_LIVE_PROOF="$HINT_CAPTURE_ROOT/live-proof.json"
export PHYSICIAN_TEST_DATABASE_URL="$PHYSICIAN_RUNTIME_TEST_DATABASE_URL"
export CODA_REQUIRE_POSTGRES_TESTS=1
cargo test --locked -p physician-sample --test recorded_adapter_app --test adapter_delivery --test chart_load --test fhir_table_records -- --include-ignored
cargo test --locked -p local-product --test chat_finish --test second_source_adapter -- --include-ignored
cargo test --locked -p physician-sample --test site -- --ignored
