#!/usr/bin/env python3
"""Completion-triggered shutdown for the one native Coda audit automation.

The notify callback only schedules a single delayed verification. It never
decides success from assistant prose, process exit, or a clock deadline.
No retry timer is retained: missing evidence or other work leaves the VM on.
"""
import hashlib
import itertools
import json
import os
from pathlib import Path
import re
import selectors
import sqlite3
import subprocess
import sys
import time

HOME = Path.home()
CODA_DIR = HOME / 'coda-audit'
AUTOMATION = 'coda-rust-code-and-live-infrastructure-security'
SCANS = HOME / '.codex/state/plugins/codex-security/scans'
DB = HOME / '.codex/sqlite/codex-dev.db'
ENABLED = Path('/etc/coda-audit/shutdown-enabled')
UUID = re.compile(r'^[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$')
KINDS = ['cli', 'vscode', 'exec', 'appServer', 'subAgent', 'subAgentReview',
         'subAgentCompact', 'subAgentThreadSpawn', 'subAgentOther', 'unknown']


def require(ok, reason):
    if not ok:
        raise RuntimeError(reason)


def log(message):
    print(message, flush=True)
    subprocess.run(['/usr/bin/logger', '-t', 'coda-audit-shutdown', '--', message],
                   check=False, timeout=5)


def row_for(thread):
    with sqlite3.connect(f'file:{DB}?mode=ro', uri=True, timeout=5) as db:
        db.row_factory = sqlite3.Row
        row = db.execute('SELECT * FROM automation_runs WHERE thread_id=?',
                         (thread,)).fetchone()
        return dict(row) if row else None


class Rpc:
    def __enter__(self):
        # A short-lived read-only API client works even with the Mac offline.
        # It only reads persisted native threads; it never starts model turns.
        self.proc = subprocess.Popen(['/usr/lib/chatgpt/resources/codex', 'app-server'],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
            bufsize=0)
        self.selector = selectors.DefaultSelector()
        self.selector.register(self.proc.stdout, selectors.EVENT_READ)
        self.buffer = b''
        self.ids = itertools.count(1)
        try:
            self.call('initialize', {'clientInfo': {'name': 'coda-audit-shutdown', 'version': '1'},
                                    'capabilities': {'experimentalApi': True}})
            self.send({'method': 'initialized'})
        except Exception:
            self.__exit__()
            raise
        return self

    def __exit__(self, *_):
        self.selector.close()
        self.proc.terminate()
        try:
            self.proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            self.proc.kill()
            self.proc.wait(timeout=5)

    def send(self, value):
        self.proc.stdin.write((json.dumps(value) + '\n').encode())

    def call(self, method, params=None):
        seq = next(self.ids)
        self.send({'id': seq, 'method': method, 'params': params or {}})
        deadline = time.monotonic() + 30
        while True:
            while b'\n' in self.buffer:
                line, self.buffer = self.buffer.split(b'\n', 1)
                value = json.loads(line)
                if value.get('id') == seq:
                    require('error' not in value, f'app-server rejected {method}')
                    return value['result']
            require(self.selector.select(max(0, deadline - time.monotonic())),
                    f'app-server timed out reading {method}')
            chunk = os.read(self.proc.stdout.fileno(), 65536)
            require(chunk, 'app-server closed before readback')
            self.buffer += chunk

    def last_turn(self, thread, full=False):
        rows = self.call('thread/turns/list', {'threadId': thread, 'limit': 1,
                         'itemsView': 'full' if full else 'notLoaded'})['data']
        return rows[0] if rows else None


def checked_file(path, root):
    path = Path(path).resolve(strict=True)
    require(path.is_relative_to(root.resolve()) and path.is_file(),
            'report or artifact is outside its canonical directory')
    require(path.stat().st_size > 0, 'saved report or artifact is empty')
    return path


def verify_artifacts(final_text):
    # The native task owns the integrated report. Its persisted final message
    # is verified by thread/turn identity above, without requiring an extra
    # model-authored completion receipt or a particular companion filename.
    links = re.findall(r'\]\((/[^)\n]+)\)', final_text)
    sources = [p for p in links if p.startswith(str(SCANS) + '/') and p.endswith('/report.md')]
    require(len(sources) == 1, 'native final result must link one canonical source report')
    source = checked_file(sources[0], SCANS)
    manifest_path = checked_file(source.parent / 'scan-manifest.json', SCANS)
    manifest = json.loads(manifest_path.read_text())
    scan = manifest['scan']
    require(manifest['documentType'] == 'codex-security.scan-manifest'
            and scan['status'] == 'completed' and scan.get('sealedAt')
            and scan.get('completedAt'), 'canonical scan is not completed and sealed')
    require(scan.get('artifacts'), 'sealed scan has no artifacts')
    for artifact in scan['artifacts']:
        path = checked_file(source.parent / artifact['path'], source.parent)
        require(hashlib.sha256(path.read_bytes()).hexdigest() == artifact['sha256'],
                'sealed artifact hash mismatch')
    return {'scan': scan['id'], 'revision': scan['target']['revision'],
            'sealed_artifacts': len(scan['artifacts']),
            'native_report_sha256': hashlib.sha256(final_text.encode()).hexdigest(),
            'source_report_sha256': hashlib.sha256(source.read_bytes()).hexdigest()}


def verify(thread, turn):
    row = row_for(thread)
    require(row and row['automation_id'] == AUTOMATION, 'event is not the scheduled audit')
    require(row['status'] in ('PENDING_REVIEW', 'ACCEPTED')
            and row['inbox_title'] and row['inbox_summary'],
            'native saved inbox result is not ready')
    with sqlite3.connect(f'file:{DB}?mode=ro', uri=True, timeout=5) as db:
        require(db.execute('SELECT status FROM automations WHERE id=?', (AUTOMATION,)).fetchone()
                == ('ACTIVE',), 'native audit schedule is not active')
        require(db.execute('SELECT thread_id FROM automation_runs WHERE automation_id=? '
                           'ORDER BY created_at DESC LIMIT 1', (AUTOMATION,)).fetchone() == (thread,),
                'a newer audit run exists')
        require(db.execute("SELECT count(*) FROM automation_runs WHERE status='IN_PROGRESS'").fetchone()[0]
                == 0, 'another native automation is running')
    with Rpc() as rpc:
        latest = rpc.last_turn(thread, full=True)
        require(latest and latest['id'] == turn and latest['status'] == 'completed',
                'matching native turn is not completed')
        messages = [i['text'] for i in latest.get('items', [])
                    if i.get('type') == 'agentMessage' and i.get('phase') == 'final_answer']
        require(messages, 'native completed turn has no saved final result')
        proof = verify_artifacts(messages[-1])
        cursor = None
        while True:
            page = rpc.call('thread/list', {'limit': 100, 'sourceKinds': KINDS,
                            'sortKey': 'updated_at', 'cursor': cursor})
            for other in page['data']:
                require(other['status']['type'] in ('idle', 'notLoaded'),
                        'another loaded Codex task is active or unhealthy')
                last = rpc.last_turn(other['id'])
                require(last is None or last['status'] in ('completed', 'interrupted', 'failed'),
                        'another saved Codex turn is unfinished')
            cursor = page.get('nextCursor')
            if not cursor:
                break
    return proof


def ensure_host_idle():
    # A connected viewer is treated as active use; disconnect before auto-off.
    connections = subprocess.check_output(['/usr/bin/ss', '-Htn', 'state', 'established',
        '( sport = :5901 or sport = :6080 )'], text=True, timeout=5)
    require(not connections.strip(), 'remote desktop viewer is connected')
    require(not subprocess.check_output(['/usr/bin/docker', 'ps', '-q'], text=True, timeout=10).strip(),
            'a Docker container is running')
    inhibitors = json.loads(subprocess.check_output(
        ['/usr/bin/busctl', '--json=short', 'call', 'org.freedesktop.login1',
         '/org/freedesktop/login1', 'org.freedesktop.login1.Manager', 'ListInhibitors'],
        text=True, timeout=5))
    require(inhibitors.get('type') == 'a(ssssuu)', 'unrecognized shutdown inhibitor response')
    require(not any('shutdown' in i[0].split(':') and i[3] == 'block'
                    for i in inhibitors['data'][0]), 'another application inhibits shutdown')
    for proc in Path('/proc').iterdir():
        if not proc.name.isdecimal():
            continue
        try:
            comm = (proc / 'comm').read_text().strip()
            args = (proc / 'cmdline').read_bytes().split(b'\0')
        except (FileNotFoundError, PermissionError, ProcessLookupError):
            continue
        require(comm not in ('cargo', 'rustc', 'nix', 'nix-build', 'make', 'cmake', 'pytest'),
                'a developer build or test is running')
        require(not (comm == 'codex' and b'exec' in args), 'a Codex CLI execution is running')
        # Claude Desktop has no verified idle/completion integration here.
        # Keep the workstation on while that application or its CLI is open.
        require(comm not in ('claude-desktop', 'claude'), 'Claude is running')


def main():
    if len(sys.argv) == 2:
        event = json.loads(sys.argv[1])
        if event.get('type') != 'agent-turn-complete':
            return
        thread, turn = event.get('thread-id', ''), event.get('turn-id', '')
        require(UUID.fullmatch(thread) and UUID.fullmatch(turn), 'invalid completion identifiers')
        row = row_for(thread)
        if not row or row['automation_id'] != AUTOMATION:
            return
        # Allow the desktop to save its normal inbox/notification result, then
        # check once. This delay is not a shutdown deadline or success signal.
        env = os.environ.copy()
        env['XDG_RUNTIME_DIR'] = f'/run/user/{os.getuid()}'
        subprocess.run(['/usr/bin/systemd-run', '--user', '--collect',
            f'--unit=coda-audit-complete-{turn}', '--on-active=30s',
            '--property=TimeoutStartSec=120',
            '/usr/bin/python3', str(Path(__file__).resolve()),
            '--event-check', thread, turn], env=env, check=True, timeout=10,
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        log(f'completion event received: thread={thread} turn={turn}; one verification scheduled')
        return
    require(len(sys.argv) == 4 and sys.argv[1] in ('--check', '--event-check'), 'invalid invocation')
    mode, thread, turn = sys.argv[1:]
    require(UUID.fullmatch(thread) and UUID.fullmatch(turn), 'invalid completion identifiers')
    proof = verify(thread, turn)
    log(f'saved completion verified: thread={thread} turn={turn} proof={json.dumps(proof, sort_keys=True)}')
    ensure_host_idle()
    if mode == '--check' or not ENABLED.is_file():
        log('dry run passed; no power-off requested')
        return
    log('all completion and idle checks passed; requesting operating-system poweroff')
    os.sync()
    subprocess.run(['/usr/bin/sudo', '-n', '/usr/bin/systemctl', 'poweroff'], check=True, timeout=15)


if __name__ == '__main__':
    try:
        main()
    except Exception as error:
        log(f'kept VM running: {type(error).__name__}: {error}')
        sys.exit(1)
