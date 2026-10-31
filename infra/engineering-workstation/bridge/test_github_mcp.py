"""Real MCP transport/process lifecycle; GitHub permissions need live readback."""
from contextlib import ExitStack
import json
import os
from pathlib import Path
import queue
import subprocess
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch

from websockets.sync.client import connect

from github_mcp import github_runtime
from slack_bridge import SlackIngress
from slack_gateway import gateway_server, main
from workstation_connector import relay
from test_bridge import CONFIG
from test_gateway import ScriptedNative, ScriptedSlack


class CompanyMcpTests(unittest.TestCase):
    def test_provider_sessions_share_startup_credentials_without_repeating_oidc_exchange(self):
        config = {**CONFIG, 'github': {'app_id': '123', 'installation_id': '456'},
                  'doppler': {'identity': 'gateway-identity', 'audience': 'https://example.test/gateway',
                              'account': 'gateway@example.test', 'instance_id': '5678',
                              'project': 'remote-agents', 'config': 'gateway'}}
        secrets = {'SLACK_BOT_TOKEN': 'synthetic-bot', 'SLACK_APP_TOKEN': 'synthetic-app',
                   'GITHUB_APP_PRIVATE_KEY': 'synthetic-company-key'}
        with tempfile.TemporaryDirectory() as directory, \
                patch.dict(os.environ, {'GH_TOKEN': 'synthetic-personal-token'}, clear=True), \
                patch('slack_gateway.fetch', side_effect=[secrets]) as exchange:
            path = Path(directory) / 'selected-gateway.json'
            path.write_text(json.dumps(config))
            with patch.object(sys, 'argv', ['slack_gateway.py', '--config', str(path), '--check-credentials']):
                main()
            for _ in range(2):
                _, environment = github_runtime(config)
                self.assertEqual(environment['GITHUB_APP_PRIVATE_KEY'], secrets['GITHUB_APP_PRIVATE_KEY'])
                self.assertNotIn('GH_TOKEN', environment)
                self.assertNotIn('SLACK_BOT_TOKEN', environment)
                self.assertNotIn('SLACK_APP_TOKEN', environment)
            exchange.assert_called_once_with(list(secrets), config['doppler'])

    def test_mcp_roundtrip_does_not_take_the_native_connection_and_closes_its_provider(self):
        with tempfile.TemporaryDirectory(prefix='company-mcp-') as directory:
            directory = Path(directory)
            stopped = directory / 'provider-stopped'
            provider = directory / 'provider.py'
            provider.write_text('''import json, pathlib, sys
for line in sys.stdin:
    request = json.loads(line)
    print(json.dumps({'jsonrpc':'2.0','method':'notifications/message',
                      'params':{'data':'provider notification'}}), flush=True)
    print(json.dumps({'jsonrpc':'2.0','id':request['id'],
                      'result':request['params']}), flush=True)
pathlib.Path(sys.argv[1]).write_text('stdin closed')
''')
            config = {**CONFIG, 'github': {'app_id': '123', 'installation_id': '456'}}
            web = ScriptedSlack()
            with ExitStack() as cleanup:
                with patch('slack_bridge.SocketModeClient'):
                    ingress = SlackIngress(web, config, 'synthetic-token')
                # The external provider boundary is a real stdio process here.
                # The separate live check uses GitHub's released binary and App.
                cleanup.enter_context(patch('github_mcp.github_runtime', return_value=(
                    [sys.executable, str(provider), str(stopped)], {'PATH': os.defpath})))
                server = cleanup.enter_context(gateway_server(None, directory / 'state', web,
                    config, ingress, iap_host='127.0.0.1', port=0))
                threading.Thread(target=server.serve_forever, daemon=True).start()
                cleanup.callback(server.shutdown)
                port = server.socket.getsockname()[1]
                native = ScriptedNative([])
                connection = cleanup.enter_context(connect(f'ws://127.0.0.1:{port}', proxy=None))
                worker = threading.Thread(target=relay, args=(connection, native), daemon=True)
                worker.start()
                cleanup.callback(connection.close)
                self.assertTrue(web.await_posts(1))

                with subprocess.Popen([sys.executable, str(Path(__file__).with_name('github_mcp.py')),
                        '--iap-port', str(port)], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                        stderr=subprocess.PIPE, text=True, bufsize=1) as client:
                    received = queue.Queue()
                    output = threading.Thread(target=lambda: [received.put(json.loads(line))
                        for line in client.stdout], daemon=True)
                    output.start()
                    for ident, value in ((1, 'one\ntwo'), (2, 'quotes " and Unicode λ')):
                        params = {'value': value}
                        client.stdin.write(json.dumps({'jsonrpc': '2.0', 'id': ident,
                                                       'method': 'tools/call', 'params': params}) + '\n')
                        client.stdin.flush()
                        self.assertEqual(received.get(timeout=5)['method'], 'notifications/message')
                        self.assertEqual(received.get(timeout=5), {'jsonrpc': '2.0', 'id': ident,
                                                                  'result': params})
                    self.assertFalse(native.closed.is_set(), 'The MCP connection displaced native Codex')
                    client.stdin.close()
                    self.assertEqual(client.wait(timeout=10), 0, client.stderr.read())
                    output.join(timeout=5)
                    self.assertFalse(output.is_alive())
                connection.close()
                worker.join(timeout=5)
                self.assertFalse(worker.is_alive())
            self.assertEqual(stopped.read_text(), 'stdin closed')


if __name__ == '__main__':
    unittest.main()
