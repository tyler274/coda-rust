#!/usr/bin/env python3
"""Fetch named secrets through a caller's explicit, VM-bound Doppler configuration."""
import json
import urllib.parse
import urllib.request

def request(url, headers, body=None, as_json=True):
    data = json.dumps(body).encode() if body is not None else None
    req = urllib.request.Request(url, data=data, headers=headers)
    with urllib.request.urlopen(req, timeout=25) as response:
        return json.load(response) if as_json else response.read().decode().strip()


def fetch(names, config):
    metadata = 'http://metadata.google.internal/computeMetadata/v1/instance/'
    headers = {'Metadata-Flavor': 'Google'}
    if request(metadata + 'id', headers, as_json=False) != config['instance_id']:
        raise RuntimeError('These credentials are configured for a different VM')
    account_path = metadata + 'service-accounts/default/'
    if request(account_path + 'email', headers, as_json=False) != config['account']:
        raise RuntimeError('Unexpected VM service account')
    query = urllib.parse.urlencode({'audience': config['audience'], 'format': 'full'})
    oidc = request(account_path + 'identity?' + query, headers, as_json=False)
    auth = request('https://api.doppler.com/v3/auth/oidc', {'Content-Type': 'application/json'},
                   {'identity': config['identity'], 'token': oidc})
    query = urllib.parse.urlencode({'project': config['project'], 'config': config['config'],
                                    'format': 'json', 'secrets': ','.join(names)})
    secrets = request('https://api.doppler.com/v3/configs/config/secrets/download?' + query,
                      {'Authorization': 'Bearer ' + auth['token']})
    for name in names:
        value = secrets.get(name)
        if not isinstance(value, str) or not value:
            raise RuntimeError('Required deployment credential is unavailable: ' + name)
    return {name: secrets[name] for name in names}
