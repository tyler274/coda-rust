"""The local launcher passes secret names, not secret values, on argv."""
import contextlib
import io
import json
import subprocess
import unittest
from unittest.mock import patch

import warm_nix_cache as warm


class LauncherTests(unittest.TestCase):
    def test_scoped_secrets_and_explicit_workflow(self):
        sha = 'a' * 40
        values = {name: {'computed': f'synthetic-{name}'}
                  for name in (*warm.SECRET_NAMES, *warm.VARIABLE_NAMES)}
        with patch('sys.argv', ['warm', '--expected-sha', sha, '--acknowledge-exclusive-writer']), \
                patch.object(warm.subprocess, 'check_output', side_effect=[sha, '']), \
                patch.object(warm.subprocess, 'run', return_value=subprocess.CompletedProcess([], 0, json.dumps(values))) as get, \
                patch.object(warm.subprocess, 'call', return_value=0) as act, \
                contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(warm.main(), 0)
        self.assertEqual(get.call_args.args[0][:3], ['doppler', 'secrets', 'get'])
        command = act.call_args.args[0]
        self.assertIn('.github/workflows/nix-cache-local.yml', command)
        self.assertEqual(command[command.index('-j') + 1], 'nix-cache-local')
        self.assertEqual(command[command.index('--container-daemon-socket') + 1], '-')
        for name in warm.SECRET_NAMES:
            self.assertIn(name, command)
            self.assertNotIn(values[name]['computed'], command)
        self.assertNotIn('--reuse', command)
        self.assertNotIn('--bind', command)

    def test_missing_ack_never_fetches_secrets(self):
        with patch('sys.argv', ['warm', '--expected-sha', 'a' * 40]), \
                patch.object(warm.subprocess, 'run') as get, \
                contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            warm.main()
        get.assert_not_called()


if __name__ == '__main__':
    unittest.main()
