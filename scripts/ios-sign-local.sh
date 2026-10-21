#!/usr/bin/env bash
# Local signing only; no upload/deploy. Run in nix develop on an authorized macOS host.
set -euo pipefail
: "${IN_NIX_SHELL:?Run inside nix develop}"
: "${DUET_IOS_SIGNING_IDENTITY:?Set an authorized Apple development signing identity}"
: "${DUET_IOS_BUNDLE_ID:?Set the registered bundle identifier}"
app=${1:?Pass the dx-built .app path}
root=$(git rev-parse --show-toplevel)
python "$root/scripts/ios-sign-local.py" "$app"
