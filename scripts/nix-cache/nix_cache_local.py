"""Explicit trusted-local policy, separate from protected hosted-main publication."""
import re


def authorize(env, expected_sha, acknowledged, run, error):
    """Accident guards only: possessing the signing key remains publish authority."""
    if env.get('ACT') != 'true' or env.get('GITHUB_ACTIONS') != 'true':
        raise error('Local publishing requires an explicit act run')
    if not acknowledged:
        raise error('Local publishing requires --acknowledge-local-write')
    if not re.fullmatch(r'[0-9a-f]{40}', expected_sha or ''):
        raise error('Local publishing requires a full expected commit SHA')
    if run(['git', 'rev-parse', 'HEAD'], env=env) != expected_sha:
        raise error('Local checkout does not match the expected commit')
    if run(['git', 'status', '--porcelain', '--untracked-files=normal'], env=env):
        raise error('Local publishing requires a clean checkout, including untracked source')
    return expected_sha
