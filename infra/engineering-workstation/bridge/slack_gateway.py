#!/usr/bin/env python3
"""Run Slack delivery independently of the existing Codex workstation.

This first slice accepts one configured workstation through a private Unix
socket or a TCP listener protected by GCP IAP and its cloud firewall. TCP mode
requires that deployment boundary; the application does not authenticate peers.
"""
import argparse
from contextlib import contextmanager
import fcntl
import logging
import os
from pathlib import Path
import stat
import threading
import urllib.error

from slack_sdk.web import WebClient
from websockets.sync.server import serve, unix_serve

from slack_bridge import Bridge, SlackIngress, State
from deployment_config import load_gateway_config
from workstation_connector import RemoteCodex
from github_mcp import serve_github
from github_access import serve_credentials
from slack_mcp import serve_slack
from with_doppler import fetch


def load_credentials(config):
    names = ['SLACK_BOT_TOKEN', 'SLACK_APP_TOKEN']
    if config.get('github'):
        names.append('GITHUB_APP_PRIVATE_KEY')
    # One exchange per gateway startup. Provider sessions share these values:
    # GCE may return the same OIDC token, and Doppler rejects a repeated exchange.
    os.environ.update(fetch(names, config['doppler']))


@contextmanager
def gateway_server(socket, state_directory, web, config, ingress, connect_latest=False, thread_id=None,
                   *, iap_host=None, port=8765):
    """Keep Slack ingress and routing state alive across workstation connections."""
    if (socket is None) == (iap_host is None):
        raise ValueError('Choose a private socket or an IAP-protected listener')
    if socket is not None:
        socket = Path(socket)
        socket.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
        info = socket.parent.stat()
        if info.st_uid != os.getuid() or stat.S_IMODE(info.st_mode) & 0o077:
            raise ValueError('Gateway socket requires a private directory owned by this user')
    state_directory = Path(state_directory)
    state_directory.mkdir(mode=0o700, parents=True, exist_ok=True)
    connection_lock = threading.Lock()

    def connected(connection):
        if connection.request.path == '/slack':
            try:
                serve_slack(connection, web, config)
            except Exception as error:
                logging.warning('Slack history connection ended (%s)', type(error).__name__)
            finally:
                connection.close()
            return
        if connection.request.path == '/github/credential' and config.get('github', {}).get('repository'):
            try:
                serve_credentials(connection, config)
            except Exception as error:
                logging.warning('Company GitHub credential request ended (%s)', type(error).__name__)
            finally:
                connection.close()
            return
        if connection.request.path == '/github' and config.get('github'):
            try:
                serve_github(connection, config)
            except Exception as error:
                logging.warning('Company GitHub connection ended (%s)', type(error).__name__)
            finally:
                connection.close()
            return
        if connection.request.path != '/':
            connection.close(code=1008, reason='This gateway endpoint is not configured')
            return
        # A reconnect can arrive while the previous handler finishes closing its
        # pending calls. Give that normal cleanup time without replacing a live peer.
        if not connection_lock.acquire(timeout=5):
            connection.close(code=1008, reason='The configured workstation is already connected')
            return
        try:
            # Reload after reconnect: acknowledged messages and uncertain effects
            # remain delivery facts, while native Codex owns all execution state.
            Bridge(State(state_directory, bool(config.get('new_conversations'))), web,
                   RemoteCodex(connection), config).run(ingress, connect_latest, thread_id)
        except Exception as error:
            # Provider messages can contain private task details. Keep those out
            # of service logs; persisted receipts remain available for inspection.
            logging.warning('Workstation connection ended (%s); inspect native state and delivery receipts', type(error).__name__)
        finally:
            connection.close()
            connection_lock.release()

    with (state_directory / 'bridge.lock').open('w') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        # Establish the feature's history boundary even with no worker online.
        State(state_directory, bool(config.get('new_conversations')))
        # The directory is private and this process owns the delivery lock.
        if socket is not None:
            socket.unlink(missing_ok=True)
        try:
            serving = (unix_serve(connected, path=str(socket), max_size=16 * 1024 * 1024)
                       if socket is not None else
                       serve(connected, iap_host, port, max_size=16 * 1024 * 1024))
            with serving as server:
                yield server
        finally:
            if socket is not None:
                socket.unlink(missing_ok=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--config', required=True)
    parser.add_argument('--check-credentials', action='store_true', help='Verify credentials without starting delivery')
    transport = parser.add_mutually_exclusive_group()
    transport.add_argument('--socket')
    transport.add_argument('--iap-host', help='Private VM address; firewall must admit only GCP IAP')
    parser.add_argument('--port', type=int, default=8765)
    parser.add_argument('--state-dir')
    parser.add_argument('--connect-latest', action='store_true')
    parser.add_argument('--thread', help='Connect an explicitly chosen native conversation using its name and creation time')
    args = parser.parse_args()
    if not args.check_credentials and (not (args.socket or args.iap_host) or not args.state_dir):
        parser.error('Delivery requires --state-dir and either --socket or --iap-host')
    config = load_gateway_config(args.config)
    load_credentials(config)
    if args.check_credentials:
        print('VM identity and configured gateway credentials verified; values remain in memory.')
        return
    web = WebClient(token=os.environ['SLACK_BOT_TOKEN'], retry_handlers=[])
    ingress = SlackIngress(web, config, os.environ['SLACK_APP_TOKEN'])
    try:
        with gateway_server(args.socket, args.state_dir, web, config, ingress, args.connect_latest,
                            args.thread, iap_host=args.iap_host, port=args.port) as server:
            ingress.client.connect()
            server.serve_forever()
    finally:
        ingress.close()


if __name__ == '__main__':
    try:
        main()
    except urllib.error.HTTPError as error:
        # Provider error bodies can contain authentication details.
        raise SystemExit(f'Gateway credential retrieval failed: HTTP {error.code}')
