#!/usr/bin/env python3
"""Unit coverage for the image's CI-safe environment renderer."""

import os
import subprocess
import shutil
import unittest

from postgres_image_env import render


class RuntimeEnvironmentTests(unittest.TestCase):
    def test_runner_context_and_nested_bash_survive(self):
        baked = render({
            "HOME": "/homeless-shelter", "PWD": "/build", "TMPDIR": "/build",
            "PATH": "/shell/bin", "GITHUB_WORKSPACE": "/build",
            "SSL_CERT_FILE": "/no-cert-file.crt", "NIX_SSL_CERT_FILE": "/no-cert-file.crt",
            "out": "/nix/store/capture", "NIX_BUILD_CORES": "32",
            "nativeBuildInputs": "/nix/store/build-only-input",
            "depsBuildBuild": "/nix/store/build-only-hook",
            "PKG_CONFIG_PATH": "/shell/pkgconfig",
            "shellHook": 'export C_INCLUDE_PATH="/shell/include${C_INCLUDE_PATH:+:}${C_INCLUDE_PATH:-}"',
        })
        script = baked + "\n" + baked + '\n/usr/bin/env -0'
        env = {
            "HOME": "/github/home", "PWD": os.getcwd(), "TMPDIR": "/runner/tmp",
            "PATH": "/action/bin", "GITHUB_WORKSPACE": "/workspace",
            "GITHUB_OUTPUT": "/runner/output", "C_INCLUDE_PATH": "/action/include",
            "SSL_CERT_FILE": "/etc/ssl/certs/ca-bundle.crt",
        }
        result = subprocess.check_output([shutil.which("bash"), "--noprofile", "--norc", "-c", script], env=env)
        actual = dict(item.split("=", 1) for item in result.decode().split("\0") if item)
        for key, value in env.items():
            if key not in {"PATH", "C_INCLUDE_PATH"}:
                self.assertEqual(actual[key], value)
        self.assertEqual(actual["PATH"], "/shell/bin:/action/bin")
        self.assertEqual(actual["C_INCLUDE_PATH"], "/shell/include:/action/include")
        self.assertEqual(actual["PKG_CONFIG_PATH"], "/shell/pkgconfig")
        self.assertNotIn("NIX_BUILD_CORES", actual)
        self.assertNotIn("nativeBuildInputs", actual)
        self.assertNotIn("depsBuildBuild", actual)

    def test_escaping_and_capture_paths(self):
        script = render({
            "PATH": "/shell/bin", "out": "/nix/store/capture",
            "NIX_CFLAGS_COMPILE": " -frandom-seed=abc -isystem /compiler/include",
            "NIX_LDFLAGS": "-rpath /nix/store/capture/lib  -L/library/lib",
            "QUOTED": "a'b\n$(false)",
        })
        result = subprocess.check_output([shutil.which("bash"), "-c", script + '\n/usr/bin/env -0'], env={})
        actual = dict(item.split("=", 1) for item in result.decode().split("\0") if item)
        self.assertEqual(actual["NIX_CFLAGS_COMPILE"], " -isystem /compiler/include")
        self.assertEqual(actual["NIX_LDFLAGS"], " -L/library/lib")
        self.assertEqual(actual["QUOTED"], "a'b\n$(false)")


if __name__ == "__main__":
    unittest.main()
