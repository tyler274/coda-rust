#!/usr/bin/env bash
# Open-Inspect runs this existing repository hook on fresh and image-build boots.
# A prepared repository image retains /nix; no account or secret is baked here.
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
if [[ $(uname -s) != Linux || $(uname -m) != x86_64 ]]; then
  echo 'The Coda sandbox profile requires x86_64 Linux.' >&2
  exit 1
fi
# E2B runs hooks as its regular user, with passwordless sudo in the base image.
# Escalate only to create the store; the Nix install and toolchain run as the user.
if [[ ! -d /nix ]]; then
  if [[ $EUID == 0 ]]; then
    install -d -m 755 /nix
  else
    sudo -n install -d -m 755 -o "$(id -u)" -g "$(id -g)" /nix
  fi
fi
mkdir -p "$HOME/.config/nix"
# This is a disposable single-user sandbox, with no host daemon or credentials.
cat > "$HOME/.config/nix/nix.conf" <<'NIX'
experimental-features = nix-command flakes
sandbox = false
filter-syscalls = false
build-users-group =
NIX
nix_bin=$(bash "$root/nix/ensure-user-nix.sh")
export PATH="$nix_bin:$PATH"
cd "$root"
# The source owns all versions; this profile shares its toolchain with the desktop.
nix develop .#agent --command true
