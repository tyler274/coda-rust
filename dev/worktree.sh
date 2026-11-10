#!/usr/bin/env bash
# Worktrees whose build output costs nothing until it diverges. Each worktree
# under .worktrees/<slug> keeps only its source; its target/ links to its own
# directory under the build root, made as a reflink clone of a seed that holds
# the third-party build output. See "Worktrees and build output" in README.md.
#
#   dev/worktree.sh new <slug> [--from <ref>]   worktree plus a seeded target
#   dev/worktree.sh seed [<ref>]                build a seed from <ref> (origin/main)
#   dev/worktree.sh reset <slug>                replace a target with a fresh clone
#   dev/worktree.sh gc [--idle <days>]          reset idle targets, drop orphans
#   dev/worktree.sh rm <slug>                   remove a worktree and its target
#   dev/worktree.sh prune [--yes]               rm every clean worktree merged into origin/main
#   dev/worktree.sh status                      worktrees, targets and disk use
set -euo pipefail

common=$(git rev-parse --path-format=absolute --git-common-dir)
main=$(dirname "$common")
trees=$main/.worktrees
build=${CODA_BUILD_ROOT:-/agents/coda/build}
seeds=$build/seed
targets=$build/wt
marker=.coda-seed
keep_seeds=2

die() {
  echo "worktree.sh: $*" >&2
  exit 1
}

usage() {
  sed -n '7,13s/^# \{0,3\}//p' "$0" >&2
  exit 2
}

check_slug() {
  [[ $1 =~ ^[a-z0-9][a-z0-9._-]*$ ]] || die "slug '$1' must be lowercase letters, digits, '.', '_' or '-'"
}

need_build_root() {
  [ -d "$build" ] || die "build root $build does not exist; set CODA_BUILD_ROOT to a directory on a filesystem with reflinks (ZFS, btrfs, XFS)"
  [ -w "$build" ] || die "build root $build is not writable by $(id -un); on Cyrene it belongs to the coda-agent sandbox"
  mkdir -p "$seeds" "$targets"
}

# The inputs that decide whether third-party output can be reused.
seed_key() {
  local tree=$1
  cat "$tree/Cargo.lock" "$tree/flake.lock" "$tree/rust-toolchain.toml" | sha256sum | cut -c1-16
}

current_seed() {
  [ -L "$seeds/current" ] && readlink -f "$seeds/current"
}

# Clone the current seed into a target directory. Never falls back to a copy:
# a full copy per worktree is the disk use this script exists to avoid.
# target/verification holds retained proof, not build output, so it survives.
clone_seed() {
  local dest=$1 seed kept='' cloned=true
  if [ -d "$dest/verification" ]; then
    kept=$(mktemp -d "$targets/.verification.XXXXXX")
    mv "$dest/verification" "$kept/"
  fi
  rm -rf "$dest"
  mkdir -p "$dest"
  seed=$(current_seed) || true
  if [ -z "$seed" ]; then
    echo "No seed yet; $dest starts empty. Run dev/worktree.sh seed to share third-party builds." >&2
  elif cp -a --reflink=always "$seed/." "$dest/"; then
    basename "$seed" >"$dest/$marker"
  else
    cloned=false
    rm -rf "$dest"
    mkdir -p "$dest"
  fi
  if [ -n "$kept" ]; then
    rm -rf "$dest/verification"
    mv "$kept/verification" "$dest/"
    rmdir "$kept"
  fi
  $cloned || die "reflink clone of $seed into $dest failed; the build root must support reflinks"
}

link_target() {
  local tree=$1 dest=$2
  if [ -e "$tree/target" ] && [ ! -L "$tree/target" ]; then
    die "$tree/target is a real directory; move it away before linking the shared build root"
  fi
  ln -sfn "$dest" "$tree/target"
}

cmd_new() {
  local slug=${1:-} from=origin/main
  [ -n "$slug" ] || usage
  shift
  while [ $# -gt 0 ]; do
    case $1 in
      --from) from=${2:?--from needs a ref}; shift 2 ;;
      *) usage ;;
    esac
  done
  check_slug "$slug"
  need_build_root
  local tree=$trees/$slug dest=$targets/$slug
  [ -e "$tree" ] && die "$tree already exists"
  mkdir -p "$trees"
  if git -C "$main" show-ref --verify --quiet "refs/heads/$slug"; then
    git -C "$main" worktree add "$tree" "$slug"
  else
    # No upstream: the first push names the branch's own remote ref.
    git -C "$main" worktree add --no-track -b "$slug" "$tree" "$from"
  fi
  clone_seed "$dest"
  link_target "$tree" "$dest"
  if [ -f "$tree/.envrc" ] && command -v direnv >/dev/null; then
    direnv allow "$tree"
  fi
  echo "$tree"
}

cmd_seed() {
  local ref=${1:-origin/main}
  need_build_root
  local tree=$trees/.seed
  if [ -e "$tree" ]; then
    git -C "$tree" checkout -q --detach "$ref"
  else
    mkdir -p "$trees"
    git -C "$main" worktree add -q --detach "$tree" "$ref"
  fi
  local key
  key=$(seed_key "$tree")
  local dest=$seeds/$key
  if [ -d "$dest" ] && [ -e "$dest/$marker" ]; then
    echo "Seed $key is already built."
  else
    rm -rf "$dest"
    mkdir -p "$dest"
    link_target "$tree" "$dest"
    (cd "$tree" && nix develop .#agent --command dev/session-seed.sh)
    echo "$key" >"$dest/$marker"
  fi
  # Newest first when pruning below, even when it was built long ago.
  touch "$dest"
  ln -sfn "$key" "$seeds/current.new"
  mv -T "$seeds/current.new" "$seeds/current"
  # Older seeds go; clones keep their blocks until they are reset.
  find "$seeds" -mindepth 1 -maxdepth 1 -type d -printf '%T@ %p\n' | sort -rn |
    tail -n +$((keep_seeds + 1)) | cut -d' ' -f2- | xargs -r rm -rf
  echo "Seed $key is current."
}

cmd_reset() {
  local slug=${1:-}
  [ -n "$slug" ] || usage
  check_slug "$slug"
  need_build_root
  [ -d "$trees/$slug" ] || die "no worktree $trees/$slug"
  clone_seed "$targets/$slug"
  link_target "$trees/$slug" "$targets/$slug"
}

# True when nothing in the target changed in the last <days> days. Cargo adds
# entries to deps/, .fingerprint/ and incremental/ on every build, so the
# directories near the top are enough to tell.
idle() {
  local dir=$1 days=$2
  [ -z "$(find "$dir" -mindepth 1 -maxdepth 3 -newermt "$days days ago" -print -quit)" ]
}

# True when the target is an untouched clone of the current seed.
pristine() {
  local dir=$1 seed
  seed=$(current_seed) || return 1
  [ -f "$dir/$marker" ] && [ "$(cat "$dir/$marker")" = "$(basename "$seed")" ] &&
    [ -z "$(find "$dir" -mindepth 1 -maxdepth 3 -newer "$dir/$marker" -print -quit)" ]
}

cmd_gc() {
  local days=7
  case ${1:-} in
    --idle) days=${2:?--idle needs a number of days} ;;
    "") ;;
    *) usage ;;
  esac
  need_build_root
  local dir slug
  for dir in "$targets"/*/; do
    [ -d "$dir" ] || continue
    dir=${dir%/}
    slug=$(basename "$dir")
    if [ ! -d "$trees/$slug" ]; then
      echo "Removing $dir: its worktree is gone."
      rm -rf "$dir"
    elif idle "$dir" "$days" && ! pristine "$dir"; then
      echo "Resetting $slug: no build for $days days."
      clone_seed "$dir"
    fi
  done
}

cmd_rm() {
  local slug=${1:-}
  [ -n "$slug" ] || usage
  check_slug "$slug"
  # Refuses a worktree with changes; commit, stash or remove them first.
  git -C "$main" worktree remove "$trees/$slug"
  rm -rf "${targets:?}/$slug"
  echo "Removed $slug; branch $slug is kept."
}

cmd_prune() {
  local yes=${1:-}
  git -C "$main" fetch -q origin main
  local tree slug merged=()
  for tree in "$trees"/*/; do
    [ -d "$tree" ] || continue
    tree=${tree%/}
    slug=$(basename "$tree")
    git -C "$tree" merge-base --is-ancestor HEAD origin/main 2>/dev/null || continue
    [ -z "$(git -C "$tree" status --porcelain)" ] || continue
    merged+=("$slug")
  done
  if [ ${#merged[@]} -eq 0 ]; then
    echo "No clean worktree is merged into origin/main."
    return
  fi
  printf 'Merged and clean: %s\n' "${merged[@]}"
  if [ "$yes" != --yes ]; then
    [ -t 0 ] || die "rerun with --yes to remove them"
    read -r -p "Remove these worktrees and their targets? [y/N] " answer
    [ "$answer" = y ] || return
  fi
  for slug in "${merged[@]}"; do
    cmd_rm "$slug"
  done
}

cmd_status() {
  local seed
  seed=$(current_seed) || true
  echo "Seed: $(if [ -n "$seed" ]; then basename "$seed"; else echo none; fi)"
  printf '%-40s %-40s %-12s %s\n' WORKTREE BRANCH LAST-BUILD TARGET-ALLOCATED
  local tree slug dest branch last used
  for tree in "$trees"/*/; do
    [ -d "$tree" ] || continue
    tree=${tree%/}
    slug=$(basename "$tree")
    dest=$targets/$slug
    branch=$(git -C "$tree" symbolic-ref -q --short HEAD || echo "(detached)")
    if [ -d "$dest" ]; then
      last=$(find "$dest" -mindepth 1 -maxdepth 3 -printf '%TY-%Tm-%Td\n' 2>/dev/null | sort | tail -1)
      used=$(du -sh "$dest" 2>/dev/null | cut -f1)
    else
      last=-
      used=-
    fi
    printf '%-40s %-40s %-12s %s\n' "$slug" "$branch" "${last:--}" "$used"
  done
  echo "Allocated sizes count blocks shared with the seed in full; the dataset totals below do not."
  df -h --output=target,used,avail "$build" "$main"
  if command -v zpool >/dev/null; then
    zpool get -H -o name,property,value bclonesaved,bcloneratio "$(df --output=source "$build" | tail -1 | cut -d/ -f1)" || true
  fi
}

command=${1:-}
[ -n "$command" ] || usage
shift
case $command in
  new) cmd_new "$@" ;;
  seed) cmd_seed "$@" ;;
  reset) cmd_reset "$@" ;;
  gc) cmd_gc "$@" ;;
  rm) cmd_rm "$@" ;;
  prune) cmd_prune "$@" ;;
  status) cmd_status ;;
  *) usage ;;
esac
