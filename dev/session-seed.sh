#!/usr/bin/env bash
# What a parallel session's seed builds, run in the dev shell. Clones reuse only
# what these same commands built: Cargo resolves features per command, so a
# workspace build alone misses -p builds of the same crates. Only third-party
# crates stay in the seed.
set -euo pipefail

cargo fetch --locked --target wasm32-unknown-unknown
cargo build --locked --workspace --all-targets
cargo build --locked -p physician-sample
cargo build --locked -p patient-bank
cargo test --locked --no-run -p physician-sample
env -u PHYSICIAN_API_URL CARGO_TARGET_DIR=target/physician-wasm \
  cargo build --locked -p physician-app --target wasm32-unknown-unknown --features web
# Browser tests clear their environment before this build, which Cargo notices.
env -i HOME="$HOME" USER="${USER:-vscode}" PATH="$PATH" CARGO_TARGET_DIR=target/coda-physician-wasm \
  cargo build --locked -p physician-app --target wasm32-unknown-unknown --features web
# The pre-commit hook's Clippy runs, whose third-party output no build shares.
cargo clippy --locked --workspace --all-targets --quiet
cargo clippy --locked -p physician-app -p duet-app -p mobile -p ui-gallery --target wasm32-unknown-unknown \
  --features physician-app/web,duet-app/web,mobile/web,ui-gallery/web --quiet

# Workspace crates have this checkout's paths compiled in, and Cargo would trust
# them in a worktree whose files are older than the seed, even when the code differs.
packages=$(cargo metadata --locked --no-deps --format-version 1 | jq -r '.packages[].id | "-p " + .')
for dir in target target/physician-wasm target/coda-physician-wasm; do
  CARGO_TARGET_DIR=$dir cargo clean --quiet $packages
  CARGO_TARGET_DIR=$dir cargo clean --quiet --target wasm32-unknown-unknown $packages
done
