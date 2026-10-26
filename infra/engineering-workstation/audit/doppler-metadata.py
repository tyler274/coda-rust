#!/usr/bin/env python3
"""Read Doppler audit metadata using a short-lived GCE OIDC identity.
No token is persisted or printed. This helper never requests secret values.
"""
import datetime
import json
import urllib.error
import urllib.parse
import urllib.request

IDENTITY = 'a21e6c88-c6b1-4bbc-bf3e-befbce0d5746'
AUDIENCE = 'https://doppler.com/coda-rust-infrastructure-audit'
ACCOUNT = 'coda-rust-security-auditor@coda-engineering.iam.gserviceaccount.com'
PROJECT = 'coda-rust'


def request(url, headers=None, body=None):
    payload = None if body is None else json.dumps(body).encode()
    req = urllib.request.Request(url, data=payload, headers=headers or {})
    with urllib.request.urlopen(req, timeout=25) as response:
        return json.load(response)


def pick(obj, keys):
    return {key: obj[key] for key in keys if key in obj}


def main():
    meta = 'http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/'
    headers = {'Metadata-Flavor': 'Google'}
    req = urllib.request.Request(meta + 'email', headers=headers)
    with urllib.request.urlopen(req, timeout=5) as response:
        actual_account = response.read().decode().strip()
    if actual_account != ACCOUNT:
        raise RuntimeError('Unexpected GCE service account; refusing Doppler authentication')
    query = urllib.parse.urlencode({'audience': AUDIENCE, 'format': 'full'})
    req = urllib.request.Request(meta + 'identity?' + query, headers=headers)
    with urllib.request.urlopen(req, timeout=5) as response:
        oidc = response.read().decode().strip()
    auth = request('https://api.doppler.com/v3/auth/oidc',
                   {'Content-Type': 'application/json'},
                   {'identity': IDENTITY, 'token': oidc})
    token = auth['token']
    headers = {'Authorization': 'Bearer ' + token, 'Accept': 'application/json'}
    results = {'observed_at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
               'gcp_principal': actual_account, 'doppler_identity': IDENTITY,
               'credential_storage': 'memory only; identity token lifetime 900 seconds',
               'checks': []}

    def read(label, path, selector):
        row = {'check': label}
        try:
            data = request('https://api.doppler.com' + path, headers)
            row.update(status='ok', metadata=selector(data))
        except urllib.error.HTTPError as error:
            # Provider error bodies can contain request details; do not emit them.
            row.update(status='unavailable', http_status=error.code)
        results['checks'].append(row)
        return row.get('metadata')

    read('workplace roles', '/v3/workplace/roles',
         lambda d: [pick(v, ['name', 'identifier', 'permissions', 'is_custom_role']) for v in d.get('roles', [])])
    read('project membership', '/v3/projects/project/members?project=' + PROJECT,
         lambda d: [pick(v, ['type', 'slug', 'role', 'roles', 'environments', 'access_all_environments']) for v in d.get('members', [])])
    read('project environments', '/v3/environments?project=' + PROJECT,
         lambda d: [pick(v, ['id', 'slug', 'name', 'project', 'created_at', 'personal_configs']) for v in d.get('environments', [])])
    read('project configs', '/v3/configs?project=' + PROJECT,
         lambda d: [pick(v, ['name', 'root', 'environment', 'project', 'created_at', 'locked', 'initial_fetch_at']) for v in d.get('configs', [])])
    accounts = read('service accounts', '/v3/workplace/service_accounts?per_page=100',
                    lambda d: [pick(v, ['slug', 'name', 'created_at', 'workplace_role']) for v in d.get('service_accounts', [])]) or []
    for account in accounts:
        slug = urllib.parse.quote(account['slug'], safe='')
        base = '/v3/workplace/service_accounts/service_account/' + slug
        read('service account identities: ' + account['name'], base + '/identities',
             lambda d: [pick(v, ['slug', 'name', 'method', 'config', 'ttl_seconds', 'created_at', 'last_seen_at']) for v in d.get('identities', [])])
        read('service account token metadata: ' + account['name'], base + '/tokens',
             lambda d: [pick(v, ['slug', 'name', 'created_at', 'last_seen_at', 'expires_at']) for v in d.get('tokens', [])])
    # Secret values, config logs, and secret rotation details are intentionally not
    # requested: their permissions can include secret-value access.
    print(json.dumps(results, indent=2))


if __name__ == '__main__':
    try:
        main()
    except urllib.error.HTTPError as error:
        raise SystemExit('Authentication or metadata request failed: HTTP ' + str(error.code))
