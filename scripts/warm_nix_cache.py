#!/usr/bin/env python3
"""Launch the cache-only act workflow with explicitly selected Doppler secrets."""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
SECRET_NAMES = (
    'NIX_CACHE_WRITE_ACCESS_KEY_ID', 'NIX_CACHE_WRITE_SECRET_ACCESS_KEY',
    'NIX_CACHE_SIGNING_KEY', 'NIX_CACHE_READ_ACCESS_KEY_ID',
    'NIX_CACHE_READ_SECRET_ACCESS_KEY',
)
VARIABLE_NAMES = ('NIX_CACHE_BUCKET', 'NIX_CACHE_PUBLIC_KEY')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--expected-sha', required=True)
    parser.add_argument('--doppler-config', default='stg_nix_cache')
    parser.add_argument('--acknowledge-exclusive-writer', action='store_true')
    args = parser.parse_args()
    if not args.acknowledge_exclusive_writer:
        parser.error('Pause hosted publication and drain active writers, then acknowledge the exclusive window')
    if not re.fullmatch(r'[0-9a-f]{40}', args.expected_sha):
        parser.error('Use the full reviewed commit SHA')
    head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    dirty = subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=normal'], cwd=ROOT, text=True)
    if head != args.expected_sha or dirty:
        parser.error('The checkout must be clean and match the reviewed commit')
    result = subprocess.run(['doppler', 'secrets', 'get', *SECRET_NAMES, *VARIABLE_NAMES,
                             '--project', 'coda-rust', '--config', args.doppler_config, '--json'],
                            capture_output=True, text=True)
    if result.returncode:
        parser.exit(1, 'Unable to retrieve scoped cache credentials from Doppler; output withheld.\n')
    try:
        values = json.loads(result.stdout)
        selected = {name: values[name]['computed'] for name in (*SECRET_NAMES, *VARIABLE_NAMES)}
        if any(not isinstance(value, str) or not value or '\n' in value for value in selected.values()):
            raise ValueError()
    except (ValueError, KeyError, TypeError):
        parser.exit(1, 'Doppler did not return the required single-line cache settings.\n')
    child = dict(os.environ, **selected)
    command = ['act', 'workflow_dispatch', '-W', '.github/workflows/nix-cache-local.yml',
               '-j', 'nix-cache-local', '--var', 'ACT=true', '--container-architecture', 'linux/amd64',
               '--container-daemon-socket', '-', '--rm',
               '--secret-file', '/dev/null', '--env-file', '/dev/null',
               '--var-file', '/dev/null', '--input-file', '/dev/null',
               '--input', f'expected_sha={args.expected_sha}']
    for name in (*SECRET_NAMES, *VARIABLE_NAMES):
        command.extend(['--secret', name])
    print('Starting cache-only act workflow; only named cache credentials are passed.', flush=True)
    return subprocess.call(command, cwd=ROOT, env=child)


if __name__ == '__main__':
    sys.exit(main())
