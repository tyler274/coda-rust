#!/usr/bin/env bash
# The sandbox image supplies Nix; this checkout owns the build environment.
set -euo pipefail
cd "$(dirname "$0")/.."
nix develop .#agent --command true
