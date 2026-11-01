"""Concurrent client of the desktop-owned Codex server; no model or auth loop."""
import itertools
from contextlib import ExitStack
import json
import queue
import threading

from websockets.sync.client import unix_connect
from websockets.exceptions import ConnectionClosed


class RpcError(RuntimeError):
    def __init__(self, method, error):
        self.code = error.get('code')
        super().__init__(f'{method}: {error.get("message", "unknown error")}')


class Rpc:
    """Request/event transport, also used across the private workstation connection."""

    def __init__(self, connection):
        self.ws = connection
        self.ids = itertools.count(1)
        self.pending = {}
        self.events = queue.Queue()
        self.closed = threading.Event()
        self.reader = threading.Thread(target=self._read, daemon=True)
        self.reader.start()

    def _read(self):
        try:
            for raw in self.ws:
                message = json.loads(raw)
                if 'method' in message:
                    self.events.put(message)
                elif message.get('id') in self.pending:
                    self.pending[message['id']].put(message)
        except ConnectionClosed:
            pass  # Consumers see closed and pending calls receive the error below.
        finally:
            self.closed.set()
            for response in list(self.pending.values()):
                response.put({'error': {'message': 'Codex connection closed; inspect before retrying'}})

    def call(self, method, params, timeout=30):
        ident = next(self.ids)
        response = self.pending[ident] = queue.Queue()
        try:
            self.ws.send(json.dumps({'id': ident, 'method': method, 'params': params}))
            message = response.get(timeout=timeout)
            if 'error' in message:
                raise RpcError(method, message['error'])
            return message['result']
        finally:
            self.pending.pop(ident, None)

    def reply(self, ident, result):
        self.ws.send(json.dumps({'id': ident, 'result': result}))

    def close(self):
        self.ws.close()
        self.reader.join(timeout=5)


class Codex(Rpc):
    def __init__(self, path):
        self.connection_context = ExitStack()
        super().__init__(self.connection_context.enter_context(
            unix_connect(path=str(path), open_timeout=10, max_size=16 * 1024 * 1024)))
        try:
            self.call('initialize', {
                'clientInfo': {'name': 'coda-workstation-slack', 'version': '0.1'},
                'capabilities': {'experimentalApi': True},
            })
            self.ws.send(json.dumps({'method': 'initialized'}))
        except BaseException:
            self.close()
            raise

    def close(self):
        try:
            super().close()
        finally:
            self.connection_context.close()
