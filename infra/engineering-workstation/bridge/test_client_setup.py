"""Installer registration for one Git URL and the two Codex proxies."""
from pathlib import Path
import stat
import tempfile
import unittest

from client_setup import register_git_helper, register_mcp_servers, upsert_mcp_server


class ClientSetupTests(unittest.TestCase):
    def test_git_helper_is_scoped_to_one_repository_url(self):
        with tempfile.TemporaryDirectory() as directory:
            gitconfig = Path(directory) / 'gitconfig'
            helper = Path(directory) / 'git-credential-coda-company'
            register_git_helper('Coda-Health/coda-rust', helper, gitconfig)
            register_git_helper('Other/repo', helper, gitconfig)
            text = gitconfig.read_text()
            self.assertIn('[credential "https://github.com/Coda-Health/coda-rust"]', text)
            self.assertIn('[credential "https://github.com/Other/repo"]', text)
            self.assertNotIn('[credential]\n', text)
            self.assertEqual(text.count('helper = '), 2)
            self.assertIn('useHttpPath = true', text)

    def test_mcp_registration_replaces_only_the_workstation_servers(self):
        existing = (
            'model = "kept"\n\n'
            '[mcp_servers.other]\n'
            'command = "keep-me"\n\n'
            '[mcp_servers.github-company]\n'
            'command = "old"\n'
            'args = ["old"]\n'
        )
        updated = upsert_mcp_server(existing, 'github-company', '/venv/bin/python', ['/bridge/github_mcp.py'])
        updated = upsert_mcp_server(updated, 'slack-history', '/venv/bin/python', ['/bridge/slack_mcp.py'])
        self.assertIn('model = "kept"', updated)
        self.assertIn('[mcp_servers.other]\ncommand = "keep-me"', updated)
        self.assertNotIn('command = "old"', updated)
        self.assertEqual(updated.count('[mcp_servers.github-company]'), 1)
        self.assertIn('[mcp_servers.slack-history]', updated)
        with tempfile.TemporaryDirectory() as directory:
            config = Path(directory) / 'codex' / 'config.toml'
            register_mcp_servers(config, '/venv/bin/python', '/bridge')
            self.assertEqual(stat.S_IMODE(config.stat().st_mode), 0o600)
            self.assertIn('github_mcp.py', config.read_text())
            self.assertIn('slack_mcp.py', config.read_text())


if __name__ == '__main__':
    unittest.main()
