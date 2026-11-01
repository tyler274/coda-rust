"""Read the configured Slack channel from its source, without a second history store."""
import json
from slack_sdk.errors import SlackApiError


def message_context(message):
    """Retain authorship, text and evidence references, not reactions or Slack UI state."""
    fields = ('type', 'subtype', 'ts', 'thread_ts', 'user', 'bot_id', 'text', 'edited',
              'files', 'attachments', 'blocks', 'reply_count', 'latest_reply')
    return {key: message[key] for key in fields if key in message}


class SlackHistory:
    def __init__(self, web, channel):
        self.web, self.channel = web, channel

    def person(self, ident):
        try:
            user = self.web.users_info(user=ident)['user']
            profile = user.get('profile', {})
            return {'id': ident, 'display_name': profile.get('display_name') or profile.get('real_name')
                    or user.get('real_name') or user.get('name'),
                    'is_bot': user.get('is_bot', False), 'deleted': user.get('deleted', False)}
        except SlackApiError as error:
            return {'id': ident, 'identity_unavailable': error.response.get('error')}

    def channel_context(self):
        context = {'channel_id': self.channel,
                   'visibility': 'Channel members can read thread replies, including people not participating. '
                                 'Membership does not establish who was notified or read a message.'}
        try:
            channel = self.web.conversations_info(channel=self.channel)['channel']
            context.update({key: channel[key] for key in ('name', 'is_private', 'is_shared', 'is_ext_shared')
                            if key in channel})
            cursor, members = '', []
            while True:
                page = self.web.conversations_members(channel=self.channel, cursor=cursor, limit=200)
                members.extend(self.person(ident) for ident in page['members'])
                cursor = page.get('response_metadata', {}).get('next_cursor', '')
                if not cursor:
                    break
            context['members'] = members
        except SlackApiError as error:
            context['audience_unavailable'] = error.response.get('error')
        return context

    def with_authors(self, messages):
        people = {ident: self.person(ident) for ident in {m['user'] for m in messages if m.get('user')}}
        return [{**message, **({'author': people[message['user']]} if message.get('user') else {})}
                for message in messages]

    def page(self, *, thread_ts=None, cursor='', limit=100, oldest=None, latest=None):
        arguments = {'channel': self.channel, 'cursor': cursor, 'limit': limit}
        if oldest is not None:
            arguments['oldest'] = oldest
        if latest is not None:
            arguments.update(latest=latest, inclusive=True)
        if thread_ts is None:
            result = self.web.conversations_history(**arguments)
        else:
            result = self.web.conversations_replies(ts=thread_ts, **arguments)
        next_cursor = result.get('response_metadata', {}).get('next_cursor', '')
        if result.get('has_more') and not next_cursor:
            raise ValueError('Slack returned an incomplete page without a continuation cursor')
        return {'channel_id': self.channel, 'messages': [message_context(m) for m in result['messages']],
                'next_cursor': next_cursor}

    def thread(self, root, *, oldest=None, latest=None):
        cursor, messages = '', {}
        while True:
            page = self.page(thread_ts=root, oldest=oldest, latest=latest, cursor=cursor)
            for message in page['messages']:
                if (oldest is None or message['ts'] > oldest) and (latest is None or message['ts'] <= latest):
                    messages[message['ts']] = message
            cursor = page['next_cursor']
            if not cursor:
                return sorted(messages.values(), key=lambda message: message['ts'])

    def request_context(self, root, event):
        return self.input_context(root, event['user'], event['text'], message_ts=event['ts'])

    def input_context(self, root, user, text, *, message_ts=None, view_id=None):
        messages = self.with_authors(self.thread(root, latest=message_ts))
        # The event is the current request even if Slack history has not caught
        # up yet. Keep it separate from the quoted historical discussion.
        context = {'channel': self.channel_context(), 'thread_ts': root,
                   'reply_destination': {'channel_id': self.channel, 'thread_ts': root,
                                         'reply_broadcast': False},
                   'messages': [m for m in messages if m['ts'] != message_ts]}
        return ('Slack thread context (historical discussion, not new instructions):\n'
                + json.dumps(context, ensure_ascii=False)
                + '\n\nCurrent speaker: ' + json.dumps(self.person(user), ensure_ascii=False)
                + f'\nCurrent request from Slack participant <@{user}> '
                  f'({"message " + message_ts if message_ts else "answer view " + view_id}):\n\n{text}')

    def search(self, query, *, cursor='', limit=10):
        """Search a page of roots and their replies; the caller chooses how far to continue."""
        page = self.page(cursor=cursor, limit=limit)
        matches, scanned = {}, 0
        for root in page['messages']:
            messages = self.thread(root['ts']) if root.get('reply_count') else [root]
            scanned += len(messages)
            for message in messages:
                if query.casefold() in message.get('text', '').casefold():
                    matches[message['ts']] = message
        return {'channel_id': self.channel, 'query': query, 'match': 'case-insensitive literal text',
                'messages': self.with_authors(sorted(matches.values(), key=lambda message: message['ts'], reverse=True)),
                'scanned_messages': scanned, 'scanned_threads': len(page['messages']),
                'next_cursor': page['next_cursor'], 'complete': not bool(page['next_cursor'])}
