#!/usr/bin/env python3
"""The job's /nix on a Blacksmith sticky disk: measure it, bound it, release it.

start runs right after Nix is installed over the mounted disk and records how
many store paths the disk brought. finish runs last in the job, after the
private cache cleanup: it reports the store's size and the paths the job added
(fetched from a substituter or built), collects garbage when the store is over
its cap, and stops nix-daemon so the sticky disk's post step can unmount /nix
and commit it.
"""

import argparse
from dataclasses import dataclass
import os
from pathlib import Path
import shutil
import subprocess
import sys


NIX_ROOT = Path('/nix')
STORE = NIX_ROOT / 'store'
# Each install over a warm disk adds a generation to the default profile.
DEFAULT_PROFILE = NIX_ROOT / 'var/nix/profiles/default'
GIB = 1024 ** 3
# GC frees down to this share of the cap, so the next job has headroom.
GC_TARGET = 0.8
# The cap never exceeds this share of the disk, whatever its configured size.
DISK_SHARE = 0.8
START_VAR = 'NIX_STORE_PATHS_AT_START'


@dataclass(frozen=True)
class Usage:
    used: int
    size: int


def store_paths():
    if not STORE.is_dir():
        return 0
    return sum(1 for entry in os.scandir(STORE) if not entry.name.startswith('.'))


def usage():
    stats = shutil.disk_usage(NIX_ROOT)
    return Usage(used=stats.used, size=stats.total)


def cap_bytes(cap_gib, disk):
    return min(int(cap_gib * GIB), int(disk.size * DISK_SHARE))


def bytes_to_free(cap_gib, disk):
    """Bytes GC must free, or 0 while the store is within its cap."""
    cap = cap_bytes(cap_gib, disk)
    if disk.used <= cap:
        return 0
    return disk.used - int(cap * GC_TARGET)


def gib(value):
    return f'{value / GIB:.1f}'


def append(path, line):
    if path:
        with open(path, 'a') as out:
            out.write(line + '\n')


def start(env):
    paths = store_paths()
    disk = usage()
    state = 'warm' if paths else 'cold'
    print(f'Nix store start disk={state} paths={paths} used_gib={gib(disk.used)} '
          f'size_gib={gib(disk.size)}', flush=True)
    append(env.get('GITHUB_ENV'), f'{START_VAR}={paths}')


def collect_garbage(amount, run=subprocess.run):
    # Old default-profile generations pin earlier installers' Nix; drop them first.
    run(['sudo', '-n', str(DEFAULT_PROFILE / 'bin/nix-env'), '--profile', str(DEFAULT_PROFILE),
         '--delete-generations', 'old'], check=True)
    run(['nix', 'store', 'gc', '--max', str(amount)], check=True)


def stop_daemon(run=subprocess.run):
    """Release /nix so the sticky disk's post step can unmount and commit it."""
    run(['sudo', '-n', 'systemctl', 'stop', 'nix-daemon.socket', 'nix-daemon.service'],
        check=True)
    if shutil.which('fuser'):
        # Anything a nix develop shell left running (an emulator, a server).
        run(['sudo', '-n', 'fuser', '-k', '-m', str(NIX_ROOT)], check=False)


def finish(env, cap_gib, run=subprocess.run):
    at_start = env.get(START_VAR)
    paths = store_paths()
    disk = usage()
    added = paths - int(at_start) if at_start and at_start.isdigit() else None
    amount = bytes_to_free(cap_gib, disk)
    line = (f'Nix store summary paths_at_start={at_start or "unknown"} '
            f'paths_added={"unknown" if added is None else added} '
            f'used_gib={gib(disk.used)} cap_gib={gib(cap_bytes(cap_gib, disk))} '
            f'gc={"yes" if amount else "no"}')
    print(line, flush=True)
    append(env.get('GITHUB_STEP_SUMMARY'), line)
    try:
        if amount:
            collect_garbage(amount, run)
            print(f'Nix store after gc used_gib={gib(usage().used)}', flush=True)
    finally:
        stop_daemon(run)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    commands.add_parser('start')
    end = commands.add_parser('finish')
    end.add_argument('--cap-gib', type=float, required=True)
    args = parser.parse_args()
    try:
        if args.command == 'start':
            start(os.environ)
        else:
            finish(os.environ, args.cap_gib)
    except (OSError, subprocess.CalledProcessError) as error:
        print(f'Nix store {args.command} failed: {error}', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
