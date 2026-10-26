#!/usr/bin/env bash
# Install as the desktop owner after the shared gateway and scoped IAP grant exist.
# Does not restart the native desktop or enable delivery.
set -euo pipefail
if [[ $EUID == 0 || $# -ne 3 ]]; then
  echo "Usage: bash $0 GCP_PROJECT GATEWAY_ZONE GATEWAY_VM (as the desktop owner)" >&2
  exit 1
fi
recipe_dir=$(cd -- "$(dirname -- "$0")" && pwd)
bridge_dir="$HOME/.local/share/coda-engineering/slack-bridge"
test -x "$bridge_dir/.venv/bin/python"
command -v gcloud >/dev/null
install -m 600 "$recipe_dir/bridge/workstation_connector.py" "$bridge_dir/"
python3 - "$recipe_dir" "$@" <<'PY'
from pathlib import Path
import re, sys
recipe, project, zone, gateway = sys.argv[1:]
for value in (project, zone, gateway):
    if not re.fullmatch(r'[a-z0-9][a-z0-9-]*', value):
        raise SystemExit('Use GCP resource names')
directory = Path.home() / '.config/systemd/user'
directory.mkdir(parents=True, exist_ok=True)
for name in ('coda-gateway-tunnel', 'coda-workstation-connector'):
    text = (Path(recipe) / 'templates' / (name + '.service.in')).read_text()
    for key, value in {'HOME':str(Path.home()),'PROJECT':project,'ZONE':zone,'GATEWAY':gateway}.items():
        text = text.replace('@' + key + '@', value)
    (directory / (name + '.service')).write_text(text)
PY
systemctl --user daemon-reload
echo 'Installed. Verify the configured gateway and scoped IAP grant, then start coda-workstation-connector.'
