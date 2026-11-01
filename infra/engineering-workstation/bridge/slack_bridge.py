#!/usr/bin/env python3
"""Route shared Slack threads to native Codex conversations and scheduled runs.

Codex owns schedules, conversations, tools, approvals and credentials. Slack
owns messages received while the VM is off. This process persists routing IDs
and delivery cursors only, and never reads or refreshes Codex auth files.
"""
from datetime import datetime
import json
import os
import queue
import re
import threading
import time
import uuid
from zoneinfo import ZoneInfo

from slack_sdk.socket_mode import SocketModeClient
from slack_sdk.socket_mode.response import SocketModeResponse
from slack_sdk.errors import SlackApiError

from codex_rpc import RpcError
from slack_history import SlackHistory
from slack_interactions import (APPROVALS, QUESTIONS, action_block, answer_modal, approval_response,
                                async_questions, request_text, tool_approval)
from slack_render import STATUS_REACTIONS, chunks, markdown_payload, render_native, run_status, thread_root_payload

CONTROL_WORDS = frozenset(('approve', 'deny', 'answer'))
REQUEST_TOKEN = re.compile(r'[0-9a-fA-F]{8}\Z')


def human_message(event, channel):
    # Slack history keeps an edited message under its original timestamp and
    # sets `edited`. A live edit arrives separately as subtype message_changed.
    return (event.get('type', 'message') == 'message'
            and event.get('channel') == channel and bool(event.get('user'))
            and not event.get('subtype')
            and not event.get('bot_id')
            and bool(event.get('text', '').strip()))


def control_command(words, requests):
    """Approve, deny, and answer apply only to a pending request token."""
    if len(words) < 2 or words[0].lower() not in CONTROL_WORDS:
        return False
    token = words[1]
    return token in requests or REQUEST_TOKEN.fullmatch(token) is not None


def definite_rejection(error):
    return isinstance(error, RpcError) and not error.lost


def admitted(event, root, channel, slack_started=False):
    return human_message(event, channel) and (
        (event.get('thread_ts') == root and event.get('ts') != root)
        or (slack_started and event.get('ts') == root
            and event.get('thread_ts') in (None, root)))


def final_text(turn):
    # A decision can follow its review packet in commentary. Deliver the
    # agent-visible context with the question, rather than dropping the patch.
    questions = async_questions(turn)
    messages = [i['text'] for i in turn.get('items', [])
                if i.get('type') == 'agentMessage' and i.get('text')
                and (questions or i.get('phase') == 'final_answer')]
    if messages:
        prefix = f'Codex turn status: {turn["status"]}.\n\n' if turn['status'] != 'completed' else ''
        return prefix + '\n\n'.join(messages)
    error = turn.get('error') or {}
    detail = f'\n{error["message"]}' if error.get('message') else ''
    return f'The run ended with status {turn["status"]}; no final answer was saved.{detail}'


class State:
    def __init__(self, directory, new_conversations=False):
        directory.mkdir(mode=0o700, parents=True, exist_ok=True)
        self.path = directory / 'routes.json'
        self.data = json.loads(self.path.read_text()) if self.path.exists() else {
            'enabled_at_ms': int(time.time() * 1000), 'routes': {}, 'pending_post': None,
        }
        if new_conversations:
            self.data.setdefault('mention_cursor', f'{time.time():.6f}')
        self.save()

    def save(self):
        temporary = self.path.with_suffix('.tmp')
        with temporary.open('w') as handle:
            os.chmod(temporary, 0o600)
            json.dump(self.data, handle, indent=2)
            handle.flush()
            os.fsync(handle.fileno())
        temporary.replace(self.path)


class SlackIngress:
    """One Slack connection, retained while the workstation disconnects/reconnects."""

    def __init__(self, web, config, app_token):
        self.config = config
        self.incoming = queue.Queue()
        self.catch_up = threading.Event()
        identity = web.auth_test()
        if identity['team_id'] != config['slack_team'] or not identity.get('bot_id'):
            raise RuntimeError('Slack credentials belong to a different workspace or are not a bot')
        if 'reactions:write' not in identity.headers.get('x-oauth-scopes', '').split(','):
            raise RuntimeError('The Slack app needs reactions:write; authorize that scope before enabling reactions')
        self.bot_user = identity['user_id']
        self.client = SocketModeClient(app_token=app_token, web_client=web)
        self.client.socket_mode_request_listeners.append(self.receive)
        self.client.on_message_listeners.append(self.connection_message)

    def connection_message(self, raw):
        if json.loads(raw).get('type') == 'hello':
            self.catch_up.set()

    def receive(self, connection, request):
        if request.type not in ('events_api', 'interactive'):
            return
        payload = request.payload
        if request.type == 'interactive':
            if payload.get('team', {}).get('id') == self.config['slack_team']:
                self.incoming.put({'interaction': payload})
        elif payload.get('team_id') == self.config['slack_team']:
            event = payload.get('event', {})
            if event.get('channel') == self.config['slack_channel']:
                self.incoming.put(event)
        connection.send_socket_mode_response(SocketModeResponse(envelope_id=request.envelope_id))

    def close(self):
        self.client.close()


class Bridge:
    def __init__(self, state, web, codex, config):
        self.state, self.web, self.codex = state, web, codex
        self.config = config
        self.channel, self.owner = config['slack_channel'], config['slack_owner']
        self.slack_history = SlackHistory(web, self.channel)
        self.root_updates = {}  # Transport deduplication only; native turns own status.
        self.bot_user = None
        self.requests = {}

    def subscribe(self, ident, participant_reply=False):
        try:
            return self.codex.call('thread/resume', {'threadId': ident})['thread']
        except RpcError:
            if not self.codex.archived(ident):
                raise
            if participant_reply:
                self.codex.call('thread/unarchive', {'threadId': ident})
                return self.codex.call('thread/resume', {'threadId': ident})['thread']
            return self.read_thread(ident)

    def post(self, text, root=None, blocks=None):
        if self.state.data['pending_post']:
            raise RuntimeError('A Slack delivery has an unknown outcome; reconcile it before sending again')
        marker = str(uuid.uuid4())
        self.state.data['pending_post'] = {'client_msg_id': marker, 'root': root}
        self.state.save()
        arguments = dict(channel=self.channel, **markdown_payload(text), client_msg_id=marker,
                         unfurl_links=False, unfurl_media=False)
        if blocks is not None:
            arguments['blocks'] = blocks
        if root:
            arguments['thread_ts'] = root
            arguments['reply_broadcast'] = False
        try:
            result = self.web.chat_postMessage(**arguments)
        except SlackApiError:
            # Slack answered. A later send must not stay blocked on this refusal.
            self.state.data['pending_post'] = None
            self.state.save()
            raise
        # Slack owns Markdown rendering and normalizes the returned text. Its
        # successful channel/timestamp acknowledgement confirms this submission;
        # comparing rendered text to Markdown would misclassify a successful send.
        if not result.get('ok') or result.get('channel') != self.channel or not result.get('ts'):
            self.state.data['pending_post'] = None
            self.state.save()
            raise RuntimeError('Slack did not confirm delivery to the intended channel')
        # Caller records the routing/cursor and clears this in the same save.
        return result['ts']

    def posted(self):
        self.state.data['pending_post'] = None
        self.state.save()

    def update_message(self, timestamp, **payload):
        result = self.web.chat_update(channel=self.channel, ts=timestamp, **payload)
        if not result.get('ok') or result.get('channel') != self.channel or result.get('ts') != timestamp:
            raise RuntimeError('Slack did not confirm the message update')

    def route(self, native):
        ident = native['thread_id']
        if ident in self.state.data['routes']:
            return
        result = self.subscribe(ident)
        self.connect(native, result)

    def connect_thread(self, ident):
        """Attach an explicitly selected native conversation, without a schedule."""
        if ident in self.state.data['routes']:
            return
        thread = self.subscribe(ident)
        self.connect({'thread_id': ident, 'created_at': thread['createdAt'] * 1000}, thread)

    def connect(self, native, result):
        ident = native['thread_id']
        if result['id'] != ident:
            raise RuntimeError('Codex returned a different thread')
        root = self.post(**self.root_payload(native, result))
        self.state.data['routes'][ident] = {
            **native, 'root': root, 'cursor': root, 'delivered': {}, 'pending_input': None,
        }
        self.posted()
        self.deliver(ident)

    def is_new_mention(self, event):
        return (bool(self.config.get('new_conversations'))
                and human_message(event, self.channel)
                and f'<@{self.bot_user}>' in event.get('text', ''))

    def start_from_mention(self, event):
        if not self.is_new_mention(event):
            return
        root = event.get('thread_ts') or event['ts']
        for ident, route in self.state.data['routes'].items():
            if route['root'] == root:
                self.receive(ident, event)
                return
        if self.state.data.get('pending_start'):
            raise RuntimeError('A native conversation start has an unknown outcome; reconcile it before starting again')
        settings = self.config['new_conversations']
        params = {'cwd': settings['cwd']}
        if settings.get('model'):
            params['model'] = settings['model']
        if settings.get('reasoning_effort'):
            params['config'] = {'model_reasoning_effort': settings['reasoning_effort']}
        # Like input/post receipts, retain an uncertain submission instead of
        # creating another native conversation after a lost acknowledgement.
        self.state.data['pending_start'] = {'root': root, 'message': event['ts'], 'user': event['user']}
        self.state.save()
        try:
            thread = self.codex.call('thread/start', params)['thread']
        except RpcError as error:
            if definite_rejection(error):
                self.state.data['pending_start'] = None
                self.state.save()
            raise
        ident = thread['id']
        self.state.data['routes'][ident] = {
            'thread_id': ident, 'created_at': thread['createdAt'] * 1000,
            'root': root, 'cursor': '0', 'source': 'slack',
            'requested_by': event['user'], 'delivered': {}, 'pending_input': None,
        }
        self.state.data['pending_start'] = None
        self.state.save()
        self.receive(ident, event, thread=thread)

    def new_mentions(self):
        if not self.config.get('new_conversations'):
            return
        cursor, messages = '', {}
        since = self.state.data['mention_cursor']
        known_roots = {route['root'] for route in self.state.data['routes'].values()}
        while True:
            # A new mention can be a reply under an arbitrarily old root.
            # Channel history filters root timestamps, not latest_reply, so scan
            # root metadata and read only threads with newer activity. Known
            # routes retain their own catch-up cursor in history().
            page = self.slack_history.page(cursor=cursor)
            for root in page['messages']:
                if root['ts'] > since:
                    messages[root['ts']] = root
                if root['ts'] not in known_roots and root.get('latest_reply', '0') > since:
                    for reply in self.slack_history.thread(root['ts'], oldest=since):
                        messages[reply['ts']] = reply
            cursor = page['next_cursor']
            if not cursor:
                break
        # Slack history is newest first, including across pages. Process all
        # pages oldest first before advancing the durable channel cursor.
        for event in sorted(messages.values(), key=lambda message: message['ts']):
            self.start_from_mention({**event, 'channel': self.channel})
            self.state.data['mention_cursor'] = event['ts']
            self.state.save()

    def read_thread(self, ident):
        return self.codex.call('thread/read', {'threadId': ident, 'includeTurns': True})['thread']

    def root_payload(self, route, thread):
        timestamp = datetime.fromtimestamp(route['created_at'] / 1000, ZoneInfo(self.config['timezone']))
        title = (self.config['automations'][route['automation_id']] if route.get('automation_id')
                 else thread.get('name') or 'Agent conversation')
        return thread_root_payload(title, timestamp,
                                   self.status(route['thread_id'], thread), self.owner, self.config['availability'])

    def status(self, ident, thread):
        needs_input = any(r['params'].get('threadId') == ident for r in self.requests.values())
        return run_status(thread, needs_input)

    def update_reaction(self, root, status):
        # Read Slack's actual reactions so restarting does not leave an old
        # working marker behind. Only this bot's managed reactions are replaced.
        message = self.web.conversations_replies(channel=self.channel, ts=root, limit=1)['messages'][0]
        current = {r['name'] for r in message.get('reactions', []) if self.bot_user in r.get('users', [])}
        desired = STATUS_REACTIONS.get(status)
        for name in current & set(STATUS_REACTIONS.values()) - {desired}:
            try:
                self.web.reactions_remove(channel=self.channel, timestamp=root, name=name)
            except SlackApiError as error:
                if error.response.get('error') != 'no_reaction':
                    raise
        if desired and desired not in current:
            try:
                self.web.reactions_add(channel=self.channel, timestamp=root, name=desired)
            except SlackApiError as error:
                if error.response.get('error') != 'already_reacted':
                    raise

    def update_root(self, ident, thread=None):
        route = self.state.data['routes'][ident]
        thread = thread if thread is not None else self.read_thread(ident)
        payload = self.root_payload(route, thread)
        if self.root_updates.get(ident) == payload:
            return
        if route.get('source') == 'slack' and not route.get('status_message'):
            route['status_message'] = self.post(root=route['root'], **payload)
            self.posted()
        target = route.get('status_message', route['root'])
        # Slack-started conversations retain the human's original message.
        # Updating a known bot message replaces its presentation; it never posts a
        # second message or grants approval. A retry has the same destination.
        self.update_message(target, **payload)
        self.update_reaction(route['root'], self.status(ident, thread))
        self.root_updates[ident] = payload

    def deliver(self, ident):
        route = self.state.data['routes'][ident]
        thread = self.read_thread(ident)
        if not any(t['status'] == 'inProgress' for t in thread['turns']):
            self.requests = {key: r for key, r in self.requests.items()
                             if r['params'].get('threadId') != ident}
        for turn in thread['turns']:
            if turn['status'] == 'inProgress':
                continue
            text = render_native(final_text(turn))
            parts = chunks(text)
            receipts = route['delivered'].get(turn['id'], [])
            if isinstance(receipts, int):
                receipts = [None] * receipts  # Older receipts did not retain Slack timestamps.
            for index, part in enumerate(parts[len(receipts):], start=len(receipts)):
                blocks = markdown_payload(part)['blocks']
                if index == len(parts) - 1 and async_questions(turn):
                    blocks.append(action_block(json.dumps({'thread': ident, 'turn': turn['id']}),
                                               [('question', 'Answer')]))
                receipts.append(self.post(part, route['root'], blocks))
                route['delivered'][turn['id']] = receipts
                self.posted()
        self.update_root(ident, thread)

    def answer_request(self, token, result, user, receipt):
        request = self.requests[token]
        ident = request['params']['threadId']
        self.codex.reply(request['id'], result)
        del self.requests[token]
        route = self.state.data['routes'][ident]
        route['last_input'] = {**receipt, 'user': user, 'request_id': request['id']}
        route.setdefault('input_receipts', []).append(route['last_input'])
        self.state.save()
        if request.get('_slack'):
            card = request['_slack']
            decision = result.get('decision', result.get('action'))
            outcome = 'Approved' if decision == 'accept' else 'Denied' if decision == 'decline' else 'Answered'
            self.update_message(card['ts'],
                **markdown_payload(card['text'] + f'\n\n{outcome} by <@{user}>.'))
        self.update_root(ident)

    def interaction(self, payload):
        if payload.get('team', {}).get('id') != self.config['slack_team'] or not payload.get('user', {}).get('id'):
            return
        user = payload['user']['id']
        if payload.get('type') == 'block_actions':
            if payload.get('channel', {}).get('id') != self.channel:
                return
            for action in payload.get('actions', []):
                token = action.get('value')
                if action['action_id'] == 'coda_question':
                    metadata = {**json.loads(token), 'message': payload['message']['ts']}
                    turn = self.saved_question_turn(metadata)
                    if turn:
                        view = answer_modal(json.dumps(metadata), async_questions(turn))
                        view['callback_id'] = 'coda_async_answer'
                        self.web.views_open(trigger_id=payload['trigger_id'], view=view)
                    continue
                request = self.requests.get(token)
                if not request or request.get('_slack', {}).get('ts') != payload.get('message', {}).get('ts'):
                    continue
                if action['action_id'] == 'coda_answer' and request['method'] == QUESTIONS:
                    self.web.views_open(trigger_id=payload['trigger_id'],
                        view=answer_modal(token, request['params']['questions']))
                elif (request['method'] in APPROVALS or tool_approval(request)) and action['action_id'] in ('coda_accept', 'coda_decline'):
                    self.answer_request(token, approval_response(request, action['action_id'].removeprefix('coda_')), user,
                        {'kind': 'approval', 'message_ts': payload['message']['ts'],
                         'action_ts': action.get('action_ts')})
        elif payload.get('type') == 'view_submission' and payload.get('view', {}).get('callback_id') == 'coda_async_answer':
            view = payload['view']
            metadata = json.loads(view['private_metadata'])
            question_turn = self.saved_question_turn(metadata)
            if not question_turn:
                return
            questions = async_questions(question_turn)
            ident = metadata['thread']
            route = self.state.data['routes'][ident]
            if any(receipt.get('view_id') == view['id'] for receipt in route.get('input_receipts', [])):
                return  # Slack can redeliver an acknowledged view submission.
            answers = [{'questionItemId': json.dumps(question['id'], separators=(',', ':')),
                        'question': question['question'],
                        'answer': view['state']['values'][f'q{index}']['answer']['value']}
                       for index, question in enumerate(questions)]
            text = '<send_user_message_question_reply>\n' + json.dumps(answers) + '\n</send_user_message_question_reply>'
            context = self.slack_history.input_context(route['root'], user, text, view_id=view['id'])
            self.submit(ident, context, {'kind': 'answer', 'user': user, 'view_id': view['id'],
                                       'question_turn': metadata['turn']},
                        f'slack-view:{self.config["slack_team"]}:{view["id"]}')
            summary = '\n\n'.join(f'{a["question"]}\n{a["answer"]}' for a in answers)
            original = chunks(render_native(final_text(question_turn)))[-1]
            self.update_message(metadata['message'],
                **markdown_payload(f'{original}\n\nAnswered by <@{user}>\n\n{summary}'))
        elif payload.get('type') == 'view_submission' and payload.get('view', {}).get('callback_id') == 'coda_answer':
            view = payload['view']
            token = view['private_metadata']
            request = self.requests.get(token)
            if not request or request['method'] != QUESTIONS:
                return
            answers = {question['id']: {'answers': [view['state']['values'][f'q{index}']['answer']['value']]}
                       for index, question in enumerate(request['params']['questions'])}
            self.answer_request(token, {'answers': answers}, user,
                                {'kind': 'answer', 'view_id': view['id']})

    def saved_question_turn(self, metadata):
        route = self.state.data['routes'].get(metadata['thread'])
        receipts = route['delivered'].get(metadata['turn'], []) if route else []
        if not isinstance(receipts, list) or metadata['message'] not in receipts:
            return None
        thread = self.read_thread(metadata['thread'])
        turn = next((t for t in thread['turns'] if t['id'] == metadata['turn']), None)
        return turn if turn and async_questions(turn) else None

    def receive(self, ident, event, thread=None):
        route = self.state.data['routes'][ident]
        if event['ts'] <= route['cursor'] or not admitted(
                event, route['root'], self.channel, route.get('source') == 'slack'):
            return
        mentioned = f'<@{self.bot_user}>' in event['text']
        text = event['text'].replace(f'<@{self.bot_user}>', '').strip()
        words = text.split(maxsplit=2)
        control = control_command(words, self.requests)
        if not mentioned and text.lower() != 'stop' and not control:
            return
        if route['pending_input']:
            raise RuntimeError(f'Uncertain input delivery for {ident}; inspect the native conversation before retrying')
        if control:
            token = words[1]
            request = self.requests.get(token)
            if request and request['params']['threadId'] == ident:
                method = request['method']
                if method in APPROVALS or tool_approval(request):
                    if words[0].lower() not in ('approve', 'deny'):
                        return
                    result = approval_response(request, 'accept' if words[0].lower() == 'approve' else 'decline')
                elif method == 'item/tool/requestUserInput' and words[0].lower() == 'answer' and len(words) == 3:
                    answers = words[2].split(' | ')
                    questions = request['params']['questions']
                    if len(answers) != len(questions):
                        self.post(f'Please answer all {len(questions)} questions, separated by ` | `.', route['root'])
                        self.posted()
                        return
                    result = {'answers': {q['id']: {'answers': [a]} for q, a in zip(questions, answers)}}
                else:
                    return
                route['cursor'] = event['ts']
                self.answer_request(token, result, event['user'], {'ts': event['ts'], 'kind': words[0].lower()})
                return
            self.post('That request is no longer pending in this connection. No approval was applied.', route['root'])
            self.posted()
            route['cursor'] = event['ts']
            self.state.save()
            return
        stop = text.lower() == 'stop'
        context = self.slack_history.request_context(route['root'], event) if not stop else None
        self.submit(ident, context, {'ts': event['ts'], 'user': event['user'], 'kind': 'stop' if stop else 'message'},
                    f'slack:{self.channel}:{event["ts"]}', thread=thread, stop=stop)

    def submit(self, ident, context, receipt, client_id, *, thread=None, stop=False):
        route = self.state.data['routes'][ident]
        if route['pending_input']:
            raise RuntimeError(f'Uncertain input delivery for {ident}; inspect the native conversation before retrying')
        if thread is None and not stop:
            self.subscribe(ident, participant_reply=True)
        thread = thread if thread is not None else self.read_thread(ident)
        active = next((t for t in reversed(thread['turns']) if t['status'] == 'inProgress'), None)
        route['pending_input'] = receipt
        self.state.save()
        try:
            if stop:
                if active:
                    self.codex.call('turn/interrupt', {'threadId': ident, 'turnId': active['id']})
            elif active:
                self.codex.call('turn/steer', {'threadId': ident, 'expectedTurnId': active['id'],
                                             'clientUserMessageId': client_id,
                                             'input': [{'type': 'text', 'text': context}]})
            else:
                turn = self.codex.call('turn/start', {'threadId': ident,
                    'clientUserMessageId': client_id,
                    'input': [{'type': 'text', 'text': context}]})['turn']
                # The native response already owns the accepted turn's status. A
                # brand-new session file may not be readable until its first save.
                thread = {**thread, 'turns': [*thread['turns'], turn]}
        except RpcError as error:
            if definite_rejection(error):
                route['pending_input'] = None
                self.state.save()
            raise
        if receipt.get('ts'):
            route['cursor'] = receipt['ts']
        route['pending_input'] = None
        route['last_input'] = receipt
        if not stop:
            route['last_input']['turn_id'] = active['id'] if active else turn['id']
        route.setdefault('input_receipts', []).append(route['last_input'])
        self.state.save()
        self.update_root(ident, thread if not stop else None)

    def history(self, ident):
        route = self.state.data['routes'][ident]
        for event in self.slack_history.thread(route['root'], oldest=route['cursor']):
            self.receive(ident, {**event, 'channel': self.channel})

    def server_event(self, event):
        params = event.get('params', {})
        ident = params.get('threadId')
        if ident not in self.state.data['routes']:
            return
        route = self.state.data['routes'][ident]
        if 'id' in event:
            token = uuid.uuid4().hex[:8]
            detail = dict(params)
            if event['method'] == 'item/fileChange/requestApproval':
                thread = self.read_thread(ident)
                detail['fileChange'] = next((item for turn in thread['turns'] for item in turn['items']
                    if item.get('id') == params.get('itemId')), None)
            text, actions = request_text(event, token, detail)
            if event['method'] == QUESTIONS or tool_approval(event):
                thread = self.read_thread(ident)
                turn = next((t for t in thread['turns'] if t['id'] == params['turnId']), None)
                context = '\n\n'.join(item['text'] for item in (turn or {}).get('items', [])
                    if item.get('type') == 'agentMessage' and item.get('text'))
                if context:
                    text = context + '\n\n' + text
            parts = chunks(text)
            for index, part in enumerate(parts):
                blocks = markdown_payload(part)['blocks']
                if actions and index == len(parts) - 1:
                    blocks.append(action_block(token, actions))
                timestamp = self.post(part, route['root'], blocks)
                self.posted()
            self.requests[token] = {**event, **({'_slack': {'ts': timestamp, 'text': parts[-1]}} if actions else {})}
            self.update_root(ident)
        elif event['method'] == 'turn/completed':
            self.deliver(ident)
        elif event['method'] == 'turn/started':
            self.update_root(ident)

    def run(self, ingress, connect_latest=False, thread_id=None):
        self.bot_user = ingress.bot_user
        try:
            if thread_id:
                self.connect_thread(thread_id)
            if connect_latest:
                latest = {r['automation_id']: r for r in self.codex.runs(0, self.config['automations'])}
                for native in latest.values():
                    self.route(native)
            for ident in self.state.data['routes']:
                self.subscribe(ident)
                self.deliver(ident)
            ingress.catch_up.set()
            next_discovery = 0
            while not self.codex.closed.is_set():
                if time.monotonic() >= next_discovery:
                    for native in self.codex.runs(self.state.data['enabled_at_ms'], self.config['automations']):
                        self.route(native)
                    next_discovery = time.monotonic() + 30
                if ingress.catch_up.is_set():
                    ingress.catch_up.clear()
                    self.new_mentions()
                    for ident in self.state.data['routes']:
                        self.history(ident)
                while not ingress.incoming.empty():
                    event = ingress.incoming.get_nowait()
                    if 'interaction' in event:
                        self.interaction(event['interaction'])
                        continue
                    if self.is_new_mention(event):
                        self.start_from_mention(event)
                    for ident, route in self.state.data['routes'].items():
                        if event.get('thread_ts') == route['root']:
                            self.receive(ident, event)
                try:
                    self.server_event(self.codex.events.get(timeout=1))
                except queue.Empty:
                    pass
            raise RuntimeError('Desktop Codex connection closed; reconnect through the service supervisor')
        finally:
            self.codex.close()
