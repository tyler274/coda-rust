#!/usr/bin/env bash
# Bootstrap the pinned single-user Nix client for disposable Linux environments.
# Existing workstation/host-daemon installations keep their own installation.
# Prints only the bin directory; callers put it on PATH.
set -euo pipefail

nix_version=2.34.8
nix_tarball="nix-${nix_version}-x86_64-linux.tar.xz"
nix_digest=2c2e146b80834fe0ca201b51deeb939405b4f18e8d2071bf80b10f8123c50464
# The dev container can mount the workstation store without a default profile.
host_nix_bin=/nix/store/mls1lldgdxny8i0ll3n1asb1kn38fw73-nix-2.34.8/bin
for candidate in /nix/var/nix/profiles/default/bin "$HOME/.nix-profile/bin" "$host_nix_bin"; do
  if [[ -x "$candidate/nix" ]]; then
    printf '%s\n' "$candidate"
    exit 0
  fi
done
if command -v nix >/dev/null; then
  dirname "$(command -v nix)"
  exit 0
fi
if [[ $(uname -s) != Linux || $(uname -m) != x86_64 ]]; then
  echo 'This bootstrap requires x86_64 Linux.' >&2
  exit 1
fi
if [[ ! -w /nix ]]; then
  echo 'Provide a writable /nix directory or the existing host Nix store first.' >&2
  exit 1
fi
nix_install_dir=$(mktemp -d)
trap 'rm -rf "$nix_install_dir"' EXIT
curl --proto '=https' --tlsv1.2 -fsSL \
  "https://releases.nixos.org/nix/nix-${nix_version}/${nix_tarball}" \
  -o "$nix_install_dir/$nix_tarball"
printf '%s  %s\n' "$nix_digest" "$nix_install_dir/$nix_tarball" | sha256sum -c --status
tar -xJf "$nix_install_dir/$nix_tarball" -C "$nix_install_dir"
WRITE_TO_NIX_STORE=1 "$nix_install_dir/nix-${nix_version}-x86_64-linux/install" --no-daemon --no-modify-profile >&2
if [[ ! -x "$HOME/.nix-profile/bin/nix" ]]; then
  echo 'Nix installation did not produce a user profile.' >&2
  exit 1
fi
printf '%s\n' "$HOME/.nix-profile/bin"
