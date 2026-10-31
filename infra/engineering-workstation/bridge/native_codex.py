"""Workstation-owned access to native Codex. Never writes its databases or auth."""
import os
from contextlib import closing
from pathlib import Path
import sqlite3

from codex_rpc import Codex

DATABASE = Path.home() / '.codex/sqlite/codex-dev.db'
THREAD_DATABASE = Path.home() / '.codex/state_5.sqlite'
SOCKET = Path(os.environ.get('XDG_RUNTIME_DIR', f'/run/user/{os.getuid()}')) / 'coda-codex/app-server.sock'


def native_runs(since, automations, database=DATABASE):
    # This desktop version has no schedule-to-run app-server method. This
    # read-only adapter consumes the native owner; it does not keep a run ledger.
    with closing(sqlite3.connect(f'file:{database}?mode=ro', uri=True, timeout=5)) as db:
        db.row_factory = sqlite3.Row
        return [dict(row) for row in db.execute(
            'SELECT r.thread_id, r.automation_id, r.created_at, a.name '
            'FROM automation_runs r JOIN automations a ON a.id=r.automation_id '
            'WHERE r.automation_id IN (' + ','.join('?' for _ in automations) + ') '
            'AND r.created_at >= ? ORDER BY r.created_at', (*automations, since),
        )]


class NativeCodex(Codex):
    def runs(self, since, automations):
        return native_runs(since, automations)

    def archived(self, ident):
        # Resume rejects archived sessions, but thread/read omits this fact in
        # the installed version. Keep that version coupling on the workstation.
        with closing(sqlite3.connect(f'file:{THREAD_DATABASE}?mode=ro', uri=True, timeout=5)) as db:
            row = db.execute('SELECT archived FROM threads WHERE id=?', (ident,)).fetchone()
            return row is not None and row[0] == 1
