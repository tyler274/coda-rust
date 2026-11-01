#!/usr/bin/env python3
"""Connect native Codex outward to a private gateway socket, optionally SSH-forwarded.

No Slack or model credentials cross this connection. The connector supports
Slack conversations and scheduled runs; it does not start a second native server.
"""
import argparse
import json
import queue
import threading

from websockets.sync.client import connect, unix_connect
from websockets.exceptions import ConnectionClosed

from codex_rpc import Rpc, RpcError

# These are the operations used by the Slack conversation workflow. This
# connector is not a general-purpose remote Codex administration endpoint.
NATIVE_METHODS = frozenset((
    'thread/start', 'thread/read', 'thread/resume', 'thread/unarchive',
    'turn/start', 'turn/steer', 'turn/interrupt',
))


class RemoteCodex(Rpc):
    def runs(self, since, automations):
        return self.call('workstation/runs', {'since': since, 'automations': list(automations)})

    def archived(self, ident):
        return self.call('workstation/archived', {'threadId': ident})


def relay(connection, native):
    """Serve one gateway connection and forward the native owner's events."""
    stopped = threading.Event()

    def events():
        try:
            while not stopped.is_set() and not native.closed.is_set():
                try:
                    event = native.events.get(timeout=0.2)
                except queue.Empty:
                    continue
                connection.send(json.dumps(event))
        except ConnectionClosed:
            pass
        finally:
            connection.close()

    sender = threading.Thread(target=events, daemon=True)
    sender.start()
    try:
        for raw in connection:
            message = json.loads(raw)
            if 'method' not in message:
                native.reply(message['id'], message['result'])
                continue
            method, params = message['method'], message.get('params', {})
            try:
                if method == 'workstation/runs':
                    result = native.runs(params['since'], params['automations'])
                elif method == 'workstation/archived':
                    result = native.archived(params['threadId'])
                elif method in NATIVE_METHODS:
                    result = native.call(method, params)
                else:
                    raise ValueError('Unsupported workstation operation')
                response = {'id': message['id'], 'result': result}
            except RpcError as error:
                response = {'id': message['id'], 'error': {'code': error.code, 'message': str(error)}}
            except (ValueError, KeyError) as error:
                response = {'id': message['id'], 'error': {'code': -32602, 'message': str(error)}}
            connection.send(json.dumps(response))
    except ConnectionClosed:
        pass
    finally:
        stopped.set()
        native.close()
        connection.close()
        sender.join(timeout=5)


def main():
    # The gateway imports RemoteCodex; only the worker opens native app state.
    from native_codex import NativeCodex, SOCKET
    parser = argparse.ArgumentParser(description=__doc__)
    transport = parser.add_mutually_exclusive_group(required=True)
    transport.add_argument('--gateway-socket', help='Private socket, or local end of an SSH Unix-socket forward')
    transport.add_argument('--iap-port', type=int, help='Local loopback port of the authenticated GCP IAP tunnel')
    parser.add_argument('--native-socket', default=str(SOCKET))
    args = parser.parse_args()
    connection_context = (unix_connect(path=args.gateway_socket, open_timeout=10, max_size=16 * 1024 * 1024)
                          if args.gateway_socket else
                          connect(f'ws://127.0.0.1:{args.iap_port}', proxy=None,
                                  open_timeout=10, max_size=16 * 1024 * 1024))
    with connection_context as connection:
        relay(connection, NativeCodex(args.native_socket))


if __name__ == '__main__':
    main()
