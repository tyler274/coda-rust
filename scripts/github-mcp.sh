#!/usr/bin/env bash
# Runs GitHub's MCP server over stdio for Cursor's project config.
# It signs in with GITHUB_MCP_TOKEN when set, as Codex and Claude Code do, and
# otherwise with the developer's `gh` login, so Cursor needs no token variable.
# With Nix it runs the server from this checkout's locked nixpkgs. Without Nix,
# as on the Apple container route, it runs the Brewfile's github-mcp-server.
# Keep it to macOS's bash 3.2: no arrays or newer expansions.
set -euo pipefail

token=${GITHUB_MCP_TOKEN:-}
if [[ -z $token ]]; then
  if ! command -v gh >/dev/null; then
    echo "github-mcp: set GITHUB_MCP_TOKEN, or install gh and run \`gh auth login\`." >&2
    exit 1
  fi
  status=0
  token=$(gh auth token) || status=$?
  if [[ $status -ne 0 || -z $token ]]; then
    echo "github-mcp: run \`gh auth login\`, or set GITHUB_MCP_TOKEN." >&2
    exit 1
  fi
fi
export GITHUB_PERSONAL_ACCESS_TOKEN=$token

if command -v nix >/dev/null; then
  root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
  exec nix shell --no-warn-dirty --inputs-from "$root" nixpkgs#github-mcp-server \
    --command github-mcp-server stdio
fi
if ! command -v github-mcp-server >/dev/null; then
  echo "github-mcp: install Nix, or run \`brew bundle\` for github-mcp-server." >&2
  exit 1
fi
exec github-mcp-server stdio
