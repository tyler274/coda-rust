"""Transport-boundary regressions; these tests do not claim live provider proof."""
from pathlib import Path
from contextlib import closing
import sqlite3
import tempfile
import threading
import unittest

from slack_sdk.errors import SlackApiError

from codex_rpc import Rpc, RpcError
from slack_bridge import Bridge, State
from native_codex import native_runs

CHANNEL, OWNER = 'CSECONDWORKSPACE', 'USECONDOWNER'
CONFIG = {'slack_team': 'TSECONDWORKSPACE', 'slack_channel': CHANNEL, 'slack_owner': OWNER,
          'timezone': 'Europe/London', 'availability': 'Available during the chosen working hours.',
          'automations': {'custom-review': 'Project review'}}


class Slack:
    def __init__(self, fail=False):
        self.messages = []
        self.fail = fail
        self.updates = []
        self.reactions = []

    def users_info(self, user):
        return {'user': {'id': user, 'is_bot': user == 'UAGENT',
                         'profile': {'display_name': {'UFIRST': 'Robin', 'USECOND': 'Sam',
                                                     'UTEAMMATE': 'Casey'}.get(user, 'Alex')}}}

    def conversations_info(self, channel):
        return {'channel': {'id': channel, 'name': 'shared-engineering', 'is_private': True,
                            'is_shared': False, 'is_ext_shared': False}}

    def conversations_members(self, **request):
        return {'members': ['UFIRST', 'USECOND', 'USILENT', 'UAGENT']}

    def conversations_replies(self, **request):
        return {'messages': [{'ts': request['ts'], 'reactions': self.reactions}]}

    def reactions_add(self, **request):
        self.reactions.append({'name': request['name'], 'users': ['UAGENT']})
        return {'ok': True}

    def reactions_remove(self, **request):
        self.reactions = [r for r in self.reactions if not (r['name'] == request['name'] and 'UAGENT' in r['users'])]
        return {'ok': True}

    def chat_postMessage(self, **message):
        self.messages.append(message)
        if self.fail:
            raise TimeoutError('connection lost after submission')
        return {'ok': True, 'channel': CHANNEL, 'message': {'text': message['text']}, 'ts': '100.0'}

    def chat_update(self, **message):
        self.updates.append(message)
        return {'ok': True, 'channel': message['channel'], 'ts': message['ts']}


class CodexBoundary:
    def __init__(self):
        self.started = []
        self.replies = []
        self.turns = []

    def call(self, method, params):
        if method in ('thread/read', 'thread/resume'):
            return {'thread': {'id': params['threadId'], 'turns': self.turns}}
        if method == 'turn/start':
            self.started.append(params)
            return {'turn': {'id': 'native-turn', 'status': 'inProgress', 'items': []}}
        raise AssertionError(method)

    def reply(self, ident, result):
        self.replies.append((ident, result))


class RoutingTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.state = State(Path(self.directory.name))
        self.state.data['routes']['native-thread'] = {
            'root': '100.0', 'cursor': '100.0', 'delivered': {}, 'pending_input': None,
            'thread_id': 'native-thread', 'automation_id': 'custom-review', 'created_at': 0,
        }
        self.state.save()
        self.slack, self.codex = Slack(), CodexBoundary()
        self.bridge = Bridge(self.state, self.slack, self.codex, CONFIG)
        self.bridge.bot_user = 'UAGENT'
        self.message = {'type': 'message', 'channel': CHANNEL, 'thread_ts': '100.0', 'user': OWNER,
                        'ts': '101.0', 'text': '<@UAGENT> Continue this run'}

    def tearDown(self):
        self.directory.cleanup()

    def test_channel_participants_continue_the_exact_thread_with_attribution_and_replays_are_ignored(self):
        for change in ({'channel': 'another-channel'}, {'user': ''}, {'bot_id': 'bot'}, {'thread_ts': 'other'},
                       {'subtype': 'message_changed'},
                       {'text': 'Dev, can the release wait?'}):
            self.bridge.receive('native-thread', {**self.message, **change})
        self.assertEqual(self.codex.started, [])
        self.message['user'] = 'UTEAMMATE'
        self.bridge.receive('native-thread', self.message)
        self.assertEqual(len(self.codex.started), 1)
        self.assertEqual(self.codex.started[0]['threadId'], 'native-thread')
        self.assertIn('<@UTEAMMATE>', self.codex.started[0]['input'][0]['text'])
        self.assertIn('Continue this run', self.codex.started[0]['input'][0]['text'])
        restarted = Bridge(State(Path(self.directory.name)), self.slack, self.codex, CONFIG)
        restarted.bot_user = 'UAGENT'
        restarted.receive('native-thread', self.message)
        self.assertEqual(len(self.codex.started), 1)

    def test_ordinary_approve_wording_is_a_mention_and_not_a_command(self):
        self.bridge.receive('native-thread', {**self.message, 'text': '<@UAGENT> approve the migration plan'})
        self.assertEqual(len(self.codex.started), 1)
        self.assertIn('approve the migration plan', self.codex.started[0]['input'][0]['text'])
        self.assertFalse(any('No approval was applied' in message['text'] for message in self.slack.messages))
        self.bridge.receive('native-thread', {**self.message, 'ts': '102.0', 'text': 'approve the release notes'})
        self.assertEqual(len(self.codex.started), 1)
        self.assertEqual(self.state.data['routes']['native-thread']['cursor'], '101.0')

    def test_a_request_shaped_token_still_cannot_become_model_input(self):
        self.bridge.receive('native-thread', {**self.message, 'text': 'approve deadbeef'})
        self.assertEqual(self.codex.started, [])
        self.assertIn('No approval was applied', self.slack.messages[0]['text'])
        self.assertEqual(self.state.data['routes']['native-thread']['cursor'], '101.0')

    def test_an_edited_history_mention_starts_one_turn(self):
        edited = {**self.message, 'edited': {'user': OWNER, 'ts': '101.5'}}
        self.bridge.receive('native-thread', edited)
        self.assertEqual(len(self.codex.started), 1)
        self.bridge.receive('native-thread', edited)
        self.assertEqual(len(self.codex.started), 1)

    def test_an_edited_mention_can_start_a_conversation(self):
        class Started(CodexBoundary):
            def call(self, method, params):
                if method == 'thread/start':
                    self.started.append(params)
                    return {'thread': {'id': 'from-edit', 'createdAt': 200, 'turns': []}}
                if method == 'turn/start':
                    self.turns.append(params)
                    return {'turn': {'id': 'accepted-turn', 'status': 'inProgress', 'items': []}}
                if method in ('thread/read', 'thread/resume'):
                    return {'thread': {'id': params['threadId'], 'turns': []}}
                raise AssertionError(method)

        native = Started()
        config = {**CONFIG, 'new_conversations': {'cwd': '/workspace/project'}}
        bridge = Bridge(self.state, self.slack, native, config)
        bridge.bot_user = 'UAGENT'
        bridge.start_from_mention({'type': 'message', 'channel': CHANNEL, 'ts': '200.0', 'user': OWNER,
                                  'text': '<@UAGENT> A corrected request', 'edited': {'user': OWNER, 'ts': '200.5'}})
        self.assertEqual(len(native.started), 1)
        self.assertIn('from-edit', self.state.data['routes'])

    def test_unknown_post_outcome_is_not_resent_after_restart(self):
        bridge = Bridge(self.state, Slack(fail=True), self.codex, CONFIG)
        with self.assertRaises(TimeoutError):
            bridge.post('Actual native answer', '100.0')
        restarted = Bridge(State(Path(self.directory.name)), self.slack, self.codex, CONFIG)
        with self.assertRaisesRegex(RuntimeError, 'unknown outcome'):
            restarted.post('Actual native answer', '100.0')
        self.assertEqual(self.slack.messages, [])

    def test_a_slack_refusal_does_not_block_the_next_delivery(self):
        class Refused(Slack):
            def chat_postMessage(self, **message):
                raise SlackApiError('invalid_blocks', {'ok': False, 'error': 'invalid_blocks'})

        bridge = Bridge(self.state, Refused(), self.codex, CONFIG)
        with self.assertRaises(SlackApiError):
            bridge.post('Status', '100.0')
        self.assertIsNone(self.state.data['pending_post'])
        delivered = Bridge(self.state, self.slack, self.codex, CONFIG)
        self.assertEqual(delivered.post('Status', '100.0'), '100.0')

    def test_unknown_conversation_start_does_not_create_another_native_thread(self):
        class LostStart(CodexBoundary):
            def call(self, method, params):
                if method == 'thread/start':
                    self.started.append(params)
                    raise TimeoutError('Native created the conversation before the connection broke')
                return super().call(method, params)

        config = {**CONFIG, 'new_conversations': {'cwd': '/workspace/project'}}
        native = LostStart()
        message = {'type': 'message', 'channel': CHANNEL, 'ts': '200.0',
                   'user': OWNER, 'text': '<@UAGENT> A new request'}
        bridge = Bridge(self.state, self.slack, native, config)
        bridge.bot_user = 'UAGENT'
        with self.assertRaises(TimeoutError):
            bridge.start_from_mention(message)
        restarted = Bridge(State(Path(self.directory.name)), self.slack, native, config)
        restarted.bot_user = 'UAGENT'
        with self.assertRaisesRegex(RuntimeError, 'unknown outcome'):
            restarted.start_from_mention(message)
        self.assertEqual(len(native.started), 1)

    def test_a_rejected_conversation_start_can_be_retried(self):
        class Rejected(CodexBoundary):
            def call(self, method, params):
                if method == 'thread/start':
                    self.started.append(params)
                    raise RpcError(method, {'code': -32600, 'message': 'cwd is not available'})
                return super().call(method, params)

        native = Rejected()
        config = {**CONFIG, 'new_conversations': {'cwd': '/workspace/project'}}
        message = {'type': 'message', 'channel': CHANNEL, 'ts': '200.0',
                   'user': OWNER, 'text': '<@UAGENT> A new request'}
        bridge = Bridge(self.state, self.slack, native, config)
        bridge.bot_user = 'UAGENT'
        with self.assertRaises(RpcError):
            bridge.start_from_mention(message)
        self.assertIsNone(self.state.data.get('pending_start'))
        retried = Bridge(State(Path(self.directory.name)), self.slack, CodexBoundary(), config)
        retried.codex.call = lambda method, params: (
            {'thread': {'id': 'retried', 'createdAt': 200, 'turns': []}} if method == 'thread/start'
            else {'turn': {'id': 'accepted-turn', 'status': 'inProgress', 'items': []}})
        retried.bot_user = 'UAGENT'
        retried.start_from_mention(message)
        self.assertIn('retried', retried.state.data['routes'])

    def test_a_lost_conversation_start_stays_unresolved(self):
        class Lost(CodexBoundary):
            def call(self, method, params):
                if method == 'thread/start':
                    raise RpcError(method, {'message': 'Codex connection closed', 'lost': True})
                return super().call(method, params)

        config = {**CONFIG, 'new_conversations': {'cwd': '/workspace/project'}}
        bridge = Bridge(self.state, self.slack, Lost(), config)
        bridge.bot_user = 'UAGENT'
        message = {'type': 'message', 'channel': CHANNEL, 'ts': '200.0',
                   'user': OWNER, 'text': '<@UAGENT> A new request'}
        with self.assertRaises(RpcError) as caught:
            bridge.start_from_mention(message)
        self.assertTrue(caught.exception.lost)
        restarted = Bridge(State(Path(self.directory.name)), self.slack, Lost(), config)
        restarted.bot_user = 'UAGENT'
        with self.assertRaisesRegex(RuntimeError, 'unknown outcome'):
            restarted.start_from_mention(message)

    def test_new_turn_status_uses_native_response_before_session_file_is_readable(self):
        class NativeBeforeSave(CodexBoundary):
            def call(self, method, params):
                if method == 'thread/start':
                    return {'thread': {'id': 'new-native', 'createdAt': 200, 'turns': []}}
                if method == 'turn/start':
                    return {'turn': {'id': 'accepted-turn', 'status': 'inProgress', 'items': []}}
                raise RuntimeError('Native session file is not yet readable')

        config = {**CONFIG, 'new_conversations': {'cwd': '/workspace/project'}}
        bridge = Bridge(self.state, self.slack, NativeBeforeSave(), config)
        bridge.bot_user = 'UAGENT'
        bridge.start_from_mention({'type': 'message', 'channel': CHANNEL, 'ts': '200.0',
                                  'user': OWNER, 'text': '<@UAGENT> A new request'})
        self.assertIn('Last observed: Working', self.slack.messages[0]['text'])
        self.assertIsNone(self.state.data['routes']['new-native']['pending_input'])

    def test_lost_input_acknowledgement_does_not_repeat_the_native_turn_after_restart(self):
        class LostAcknowledgement(CodexBoundary):
            def call(self, method, params):
                result = super().call(method, params)
                if method == 'turn/start':
                    raise TimeoutError('Native accepted the turn before the connection broke')
                return result

        native = LostAcknowledgement()
        bridge = Bridge(self.state, self.slack, native, CONFIG)
        bridge.bot_user = 'UAGENT'
        with self.assertRaises(TimeoutError):
            bridge.receive('native-thread', self.message)
        restarted = Bridge(State(Path(self.directory.name)), self.slack, native, CONFIG)
        restarted.bot_user = 'UAGENT'
        with self.assertRaisesRegex(RuntimeError, 'Uncertain input delivery'):
            restarted.receive('native-thread', self.message)
        self.assertEqual(len(native.started), 1)

    def test_a_rejected_turn_does_not_block_the_route(self):
        class Rejected(CodexBoundary):
            def call(self, method, params):
                if method == 'turn/start':
                    raise RpcError(method, {'message': 'turn rejected'})
                return super().call(method, params)

        bridge = Bridge(self.state, self.slack, Rejected(), CONFIG)
        bridge.bot_user = 'UAGENT'
        with self.assertRaises(RpcError):
            bridge.receive('native-thread', self.message)
        self.assertIsNone(self.state.data['routes']['native-thread']['pending_input'])
        bridge.codex = CodexBoundary()
        later = {**self.message, 'ts': '102.0'}
        bridge.receive('native-thread', later)
        self.assertEqual(len(bridge.codex.started), 1)

    def test_stale_or_cross_thread_approval_never_becomes_model_input(self):
        self.bridge.requests['request'] = {'id': 5, 'method': 'item/commandExecution/requestApproval',
                                           'params': {'threadId': 'different-thread'}}
        self.bridge.receive('native-thread', {**self.message, 'text': 'approve request'})
        self.assertEqual(self.codex.replies, [])
        self.assertEqual(self.codex.started, [])
        self.assertIn('No approval was applied', self.slack.messages[0]['text'])

    def test_status_comes_from_native_turn_and_updates_the_existing_message(self):
        self.codex.turns = [{'id': 'turn', 'status': 'inProgress', 'items': []}]
        self.bridge.deliver('native-thread')
        self.assertIn('Working', self.slack.updates[-1]['text'])
        self.assertEqual(self.slack.reactions, [{'name': 'eyes', 'users': ['UAGENT']}])
        self.slack.reactions.append({'name': 'eyes', 'users': ['UHUMAN']})
        self.slack.reactions.append({'name': 'thumbsup', 'users': ['UAGENT']})
        self.bridge.server_event({'id': 7, 'method': 'item/commandExecution/requestApproval',
                                 'params': {'threadId': 'native-thread', 'command': 'reviewed command'}})
        self.assertIn('Needs your input', self.slack.updates[-1]['text'])
        self.assertIn({'name': 'question', 'users': ['UAGENT']}, self.slack.reactions)
        self.assertNotIn({'name': 'eyes', 'users': ['UAGENT']}, self.slack.reactions)
        self.codex.turns[0].update(status='failed', error={'message': 'Provider unavailable'})
        self.bridge.server_event({'method': 'turn/completed', 'params': {'threadId': 'native-thread'}})
        self.assertIn('Run failed', self.slack.updates[-1]['text'])
        self.assertIn({'name': 'warning', 'users': ['UAGENT']}, self.slack.reactions)
        self.assertEqual(self.bridge.requests, {})
        self.codex.turns.append({'id': 'second', 'status': 'completed', 'items': [
            {'type': 'agentMessage', 'phase': 'final_answer', 'text': 'Two critical issues remain.'}]})
        self.bridge.deliver('native-thread')
        self.assertIn('Report ready', self.slack.updates[-1]['text'])
        self.assertEqual(self.slack.reactions, [{'name': 'eyes', 'users': ['UHUMAN']},
                                              {'name': 'thumbsup', 'users': ['UAGENT']},
                                              {'name': 'white_check_mark', 'users': ['UAGENT']}])
        self.assertNotIn('All clear', self.slack.updates[-1]['text'])
        self.assertTrue(all(m['channel'] == CHANNEL and m['ts'] == '100.0' for m in self.slack.updates))
        self.assertTrue(all(m['thread_ts'] == '100.0' for m in self.slack.messages))
        before = len(self.slack.updates), len(self.slack.messages)
        self.bridge.deliver('native-thread')
        self.assertEqual((len(self.slack.updates), len(self.slack.messages)), before)

    def test_native_discovery_accepts_a_different_number_of_selected_schedules(self):
        database = Path(self.directory.name) / 'native.db'
        with closing(sqlite3.connect(database)) as db, db:
            db.executescript('CREATE TABLE automations (id TEXT, name TEXT); '
                             'CREATE TABLE automation_runs (thread_id TEXT, automation_id TEXT, created_at INTEGER);')
            for number in range(4):
                db.execute('INSERT INTO automations VALUES (?, ?)', (f'job{number}', f'Job {number}'))
                db.execute('INSERT INTO automation_runs VALUES (?, ?, ?)', (f'thread{number}', f'job{number}', number))
        self.assertEqual([r['thread_id'] for r in native_runs(0, ['job2'], database)], ['thread2'])
        self.assertEqual(len(native_runs(1, ['job0', 'job1', 'job2', 'job3'], database)), 3)

    def test_approval_is_for_one_live_request_and_never_persistent(self):
        self.bridge.requests['request'] = {'id': 5, 'method': 'item/commandExecution/requestApproval',
                                           'params': {'threadId': 'native-thread'}}
        self.bridge.receive('native-thread', {**self.message, 'channel': 'another-channel',
                                             'text': 'approve request'})
        self.assertEqual(self.codex.replies, [])
        self.assertIn('request', self.bridge.requests)
        self.bridge.receive('native-thread', {**self.message, 'user': 'UTEAMMATE', 'text': 'approve request'})
        self.assertEqual(self.codex.replies, [(5, {'decision': 'accept'})])
        self.assertEqual(self.state.data['routes']['native-thread']['last_input']['user'], 'UTEAMMATE')
        self.bridge.receive('native-thread', {**self.message, 'ts': '102.0', 'text': 'approve request'})
        self.assertEqual(len(self.codex.replies), 1)
        self.assertEqual(self.codex.started, [])


class ClosedSocket:
    def __init__(self):
        self.sent = threading.Event()

    def send(self, raw):
        self.sent.set()

    def close(self):
        self.sent.set()

    def __iter__(self):
        if not self.sent.wait(2):
            raise AssertionError('RPC send did not happen')
        return iter(())


class TransportTests(unittest.TestCase):
    def test_a_closed_socket_is_a_lost_acknowledgement(self):
        rpc = Rpc(ClosedSocket())
        self.addCleanup(rpc.close)
        with self.assertRaises(RpcError) as caught:
            rpc.call('thread/start', {}, timeout=2)
        self.assertTrue(caught.exception.lost)


if __name__ == '__main__':
    unittest.main()
