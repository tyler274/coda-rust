#!/usr/bin/env python3
"""The reusable dependency and image roots shared by hosted and local cache writers."""

import argparse
import json
from pathlib import Path
import re
import subprocess


ROOT = Path(__file__).resolve().parents[2]
DEPENDENCY_ROOTS = ('coda-servers-deps', 'coda-web-deps', 'patient-bank-deps')
IMAGE_ROOTS = (
    'physician-sample-image', 'physician-worker-image', 'duet-api-image',
    'physician-migration-image', 'duet-migration-image', 'duet-privacy-delivery-image',
    'observability-relay-image',
)
# CI jobs enter these shells; otherwise each job rebuilds their wrappers
# (claude-code, google-cloud-sdk) and refetches them from cache.nixos.org.
SHELL_ROOTS = tuple(f'devShells.x86_64-linux.{name}'
                    for name in ('agent', 'postgres-ci', 'release', 'observability', 'android'))
ROOTS = DEPENDENCY_ROOTS + IMAGE_ROOTS + SHELL_ROOTS


def verify_checkout(expected_sha):
    """Fail closed before the local workflow receives publication credentials."""
    if not re.fullmatch(r'[0-9a-f]{40}', expected_sha):
        raise ValueError('Expected a full lowercase 40-character commit SHA')
    head = subprocess.run(['git', 'rev-parse', 'HEAD'], cwd=ROOT, check=True,
                          capture_output=True, text=True).stdout.strip()
    status = subprocess.run(
        ['git', 'status', '--porcelain', '--untracked-files=all'], cwd=ROOT,
        check=True, capture_output=True, text=True).stdout
    if head != expected_sha:
        raise ValueError('Checkout HEAD does not match the expected commit')
    if status:
        raise ValueError('Checkout must be clean, including untracked files')


def build(output):
    # Do not accept arbitrary installables; both writers use exactly this list.
    with output.open('w') as target:
        subprocess.run(
            ['nix', 'build', '--no-link', '--json', '--no-update-lock-file',
             *[f'.#{name}' for name in ROOTS]], cwd=ROOT, stdout=target, check=True)


def smoke(build_json, output):
    # Select a leaf derivation already in the built closure, not an arbitrary
    # operator-supplied installable. Its complete closure is signed normally.
    from nix_cache import selected_paths, STORE_PATH
    roots = selected_paths([], str(build_json))
    closure = subprocess.check_output(['nix-store', '-qR', '--include-outputs', *roots],
                                      text=True).splitlines()
    candidates = sorted(p for p in closure if p.endswith('.drv') and STORE_PATH.fullmatch(p))
    for path in candidates:
        refs = subprocess.check_output(['nix-store', '-q', '--references', path], text=True).splitlines()
        if not refs:
            output.write_text(json.dumps([{'drvPath': path, 'outputs': {}}]))
            return
    raise ValueError('No leaf derivation found for smoke publication')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    collect = commands.add_parser('build')
    collect.add_argument('--output', type=Path, required=True)
    verify = commands.add_parser('verify-checkout')
    verify.add_argument('--expected-sha', required=True)
    small = commands.add_parser('smoke')
    small.add_argument('--build-json', type=Path, required=True)
    small.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    try:
        if args.command == 'smoke':
            smoke(args.build_json, args.output)
        elif args.command == 'build':
            build(args.output)
        else:
            verify_checkout(args.expected_sha)
    except (ValueError, OSError, subprocess.CalledProcessError) as exc:
        if isinstance(exc, ValueError):
            parser.exit(1, f'{exc}\n')
        parser.exit(1, 'Nix root collection or checkout verification failed\n')


if __name__ == '__main__':
    main()
