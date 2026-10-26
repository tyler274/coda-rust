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
import threading
import time
import uuid
from zoneinfo import ZoneInfo

from slack_sdk.socket_mode import SocketModeClient
from slack_sdk.socket_mode.response import SocketModeResponse
from slack_sdk.errors import SlackApiError

from codex_rpc import RpcError
from slack_render import STATUS_REACTIONS, chunks, markdown_payload, render_native, run_status, thread_root_payload


def human_message(event, channel):
    return (event.get('type', 'message') == 'message'
            and event.get('channel') == channel and bool(event.get('user'))
            and not event.get('subtype')
            and not event.get('bot_id') and not event.get('edited')
            and bool(event.get('text', '').strip()))


def admitted(event, root, channel, slack_started=False):
    return human_message(event, channel) and (
        (event.get('thread_ts') == root and event.get('ts') != root)
        or (slack_started and event.get('ts') == root
            and event.get('thread_ts') in (None, root)))


def final_text(turn):
    messages = [i['text'] for i in turn.get('items', [])
                if i.get('type') == 'agentMessage' and i.get('phase') == 'final_answer']
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
        if request.type != 'events_api':
            return
        payload = request.payload
        if payload.get('team_id') == self.config['slack_team']:
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
        result = self.web.chat_postMessage(**arguments)
        # Slack owns Markdown rendering and normalizes the returned text. Its
        # successful channel/timestamp acknowledgement confirms this submission;
        # comparing rendered text to Markdown would misclassify a successful send.
        if not result.get('ok') or result['channel'] != self.channel or not result.get('ts'):
            raise RuntimeError('Slack did not confirm delivery to the intended channel')
        # Caller records the routing/cursor and clears this in the same save.
        return result['ts']

    def posted(self):
        self.state.data['pending_post'] = None
        self.state.save()

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
                and event.get('thread_ts') in (None, event.get('ts'))
                and f'<@{self.bot_user}>' in event.get('text', ''))

    def start_from_mention(self, event):
        if not self.is_new_mention(event):
            return
        if any(route['root'] == event['ts'] for route in self.state.data['routes'].values()):
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
        self.state.data['pending_start'] = {'root': event['ts'], 'user': event['user']}
        self.state.save()
        thread = self.codex.call('thread/start', params)['thread']
        ident = thread['id']
        self.state.data['routes'][ident] = {
            'thread_id': ident, 'created_at': thread['createdAt'] * 1000,
            'root': event['ts'], 'cursor': '0', 'source': 'slack',
            'requested_by': event['user'], 'delivered': {}, 'pending_input': None,
        }
        self.state.data['pending_start'] = None
        self.state.save()
        self.receive(ident, event, thread=thread)

    def new_mentions(self):
        if not self.config.get('new_conversations'):
            return
        cursor, messages = None, []
        while True:
            result = self.web.conversations_history(channel=self.channel,
                oldest=self.state.data['mention_cursor'], inclusive=False, cursor=cursor, limit=100)
            messages.extend(result['messages'])
            cursor = result.get('response_metadata', {}).get('next_cursor')
            if not cursor:
                break
        # Slack history is newest first, including across pages. Process all
        # pages oldest first before advancing the durable channel cursor.
        for event in sorted(messages, key=lambda message: message['ts']):
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
        result = self.web.chat_update(channel=self.channel, ts=target, **payload)
        if not result.get('ok') or result.get('channel') != self.channel or result.get('ts') != target:
            raise RuntimeError('Slack did not confirm the thread status update')
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
            sent = route['delivered'].get(turn['id'], 0)
            for index, part in enumerate(parts[sent:], start=sent):
                self.post(part, route['root'])
                route['delivered'][turn['id']] = index + 1
                self.posted()
        self.update_root(ident, thread)

    def receive(self, ident, event, thread=None):
        route = self.state.data['routes'][ident]
        if event['ts'] <= route['cursor'] or not admitted(
                event, route['root'], self.channel, route.get('source') == 'slack'):
            return
        if route['pending_input']:
            raise RuntimeError(f'Uncertain input delivery for {ident}; inspect the native conversation before retrying')
        text = event['text'].strip()
        words = text.split(maxsplit=2)
        if words[0].lower() in ('approve', 'deny', 'answer') and len(words) >= 2:
            token = words[1]
            request = self.requests.get(token)
            if request and request['params']['threadId'] == ident:
                method = request['method']
                if method in ('item/commandExecution/requestApproval', 'item/fileChange/requestApproval'):
                    if words[0].lower() not in ('approve', 'deny'):
                        return
                    result = {'decision': 'accept' if words[0].lower() == 'approve' else 'decline'}
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
                self.codex.reply(request['id'], result)
                del self.requests[token]
                route['cursor'] = event['ts']
                route['last_input'] = {'ts': event['ts'], 'user': event['user'], 'kind': words[0].lower()}
                self.state.save()
                self.update_root(ident)
                return
            self.post('That request is no longer pending in this connection. No approval was applied.', route['root'])
            self.posted()
            route['cursor'] = event['ts']
            self.state.save()
            return
        if thread is None and text.lower() != 'stop':
            self.subscribe(ident, participant_reply=True)
        thread = thread if thread is not None else self.read_thread(ident)
        active = next((t for t in reversed(thread['turns']) if t['status'] == 'inProgress'), None)
        route['pending_input'] = {'ts': event['ts'], 'operation': 'stop' if text.lower() == 'stop' else 'message'}
        self.state.save()
        if text.lower() == 'stop':
            if active:
                self.codex.call('turn/interrupt', {'threadId': ident, 'turnId': active['id']})
        elif active:
            self.codex.call('turn/steer', {'threadId': ident, 'expectedTurnId': active['id'],
                                         'input': [{'type': 'text', 'text': f'Slack participant <@{event["user"]}>:\n\n{text}'}]})
        else:
            self.codex.call('turn/start', {'threadId': ident,
                'clientUserMessageId': f'slack:{self.channel}:{event["ts"]}',
                'input': [{'type': 'text', 'text': f'Slack participant <@{event["user"]}>:\n\n{text}'}]})
        route['cursor'], route['pending_input'] = event['ts'], None
        route['last_input'] = {'ts': event['ts'], 'user': event['user'], 'kind': 'stop' if text.lower() == 'stop' else 'message'}
        self.state.save()
        self.update_root(ident)

    def history(self, ident):
        route = self.state.data['routes'][ident]
        cursor = None
        while True:
            result = self.web.conversations_replies(channel=self.channel, ts=route['root'],
                oldest=route['cursor'], inclusive=False, cursor=cursor, limit=100)
            for event in sorted(result['messages'], key=lambda m: m['ts']):
                self.receive(ident, {**event, 'channel': self.channel})
            cursor = result.get('response_metadata', {}).get('next_cursor')
            if not cursor:
                break

    def server_event(self, event):
        params = event.get('params', {})
        ident = params.get('threadId')
        if ident not in self.state.data['routes']:
            return
        route = self.state.data['routes'][ident]
        if 'id' in event:
            token = uuid.uuid4().hex[:8]
            self.requests[token] = event
            if event['method'] in ('item/commandExecution/requestApproval', 'item/fileChange/requestApproval'):
                instructions = f'Reply `approve {token}` or `deny {token}` for this request only.'
            elif event['method'] == 'item/tool/requestUserInput':
                instructions = f'Reply `answer {token} YOUR ANSWER`; separate multiple answers with ` | `.'
            else:
                instructions = 'This request needs the native desktop; no permission was granted by the bridge.'
            detail = dict(params)
            if event['method'] == 'item/fileChange/requestApproval':
                thread = self.read_thread(ident)
                detail['fileChange'] = next((item for turn in thread['turns'] for item in turn['items']
                    if item.get('id') == params.get('itemId')), None)
                if detail['fileChange'] is None:
                    del self.requests[token]
                    instructions = 'The proposed file changes could not be read. This request cannot be approved from Slack.'
            details = json.dumps(detail, indent=2)
            for part in chunks(f'Your input is needed\n\n{instructions}\n\n```json\n{details}\n```'):
                self.post(part, route['root'])
                self.posted()
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
                    if self.is_new_mention(event):
                        ingress.catch_up.set()
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
