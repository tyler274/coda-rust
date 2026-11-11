import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import shell_probe as probe


class SourceControlTests(unittest.TestCase):
    def setUp(self):
        from subprocess import CompletedProcess
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.directory = Path(self.tmp.name)
        self.source = '/nix/store/00000000000000000000000000000000-source'
        self.archive = CompletedProcess([], 0, json.dumps({'inputs': {'x': {'path': self.source}}}),
                                        f"copying path '{self.source}' from 's3://private?priority=50'...\n")
        self.enter = CompletedProcess([], 0, '', '')
        cold_store = patch.object(Path, 'glob', return_value=[])
        cold_store.start()
        self.addCleanup(cold_store.stop)

    def test_baseline_only_enters_ordinary_shell(self):
        with patch.object(probe, 'run_logged', return_value=self.enter) as run:
            probe.shell('baseline', self.directory)
        self.assertEqual(run.call_count, 1)
        self.assertEqual(run.call_args.args[0], ['nix', 'develop', '.#postgres-ci', '--command', 'true'])

    def test_baseline_rejects_already_warm_private_sources(self):
        with patch.object(probe, 'run_logged', return_value=self.archive), self.assertRaisesRegex(ValueError, 'contaminated'):
            probe.shell('baseline', self.directory)

    def test_a_override_is_source_only(self):
        with patch.object(probe, 'run_logged', side_effect=[self.archive, self.enter]) as run:
            probe.shell('A', self.directory)
        self.assertEqual(run.call_args_list[0].args[0][-3:], ['--option', 'substituters', ''])
        self.assertNotIn('--option', run.call_args_list[1].args[0])

    def test_b_times_only_production_entry_with_log_provenance(self):
        with patch.object(Path, 'glob', return_value=[]), patch.object(probe, 'run_logged', return_value=self.archive) as run, patch.object(probe.time, 'monotonic', side_effect=[10, 12]):
            probe.shell('B', self.directory)
        self.assertEqual(run.call_count, 1)
        self.assertEqual(run.call_args.args[0], ['nix', 'develop', '.#postgres-ci', '--command', 'true'])
        evidence = json.loads((self.directory / 'shell.json').read_text())
        self.assertEqual(evidence['production_shell_seconds'], 2)
        self.assertIsNone(evidence['source_seconds'])
        self.assertEqual(evidence['source_split']['copies'][0]['route'], 'private-s3')

    def test_b_rejects_upstream_fallback(self):
        self.archive.stderr = 'unpacking github input\n'
        with patch.object(probe, 'run_logged', return_value=self.archive), self.assertRaisesRegex(ValueError, 'private S3'):
            probe.shell('B', self.directory)

    def test_b_rejects_mixed_private_and_upstream_sources(self):
        self.archive.stderr += 'unpacking github input\n'
        with patch.object(probe, 'run_logged', return_value=self.archive), self.assertRaisesRegex(ValueError, 'private S3'):
            probe.shell('B', self.directory)

    def test_b_rejects_warm_store_without_running_nix(self):
        with patch.object(Path, 'glob', return_value=[Path(self.source)]), patch.object(probe, 'run_logged') as run, self.assertRaisesRegex(ValueError, 'Contaminated'):
            probe.shell('B', self.directory)
        run.assert_not_called()

    def test_existing_source_rejected(self):
        with patch.object(Path, 'glob', return_value=[Path(self.source)]), patch.object(probe, 'run_logged', return_value=self.archive), self.assertRaisesRegex(ValueError, 'Contaminated'):
            probe.shell('A', self.directory)

    def test_probe_is_read_only_and_ci_routing_is_isolated(self):
        workflow = (probe.ROOT / '.github/workflows/postgres-image-probe.yml').read_text()
        self.assertIn('workflow_call:', workflow)
        self.assertNotIn('packages: write', workflow)
        self.assertNotIn('actions/cache/save', workflow)
        self.assertNotIn('download-artifact', workflow)
        self.assertEqual(workflow.count('fail-on-cache-miss: true'), 2)
        self.assertNotIn("if: inputs.target_key != ''", workflow)
        ci = (probe.ROOT / '.github/workflows/ci.yml').read_text()
        condition = "github.event_name != 'workflow_dispatch' || inputs.postgres_probe == 'none' || inputs.postgres_probe == ''"
        self.assertIn('  scope:\n    if: ' + condition, ci)
        self.assertIn('  lightweight:\n    if: ' + condition, ci)
        self.assertIn('    name: CI result\n    if: always() && (' + condition + ')', ci)

    def test_measurement_includes_image_preparation_and_gap(self):
        def job(name, start, end, steps=()):
            return {'name': 'postgres-probe / ' + name, 'status': 'completed', 'conclusion': 'success',
                    'started_at': f'2026-11-06T00:00:{start:02d}Z',
                    'completed_at': f'2026-11-06T00:00:{end:02d}Z', 'steps': steps}
        test = job('Exercise full shard 2', 30, 40)
        test['name'] = 'Exercise full shard 2'
        result = probe.measurements([job('Prepare probe image', 0, 10), job('Probe container', 20, 50, [test])])
        self.assertEqual(result['critical_path_seconds'], 50)
        self.assertEqual(result['setup_before_test_seconds'], 30)
        self.assertEqual(result['runner_seconds'], 40)


class PortableCacheTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.cargo = self.root / '.ci-cargo'
        for name in ('.ci-cargo/registry/cache/index/crate-1.crate',
                     '.ci-cargo/registry/index/index/.cache/cr/at/crate',
                     'target/debug/deps/libcrate.rlib'):
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b'cached contents')

    def check(self):
        with patch.dict(os.environ, CARGO_HOME=str(self.cargo), PROBE_TARGET_HIT='true'):
            return probe.cache_ready(self.root, self.root / 'proof')

    def test_populated_portable_cache_is_ready(self):
        self.check()
        self.assertTrue((self.root / 'proof/cache-ready.json').exists())

    def test_exact_hit_with_downloads_in_wrong_home_is_rejected(self):
        self.cargo.rename(self.root / 'wrong-home')
        with self.assertRaisesRegex(ValueError, 'Cargo downloads'):
            self.check()

    def test_empty_download_is_rejected(self):
        (self.cargo / 'registry/cache/index/crate-1.crate').write_bytes(b'')
        with self.assertRaisesRegex(ValueError, 'Cargo downloads'):
            self.check()

    def test_empty_registry_index_is_rejected(self):
        (self.cargo / 'registry/index/index/.cache/cr/at/crate').write_bytes(b'')
        with self.assertRaisesRegex(ValueError, 'registry index'):
            self.check()

    def test_wrong_cargo_home_is_rejected(self):
        with patch.dict(os.environ, CARGO_HOME=str(self.root / 'home/.cargo'), PROBE_TARGET_HIT='true'), self.assertRaisesRegex(ValueError, 'portable'):
            probe.cache_ready(self.root, self.root / 'proof')

    def test_workspace_archive_relocates_to_container_cargo_home(self):
        import tarfile
        archive = self.root / 'cache.tar'
        with tarfile.open(archive, 'w') as output:
            for name in ('.ci-cargo/registry/index', '.ci-cargo/registry/cache', 'target'):
                output.add(self.root / name, arcname=name)
        container_workspace = self.root / 'container/__w/repo/repo'
        container_workspace.mkdir(parents=True)
        with tarfile.open(archive) as source:
            source.extractall(container_workspace, filter='data')
        with patch.dict(os.environ, CARGO_HOME=str(container_workspace / '.ci-cargo'), PROBE_TARGET_HIT='true'):
            probe.cache_ready(container_workspace, container_workspace / 'proof')

    def test_empty_target_is_rejected(self):
        (self.root / 'target/debug/deps/libcrate.rlib').write_bytes(b'')
        with self.assertRaisesRegex(ValueError, 'target'):
            self.check()

    def test_nonexact_hit_is_rejected(self):
        with patch.dict(os.environ, CARGO_HOME=str(self.cargo), PROBE_TARGET_HIT='false'), self.assertRaisesRegex(ValueError, 'exact'):
            probe.cache_ready(self.root, self.root / 'proof')

    def test_writer_and_readers_share_relative_paths_and_new_schema(self):
        ci = (probe.ROOT / '.github/workflows/ci.yml').read_text().split('\n  postgres:\n')[1].split('\n  # Rebuilds')[0]
        workflow = (probe.ROOT / '.github/workflows/postgres-image-probe.yml').read_text()
        paths = '.ci-cargo/registry/index\n            .ci-cargo/registry/cache\n            .ci-cargo/git/db\n            target'
        self.assertEqual(ci.count(paths), 2)
        self.assertEqual(workflow.count(paths), 2)
        for text in (ci, workflow):
            self.assertIn('CARGO_HOME: ${{ github.workspace }}/.ci-cargo', text)
            self.assertNotIn('~/.cargo', text)
            self.assertIn('postgres-v2-', text)
        shell = workflow.split('\n  shell:\n')[1].split('\n  image:\n')[0]
        self.assertLess(shell.index('shell_probe.py cache-ready'), shell.index('shell_probe.py shell'))
        self.assertEqual(workflow.count('shell_probe.py cache-ready'), 2)


@unittest.skipUnless(os.environ.get('NIX_CACHE_TRANSPORT_TEST') == '1', 'explicit real-Nix opt-in')
class RealSourceRouteTests(unittest.TestCase):
    def test_source_only_override_beats_already_published_sources(self):
        import subprocess
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            tree = root / 'tree'
            tree.mkdir()
            (tree / 'payload').write_text('actual NAR payload\n')
            env = dict(os.environ, NIX_USER_CONF_FILES='/dev/null', NIX_CONF_DIR=str(root),
                       NIX_CONFIG='experimental-features = nix-command flakes\n',
                       https_proxy='http://127.0.0.1:9', HTTPS_PROXY='http://127.0.0.1:9',
                       NO_PROXY='', no_proxy='')

            def nix(*args, name='producer', check=True):
                isolated = dict(env, XDG_CACHE_HOME=str(root / f'cache-{name}'))
                result = subprocess.run(['nix', '--store', f'local?root={root / name}',
                                         '--option', 'connect-timeout', '1',
                                         '--option', 'download-attempts', '1', *args],
                                        env=isolated, capture_output=True, text=True, timeout=60)
                if check:
                    self.assertEqual(result.returncode, 0, result.stderr)
                return result

            source = nix('store', 'add-path', '--name', 'source', str(tree)).stdout.strip()
            info = json.loads(nix('path-info', '--json', '--json-format', '1', source).stdout)[source]
            cache = root / 'binary-cache'
            cache.mkdir()
            nix('copy', '--to', cache.as_uri(), source)
            flake = root / 'flake'
            flake.mkdir()
            (flake / 'flake.nix').write_text(
                '{ inputs.dep = { url = "github:coda-probe-unreachable/dep"; flake = false; }; outputs = _: {}; }')
            locked = {'type': 'github', 'owner': 'coda-probe-unreachable', 'repo': 'dep',
                      'rev': '0' * 40, 'lastModified': 1, 'narHash': info['narHash']}
            (flake / 'flake.lock').write_text(json.dumps({'version': 7, 'root': 'root', 'nodes': {
                'root': {'inputs': {'dep': 'dep'}},
                'dep': {'flake': False, 'locked': locked,
                        'original': {'type': 'github', 'owner': 'coda-probe-unreachable', 'repo': 'dep'}}}}))
            # Simulate reader config with the source ALREADY published. CLI A
            # override must replace extra-substituters, not just add a fallback.
            env['NIX_CONFIG'] += f'extra-substituters = {cache.as_uri()}\n'
            result = nix('flake', 'archive', '--json', '--no-update-lock-file', str(flake),
                         '--option', 'substituters', '', name='control-a', check=False)
            self.assertNotEqual(result.returncode, 0, result.stdout)
            self.assertIn('github.com', result.stderr)
            result = nix('flake', 'archive', '--json', '--no-update-lock-file', str(flake), name='control-b')
            self.assertEqual(json.loads(result.stdout)['inputs']['dep']['path'], source)
            self.assertNotIn('unpacking', result.stderr)


if __name__ == '__main__':
    unittest.main()
