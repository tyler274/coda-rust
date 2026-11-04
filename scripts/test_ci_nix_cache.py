"""Workflow boundaries for the private Nix cache and timing report."""
import unittest
from pathlib import Path

import yaml


ROOT = Path(__file__).resolve().parents[1]


class WorkflowTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.jobs = yaml.safe_load((ROOT / '.github/workflows/ci.yml').read_text())['jobs']

    def test_signing_only_in_protected_main_environment(self):
        images = self.jobs['images']
        environment = images['environment']
        self.assertIn("github.event_name == 'push'", environment)
        self.assertIn("github.ref == 'refs/heads/main'", environment)
        self.assertIn("vars.NIX_CACHE_ENABLED == 'true'", environment)
        self.assertIn("'nix-cache-write'", environment)
        for job_id, job in self.jobs.items():
            for step in job.get('steps', []):
                if 'NIX_CACHE_SIGNING_KEY' in step.get('env', {}):
                    self.assertEqual(job_id, 'images')
                    self.assertIn("github.event_name == 'push'", step['if'])
                    self.assertIn("github.ref == 'refs/heads/main'", step['if'])
                    self.assertIn("vars.NIX_CACHE_ENABLED == 'true'", step['if'])

    def test_cache_writers_are_serialized_without_cancelling_active_builds(self):
        concurrency = self.jobs['images']['concurrency']
        self.assertIs(concurrency['cancel-in-progress'], False)
        group = concurrency['group']
        self.assertIn("github.event_name == 'push'", group)
        self.assertIn("github.ref == 'refs/heads/main'", group)
        self.assertIn("vars.NIX_CACHE_ENABLED == 'true'", group)
        self.assertIn("'nix-cache-main-writer'", group)
        self.assertIn('github.run_id', group)
        self.assertIn('github.run_attempt', group)

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
                cleanup = [s for s in job['steps'] if s.get('run') == 'python3 scripts/nix_cache.py cleanup']
                self.assertEqual(len(cleanup), 1)
                self.assertIn('always()', cleanup[0]['if'])
        self.assertEqual(readers, 6)

    def test_image_proof_remains_mandatory(self):
        proof = next(s for s in self.jobs['images']['steps'] if 'bash scripts/check-images.sh' in s.get('run', ''))
        self.assertFalse(proof.get('continue-on-error', False))
        self.assertNotIn('if', proof)


if __name__ == '__main__':
    unittest.main()
