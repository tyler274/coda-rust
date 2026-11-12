#!/usr/bin/env python3
"""Compare the baked runtime to nix develop and smoke-test it in Docker.

Run inside the agent shell. Pass --node PATH to additionally run an unmodified
GitHub runner Node binary mounted at /__e/node/bin/node (without BASH_ENV).
"""

import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile


# pubsub::testing clears the emulator's environment and keeps PATH but not
# JAVA_HOME, so gcloud's launcher falls back to `which java`. Mirror that.
EMULATOR_JAVA_CHECK = (
    'env -i PATH="$PATH" HOME=/tmp/emulator-home /bin/sh -c '
    '\'java=$(which java) && test -x "$java" && "$java" -version\''
)


def output(command, **kwargs):
    return subprocess.check_output(command, text=True, **kwargs).strip()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--image", default="postgres-ci:latest")
    parser.add_argument("--node", type=Path)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    docker = ["docker", "run", "--rm", "-v", f"{root}:/workspace:ro", "-w", "/workspace"]
    # A local worktree's .git points outside the checkout; Actions checkout is
    # standalone. Expose metadata read-only and override its host worktree path.
    git_dir = output(["git", "rev-parse", "--path-format=absolute", "--git-common-dir"], cwd=root)
    docker += ["-v", f"{git_dir}:{git_dir}:ro", "-e", "GIT_WORK_TREE=/workspace"]
    context = {
        "HOME": "/github/home", "TMPDIR": "/tmp", "TMP": "/tmp", "TEMP": "/tmp",
        "GITHUB_WORKSPACE": "/workspace", "GITHUB_OUTPUT": "/tmp/job-output",
        "GITHUB_ENV": "/tmp/job-env", "RUNNER_TEMP": "/tmp", "ACTION_SENTINEL": "preserved",
        "PATH": "/action/bin:/bin:/usr/bin", "C_INCLUDE_PATH": "", "LIBRARY_PATH": "",
    }
    for key, value in context.items():
        docker += ["-e", f"{key}={value}"]
    shell = docker + [args.image, "bash", "--noprofile", "--norc", "-eo", "pipefail", "-c"]
    dump = "import json, os; print(json.dumps(dict(os.environ)))"
    actual = json.loads(output(shell + [f"python -c '{dump}'"]))
    # Do not leak the agent shell's C_INCLUDE_PATH/LIBRARY_PATH into the reference.
    clean = {key: os.environ[key] for key in ("HOME", "PATH", "NIX_CONFIG") if key in os.environ}
    clean.update(C_INCLUDE_PATH="", LIBRARY_PATH="")
    expected = json.loads(output(["nix", "develop", ".#postgres-ci", "--command", "python", "-c", dump], cwd=root, env=clean))
    prefixes = ("NIX_CC", "NIX_BINTOOLS", "NIX_CFLAGS", "NIX_LDFLAGS", "NIX_HARDENING", "NIX_PKG_CONFIG", "CARGO_TARGET_", "PKG_CONFIG", "PYTHON")
    names = {"CC", "CXX", "AR", "AS", "LD", "NM", "RANLIB", "STRIP", "JAVA_HOME", "OPENSSL_NO_VENDOR", "C_INCLUDE_PATH", "LIBRARY_PATH"}
    names.update(key for key in expected if key.startswith(prefixes))
    for key in sorted(names):
        left, right = expected.get(key), actual.get(key)
        if key == "NIX_CFLAGS_COMPILE":
            left = re.sub(r" -frandom-seed=\S+", "", left)
        if key == "NIX_LDFLAGS":
            left = re.sub(r"-rpath " + re.escape(str(root)) + r"/outputs/out/lib ?", "", left)
        assert left == right, f"{key} differs:\n shell: {left}\n image: {right}"
    print(f"PASS: {len(names)} compiler/tool variables match nix develop (capture seed/self-rpath excluded)")
    for key, value in context.items():
        if key not in {"PATH", "C_INCLUDE_PATH", "LIBRARY_PATH"}:
            assert actual[key] == value, (key, actual[key], value)
    assert actual["PWD"] == "/workspace"
    assert actual["PATH"].endswith(":" + context["PATH"])
    # The shell's PATH is an ordered prefix; the remainder belongs to the runner.
    shell_path = json.loads(output(["nix", "print-dev-env", ".#postgres-ci", "--json"], cwd=root))["variables"]["PATH"]["value"]
    assert actual["PATH"] == shell_path + ":" + context["PATH"]
    nested = json.loads(output(shell + [f'bash -c \'python -c "{dump}"\'']))
    for key in names | {"PATH", "HOME", "PWD", "TMPDIR"}:
        assert nested.get(key) == actual.get(key), f"nested bash changed {key}"
    print("PASS: runner context, ordered PATH and nested bash preserved")
    # The image runs as its unprivileged runner account, which initdb accepts
    # (the crash-restart proof calls it and refuses root).
    subprocess.run(docker + ["--network", "none", args.image, "bash", "--noprofile", "--norc", "-eo", "pipefail", "-c",
                             'test "$(id -u)" != 0 && test "$(id -un)" = runner && '
                             'initdb -U postgres -A trust -D /tmp/initdb-smoke >/dev/null && '
                             'test -f /tmp/initdb-smoke/PG_VERSION'], check=True)
    print("PASS: image runs as the unprivileged runner account and initdb accepts it")
    import yaml
    workflow = yaml.safe_load((root / ".github/workflows/ci.yml").read_text())
    setup = next(step["run"] for step in workflow["jobs"]["postgres"]["steps"]
                 if step.get("name") == "Prepare container workspace")
    # A checkout owned by another uid must stop the job before any build.
    if os.getuid() != 1001:
        refused = subprocess.run(shell + [setup], text=True, capture_output=True)
        assert refused.returncode == 1 and "needs the unprivileged owner" in refused.stdout, refused
        print("PASS: workspace setup refuses a job uid that does not own the checkout")
    # On the runner the job's uid owns the checkout and the mounted HOME.
    # Replay that locally as the checkout's owner with a writable HOME.
    home = Path(tempfile.mkdtemp(prefix="postgres-ci-home-"))
    owner = docker + ["--user", f"{os.getuid()}:{os.getgid()}", "-v", f"{home}:/github/home"]
    subprocess.run(owner + [args.image, "bash", "--noprofile", "--norc", "-eo", "pipefail", "-c",
        setup + '\n'
        'source "$GITHUB_ENV"; export CARGO_HOME; '
        'test "$CARGO_HOME" = "$PWD/.ci-cargo" && '
        'test "$(git config --global --get-all safe.directory)" = "$PWD" && '
        'bash -c \'test "$HOME" = /github/home && test "$CARGO_HOME" = "$PWD/.ci-cargo" && git ls-files --error-unmatch Cargo.toml\' && '
        'test -r "$SSL_CERT_FILE" && python scripts/check-sql.py && cargo --version && rustc --version && createdb --version && pkg-config --modversion openssl && java -version && git --version'
    ], check=True)
    shutil.rmtree(home)
    print("PASS: workflow setup as the checkout owner sets exact trust and Cargo path across nested BASH_ENV")
    subprocess.run(shell + [EMULATOR_JAVA_CHECK], check=True)
    print("PASS: `which java` resolves under the Pub/Sub emulator's cleared environment")
    # Preserve native build capability and archive tools used by Actions cache.
    subprocess.run(["docker", "run", "--rm", "--network", "none", args.image,
                    "bash", "--noprofile", "--norc", "-eo", "pipefail", "-c",
                    'cargo new --vcs none /tmp/image-cargo-smoke && cd /tmp/image-cargo-smoke && '
                    'cargo run --offline && gcc -x c -o /tmp/c-smoke - <<< "int main(void) { return 0; }" && '
                    '/tmp/c-smoke && tar -cf /tmp/smoke.tar Cargo.toml && '
                    'gzip -c /tmp/smoke.tar | gzip -dc | tar -tf - && '
                    'zstd -q -c /tmp/smoke.tar | zstd -q -dc | tar -tf - && '
                    'test -x "$(git --exec-path)/git-remote-https"'], check=True)
    if args.node:
        node = str(args.node.resolve())
        subprocess.run(docker + ["-v", f"{node}:/__e/node/bin/node:ro", args.image, "/__e/node/bin/node", "-e", "console.log(process.version); require('child_process').execFileSync('/usr/bin/env', ['git', '--version'], {stdio:'inherit'}); console.log('PASS: runner Node and child process without BASH_ENV')"], check=True)
    else:
        print("NOT TESTED: mounted GitHub runner Node (supply --node)")


if __name__ == "__main__":
    main()
