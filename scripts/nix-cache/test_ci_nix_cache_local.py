"""Offline contracts for local/hosted cache root parity and authority boundaries."""

import json
from pathlib import Path
import re
import subprocess
import tempfile
import unittest
from unittest.mock import patch

import nix_cache_roots as roots


REPO = Path(__file__).resolve().parents[2]


class RootTests(unittest.TestCase):
    def test_exact_reusable_roots(self):
        self.assertEqual(roots.DEPENDENCY_ROOTS, (
            'coda-servers-deps', 'coda-web-deps', 'patient-bank-deps'))
        self.assertEqual(roots.IMAGE_ROOTS, (
            'physician-sample-image', 'physician-worker-image', 'duet-api-image',
            'physician-migration-image', 'duet-migration-image', 'duet-privacy-delivery-image',
            'observability-relay-image', 'langfuse-sso-image'))
        packages = (REPO / 'nix/packages.nix').read_text()
        for root in roots.DEPENDENCY_ROOTS + roots.IMAGE_ROOTS:
            self.assertIn(root, packages)

    def test_every_ci_shell_is_a_root(self):
        workflow = (REPO / '.github/workflows/ci.yml').read_text()
        used = set(re.findall(r'nix develop \.#(\w+)', workflow))
        self.assertTrue(used)
        self.assertEqual({r.rsplit('.', 1)[1] for r in roots.SHELL_ROOTS}, used)

    def test_build_preserves_json_and_only_builds_allowlist(self):
        result = [{'drvPath': '/nix/store/example.drv', 'outputs': {'out': '/nix/store/example'}}]
        def run(command, **kwargs):
            self.assertEqual(command, ['nix', 'build', '--no-link', '--json',
                                       '--no-update-lock-file', *[f'.#{r}' for r in roots.ROOTS]])
            self.assertTrue(kwargs['check'])
            kwargs['stdout'].write(json.dumps(result))
        with tempfile.TemporaryDirectory() as temp, patch.object(roots.subprocess, 'run', run):
            output = Path(temp) / 'build.json'
            roots.build(output)
            self.assertEqual(json.loads(output.read_text()), result)

    def test_checkout_checks_full_sha_head_and_untracked_files(self):
        sha = 'a' * 40
        with tempfile.TemporaryDirectory() as temp, patch.object(roots, 'ROOT', Path(temp)):
            subprocess.run(['git', 'init', '-q', temp], check=True)
            subprocess.run(['git', '-c', 'user.name=Test', '-c', 'user.email=test@example.invalid',
                            'commit', '--allow-empty', '-qm', 'fixture'], cwd=temp, check=True)
            actual = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=temp, text=True).strip()
            roots.verify_checkout(actual)
            for invalid in ('', actual[:12], 'A' * 40, sha):
                with self.assertRaises(ValueError):
                    roots.verify_checkout(invalid)
            file = Path(temp) / 'untracked'
            file.touch()
            with self.assertRaisesRegex(ValueError, 'clean'):
                roots.verify_checkout(actual)
            subprocess.run(['git', 'add', 'untracked'], cwd=temp, check=True)
            with self.assertRaisesRegex(ValueError, 'clean'):
                roots.verify_checkout(actual)

    def test_smoke_selects_realized_leaf_derivation(self):
        drv = '/nix/store/' + 'a' * 32 + '-root.drv'
        leaf = '/nix/store/' + 'b' * 32 + '-leaf.drv'
        with tempfile.TemporaryDirectory() as temp:
            build, output = Path(temp) / 'build.json', Path(temp) / 'smoke.json'
            build.write_text(json.dumps([{'drvPath': drv, 'outputs': {}}]))
            with patch.object(roots.subprocess, 'check_output', side_effect=[drv + '\n' + leaf, leaf, '']):
                roots.smoke(build, output)
            self.assertEqual(json.loads(output.read_text()), [{'drvPath': leaf, 'outputs': {}}])

    def test_build_failure_is_not_success(self):
        with tempfile.TemporaryDirectory() as temp, patch.object(
                roots.subprocess, 'run', side_effect=subprocess.CalledProcessError(1, 'nix')):
            with self.assertRaises(subprocess.CalledProcessError):
                roots.build(Path(temp) / 'build.json')


class WorkflowTests(unittest.TestCase):
    def setUp(self):
        self.local = (REPO / '.github/workflows/nix-cache-local.yml').read_text()
        self.hosted = (REPO / '.github/workflows/ci.yml').read_text()

    def test_both_writers_use_shared_collector(self):
        for workflow in (self.local, self.hosted):
            self.assertIn('python3 scripts/nix-cache/nix_cache_roots.py build', workflow)
            self.assertNotIn('.#coda-servers-deps', workflow)

    def test_local_credentials_only_reach_publish(self):
        before, publish = self.local.split('      - name: Publish signed reusable Nix outputs\n')
        self.assertNotIn('secrets.', before)
        self.assertIn('verify-checkout --expected-sha "$EXPECTED_SHA"', before)
        self.assertLess(before.index('verify-checkout'), before.index('uses: cachix/'))
        self.assertIn('publish-local', publish)
        self.assertIn('--acknowledge-local-write', publish)
        self.assertIn('--expected-sha "$EXPECTED_SHA"', publish)
        self.assertNotIn('continue-on-error', self.local)

    def test_local_is_manual_and_act_only_without_deployment(self):
        self.assertIn("github.event_name == 'workflow_dispatch'", self.local)
        self.assertIn("vars.ACT == 'true'", self.local)
        self.assertIn('test "${ACT:-}" = true', self.local)
        self.assertNotIn('  push:', self.local)
        self.assertNotIn('  pull_request:', self.local)
        self.assertIn('install_options: --no-daemon --yes', self.local)
        self.assertIn('enable_kvm: false', self.local)
        for forbidden in ('systemctl', 'docker push', 'gcloud', 'deploy-staging', 'setup --disposable-runner'):
            self.assertNotIn(forbidden, self.local)

    def test_hosted_publication_retains_protected_main_boundary(self):
        publish = self.hosted.split('      - name: Publish signed reusable Nix outputs\n')[1].split('      - name:')[0]
        self.assertIn("github.event_name == 'push' && github.ref == 'refs/heads/main'", publish)
        self.assertIn("vars.NIX_CACHE_ENABLED == 'true'", publish)
        self.assertIn('scripts/nix-cache/nix_cache.py publish --build-json', publish)
        self.assertNotIn('publish-local', self.hosted)
        self.assertIn('    environment: nix-cache-write\n', self.hosted)


if __name__ == '__main__':
    unittest.main()
