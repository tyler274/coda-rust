#!/usr/bin/env python3
"""Prove private-cache substitution in a fresh store using reader credentials only."""
import argparse
import json
import os
from pathlib import Path
import tempfile

import nix_cache as cache


def verify(env, build_json):
    url, public = cache.config(env)
    try:
        results = json.loads(Path(build_json).read_text())
        dependency = next(item for item in results
                          if '-coda-servers-deps-' in item['drvPath'])
        paths = list(dependency['outputs'].values())
    except (OSError, ValueError, KeyError, IndexError, TypeError, StopIteration):
        raise cache.CacheError('Expected build JSON with a dependency output') from None
    if not paths or any(not isinstance(p, str) or not cache.STORE_PATH.fullmatch(p) for p in paths):
        raise cache.CacheError('Invalid verification paths')
    # Do not inherit writer credentials or a signing key into the reader process.
    child = {k: v for k, v in env.items() if k not in cache.SECRET_VARS
             and not k.startswith('NIX_CACHE_WRITE_') and k != 'NIX_CONFIG'}
    child.pop('NIX_CACHE_READ_ACCESS_KEY_ID', None)
    child.pop('NIX_CACHE_READ_SECRET_ACCESS_KEY', None)
    reader = dict(child,
                  AWS_ACCESS_KEY_ID=cache.required(env, 'NIX_CACHE_READ_ACCESS_KEY_ID'),
                  AWS_SECRET_ACCESS_KEY=cache.required(env, 'NIX_CACHE_READ_SECRET_ACCESS_KEY'))
    creds = cache.credentials(reader)
    parent = cache.temp_parent(env)
    with tempfile.TemporaryDirectory(prefix='coda-nix-reader-', dir=parent) as scratch:
        directory = Path(scratch)
        cache.private_file(directory / 'credentials', creds)
        child.update(AWS_SHARED_CREDENTIALS_FILE=str(directory / 'credentials'),
                     AWS_EC2_METADATA_DISABLED='true',
                     NIX_USER_CONF_FILES='/dev/null',
                     XDG_CACHE_HOME=str(directory / 'cache'))
        store = f'local?root={directory / "store"}'
        common = ['--option', 'trusted-public-keys', public,
                  '--option', 'require-sigs', 'true', '--option', 'substituters', '']
        print('Fresh reader: copying dependency closure from private cache only; builds disabled.', flush=True)
        cache.run(['nix', 'copy', '--from', url, '--to', store, *common, *paths], env=child)
        cache.run(['nix', 'store', 'verify', '--store', store, '--recursive',
                   '--sigs-needed', '1', *common, *paths], env=child)
        print('Fresh reader: dependency closure fetched and contents/trust verified.', flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build-json', required=True)
    args = parser.parse_args()
    try:
        verify(os.environ, args.build_json)
    except cache.CacheError as error:
        parser.exit(1, f'{error}\n')


if __name__ == '__main__':
    main()
