#!/usr/bin/env bash
# One-time container preparation. Idempotent: every step checks before acting,
# so reopening the container reuses the bound store and the applied database.
set -euo pipefail

workspace=/workspace
nix_version=2.34.8
nix_tarball="nix-${nix_version}-x86_64-linux.tar.xz"
nix_digest=2c2e146b80834fe0ca201b51deeb939405b4f18e8d2071bf80b10f8123c50464

# --- Runtime packages --------------------------------------------------------
# Two additions the thin image needs: xz to unpack the fallback Nix tarball,
# and psql for the migration applier. Compose pins resolvers that answer
# (see docker-compose.yml dns); the build cannot.
if ! command -v psql >/dev/null || ! command -v xz >/dev/null; then
  sudo apt-get update -qq
  sudo apt-get install -y -qq --no-install-recommends xz-utils postgresql-client >/dev/null
fi

# --- Nix ---------------------------------------------------------------------
# The host binds its populated /nix through (docker-compose.yml), so the
# container reuses the host's store through its daemon socket; nix resolves to
# the store path the host system profile already pins. Fall back to a pinned
# single-user install into /nix when that bind is absent (hosts without Nix
# use the named volume there instead).
nix_bin=/nix/store/mls1lldgdxny8i0ll3n1asb1kn38fw73-nix-2.34.8/bin
if [ -x /nix/var/nix/profiles/default/bin/nix ] || [ -x "$nix_bin/nix" ]; then
  echo "reusing the host's /nix store"
else
  if [ ! -w /nix ]; then
    echo "/nix is not writable and no host store is bound" >&2
    exit 1
  fi
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' EXIT
  curl -fsSL "https://releases.nixos.org/nix/nix-${nix_version}/${nix_tarball}" -o "$tmp/$nix_tarball"
  echo "$nix_digest  $tmp/$nix_tarball" | sha256sum -c --status
  tar -xJf "$tmp/$nix_tarball" -C "$tmp"
  WRITE_TO_NIX_STORE=1 "$tmp"/*/install --no-daemon >/dev/null
  nix_bin=/nix/var/nix/profiles/default/bin
fi
if [ -x /nix/var/nix/profiles/default/bin/nix ]; then
  nix_bin=/nix/var/nix/profiles/default/bin
fi
export PATH="$nix_bin:$PATH"
grep -q 'nix-2.34.8/bin\|profiles/default/bin' "$HOME/.bashrc" 2>/dev/null || \
  printf '\nexport PATH="%s:$PATH"\n' "$nix_bin" >> "$HOME/.bashrc"
# Non-interactive shells (VS Code tasks, `docker exec`) never read .bashrc;
# expose nix on the default PATH for every shell.
sudo ln -sf "$(command -v nix)" /usr/local/bin/nix

# --- Submodules: fetch only what is missing ---------------------------------
# Never overwrite an existing checkout; the host's checkout binds here.
for sub in $(git -C "$workspace" config --file .gitmodules --get-regexp path | awk '{print $3}'); do
  if [ -z "$(ls -A "$workspace/$sub" 2>/dev/null)" ]; then
    git -C "$workspace" submodule update --init --depth 1 -- "$sub"
  fi
done

# --- Materialize the nix dev shell ------------------------------------------
# Also the first real exercise of the flake inside the container.
cd "$workspace"
nix develop --command true

# --- Database schema ---------------------------------------------------------
# Fail closed: duet-api refuses to serve a database missing its migration.
# With network_mode: host the postgres service listens on host loopback 5433,
# and POSTGRES_HOST_AUTH_METHOD=trust admits the local connection.
nix develop --command cargo run -p workspace-tools -- apply-migrations -- psql -v ON_ERROR_STOP=1 -h 127.0.0.1 -p 5433 -U coda -d duet

echo "dev container ready"
