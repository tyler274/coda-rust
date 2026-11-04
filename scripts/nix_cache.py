#!/usr/bin/env python3
"""Job-scoped native Nix/GCS cache transport (Linux/systemd disposable CI only).

setup --disposable-runner: reader HMAC -> mode-0600 files, temporary daemon
configuration, and GITHUB_ENV for subsequent steps. cleanup must run with always().
publish PATH...: pass realized .drv roots AND selected outputs, or --build-json
from `nix build --no-link --json`. Publish after the protected main job rebuilds
at GITHUB_SHA; never sign store paths/artifacts supplied by a PR job.

Only publish receives writer AWS credentials and NIX_CACHE_SIGNING_KEY. Setup
should receive reader credentials even in the writer job. No credential is a Nix
expression, store input, command argument or GITHUB_ENV value. This is not a
security boundary against other code running as the runner user/root.

Sources: nix.dev/manual/nix/2.34/store/types/s3-binary-cache-store.html,
command-ref/conf-file.html, command-ref/nix-store/query.html, and
command-ref/new-cli/nix3-store-sign.html. Nix 2.34/2.35 uses curl for S3
transfers and AWS CRT for credentials. The endpoint accepts a full HTTPS URL;
connect-timeout and stalled-download-timeout apply to S3 transfers. Retries
and a large closure can still take longer; bound publication with a CI timeout.
The native S3 destination is opened by the client with writer credentials;
the local-store daemon retains its reader credentials.
"""

import argparse
import base64
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile


ROOT = Path(__file__).resolve().parents[1]
DAEMON_DIR = Path('/run/coda-nix-cache')
DROPIN = Path('/run/systemd/system/nix-daemon.service.d/90-coda-nix-cache.conf')
SECRET_VARS = ('AWS_ACCESS_KEY_ID', 'AWS_SECRET_ACCESS_KEY', 'AWS_SESSION_TOKEN',
               'NIX_CACHE_SIGNING_KEY')
STORE_PATH = re.compile(r'/nix/store/[0-9a-df-np-sv-z]{32}-[A-Za-z0-9+._?=-]+')


class CacheError(Exception):
    pass


def run(args, *, env=None, data=None):
    """Never echo subprocess output: SDK/parser errors may contain credentials."""
    clean = dict(os.environ if env is None else env)
    for name in SECRET_VARS:
        clean.pop(name, None)
    try:
        result = subprocess.run(args, input=data, env=clean, text=True,
                                capture_output=True, check=False)
    except OSError:
        raise CacheError('Could not execute cache helper command') from None
    if result.returncode:
        raise CacheError(f'Cache helper command failed ({args[0]}, exit {result.returncode}); output withheld')
    return result.stdout.strip()


def required(env, name):
    value = env.get(name, '')
    if not value or any(c.isspace() for c in value) or '\x00' in value:
        raise CacheError(f'{name} must be a nonempty single-line value')
    return value


def config(env):
    bucket = required(env, 'NIX_CACHE_BUCKET')
    if not re.fullmatch(r'[a-z0-9][a-z0-9-]{1,61}[a-z0-9]', bucket):
        raise CacheError('NIX_CACHE_BUCKET must be a DNS-safe bucket name')
    key = required(env, 'NIX_CACHE_PUBLIC_KEY')
    try:
        name, encoded = key.split(':')
        if not re.fullmatch(r'[A-Za-z0-9._-]+', name) or len(base64.b64decode(encoded, validate=True)) != 32:
            raise ValueError
    except (ValueError, TypeError):
        raise CacheError('NIX_CACHE_PUBLIC_KEY must be a Nix Ed25519 public key') from None
    return f's3://{bucket}?endpoint=https://storage.googleapis.com&scheme=https&profile=default&multipart-upload=false', key


def credentials(env):
    access = required(env, 'AWS_ACCESS_KEY_ID')
    secret = required(env, 'AWS_SECRET_ACCESS_KEY')
    if not re.fullmatch(r'[A-Za-z0-9]+', access) or not re.fullmatch(r'[A-Za-z0-9/+=]+', secret):
        raise CacheError('Expected GCS HMAC credentials')
    if env.get('AWS_SESSION_TOKEN'):
        raise CacheError('GCS HMAC credentials do not use AWS_SESSION_TOKEN')
    return f'[default]\naws_access_key_id = {access}\naws_secret_access_key = {secret}\n'


def ci_only(env):
    if env.get('GITHUB_ACTIONS') != 'true':
        raise CacheError('This helper only supports disposable GitHub Actions runners')


def temp_parent(env):
    original = Path(required(env, 'RUNNER_TEMP'))
    parent = original.resolve()
    workspace = Path(env.get('GITHUB_WORKSPACE', ROOT)).resolve()
    if not original.is_absolute() or not parent.is_dir() or any(
            parent.is_relative_to(p) for p in (ROOT, workspace, Path('/nix/store'))):
        raise CacheError('RUNNER_TEMP must exist outside the checkout and Nix store')
    if not re.fullmatch(r'/[A-Za-z0-9/_.-]+', str(parent)):
        raise CacheError('RUNNER_TEMP contains unsupported configuration characters')
    return parent


def private_file(path, contents):
    with open(path, 'x', opener=lambda p, flags: os.open(p, flags, 0o600)) as out:
        out.write(contents)


def nix_config(url, key):
    return (f'extra-substituters = https://cache.nixos.org {url}\n'
            f'extra-trusted-public-keys = {key}\n'
            'require-sigs = true\nfallback = true\nconnect-timeout = 5\n'
            'stalled-download-timeout = 30\ndownload-attempts = 2\n')


def daemon_remove():
    # Only our dedicated runtime files, never /etc/nix or a developer config.
    try:
        run(['sudo', '-n', 'rm', '-f', str(DROPIN)])
        run(['sudo', '-n', 'systemctl', 'daemon-reload'])
        run(['sudo', '-n', 'systemctl', 'restart', 'nix-daemon.service'])
    finally:
        run(['sudo', '-n', 'rm', '-rf', str(DAEMON_DIR)])


def setup(env, disposable_runner):
    ci_only(env)
    if not disposable_runner:
        raise CacheError('setup requires --disposable-runner; it restarts nix-daemon')
    if env.get('NIX_CONFIG'):
        raise CacheError('Unset NIX_CONFIG so it cannot override signature/fallback settings')
    url, key = config(env)
    creds = credentials(env)
    parent = temp_parent(env)
    github_env = Path(required(env, 'GITHUB_ENV'))
    if DAEMON_DIR.exists() or DROPIN.exists():
        raise CacheError('Cache daemon configuration already exists; do not share runners')
    state = Path(tempfile.mkdtemp(prefix='coda-nix-cache-', dir=parent))
    installed = False
    try:
        private_file(state / 'credentials', creds)
        private_file(state / 'nix.conf', nix_config(url, key))
        private_file(state / 'daemon.conf', '[Service]\n'
                     f'Environment="AWS_SHARED_CREDENTIALS_FILE={DAEMON_DIR}/credentials"\n'
                     'Environment="AWS_EC2_METADATA_DISABLED=true"\n'
                     f'Environment="NIX_USER_CONF_FILES={DAEMON_DIR}/nix.conf"\n')
        run(['sudo', '-n', 'install', '-d', '-m', '0700', str(DAEMON_DIR)])
        installed = True
        for name in ('credentials', 'nix.conf'):
            run(['sudo', '-n', 'install', '-m', '0600', str(state / name), str(DAEMON_DIR / name)])
        run(['sudo', '-n', 'install', '-d', '-m', '0755', str(DROPIN.parent)])
        run(['sudo', '-n', 'install', '-m', '0600', str(state / 'daemon.conf'), str(DROPIN)])
        run(['sudo', '-n', 'systemctl', 'daemon-reload'])
        run(['sudo', '-n', 'systemctl', 'restart', 'nix-daemon.service'])
        # NIX_USER_CONF_FILES is loaded in reverse order: prepend our overrides.
        previous = env.get('NIX_USER_CONF_FILES')
        if previous is None:
            home = env.get('XDG_CONFIG_HOME', str(Path(env['HOME']) / '.config'))
            dirs = env.get('XDG_CONFIG_DIRS', '/etc/xdg').split(':')
            previous = ':'.join(str(Path(d) / 'nix/nix.conf') for d in [home, *dirs])
        if '\n' in previous or '\r' in previous:
            raise CacheError('Invalid NIX_USER_CONF_FILES')
        with github_env.open('a') as out:
            out.write(f'NIX_USER_CONF_FILES={state}/nix.conf:{previous}\n'
                      f'AWS_SHARED_CREDENTIALS_FILE={state}/credentials\n'
                      'AWS_EC2_METADATA_DISABLED=true\n'
                      f'NIX_CACHE_STATE_DIR={state}\n')
    except Exception:
        try:
            if installed:
                daemon_remove()
        finally:
            shutil.rmtree(state)
        raise
    print('Private Nix cache configured; credentials are job-scoped. Run cleanup with always().')


def cleanup(env):
    ci_only(env)
    state = Path(required(env, 'NIX_CACHE_STATE_DIR'))
    parent = temp_parent(env)
    if state.is_symlink() or state.parent.resolve() != parent or not state.name.startswith('coda-nix-cache-'):
        raise CacheError('Refusing to clean an unrecognized state directory')
    try:
        daemon_remove()
    finally:
        shutil.rmtree(state)
    print('Removed job-scoped Nix cache configuration.')


def selected_paths(paths, build_json):
    roots = list(paths)
    if build_json:
        try:
            for result in json.loads(Path(build_json).read_text()):
                roots.append(result['drvPath'])
                roots.extend(result['outputs'].values())
        except (KeyError, TypeError, ValueError):
            raise CacheError('Expected nix build --json results') from None
    if not roots or any(not isinstance(p, str) or not STORE_PATH.fullmatch(p) for p in roots):
        raise CacheError('Publish requires explicit /nix/store paths, not expressions or directories')
    if not any(p.endswith('.drv') for p in roots):
        raise CacheError('Include realized derivation roots to cache build-time dependencies')
    return sorted(set(roots))


def publish(env, paths, build_json=None):
    ci_only(env)
    if env.get('GITHUB_EVENT_NAME') != 'push' or env.get('GITHUB_REF') != 'refs/heads/main':
        raise CacheError('Publishing is restricted to main push jobs in a protected environment')
    sha = required(env, 'GITHUB_SHA')
    if run(['git', 'rev-parse', 'HEAD'], env=env) != sha:
        raise CacheError('Publish checkout does not match GITHUB_SHA')
    url, public = config(env)
    roots = selected_paths(paths, build_json)
    creds = credentials(env)
    secret = required(env, 'NIX_CACHE_SIGNING_KEY')
    parent = temp_parent(env)
    # Query includes build-time-only dependencies and their existing outputs,
    # unlike `nix path-info -r` on outputs, which only covers runtime references.
    closure = run(['nix-store', '--query', '--requisites', '--include-outputs', *roots], env=env).splitlines()
    if not closure or any(not STORE_PATH.fullmatch(p) for p in closure):
        raise CacheError('Nix returned an invalid or empty closure')
    with tempfile.TemporaryDirectory(prefix='coda-nix-publish-', dir=parent) as tmp:
        directory = Path(tmp)
        private_file(directory / 'credentials', creds)
        private_file(directory / 'signing-key', secret + '\n')
        child = dict(env, AWS_SHARED_CREDENTIALS_FILE=str(directory / 'credentials'),
                     AWS_EC2_METADATA_DISABLED='true')
        derived = run(['nix', 'key', 'convert-secret-to-public'], env=child, data=secret)
        if derived != public:
            raise CacheError('Signing key does not match NIX_CACHE_PUBLIC_KEY')
        data = '\n'.join(sorted(set(closure))) + '\n'
        run(['nix', 'store', 'sign', '--key-file', str(directory / 'signing-key'), '--stdin'], env=child, data=data)
        run(['nix', 'copy', '--to', url, '--stdin'], env=child, data=data)
    print(f'Published selected signed closure ({len(set(closure))} paths).')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    commands.add_parser('setup').add_argument('--disposable-runner', action='store_true')
    commands.add_parser('cleanup')
    pub = commands.add_parser('publish')
    pub.add_argument('paths', nargs='*')
    pub.add_argument('--build-json')
    args = parser.parse_args()
    try:
        if args.command == 'setup':
            setup(os.environ, args.disposable_runner)
        elif args.command == 'cleanup':
            cleanup(os.environ)
        else:
            publish(os.environ, args.paths, args.build_json)
    except CacheError as error:
        # CacheError messages are code-owned and never contain secret values.
        print(str(error), file=sys.stderr)
        return 1
    except OSError:
        # OS errors can contain file names; don't echo arbitrary secret inputs.
        print('Nix cache operation failed; validate runner permissions and service health.', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
