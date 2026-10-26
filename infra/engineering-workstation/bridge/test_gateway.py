"""Exercise gateway/connector transport with scripted external provider boundaries.

This protects real socket, reconnect and receipt behavior, not model quality or
Slack's hosted service. Live native readback is a separate deployment check.
"""
from contextlib import ExitStack
from pathlib import Path
import queue
import re
import tempfile
import threading
from types import SimpleNamespace
import unittest
from unittest.mock import patch

from websockets.sync.client import connect as websocket_connect, unix_connect

from slack_bridge import SlackIngress
from slack_gateway import gateway_server
from workstation_connector import relay
from test_bridge import CONFIG, CHANNEL, Slack


class ScriptedSlack(Slack):
    def __init__(self):
        super().__init__()
        self.changed = threading.Condition()
        self.history = []

    def auth_test(self):
        class Response(dict):
            headers = {'x-oauth-scopes': 'chat:write,groups:history,reactions:write'}
        return Response(team_id=CONFIG['slack_team'], bot_id='BAGENT', user_id='UAGENT')

    def chat_postMessage(self, **message):
        with self.changed:
            result = super().chat_postMessage(**message)
            self.changed.notify_all()
            return result

    def conversations_replies(self, **request):
        if request.get('limit') == 1:
            return super().conversations_replies(**request)
        messages = sorted((event for event in self.history
                           if event['ts'] > request.get('oldest', '0')
                           and event['ts'] <= request.get('latest', '9999999999')
                           and (event['ts'] == request['ts'] or event.get('thread_ts') == request['ts'])),
                          key=lambda event: event['ts'])
        return self.page(messages, request)

    def conversations_history(self, **request):
        messages = sorted((dict(event) for event in self.history if event['ts'] > request.get('oldest', '0')
                           and event.get('thread_ts') in (None, event['ts'])),
                          key=lambda event: event['ts'], reverse=True)
        for message in messages:
            replies = [event for event in self.history if event.get('thread_ts') == message['ts']
                       and event['ts'] != message['ts']]
            if replies:
                message.update(reply_count=len(replies), latest_reply=max(event['ts'] for event in replies))
        return self.page(messages, request)

    @staticmethod
    def page(messages, request):
        offset = int(request.get('cursor') or '0')
        return {'messages': messages[offset:offset + 1],
                'response_metadata': {'next_cursor': str(offset + 1) if offset + 1 < len(messages) else ''}}

    def await_posts(self, count):
        with self.changed:
            return self.changed.wait_for(lambda: len(self.messages) >= count, timeout=5)


class ScriptedNative:
    """A native conversation owner whose state outlives individual connections."""
    def __init__(self, turns, threads=None):
        self.threads = threads if threads is not None else {
            'native-review': {'id': 'native-review', 'name': 'Gateway connection test',
                              'createdAt': 0, 'turns': turns}}
        self.events = queue.Queue()
        self.closed = threading.Event()
        self.calls = queue.Queue()
        self.replies = queue.Queue()

    def runs(self, since, automations):
        return ([{'thread_id': 'native-review', 'automation_id': 'custom-review', 'created_at': 0}]
                if 'native-review' in self.threads else [])

    def archived(self, ident):
        return False

    def call(self, method, params):
        if method in ('thread/read', 'thread/resume'):
            return {'thread': self.threads[params['threadId']]}
        self.calls.put((method, params))
        if method == 'thread/start':
            ident = f'native-created-{len(self.threads) + 1}'
            self.threads[ident] = {'id': ident, 'createdAt': 200, 'turns': []}
            return {'thread': self.threads[ident]}
        if method == 'turn/start':
            self.threads[params['threadId']]['turns'].append(
                {'id': 'continued-turn', 'status': 'inProgress', 'items': []})
            return {'turn': self.threads[params['threadId']]['turns'][-1]}
        return {'turn': {'id': 'continued-turn'}}

    def reply(self, ident, result):
        self.replies.put((ident, result))

    def close(self):
        self.closed.set()


class GatewayTests(unittest.TestCase):
    def test_slack_mentions_create_native_conversations_and_recover_offline_without_duplicates(self):
        config = {**CONFIG, 'new_conversations': {'cwd': '/workspace/project'}}
        with tempfile.TemporaryDirectory(prefix='gateway-', dir='/tmp') as directory, ExitStack() as cleanup:
            directory = Path(directory)
            web, threads = ScriptedSlack(), {}
            with patch('slack_bridge.SocketModeClient'):
                ingress = SlackIngress(web, config, 'synthetic-token')
            with patch('slack_bridge.time.time', return_value=100):
                server = cleanup.enter_context(gateway_server(None, directory / 'state', web, config,
                                                              ingress, iap_host='127.0.0.1', port=0))
            threading.Thread(target=server.serve_forever, daemon=True).start()
            cleanup.callback(server.shutdown)

            def connect():
                native = ScriptedNative([], threads)
                connection = cleanup.enter_context(websocket_connect(
                    f'ws://127.0.0.1:{server.socket.getsockname()[1]}', proxy=None, open_timeout=2))
                worker = threading.Thread(target=relay, args=(connection, native), daemon=True)
                worker.start()
                cleanup.callback(connection.close)
                return native, connection, worker

            # A mention in a thread whose root predates gateway setup must be
            # discovered after reconnect, with paginated prior discussion.
            root = {'type': 'message', 'channel': CHANNEL, 'ts': '050.0',
                    'user': 'UFIRST', 'text': 'The chosen baseline is Tuesday.'}
            context = {**root, 'ts': '060.0', 'thread_ts': root['ts'],
                       'user': 'USECOND', 'text': 'Compare the two runner configurations.'}
            first = {'type': 'message', 'channel': CHANNEL, 'ts': '200.0', 'thread_ts': root['ts'],
                     'user': 'UFIRST', 'text': '<@UAGENT> First request'}
            second = {**root, 'ts': '201.0', 'user': 'USECOND', 'text': '<@UAGENT> Second request'}
            web.history.extend([root, context, first, second])
            native, connection, worker = connect()
            for index, person in ((1, 'UFIRST'), (2, 'USECOND')):
                method, params = native.calls.get(timeout=5)
                self.assertEqual((method, params), ('thread/start', {'cwd': '/workspace/project'}))
                method, params = native.calls.get(timeout=5)
                self.assertEqual((method, params['threadId']), ('turn/start', f'native-created-{index}'))
                self.assertIn(f'<@{person}>', params['input'][0]['text'])
                if index == 1:
                    self.assertIn('The chosen baseline is Tuesday.', params['input'][0]['text'])
                    self.assertIn('Compare the two runner configurations.', params['input'][0]['text'])
            self.assertTrue(web.await_posts(2))
            self.assertEqual({message['thread_ts'] for message in web.messages}, {root['ts'], '201.0'})
            self.assertTrue(all(message['ts'] not in (root['ts'], '201.0') for message in web.updates))
            self.assertEqual(first['text'], '<@UAGENT> First request')
            connection.close()
            worker.join(timeout=5)
            self.assertFalse(worker.is_alive())

            quiet = {**first, 'ts': '202.0', 'user': 'UTEAMMATE', 'text': 'Dev, the release can wait.'}
            reply = {**quiet, 'ts': '203.0', 'text': '<@UAGENT> Also this'}
            web.history.extend([quiet, reply])
            native, connection, worker = connect()
            method, params = native.calls.get(timeout=5)
            self.assertEqual((method, params['threadId']), ('turn/steer', 'native-created-1'))
            self.assertIn('<@UTEAMMATE>', params['input'][0]['text'])
            self.assertIn('Dev, the release can wait.', params['input'][0]['text'])
            self.assertIn('Also this', params['input'][0]['text'])
            self.assertEqual(len(threads), 2)
            self.assertEqual(len(web.messages), 2)

            # Duplicate socket delivery must still create only one conversation.
            live = {**root, 'ts': '204.0', 'text': '<@UAGENT> Live request'}
            web.history.append(live)
            for _ in range(2):
                ingress.receive(ingress.client, SimpleNamespace(type='events_api', envelope_id='live',
                    payload={'team_id': CONFIG['slack_team'], 'event': live}))
            self.assertEqual(native.calls.get(timeout=5)[0], 'thread/start')
            self.assertEqual(native.calls.get(timeout=5)[0], 'turn/start')
            self.assertTrue(web.await_posts(3))
            self.assertEqual(len(threads), 3)
            connection.close()
            worker.join(timeout=5)
            self.assertFalse(worker.is_alive())

    def test_reconnect_preserves_report_and_native_session_then_two_people_can_contribute(self):
        self.check_reconnect(tcp=False)

    def test_tcp_reconnect_preserves_report_and_native_session_then_two_people_can_contribute(self):
        # The local TCP hop models the IAP tunnel's endpoints. Actual IAM and
        # firewall enforcement require the separate deployed connection check.
        self.check_reconnect(tcp=True)

    def check_reconnect(self, tcp):
        with tempfile.TemporaryDirectory(prefix='gateway-', dir='/tmp') as directory, ExitStack() as cleanup:
            directory = Path(directory)
            web = ScriptedSlack()
            with patch('slack_bridge.SocketModeClient'):
                ingress = SlackIngress(web, CONFIG, 'synthetic-token')
            ingress.client.connect()
            socket = directory / 'worker.sock'
            server = cleanup.enter_context(gateway_server(None if tcp else socket, directory / 'state',
                                                          web, CONFIG, ingress, thread_id='native-review',
                                                          iap_host='127.0.0.1' if tcp else None, port=0))
            serving = threading.Thread(target=server.serve_forever, daemon=True)
            serving.start()
            cleanup.callback(server.shutdown)
            turns = [{'id': 'saved-turn', 'status': 'completed', 'items': [
                {'type': 'agentMessage', 'phase': 'final_answer', 'text': 'The saved review found an obsolete dependency.'}]}]

            def connect():
                native = ScriptedNative(turns)
                context = (websocket_connect(f'ws://127.0.0.1:{server.socket.getsockname()[1]}',
                                             proxy=None, open_timeout=2) if tcp else
                           unix_connect(path=str(socket), open_timeout=2))
                connection = cleanup.enter_context(context)
                worker = threading.Thread(target=relay, args=(connection, native), daemon=True)
                worker.start()
                cleanup.callback(connection.close)
                return native, connection, worker

            native, connection, worker = connect()
            self.assertTrue(web.await_posts(2), 'The saved native report did not reach the Slack boundary')
            self.assertIn('Gateway connection test', web.messages[0]['text'])
            self.assertNotIn('Project review', web.messages[0]['text'])
            self.assertIn('obsolete dependency', web.messages[1]['text'])
            connection.close()
            worker.join(timeout=5)
            self.assertFalse(worker.is_alive())
            self.assertTrue(native.closed.wait(2))

            # Slack owns the offline message. The gateway must recover it through
            # history, not through a test-injected delivery acknowledgement.
            offline = {'type': 'message', 'channel': CHANNEL, 'thread_ts': '100.0',
                       'ts': '101.0', 'user': 'UFIRST', 'text': '<@UAGENT> Explain the dependency finding'}
            web.history.append(offline)
            native, connection, worker = connect()
            method, params = native.calls.get(timeout=5)
            self.assertEqual(method, 'turn/start')
            self.assertEqual(params['threadId'], 'native-review')
            self.assertIn('<@UFIRST>', params['input'][0]['text'])
            self.assertIn('Explain the dependency finding', params['input'][0]['text'])
            self.assertEqual(len(web.messages), 2, 'Reconnect reposted the saved report')

            def event(text, timestamp, user='USECOND', team=CONFIG['slack_team']):
                value = {**offline, 'ts': timestamp, 'user': user, 'text': text}
                web.history.append(value)
                request = SimpleNamespace(type='events_api', envelope_id=timestamp,
                                          payload={'team_id': team, 'event': value})
                ingress.receive(ingress.client, request)

            event('<@UAGENT> Also check the version constraints', '102.0')
            method, params = native.calls.get(timeout=5)
            self.assertEqual(method, 'turn/steer')
            self.assertEqual((params['threadId'], params['expectedTurnId']), ('native-review', 'continued-turn'))
            self.assertIn('<@USECOND>', params['input'][0]['text'])

            native.events.put({'id': 87, 'method': 'item/commandExecution/requestApproval',
                               'params': {'threadId': 'native-review', 'command': 'reviewed command'}})
            self.assertTrue(web.await_posts(3))
            token = re.search(r'approve ([0-9a-f]+)', web.messages[-1]['text'])[1]
            event('approve ' + token, '103.0')
            self.assertEqual(native.replies.get(timeout=5), (87, {'decision': 'accept'}))
            ingress.client.connect.assert_called_once()
            connection.close()
            worker.join(timeout=5)
            self.assertFalse(worker.is_alive())


if __name__ == '__main__':
    unittest.main()
