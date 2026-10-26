"""Transport native MCP stdio over the workstation's existing private IAP tunnel."""
import argparse
import sys
import threading

from websockets.exceptions import ConnectionClosed
from websockets.sync.client import connect


def forward_lines(source, send, close):
    try:
        for line in source:
            send(line.rstrip('\n'))
    except (ConnectionClosed, BrokenPipeError):
        pass
    finally:
        close()


def proxy(endpoint):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--iap-port', type=int, default=8765)
    args = parser.parse_args()
    with connect(f'ws://127.0.0.1:{args.iap_port}{endpoint}', proxy=None,
                 open_timeout=15, max_size=16 * 1024 * 1024) as connection:
        incoming = threading.Thread(target=forward_lines,
            args=(sys.stdin, connection.send, connection.close), daemon=True)
        incoming.start()
        try:
            for message in connection:
                sys.stdout.write(message + '\n')
                sys.stdout.flush()
        except ConnectionClosed:
            pass
