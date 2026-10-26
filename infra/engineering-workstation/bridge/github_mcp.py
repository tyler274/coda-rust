#!/usr/bin/env python3
"""Carry native MCP stdio over the existing IAP tunnel to GitHub's official server.

The gateway owns the company app key. GitHub's server owns installation-token
refresh and all GitHub tools. The workstation transports MCP messages only.
"""
import argparse
import logging
import os
from pathlib import Path
import subprocess
import sys
import threading

from websockets.exceptions import ConnectionClosed
from websockets.sync.client import connect


def github_runtime(config):
    settings = config['github']
    # Do not inherit the gateway's Slack credentials or any personal GitHub
    # login. The official server receives only the configured installation.
    environment = {
        'PATH': os.defpath, 'HOME': str(Path.home()),
        'GITHUB_APP_ID': settings['app_id'],
        'GITHUB_APP_INSTALLATION_ID': settings['installation_id'],
        'GITHUB_APP_PRIVATE_KEY': os.environ['GITHUB_APP_PRIVATE_KEY'],
    }
    command = [str(Path(__file__).with_name('github-mcp-server')), 'stdio',
               '--read-only', '--toolsets', 'repos,pull_requests,actions']
    return command, environment


def forward_lines(source, send, close):
    try:
        for line in source:
            send(line.rstrip('\n'))
    except (ConnectionClosed, BrokenPipeError):
        pass
    finally:
        close()


def serve_github(connection, config):
    command, environment = github_runtime(config)
    with subprocess.Popen(command, env=environment, stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True, bufsize=1) as process:
        output = threading.Thread(target=forward_lines,
            args=(process.stdout, connection.send, connection.close), daemon=True)
        output.start()
        try:
            for message in connection:
                process.stdin.write(message + '\n')
                process.stdin.flush()
        except (ConnectionClosed, BrokenPipeError):
            pass
        finally:
            process.stdin.close()
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
            connection.close()
            output.join(timeout=5)
        if process.returncode:
            logging.warning('GitHub MCP process exited with status %s', process.returncode)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--iap-port', type=int, default=8765)
    args = parser.parse_args()
    with connect(f'ws://127.0.0.1:{args.iap_port}/github', proxy=None,
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


if __name__ == '__main__':
    main()
