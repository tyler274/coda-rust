#!/usr/bin/env python3
"""Add workspace contracts without replacing reference paths or generated DTOs.

Only new paths, new operations on existing paths, and new named components are
allowed. Existing definitions can be repeated byte-for-structure unchanged, but
never overridden. This is intentionally not a generic JSON merge patch.
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
    return result


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
