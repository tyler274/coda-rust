#!/usr/bin/env python3
"""Register the company Git helper and the workstation's Codex MCP proxies.

The Git helper applies only to one repository URL. The two stdio proxies are
Codex servers named github-company and slack-history. Re-running replaces those
entries and leaves every other Git or Codex setting in place.
"""
import argparse
from pathlib import Path
import re
import shlex
import subprocess


REPOSITORY = re.compile(r'[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+')
SECTION = re.compile(r'^\[mcp_servers\.(?:"([^"]+)"|([A-Za-z0-9_-]+))\]\s*$')


def toml_string(value):
    return '"' + str(value).replace('\\', '\\\\').replace('"', '\\"') + '"'


def mcp_server(name, command, args):
    lines = [f'[mcp_servers.{name}]', f'command = {toml_string(command)}', 'args = [']
    lines.extend(f'  {toml_string(arg)},' for arg in args)
    lines.append(']')
    return '\n'.join(lines) + '\n'


def upsert_mcp_server(text, name, command, args):
    block = mcp_server(name, command, args)
    lines = text.splitlines(keepends=True)
    start = None
    for index, line in enumerate(lines):
        match = SECTION.match(line.rstrip('\n'))
        if match and name in match.groups():
            start = index
            break
    if start is None:
        body = text
        if body and not body.endswith('\n'):
            body += '\n'
        if body and not body.endswith('\n\n'):
            body += '\n'
        return body + block
    end = next((index for index in range(start + 1, len(lines)) if lines[index].startswith('[')), len(lines))
    while end > start + 1 and lines[end - 1].strip() == '':
        end -= 1
    rest = ''.join(lines[end:])
    if rest.startswith('['):
        rest = '\n' + rest
    return ''.join(lines[:start]) + block + rest


def register_git_helper(repository, helper, gitconfig):
    if not REPOSITORY.fullmatch(repository):
        raise SystemExit('Use owner/repository')
    url = f'https://github.com/{repository}'
    gitconfig = Path(gitconfig)
    gitconfig.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(['git', 'config', '--file', str(gitconfig), '--replace-all',
                    f'credential.{url}.helper', '!' + shlex.quote(str(helper))], check=True)
    subprocess.run(['git', 'config', '--file', str(gitconfig),
                    f'credential.{url}.useHttpPath', 'true'], check=True)


def register_mcp_servers(config, python, bridge):
    python, bridge = Path(python), Path(bridge)
    config = Path(config)
    config.parent.mkdir(parents=True, exist_ok=True)
    created = not config.exists()
    text = config.read_text() if not created else ''
    for name, script in (('github-company', 'github_mcp.py'), ('slack-history', 'slack_mcp.py')):
        text = upsert_mcp_server(text, name, python, [bridge / script])
    config.write_text(text)
    if created:
        config.chmod(0o600)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    git = commands.add_parser('git')
    git.add_argument('--repository', required=True)
    git.add_argument('--helper', type=Path, required=True)
    git.add_argument('--gitconfig', type=Path, required=True)
    mcp = commands.add_parser('mcp')
    mcp.add_argument('--config', type=Path, required=True)
    mcp.add_argument('--python', type=Path, required=True)
    mcp.add_argument('--bridge', type=Path, required=True)
    args = parser.parse_args(argv)
    if args.command == 'git':
        register_git_helper(args.repository, args.helper, args.gitconfig)
    else:
        register_mcp_servers(args.config, args.python, args.bridge)


if __name__ == '__main__':
    main()
