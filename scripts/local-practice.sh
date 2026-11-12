#!/usr/bin/env bash
# Starts the synthetic practice's Postgres, Valkey, site and worker.
# The practice database persists; the selected model transport uses the same
# durable job and chart owners as staging.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

cargo run -q -p patient-bank -- build
env -u PHYSICIAN_API_URL CARGO_TARGET_DIR=target/physician-wasm \
  cargo build -q -p physician-app --target wasm32-unknown-unknown --features web
wasm-bindgen --target web --out-dir target/physician-web \
  target/physician-wasm/wasm32-unknown-unknown/debug/physician-app.wasm

cargo build -q --release -p physician-sample --bins
exec python3 scripts/physician-scale-stack.py \
  --practice-dir "${CODA_LOCAL_PRACTICE_DIR:-target/local-practice}" \
  --model-execution "${CODA_LOCAL_MODEL_EXECUTION:-subscription}" \
  --bin-dir "${CARGO_TARGET_DIR:-target}/release" \
  --site-dir target/physician-web \
  --charts target/patient-bank/bank "$@"
