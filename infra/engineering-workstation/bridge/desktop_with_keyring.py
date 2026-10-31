#!/usr/bin/env python3
"""Unlock the encrypted desktop keyring before starting the native app."""
import argparse
import os
import subprocess
import urllib.error

from with_doppler import fetch
from deployment_config import WORKSTATION_CONFIG, load_workstation_config


def unlock(config):
    password = fetch(('DESKTOP_KEYRING_PASSWORD',), config['doppler'])['DESKTOP_KEYRING_PASSWORD']
    result = subprocess.run(['gnome-keyring-daemon', '--unlock'], input=password.encode(),
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=30)
    del password
    if result.returncode:
        raise SystemExit('Desktop keyring unlock failed; native app was not started')
    # The password never enters the app environment, a command argument, or disk.
    # Verify the native Secret Service's unlocked state, not just process success.
    alias = subprocess.check_output([
        'gdbus', 'call', '--session', '--dest', 'org.freedesktop.secrets',
        '--object-path', '/org/freedesktop/secrets', '--method',
        'org.freedesktop.Secret.Service.ReadAlias', 'default'], text=True, timeout=10).strip()
    path = alias.split("'")[1]
    if not path.startswith('/org/freedesktop/secrets/collection/'):
        raise SystemExit('Desktop login keyring was not found')
    command = [
        'gdbus', 'call', '--session', '--dest', 'org.freedesktop.secrets',
        '--object-path', path, '--method', 'org.freedesktop.DBus.Properties.Get',
        'org.freedesktop.Secret.Collection', 'Locked']
    locked = subprocess.run(command, capture_output=True, text=True, timeout=10)
    if locked.returncode != 0 or locked.stdout.strip() != '(<false>,)':
        raise SystemExit('Desktop login keyring is still locked')
    print('Encrypted desktop keyring unlocked through VM-scoped credentials.', flush=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--config', default=str(WORKSTATION_CONFIG))
    checks = parser.add_mutually_exclusive_group()
    checks.add_argument('--check', action='store_true', help='Verify keyring unlock without starting the app')
    checks.add_argument('--check-credentials', action='store_true', help='Verify credential retrieval only')
    args = parser.parse_args()
    try:
        config = load_workstation_config(args.config)
        if args.check_credentials:
            fetch(('DESKTOP_KEYRING_PASSWORD',), config['doppler'])
            print('VM identity and desktop credential verified; value remains in memory.')
        else:
            unlock(config)
            if not args.check:
                os.execv('/usr/bin/chatgpt', ['/usr/bin/chatgpt'])
    except urllib.error.HTTPError as error:
        raise SystemExit(f'Desktop credential retrieval failed: HTTP {error.code}')
