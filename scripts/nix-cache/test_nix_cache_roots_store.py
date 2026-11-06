"""Recorded locked-input sources must be valid in a store that starts empty.

Uses the real Nix on PATH against a disposable `local?root=` store, offline,
with a `git+file` input: Nix unpacks git-type inputs into its fetcher cache, so
reporting their paths does not make them valid store paths.
"""

import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch

import nix_cache_roots as roots


@unittest.skipUnless(shutil.which('nix') and shutil.which('git'), 'requires nix and git')
class FreshStoreTests(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory(prefix='nix-cache-roots-')
        self.root = Path(temp.name)
        self.store = f'local?root={self.root / "store"}'
        env = {k: v for k, v in os.environ.items() if not k.startswith('NIX_')}
        env.update({
            'HOME': str(self.root / 'home'), 'XDG_CACHE_HOME': str(self.root / 'cache'),
            'NIX_CONF_DIR': str(self.root), 'NIX_USER_CONF_FILES': '/dev/null',
            'NIX_CONFIG': ('experimental-features = nix-command flakes\n'
                           f'store = {self.store}\nsubstituters =\n'),
            'GIT_CONFIG_GLOBAL': '/dev/null', 'GIT_CONFIG_NOSYSTEM': '1',
        })
        env_patch = patch.dict(os.environ, env, clear=True)
        env_patch.start()
        # Remove the read-only store contents only after restoring the environment.
        self.addCleanup(temp.cleanup)
        self.addCleanup(lambda: subprocess.run(['chmod', '-R', 'u+w', str(self.root)], check=False))
        self.addCleanup(env_patch.stop)

        dep = self.root / 'dep'
        dep.mkdir()
        (dep / 'payload.txt').write_text('locked git input\n')
        git = ['git', '-c', 'user.name=Test', '-c', 'user.email=test@example.invalid']
        subprocess.run(['git', 'init', '-q', str(dep)], check=True)
        subprocess.run([*git, 'add', '.'], cwd=dep, check=True)
        subprocess.run([*git, 'commit', '-qm', 'fixture'], cwd=dep, check=True)
        self.flake = self.root / 'flake'
        self.flake.mkdir()
        (self.flake / 'flake.nix').write_text(
            f'{{ inputs.dep = {{ url = "git+file://{dep}"; flake = false; }};\n'
            '  outputs = { self, dep }: { }; }\n')
        # Lock in a separate store and fetcher cache, so the store under test
        # starts empty, as on a fresh runner.
        subprocess.run(['nix', '--offline', '--store', f'local?root={self.root / "lock-store"}', 'flake', 'lock'],
                       cwd=self.flake, check=True, capture_output=True,
                       env=dict(os.environ, XDG_CACHE_HOME=str(self.root / 'lock-cache')))
        self.assertFalse((self.root / 'store').exists())

    def valid(self, path):
        return subprocess.run(['nix', 'path-info', path], capture_output=True).returncode == 0

    def test_recorded_inputs_exist_in_a_fresh_store(self):
        with patch.object(roots, 'ROOT', self.flake):
            inputs = roots.locked_inputs()
        self.assertEqual(set(inputs), {'dep'})
        for name, path in inputs.items():
            self.assertTrue(self.valid(path), f'{name} recorded as {path} but not valid in {self.store}')


if __name__ == '__main__':
    unittest.main()
