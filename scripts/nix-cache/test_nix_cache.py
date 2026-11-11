"""No Nix daemon, cloud credentials, network, or sudo required."""
import base64
import contextlib
import io
import json
import os
from pathlib import Path
import stat
import subprocess
import sys
import tempfile
import time
import unittest
from unittest.mock import patch

import nix_cache as cache


DRV = '/nix/store/' + 'a' * 32 + '-app.drv'
OUT = '/nix/store/' + 'b' * 32 + '-app'
BUILD_DEP = '/nix/store/' + 'c' * 32 + '-compiler'
PUBLIC = 'test-1:' + base64.b64encode(b'p' * 32).decode()
SECRET = 'test-1:' + base64.b64encode(b's' * 64).decode()


class CacheTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)
        self.github_env = self.directory / 'github-env'
        self.github_env.touch()
        self.env = {
            'GITHUB_ACTIONS': 'true', 'GITHUB_EVENT_NAME': 'push',
            'GITHUB_REF': 'refs/heads/main', 'GITHUB_SHA': 'abc123',
            'HOME': str(self.directory / 'home'),
            'RUNNER_TEMP': str(self.directory), 'GITHUB_ENV': str(self.github_env),
            'GITHUB_WORKSPACE': str(cache.ROOT), 'NIX_CACHE_BUCKET': 'private-cache',
            'NIX_CACHE_PUBLIC_KEY': PUBLIC, 'AWS_ACCESS_KEY_ID': 'GOOGREADER',
            'AWS_SECRET_ACCESS_KEY': 'privateAccessSecret/+=',
            'NIX_CACHE_SIGNING_KEY': SECRET,
        }
        self.calls = []
        self.daemon = self.directory / 'daemon'
        self.dropin = self.directory / 'systemd' / '90-cache.conf'
        self.addCleanup(patch.stopall)
        patch.object(cache, 'DAEMON_DIR', self.daemon).start()
        patch.object(cache, 'DROPIN', self.dropin).start()
        self.runner = patch.object(cache, 'capture_command', side_effect=self.fake_run).start()
        self.output = io.StringIO()
        self.enterContext(contextlib.redirect_stdout(self.output))

    def fake_run(self, args, **kwargs):
        self.calls.append((args, kwargs))
        self.assertIsInstance(kwargs['label'], cache.Stage)
        self.assertFalse(any(secret in ' '.join(args) for secret in (SECRET, 'privateAccessSecret/+=')))
        for name in cache.SECRET_VARS:
            self.assertNotIn(name, kwargs['env'])
        if args[:3] == ['git', 'rev-parse', 'HEAD']:
            value = self.env['GITHUB_SHA']
        elif args[0] == 'nix-store':
            value = '\n'.join([DRV, OUT, BUILD_DEP])
        elif args[:3] == ['nix', 'path-info', '--json']:
            value = json.dumps({p: {'narHash': p, 'references': []} for p in (DRV, OUT, BUILD_DEP)})
        elif args[:3] == ['nix', 'key', 'convert-secret-to-public']:
            self.assertEqual(kwargs['input'], SECRET)
            value = PUBLIC
        elif args[:3] == ['nix', 'store', 'sign']:
            key = Path(args[args.index('--key-file') + 1])
            self.assertEqual(key.read_text(), SECRET + '\n')
            self.assertEqual(stat.S_IMODE(key.stat().st_mode), 0o600)
            self.assertEqual(stat.S_IMODE(key.parent.stat().st_mode), 0o700)
            self.assertFalse(key.is_relative_to(cache.ROOT))
            value = ''
        elif args[:2] == ['nix', 'copy']:
            credential = Path(kwargs['env']['AWS_SHARED_CREDENTIALS_FILE'])
            self.assertIn('privateAccessSecret/+=', credential.read_text())
            self.assertEqual(stat.S_IMODE(credential.stat().st_mode), 0o600)
            value = ''
        else:
            value = ''
        return subprocess.CompletedProcess(args, 0, stdout=value, stderr='')

    def test_setup_installs_private_daemon_credentials_and_scoped_config(self):
        cache.setup(self.env, True)
        values = dict(line.split('=', 1) for line in self.github_env.read_text().splitlines())
        state = Path(values['NIX_CACHE_STATE_DIR'])
        self.assertEqual(stat.S_IMODE(state.stat().st_mode), 0o700)
        self.assertEqual(stat.S_IMODE((state / 'credentials').stat().st_mode), 0o600)
        settings = (state / 'nix.conf').read_text()
        self.assertIn('extra-substituters = https://cache.nixos.org s3://private-cache?endpoint=https://storage.googleapis.com&scheme=https', settings)
        # Upstream (Priority 40) must win for public paths; private fills gaps.
        self.assertRegex(settings, r'extra-substituters = \S+ s3://private-cache\?\S*&priority=50\n')
        self.assertIn('extra-trusted-public-keys = ' + PUBLIC, settings)
        self.assertIn('require-sigs = true', settings)
        self.assertIn('fallback = true', settings)
        self.assertIn('connect-timeout = 5', settings)
        self.assertNotIn('trusted=true', settings)
        self.assertNotIn('secret-key', settings)
        daemon_config = (state / 'daemon.conf').read_text()
        self.assertIn(f'AWS_SHARED_CREDENTIALS_FILE={self.daemon}/credentials', daemon_config)
        self.assertIn(f'NIX_USER_CONF_FILES={self.daemon}/nix.conf', daemon_config)
        self.assertIn(['sudo', '-n', 'install', '-m', '0600', str(state / 'credentials'), str(self.daemon / 'credentials')], [c[0] for c in self.calls])
        self.assertIn(['sudo', '-n', 'systemctl', 'restart', 'nix-daemon.service'], [c[0] for c in self.calls])
        for secret in (SECRET, 'privateAccessSecret/+='):
            self.assertNotIn(secret, self.github_env.read_text())
            self.assertNotIn(secret, self.output.getvalue())
        self.assertNotIn('NIX_CACHE_SIGNING_KEY', daemon_config)

    def test_existing_user_configuration_preserved_after_override(self):
        self.env['NIX_USER_CONF_FILES'] = '/tmp/first.conf:/tmp/second.conf'
        cache.setup(self.env, True)
        self.assertIn('/nix.conf:/tmp/first.conf:/tmp/second.conf', self.github_env.read_text())

    def test_setup_refuses_nix_config_override(self):
        self.env['NIX_CONFIG'] = 'require-sigs = false'
        with self.assertRaises(cache.CacheError):
            cache.setup(self.env, True)
        self.assertFalse(self.calls)

    def test_setup_requires_explicit_disposable_runner(self):
        with self.assertRaises(cache.CacheError):
            cache.setup(self.env, False)
        self.env['GITHUB_ACTIONS'] = 'false'
        with self.assertRaises(cache.CacheError):
            cache.setup(self.env, True)
        self.assertFalse(self.calls)

    def test_no_shared_runner_overwrite(self):
        self.daemon.mkdir()
        with self.assertRaises(cache.CacheError):
            cache.setup(self.env, True)
        self.assertFalse(self.calls)

    def test_missing_credential_fails_before_mutation(self):
        del self.env['AWS_SECRET_ACCESS_KEY']
        with self.assertRaises(cache.CacheError):
            cache.setup(self.env, True)
        self.assertFalse(self.calls)

    def test_rejects_bucket_and_key_injection(self):
        for bucket in ('bad\nrequire-sigs=false', 'abc?trusted=true', 'a/b', 'abc&profile=other'):
            with self.subTest(bucket=bucket), self.assertRaises(cache.CacheError):
                cache.config(dict(self.env, NIX_CACHE_BUCKET=bucket))
        for key in ('bad', 'test:!!!', 'test:' + base64.b64encode(b'x').decode(), PUBLIC + '\nrequire-sigs=false'):
            with self.subTest(key=key), self.assertRaises(cache.CacheError):
                cache.config(dict(self.env, NIX_CACHE_PUBLIC_KEY=key))

    def test_rejects_credential_injection_and_sessions(self):
        for update in ({'AWS_ACCESS_KEY_ID': 'a\n[attacker]'},
                       {'AWS_SECRET_ACCESS_KEY': 'a\nfoo=bar'},
                       {'AWS_SESSION_TOKEN': 'temporary'}):
            with self.subTest(update=update), self.assertRaises(cache.CacheError):
                cache.credentials(dict(self.env, **update))

    def test_rejects_secrets_inside_repository_or_store(self):
        for directory in (str(cache.ROOT), '/nix/store'):
            with self.subTest(directory=directory), self.assertRaises(cache.CacheError):
                cache.temp_parent(dict(self.env, RUNNER_TEMP=directory))

    def test_setup_failure_rolls_back_runtime_override(self):
        original = self.fake_run
        failed = False
        def fail_once(args, **kwargs):
            nonlocal failed
            if args[:4] == ['sudo', '-n', 'systemctl', 'restart'] and not failed:
                failed = True
                return subprocess.CompletedProcess(args, 1, '', 'secret diagnostic')
            return original(args, **kwargs)
        self.runner.side_effect = fail_once
        with self.assertRaises(cache.CacheError):
            cache.setup(self.env, True)
        self.assertFalse(list(self.directory.glob('coda-nix-cache-*')))
        self.assertEqual(self.github_env.read_text(), '')
        self.assertIn(['sudo', '-n', 'rm', '-f', str(self.dropin)], [c[0] for c in self.calls])

    def test_publish_signs_build_dependency_closure_before_copy(self):
        cache.publish(self.env, [DRV, OUT])
        args = [c[0] for c in self.calls]
        self.assertIn(['nix-store', '--query', '--requisites', '--include-outputs', DRV, OUT], args)
        sign = next(i for i, a in enumerate(args) if a[:3] == ['nix', 'store', 'sign'])
        copy = next(i for i, a in enumerate(args) if a[:2] == ['nix', 'copy'])
        self.assertLess(sign, copy)
        self.assertEqual(self.calls[sign][1]['input'], self.calls[copy][1]['input'])
        self.assertIn(BUILD_DEP, self.calls[copy][1]['input'])
        self.assertIn('--stdin', args[copy])
        self.assertNotIn('--all', args[copy])
        self.assertFalse(list(self.directory.glob('coda-nix-publish-*')))

    def test_publish_stages_and_summary_are_sanitized(self):
        cache.publish(self.env, [DRV, OUT])
        log = self.output.getvalue()
        for label in ('check', 'version', 'closure', 'key', 'sign', 'copy', 'cleanup'):
            self.assertIn(f'stage={label} status=started', log)
            self.assertIn(f'stage={label} status=completed', log)
        self.assertIn('summary status=completed closure_paths=3', log)
        self.assertIn('not uploaded path count', log)
        for value in (*self.env.values(), DRV, OUT, BUILD_DEP, 's3://', 'https://'):
            self.assertNotIn(value, log)

    def test_copy_classification_and_cleanup_precede_failed_summary(self):
        original = self.fake_run
        def fail(args, **kwargs):
            if args[:2] == ['nix', 'copy']:
                return subprocess.CompletedProcess(args, 1, '', 'HTTP error 403 ' + SECRET)
            return original(args, **kwargs)
        self.runner.side_effect = fail
        with self.assertRaises(cache.CacheError):
            cache.publish(self.env, [DRV, OUT])
        log = self.output.getvalue()
        self.assertIn('stage=copy status=failed', log)
        self.assertIn('classification=permission-or-403', log)
        self.assertLess(log.index('stage=cleanup status=completed'), log.index('summary status=failed'))
        self.assertNotIn('summary status=completed', log)
        self.assertNotIn(SECRET, log)

    def test_build_json_selects_drv_and_outputs(self):
        path = self.directory / 'build.json'
        path.write_text(json.dumps([{'drvPath': DRV, 'outputs': {'out': OUT}}]))
        cache.publish(self.env, [], str(path))
        self.assertIn(['nix-store', '--query', '--requisites', '--include-outputs', DRV, OUT], [c[0] for c in self.calls])

    def test_build_json_includes_flake_input_sources(self):
        source = '/nix/store/' + 'c' * 32 + '-source'
        path = self.directory / 'build.json'
        path.write_text(json.dumps([{'drvPath': DRV, 'outputs': {'out': OUT}},
                                    {'flakeInputs': {'nixpkgs': source}}]))
        self.assertEqual(cache.selected_paths([], str(path)), sorted([DRV, OUT, source]))
        for bad in ({'nixpkgs': '.#x'}, {'nixpkgs': '/tmp/source'}):
            path.write_text(json.dumps([{'drvPath': DRV, 'outputs': {}}, {'flakeInputs': bad}]))
            with self.subTest(bad=bad), self.assertRaises(cache.CacheError):
                cache.selected_paths([], str(path))

    def test_runtime_only_and_expression_roots_rejected(self):
        for paths in ([], [OUT], ['.#package'], [DRV, '/tmp/secret'], [DRV, '--all']):
            with self.subTest(paths=paths), self.assertRaises(cache.CacheError):
                cache.selected_paths(paths, None)

    def test_pr_and_non_main_publication_refused(self):
        for update in ({'GITHUB_EVENT_NAME': 'pull_request'}, {'GITHUB_REF': 'refs/heads/feature'}):
            with self.subTest(update=update), self.assertRaises(cache.CacheError):
                cache.publish(dict(self.env, **update), [DRV, OUT])
        self.assertFalse(self.calls)

    def test_act_cannot_use_hosted_publish_even_with_main_metadata(self):
        with self.assertRaisesRegex(cache.CacheError, 'publish-local'):
            cache.publish(dict(self.env, ACT='true'), [DRV, OUT])
        self.assertFalse(self.calls)

    def test_local_publish_reuses_transport_after_explicit_authorization(self):
        sha = 'a' * 40
        self.env.update(ACT='true', GITHUB_SHA=sha, GITHUB_EVENT_NAME='workflow_dispatch')
        cache.publish(self.env, [DRV, OUT], local_sha=sha, acknowledged=True)
        self.assertTrue(any(args[:2] == ['nix', 'copy'] for args, _ in self.calls))

    def test_wrong_sha_refused(self):
        self.runner.return_value = subprocess.CompletedProcess([], 0, 'different', '')
        self.runner.side_effect = None
        with self.assertRaises(cache.CacheError):
            cache.publish(self.env, [DRV, OUT])
        self.assertEqual(self.runner.call_count, 1)

    def test_wrong_signing_key_prevents_sign_and_copy(self):
        original = self.fake_run
        def mismatch(args, **kwargs):
            if args[:3] == ['nix', 'key', 'convert-secret-to-public']:
                return subprocess.CompletedProcess(args, 0, 'other-key', '')
            return original(args, **kwargs)
        self.runner.side_effect = mismatch
        with self.assertRaises(cache.CacheError):
            cache.publish(self.env, [DRV, OUT])
        self.assertFalse(any(c[0][:2] == ['nix', 'copy'] for c in self.calls))
        self.assertFalse(list(self.directory.glob('coda-nix-publish-*')))

    def test_copy_failure_cleans_private_key_and_withholds_output(self):
        original = self.fake_run
        def fail(args, **kwargs):
            if args[:2] == ['nix', 'copy']:
                return subprocess.CompletedProcess(args, 1, SECRET, 'privateAccessSecret/+=')
            return original(args, **kwargs)
        self.runner.side_effect = fail
        with self.assertRaises(cache.CacheError) as error:
            cache.publish(self.env, [DRV, OUT])
        self.assertNotIn(SECRET, str(error.exception))
        self.assertFalse(list(self.directory.glob('coda-nix-publish-*')))
        self.assertNotIn('privateAccessSecret/+=', self.output.getvalue())

    def test_cleanup_only_removes_job_owned_configuration(self):
        cache.setup(self.env, True)
        values = dict(line.split('=', 1) for line in self.github_env.read_text().splitlines())
        state = Path(values['NIX_CACHE_STATE_DIR'])
        cache.cleanup(dict(self.env, **values))
        self.assertFalse(state.exists())
        args = [c[0] for c in self.calls]
        self.assertIn(['sudo', '-n', 'rm', '-rf', str(self.daemon)], args)
        self.assertFalse(any('/etc/nix' in ' '.join(a) for a in args))

    def test_failed_daemon_restart_still_removes_credentials(self):
        cache.setup(self.env, True)
        values = dict(line.split('=', 1) for line in self.github_env.read_text().splitlines())
        state = Path(values['NIX_CACHE_STATE_DIR'])
        original = self.fake_run
        def fail_restart(args, **kwargs):
            if args[:4] == ['sudo', '-n', 'systemctl', 'restart']:
                return subprocess.CompletedProcess(args, 1, '', 'error')
            return original(args, **kwargs)
        self.runner.side_effect = fail_restart
        with self.assertRaises(cache.CacheError):
            cache.cleanup(dict(self.env, **values))
        self.assertFalse(state.exists())
        self.assertIn(['sudo', '-n', 'rm', '-rf', str(self.daemon)], [c[0] for c in self.calls])

    def test_explicit_profile_uses_credentials_file_not_other_aws_provider(self):
        url, _ = cache.config(self.env)
        self.assertIn('&profile=default', url)

    def test_cleanup_refuses_arbitrary_directory(self):
        with self.assertRaises(cache.CacheError):
            cache.cleanup(dict(self.env, NIX_CACHE_STATE_DIR='/tmp'))
        self.assertFalse(self.calls)


class CaptureTests(unittest.TestCase):
    def test_progress_paths_do_not_override_real_errors(self):
        for progress, error, expected in (
            ('copying /nix/store/' + 'a' * 29 + '403-thing', 'No space left on device', cache.Failure.DISK),
            ('copying /nix/store/' + 'a' * 32 + '-openssl', 'path is not valid', cache.Failure.MISSING_PATH),
        ):
            with self.subTest(expected=expected):
                result = cache.capture_command(
                    [sys.executable, '-c', f'import sys; print({progress!r}); print({error!r}, file=sys.stderr); sys.exit(1)'],
                    env=os.environ.copy(), input=None, label=cache.Stage.COPY)
                self.assertEqual(result.failure_category, expected)

    def test_fixed_failure_classification(self):
        for message, expected in (
            ('HTTP 403 Forbidden', cache.Failure.PERMISSION),
            ('Permission denied', cache.Failure.PERMISSION),
            ('Connection reset by peer', cache.Failure.NETWORK),
            ('No space left on device', cache.Failure.DISK),
            ('path is not valid', cache.Failure.MISSING_PATH),
            ('unrecognized secret diagnostic', cache.Failure.UNKNOWN),
        ):
            with self.subTest(expected=expected):
                self.assertEqual(cache.classify_failure(message), expected)
        self.assertEqual(cache.classify_failure('x' * cache.DIAGNOSTIC_LIMIT + '403'),
                         cache.Failure.UNKNOWN)

    def test_child_that_closes_its_pipes_early_returns_at_exit(self):
        # sudo, systemctl and daemons often close output before exiting. The
        # helper must notice the exit, not wait out the 15-second heartbeat.
        started = time.monotonic()
        result = cache.capture_command(
            ['sh', '-c', 'exec >&- 2>&-; sleep 1; exit 3'],
            env=os.environ.copy(), input=None, label=cache.Stage.HELPER)
        elapsed = time.monotonic() - started
        self.assertEqual(result.returncode, 3)
        self.assertGreaterEqual(elapsed, 1)
        self.assertLess(elapsed, 3, f'child exit noticed after {elapsed:.1f}s')

    def test_real_subprocess_live_heartbeat_and_synthetic_failure(self):
        # The child waits for a marker written only when a heartbeat is flushed.
        # Thus this proves a live heartbeat, not buffered logs after completion.
        with tempfile.TemporaryDirectory() as directory:
            marker = Path(directory) / 'heartbeat'
            class LiveOutput(io.StringIO):
                def flush(self):
                    if self.getvalue().count('status=heartbeat') >= 2:
                        marker.touch()
                    super().flush()
            output = LiveOutput()
            script = (
                'import os, pathlib, sys, time\n'
                'print(os.environ["SYNTHETIC_SECRET"], flush=True)\n'
                'sys.stderr.write("https://secret.invalid/path?token=hidden 403 Forbidden\\n")\n'
                'sys.stderr.flush()\n'
                'deadline = time.monotonic() + 5\n'
                'while not pathlib.Path(sys.argv[1]).exists() and time.monotonic() < deadline:\n'
                '    time.sleep(0.01)\n'
                'sys.exit(1)\n'
            )
            with patch.object(cache, 'HEARTBEAT_SECONDS', 0.05), contextlib.redirect_stdout(output):
                with self.assertRaises(cache.CacheError) as error:
                    with cache.stage(cache.Stage.COPY):
                        cache.run([sys.executable, '-c', script, str(marker)],
                                  env=dict(os.environ, SYNTHETIC_SECRET=SECRET), label=cache.Stage.COPY)
            self.assertTrue(marker.exists())
            log = output.getvalue() + str(error.exception)
            self.assertIn('stage=copy status=heartbeat', log)
            self.assertIn('still running; not an upload count', log)
            self.assertIn('classification=permission-or-403', log)
            for value in (SECRET, 'https://', 'secret.invalid', 'token=hidden', str(marker), script):
                self.assertNotIn(value, log)

    def test_real_subprocess_failure_categories_never_echo_output(self):
        for message, expected in (
            ('Permission denied', 'permission-or-403'),
            ('Connection refused', 'network'),
            ('Disk quota exceeded', 'disk'),
            ('No such file or directory', 'missing-path'),
            ('unexpected arbitrary output', 'unknown'),
        ):
            with self.subTest(expected=expected):
                output = io.StringIO()
                script = 'import sys; sys.stderr.write(sys.argv[1]); sys.exit(2)'
                with contextlib.redirect_stdout(output), self.assertRaises(cache.CacheError) as error:
                    cache.run([sys.executable, '-c', script, message + ' ' + SECRET],
                              label=cache.Stage.COPY)
                self.assertEqual(error.exception.category.value, expected)
                self.assertNotIn(SECRET, str(error.exception) + output.getvalue())
                self.assertNotIn(message, str(error.exception) + output.getvalue())

    def test_copy_does_not_retain_or_limit_unused_stdout(self):
        with patch.object(cache, 'OUTPUT_LIMIT', 8):
            result = cache.run([sys.executable, '-c', 'print("private" * 10000)'],
                               label=cache.Stage.COPY)
        self.assertEqual(result, '')

    def test_real_subprocess_drains_large_stderr_and_bounds_stdout(self):
        output = io.StringIO()
        script = 'import sys; sys.stderr.write("secret" * 100000); sys.stdout.write("x" * 8192)'
        with patch.object(cache, 'OUTPUT_LIMIT', 1024), contextlib.redirect_stdout(output):
            with self.assertRaisesRegex(cache.CacheError, 'safe capture limit'):
                cache.run([sys.executable, '-c', script], label=cache.Stage.CLOSURE)
        self.assertNotIn('secret', output.getvalue())

    def test_real_subprocess_input_and_invalid_utf8_are_not_logged(self):
        output = io.StringIO()
        script = 'import sys; assert sys.stdin.read() == "private"; sys.stdout.buffer.write(b"ok\\xff")'
        with contextlib.redirect_stdout(output):
            result = cache.run([sys.executable, '-c', script], data='private')
        self.assertEqual(result, 'ok\ufffd')
        self.assertEqual(output.getvalue(), '')

    def test_spawn_error_does_not_echo_executable(self):
        with self.assertRaises(cache.CacheError) as error:
            cache.run(['/nonexistent/private-token'], label=cache.Stage.VERSION)
        self.assertIn('classification=missing-path', str(error.exception))
        self.assertNotIn('private-token', str(error.exception))


if __name__ == '__main__':
    unittest.main()
