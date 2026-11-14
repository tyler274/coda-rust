"""Workflow boundaries for the private Nix cache and timing report."""
import unittest
from pathlib import Path

import yaml


ROOT = Path(__file__).resolve().parents[2]


class WorkflowTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.jobs = yaml.safe_load((ROOT / '.github/workflows/ci.yml').read_text())['jobs']

    def test_signing_only_in_protected_main_environment(self):
        publisher = self.jobs['nix-cache-publish']
        self.assertEqual(publisher['environment'], 'nix-cache-write')
        for clause in ("github.event_name == 'push'", "github.ref == 'refs/heads/main'",
                       "vars.NIX_CACHE_ENABLED == 'true'", "needs.images.result == 'success'"):
            self.assertIn(clause, publisher['if'])
        for job_id, job in self.jobs.items():
            if job_id != 'nix-cache-publish':
                self.assertNotIn('nix-cache-write', str(job.get('environment', '')))
            for step in job.get('steps', []):
                if 'NIX_CACHE_SIGNING_KEY' in step.get('env', {}):
                    self.assertEqual(job_id, 'nix-cache-publish')
                    self.assertIn("github.event_name == 'push'", step['if'])
                    self.assertIn("github.ref == 'refs/heads/main'", step['if'])
                    self.assertIn("vars.NIX_CACHE_ENABLED == 'true'", step['if'])

    def test_only_the_publisher_is_serialized(self):
        concurrency = self.jobs['nix-cache-publish']['concurrency']
        self.assertEqual(concurrency['group'], 'nix-cache-main-writer')
        self.assertIs(concurrency['cancel-in-progress'], False)
        for job_id, job in self.jobs.items():
            if job_id != 'nix-cache-publish':
                self.assertNotIn('nix-cache-main-writer', str(job.get('concurrency', '')))

    def test_publication_never_gates_or_delays_deployment(self):
        for job_id, job in self.jobs.items():
            needs = job.get('needs', [])
            self.assertNotIn('nix-cache-publish', needs if isinstance(needs, list) else [needs], job_id)

    def test_cache_and_reporting_do_not_replace_gate(self):
        result = self.jobs['result']
        self.assertEqual(result['permissions']['actions'], 'read')
        self.assertIn('images', result['needs'])
        gate = next(s for s in result['steps'] if s.get('run') == 'python3 scripts/ci_scope.py result')
        self.assertFalse(gate.get('continue-on-error', False))
        reports = [s for s in result['steps'] if 'ci_timings.py' in s.get('run', '')]
        self.assertEqual(len(reports), 1)
        self.assertTrue(reports[0]['continue-on-error'])
        self.assertEqual(reports[0]['if'], 'always()')

    def test_every_reader_cleans_credentials_even_on_failure(self):
        readers = 0
        for job in self.jobs.values():
            if any(s.get('uses') == './.github/actions/nix-cache' for s in job.get('steps', [])):
                readers += 1
                cleanup = [s for s in job['steps'] if s.get('run') == 'python3 scripts/nix-cache/nix_cache.py cleanup']
                self.assertEqual(len(cleanup), 1)
                self.assertIn('always()', cleanup[0]['if'])
        self.assertEqual(readers, 8)

    def test_every_reader_keeps_its_store_on_its_own_sticky_disk(self):
        for job_id, job in self.jobs.items():
            steps = job.get('steps', [])
            if not any(s.get('uses') == './.github/actions/nix-cache' for s in steps):
                continue
            self.assertNotIn('cachix/install-nix-action@v31', [s.get('uses') for s in steps], job_id)
            store = [i for i, s in enumerate(steps) if s.get('uses') == './.github/actions/nix-store']
            self.assertEqual(len(store), 1, job_id)
            self.assertEqual(steps[store[0]]['with']['job'], job_id)
            reader = next(i for i, s in enumerate(steps) if s.get('uses') == './.github/actions/nix-cache')
            self.assertLess(store[0], reader, job_id)
            # Release /nix last, after the cache cleanup restarted nix-daemon.
            finish = steps[-1]
            self.assertTrue(finish.get('run', '').startswith('python3 scripts/nix-cache/nix_store.py finish'), job_id)
            self.assertIn('always()', finish['if'])
            self.assertTrue(finish['continue-on-error'])

    def test_only_main_runs_save_a_store_disk(self):
        action = yaml.safe_load((ROOT / '.github/actions/nix-store/action.yml').read_text())
        steps = action['runs']['steps']
        mount = steps[0]
        self.assertEqual(mount['uses'], 'useblacksmith/stickydisk@v1')
        self.assertEqual(mount['with']['path'], '/nix')
        self.assertEqual(mount['with']['key'], 'nix-${{ inputs.job }}')
        for clause in ("github.ref == 'refs/heads/main'", "github.event_name != 'pull_request'",
                       "github.event_name != 'merge_group'"):
            self.assertIn(clause, mount['with']['commit'])
        self.assertEqual(steps[1]['uses'], 'cachix/install-nix-action@v31')

    def test_image_proof_remains_mandatory(self):
        proof = next(s for s in self.jobs['images']['steps'] if 'bash scripts/check-images.sh' in s.get('run', ''))
        self.assertFalse(proof.get('continue-on-error', False))
        self.assertNotIn('if', proof)


if __name__ == '__main__':
    unittest.main()
