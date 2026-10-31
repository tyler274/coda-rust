#!/usr/bin/env bash
# Install on an approved Ubuntu gateway VM. Does not start Slack delivery.
set -euo pipefail
if [[ $EUID != 0 || $# -ne 2 ]]; then
  echo "Usage: sudo bash $0 /absolute/path/to/gateway-config.json PRIVATE_VM_IP" >&2
  exit 1
fi
recipe_dir=$(cd -- "$(dirname -- "$0")" && pwd)
python3 "$recipe_dir/bridge/deployment_config.py" gateway "$1"
python3 - "$2" <<'PY'
import ipaddress, sys
address = ipaddress.ip_address(sys.argv[1])
assert address.version == 4 and address.is_private and not address.is_loopback
PY
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y --no-install-recommends python3-venv ca-certificates curl
id coda-gateway >/dev/null 2>&1 || useradd --system --home-dir /var/lib/coda-agent-gateway --shell /usr/sbin/nologin coda-gateway
install -d -m 755 /opt/coda-agent-gateway
install -d -m 700 -o coda-gateway -g coda-gateway /var/lib/coda-agent-gateway
python3 -m venv /opt/coda-agent-gateway/.venv
/opt/coda-agent-gateway/.venv/bin/pip install --disable-pip-version-check -r "$recipe_dir/bridge/requirements.txt"
for module in codex_rpc deployment_config with_doppler slack_bridge slack_gateway \
    slack_render slack_history slack_interactions slack_mcp mcp_transport workstation_connector github_mcp github_access; do
  install -m 644 "$recipe_dir/bridge/$module.py" /opt/coda-agent-gateway/
done
install -m 640 -o root -g coda-gateway "$1" /opt/coda-agent-gateway/gateway-config.json
if python3 -c 'import json,sys; sys.exit(0 if json.load(open(sys.argv[1])).get("github") else 1)' "$1"; then
  /opt/coda-agent-gateway/.venv/bin/pip install --disable-pip-version-check -r "$recipe_dir/bridge/requirements-github.txt"
  bash "$recipe_dir/install-github-provider.sh"
fi
sed "s/@PRIVATE_IP@/$2/g" "$recipe_dir/templates/coda-agent-gateway.service.in" > /etc/systemd/system/coda-agent-gateway.service
systemctl daemon-reload
echo 'Installed. Verify IAP/firewall, credentials and the intended state directory before enabling coda-agent-gateway.'
