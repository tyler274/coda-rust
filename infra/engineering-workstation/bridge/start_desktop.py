#!/usr/bin/env python3
"""Start user services with this Xfce session's display and accessibility bus."""
import os
from pathlib import Path
import subprocess


def main():
    runtime = os.environ['XDG_RUNTIME_DIR']
    names = ('DISPLAY', 'XAUTHORITY', 'DBUS_SESSION_BUS_ADDRESS', 'XDG_RUNTIME_DIR',
             'XDG_SESSION_TYPE', 'XDG_CURRENT_DESKTOP', 'DESKTOP_SESSION')
    def quoted(value):
        return '"' + value.replace('\\', '\\\\').replace('"', '\\"').replace('`', '\\`').replace('$', '\\$') + '"'
    values = [f'{name}={quoted(os.environ[name])}' for name in names if name in os.environ]
    # Xfce under TigerVNC has its own session bus. systemd lives on the user
    # manager bus; the desktop and its accessibility tools still need Xfce's bus.
    control = dict(os.environ, DBUS_SESSION_BUS_ADDRESS=f'unix:path={runtime}/bus')
    # Scope the GUI bus to the desktop services. Setting it on the whole user
    # manager also redirects unrelated D-Bus-activated services such as keyring.
    environment = Path(runtime) / 'coda-desktop-session.env'
    environment.write_text('\n'.join(values) + '\n')
    environment.chmod(0o600)
    connector = Path.home() / '.config/systemd/user/coda-workstation-connector.service'
    services = ['coda-codex-desktop.service']
    if connector.exists():
        services.append('coda-workstation-connector.service')
    subprocess.run(['systemctl', '--user', 'start', *services], env=control, check=True)


if __name__ == '__main__':
    main()
