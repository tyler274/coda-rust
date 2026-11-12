#!/usr/bin/env python3
"""Local synthetic physician stack. No cloud credentials or hosted auth bypass.

Two physician-sample sites and one physician-worker share a local Postgres,
Valkey and Pub/Sub emulator on loopback ports. Sites run PHYSICIAN_AUTH=required
with local proof sign-in for the synthetic physician@codahealth.test account.
--dependencies-only --check proves Valkey, Postgres and Pub/Sub provisioning,
NOT the application journey. --practice-dir preserves Postgres between runs;
--model-execution subscription uses the developer plan on this same worker. The coda-verify-physician recipe drives the rest.

While running, SIGUSR1 restarts Valkey on the same port (the resync journey),
and SIGTERM or Ctrl-C stops every owned process. Logs and stack.json are kept
in the evidence directory under target/verification/runs/.
"""
import argparse
import sys
import contextlib
import datetime
import http.client
import json
import os
from pathlib import Path
import shutil
import signal
import socket
import subprocess
import tempfile
import threading
import time
import urllib.error
import urllib.request
import uuid

PROJECT = "coda-local-physician"
TOPIC = f"projects/{PROJECT}/topics/physician-chat-turns"
DLQ = f"projects/{PROJECT}/topics/physician-chat-turns-dead-letter"
CHANNEL = "physician:workspace-events:v1"
PRACTICE = "roger-synthetic-practice"
PHYSICIAN = "staging-physician"
EMAIL = "physician@codahealth.test"
# Abigail Lemieux: a bank patient on the staging panel.
WORKSPACE = "pw-2a995cfc98bbfd3c4e6c1d7492ee6998"
FIXED_TEXT = "Noted."
HOSTED = ("K_SERVICE", "K_REVISION", "CLOUD_RUN_JOB", "CODA_HOSTED", "GAE_ENV")
# Deliberately do not inherit Doppler, Google ADC, model keys, database URLs,
# telemetry exporters, PORT, product modes, or auth configuration.
SAFE_ENV = ("PATH", "LD_LIBRARY_PATH", "LIBRARY_PATH", "SSL_CERT_FILE", "LANG", "TZ",
            "CARGO_HOME", "RUSTUP_HOME")
# Pending pg_stat counters of other backends can wait up to PGSTAT_IDLE_INTERVAL
# (10 s) before they are flushed to pg_stat_database.
STATS_SETTLE_SECONDS = 11


def environment(source, home):
    if any(source.get(key) for key in HOSTED):
        raise ValueError("local scale stack is forbidden in a hosted environment")
    result = {key: source[key] for key in SAFE_ENV if key in source}
    # The throwaway HOME must not hide the crate cache, or offline child cargo fails.
    if source.get("HOME"):
        result.setdefault("CARGO_HOME", str(Path(source["HOME"]) / ".cargo"))
        result.setdefault("RUSTUP_HOME", str(Path(source["HOME"]) / ".rustup"))
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
        log = (self.root / f"{name}.log").open("ab")
        self.logs.append(log)
        process = subprocess.Popen(argv, env=self.env | (extra or {}), stdout=log,
                                   stderr=subprocess.STDOUT, start_new_session=True)
        self.processes.append((name, process))
        return process

    def alive(self):
        for name, process in self.processes:
            if process.poll() is not None:
                raise RuntimeError(f"{name} exited ({process.returncode}); see {self.root}/{name}.log")

    def wait(self, probe, label, timeout=60):
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            self.alive()
            try:
                if probe():
                    return
            except (OSError, urllib.error.URLError, http.client.HTTPException):
                pass
            time.sleep(0.1)
        raise RuntimeError(f"timed out waiting for {label}; logs: {self.root}")

    def stop(self, name):
        """Stops one owned process and forgets it, so alive() does not trip."""
        for index, (owned, process) in enumerate(self.processes):
            if owned != name:
                continue
            with contextlib.suppress(ProcessLookupError):
                os.killpg(process.pid, signal.SIGKILL)
            process.wait()
            del self.processes[index]
            return

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


def valkey_argv(port):
    return ["valkey-server", "--bind", "127.0.0.1", "--port", str(port),
            "--save", "", "--appendonly", "no", "--protected-mode", "yes"]


def valkey_ping(stack, port):
    return subprocess.run(["valkey-cli", "-h", "127.0.0.1", "-p", str(port), "ping"],
                          env=stack.env, capture_output=True).stdout.strip() == b"PONG"


def subscribers(stack, port):
    output = subprocess.run(
        ["valkey-cli", "-h", "127.0.0.1", "-p", str(port), "--raw", "PUBSUB", "NUMSUB", CHANNEL],
        env=stack.env, capture_output=True, text=True).stdout.split()
    return int(output[1]) if len(output) > 1 and output[1].isdigit() else 0


def restart_valkey(stack, port):
    stack.stop("valkey")
    deadline = time.monotonic() + 5
    while time.monotonic() < deadline:
        try:
            tcp(port)
        except OSError:
            break
        time.sleep(0.02)
    stack.start("valkey", valkey_argv(port))
    stack.wait(lambda: valkey_ping(stack, port), "Valkey after restart", timeout=10)


def seed_account(stack, database):
    """The synthetic proof account the scale tests also create (ensure_account).

    Raw SQL exception: this Python harness cannot call the Rust PostgresAuthStore,
    and physician-migrate seeds accounts only under the hosted staging profile.
    Every value is a fixed synthetic constant bound as a psql variable; an
    existing row is left as it is, like ensure_account.
    """
    sql = (
        "INSERT INTO physician_accounts (id, practice_id, physician_id, email, display_name,"
        " active, admin, sessions_ended_before, created_at) VALUES (gen_random_uuid(),"
        " :'practice', :'physician', :'email', 'Staging Physician', true, true, 0,"
        " extract(epoch FROM now())::bigint) ON CONFLICT (email) DO NOTHING;\n")
    subprocess.run(["psql", database, "-v", "ON_ERROR_STOP=1", "-q",
                    "-v", f"practice={PRACTICE}", "-v", f"physician={PHYSICIAN}",
                    "-v", f"email={EMAIL}"],
                   input=sql, text=True, env=stack.env, check=True, stdout=subprocess.DEVNULL)


# --- load numbers -----------------------------------------------------------

def http_json(port, method, path, body=None, bearer=None, timeout=10):
    headers = {"Content-Type": "application/json"}
    if bearer:
        headers["Authorization"] = f"Bearer {bearer}"
    request = urllib.request.Request(
        f"http://127.0.0.1:{port}{path}", method=method, headers=headers,
        data=None if body is None else json.dumps(body).encode())
    with urllib.request.urlopen(request, timeout=timeout) as response:
        raw = response.read()
        return response.status, (json.loads(raw) if raw else None)


def sign_in(port):
    _, issued = http_json(port, "POST", "/auth/sign-in", {"email": EMAIL})
    _, verified = http_json(port, "POST", "/auth/verify", {"email": EMAIL, "token": issued["token"]})
    return verified["session"]


def xact_commit(stack, database):
    output = subprocess.run(
        ["psql", database, "-At", "-c",
         "SELECT xact_commit FROM pg_stat_database WHERE datname = current_database()"],
        env=stack.env, capture_output=True, text=True, check=True).stdout
    return int(output.strip())


def settled_xact(stack, database):
    time.sleep(STATS_SETTLE_SECONDS)
    return xact_commit(stack, database)


class Stream:
    """One held /patient-workspaces/events stream, read on a thread."""

    def __init__(self, port, bearer):
        self.connection = http.client.HTTPConnection("127.0.0.1", port, timeout=120)
        self.connection.request("GET", "/patient-workspaces/events?after=0",
                                headers={"Authorization": f"Bearer {bearer}",
                                         "Accept": "text/event-stream"})
        self.response = self.connection.getresponse()
        if self.response.status != 200:
            raise RuntimeError(f"event stream on {port} answered {self.response.status}")
        self.frames = []
        self.closed_at = None
        self.thread = threading.Thread(target=self._read, daemon=True)
        self.thread.start()

    def _read(self):
        try:
            while True:
                line = self.response.readline()
                if not line:
                    break
                self.frames.append(line.decode(errors="replace").rstrip("\n"))
        except (OSError, http.client.HTTPException):
            pass
        self.closed_at = time.monotonic()

    def close(self):
        with contextlib.suppress(OSError):
            self.connection.close()


def wait_reply(port, bearer, chat, timeout=60):
    path = f"/patient-workspaces/{WORKSPACE}/chats/{chat}/snapshot"
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        _, body = http_json(port, "GET", path, bearer=bearer)
        texts = [item.get("text") for item in (body or {}).get("snapshot", {}).get("conversation", [])]
        if FIXED_TEXT in texts:
            return True
        time.sleep(0.2)
    return False


def load_numbers(stack, database, ports, evidence, idle_seconds, worker_failures):
    """Postgres transactions while idle and for one turn, and stream-close time."""
    site_a, site_b = ports
    out = evidence / "load-numbers.json"
    result = {
        "measured_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "database": "postgres (disposable, this stack only)",
        "counter": "pg_stat_database.xact_commit",
        "stats_settle_seconds": STATS_SETTLE_SECONDS,
        "probe_note": "each read is one psql transaction; deltas subtract the earlier probe",
        "idle_seconds": idle_seconds,
        # Each idle delta spans the idle sleep plus one settle wait.
        "idle_window_seconds": idle_seconds + STATS_SETTLE_SECONDS,
        "sites": {"a": f"http://127.0.0.1:{site_a}", "b": f"http://127.0.0.1:{site_b}"},
    }
    session = sign_in(site_a)

    before = settled_xact(stack, database)
    time.sleep(idle_seconds)
    after = settled_xact(stack, database)
    result["idle_without_streams"] = {"before": before, "after": after,
                                      "transactions": after - before - 1}

    streams = [Stream(site_a, session), Stream(site_b, session)]
    before = settled_xact(stack, database)
    time.sleep(idle_seconds)
    after = settled_xact(stack, database)
    result["idle_with_two_streams"] = {"before": before, "after": after,
                                       "transactions": after - before - 1,
                                       "streams": ["site-a", "site-b"]}

    if worker_failures:
        result["one_turn"] = {"skipped": f"--worker-failures {worker_failures} makes turns fail"}
    else:
        chat = f"chat-load-{uuid.uuid4().hex}"
        before = settled_xact(stack, database)
        started = time.monotonic()
        _, accepted = http_json(site_a, "POST", "/commands", {
            "type": "message.submit", "idempotency_key": f"load-{uuid.uuid4().hex}",
            "payload": {"patient_workspace_id": WORKSPACE, "chat_id": chat,
                        "text": "Load-number turn", "physician_time_zone": "America/New_York"}},
            bearer=session)
        replied = wait_reply(site_b, session, chat)
        elapsed = time.monotonic() - started
        after = settled_xact(stack, database)
        result["one_turn"] = {"chat_id": chat, "outcome": (accepted or {}).get("outcome"),
                              "reply_seen_on_site_b": replied,
                              "submit_to_reply_seconds": round(elapsed, 3),
                              "before": before, "after": after,
                              "transactions_including_snapshot_polls": after - before - 1}

    # A second session's stream on site B closes when that session signs out on site A.
    revoked = sign_in(site_a)
    held = Stream(site_b, revoked)
    time.sleep(1)
    started = time.monotonic()
    status, _ = http_json(site_a, "POST", "/auth/sign-out", bearer=revoked)
    deadline = started + 30
    while held.closed_at is None and time.monotonic() < deadline:
        time.sleep(0.01)
    result["stream_close_after_sign_out"] = {
        "sign_out_site": "a", "stream_site": "b", "sign_out_status": status,
        "closed": held.closed_at is not None,
        "seconds": None if held.closed_at is None else round(held.closed_at - started, 3),
        "other_session_streams_still_open": [s.closed_at is None for s in streams],
    }
    for stream in [held, *streams]:
        stream.close()
    out.write_text(json.dumps(result, indent=2) + "\n")
    print(f"Load numbers: {out}", flush=True)
    return result


# --- stack ------------------------------------------------------------------

def run(args, root, evidence):
    stack = Stack(root)
    if args.practice_dir:
        # Practice runs keep the caller's explicit observability configuration.
        # Disposable qualification keeps its isolated environment.
        stack.env.update({key: value for key, value in os.environ.items()
                          if key.startswith(("OTEL_", "DD_", "CODA_TELEMETRY_"))})
    restart = threading.Event()
    signal.signal(signal.SIGUSR1, lambda _signal, _frame: restart.set())
    # Ports remain loopback-only; an allocation race fails startup, never falls
    # back to a public interface or an existing ambient service.
    pg, redis, emulator, site_a, site_b, worker, provider = [free_port() for _ in range(7)]
    if len({pg, redis, emulator, site_a, site_b, worker, provider}) != 7:
        raise RuntimeError("port allocation collided; retry the command")
    host = f"127.0.0.1:{emulator}"
    database = f"postgresql://postgres@127.0.0.1:{pg}/postgres"
    shared = {
        "PHYSICIAN_LOCAL_SCALE": "1", "PHYSICIAN_AUTH": "required",
        "PHYSICIAN_DATABASE_URL": database,
        "PHYSICIAN_REDIS_URL": f"redis://127.0.0.1:{redis}",
        "PUBSUB_EMULATOR_HOST": host, "GOOGLE_CLOUD_PROJECT": PROJECT,
        "PHYSICIAN_CHAT_TOPIC": TOPIC,
    }
    state = {
        "pid": os.getpid(), "root": str(root), "evidence": str(evidence) if evidence else None,
        "postgres": {"port": pg, "url": database}, "valkey": {"port": redis, "channel": CHANNEL},
        "pubsub": {"host": host, "project": PROJECT, "topic": TOPIC, "dead_letter": DLQ},
        "worker": {"url": f"http://127.0.0.1:{worker}", "fixed_text": FIXED_TEXT,
                   "fixed_failures": args.worker_failures,
                   "failures_file": str(root / "provider-failures")},
        "sites": {"a": f"http://127.0.0.1:{site_a}", "b": f"http://127.0.0.1:{site_b}"},
        "account": {"email": EMAIL, "practice": PRACTICE, "physician": PHYSICIAN},
        "workspace": WORKSPACE,
        # Flat handles read by .agents/skills/coda-verify-physician/scripts/physician-drive.py.
        "ports": {"postgres": pg, "valkey": redis, "pubsub": emulator, "worker": worker,
                  "site-a": site_a, "site-b": site_b},
        "sign_in_email": EMAIL,
        "pubsub_emulator_host": host,
        "started_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "dependencies_only": args.dependencies_only,
        "model_execution": args.model_execution,
        "practice_dir": str(args.practice_dir.resolve()) if args.practice_dir else None,
    }

    def write_state(**extra):
        if evidence:
            state.update(extra)
            (evidence / "stack.json").write_text(json.dumps(state, indent=2) + "\n")

    try:
        data = (args.practice_dir.resolve() if args.practice_dir else root) / "pg"
        data.parent.mkdir(parents=True, exist_ok=True)
        if not (data / "PG_VERSION").exists():
            subprocess.run(["initdb", "-D", str(data), "-U", "postgres", "--auth=trust"],
                           env=stack.env, check=True, stdout=subprocess.DEVNULL)
        stack.start("postgres", ["postgres", "-D", str(data), "-h", "127.0.0.1",
                                 "-p", str(pg), "-k", str(root)])
        stack.start("valkey", valkey_argv(redis))
        # gcloud writes env.yaml into --data-dir and does not create it.
        (root / "pubsub").mkdir()
        stack.start("pubsub", ["gcloud", "beta", "emulators", "pubsub", "start",
                               f"--host-port={host}", f"--project={PROJECT}",
                               f"--data-dir={root / 'pubsub'}", "--quiet"])
        stack.wait(lambda: subprocess.run(["pg_isready", "-d", database], env=stack.env,
                                          stdout=subprocess.DEVNULL).returncode == 0, "Postgres")
        stack.wait(lambda: valkey_ping(stack, redis), "Valkey")
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
            seed_account(stack, database)
            # The practice's patients come from the chart load.
            subprocess.run([str(binary / "physician-migrate"), "load-charts"], check=True, env=stack.env | shared | {
                "CODA_HOSTED_PROFILE": "staging-preview",
                "PHYSICIAN_SYNTHETIC_CHARTS": str(args.charts.resolve())})
            worker_env = shared | {
                "PHYSICIAN_WORKER_LISTEN_ADDR": f"127.0.0.1:{worker}",
                "PHYSICIAN_LISTEN_ADDR": f"127.0.0.1:{worker}"}
            if args.model_execution == "stub":
                failures = root / "provider-failures"
                failures.write_text(str(args.worker_failures))
                stack.start("responses", [sys.executable, str(Path(__file__).with_name("physician-responses-stub.py")),
                                          "--port", str(provider), "--failures", str(failures)])
                stack.wait(lambda: healthy(provider), "local Responses stub readiness")
                worker_env["PHYSICIAN_RESPONSES_STUB_PORT"] = str(provider)
            else:
                worker_env["CODA_LOCAL_MODEL_EXECUTION"] = args.model_execution
                if args.model_execution == "subscription":
                    # Mount or reuse the grant directory; never copy refresh tokens.
                    config = Path(os.environ.get("XDG_CONFIG_HOME", Path.home() / ".config"))
                    worker_env["XDG_CONFIG_HOME"] = str(config.resolve())
                else:
                    for key in ("CODA_ALLOW_MODEL_CALLS", "OPENAI_API_KEY"):
                        if key in os.environ:
                            worker_env[key] = os.environ[key]
            model_settings = {}
            for role, model, effort in (("CHAT", "gpt-6.1-sol", "high"),
                                        ("BACKGROUND", "gpt-6-luna", "max")):
                model_settings.update({f"CODA_{role}_MODEL": model,
                                   f"CODA_{role}_EFFORT": effort,
                                   f"CODA_{role}_CONTEXT_WINDOW_TOKENS": "272000",
                                   f"CODA_{role}_INPUT_LIMIT_TOKENS": "922000"})
            stack.start("worker", [str(binary / "physician-worker")], worker_env | model_settings)
            stack.wait(lambda: healthy(worker), f"worker readiness on {worker}")
            site_env = shared | model_settings | {
                "PHYSICIAN_SYNTHETIC_CHARTS": str(args.charts.resolve()),
            }
            if args.practice_dir and os.environ.get("PHYSICIAN_SYNTHETIC_REPLY"):
                site_env["PHYSICIAN_SYNTHETIC_REPLY"] = os.environ["PHYSICIAN_SYNTHETIC_REPLY"]
            if args.site_dir:
                site_env["PHYSICIAN_SITE_DIR"] = str(args.site_dir.resolve())
            for name, port in (("site-a", site_a), ("site-b", site_b)):
                stack.start(name, [str(binary / "physician-sample")], site_env | {
                    "PHYSICIAN_LISTEN_ADDR": f"127.0.0.1:{port}"})
                stack.wait(lambda port=port: healthy(port), f"{name} readiness on {port}")
            stack.wait(lambda: subscribers(stack, redis) >= 3, "both sites and the worker on Valkey")
            print(f"Site A: http://127.0.0.1:{site_a} (port {site_a})", flush=True)
            print(f"Site B: http://127.0.0.1:{site_b} (port {site_b})", flush=True)
            print(f"Worker: http://127.0.0.1:{worker} (port {worker}); {args.model_execution}", flush=True)
            print(f"Sites: http://127.0.0.1:{site_a} and http://127.0.0.1:{site_b}", flush=True)
        print(f"Dependencies ready; emulator {host}, Redis {redis}, Postgres {pg}. Logs: {root}", flush=True)
        write_state(ready_at=datetime.datetime.now(datetime.timezone.utc).isoformat())
        if evidence:
            print(f"Evidence: {evidence} (stack.json; logs copied on exit). "
                  f"Restart Valkey: kill -USR1 {os.getpid()}. Stop: kill -TERM {os.getpid()}", flush=True)
        if args.load_numbers and not args.dependencies_only:
            load_numbers(stack, database, (site_a, site_b), evidence, args.idle_seconds,
                         args.worker_failures)
        if args.command:
            return subprocess.run(args.command, env=stack.env | shared | {
                "PHYSICIAN_TEST_DATABASE_URL": database,
                "PHYSICIAN_RUNTIME_TEST_DATABASE_URL": database,
                "PHYSICIAN_SCALE_SITE_A": f"http://127.0.0.1:{site_a}",
                "PHYSICIAN_SCALE_SITE_B": f"http://127.0.0.1:{site_b}",
            }).returncode
        if not args.check:
            restarts = 0
            while True:
                if restart.is_set():
                    restart.clear()
                    restart_valkey(stack, redis)
                    restarts += 1
                    write_state(valkey_restarts=restarts,
                                last_valkey_restart=datetime.datetime.now(datetime.timezone.utc).isoformat())
                    print(f"Valkey restarted on {redis} ({restarts})", flush=True)
                stack.alive()
                time.sleep(0.2)
        return 0
    finally:
        stack.close()
        write_state(stopped_at=datetime.datetime.now(datetime.timezone.utc).isoformat())
        if evidence:
            logs = evidence / "logs"
            logs.mkdir(exist_ok=True)
            for log in root.glob("*.log"):
                shutil.copy(log, logs / log.name)


def evidence_dir(requested, dependencies_only):
    if requested:
        path = requested
    elif dependencies_only:
        return None
    else:
        stamp = datetime.datetime.now().strftime("%Y%m%d-%H%M%S")
        path = Path("target/verification/runs") / f"physician-scale-stack-{stamp}-{os.getpid()}"
    path = path.resolve()
    path.mkdir(parents=True, exist_ok=False)
    return path


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--dependencies-only", action="store_true")
    parser.add_argument("--practice-dir", type=Path, help="keep the local Postgres data in this directory")
    parser.add_argument("--model-execution", choices=("stub", "subscription", "api"), default="stub",
                        help="worker transport; subscription and api use real models")
    parser.add_argument("--check", action="store_true", help="start, check readiness, stop (not a journey proof)")
    parser.add_argument("--bin-dir", type=Path, default=Path("target/debug"),
                        help="directory holding physician-sample, physician-worker and physician-migrate")
    parser.add_argument("--site-dir", type=Path, default=None,
                        help="wasm-bindgen output for the page (PHYSICIAN_SITE_DIR); "
                             "the sites fall back to target/physician-web")
    parser.add_argument("--charts", type=Path, default=Path("target/patient-bank/bank"),
                        help="built synthetic charts (cargo run -p patient-bank -- build)")
    parser.add_argument("--worker-failures", type=int, default=0, metavar="N",
                        help="the local provider's first N calls "
                             "fail; 5 dead-letters the first turn")
    parser.add_argument("--load-numbers", action="store_true",
                        help="after readiness, record Postgres transactions (idle, two streams, one "
                             "turn) and stream-close time to <evidence>/load-numbers.json")
    parser.add_argument("--idle-seconds", type=int, default=20,
                        help="idle window for --load-numbers (default 20)")
    parser.add_argument("--evidence", type=Path, default=None,
                        help="new evidence directory (default "
                             "target/verification/runs/physician-scale-stack-<time>-<pid>)")
    parser.add_argument("command", nargs=argparse.REMAINDER, help="optional child command after --")
    args = parser.parse_args()
    if args.command[:1] == ["--"]:
        args.command = args.command[1:]
    if args.model_execution != "stub" and (args.worker_failures or args.load_numbers):
        parser.error("--worker-failures and --load-numbers require the stub transport")
    if args.worker_failures < 0:
        parser.error("--worker-failures must be zero or more")
    if args.dependencies_only and (args.worker_failures
                                   or args.load_numbers):
        parser.error("--worker-failures and --load-numbers need the "
                     "application processes")
    environment(os.environ, Path("/unused"))  # Refuse hosted before allocating anything.

    def stop(_signal, _frame):
        raise KeyboardInterrupt
    signal.signal(signal.SIGTERM, stop)
    evidence = evidence_dir(args.evidence, args.dependencies_only)
    with tempfile.TemporaryDirectory(prefix="physician-scale-") as directory:
        root = Path(directory)
        try:
            return run(args, root, evidence)
        except KeyboardInterrupt:
            return 130
        except Exception:
            # Logs contain only synthetic stack output: credentials were never inherited.
            for log in root.glob("*.log"):
                print(f"--- {log.name} ---\n{log.read_text(errors='replace')[-12000:]}")
            raise


if __name__ == "__main__":
    raise SystemExit(main())
