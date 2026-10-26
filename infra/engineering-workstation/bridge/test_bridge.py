"""Transport-boundary regressions; these tests do not claim live provider proof."""
from pathlib import Path
from contextlib import closing
import sqlite3
import tempfile
import unittest

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
                       {'subtype': 'message_changed'}, {'edited': {'ts': '102.0'}},
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

    def test_unknown_post_outcome_is_not_resent_after_restart(self):
        bridge = Bridge(self.state, Slack(fail=True), self.codex, CONFIG)
        with self.assertRaises(TimeoutError):
            bridge.post('Actual native answer', '100.0')
        restarted = Bridge(State(Path(self.directory.name)), self.slack, self.codex, CONFIG)
        with self.assertRaisesRegex(RuntimeError, 'unknown outcome'):
            restarted.post('Actual native answer', '100.0')
        self.assertEqual(self.slack.messages, [])

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


if __name__ == '__main__':
    unittest.main()
