"""Nonsecret configuration at the workstation and gateway ownership boundaries."""
import argparse
import json
from pathlib import Path
import re
from zoneinfo import ZoneInfo


WORKSTATION_CONFIG = Path(__file__).with_name('workstation-config.json')
GATEWAY_CONFIG = Path(__file__).with_name('gateway-config.json')


def load_workstation_config(path=WORKSTATION_CONFIG):
    """The desktop needs its VM-bound keyring credential, not Slack routing."""
    config = json.loads(Path(path).read_text())
    validate_doppler(config.get('doppler', {}))
    return config


def validate_doppler(doppler):
    for key in ('identity', 'audience', 'account', 'instance_id', 'project', 'config'):
        if not isinstance(doppler.get(key), str) or not doppler[key].strip() or '<' in doppler[key]:
            raise ValueError(f'Fill in deployment configuration: doppler.{key}')


def load_gateway_config(path=GATEWAY_CONFIG):
    """Slack delivery, selected native runs and company providers belong here."""
    config = json.loads(Path(path).read_text())
    validate_doppler(config.get('doppler', {}))
    for key in ('slack_team', 'slack_channel', 'slack_owner', 'timezone', 'availability'):
        if not isinstance(config.get(key), str) or not config[key].strip() or '<' in config[key]:
            raise ValueError(f'Fill in gateway configuration: {key}')
    for key, prefix in (('slack_team', 'T'), ('slack_channel', 'C'), ('slack_owner', 'U')):
        if not config[key].startswith(prefix) or not config[key].isalnum():
            raise ValueError(f'Use the Slack ID for {key}')
    ZoneInfo(config['timezone'])
    automations = config.get('automations')
    if not isinstance(automations, dict) or not automations:
        raise ValueError('Select at least one native automation and its display title')
    for ident, title in automations.items():
        if not isinstance(title, str) or not ident.strip() or not title.strip() or '<' in ident + title:
            raise ValueError('Fill in each automation ID and display title')
    if 'new_conversations' in config:
        settings = config['new_conversations']
        if not isinstance(settings, dict) or set(settings) - {'cwd', 'model', 'reasoning_effort'}:
            raise ValueError('new_conversations accepts cwd, model and reasoning_effort')
        cwd = settings.get('cwd')
        if not isinstance(cwd, str) or not Path(cwd).is_absolute() or '<' in cwd:
            raise ValueError('Set new_conversations.cwd to the absolute workspace path on the workstation')
        for key in ('model', 'reasoning_effort'):
            if key in settings and (not isinstance(settings[key], str) or not settings[key].strip()):
                raise ValueError(f'Fill in new_conversations.{key}, or omit it to use the native default')
    if 'github' in config:
        settings = config['github']
        if not isinstance(settings, dict) or set(settings) not in (
                {'app_id', 'installation_id'}, {'app_id', 'installation_id', 'repository', 'repository_id'}):
            raise ValueError('github requires app_id and installation_id, plus repository and repository_id for Git/gh')
        for key in ('app_id', 'installation_id'):
            if not isinstance(settings[key], str) or not settings[key].isdigit():
                raise ValueError(f'Use the numeric GitHub {key} as a string')
        if 'repository' in settings:
            if not isinstance(settings['repository'], str) or not re.fullmatch(
                    r'[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+', settings['repository']):
                raise ValueError('Use owner/repository for github.repository')
            if not isinstance(settings['repository_id'], str) or not settings['repository_id'].isdigit():
                raise ValueError('Use the numeric GitHub repository_id as a string')
    return config


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('role', choices=('workstation', 'gateway'))
    parser.add_argument('path')
    args = parser.parse_args()
    loader = load_workstation_config if args.role == 'workstation' else load_gateway_config
    loader(args.path)
    print(f'{args.role.capitalize()} configuration is valid; credentials and access were not tested.')
