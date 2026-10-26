#!/usr/bin/env bash
# Configure the private desktop for an existing OS Login user.
set -euo pipefail
if [[ $EUID -ne 0 || $# -ne 1 ]]; then
  echo "Usage: sudo bash $0 OS_LOGIN_USER" >&2
  exit 1
fi
desktop_user=$1
desktop_uid=$(id -u "$desktop_user")
desktop_group=$(id -gn "$desktop_user")
desktop_home=$(getent passwd "$desktop_user" | cut -d: -f6)
recipe_dir=$(cd -- "$(dirname -- "$0")" && pwd)
if [[ $desktop_uid == 0 || ! -d $desktop_home || ! -s $desktop_home/.vnc/passwd ]]; then
  echo 'First log in as the non-root owner and run tigervncpasswd in ~/.vnc.' >&2
  exit 1
fi
# Do not replace a running desktop's configuration under an active user.
if systemctl is-active --quiet coda-desktop.service; then
  echo 'Desktop already running. Review changes and arrange an idle maintenance window.' >&2
  exit 1
fi
usermod -aG docker "$desktop_user"
loginctl enable-linger "$desktop_user"
timedatectl set-timezone America/New_York
install -d -o "$desktop_user" -g "$desktop_group" -m 700 "$desktop_home/.vnc"
chmod 600 "$desktop_home/.vnc/passwd"
chown "$desktop_user:$desktop_group" "$desktop_home/.vnc/passwd"
install -o "$desktop_user" -g "$desktop_group" -m 755 \
  "$recipe_dir/templates/xstartup" "$desktop_home/.vnc/xstartup"
install -d -o "$desktop_user" -g "$desktop_group" -m 755 "$desktop_home/.config/autostart"
install -o "$desktop_user" -g "$desktop_group" -m 644 \
  "$recipe_dir/templates/chatgpt.desktop" "$desktop_home/.config/autostart/chatgpt.desktop"
python3 - "$recipe_dir" "$desktop_user" "$desktop_group" "$desktop_home" "$desktop_uid" <<'PY'
from pathlib import Path
import sys
root, user, group, home, uid = sys.argv[1:]
for unit in ('coda-desktop', 'coda-desktop-web'):
    content = (Path(root) / 'templates' / (unit + '.service.in')).read_text()
    for key, value in {'USER': user, 'GROUP': group, 'HOME': home, 'UID': uid}.items():
        content = content.replace('@' + key + '@', value)
    Path('/etc/systemd/system/' + unit + '.service').write_text(content)
PY
systemctl daemon-reload
systemctl enable --now coda-desktop.service coda-desktop-web.service
echo 'Connect over the IAP SSH tunnel, then finish account sign-in in the desktop.'
