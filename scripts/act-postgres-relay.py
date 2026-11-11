#!/usr/bin/env python3
"""Relay 127.0.0.1:<port> to the CI postgres service when the job runs under act.

act publishes no service port; its job container reaches the service only by
name on the job network. The PostgreSQL proofs accept a loopback server only,
so under act CI starts this relay and points them at loopback. It detaches,
serves until the job container is removed, and returns once it is listening.

    act-postgres-relay.py <listen-port> <service-host> <service-port>
"""

import os
import socket
import sys
import threading


def pump(source, sink):
    try:
        while data := source.recv(65536):
            sink.sendall(data)
    except OSError:
        pass
    finally:
        for end in (source, sink):
            try:
                end.shutdown(socket.SHUT_RDWR)
            except OSError:
                pass


def serve(listener, target):
    while True:
        client, _ = listener.accept()
        try:
            upstream = socket.create_connection(target)
        except OSError as error:
            print(f"act-postgres-relay: {target[0]}:{target[1]}: {error}", file=sys.stderr)
            client.close()
            continue
        threading.Thread(target=pump, args=(client, upstream), daemon=True).start()
        threading.Thread(target=pump, args=(upstream, client), daemon=True).start()


def main(argv):
    if len(argv) != 3:
        raise SystemExit(__doc__.strip().splitlines()[-1].strip())
    listen_port, host, port = int(argv[0]), argv[1], int(argv[2])
    listener = socket.create_server(("127.0.0.1", listen_port))
    # Bound before the fork, so the step that starts the relay can connect as
    # soon as this returns.
    if os.fork():
        return
    os.setsid()
    devnull = os.open(os.devnull, os.O_RDWR)
    for fd in (0, 1):
        os.dup2(devnull, fd)
    serve(listener, (host, port))


if __name__ == "__main__":
    main(sys.argv[1:])
