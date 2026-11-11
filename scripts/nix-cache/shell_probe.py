#!/usr/bin/env python3
"""Temporary hosted probe: baseline before warming, then controlled A/B/C.

baseline and B time only ordinary postgres-ci entry, without source prefetch.
B verifies the sources consumed by that entry using its own Nix logs. A is an
upstream diagnostic, not the adoption baseline: it prefetches every locked input
with substitution disabled for archive only.
Measured runs have reader authority only. Publication is an operator phase.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time

from nix_cache_roots import flake_inputs

ROOT = Path(__file__).resolve().parents[2]
COPY = re.compile(r"copying path '([^']+)' from '([^']+)'")


def source_split(log):
    copies = [{'path': path, 'route': 'private-s3' if url.startswith('s3://') else 'public-or-other'}
              for path, url in COPY.findall(log)]
    return {'copies': copies, 'upstream_fetches': [line for line in log.splitlines()
            if ('unpacking' in line or 'fetching' in line) and ('github' in line or 'https://' in line)]}


def run_logged(command, directory, label):
    # Nix's ordinary verbosity preserves source/package provenance, without
    # dumping the process environment or reader credential files.
    result = subprocess.run(command, cwd=ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    (directory / f'{label}.log').write_text(result.stderr)
    print(result.stderr, end='', file=sys.stderr, flush=True)
    result.check_returncode()
    return result


def shell(variant, directory):
    directory.mkdir(parents=True, exist_ok=True)
    existing = set(Path('/nix/store').glob('*-source'))
    sources = []
    logs = ''
    source_seconds = None
    # A is a separately labelled upstream diagnostic. Never run archive before
    # B: even an untimed prefetch would warm its cold production entry.
    if variant == 'A':
        started = time.monotonic()
        result = run_logged(['nix', 'flake', 'archive', '--json', '--no-update-lock-file',
                             '--option', 'substituters', ''], directory, 'sources')
        source_seconds = time.monotonic() - started
        sources = sorted(set(flake_inputs(json.loads(result.stdout)).values()))
        if any(Path(path) in existing for path in sources):
            raise ValueError('Contaminated runner: a locked source was present before measurement')
        logs += result.stderr
    elif existing:
        raise ValueError('Contaminated runner: source paths were present before cold shell entry')
    started = time.monotonic()
    result = run_logged(['nix', 'develop', '.#postgres-ci', '--command', 'true'], directory, 'shell')
    shell_seconds = time.monotonic() - started
    logs += result.stderr
    split = source_split(result.stderr)
    private_sources = {entry['path'] for entry in split['copies']
                       if entry['route'] == 'private-s3' and entry['path'].endswith('-source')}
    if variant == 'baseline' and private_sources:
        raise ValueError('Baseline already substituted source paths from private S3; prewarm calibration is contaminated')
    if variant == 'B':
        sources = sorted(private_sources)
        other_sources = [entry for entry in split['copies']
                         if entry['path'].endswith('-source') and entry['route'] != 'private-s3']
        if not sources or split['upstream_fetches'] or other_sources:
            raise ValueError('B did not prove cold source entry from private S3; inspect shell.log')
    (directory / 'shell.json').write_text(json.dumps({
        'variant': variant, 'source_seconds': source_seconds,
        'production_shell_seconds': shell_seconds if variant != 'A' else None,
        'source_and_shell_seconds': shell_seconds + (source_seconds or 0),
        'sources': sources, 'source_split': source_split(logs),
        'lock_sha256': hashlib.sha256((ROOT / 'flake.lock').read_bytes()).hexdigest(),
    }, indent=2))


def cache_ready(workspace, directory):
    """An archive hit alone does not establish a comparable warm build."""
    if os.environ.get('PROBE_TARGET_HIT') != 'true':
        raise ValueError('An exact cache hit is required')
    cargo = workspace / '.ci-cargo'
    if Path(os.environ.get('CARGO_HOME', '')).resolve() != cargo.resolve():
        raise ValueError('CARGO_HOME must use the portable workspace cache')

    def populated(path, pattern):
        return any(item.is_file() and item.stat().st_size > 0 for item in path.glob(pattern))

    if not populated(cargo / 'registry/cache', '*/*.crate'):
        raise ValueError('Cargo downloads missing or empty in CARGO_HOME')
    if not populated(cargo / 'registry/index', '**/*'):
        raise ValueError('Cargo registry index missing or empty in CARGO_HOME')
    if not populated(workspace / 'target/debug/deps', '*'):
        raise ValueError('Compiled target dependencies missing or empty')
    directory.mkdir(parents=True, exist_ok=True)
    (directory / 'cache-ready.json').write_text(json.dumps({
        'cargo_home': str(cargo), 'target': str(workspace / 'target'),
        'exact_hit': True, 'downloads_index_and_target_populated': True,
    }, indent=2))


def record(variant, directory):
    directory.mkdir(exist_ok=True, parents=True)
    (directory / 'sample.json').write_text(json.dumps({
        'variant': variant, 'revision': os.environ['GITHUB_SHA'], 'sample': os.environ['PROBE_SAMPLE'],
        'target_key_requested': os.environ['PROBE_TARGET_KEY'],
        'target_key_matched': os.environ.get('PROBE_TARGET_MATCH', ''),
        'target_cache_hit': os.environ.get('PROBE_TARGET_HIT', ''),
        'target_mode': 'exact-restore-no-save', 'image': os.environ.get('PROBE_IMAGE', ''),
        'comparison': 'baseline=ordinary-shell-prewarm; A=upstream-prefetch-diagnostic; B=ordinary-shell-private-S3',
    }, indent=2))


def measurements(jobs):
    sys.path.insert(0, str(ROOT / 'scripts'))
    from ci_timings import interval, timestamp
    names = {'Probe shell', 'Probe container', 'Prepare probe image'}
    # Reusable workflows prefix job names with the caller name.
    selected = [job for job in jobs if job['name'].split(' / ')[-1] in names and interval(job)]
    rows = []
    for job in selected:
        start, end = interval(job)
        rows.append({'name': job['name'], 'conclusion': job['conclusion'],
                     'seconds': (end - start).total_seconds(),
                     'steps': [{'name': step['name'], 'seconds': (interval(step)[1] - interval(step)[0]).total_seconds()}
                               for step in job.get('steps', []) if interval(step)]})
    bounds = [interval(job) for job in selected]
    tests = [timestamp(step.get('started_at')) for job in selected for step in job.get('steps', [])
             if step['name'] == 'Exercise full shard 2' and timestamp(step.get('started_at'))]
    return {'jobs': rows,
            'critical_path_seconds': (max(b[1] for b in bounds) - min(b[0] for b in bounds)).total_seconds() if bounds else None,
            'setup_before_test_seconds': (min(tests) - min(b[0] for b in bounds)).total_seconds() if bounds and tests else None,
            'runner_seconds': sum(row['seconds'] for row in rows)}


def collect(directory):
    sys.path.insert(0, str(ROOT / 'scripts'))
    from ci_timings import Collector
    collector = Collector(os.environ['GITHUB_REPOSITORY'])
    collector.run(int(os.environ['GITHUB_RUN_ID']), int(os.environ['GITHUB_RUN_ATTEMPT']))
    jobs = [job for endpoint, page in collector.raw.items() if '/jobs?' in endpoint for job in page['jobs']]
    result = measurements(jobs) | {
        'variant': os.environ['PROBE_VARIANT'], 'sample': os.environ['PROBE_SAMPLE'],
        'revision': os.environ['GITHUB_SHA'], 'run_id': os.environ['GITHUB_RUN_ID'],
        'attempt': os.environ['GITHUB_RUN_ATTEMPT'],
        'method': 'Setup starts at first preparation job and ends at test step start, including source fetch, shell, image preparation, dependency gap, container init/pull and target restore. Whole duration includes teardown. Initial queue and report job excluded. Source/image publication is a separate operator cost.'}
    directory.mkdir(exist_ok=True, parents=True)
    (directory / 'measurement.json').write_text(json.dumps(result, indent=2))
    (directory / 'raw-api.json').write_text(json.dumps(collector.raw, indent=2))
    print(json.dumps(result, indent=2))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command', choices=('shell', 'record', 'collect', 'cache-ready'))
    parser.add_argument('--variant', choices=('baseline', 'A', 'B', 'C'), default='baseline')
    parser.add_argument('--directory', type=Path, required=True)
    args = parser.parse_args()
    if args.command == 'shell':
        if args.variant == 'C':
            parser.error('shell requires baseline, A or B')
        shell(args.variant, args.directory)
    elif args.command == 'cache-ready':
        cache_ready(ROOT, args.directory)
    elif args.command == 'record':
        record(args.variant, args.directory)
    else:
        collect(args.directory)


if __name__ == '__main__':
    main()
