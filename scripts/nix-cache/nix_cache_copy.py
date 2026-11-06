"""Preseed repeated NAR contents without serializing the entire cache upload."""
import json


def seed_paths(raw, closure, error):
    """Return prerequisite-first seeds for NAR hashes shared by distinct paths.

    Nix's concurrent HEAD/PUT sequence can race for identical content even
    inside one publisher. The immutable GCS role correctly refuses the second
    PUT. Seed one representative and its references before the parallel copy.
    """
    try:
        parsed = json.loads(raw)
        info = ({item['path']: item for item in parsed} if isinstance(parsed, list) else parsed)
        if not isinstance(info, dict) or set(info) != set(closure):
            raise ValueError()
        groups = {}
        for path, item in info.items():
            digest = item['narHash']
            refs = item['references']
            if not isinstance(digest, str) or not digest or not isinstance(refs, list):
                raise ValueError()
            if any(not isinstance(ref, str) or ref not in info for ref in refs):
                raise ValueError()
            groups.setdefault(digest, []).append(path)
    except (ValueError, KeyError, TypeError):
        raise error('Invalid NAR metadata for selected closure') from None
    seeds = sorted(min(paths) for paths in groups.values() if len(paths) > 1)
    result, visited, active = [], set(), set()

    def visit(path):
        if path in visited:
            return
        if path in active:
            raise error('Unexpected reference cycle in selected closure')
        active.add(path)
        for ref in sorted(info[path]['references']):
            if ref != path:
                visit(ref)
        active.remove(path)
        visited.add(path)
        result.append(path)

    for path in seeds:
        visit(path)
    return result


def copy_closure(run, env, url, data, label, error):
    paths = data.splitlines()
    raw = run(['nix', 'path-info', '--json', '--stdin'], env=env, data=data)
    seeds = seed_paths(raw, paths, error)
    print(f'Nix cache duplicate-content preseed_paths={len(seeds)}', flush=True)
    for index, path in enumerate(seeds, 1):
        # References were seeded first; --no-recursive keeps this invocation
        # from reintroducing parallel writes for identical dependency contents.
        run(['nix', 'copy', '--no-recursive', '--to', url, '--stdin'],
            env=env, data=path + '\n', label=label)
        print(f'Nix cache preseed_completed={index} preseed_total={len(seeds)}', flush=True)
    run(['nix', 'copy', '--to', url, '--stdin'], env=env, data=data, label=label)
