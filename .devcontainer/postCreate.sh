#!/usr/bin/env bash
# One-time container preparation. Idempotent: every step checks before acting,
# so reopening the container reuses the bound store and the applied database.
set -euo pipefail

workspace=/workspace

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
nix_bin=$(bash "$workspace/nix/ensure-user-nix.sh")
export PATH="$nix_bin:$PATH"
# Keep $PATH literal for the shell that later reads .bashrc.
# shellcheck disable=SC2016
grep -Fq "$nix_bin" "$HOME/.bashrc" 2>/dev/null || \
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

# Schema setup is explicit; never replay the legacy coda-app ledger here.
# README.md and book/src/backend-architecture.md describe the Rust migration
# owner and distinct runtime role. Existing volumes are left untouched.

echo "dev shell ready; follow README.md for explicit Rust database setup"
