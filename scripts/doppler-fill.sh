#!/usr/bin/env bash
# Fill the Doppler config the dev shell loads (project coda-rust, config dev).
#
#   scripts/doppler-fill.sh            # ask for each secret that is not set yet
#   scripts/doppler-fill.sh --replace  # ask for every secret, set or not
#   scripts/doppler-fill.sh --check    # list which are set; ask for nothing
#
# Run it from the nix dev shell after `doppler login`. Each value is typed or
# pasted at a hidden prompt and piped to Doppler, so it reaches no file, no
# argument list and no shell history. An empty answer skips that secret.
# Nothing here is needed by tests or the sample server.
set -euo pipefail

project=coda-rust
config=dev

# The secrets `dev` holds, each with what reads it.
secrets=(
  "OPENAI_API_KEY|model calls and dictation when CODA_LOCAL_MODEL_EXECUTION=api"
  "DD_API_KEY|scripts/observability-up.sh --datadog"
  "SPRUCE_API_KEY|the ignored live Spruce test, cargo test -p spruce --test live"
  "NCBI_API_KEY|optional: a higher PubMed rate limit for clinical-library-pubmed-status"
)

mode=missing
case "${1:-}" in
  "") ;;
  --replace) mode=replace ;;
  --check) mode=check ;;
  *)
    echo "usage: scripts/doppler-fill.sh [--replace | --check]" >&2
    exit 2
    ;;
esac

if [ "$mode" != check ] && [ ! -t 0 ]; then
  echo "Run this in a terminal: it asks for each value at a hidden prompt." >&2
  exit 2
fi

existing="$(doppler secrets --only-names --json --project "$project" --config "$config" | jq -r 'keys[]')"

for entry in "${secrets[@]}"; do
  name="${entry%%|*}"
  reader="${entry#*|}"
  if grep -qx "$name" <<<"$existing"; then
    state=set
  else
    state=missing
  fi

  if [ "$mode" = check ]; then
    printf '%-8s %s  (%s)\n' "$state" "$name" "$reader"
    continue
  fi
  if [ "$state" = set ] && [ "$mode" = missing ]; then
    printf 'kept     %s\n' "$name"
    continue
  fi

  read -rs -p "$name ($reader): " value
  echo
  if [ -z "$value" ]; then
    printf 'skipped  %s\n' "$name"
    continue
  fi
  printf %s "$value" |
    doppler secrets set "$name" --project "$project" --config "$config" --silent --no-interactive >/dev/null
  unset value
  printf 'saved    %s\n' "$name"
done
