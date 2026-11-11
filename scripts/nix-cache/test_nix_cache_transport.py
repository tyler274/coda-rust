#!/usr/bin/env python3
"""Local create-only S3 transport probe; no cloud service or real credentials.

Opt in: NIX_CACHE_TRANSPORT_TEST=1 python -m unittest discover -s scripts/nix-cache \
    -p test_nix_cache_transport.py -v
Optional NIX_CACHE_TRANSPORT_NIX selects an already available Nix executable.
This tests a hypothesized failure mechanism, not the cause of a historical run.
"""

import contextlib
import hashlib
import http.server
import json
import os
from pathlib import Path
import shlex
import signal
import subprocess
import tempfile
import threading
import time
import unittest
import urllib.parse

import nix_cache as cache


class CacheServer(http.server.ThreadingHTTPServer):
    daemon_threads = True

    def __init__(self, *, race=False, interrupt=False):
        super().__init__(("127.0.0.1", 0), CacheHandler)
        # The Nix disk-cache identity can omit endpoint parameters. Give each
        # server a distinct bucket too, so independent fixtures cannot alias.
        self.bucket = f"test-cache-{self.server_port}"
        self.objects = {f"/{self.bucket}/nix-cache-info": b"StoreDir: /nix/store\n"}
        self.events = []
        self.lock = threading.Lock()
        self.barrier = threading.Barrier(2) if race else None
        self.narinfo_waiting = threading.Event()
        self.release_narinfo = threading.Event()
        self.interrupt = interrupt
        self.thread = threading.Thread(target=self.serve_forever, daemon=True)

    def __enter__(self):
        self.thread.start()
        return self

    def __exit__(self, *args):
        self.release_narinfo.set()
        self.shutdown()
        self.server_close()
        self.thread.join()
        print("HTTP events:", self.events, flush=True)

    @property
    def destination(self):
        return (f"s3://{self.bucket}?endpoint=http://127.0.0.1:{self.server_port}"
                "&scheme=http&region=us-east-1&multipart-upload=false&compression=none")


class CacheHandler(http.server.BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def log_message(self, *args):
        pass

    def reply(self, status, body=b""):
        with self.server.lock:
            self.server.events.append((self.command, self.path, status))
        self.send_response(status)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Content-Type", "application/xml")
        self.send_header("ETag", '"' + hashlib.md5(body).hexdigest() + '"')
        self.end_headers()
        if self.command != "HEAD":
            with contextlib.suppress(BrokenPipeError, ConnectionResetError):
                self.wfile.write(body)

    def read_object(self):
        key = urllib.parse.urlsplit(self.path).path
        with self.server.lock:
            body = self.server.objects.get(key)
        # Snapshot absence BEFORE either HEAD is allowed to complete. Both
        # upload workers must now PUT the same immutable content-addressed key.
        if self.command == "HEAD" and "/nar/" in key and body is None and self.server.barrier:
            try:
                self.server.barrier.wait(timeout=10)
            except threading.BrokenBarrierError:
                self.reply(500, b"HEAD race barrier was not reached by two workers")
                return
        self.reply(404 if body is None else 200, body or b"")

    do_GET = read_object
    do_HEAD = read_object

    def do_PUT(self):
        key = urllib.parse.urlsplit(self.path).path
        if self.headers.get("Transfer-Encoding", "").lower() == "chunked":
            chunks = []
            while True:
                size = int(self.rfile.readline().split(b";", 1)[0], 16)
                if size == 0:
                    while self.rfile.readline() != b"\r\n":
                        pass
                    break
                chunks.append(self.rfile.read(size))
                self.rfile.read(2)
            body = b"".join(chunks)
        else:
            body = self.rfile.read(int(self.headers.get("Content-Length", 0)))
        if key.endswith(".narinfo") and self.server.interrupt:
            self.server.narinfo_waiting.set()
            self.server.release_narinfo.wait(timeout=20)
            # Interruption deliberately leaves uploaded NAR but no narinfo.
            self.reply(403, b"<Error><Code>AccessDenied</Code><Message>publish interrupted</Message></Error>")
            return
        with self.server.lock:
            exists = key in self.server.objects
            if not exists:
                self.server.objects[key] = body
        if exists:
            self.reply(403, b"<Error><Code>AccessDenied</Code><Message>create-only: object exists</Message></Error>")
        else:
            self.reply(200)


@unittest.skipUnless(os.environ.get("NIX_CACHE_TRANSPORT_TEST") == "1", "requires explicit real-Nix opt-in")
class NixCacheTransport(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="nix-cache-transport-")
        self.addCleanup(self.temp.cleanup)
        root = Path(self.temp.name)
        self.nix = os.environ.get("NIX_CACHE_TRANSPORT_NIX", "nix")
        self.env = {key: value for key, value in os.environ.items()
                    if not key.startswith(("AWS_", "NIX_")) and key.lower() not in
                    {"http_proxy", "https_proxy", "all_proxy", "no_proxy"}}
        self.env.update({
            "HOME": str(root), "XDG_CACHE_HOME": str(root / "cache"),
            "NIX_CONF_DIR": str(root), "NIX_USER_CONF_FILES": "/dev/null",
            "NIX_CONFIG": "experimental-features = nix-command\nsubstituters =\n",
            "AWS_ACCESS_KEY_ID": "synthetic-local-access", "AWS_SECRET_ACCESS_KEY": "synthetic-local-secret",
            "AWS_EC2_METADATA_DISABLED": "true", "AWS_REGION": "us-east-1",
            "AWS_CONFIG_FILE": str(root / "absent-config"),
            "AWS_SHARED_CREDENTIALS_FILE": str(root / "absent-credentials"),
            "NO_PROXY": "*",
        })
        self.source = "local?root=" + str(root / "store")
        payload = root / "payload"
        payload.write_text("identical deterministic NAR payload\n")
        self.paths = [self.run_nix(["--store", self.source, "store", "add-path", "--name", name, str(payload)]).stdout.strip()
                      for name in ("transport-a", "transport-b")]
        self.assertNotEqual(*self.paths)
        hashes = [self.run_nix(["--store", self.source, "path-info", "--json", "--json-format", "1", path]).stdout for path in self.paths]
        self.assertEqual(*(next(iter(json.loads(info).values()))["narHash"] for info in hashes))
        print("Nix version:", self.run_nix(["--version"]).stdout.strip(), flush=True)

    def command(self, args):
        return [self.nix, "--offline", *args]

    def run_nix(self, args, *, success=True):
        cmd = self.command(args)
        result = subprocess.run(cmd, env=self.env, capture_output=True, text=True, timeout=45)
        print("COMMAND:", shlex.join(cmd), "\nEXIT:", result.returncode,
              "\nSTDOUT:", result.stdout, "STDERR:", result.stderr, flush=True)
        if success:
            self.assertEqual(result.returncode, 0, result.stderr)
        return result

    def copy_args(self, server, paths=None, options=()):
        return ["copy", "--from", self.source, "--to", server.destination, *options, *(paths or self.paths)]

    def snapshot(self, server):
        with server.lock:
            return dict(server.objects)

    def assert_preserved(self, before, server):
        after = self.snapshot(server)
        for key, body in before.items():
            self.assertEqual(after[key], body, key)

    def test_concurrent_failure_and_retry(self):
        # These controls are deliberately tested, not assumed to serialize
        # binary-cache uploads (whose processGraph pool uses hardware threads).
        for options in ((), ("--max-jobs", "1"), ("--option", "max-substitution-jobs", "1")):
            with self.subTest(options=options), CacheServer(race=True) as server:
                result = self.run_nix(self.copy_args(server, options=options), success=False)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("403", result.stderr)
                puts = [status for method, key, status in server.events if method == "PUT" and "/nar/" in key]
                self.assertEqual(sorted(puts), [200, 403])
                before = self.snapshot(server)
                server.barrier = None
                self.run_nix(self.copy_args(server))
                self.assert_preserved(before, server)
                self.assertEqual(sum(key.endswith(".narinfo") for key in server.objects), 2)

    def test_serialized_paths_succeed_and_repeat_preserves_objects(self):
        with CacheServer() as server:
            for path in self.paths:
                self.run_nix(self.copy_args(server, [path]))
            self.assertEqual(sum("/nar/" in key for key in server.objects), 1)
            self.assertEqual(sum(key.endswith(".narinfo") for key in server.objects), 2)
            before = self.snapshot(server)
            put_count = sum(method == "PUT" for method, _, _ in server.events)
            self.run_nix(self.copy_args(server))
            self.assert_preserved(before, server)
            self.assertEqual(sum(method == "PUT" for method, _, _ in server.events), put_count)

    def test_interrupted_publish_retry_preserves_nar(self):
        with CacheServer(interrupt=True) as server:
            cmd = self.command(self.copy_args(server, [self.paths[0]]))
            print("INTERRUPT COMMAND:", shlex.join(cmd), flush=True)
            proc = subprocess.Popen(cmd, env=self.env, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
            try:
                self.assertTrue(server.narinfo_waiting.wait(timeout=15), "Nix did not reach narinfo publish")
                before = self.snapshot(server)
                self.assertEqual(sum("/nar/" in key for key in before), 1)
                self.assertFalse(any(key.endswith(".narinfo") for key in before))
                proc.send_signal(signal.SIGINT)
                server.release_narinfo.set()
                stdout, stderr = proc.communicate(timeout=15)
                print("INTERRUPTED EXIT:", proc.returncode, "\nSTDOUT:", stdout, "STDERR:", stderr, flush=True)
                self.assertNotEqual(proc.returncode, 0)
            finally:
                if proc.poll() is None:
                    proc.kill()
                    proc.communicate()
            server.interrupt = False
            self.run_nix(self.copy_args(server))
            self.assert_preserved(before, server)
            self.assertEqual(sum(key.endswith(".narinfo") for key in server.objects), 2)
            self.assertEqual(sum(method == "PUT" and "/nar/" in key for method, key, _ in server.events), 1)


@unittest.skipUnless(os.environ.get("NIX_CACHE_TRANSPORT_TEST") == "1", "requires explicit real-Nix opt-in")
class NixCacheSelection(unittest.TestCase):
    """The reader's production substituter line against two local caches.

    `public` stands in for cache.nixos.org (Priority 40); the S3 server is the
    private cache. Paths are signed by the trusted private key unless noted.
    """

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="nix-cache-selection-")
        self.addCleanup(self.temp.cleanup)
        self.root = root = Path(self.temp.name)
        self.nix_bin = os.environ.get("NIX_CACHE_TRANSPORT_NIX", "nix")
        self.env = {key: value for key, value in os.environ.items()
                    if not key.startswith(("AWS_", "NIX_")) and key.lower() not in
                    {"http_proxy", "https_proxy", "all_proxy", "no_proxy"}}
        self.env.update({
            "HOME": str(root), "XDG_CACHE_HOME": str(root / "cache"),
            "NIX_CONF_DIR": str(root), "NIX_USER_CONF_FILES": "/dev/null",
            "NIX_CONFIG": "experimental-features = nix-command\n",
            "AWS_ACCESS_KEY_ID": "synthetic-local-access", "AWS_SECRET_ACCESS_KEY": "synthetic-local-secret",
            "AWS_EC2_METADATA_DISABLED": "true", "AWS_REGION": "us-east-1",
            "AWS_CONFIG_FILE": str(root / "absent-config"),
            "AWS_SHARED_CREDENTIALS_FILE": str(root / "absent-credentials"),
            "NO_PROXY": "*",
        })
        self.source = "local?root=" + str(root / "store")
        self.key = self.keypair("coda-test-1")
        self.untrusted = self.keypair("untrusted-1")
        self.paths = {}
        for name in ("shared", "private-only", "bad-signature", "missing"):
            payload = root / f"{name}.txt"
            payload.write_text(f"{name} payload\n")
            path = self.nix_ok(["--store", self.source, "store", "add-path", "--name", name, str(payload)]).stdout.strip()
            key = self.untrusted if name == "bad-signature" else self.key
            self.nix_ok(["--store", self.source, "store", "sign", "--key-file", str(key[0]), path])
            self.paths[name] = path
        self.public = f"file://{root / 'public'}"
        self.nix_ok(["copy", "--from", self.source, "--to", self.public, self.paths["shared"]])
        # Same advertisement as https://cache.nixos.org/nix-cache-info.
        (root / "public" / "nix-cache-info").write_text("StoreDir: /nix/store\nWantMassQuery: 1\nPriority: 40\n")

    def keypair(self, name):
        secret, public = self.root / f"{name}.sec", self.root / f"{name}.pub"
        secret.write_text(self.nix_ok(["key", "generate-secret", "--key-name", name]).stdout)
        public.write_text(self.nix_ok(["key", "convert-secret-to-public"], stdin=secret.read_text()).stdout)
        return secret, public.read_text().strip()

    def call(self, args, *, stdin=None, env=None):
        cmd = [self.nix_bin, *args]
        result = subprocess.run(cmd, env=env or self.env, input=stdin, capture_output=True, text=True, timeout=60)
        print("COMMAND:", shlex.join(cmd), "\nEXIT:", result.returncode, "\nSTDERR:", result.stderr, flush=True)
        return result

    def nix_ok(self, args, **kwargs):
        result = self.call(args, **kwargs)
        self.assertEqual(result.returncode, 0, result.stderr)
        return result

    def reader(self, server):
        # The production line, with only the public endpoint made local.
        settings = cache.nix_config(server.destination, self.key[1]).replace("https://cache.nixos.org", self.public)
        conf = self.root / "reader.conf"
        conf.write_text("substituters =\ntrusted-public-keys =\n" + settings)
        return dict(self.env, NIX_USER_CONF_FILES=str(conf), XDG_CACHE_HOME=str(self.root / "reader-cache"))

    def substitute(self, server, name):
        store = self.root / f"dest-{name}"
        result = self.call(["--store", f"local?root={store}", "build", "--no-link", self.paths[name]],
                          env=self.reader(server))
        nar_gets = [key for method, key, status in server.events if method == "GET" and "/nar/" in key and status == 200]
        return result, nar_gets

    def private_server(self):
        server = CacheServer()
        self.enterContext(server)
        names = ("shared", "private-only", "bad-signature")
        self.nix_ok(["copy", "--from", self.source, "--to", server.destination,
                     *(self.paths[name] for name in names)])
        # add-path makes content-addressed paths, which Nix trusts unsigned.
        # Dropping CA turns this one into an input-addressed claim whose only
        # authority is its (untrusted) signature, as for a forged narinfo.
        digest = Path(self.paths["bad-signature"]).name.split("-", 1)[0]
        key = f"/{server.bucket}/{digest}.narinfo"
        with server.lock:
            server.objects[key] = b"".join(line for line in server.objects[key].splitlines(True)
                                            if not line.startswith(b"CA:"))
        server.events.clear()
        return server

    def test_reader_prefers_public_and_falls_back_to_private(self):
        settings = cache.nix_config("s3://bucket?endpoint=https://e", self.key[1])
        self.assertIn("require-sigs = true", settings)
        server = self.private_server()
        result, gets = self.substitute(server, "shared")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(gets, [], "a path present in both caches must come from the public cache")
        result, gets = self.substitute(server, "private-only")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(len(gets), 1, "a private-only path must still come from the private cache")

    def test_reader_refuses_missing_and_untrusted_paths(self):
        server = self.private_server()
        for name in ("missing", "bad-signature"):
            with self.subTest(name=name):
                started = time.monotonic()
                result, gets = self.substitute(server, name)
                self.assertNotEqual(result.returncode, 0)
                self.assertLess(time.monotonic() - started, 30)
                store = self.root / f"dest-{name}"
                verify = self.call(["--store", f"local?root={store}", "path-info", self.paths[name]])
                self.assertNotEqual(verify.returncode, 0, "the path must not become valid")


if __name__ == "__main__":
    unittest.main(verbosity=2)
