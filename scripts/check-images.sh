#!/usr/bin/env bash
# Build-only proof: no registry, deployment, token, or network runtime required.
set -euo pipefail
nix build --no-link --print-out-paths \
  .#physician-sample-image .#duet-api-image \
  .#physician-migration-image .#duet-migration-image > image-paths
mapfile -t images < image-paths
python scripts/check-images.py "${images[@]}"
# Credential-free native-link smoke. Migration executables must fail closed when
# their database URL is absent; this does not run a migration or hosted service.
for archive in "${images[@]}"; do
  docker load --input "$archive"
  tag=$(python -c 'import json,sys,tarfile; t=tarfile.open(sys.argv[1]); print(json.load(t.extractfile("manifest.json"))[0]["RepoTags"][0])' "$archive")
  case "$tag" in
    coda-physician-sample:*|coda-duet-api:*)
      binary=$(docker image inspect --format '{{index .Config.Labels "com.coda.binary"}}' "$tag")
      if output=$(docker run --rm --network none --read-only --cap-drop ALL --security-opt no-new-privileges -e K_SERVICE=qualification-smoke --entrypoint "$binary" "$tag" 2>&1); then
        printf '%s\n' 'unqualified runtime unexpectedly accepted hosted startup' >&2
        exit 1
      fi
      case "$output" in
        *hosted\ activation\ is\ not\ qualified*) printf '%s\n' "$tag: hosted activation fails closed" ;;
        *) printf '%s\n' "$output" >&2; exit 1 ;;
      esac
      ;;
    *migrate:*)
      binary=$(docker image inspect --format '{{index .Config.Labels "com.coda.binary"}}' "$tag")
      if output=$(docker run --rm --network none --read-only --cap-drop ALL --security-opt no-new-privileges --entrypoint "$binary" "$tag" 2>&1); then
        printf '%s\n' 'migration unexpectedly accepted missing database configuration' >&2
        exit 1
      fi
      case "$output" in
        *DATABASE_URL\ is\ required*) printf '%s\n' "$tag: native entrypoint fails closed without database configuration" ;;
        *) printf '%s\n' "$output" >&2; exit 1 ;;
      esac
      ;;
  esac
done
