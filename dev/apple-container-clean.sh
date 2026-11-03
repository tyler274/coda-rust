#!/usr/bin/env bash
# Reclaims disk space inside the Apple container; dev/apple-container.sh runs it
# here. The container's disk images never shrink when files are deleted, so this
# ends by trimming them, which gives the freed space back to the Mac. One step
# failing doesn't stop the others; the script then exits non-zero.
#
#   apple-container-clean.sh <flake>   the checkout whose dev shell to keep
set -uo pipefail

flake=$1
stamp=/nix/var/coda-last-clean
keep_shell=/nix/var/nix/profiles/coda-agent
target=/workspace/target
PATH="$HOME/.nix-profile/bin:$PATH"
status=0

exec 9>/tmp/coda-clean.lock
flock -n 9 || exit 0
touch "$stamp"

# `nix develop --command` execs its command, so nothing removes the shell's
# temporary directory afterwards. A directory a live process names stays.
live_temp_dirs=$(
  for environ in /proc/[0-9]*/environ; do
    { tr '\0' '\n' <"$environ"; } 2>/dev/null || true
  done | sed -n 's/^\(TMPDIR\|TMP\|TEMP\|NIX_BUILD_TOP\)=//p' | sort -u
)
while IFS= read -r -d '' dir; do
  grep -qxF "$dir" <<<"$live_temp_dirs" || rm -rf "$dir" || status=1
done < <(find /tmp -maxdepth 1 -name 'nix-shell.*' -mmin +60 -print0)

# Root the current dev shell first, so collection leaves it to reuse.
if nix develop "$flake#agent" --profile "$keep_shell" --command true &&
  nix-env --profile "$keep_shell" --delete-generations old; then
  nix-store --gc || status=1
else
  status=1
fi

# Cargo never removes old artifacts, and a missing one only costs a rebuild. A
# build holds .cargo-lock, so a target directory in use waits for the next run.
# Only Cargo's directories are touched: target also holds verification evidence.
while IFS= read -r -d '' lock; do
  flock -n -E 0 "$lock" find "$(dirname "$lock")" \
    \( -type f -mtime +3 ! -name .cargo-lock -o -type d -empty \) -delete || status=1
done < <(find "$target" -name .cargo-lock -print0)

for mount in / /nix /home/vscode/.cargo "$target"; do
  if ! sudo fstrim --verbose "$mount"; then
    echo "could not trim $mount, so the Mac keeps this space. A container made before it" >&2
    echo "had CAP_SYS_ADMIN must be recreated: container rm -f <name>, then dev/apple-container.sh" >&2
    status=1
  fi
done

exit "$status"
