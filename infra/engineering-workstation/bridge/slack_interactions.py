"""Present existing native requests in Slack; Codex still owns their lifetime."""
import json


APPROVALS = ('item/commandExecution/requestApproval', 'item/fileChange/requestApproval')
QUESTIONS = 'item/tool/requestUserInput'
ELICITATION = 'mcpServer/elicitation/request'


def tool_approval(request):
    """Recognize Codex's existing one-shot MCP approval, not arbitrary forms."""
    params = request['params']
    return (request['method'] == ELICITATION and params.get('mode') == 'form'
            and params.get('_meta', {}).get('codex_approval_kind') == 'mcp_tool_call'
            and params.get('requestedSchema') == {'type': 'object', 'properties': {}})


def approval_response(request, decision):
    # Omitting persistence accepts this pending call only. Codex owns the grant.
    return {'action': decision, 'content': {}} if tool_approval(request) else {'decision': decision}


def async_questions(turn):
    return [{'id': ['request_user_input_async', item['id'], index],
             'header': 'Your answer', 'question': question['title'],
             'options': [{'label': option, 'description': ''} for option in question.get('options') or []]}
            for item in turn['items'] if item.get('type') == 'agentMessage'
            for index, question in enumerate(item.get('questions') or [])]


def request_text(request, token, detail):
    method, params = request['method'], request['params']
    if tool_approval(request):
        metadata = params['_meta']
        lines = [params['message']]
        if metadata.get('tool_description'):
            lines.append(metadata['tool_description'])
        for field in metadata.get('tool_params_display', []):
            value = field['value']
            if not isinstance(value, str):
                value = json.dumps(value, ensure_ascii=False)
            lines.append(f'**{field.get("display_name", field["name"])}**\n{value}')
        if not metadata.get('tool_params_display') and metadata.get('tool_params'):
            lines.append('```json\n' + json.dumps(metadata['tool_params'], indent=2, ensure_ascii=False) + '\n```')
        lines.append('Approve or deny this action only.')
        return '\n\n'.join(lines), [('accept', 'Approve'), ('decline', 'Deny')]
    if method == QUESTIONS:
        if any(q.get('isSecret') for q in params['questions']):
            return 'This request includes a secret answer. Open the native Codex task to respond.', []
        lines = ['Your input is needed']
        for question in params['questions']:
            lines.extend(['', question['question']])
            lines.extend(f'• {option["label"]}' + (f': {option["description"]}' if option.get('description') else '')
                         for option in question.get('options') or [])
        lines.extend(['', f'Choose Answer below, or reply `answer {token} YOUR ANSWER` '
                         '(separate multiple answers with ` | `).'])
        return '\n'.join(lines), [('answer', 'Answer')]
    if method in APPROVALS:
        if method == APPROVALS[1] and detail.get('fileChange') is None:
            return 'The proposed file changes could not be read. Open the native Codex task to review them.', []
        if method == APPROVALS[0]:
            lines = ['Review the proposed command']
            if detail.get('reason'):
                lines.append(detail['reason'])
            if detail.get('cwd'):
                lines.append(f'Working directory: `{detail["cwd"]}`')
            lines.append(f'```sh\n{detail.get("command", "Command unavailable; review in native Codex.")}\n```')
            extra = {key: value for key, value in detail.items() if value is not None and key not in (
                'threadId', 'turnId', 'itemId', 'kind', 'startedAtMs', 'environmentId',
                'reason', 'cwd', 'command', 'commandActions', 'availableDecisions', 'proposedExecpolicyAmendment')}
            if extra:
                lines.append(f'Additional request details:\n```json\n{json.dumps(extra, indent=2)}\n```')
            lines.append(f'Approve or deny this request only. You can also reply `approve {token}` or `deny {token}`.')
            return '\n\n'.join(lines), [('accept', 'Approve'), ('decline', 'Deny')]
        title = 'Review the proposed file changes'
        fields = {key: value for key, value in detail.items() if key not in ('threadId', 'turnId', 'itemId')}
        return (f'{title}\n\n```json\n{json.dumps(fields, indent=2)}\n```\n\n'
                f'Approve or deny this request only. You can also reply `approve {token}` or `deny {token}`.'), [
                    ('accept', 'Approve'), ('decline', 'Deny')]
    return 'This request needs the native Codex task. No permission was granted by the bridge.', []


def action_block(token, actions):
    return {'type': 'actions', 'elements': [
        {'type': 'button', 'action_id': f'coda_{action}', 'value': token,
         'text': {'type': 'plain_text', 'text': label}} for action, label in actions]}


def answer_modal(token, questions):
    blocks = []
    for index, question in enumerate(questions):
        text = question['question']
        for option in question.get('options') or []:
            text += f'\n• {option["label"]}' + (f': {option["description"]}' if option.get('description') else '')
        # A text input accepts both a suggested choice and the free-form answers
        # supported by native Codex. No default answer is submitted for anyone.
        for start in range(0, len(text), 2900):
            blocks.append({'type': 'section', 'text': {'type': 'plain_text', 'text': text[start:start + 2900]}})
        blocks.append({'type': 'input', 'block_id': f'q{index}',
                       'label': {'type': 'plain_text', 'text': question.get('header', 'Answer')[:2000]},
                       'element': {'type': 'plain_text_input', 'action_id': 'answer', 'multiline': True}})
    return {'type': 'modal', 'callback_id': 'coda_answer', 'private_metadata': token,
            'title': {'type': 'plain_text', 'text': 'Answer Codex'},
            'submit': {'type': 'plain_text', 'text': 'Send answer'},
            'close': {'type': 'plain_text', 'text': 'Cancel'}, 'blocks': blocks}
