#!/bin/sh
# Regenerate crates/duet-wire with OpenAPI Generator's stable Rust client.
# Run inside `nix develop` so the CLI is the one pinned by flake.nix.
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cd "$root"
spec=submodules/coda-app/docs/openapi.json
props=packageName=duet_wire,packageVersion=0.1.0,reqwestDefaultFeatures=rustls,supportAsync=true,hideGenerationTimestamp=true

generate() {
  out=$1
  openapi-generator-cli generate -i "$spec" -g rust -o "$out" --additional-properties="$props"
  sed -i 's/^license = "Unlicense"/license = "UNLICENSED"/' "$out/Cargo.toml"
  sed -i 's/^edition = "2021"/edition = "2024"/' "$out/Cargo.toml"
  # The Rust generator names OpenAPI `type: null` as models::Null and does not emit it.
  cat >> "$out/src/models/mod.rs" <<'EOF'

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Null;
EOF
  rm -rf "$out/.openapi-generator" "$out/git_push.sh" "$out/README.md"
}

if [ "${1:-}" = "--check" ]; then
  current=$(mktemp -d)
  trap 'rm -rf "$current"' EXIT
  cp -a crates/duet-wire/. "$current/committed"
  generate "$current/fresh"
  diff -ru "$current/committed/src" "$current/fresh/src"
  diff -u "$current/committed/Cargo.toml" "$current/fresh/Cargo.toml"
  exit 0
fi

generate crates/duet-wire
