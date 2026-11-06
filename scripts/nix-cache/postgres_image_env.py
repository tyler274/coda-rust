#!/usr/bin/env python3
"""Render the shell's setup-hook exports as a CI-safe BASH_ENV.

Run by the shell derivation after stdenv setup, not by the image builder's shell.
Only derivation bookkeeping is dropped; compiler/tool setup is not reconstructed.
"""

import os
import re
import shlex
import sys

# These describe the Nix builder, not the job. In particular never replace the
# runner's home, working directory, temporary directories, or Actions variables.
BUILD_ONLY = {
    "BASH_ENV", "HOME", "OLDPWD", "PWD", "SHLVL", "_", "PATH",
    "TMP", "TMPDIR", "TEMP", "TEMPDIR", "TZ", "TERM", "LC_CTYPE", "SOURCE_DATE_EPOCH",
    "SSL_CERT_FILE", "NIX_SSL_CERT_FILE",
    "NIX_BUILD_TOP", "NIX_BUILD_CORES", "NIX_LOG_FD", "NIX_ENFORCE_PURITY",
    "NIX_REMOTE", "NIX_STORE", "NIX_BUILD_UID", "NIX_BUILD_GID",
    "builder", "args", "out", "outputs", "name", "system", "stdenv",
    "buildCommand", "buildPhase", "phases", "shellHook", "preferLocalBuild",
    "allowSubstitutes", "__structuredAttrs", "__sandboxProfile",
    "__darwinAllowLocalNetworking", "__impureHostDeps",
    "__propagatedImpureHostDeps", "__ignoreNulls",
    # The activated paths/flags already retain runtime inputs. These lists are
    # instructions to stdenv's builder, not inputs to Cargo or the job's tools.
    "buildInputs", "nativeBuildInputs", "propagatedBuildInputs", "propagatedNativeBuildInputs",
    "depsBuildBuild", "depsBuildBuildPropagated", "depsBuildTarget", "depsBuildTargetPropagated",
    "depsHostHost", "depsHostHostPropagated", "depsTargetTarget", "depsTargetTargetPropagated",
}


def render(environment: dict[str, str]) -> str:
    lines = [
        "# Generated from postgres-ci stdenv setup; do not edit.",
        # Child bash processes inherit the activated environment. Avoid repeating
        # PATH and shellHook prefixes while preserving later GITHUB_PATH additions.
        'if [[ ${CODA_POSTGRES_CI_ENV_ACTIVE:-} != 1 ]]; then',
    ]
    for name, value in sorted(environment.items()):
        if name in BUILD_ONLY or not re.fullmatch(r"[a-zA-Z_][a-zA-Z0-9_]*", name):
            continue
        if name.startswith(("GITHUB_", "RUNNER_", "buildCommandPath", "passAsFile")):
            continue
        if name == "NIX_CFLAGS_COMPILE":
            # stdenv seeds this from the capture derivation's output hash. It is
            # not a shell input and differs from print-dev-env's capture hash.
            value = re.sub(r" -frandom-seed=\S+", "", value)
        if name == "NIX_LDFLAGS":
            # A capture output is a file, not a library directory. nix develop
            # relocates this to $PWD/outputs/out/lib; neither is a shell library.
            value = re.sub(r"-rpath " + re.escape(environment["out"]) + r"/lib ?", "", value)
        lines.append(f"export {name}={shlex.quote(value)}")
    lines.append(f"export PATH={shlex.quote(environment['PATH'])}\"${{PATH:+:$PATH}}\"")
    # Preserve evaluation at runtime: this hook can reference existing job vars.
    lines.append(environment.get("shellHook", ""))
    lines.extend(["export CODA_POSTGRES_CI_ENV_ACTIVE=1", "fi", ""])
    return "\n".join(lines)


if __name__ == "__main__":
    sys.stdout.write(render(dict(os.environ)))
