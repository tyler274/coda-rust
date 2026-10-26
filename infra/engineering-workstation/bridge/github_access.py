#!/usr/bin/env python3
"""Company Git/gh credentials through the existing authenticated IAP tunnel.

The app key stays on the gateway. Each command gets a fresh, repository-scoped
installation token; neither side persists it or maintains a token cache.
"""
import argparse
import json
import os
import sys

from websockets.sync.client import connect


def issue_token(settings):
    # Gateway-only dependency. PyGithub signs the app JWT; use its public
    # requester because get_access_token doesn't accept repository_ids.
    # https://docs.github.com/en/rest/apps/apps#create-an-installation-access-token-for-an-app
    from github import Auth, GithubIntegration
    with GithubIntegration(auth=Auth.AppAuth(settings['app_id'],
            os.environ['GITHUB_APP_PRIVATE_KEY']), timeout=15, retry=0) as app:
        _, result = app.requester.requestJsonAndCheck('POST',
            f'/app/installations/{settings["installation_id"]}/access_tokens',
            input={'repository_ids': [int(settings['repository_id'])],
                   'permissions': {'contents': 'read', 'pull_requests': 'read', 'actions': 'read'}})
    return {key: result[key] for key in ('token', 'expires_at', 'permissions')}


def serve_credentials(connection, config):
    settings = config['github']
    request = json.loads(connection.recv(timeout=15))
    if (set(request) != {'repository'} or
            request['repository'].casefold() != settings['repository'].casefold()):
        connection.close(code=1008, reason='Repository is outside the configured company scope')
        return
    connection.send(json.dumps(issue_token(settings)))


def fetch_token(repository, port):
    with connect(f'ws://127.0.0.1:{port}/github/credential', proxy=None,
                 open_timeout=15, max_size=65536) as connection:
        connection.send(json.dumps({'repository': repository}))
        return json.loads(connection.recv(timeout=25))['token']


def credential_get(repository, port, source, destination):
    fields = dict(line.rstrip('\n').split('=', 1) for line in source if '=' in line)
    path = fields.get('path', '').removesuffix('.git')
    if (fields.get('protocol') != 'https' or fields.get('host', '').casefold() != 'github.com'
            or path.casefold() != repository.casefold()):
        # Git's credential protocol stops later helpers from supplying a
        # personal credential when the company helper refuses this resource.
        destination.write('quit=true\n\n')
        return
    try:
        token = fetch_token(repository, port)
    except Exception:
        destination.write('quit=true\n\n')
        raise
    destination.write(f'username=x-access-token\npassword={token}\n\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repository', required=True)
    parser.add_argument('--iap-port', type=int, default=8765)
    parser.add_argument('mode', choices=('git-credential', 'gh'))
    parser.add_argument('arguments', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    if args.mode == 'git-credential':
        if args.arguments == ['get']:
            credential_get(args.repository, args.iap_port, sys.stdin, sys.stdout)
        elif args.arguments not in (['store'], ['erase']):
            parser.error('Git credential helper expects get, store or erase')
    else:
        environment = {**os.environ, 'GH_TOKEN': fetch_token(args.repository, args.iap_port),
                       'GH_HOST': 'github.com'}
        # gh gives GH_TOKEN precedence over saved personal authentication.
        # Keep the token out of argv, shell interpolation and persistent config.
        os.execvpe('gh', ['gh', *args.arguments], environment)


if __name__ == '__main__':
    try:
        main()
    except Exception as error:
        print(f'Company GitHub access unavailable ({type(error).__name__}); no personal fallback.', file=sys.stderr)
        raise SystemExit(1)
