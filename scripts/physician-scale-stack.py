#!/usr/bin/env python3
"""Disposable synthetic physician stack. No cloud credentials or hosted auth bypass.

Invoked through workspace-tools physician-scale-stack. --dependencies-only --check
proves Valkey, Postgres and Pub/Sub provisioning, NOT the application journey.
"""
import argparse
import contextlib
import json
import os
from pathlib import Path
import signal
import socket
import subprocess
import tempfile
import time
import urllib.error
import urllib.request

PROJECT = "coda-local-physician"
TOPIC = f"projects/{PROJECT}/topics/physician-chat-turns"
DLQ = f"projects/{PROJECT}/topics/physician-chat-turns-dead-letter"
HOSTED = ("K_SERVICE", "K_REVISION", "CLOUD_RUN_JOB", "CODA_HOSTED", "GAE_ENV")
# Deliberately do not inherit Doppler, Google ADC, model keys, database URLs,
# telemetry exporters, PORT, product modes, or auth configuration.
SAFE_ENV = ("PATH", "LD_LIBRARY_PATH", "LIBRARY_PATH", "SSL_CERT_FILE", "LANG", "TZ")


def environment(source, home):
    if any(source.get(key) for key in HOSTED):
        raise ValueError("local scale stack is forbidden in a hosted environment")
    result = {key: source[key] for key in SAFE_ENV if key in source}
    result.update(HOME=str(home), TMPDIR=str(home), CLOUDSDK_CONFIG=str(home / "gcloud"))
    return result


def free_port():
    with socket.socket() as listener:
        listener.bind(("127.0.0.1", 0))
        return listener.getsockname()[1]


def put(host, resource, body):
    request = urllib.request.Request(
        f"http://{host}/v1/{resource}", data=json.dumps(body).encode(),
        headers={"Content-Type": "application/json"}, method="PUT")
    with urllib.request.urlopen(request, timeout=3) as response:
        return json.load(response)


class Stack:
    def __init__(self, root):
        self.root = root
        self.env = environment(os.environ, root)
        self.processes = []
        self.logs = []

    def start(self, name, argv, extra=None):
        log = (self.root / f"{name}.log").open("wb")
        self.logs.append(log)
        process = subprocess.Popen(argv, env=self.env | (extra or {}), stdout=log,
                                   stderr=subprocess.STDOUT, start_new_session=True)
        self.processes.append((name, process))
        return process

    def alive(self):
        for name, process in self.processes:
            if process.poll() is not None:
                raise RuntimeError(f"{name} exited ({process.returncode}); see {self.root}/{name}.log")

    def wait(self, probe, label):
        deadline = time.monotonic() + 60
        while time.monotonic() < deadline:
            self.alive()
            try:
                if probe():
                    return
            except (OSError, urllib.error.URLError):
                pass
            time.sleep(0.1)
        raise RuntimeError(f"timed out waiting for {label}; logs: {self.root}")

    def close(self):
        for _, process in reversed(self.processes):
            if process.poll() is None:
                with contextlib.suppress(ProcessLookupError):
                    os.killpg(process.pid, signal.SIGTERM)
        for _, process in reversed(self.processes):
            try:
                process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                with contextlib.suppress(ProcessLookupError):
                    os.killpg(process.pid, signal.SIGKILL)
                process.wait()
        for log in self.logs:
            log.close()


def tcp(port):
    with socket.create_connection(("127.0.0.1", port), timeout=0.2):
        return True


def healthy(port):
    with urllib.request.urlopen(f"http://127.0.0.1:{port}/readyz", timeout=2) as response:
        return response.status == 200


def run(args, root):
    stack = Stack(root)
    # Ports remain loopback-only; an allocation race fails startup, never falls
    # back to a public interface or an existing ambient service.
    pg, redis, emulator, site_a, site_b, worker = [free_port() for _ in range(6)]
    if len({pg, redis, emulator, site_a, site_b, worker}) != 6:
        raise RuntimeError("port allocation collided; retry the command")
    host = f"127.0.0.1:{emulator}"
    database = f"postgresql://postgres@127.0.0.1:{pg}/postgres"
    shared = {
        "PHYSICIAN_LOCAL_SCALE": "1", "PHYSICIAN_AUTH": "local-scale",
        "PHYSICIAN_DATABASE_URL": database,
        "PHYSICIAN_REDIS_URL": f"redis://127.0.0.1:{redis}",
        "PUBSUB_EMULATOR_HOST": host, "GOOGLE_CLOUD_PROJECT": PROJECT,
        "PHYSICIAN_CHAT_TOPIC": TOPIC, "CODA_LOCAL_MODEL_EXECUTION": "replay",
    }
    try:
        subprocess.run(["initdb", "-D", str(root / "pg"), "-U", "postgres", "--auth=trust"],
                       env=stack.env, check=True, stdout=subprocess.DEVNULL)
        stack.start("postgres", ["postgres", "-D", str(root / "pg"), "-h", "127.0.0.1",
                                 "-p", str(pg), "-k", str(root), "-c", "fsync=off"])
        stack.start("valkey", ["valkey-server", "--bind", "127.0.0.1", "--port", str(redis),
                               "--save", "", "--appendonly", "no", "--protected-mode", "yes"])
        stack.start("pubsub", ["gcloud", "beta", "emulators", "pubsub", "start",
                               f"--host-port={host}", f"--project={PROJECT}",
                               f"--data-dir={root / 'pubsub'}", "--quiet"])
        stack.wait(lambda: subprocess.run(["pg_isready", "-d", database], env=stack.env,
                                          stdout=subprocess.DEVNULL).returncode == 0, "Postgres")
        stack.wait(lambda: subprocess.run(["valkey-cli", "-h", "127.0.0.1", "-p", str(redis), "ping"],
                                          env=stack.env, capture_output=True).stdout.strip() == b"PONG", "Valkey")
        stack.wait(lambda: tcp(emulator), "Pub/Sub emulator")
        # TCP alone can open before the emulator HTTP API is ready.
        stack.wait(lambda: put(host, TOPIC, {}), "Pub/Sub topic")
        put(host, DLQ, {})
        put(host, f"projects/{PROJECT}/subscriptions/physician-chat-turns-dead-letter", {"topic": DLQ})
        put(host, f"projects/{PROJECT}/subscriptions/physician-chat-turns", {
            "topic": TOPIC, "ackDeadlineSeconds": 600,
            "pushConfig": {"pushEndpoint": f"http://127.0.0.1:{worker}/pubsub/chat-turn"},
            "retryPolicy": {"minimumBackoff": "1s", "maximumBackoff": "10s"},
            "deadLetterPolicy": {"deadLetterTopic": DLQ, "maxDeliveryAttempts": 5},
        })
        if not args.dependencies_only:
            binary = args.bin_dir.resolve()
            subprocess.run([str(binary / "physician-migrate")], env=stack.env | shared, check=True)
            stack.start("worker", [str(binary / "physician-worker")], shared | {
                "PHYSICIAN_WORKER_LISTEN_ADDR": f"127.0.0.1:{worker}",
                "PHYSICIAN_LISTEN_ADDR": f"127.0.0.1:{worker}",
                # The local stack is not API mode. The worker refuses to start
                # without a provider; this fixed reply never leaves the process.
                "PHYSICIAN_WORKER_FIXED_TEXT": "Noted."})
            for name, port in (("site-a", site_a), ("site-b", site_b)):
                stack.start(name, [str(binary / "physician-sample")], shared | {
                    "PHYSICIAN_LISTEN_ADDR": f"127.0.0.1:{port}"})
            for port in (worker, site_a, site_b):
                stack.wait(lambda port=port: healthy(port), f"application readiness on {port}")
            print(f"Sites: http://127.0.0.1:{site_a} and http://127.0.0.1:{site_b}", flush=True)
        print(f"Dependencies ready; emulator {host}, Redis {redis}, Postgres {pg}. Logs: {root}", flush=True)
        if args.command:
            return subprocess.run(args.command, env=stack.env | shared | {
                "PHYSICIAN_TEST_DATABASE_URL": database,
                "PHYSICIAN_RUNTIME_TEST_DATABASE_URL": database,
                "PHYSICIAN_SCALE_SITE_A": f"http://127.0.0.1:{site_a}",
                "PHYSICIAN_SCALE_SITE_B": f"http://127.0.0.1:{site_b}",
            }).returncode
        if not args.check:
            while True:
                stack.alive()
                time.sleep(0.5)
        return 0
    finally:
        stack.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dependencies-only", action="store_true")
    parser.add_argument("--check", action="store_true", help="start, check readiness, stop (not a journey proof)")
    parser.add_argument("--bin-dir", type=Path, default=Path("target/debug"))
    parser.add_argument("command", nargs=argparse.REMAINDER, help="optional child command after --")
    args = parser.parse_args()
    if args.command[:1] == ["--"]:
        args.command = args.command[1:]
    environment(os.environ, Path("/unused"))  # Refuse hosted before allocating anything.
    def stop(_signal, _frame):
        raise KeyboardInterrupt
    signal.signal(signal.SIGTERM, stop)
    with tempfile.TemporaryDirectory(prefix="physician-scale-") as directory:
        root = Path(directory)
        try:
            return run(args, root)
        except KeyboardInterrupt:
            return 130
        except Exception:
            # Logs contain only synthetic stack output: credentials were never inherited.
            for log in root.glob("*.log"):
                print(f"--- {log.name} ---\n{log.read_text(errors='replace')[-12000:]}")
            raise


if __name__ == "__main__":
    raise SystemExit(main())
