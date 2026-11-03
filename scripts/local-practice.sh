#!/usr/bin/env bash
# Runs the synthetic practice locally on Postgres: the migration sets up the
# schema and accounts, the chart load fills the practice and the site reads the
# database. The database keeps its practice between runs, and a repeat load
# changes nothing. A patient who can't load is printed; the site still starts.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
: "${PHYSICIAN_DATABASE_URL:?Set a local Postgres database URL}"

cargo run -q -p patient-bank -- build
env -u PHYSICIAN_API_URL CARGO_TARGET_DIR=target/physician-wasm \
  cargo build -q -p physician-app --target wasm32-unknown-unknown --features web
wasm-bindgen --target web --out-dir target/physician-web \
  target/physician-wasm/wasm32-unknown-unknown/debug/physician-app.wasm

CODA_HOSTED_PROFILE=staging-preview \
  PHYSICIAN_STAGING_ACCOUNTS="${PHYSICIAN_STAGING_ACCOUNTS:-physician@codahealth.test|Staging Physician|admin}" \
  cargo run -q -p physician-sample --bin physician-migrate
CODA_HOSTED_PROFILE=staging-preview \
  PHYSICIAN_SYNTHETIC_CHARTS="$PWD/target/patient-bank/bank" \
  cargo run -q -p physician-sample --bin physician-migrate -- load-charts || true

PHYSICIAN_AUTH=required exec cargo run -q -p physician-sample --bin physician-sample
