#!/usr/bin/env bash
# Install as the desktop owner. Does not restart a running desktop.
set -euo pipefail
if [[ $EUID == 0 || $# -ne 1 ]]; then
  echo "Usage: bash $0 /absolute/path/to/workstation-config.json (as the desktop owner)" >&2
  exit 1
fi
recipe_dir=$(cd -- "$(dirname -- "$0")" && pwd)
python3 "$recipe_dir/bridge/deployment_config.py" workstation "$1"
bridge_dir="$HOME/.local/share/coda-engineering/slack-bridge"
install -d -m 700 "$bridge_dir" "$HOME/.config/systemd/user"
python3 -m venv "$bridge_dir/.venv"
"$bridge_dir/.venv/bin/pip" install --disable-pip-version-check -r "$recipe_dir/bridge/requirements.txt"
for module in codex_rpc desktop-codex desktop_with_keyring native_codex start_desktop \
    deployment_config with_doppler workstation_connector mcp_transport github_mcp github_access slack_mcp slack_history; do
  install -m 600 "$recipe_dir/bridge/$module.py" "$bridge_dir/"
done
install -m 600 "$1" "$bridge_dir/workstation-config.json"
python3 - "$recipe_dir" "$bridge_dir" <<'PY'
from pathlib import Path
import os
import shlex
import sys
recipe, bridge = map(Path, sys.argv[1:])
home = Path.home()
launcher = bridge / 'codex-for-desktop'
launcher.write_text('#!/bin/sh\nexec ' + shlex.quote(str(bridge / '.venv/bin/python')) + ' '
                    + shlex.quote(str(bridge / 'desktop-codex.py')) + ' "$@"\n')
launcher.chmod(0o700)
name = 'coda-codex-desktop'
text = (recipe / 'templates' / (name + '.service.in')).read_text().replace('@HOME@', str(home))
(home / '.config/systemd/user' / (name + '.service')).write_text(text)
start = bridge / 'start-desktop'
start.write_text('#!/bin/sh\nexec /usr/bin/python3 ' + shlex.quote(str(bridge / 'start_desktop.py')) + '\n')
start.chmod(0o700)
autostart = home / '.config/autostart/chatgpt.desktop'
autostart.parent.mkdir(parents=True, exist_ok=True)
autostart.write_text('[Desktop Entry]\nType=Application\nName=Codex workstation\nExec=' + str(start)
                     + '\nTerminal=false\nX-GNOME-Autostart-enabled=true\n')
PY
"$bridge_dir/.venv/bin/python" "$recipe_dir/bridge/client_setup.py" mcp \
  --config "$HOME/.codex/config.toml" \
  --python "$bridge_dir/.venv/bin/python" \
  --bridge "$bridge_dir"
systemctl --user daemon-reload
# Credential retrieval belongs to native startup. A separate preflight exchange
# can consume the same GCE OIDC token that Doppler would see at startup.
echo 'Installed. At an idle desktop, quit the existing ChatGPT app and run the installed start-desktop launcher.'
