"""Operator configuration must not couple desktop startup to Slack routing."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


class DeploymentConfigTests(unittest.TestCase):
    def test_desktop_accepts_its_own_credentials_while_gateway_requires_routing(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'desktop.json'
            path.write_text(json.dumps({'doppler': {
                'identity': 'desktop-identity', 'audience': 'https://example.test/desktop',
                'account': 'desktop@example.test', 'instance_id': '1234',
                'project': 'remote-agents', 'config': 'desktop',
            }}))
            command = [sys.executable, str(Path(__file__).with_name('deployment_config.py'))]
            desktop = subprocess.run([*command, 'workstation', str(path)], capture_output=True, text=True)
            self.assertEqual(desktop.returncode, 0, desktop.stderr)
            gateway = subprocess.run([*command, 'gateway', str(path)], capture_output=True, text=True)
            self.assertNotEqual(gateway.returncode, 0)
            self.assertIn('slack_team', gateway.stderr)


if __name__ == '__main__':
    unittest.main()
