"""Fresh-reader verification never uses writer credentials or local builds."""
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import nix_cache as cache
import nix_cache_verify as verify
from test_nix_cache import OUT, PUBLIC


class ReaderTests(unittest.TestCase):
    def test_fresh_reader_copy_and_verify_cleanup(self):
        with tempfile.TemporaryDirectory() as scratch:
            directory = Path(scratch)
            build = directory / 'build.json'
            build.write_text(json.dumps([{'drvPath': '/nix/store/' + 'a' * 32 + '-coda-servers-deps-0.1.0.drv', 'outputs': {'out': OUT}}]))
            env = dict(os.environ, RUNNER_TEMP=scratch,
                       NIX_CACHE_BUCKET='private-cache', NIX_CACHE_PUBLIC_KEY=PUBLIC,
                       NIX_CACHE_READ_ACCESS_KEY_ID='GOOGREAD',
                       NIX_CACHE_READ_SECRET_ACCESS_KEY='readSecret',
                       AWS_ACCESS_KEY_ID='GOOGWRITE', AWS_SECRET_ACCESS_KEY='writeSecret',
                       NIX_CACHE_SIGNING_KEY='privateSigningKey',
                       NIX_CACHE_WRITE_ACCESS_KEY_ID='GOOGWRITE',
                       NIX_CACHE_WRITE_SECRET_ACCESS_KEY='writeSecret')
            calls = []

            def run(args, *, env):
                calls.append(args)
                self.assertNotIn('build', args)
                self.assertNotIn('--no-check-sigs', args)
                self.assertNotIn('privateSigningKey', str(env))
                self.assertNotIn('writeSecret', str(env))
                self.assertIn('readSecret', Path(env['AWS_SHARED_CREDENTIALS_FILE']).read_text())
                self.assertEqual(env['NIX_USER_CONF_FILES'], '/dev/null')
                return ''

            with patch.object(cache, 'run', side_effect=run):
                verify.verify(env, str(build))
            self.assertEqual(calls[0][:2], ['nix', 'copy'])
            self.assertEqual(calls[1][:3], ['nix', 'store', 'verify'])
            self.assertIn('--sigs-needed', calls[1])
            self.assertFalse(list(directory.glob('coda-nix-reader-*')))

    def test_failed_copy_cleans_reader_material(self):
        with tempfile.TemporaryDirectory() as scratch:
            build = Path(scratch) / 'build.json'
            build.write_text(json.dumps([{'drvPath': '/nix/store/' + 'a' * 32 + '-coda-servers-deps-0.1.0.drv', 'outputs': {'out': OUT}}]))
            env = {'RUNNER_TEMP': scratch, 'NIX_CACHE_BUCKET': 'private-cache',
                   'NIX_CACHE_PUBLIC_KEY': PUBLIC, 'NIX_CACHE_READ_ACCESS_KEY_ID': 'GOOGREAD',
                   'NIX_CACHE_READ_SECRET_ACCESS_KEY': 'readerSecret'}
            with patch.object(cache, 'run', side_effect=cache.CacheError('copy failed')), \
                    self.assertRaises(cache.CacheError):
                verify.verify(env, str(build))
            self.assertFalse(list(Path(scratch).glob('coda-nix-reader-*')))

    def test_empty_build_is_refused(self):
        with tempfile.TemporaryDirectory() as scratch:
            build = Path(scratch) / 'build.json'
            build.write_text('[]')
            with self.assertRaises(cache.CacheError):
                verify.verify({'NIX_CACHE_BUCKET': 'private-cache', 'NIX_CACHE_PUBLIC_KEY': PUBLIC}, str(build))


if __name__ == '__main__':
    unittest.main()
