#!/usr/bin/env bash
# Host packages for the Ubuntu engineering desktop. Run only on an approved VM.
# Developer toolchain versions belong to the repository's flake.lock.
set -euo pipefail

if [[ $EUID -ne 0 ]]; then
  echo 'Run with sudo on the target Ubuntu VM.' >&2
  exit 1
fi
# shellcheck source=/dev/null
. /etc/os-release
if [[ $ID != ubuntu || $VERSION_ID != 24.04 || $(dpkg --print-architecture) != amd64 ]]; then
  echo 'This recipe has been exercised only on Ubuntu 24.04 amd64.' >&2
  exit 1
fi

export DEBIAN_FRONTEND=noninteractive
export NEEDRESTART_MODE=l
apt-get update
apt-get install -y --no-install-recommends \
  xfce4 xfce4-terminal dbus-x11 tigervnc-standalone-server tigervnc-common \
  novnc websockify xauth x11-xserver-utils gnome-keyring libsecret-1-0 \
  git gh curl ca-certificates gnupg jq unzip zip python3 python3-venv \
  build-essential pkg-config libssl-dev docker.io docker-compose-v2 \
  fonts-liberation fonts-noto-color-emoji
systemctl enable --now docker
install -d -m 755 /opt/coda-engineering

installed() { dpkg-query -W -f='${Status}' "$1" 2>/dev/null | grep -qx 'install ok installed'; }
if ! installed chatgpt; then
  curl --proto '=https' --tlsv1.2 -fL \
    https://persistent.oaistatic.com/codex-app-prod/linux/deb/latest/chatgpt_amd64.deb \
    -o /opt/coda-engineering/chatgpt_amd64.deb
  apt-get install -y /opt/coda-engineering/chatgpt_amd64.deb
fi
if ! installed google-chrome-stable; then
  curl --proto '=https' --tlsv1.2 -fL \
    https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb \
    -o /opt/coda-engineering/google-chrome-stable_amd64.deb
  apt-get install -y /opt/coda-engineering/google-chrome-stable_amd64.deb
fi

# Verify the vendor key before configuring its package repository.
curl --proto '=https' --tlsv1.2 -fL \
  https://downloads.claude.ai/claude-desktop/key.asc \
  -o /opt/coda-engineering/claude-desktop-key.asc
fingerprint=$(gpg --batch --with-colons --show-keys \
  /opt/coda-engineering/claude-desktop-key.asc | awk -F: '$1 == "fpr" {print $10; exit}')
if [[ $fingerprint != 31DDDE24DDFAB679F42D7BD2BAA929FF1A7ECACE ]]; then
  echo 'Anthropic key changed; verify against current vendor documentation.' >&2
  exit 1
fi
install -m 644 /opt/coda-engineering/claude-desktop-key.asc \
  /usr/share/keyrings/claude-desktop-archive-keyring.asc
cat > /etc/apt/sources.list.d/claude-desktop.list <<'EOF'
deb [arch=amd64,arm64 signed-by=/usr/share/keyrings/claude-desktop-archive-keyring.asc] https://downloads.claude.ai/claude-desktop/apt/stable stable main
EOF
apt-get update
apt-get install -y claude-desktop

if [[ ! -x /nix/var/nix/profiles/default/bin/nix ]]; then
  curl --proto '=https' --tlsv1.2 -fL \
    https://artifacts.nixos.org/nix-installer/nix-installer-x86_64-linux \
    -o /opt/coda-engineering/nix-installer
  chmod 755 /opt/coda-engineering/nix-installer
  /opt/coda-engineering/nix-installer install linux --no-confirm --enable-flakes
fi
systemctl enable --now nix-daemon.socket

if ! snap list google-cloud-cli >/dev/null 2>&1; then
  snap install google-cloud-cli --classic
fi

echo 'Host packages installed. Continue with desktop, account sign-in, and native task setup in README.md.'
