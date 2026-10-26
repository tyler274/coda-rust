"""Translate Codex display cards; Slack owns standard Markdown rendering."""
import json
import re


DIRECTIVE = re.compile(r'::(code-comment|inbox-item)\{(.*)\}\s*')
ATTRIBUTE = re.compile(r'(\w+)=("(?:\\.|[^"\\])*"|[^\s}]+)')
FENCE = re.compile(r'^ {0,3}(`{3,}|~{3,})')
STATUS_REACTIONS = {'Working': 'eyes', 'Needs your input': 'question',
                    'Report ready': 'white_check_mark', 'Run failed': 'warning',
                    'Stopped': 'pause_button', 'Finished without a saved answer': 'warning'}


def render_native(text):
    rendered, fence = [], None
    for line in text.splitlines(keepends=True):
        marker = FENCE.match(line)
        if marker:
            token = marker[1]
            if fence is None:
                fence = token
            elif token[0] == fence[0] and len(token) >= len(fence):
                fence = None
            rendered.append(line)
            continue
        match = DIRECTIVE.fullmatch(line.rstrip('\n')) if fence is None else None
        if not match:
            rendered.append(line)
            continue
        attributes, position = {}, 0
        try:
            for field in ATTRIBUTE.finditer(match[2]):
                if match[2][position:field.start()].strip():
                    raise ValueError('Unparsed directive text')
                value = field[2]
                attributes[field[1]] = json.loads(value) if value.startswith('"') else value
                position = field.end()
            if match[2][position:].strip():
                raise ValueError('Unparsed directive text')
            title = attributes['title']
            if match[1] == 'code-comment':
                location = attributes['file']
                if attributes.get('start'):
                    location += ':' + attributes['start']
                    if attributes.get('end') and attributes['end'] != attributes['start']:
                        location += '–' + attributes['end']
                card = f'**{title}**\n\n{attributes["body"]}\n\nSource: `{location}`'
            else:
                card = f'**{title}**\n\n{attributes["summary"]}'
        except (KeyError, ValueError):
            rendered.append(line)  # Preserve content we cannot faithfully render.
            continue
        rendered.append(card + ('\n\n' if line.endswith('\n') else ''))
    return ''.join(rendered)


def chunks(text, size=10000):
    """Keep paragraphs and source locations together, preserving all characters."""
    result = []
    while len(text) > size:
        boundary = text.rfind('\n\n', 0, size + 1)
        if boundary > 0:
            boundary += 2
        else:
            boundary = text.rfind('\n', 0, size + 1)
            boundary = boundary + 1 if boundary > 0 else size
        result.append(text[:boundary])
        text = text[boundary:]
    if text:
        result.append(text)
    return result or ['(No text result.)']


def markdown_payload(text):
    # Slack translates standard Markdown (including tables) into native blocks.
    # The text fallback also serves notifications and accessibility. Slack
    # normalizes both fields into its rich-text representation on readback.
    return {'text': text, 'blocks': [{'type': 'markdown', 'text': text}], 'parse': 'none'}


def run_status(thread, needs_input=False):
    if needs_input:
        return 'Needs your input'
    turns = thread.get('turns', [])
    if not turns:
        return 'No turn observed yet'
    turn = turns[-1]
    if turn['status'] == 'completed':
        has_answer = any(item.get('type') == 'agentMessage' and item.get('phase') == 'final_answer'
                         for item in turn.get('items', []))
        return 'Report ready' if has_answer else 'Finished without a saved answer'
    return {'inProgress': 'Working', 'failed': 'Run failed',
            'interrupted': 'Stopped'}.get(turn['status'], f'Status: {turn["status"]}')


def thread_root_payload(title, timestamp, status, owner, availability):
    started = f'Started {timestamp:%a, %b %-d · %-I:%M %p %Z}'
    instructions = f'Reply here to contribute, or send `stop` to interrupt. Workstation owner: <@{owner}>.'
    observed = f'Last observed: {status}'
    return {
        'text': f'{title}\n{started}\n\n{observed}\n\n{instructions}\n\n{availability}',
        'blocks': [
            {'type': 'header', 'text': {'type': 'plain_text', 'text': title}},
            {'type': 'context', 'elements': [{'type': 'plain_text', 'text': started}]},
            {'type': 'section', 'text': {'type': 'mrkdwn', 'text': observed}},
            {'type': 'section', 'text': {'type': 'mrkdwn', 'text': instructions}},
            {'type': 'context', 'elements': [{'type': 'plain_text', 'text': availability}]},
        ],
    }
