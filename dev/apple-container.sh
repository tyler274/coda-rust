#!/usr/bin/env bash
# Bring up the dev shell in Apple's `container` on Apple silicon. This is the
# dev container's own image and postCreate.sh without Compose or VS Code, which
# the tool has no Docker API for. Idempotent: an existing container is reused.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
name=${CODA_CONTAINER_NAME:-coda-dev-arm}
# The dev container's pinned base image.
image=$(sed -n 's/^FROM //p' "$root/.devcontainer/Dockerfile")
cpus=${CODA_CONTAINER_CPUS:-$(sysctl -n hw.ncpu)}
# Three quarters of the host: a container defaults to 1 GB, too little to link.
memory=${CODA_CONTAINER_MEMORY:-$(($(sysctl -n hw.memsize) * 3 / 4 / 1024 / 1024 / 1024))g}

container system start --enable-kernel-install

if inspection=$(container inspect "$name" 2>/dev/null); then
  mounted_root=$(jq -er '.[0].configuration.mounts[] | select(.destination == "/workspace") | .source' <<<"$inspection")
  if [[ ! "$mounted_root" -ef "$root" ]]; then
    echo "$name mounts $mounted_root at /workspace; choose a different CODA_CONTAINER_NAME for $root" >&2
    exit 1
  fi
  container start "$name" >/dev/null 2>&1 || true
else
  for volume in nix cargo target; do
    container volume inspect "$name-$volume" >/dev/null 2>&1 ||
      container volume create "$name-$volume" >/dev/null
  done
  # No host networking: servers inside listen on 0.0.0.0 to use these ports.
  container run -d --name "$name" --arch arm64 --cpus "$cpus" --memory "$memory" \
    -v "$root:/workspace" \
    -v "$name-nix:/nix" \
    -v "$name-cargo:/home/vscode/.cargo" \
    -v "$name-target:/workspace/target" \
    -p 127.0.0.1:8790:8790 \
    -p 127.0.0.1:8080:8080 \
    "$image" sleep infinity >/dev/null
fi

# New volumes are root's, and this route skips the Dockerfile's nix.conf. That
# file goes last, so an interrupted run repeats this step.
container exec "$name" sh -ec '
  [ -f /etc/nix/nix.conf ] && exit 0
  chown -R vscode:vscode /nix /home/vscode/.cargo /workspace/target
  mkdir -p /etc/nix
  cp /workspace/.devcontainer/nix.conf /etc/nix/nix.conf
'

# After a restart the bind mount's root reads as root's, and Git and Nix then refuse it.
container exec --user vscode "$name" git config --global --replace-all safe.directory /workspace

container exec --user vscode --workdir /workspace "$name" bash .devcontainer/postCreate.sh
