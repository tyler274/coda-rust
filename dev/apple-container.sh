#!/usr/bin/env bash
# Bring up the dev shell in Apple's `container` on Apple silicon. This is the
# dev container's own image and postCreate.sh without Compose or VS Code, which
# the tool has no Docker API for. Idempotent: an existing container is reused.
#
#   dev/apple-container.sh                 bring the container up
#   dev/apple-container.sh exec <command>  run a command in the dev shell of the
#                                          current checkout or worktree
set -euo pipefail

script_dir=$(cd "$(dirname "$0")" && pwd -P)
# The main checkout, also when this runs from one of its worktrees. One
# container serves them all, because worktrees live inside the checkout it mounts.
root=$(dirname "$(git -C "$script_dir" rev-parse --path-format=absolute --git-common-dir)")
name=${CODA_CONTAINER_NAME:-coda-dev-arm}
# The dev container's pinned base image.
image=$(sed -n 's/^FROM //p' "$root/.devcontainer/Dockerfile")
cpus=${CODA_CONTAINER_CPUS:-$(sysctl -n hw.ncpu)}
# Three quarters of the host: a container defaults to 1 GB, too little to link.
memory=${CODA_CONTAINER_MEMORY:-$(($(sysctl -n hw.memsize) * 3 / 4 / 1024 / 1024 / 1024))g}

check_mount() {
  local mounted_root
  mounted_root=$(jq -er '.[0].configuration.mounts[] | select(.destination == "/workspace") | .source' <<<"$1")
  if [[ ! "$mounted_root" -ef "$root" ]]; then
    echo "$name mounts $mounted_root at /workspace; choose a different CODA_CONTAINER_NAME for $root" >&2
    exit 1
  fi
}

# A worktree's .git file and config name the host's absolute paths, so the
# checkout must also answer at its host path for Git and Nix to open a worktree.
# After a restart the bind mount's root reads as root's, and Git and Nix then
# refuse it; worktrees inside it need the same exemption.
prepare_git() {
  container exec --user vscode "$name" sh -ec '
    sudo mkdir -p "$(dirname "$1")"
    sudo ln -sfn /workspace "$1"
    git config --global --replace-all safe.directory "*"
  ' sh "$root"
}

bring_up() {
  container system start --enable-kernel-install

  local inspection
  if inspection=$(container inspect "$name" 2>/dev/null); then
    check_mount "$inspection"
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

  prepare_git
  container exec --user vscode --workdir /workspace "$name" bash .devcontainer/postCreate.sh
}

run_in_shell() {
  local here checkout inspection interactive=-i
  here=$(pwd -P)
  checkout=$(git rev-parse --show-toplevel)
  case "$here/" in
    "$root"/*) ;;
    *)
      echo "$here is outside $root, the checkout $name mounts" >&2
      exit 1
      ;;
  esac

  if inspection=$(container inspect "$name" 2>/dev/null) &&
    [[ $(jq -r '.[0].status.state' <<<"$inspection") == running ]]; then
    check_mount "$inspection"
    prepare_git
  else
    bring_up >&2
  fi

  [[ -t 0 && -t 1 ]] && interactive=-it
  # Build output stays in the target volume: the bind mount is far slower, and
  # every worktree would otherwise start its build from nothing.
  exec container exec "$interactive" --user vscode \
    --workdir "/workspace${here#"$root"}" \
    --env CARGO_TARGET_DIR=/workspace/target \
    "$name" nix develop "/workspace${checkout#"$root"}#agent" --command "$@"
}

case "${1:-}" in
  "") bring_up ;;
  exec)
    shift
    if [[ $# -eq 0 ]]; then
      echo "usage: $0 exec <command> [args...]" >&2
      exit 2
    fi
    run_in_shell "$@"
    ;;
  *)
    echo "usage: $0 [exec <command> [args...]]" >&2
    exit 2
    ;;
esac
