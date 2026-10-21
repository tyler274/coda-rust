#!/usr/bin/env bash
# From the repository/worktree root:
# nix develop --command bash crates/public-site/island/build.sh
# Outputs: island/dist/{bootstrap.js,public_site_motion.js,public_site_motion_bg.wasm}
# Embed all three files, preserve sibling URLs (one shared hash directory), and
# include <script type="module" src=".../bootstrap.js"></script> after CSS.
set -euo pipefail
# CI provides the same pinned toolchain explicitly; local runs require Nix.
if [[ -z "${IN_NIX_SHELL:-}" && "${CI:-}" != "true" ]]; then
  printf '%s\n' 'Run this script through nix develop --command bash.' >&2
  exit 1
fi
island="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cargo build --locked --manifest-path "$island/Cargo.toml" --release --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript --out-dir "$island/dist" \
  "$island/target/wasm32-unknown-unknown/release/public_site_motion.wasm"
# Build artifacts only: no source files are generated or changed by this script.
cp "$island/bootstrap.js" "$island/dist/bootstrap.js"
node "$island/check-size.mjs" "$island/dist"
