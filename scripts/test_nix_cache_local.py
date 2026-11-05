"""Local publishing requires opt-in and a clean, explicit revision."""
import unittest
from unittest.mock import Mock

from nix_cache_local import authorize


class LocalPolicyTests(unittest.TestCase):
    def setUp(self):
        self.env = {'ACT': 'true', 'GITHUB_ACTIONS': 'true'}
        self.sha = 'a' * 40
        self.run = Mock(side_effect=[self.sha, ''])

    def check(self, **kwargs):
        return authorize(kwargs.get('env', self.env), kwargs.get('sha', self.sha),
                         kwargs.get('ack', True), self.run, ValueError)

    def test_explicit_clean_revision(self):
        self.assertEqual(self.check(), self.sha)
        self.assertEqual(self.run.call_count, 2)

    def test_hosted_and_non_act_refused_before_commands(self):
        for env in ({}, {'ACT': 'false', 'GITHUB_ACTIONS': 'true'}, {'ACT': 'true'}):
            with self.subTest(env=env), self.assertRaises(ValueError):
                self.check(env=env)
        self.run.assert_not_called()

    def test_acknowledgement_required(self):
        with self.assertRaises(ValueError):
            self.check(ack=False)
        self.run.assert_not_called()

    def test_full_sha_required(self):
        for sha in ('', 'abc123', 'main', 'a' * 40 + '\n'):
            with self.subTest(sha=sha), self.assertRaises(ValueError):
                self.check(sha=sha)
        self.run.assert_not_called()

    def test_wrong_revision(self):
        self.run.side_effect = ['b' * 40]
        with self.assertRaisesRegex(ValueError, 'expected commit'):
            self.check()

    def test_dirty_tracked_or_untracked_source(self):
        for status in (' M scripts/nix_cache.py', '?? extra.nix'):
            self.run.side_effect = [self.sha, status]
            with self.subTest(status=status), self.assertRaisesRegex(ValueError, 'clean checkout'):
                self.check()


if __name__ == '__main__':
    unittest.main()
