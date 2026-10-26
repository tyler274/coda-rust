"""Git/gh protocol behavior and the GitHub installation-token HTTP contract."""
from contextlib import ExitStack
import json
import os
from pathlib import Path
import shlex
import subprocess
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch
from urllib.parse import urlsplit

from github_access import issue_token
from slack_bridge import SlackIngress
from slack_gateway import gateway_server
from test_bridge import CONFIG
from test_gateway import ScriptedSlack


SETTINGS = {'app_id': '123', 'installation_id': '456',
            'repository': 'example/product', 'repository_id': '789'}


class CompanyCliTests(unittest.TestCase):
    def test_git_and_gh_use_company_credentials_and_refusals_stop_personal_fallback(self):
        with tempfile.TemporaryDirectory() as directory, ExitStack() as cleanup:
            directory = Path(directory)
            config = {**CONFIG, 'github': SETTINGS}
            with patch('slack_bridge.SocketModeClient'):
                ingress = SlackIngress(ScriptedSlack(), config, 'synthetic-slack')
            provider = cleanup.enter_context(patch('github_access.issue_token', return_value={
                'token': 'synthetic-company-token', 'expires_at': '2099-01-01T00:00:00Z',
                'permissions': {'contents': 'read', 'actions': 'read'}}))
            server = cleanup.enter_context(gateway_server(None, directory / 'state',
                ScriptedSlack(), config, ingress, iap_host='127.0.0.1', port=0))
            threading.Thread(target=server.serve_forever, daemon=True).start()
            cleanup.callback(server.shutdown)
            port = str(server.socket.getsockname()[1])
            client = [sys.executable, str(Path(__file__).with_name('github_access.py')),
                      '--repository', SETTINGS['repository'], '--iap-port', port]
            helper = '!' + shlex.join(client + ['git-credential'])
            marker = directory / 'personal-used'
            personal = f'!touch {shlex.quote(str(marker))}; printf "username=personal\\npassword=personal\\n"'
            git = ['git', '-c', 'credential.helper=', '-c', 'credential.helper=' + helper,
                   '-c', 'credential.helper=' + personal, '-c', 'credential.useHttpPath=true',
                   'credential', 'fill']
            environment = {**os.environ, 'GIT_CONFIG_GLOBAL': os.devnull,
                           'GIT_CONFIG_NOSYSTEM': '1', 'GIT_TERMINAL_PROMPT': '0'}
            result = subprocess.run(git, input='protocol=https\nhost=github.com\npath=example/product.git\n\n',
                text=True, capture_output=True, env=environment, timeout=10)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn('username=x-access-token\npassword=synthetic-company-token', result.stdout)
            self.assertFalse(marker.exists())
            for protocol, host, path in (('https', 'github.com', 'example/other.git'),
                    ('http', 'github.com', 'example/product.git'), ('https', 'other.example', 'example/product.git')):
                result = subprocess.run(git, input=f'protocol={protocol}\nhost={host}\npath={path}\n\n',
                    text=True, capture_output=True, env=environment, timeout=10)
                self.assertNotEqual(result.returncode, 0)
                self.assertFalse(marker.exists(), result.stderr)
            self.assertEqual(provider.call_count, 1, 'Refused resources reached credential issuance')
            provider.side_effect = RuntimeError('synthetic provider unavailable')
            result = subprocess.run(git, input='protocol=https\nhost=github.com\npath=example/product.git\n\n',
                text=True, capture_output=True, env=environment, timeout=10)
            self.assertNotEqual(result.returncode, 0)
            self.assertFalse(marker.exists(), result.stderr)
            provider.side_effect = None

            gh = directory / 'gh'
            gh.write_text('#!' + sys.executable + '\nimport json, os, sys\n'
                'print(json.dumps({"company":os.environ["GH_TOKEN"]=="synthetic-company-token",'
                '"host":os.environ["GH_HOST"],"args":sys.argv[1:]}))\nsys.exit(7)\n')
            gh.chmod(0o700)
            result = subprocess.run(client + ['gh', 'run', 'view', '42', '--log'], text=True,
                capture_output=True, env={**environment, 'PATH': str(directory) + os.pathsep + os.environ['PATH'],
                                         'GH_TOKEN': 'synthetic-personal'}, timeout=10)
            self.assertEqual(result.returncode, 7, result.stderr)
            self.assertEqual(json.loads(result.stdout), {'company': True, 'host': 'github.com',
                              'args': ['run', 'view', '42', '--log']})

    def test_sdk_request_is_limited_to_the_configured_repository_and_read_permissions(self):
        import jwt
        import requests
        from cryptography.hazmat.primitives import serialization
        from cryptography.hazmat.primitives.asymmetric import rsa
        key = rsa.generate_private_key(public_exponent=65537, key_size=2048)
        pem = key.private_bytes(serialization.Encoding.PEM, serialization.PrivateFormat.PKCS8,
                                serialization.NoEncryption()).decode()
        sent = []

        def response(request, **kwargs):
            sent.append(request)
            result = requests.Response()
            result.status_code = 201
            result.headers['Content-Type'] = 'application/json'
            result._content = json.dumps({'token': 'synthetic-token', 'expires_at': '2099-01-01T00:00:00Z',
                                         'permissions': {'contents': 'read'}}).encode()
            return result

        with patch.dict(os.environ, {'GITHUB_APP_PRIVATE_KEY': pem}), \
                patch('requests.Session.send', side_effect=response):
            self.assertEqual(issue_token(SETTINGS)['token'], 'synthetic-token')
        self.assertEqual(len(sent), 1)
        request = sent[0]
        url = urlsplit(request.url)
        self.assertEqual((request.method, url.scheme, url.hostname, url.path),
                         ('POST', 'https', 'api.github.com', '/app/installations/456/access_tokens'))
        self.assertIn(url.port, (None, 443))
        self.assertEqual(json.loads(request.body), {'repository_ids': [789],
            'permissions': {'contents': 'read', 'pull_requests': 'read', 'actions': 'read'}})
        claims = jwt.decode(request.headers['Authorization'].split()[1], key.public_key(), algorithms=['RS256'])
        self.assertEqual(claims['iss'], '123')


if __name__ == '__main__':
    unittest.main()
