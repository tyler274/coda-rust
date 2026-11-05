#!/usr/bin/env python3
"""Select CI from the complete Git change, and verify the selected job results."""

import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import sys


FULL_JOBS = ("ci", "foundations", "images", "postgres", "observability", "cargo-deny")


def lightweight_path(path):
    return (
        path in {"README.md", "AGENTS.md", "CLAUDE.md"}
        or path.startswith((".agents/skills/", ".claude/skills/", "book/"))
        or (path.startswith(("intent/", "verification/")) and path.endswith(".md"))
        or (path.startswith("infra/") and path.endswith("/README.md"))
    )


def dependency_path(path):
    # Changing these rebuilds the Nix dependency derivations from source.
    return (
        path in {"Cargo.lock", "Cargo.toml", "flake.lock", "flake.nix"}
        or path.endswith("/Cargo.toml")
        or path.startswith("nix/")
    )


def git(*args):
    return subprocess.check_output(["git", *args], stderr=subprocess.PIPE)


def select(event_name, event):
    """Return (full, deps, reason). deps means a dependency build is likely cold."""
    if event_name == "workflow_dispatch":
        return True, False, "manual qualification"
    try:
        if event_name == "pull_request":
            base = event["pull_request"]["base"]["sha"]
            head = event["pull_request"]["head"]["sha"]
        elif event_name == "push":
            base, head = event["before"], event["after"]
        else:
            return True, False, "event has no supported comparison"
        if not all(isinstance(sha, str) and re.fullmatch(r"[0-9a-f]{40}", sha)
                   and sha != "0" * 40 for sha in (base, head)):
            return True, False, "comparison revisions unavailable"
        if event_name == "pull_request":
            base = git("merge-base", base, head).decode().strip()
        # No rename detection: both the deletion and addition must qualify.
        # NUL separation preserves spaces/newlines; Git has no API file limit.
        changed = git("diff", "--name-only", "--no-renames", "-z", base, head, "--")
    except (KeyError, TypeError, subprocess.CalledProcessError):
        return True, False, "complete comparison unavailable"
    paths = [os.fsdecode(path) for path in changed.split(b"\0") if path]
    if not paths:
        return True, False, "empty comparison"
    if all(lightweight_path(path) for path in paths):
        return False, False, "only engineering skills and documentation changed"
    deps = any(dependency_path(path) for path in paths)
    return True, deps, "change includes product, build, or other files"


def check_results(needs):
    scope = needs.get("scope", {})
    full = scope.get("outputs", {}).get("full")
    if scope.get("result") != "success" or full not in {"true", "false"}:
        raise ValueError("CI selection did not complete successfully")
    expected = {"lightweight": "success"}
    expected.update({job: "success" if full == "true" else "skipped" for job in FULL_JOBS})
    failures = [f"{job}: expected {result}, got {needs.get(job, {}).get('result', 'missing')}"
                for job, result in expected.items() if needs.get(job, {}).get("result") != result]
    if failures:
        raise ValueError("; ".join(failures))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("select", "result"))
    args = parser.parse_args()
    if args.command == "result":
        try:
            check_results(json.loads(os.environ["CI_NEEDS"]))
        except (KeyError, ValueError, TypeError, AttributeError) as error:
            sys.exit(f"CI result failed: {error}")
        print("All checks selected for this change passed.")
        return

    try:
        event = json.loads(Path(os.environ["GITHUB_EVENT_PATH"]).read_text())
    except (KeyError, OSError, ValueError):
        event = {}
    full, deps, reason = select(os.environ.get("GITHUB_EVENT_NAME", ""), event)
    output = f"full={str(full).lower()}\ndeps={str(deps).lower()}\n"
    print(f"CI scope: {'full' if full else 'lightweight'} ({reason})"
          + ("; dependencies changed" if deps else ""))
    if os.environ.get("GITHUB_OUTPUT"):
        with open(os.environ["GITHUB_OUTPUT"], "a") as stream:
            stream.write(output)
    else:
        print(output, end="")
    if os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(os.environ["GITHUB_STEP_SUMMARY"], "a") as stream:
            stream.write(f"CI scope: **{'full' if full else 'lightweight'}** — {reason}.\n")


if __name__ == "__main__":
    main()
