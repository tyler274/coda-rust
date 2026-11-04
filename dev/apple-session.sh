#!/usr/bin/env bash
# One Apple container per Git worktree, so several copies of the app run side by
# side. Each session's Nix store, Cargo home and target are APFS clones of a
# stopped seed, so they share disk until they diverge. A session gets its own
# filesystem and network; the worktree and the shared .git stay writable.
# See "Apple silicon Mac without Nix" in README.md.
set -euo pipefail

prefix=${CODA_SESSION_PREFIX:-coda}
seed=$prefix-seed
port_base=${CODA_SESSION_PORT_BASE:-20000}
volumes="nix cargo target"
image=$(sed -n 's/^FROM //p' "$(dirname "$0")/../.devcontainer/Dockerfile")
cpus=${CODA_SESSION_CPUS:-$(sysctl -n hw.ncpu)}
memory=${CODA_SESSION_MEMORY:-12g}
# Linking test binaries in parallel is what exhausts a capped VM.
jobs=${CODA_SESSION_JOBS:-$((${memory%g} / 3))}
max_slots=20

checkout=$(git rev-parse --show-toplevel)
common=$(git rev-parse --path-format=absolute --git-common-dir)
slots=$common/coda-sessions
seed_ready=$slots/seed-ready

# The session `up` is building, which its exit trap removes if setup fails.
created=

die() {
  echo "$*" >&2
  exit 1
}

# Git and Nix resolve a worktree through paths recorded on the Mac, so the
# checkout and the shared .git mount at those same paths.
start_container() {
  local name=$1 root=$2
  shift 2
  local mounts=(-v "$root:$root" -v "$name-nix:/nix" -v "$name-cargo:/home/vscode/.cargo" -v "$name-target:$root/target")
  # Evidence outlives the session and stays where the Mac can publish it.
  mkdir -p "$root/target/verification"
  mounts+=(-v "$root/target/verification:$root/target/verification")
  case "$common/" in
    "$root"/*) ;;
    *) mounts+=(-v "$common:$common") ;;
  esac
  # A worktree can name hooks outside itself, and Git skips ones it can't see.
  local hooks
  hooks=$(git -C "$root" config --get core.hooksPath || true)
  case "$hooks" in
    "$root"/* | "$common"/*) ;;
    /*) mounts+=(-v "$hooks:$hooks") ;;
  esac
  # --init reaps the orphans that Chromium and servers leave behind.
  container run -d --init --name "$name" --arch arm64 --cpus "$cpus" --memory "$memory" \
    "${mounts[@]}" "$@" "$image" sleep infinity >/dev/null
}

# The root filesystem is the container's own, so this runs for every new one.
prepare() {
  local name=$1 root=$2
  container exec -i "$name" sh -ec 'mkdir -p /etc/nix; cat > /etc/nix/nix.conf' <"$root/.devcontainer/nix.conf"
  # After a restart the mounts read as root's, and Git and Nix then refuse them.
  container exec --user vscode "$name" git config --global --replace-all safe.directory '*'
  container exec --user vscode --workdir "$root" \
    --env CODA_WORKSPACE="$root" --env CODA_SKIP_SUBMODULES=1 \
    "$name" bash .devcontainer/postCreate.sh >/dev/null
  local key value
  for key in user.name user.email; do
    value=$(git -C "$root" config "$key") || continue
    container exec --user vscode "$name" git config --global "$key" "$value"
  done
}

seed_running() {
  container inspect "$seed" 2>/dev/null | jq -e '.[0].status.state == "running"' >/dev/null
}

remove_container() {
  container stop "$1" >/dev/null 2>&1 || true
  container rm "$1" >/dev/null 2>&1 || true
}

remove_session() {
  local slot=${1:?} name=$prefix-session-${1:?} volume left item
  remove_container "$name"
  for volume in $volumes; do
    container volume rm "$name-$volume" >/dev/null 2>&1 || true
  done
  # Removal fails the same way for what's already gone, so ask what's left. The
  # slot stays until nothing is, or a retry couldn't find the session.
  left=$(container ls -a --quiet)
  left+=$'\n'$(container volume ls --quiet)
  for item in $left; do
    case "$item" in
      "$name" | "$name"-nix | "$name"-cargo | "$name"-target)
        die "could not fully remove $name; rerun when the container service is healthy"
        ;;
    esac
  done
  rm -rf "${slots:?}/$slot"
}

volume_image() {
  container volume inspect "$1" | jq -er '.[0].configuration.source'
}

# Volumes are files, and the tool has no clone command. Only a stopped seed
# gives a consistent copy.
clone_volumes() {
  local name=$1 volume image_path
  for volume in $volumes; do
    container volume create "$name-$volume" >/dev/null
    image_path=$(volume_image "$name-$volume")
    rm "$image_path"
    cp -c "$(volume_image "$seed-$volume")" "$image_path"
  done
}

slot_of_checkout() {
  local path
  for path in "$slots"/*/checkout; do
    [ -f "$path" ] || continue
    if [ "$(cat "$path")" = "$checkout" ]; then
      basename "$(dirname "$path")"
      return 0
    fi
  done
  return 1
}

# mkdir is the atomic step, so sessions starting together take different slots.
claim_slot() {
  local slot
  mkdir -p "$slots"
  for ((slot = 1; slot <= max_slots; slot++)); do
    if mkdir "$slots/$slot" 2>/dev/null; then
      echo "$checkout" >"$slots/$slot/checkout"
      echo "$slot"
      return 0
    fi
  done
  die "all $max_slots session slots are taken; see: $0 status"
}

# Inside, every session keeps the usual ports. Only the Mac side differs.
base_port() { echo $((port_base + $1 * 10)); }

require_session() {
  local slot
  slot=$(slot_of_checkout) || die "no session for $checkout; run: $0 up"
  echo "$slot"
}

print_ports() {
  local base
  base=$(base_port "$1")
  echo "  physician (8790): http://127.0.0.1:$base"
  echo "  duet-api or gallery (8080): http://127.0.0.1:$((base + 1))"
  echo "  local product (8787): http://127.0.0.1:$((base + 2))"
}

seed() {
  local volume
  container system start --enable-kernel-install
  seed_running && die "the seed is already building"
  rm -f "$seed_ready"
  remove_container "$seed"
  trap 'remove_container "$seed"' EXIT
  # Old build output only makes every clone bigger. Existing clones keep theirs.
  container volume rm "$seed-target" >/dev/null 2>&1 || true
  for volume in $volumes; do
    container volume inspect "$seed-$volume" >/dev/null 2>&1 ||
      container volume create "$seed-$volume" >/dev/null
  done
  # Only the seed trims, so only the seed gets the capability fstrim needs.
  start_container "$seed" "$checkout" --cap-add CAP_SYS_ADMIN
  # New volumes are root's.
  container exec "$seed" chown -R vscode:vscode /nix /home/vscode/.cargo "$checkout/target"
  prepare "$seed" "$checkout"
  # Nix's profile links live in the home directory, which clones don't carry.
  container exec --user vscode "$seed" sh -ec '
    profile=$(readlink -f "$HOME/.nix-profile" || true)
    case "$profile" in
      /nix/store/*)
        mkdir -p /nix/var/nix/profiles
        ln -sfn "$profile" /nix/var/nix/profiles/default
        ;;
    esac
  '
  container exec --user vscode --workdir "$checkout" --env CARGO_BUILD_JOBS="$jobs" \
    "$seed" nix develop .#agent --command bash dev/session-seed.sh
  # Disk images keep the space of deleted files until they're trimmed.
  for mount in /nix /home/vscode/.cargo "$checkout/target"; do
    container exec "$seed" fstrim "$mount"
  done
  mkdir -p "$slots"
  touch "$seed_ready"
  echo "seed ready from $checkout at $(git rev-parse --short HEAD)"
}

up() {
  local slot name base
  container system start --enable-kernel-install
  if slot=$(slot_of_checkout); then
    name=$prefix-session-$slot
    container start "$name" >/dev/null 2>&1 || true
  else
    seed_running && die "the seed is still building; rerun when it finishes"
    [ -f "$seed_ready" ] || die "no finished seed; run: $0 seed"
    slot=$(claim_slot)
    name=$prefix-session-$slot
    created=$slot
    trap '[ -z "$created" ] || remove_session "$created"' EXIT
    clone_volumes "$name"
    base=$(base_port "$slot")
    start_container "$name" "$checkout" \
      -p "127.0.0.1:$base:8790" \
      -p "127.0.0.1:$((base + 1)):8080" \
      -p "127.0.0.1:$((base + 2)):8787"
  fi
  prepare "$name" "$checkout"
  created=
  echo "session $slot ready for $checkout"
  print_ports "$slot"
}

run_in_session() {
  local slot input=-i
  slot=$(require_session)
  [ -t 0 ] && input=-it
  # Published ports reach only servers listening on all interfaces.
  container exec "$input" --user vscode --workdir "$checkout" \
    --env PHYSICIAN_LISTEN_ADDR=0.0.0.0:8790 \
    --env DUET_LISTEN_ADDR=0.0.0.0:8080 \
    --env UI_GALLERY_LISTEN_ADDR=0.0.0.0:8080 \
    --env CARGO_BUILD_JOBS="$jobs" \
    "$prefix-session-$slot" nix develop .#agent --command "$@"
}

# The session's own Postgres, in its root filesystem. Prints the database URL.
postgres_in_session() {
  run_in_session bash -ec '
    data=$HOME/postgres
    if [ ! -d "$data" ]; then
      initdb -D "$data" -U coda --auth=trust >/dev/null
    fi
    if ! pg_ctl -D "$data" status >/dev/null 2>&1; then
      pg_ctl -D "$data" -o "-p 5432 -k /tmp" -l "$data/server.log" -w start >/dev/null
    fi
    psql -h /tmp -U coda -d postgres -tAc "SELECT 1 FROM pg_database WHERE datname = '"'"'physician'"'"'" | grep -q 1 ||
      createdb -h /tmp -U coda physician
    echo postgres://coda@127.0.0.1:5432/physician
  ' | tail -1
}

status() {
  local path slot state
  for path in "$slots"/*/checkout; do
    [ -f "$path" ] || continue
    slot=$(basename "$(dirname "$path")")
    state=$(container inspect "$prefix-session-$slot" 2>/dev/null | jq -r '.[0].status.state // "missing"' || echo missing)
    echo "session $slot, $state: $(cat "$path")"
    print_ports "$slot"
  done
}

# Removes the sessions of worktrees that no longer exist.
prune() {
  local path slot
  for path in "$slots"/*/checkout; do
    [ -f "$path" ] || continue
    if [ ! -d "$(cat "$path")" ]; then
      slot=$(basename "$(dirname "$path")")
      remove_session "$slot"
      echo "removed session $slot"
    fi
  done
}

case "${1:-}" in
  seed) seed ;;
  up) up ;;
  exec)
    shift
    [ "${1:-}" = -- ] && shift
    [ $# -gt 0 ] || die "usage: $0 exec [--] command..."
    run_in_session "$@"
    ;;
  postgres) postgres_in_session ;;
  status) status ;;
  down)
    slot=$(require_session)
    remove_session "$slot"
    ;;
  prune) prune ;;
  *) die "usage: $0 seed | up | exec [--] command... | postgres | status | down | prune" ;;
esac
