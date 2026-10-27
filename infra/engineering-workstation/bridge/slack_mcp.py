#!/usr/bin/env python3
"""Read-only Slack history tools, using the gateway's existing channel and bot grant."""
import json
import re

from slack_sdk.errors import SlackApiError

from mcp_transport import proxy
from slack_history import SlackHistory


PAGING = {'cursor': {'type': 'string', 'description': 'next_cursor from the preceding page; omit to begin.'},
          'limit': {'type': 'integer', 'minimum': 1, 'maximum': 100}}
TOOLS = [
    {'name': 'read_channel_context',
     'description': 'Read the configured channel name, visibility and current members with Slack names. '
                    'This describes who can read replies, not who received notifications or read them.',
     'inputSchema': {'type': 'object', 'properties': {}, 'additionalProperties': False}},
    {'name': 'read_channel_history',
     'description': 'Read a page of top-level messages in the configured engineering Slack channel, newest first. '
                    'Follow next_cursor for older history; read_thread opens the replies. No fixed lookback window.',
     'inputSchema': {'type': 'object', 'properties': {**PAGING,
         'oldest': {'type': 'string', 'description': 'Optional Unix timestamp lower bound for top-level messages.'},
         'latest': {'type': 'string', 'description': 'Optional Unix timestamp upper bound for top-level messages.'}},
         'additionalProperties': False}},
    {'name': 'read_thread',
     'description': 'Read a page of a Slack thread in the configured channel, oldest first, including its root. '
                    'Follow next_cursor to read all replies, with authors and attached evidence references.',
     'inputSchema': {'type': 'object', 'properties': {**PAGING,
         'thread_ts': {'type': 'string', 'description': 'Root message timestamp from channel history or a search hit.'}},
         'required': ['thread_ts'], 'additionalProperties': False}},
    {'name': 'search_channel_history',
     'description': 'Search message text in the configured Slack channel, including thread replies. '
                    'The query is case-insensitive literal text, not Slack search syntax. '
                    'Each call scans a page of top-level messages and all their replies (default 10 roots). '
                    'Follow next_cursor until complete to search older history; an empty page is not no matches overall.',
     'inputSchema': {'type': 'object', 'properties': {**PAGING,
         'query': {'type': 'string', 'minLength': 1}}, 'required': ['query'], 'additionalProperties': False}},
]
for tool in TOOLS:
    tool['annotations'] = {'readOnlyHint': True, 'destructiveHint': False, 'idempotentHint': True,
                           'openWorldHint': True}


def call_tool(history, name, arguments):
    tool = next((tool for tool in TOOLS if tool['name'] == name), None)
    if tool is None:
        raise ValueError('Unknown read-only Slack tool')
    schema = tool['inputSchema']
    if not isinstance(arguments, dict) or set(arguments) - schema['properties'].keys():
        raise ValueError('Unexpected tool arguments; this provider reads only its configured channel')
    for key in schema.get('required', []):
        if not arguments.get(key):
            raise ValueError(f'{key} is required')
    for key, value in arguments.items():
        if key == 'limit':
            if type(value) is not int or not 1 <= value <= 100:
                raise ValueError('limit must be an integer from 1 to 100')
        elif not isinstance(value, str):
            raise ValueError(f'{key} must be a string')
        elif key in ('thread_ts', 'oldest', 'latest') and not re.fullmatch(r'\d+(?:\.\d+)?', value):
            raise ValueError(f'{key} must be a Slack timestamp')
    if name == 'read_channel_context':
        return history.channel_context()
    if name == 'search_channel_history':
        if not arguments['query'].strip():
            raise ValueError('query must contain text')
        return history.search(**arguments)
    page = history.page(**{'limit': 20, **arguments})
    page['messages'] = history.with_authors(page['messages'])
    return page


def serve_slack(connection, web, config):
    history = SlackHistory(web, config['slack_channel'])
    for raw in connection:
        try:
            request = json.loads(raw)
        except json.JSONDecodeError:
            connection.send(json.dumps({'jsonrpc': '2.0', 'id': None,
                                        'error': {'code': -32700, 'message': 'Invalid JSON'}}))
            continue
        if 'id' not in request:
            continue  # MCP notifications do not receive responses.
        response = {'jsonrpc': '2.0', 'id': request['id']}
        method, params = request.get('method'), request.get('params', {})
        if method == 'initialize':
            version = params.get('protocolVersion')
            response['result'] = {
                'protocolVersion': version if version in ('2024-11-05', '2025-03-26', '2025-06-18') else '2025-06-18',
                'capabilities': {'tools': {}},
                'serverInfo': {'name': 'coda-slack-history', 'version': '1.0.0'},
                'instructions': 'Read-only access to the configured engineering channel. Use pagination and exact '
                                'thread reads for progressive disclosure. Slack retention and bot access still apply.'}
        elif method == 'ping':
            response['result'] = {}
        elif method == 'tools/list':
            response['result'] = {'tools': TOOLS}
        elif method == 'tools/call':
            try:
                result = call_tool(history, params.get('name'), params.get('arguments', {}))
                response['result'] = {'content': [{'type': 'text', 'text': json.dumps(result, ensure_ascii=False)}]}
            except (ValueError, SlackApiError) as error:
                if isinstance(error, SlackApiError):
                    detail = f'Slack refused the read: {error.response.get("error", "unknown_error")}'
                    retry_after = error.response.headers.get('Retry-After')
                    if retry_after:
                        detail += f'; retry after {retry_after} seconds'
                else:
                    detail = str(error)
                response['result'] = {'isError': True, 'content': [{'type': 'text', 'text': detail}]}
        else:
            response['error'] = {'code': -32601, 'message': 'Method not found'}
        connection.send(json.dumps(response))


if __name__ == '__main__':
    proxy('/slack')
