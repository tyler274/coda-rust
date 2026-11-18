#!/usr/bin/env bash
# Fast crate-scoped formatting, Clippy, and unit test runner for local iteration.
# Defaults to modified crates (working tree, or HEAD~1..HEAD when clean); pass
# --base <ref>, --all, or --pre-commit (staged index only, skipping cargo test).
set -euo pipefail

root=$(git rev-parse --show-toplevel)
cd "$root"

mode="local"
base_ref=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    --pre-commit) mode="pre-commit" ;;
    --all) mode="all" ;;
    --base)
      shift
      base_ref="${1:?--base requires a git ref}"
      mode="base"
      ;;
    -h|--help)
      sed -n '2,5p' "$0"
      exit 0
      ;;
    *)
      echo "dev-check.sh: unknown argument '$1'" >&2
      exit 2
      ;;
  esac
  shift
done

cargo fmt --all --check

if [ "$mode" = "all" ]; then
  cargo clippy --workspace --all-targets --quiet -- -D warnings
  cargo clippy -p physician-app -p duet-app -p mobile -p ui-gallery -p extension-popup \
    --target wasm32-unknown-unknown \
    --features physician-app/web,duet-app/web,mobile/web,ui-gallery/web,extension-popup/web \
    --quiet -- -D warnings
  cargo test --locked --workspace
  exit 0
fi

changed_files=""
case "$mode" in
  pre-commit)
    changed_files=$(git diff --cached --name-only --diff-filter=ACMR)
    ;;
  base)
    changed_files=$(git diff --name-only --diff-filter=ACMR "${base_ref}...HEAD" ; git status --porcelain=v1 | awk '{print $NF}')
    ;;
  local)
    changed_files=$(git status --porcelain=v1 | awk '{print $NF}')
    if [ -z "$changed_files" ]; then
      changed_files=$(git diff --name-only --diff-filter=ACMR HEAD~1..HEAD 2>/dev/null || true)
    fi
    ;;
esac

if [ -z "$changed_files" ]; then
  echo "dev-check: no changed files; formatting ok."
  exit 0
fi

needs_workspace=0
needs_wasm=0
pkg_list=""

crate_package_name() {
  local manifest="$1/Cargo.toml"
  [ -f "$manifest" ] || return 1
  awk -F'"' '/^name[[:space:]]*=/{print $2; exit}' "$manifest"
}

while IFS= read -r path; do
  [ -n "$path" ] || continue
  case "$path" in
    Cargo.toml|Cargo.lock|clippy.toml|rust-toolchain.toml|.cargo/*|flake.nix|flake.lock|crates/workspace-tools/sandbox/*)
      needs_workspace=1
      ;;
    crates/*)
      seg="${path#crates/}"
      crate_dir="crates/${seg%%/*}"
      if pkg=$(crate_package_name "$crate_dir"); then
        case " $pkg_list " in
          *" $pkg "*) ;;
          *) pkg_list="$pkg_list $pkg" ;;
        esac
        case "$pkg" in
          ui|physician-app|duet-app|mobile|ui-gallery|extension-popup|physician-client|duet-client|coda-http|physician-wire|duet_wire)
            needs_wasm=1
            ;;
        esac
      else
        needs_workspace=1
      fi
      ;;
  esac
done <<<"$changed_files"

pkg_count=$(wc -w <<<"$pkg_list")
if [ "$needs_workspace" = 1 ] || [ "$pkg_count" -gt 8 ]; then
  cargo clippy --workspace --all-targets --quiet -- -D warnings
  cargo clippy -p physician-app -p duet-app -p mobile -p ui-gallery -p extension-popup \
    --target wasm32-unknown-unknown \
    --features physician-app/web,duet-app/web,mobile/web,ui-gallery/web,extension-popup/web \
    --quiet -- -D warnings
  if [ "$mode" != "pre-commit" ]; then
    cargo test --locked --workspace
  fi
  exit 0
fi

if [ "$pkg_count" -eq 0 ]; then
  echo "dev-check: no Rust crates changed; formatting ok."
  exit 0
fi

pkg_args=()
for pkg in $pkg_list; do
  pkg_args+=(-p "$pkg")
done

echo "dev-check: checking crates:${pkg_list}"
cargo clippy "${pkg_args[@]}" --all-targets --quiet -- -D warnings
if [ "$needs_wasm" = 1 ]; then
  cargo clippy -p physician-app -p duet-app -p mobile -p ui-gallery -p extension-popup \
    --target wasm32-unknown-unknown \
    --features physician-app/web,duet-app/web,mobile/web,ui-gallery/web,extension-popup/web \
    --quiet -- -D warnings
fi
if [ "$mode" != "pre-commit" ]; then
  cargo test --locked "${pkg_args[@]}"
fi
