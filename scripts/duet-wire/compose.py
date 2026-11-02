#!/usr/bin/env python3
"""Add workspace contracts without replacing reference paths or generated DTOs.

Only new paths, new operations on existing paths, and new named components are
allowed. Existing definitions can be repeated byte-for-structure unchanged, but
never overridden. After composition, retired person-to-person sharing routes
and their exclusive schema graph are removed. This is not a generic merge patch.
"""
import argparse
import copy
import json
from pathlib import Path


def compose(base: dict, additions: dict) -> dict:
    if set(additions) - {"paths", "components"}:
        raise ValueError("additions may contain only paths and components")
    result = copy.deepcopy(base)
    for section, entries in additions.items():
        if not isinstance(entries, dict):
            raise ValueError(f"{section} must be an object")
        for name, fields in entries.items():
            if not isinstance(fields, dict):
                raise ValueError(f"{section}/{name} must be an object")
            target = result.setdefault(section, {}).setdefault(name, {})
            for key, value in fields.items():
                if key in target and target[key] != value:
                    raise ValueError(f"cannot replace reference contract: {section}/{name}/{key}")
                target[key] = copy.deepcopy(value)
    return without_person_sharing(result)


def schema_refs(value):
    if isinstance(value, dict):
        ref = value.get("$ref", "")
        if ref.startswith("#/components/schemas/"):
            yield ref.rsplit("/", 1)[1]
        for child in value.values():
            yield from schema_refs(child)
    elif isinstance(value, list):
        for child in value:
            yield from schema_refs(child)


def without_person_sharing(spec: dict) -> dict:
    """Retire only person-to-person routes and their exclusive schema graph.

    The pinned source remains historical reference. Practice connections and
    doctor workspaces have separate route families and keep their contracts.
    """
    paths = spec.get("paths", {})
    retired = [paths.pop(path) for path in list(paths) if path.startswith("/api/sharing/")]
    schemas = spec.get("components", {}).get("schemas", {})

    def reachable(roots):
        found = set()
        pending = list(roots)
        while pending:
            name = pending.pop()
            if name not in found:
                found.add(name)
                pending.extend(schema_refs(schemas.get(name, {})))
        return found

    candidates = reachable(schema_refs(retired))
    # Keep schemas used by any surviving route or independent model, including
    # reusable validation, account, evidence and encryption infrastructure.
    retained = reachable([*schema_refs(paths), *(set(schemas) - candidates)])
    for name in candidates - retained:
        schemas.pop(name, None)
    return spec


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("base", type=Path)
    parser.add_argument("additions", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    result = compose(json.loads(args.base.read_text()), json.loads(args.additions.read_text()))
    args.output.write_text(json.dumps(result, indent=2) + "\n")


if __name__ == "__main__":
    main()
