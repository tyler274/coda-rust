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
from contextlib import contextmanager
from enum import Enum
import errno
import json
import os
from pathlib import Path
import re
import selectors
import shutil
import subprocess
import sys
import tempfile
import time

from nix_cache_local import authorize
from nix_cache_copy import copy_closure


ROOT = Path(__file__).resolve().parents[2]
DAEMON_DIR = Path('/run/coda-nix-cache')
DROPIN = Path('/run/systemd/system/nix-daemon.service.d/90-coda-nix-cache.conf')
SECRET_VARS = ('AWS_ACCESS_KEY_ID', 'AWS_SECRET_ACCESS_KEY', 'AWS_SESSION_TOKEN',
               'NIX_CACHE_SIGNING_KEY')
STORE_PATH = re.compile(r'/nix/store/[0-9a-df-np-sv-z]{32}-[A-Za-z0-9+._?=-]+')


class CacheError(Exception):
    def __init__(self, message, category=None):
        super().__init__(message)
        self.category = category


class Stage(Enum):
    CHECK = 'check'
    VERSION = 'version'
    CLOSURE = 'closure'
    KEY = 'key'
    SIGN = 'sign'
    COPY = 'copy'
    CLEANUP = 'cleanup'
    HELPER = 'helper'


class Failure(Enum):
    PERMISSION = 'permission-or-403'
    NETWORK = 'network'
    DISK = 'disk'
    MISSING_PATH = 'missing-path'
    UNKNOWN = 'unknown'


HEARTBEAT_SECONDS = 15
OUTPUT_LIMIT = 16 * 1024 * 1024
DIAGNOSTIC_LIMIT = 64 * 1024
FAILURE_HINTS = (
    (Failure.PERMISSION, ('http error 403', 'http status 403', 'http 403', '403 forbidden', 'accessdenied', 'access denied', 'permission denied')),
    (Failure.DISK, ('no space left', 'disk quota exceeded', 'read-only file system')),
    (Failure.NETWORK, ('could not resolve', 'connection refused', 'connection reset',
                       'timed out', 'connection timeout', 'network is unreachable',
                       'tls handshake', 'ssl certificate problem')),
    (Failure.MISSING_PATH, ('no such file', 'does not exist', 'not a valid path', 'path is not valid')),
)


def classify_failure(text):
    # These are hints, not a diagnosis. Never return a fragment of child output.
    sample = text[:DIAGNOSTIC_LIMIT].lower()
    return next((kind for kind, hints in FAILURE_HINTS if any(h in sample for h in hints)),
                Failure.UNKNOWN)


def os_failure(error):
    if error.errno in (errno.EACCES, errno.EPERM):
        return Failure.PERMISSION
    if error.errno in (errno.ENOSPC, errno.EDQUOT, errno.EROFS):
        return Failure.DISK
    if error.errno == errno.ENOENT:
        return Failure.MISSING_PATH
    return Failure.UNKNOWN


def diagnostic(stage, status, started, failure=None):
    # Enum values and numeric elapsed time are the entire logging vocabulary.
    message = f'Nix cache stage={stage.value} status={status} elapsed_seconds={int(time.monotonic() - started)}'
    if failure is not None:
        message += f' classification={failure.value}'
    if status == 'heartbeat':
        message += ' (still running; not an upload count)'
    print(message, flush=True)


@contextmanager
def stage(label):
    started = time.monotonic()
    diagnostic(label, 'started', started)
    try:
        yield
    except Exception as error:
        category = os_failure(error) if isinstance(error, OSError) else getattr(error, 'category', None)
        diagnostic(label, 'failed', started, category or Failure.UNKNOWN)
        raise
    else:
        diagnostic(label, 'completed', started)


def capture_command(args, *, env, input, label):
    """Drain both pipes with bounded memory; emit only code-owned heartbeats.

    Input is a private, immediately unlinked temporary file, avoiding a pipe
    deadlock while the child writes output. Only stdout needed by callers is
    retained. Stderr is classified in bounded overlapping chunks, then dropped.
    """
    started = time.monotonic()
    next_heartbeat = started + HEARTBEAT_SECONDS
    output = bytearray()
    overflow = False
    category = Failure.UNKNOWN
    priority = [kind for kind, _ in FAILURE_HINTS] + [Failure.UNKNOWN]
    retain_stdout = label not in (Stage.SIGN, Stage.COPY)
    previous = {}
    with tempfile.TemporaryFile() as source:
        if input is not None:
            source.write(input.encode())
            source.seek(0)
        with subprocess.Popen(args, stdin=source, stdout=subprocess.PIPE,
                              stderr=subprocess.PIPE, env=env) as process:
            try:
                with selectors.DefaultSelector() as selector:
                    for stream in (process.stdout, process.stderr):
                        selector.register(stream, selectors.EVENT_READ)
                    while selector.get_map() or process.poll() is None:
                        for key, _ in selector.select(max(0, next_heartbeat - time.monotonic())):
                            chunk = os.read(key.fd, DIAGNOSTIC_LIMIT // 2)
                            if not chunk:
                                selector.unregister(key.fileobj)
                                continue
                            sample = previous.get(key.fd, b'') + chunk
                            found = classify_failure(sample.decode('utf-8', errors='replace'))
                            if priority.index(found) < priority.index(category):
                                category = found
                            previous[key.fd] = chunk[-256:]
                            if key.fileobj is process.stdout and retain_stdout:
                                room = OUTPUT_LIMIT - len(output)
                                output.extend(chunk[:room])
                                overflow |= len(chunk) > room
                        now = time.monotonic()
                        if now >= next_heartbeat:
                            diagnostic(label, 'heartbeat', started)
                            next_heartbeat = now + HEARTBEAT_SECONDS
                    code = process.wait()
            except BaseException:
                process.kill()
                process.wait()
                raise
    if overflow and code == 0:
        raise CacheError('Cache helper output exceeded safe capture limit; output withheld')
    result = subprocess.CompletedProcess(args, code, output.decode('utf-8', errors='replace'), '')
    result.failure_category = category
    return result


def run(args, *, env=None, data=None, label=Stage.HELPER):
    """Never echo subprocess output, command arguments, paths, or environment."""
    clean = dict(os.environ if env is None else env)
    for name in SECRET_VARS:
        clean.pop(name, None)
    try:
        result = capture_command(args, input=data, env=clean, label=label)
    except OSError as error:
        raise CacheError(f'Cache stage={label.value} could not execute; classification={os_failure(error).value}; output withheld', os_failure(error)) from None
    if result.returncode:
        category = getattr(result, 'failure_category', classify_failure(result.stderr or ''))
        raise CacheError(f'Cache stage={label.value} failed; exit_code={result.returncode}; classification={category.value}; output withheld', category)
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


def publish(env, paths, build_json=None, *, local_sha=None, acknowledged=False):
    started = time.monotonic()
    try:
        count = publish_closure(env, paths, build_json, local_sha=local_sha, acknowledged=acknowledged)
    except Exception as error:
        category = os_failure(error) if isinstance(error, OSError) else getattr(error, 'category', None)
        print(f'Nix cache summary status=failed classification={(category or Failure.UNKNOWN).value} '
              f'elapsed_seconds={int(time.monotonic() - started)}', flush=True)
        raise
    print(f'Nix cache summary status=completed closure_paths={count} '
          f'elapsed_seconds={int(time.monotonic() - started)} '
          '(selected closure size; not uploaded path count)', flush=True)


def publish_closure(env, paths, build_json, *, local_sha=None, acknowledged=False):
    with stage(Stage.CHECK):
        ci_only(env)
        if local_sha is not None:
            sha = authorize(env, local_sha, acknowledged, run, CacheError)
        else:
            if env.get('ACT') == 'true':
                raise CacheError('act requires the explicit publish-local command')
            if env.get('GITHUB_EVENT_NAME') != 'push' or env.get('GITHUB_REF') != 'refs/heads/main':
                raise CacheError('Publishing is restricted to main push jobs in a protected environment')
            sha = required(env, 'GITHUB_SHA')
            if run(['git', 'rev-parse', 'HEAD'], env=env, label=Stage.CHECK) != sha:
                raise CacheError('Publish checkout does not match GITHUB_SHA')
        if re.fullmatch(r'[0-9a-f]{40}', sha):
            print(f'Nix cache revision={sha}', flush=True)
        url, public = config(env)
        roots = selected_paths(paths, build_json)
        creds = credentials(env)
        secret = required(env, 'NIX_CACHE_SIGNING_KEY')
        parent = temp_parent(env)
    with stage(Stage.VERSION):
        version = run(['nix', '--version'], env=env, label=Stage.VERSION)
        matched = re.fullmatch(r'nix \(Nix\) ([0-9]+\.[0-9]+\.[0-9]+)', version)
        print(f'Nix cache nix_version={matched[1] if matched else "unrecognized"}', flush=True)
    with stage(Stage.CLOSURE):
        # Include build-time dependencies and existing outputs, not just runtime references.
        closure = run(['nix-store', '--query', '--requisites', '--include-outputs', *roots],
                      env=env, label=Stage.CLOSURE).splitlines()
        if not closure or any(not STORE_PATH.fullmatch(p) for p in closure):
            raise CacheError('Nix returned an invalid or empty closure')
        data = '\n'.join(sorted(set(closure))) + '\n'
    temporary = None
    try:
        with stage(Stage.KEY):
            temporary = tempfile.TemporaryDirectory(prefix='coda-nix-publish-', dir=parent)
            directory = Path(temporary.name)
            private_file(directory / 'credentials', creds)
            private_file(directory / 'signing-key', secret + '\n')
            child = dict(env, AWS_SHARED_CREDENTIALS_FILE=str(directory / 'credentials'),
                         AWS_EC2_METADATA_DISABLED='true')
            derived = run(['nix', 'key', 'convert-secret-to-public'], env=child,
                          data=secret, label=Stage.KEY)
            if derived != public:
                raise CacheError('Signing key does not match NIX_CACHE_PUBLIC_KEY')
        with stage(Stage.SIGN):
            run(['nix', 'store', 'sign', '--key-file', str(directory / 'signing-key'), '--stdin'],
                env=child, data=data, label=Stage.SIGN)
        with stage(Stage.COPY):
            print(f'Nix cache roots={len(roots)} closure_paths={len(set(closure))}', flush=True)
            copy_closure(run, child, url, data, Stage.COPY, CacheError)
    finally:
        if temporary is not None:
            with stage(Stage.CLEANUP):
                temporary.cleanup()
    return len(set(closure))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    commands.add_parser('setup').add_argument('--disposable-runner', action='store_true')
    commands.add_parser('cleanup')
    pub = commands.add_parser('publish')
    pub.add_argument('paths', nargs='*')
    pub.add_argument('--build-json')
    local = commands.add_parser('publish-local')
    local.add_argument('--expected-sha', required=True)
    local.add_argument('--acknowledge-local-write', action='store_true')
    local.add_argument('--build-json', required=True)
    args = parser.parse_args()
    try:
        if args.command == 'setup':
            setup(os.environ, args.disposable_runner)
        elif args.command == 'cleanup':
            cleanup(os.environ)
        elif args.command == 'publish-local':
            publish(os.environ, [], args.build_json, local_sha=args.expected_sha,
                    acknowledged=args.acknowledge_local_write)
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
