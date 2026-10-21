#!/bin/sh
# Regenerate crates/duet-wire with OpenAPI Generator's stable Rust client.
# Run inside `nix develop` so the CLI is the one pinned by flake.nix.
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cd "$root"
# A worktree can use the pinned reference file in another checkout, without
# copying or modifying a submodule. The default remains the local submodule.
reference=${DUET_REFERENCE_OPENAPI:-submodules/coda-app/docs/openapi.json}
current=$(mktemp -d)
trap 'rm -rf "$current"' EXIT
spec=$current/openapi.json
python3 scripts/compose-duet-openapi.py "$reference" crates/duet-api/openapi/additions.json "$spec"
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
  # Normalize once before the compatibility pass so its markers are stable.
  cargo fmt --manifest-path "$out/Cargo.toml"
  python3 scripts/finalize-duet-wire.py "$spec" "$out"
  cargo fmt --manifest-path "$out/Cargo.toml"
}

if [ "${1:-}" = "--check" ]; then
  cp -a crates/duet-wire/. "$current/committed"
  generate "$current/fresh"
  diff -ru "$current/committed/src" "$current/fresh/src"
  diff -u "$current/committed/Cargo.toml" "$current/fresh/Cargo.toml"
  exit 0
fi

generate crates/duet-wire
