#!/usr/bin/env bash
# Install GitHub's official MCP server on the shared gateway. No credential setup.
set -euo pipefail
if [[ $EUID != 0 || $(uname -s) != Linux || $(uname -m) != x86_64 ]]; then
  echo 'Run as root on the approved Linux amd64 gateway.' >&2
  exit 1
fi
version=2.0.3
checksum=f797c20fe02a755865d0e2d46184ab47e31bf1619b0855b8d89c531e45a651c2
archive=$(mktemp)
directory=$(mktemp -d)
trap 'rm -f "$archive"; rm -rf "$directory"' EXIT
curl --fail --silent --show-error --location \
  "https://github.com/github/github-mcp-server/releases/download/v${version}/github-mcp-server_Linux_x86_64.tar.gz" \
  --output "$archive"
printf '%s  %s\n' "$checksum" "$archive" | sha256sum --check --status
tar -xzf "$archive" -C "$directory" github-mcp-server
install -d -m 755 /opt/coda-agent-gateway
install -m 755 "$directory/github-mcp-server" /opt/coda-agent-gateway/github-mcp-server
/opt/coda-agent-gateway/github-mcp-server --version
