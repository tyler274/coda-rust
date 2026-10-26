"""Exercise the native stdio/IAP-shaped boundary against scripted Slack pages."""
from contextlib import ExitStack
import json
from pathlib import Path
import queue
import subprocess
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch

from slack_bridge import SlackIngress
from slack_gateway import gateway_server
from test_bridge import CHANNEL, CONFIG
from test_gateway import ScriptedSlack


class SlackHistoryMcpTests(unittest.TestCase):
    def test_progressive_search_finds_old_thread_replies_without_writes_or_other_channel_access(self):
        web = ScriptedSlack()
        web.history = [
            {'ts': '050.0', 'user': 'UFIRST', 'text': 'Earlier investigation'},
            {'ts': '051.0', 'thread_ts': '050.0', 'user': 'USECOND', 'text': 'The baseline was TUESDAY.'},
            {'ts': '200.0', 'user': 'UFIRST', 'text': 'An unrelated recent discussion'},
        ]
        with tempfile.TemporaryDirectory(prefix='slack-mcp-') as directory, ExitStack() as cleanup:
            with patch('slack_bridge.SocketModeClient'):
                ingress = SlackIngress(web, CONFIG, 'synthetic-token')
            server = cleanup.enter_context(gateway_server(None, Path(directory) / 'state', web,
                CONFIG, ingress, iap_host='127.0.0.1', port=0))
            threading.Thread(target=server.serve_forever, daemon=True).start()
            cleanup.callback(server.shutdown)
            with subprocess.Popen([sys.executable, str(Path(__file__).with_name('slack_mcp.py')),
                    '--iap-port', str(server.socket.getsockname()[1])], stdin=subprocess.PIPE,
                    stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, bufsize=1) as client:
                received = queue.Queue()
                output = threading.Thread(target=lambda: [received.put(json.loads(line))
                    for line in client.stdout], daemon=True)
                output.start()

                def request(method, params=None):
                    client.stdin.write(json.dumps({'jsonrpc': '2.0', 'id': 1,
                                                   'method': method, 'params': params or {}}) + '\n')
                    client.stdin.flush()
                    return received.get(timeout=5)

                def call(name, arguments):
                    response = request('tools/call', {'name': name, 'arguments': arguments})['result']
                    self.assertFalse(response.get('isError'), response)
                    return json.loads(response['content'][0]['text'])

                initialized = request('initialize', {'protocolVersion': '2025-03-26'})['result']
                self.assertEqual(initialized['protocolVersion'], '2025-03-26')
                self.assertIn('tools', initialized['capabilities'])
                tools = request('tools/list')['result']['tools']
                self.assertTrue(all(tool['annotations']['readOnlyHint'] for tool in tools))
                recent = call('read_channel_history', {})
                self.assertEqual(recent['channel_id'], CHANNEL)
                self.assertEqual(recent['messages'][0]['text'], 'An unrelated recent discussion')
                old = call('read_channel_history', {'cursor': recent['next_cursor']})
                self.assertEqual(old['messages'][0]['ts'], '050.0')
                self.assertFalse(old['next_cursor'])
                thread = call('read_thread', {'thread_ts': '050.0'})
                reply = call('read_thread', {'thread_ts': '050.0', 'cursor': thread['next_cursor']})
                self.assertEqual((reply['messages'][0]['user'], reply['messages'][0]['text']),
                                 ('USECOND', 'The baseline was TUESDAY.'))
                first = call('search_channel_history', {'query': 'tuesday'})
                self.assertEqual(first['messages'], [])
                self.assertFalse(first['complete'], 'An empty partial search was reported as exhaustive')
                found = call('search_channel_history', {'query': 'tuesday', 'cursor': first['next_cursor']})
                self.assertTrue(found['complete'])
                self.assertEqual([message['ts'] for message in found['messages']], ['051.0'])
                for name, arguments in (
                        ('read_channel_history', {'channel': 'PRIVATEOTHER'}),
                        ('chat_postMessage', {'text': 'This must never be sent'}),
                        ('read_thread', {'thread_ts': '050.0', 'limit': 0})):
                    error = request('tools/call', {'name': name, 'arguments': arguments})['result']
                    self.assertTrue(error['isError'])
                self.assertEqual(web.messages, [])
                self.assertEqual(web.updates, [])
                client.stdin.close()
                self.assertEqual(client.wait(timeout=10), 0, client.stderr.read())
                output.join(timeout=5)


if __name__ == '__main__':
    unittest.main()
