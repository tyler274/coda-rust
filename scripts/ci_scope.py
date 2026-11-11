#!/usr/bin/env python3
"""Select CI from the complete Git change, and verify the selected job results."""

import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import sys


FULL_JOBS = ("ci", "foundations", "images", "postgres-image", "postgres", "observability", "cargo-deny")
MOBILE_JOBS = ("android-emulator",)
SANDBOX_JOBS = ("sandbox",)
# Pull requests and main pushes do not run the Android emulator yet; only a
# manual dispatch does. Set True to run it whenever a change touches the APK's
# build (mobile_path).
MOBILE_ON_CHANGE = False
# Workspace crates in the Android APK's build graph. The android-emulator job
# runs `check-mobile-crates` against `cargo tree`, so a new dependency fails
# that job until it is listed here.
MOBILE_CRATES = (
    "android", "duet-app", "duet-client", "duet-health", "duet-notify", "duet-page",
    "duet-wire", "http", "mobile", "physician-app", "physician-client", "physician-wire", "ui",
)
MOBILE_FILES = {
    "Cargo.lock", "Cargo.toml", "flake.lock", "flake.nix", "rust-toolchain.toml",
    ".cargo/config.toml", "Dioxus.toml", "nix/android-shell.nix", "nix/rust-toolchain.nix",
    # Compiled into the duet-client, duet-health and duet-notify build scripts.
    "platform/ios/build_bridge.rs",
    "scripts/ci_scope.py", "scripts/wire-duet-android.py", ".github/workflows/ci.yml",
}


def lightweight_path(path):
    return (
        path in {"README.md", "AGENTS.md", "CLAUDE.md"}
        or path.startswith((".agents/skills/", ".claude/skills/", "book/"))
        or (path.startswith(("intent/", "verification/")) and path.endswith(".md"))
        or (path.startswith("infra/") and path.endswith("/README.md"))
    )


SANDBOX_DIR = "crates/workspace-tools/sandbox/"


def sandbox_path(path):
    # coda-sandbox has its own flake, whose checks build, test and lint it;
    # no product crate depends on it, so it selects only the sandbox job. A
    # dependency change still reaches the root Cargo.lock, which selects full.
    return path.startswith(SANDBOX_DIR)


def sandbox_input(path):
    # The sandbox flake builds from the root manifest and lockfile, and the
    # workflow defines the sandbox job, so changing either reruns it too.
    return sandbox_path(path) or path in {"Cargo.toml", "Cargo.lock", ".github/workflows/ci.yml"}


def dependency_path(path):
    # Changing these rebuilds the Nix dependency derivations from source.
    return (
        path in {"Cargo.lock", "Cargo.toml", "flake.lock", "flake.nix", "rust-toolchain.toml"}
        or path.endswith("/Cargo.toml")
        or path.startswith("nix/")
    )


def mobile_path(path):
    return (
        path in MOBILE_FILES
        or path.startswith("platform/android/")
        or any(path.startswith(f"crates/{crate}/") for crate in MOBILE_CRATES)
    )


def unlisted_mobile_crates(tree, workspace):
    """Workspace crate directories in `cargo tree --prefix none` output missing from MOBILE_CRATES."""
    crates = set()
    for line in tree.splitlines():
        match = re.search(r"\((/[^)]+)\)", line)
        if match and Path(match.group(1)).is_relative_to(workspace / "crates"):
            crates.add(Path(match.group(1)).relative_to(workspace / "crates").as_posix())
    return sorted(crates - set(MOBILE_CRATES))


def git(*args):
    return subprocess.check_output(["git", *args], stderr=subprocess.PIPE)


def select(event_name, event):
    """Return (full, deps, mobile, sandbox, reason). deps means a dependency build is likely cold."""
    full, deps, mobile, sandbox, reason = classify(event_name, event)
    if mobile and event_name != "workflow_dispatch" and not MOBILE_ON_CHANGE:
        return (full, deps, False, sandbox,
                reason + "; Android emulator is manual-only, dispatch CI to run it")
    return full, deps, mobile, sandbox, reason


def classify(event_name, event):
    if event_name == "workflow_dispatch":
        return True, False, True, True, "manual qualification"
    if event_name == "schedule":
        return True, False, False, True, "scheduled cache warm"
    try:
        if event_name == "pull_request":
            base = event["pull_request"]["base"]["sha"]
            head = event["pull_request"]["head"]["sha"]
        elif event_name == "push":
            base, head = event["before"], event["after"]
        elif event_name == "merge_group":
            # The queue's candidate commit and the commit it was built on:
            # main, or the candidate of the entry ahead.
            base = event["merge_group"]["base_sha"]
            head = event["merge_group"]["head_sha"]
        else:
            return True, False, True, True, "event has no supported comparison"
        if not all(isinstance(sha, str) and re.fullmatch(r"[0-9a-f]{40}", sha)
                   and sha != "0" * 40 for sha in (base, head)):
            return True, False, True, True, "comparison revisions unavailable"
        if event_name == "pull_request":
            base = git("merge-base", base, head).decode().strip()
        # No rename detection: both the deletion and addition must qualify.
        # NUL separation preserves spaces/newlines; Git has no API file limit.
        changed = git("diff", "--name-only", "--no-renames", "-z", base, head, "--")
    except (KeyError, TypeError, subprocess.CalledProcessError):
        return True, False, True, True, "complete comparison unavailable"
    paths = [os.fsdecode(path) for path in changed.split(b"\0") if path]
    if not paths:
        return True, False, True, True, "empty comparison"
    sandbox = any(sandbox_input(path) for path in paths)
    product = [path for path in paths if not sandbox_path(path)]
    if all(lightweight_path(path) for path in product):
        reason = ("only coda-sandbox, engineering skills and documentation changed" if sandbox
                  else "only engineering skills and documentation changed")
        return False, False, False, sandbox, reason
    deps = any(dependency_path(path) for path in product)
    mobile = any(mobile_path(path) for path in product)
    return True, deps, mobile, sandbox, "change includes product, build, or other files"


def check_results(needs):
    scope = needs.get("scope", {})
    outputs = scope.get("outputs", {})
    full, mobile, sandbox = outputs.get("full"), outputs.get("mobile"), outputs.get("sandbox")
    if scope.get("result") != "success" \
            or any(flag not in {"true", "false"} for flag in (full, mobile, sandbox)):
        raise ValueError("CI selection did not complete successfully")
    expected = {"lightweight": "success"}
    expected.update({job: "success" if full == "true" else "skipped" for job in FULL_JOBS})
    expected.update({job: "success" if mobile == "true" else "skipped" for job in MOBILE_JOBS})
    expected.update({job: "success" if sandbox == "true" else "skipped" for job in SANDBOX_JOBS})
    failures = [f"{job}: expected {result}, got {needs.get(job, {}).get('result', 'missing')}"
                for job, result in expected.items() if needs.get(job, {}).get("result") != result]
    if failures:
        raise ValueError("; ".join(failures))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("select", "result", "check-mobile-crates"))
    args = parser.parse_args()
    if args.command == "check-mobile-crates":
        missing = unlisted_mobile_crates(sys.stdin.read(), Path(__file__).resolve().parents[1])
        if missing:
            sys.exit("The Android APK builds crates that ci_scope.py MOBILE_CRATES does not list: "
                     + ", ".join(missing))
        print("MOBILE_CRATES covers every workspace crate in the Android build.")
        return
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
    full, deps, mobile, sandbox, reason = select(os.environ.get("GITHUB_EVENT_NAME", ""), event)
    flags = {"full": full, "deps": deps, "mobile": mobile, "sandbox": sandbox}
    output = "".join(f"{name}={str(value).lower()}\n" for name, value in flags.items())
    print(f"CI scope: {'full' if full else 'lightweight'} ({reason})"
          + ("; dependencies changed" if deps else "")
          + ("; Android emulator selected" if mobile else "")
          + ("; coda-sandbox selected" if sandbox else ""))
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
