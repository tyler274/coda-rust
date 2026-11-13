#!/usr/bin/env python3
"""Exercise the actual collector with synthetic OTLP and loopback-only sinks.

Run: nix develop .#agent --command python scripts/test_collector_privacy.py [datadog|grafana ...]
With no argument every operations backend overlay runs: collector.datadog.yaml
(traces and logs over OTLP gRPC to the Datadog Agent) and collector.grafana.yaml
(traces over OTLP gRPC to Tempo, logs over OTLP/HTTP to Loki). Each backend's
exporters point at one loopback "ops" sink that accepts both protocols, so the
same assertions cover both. No Python packages or vendor credentials are
needed. Artifacts are retained in target/verification/collector-<timestamp>
and contain synthetic content only.
"""

import base64
import hashlib
import http.server
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import threading
import time
import urllib.error
import urllib.request
import uuid

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "target/verification" / f"collector-{time.time_ns()}"
OUT.mkdir(parents=True)
MARKERS = [f"PROMPT_{uuid.uuid4().hex}", f"RESPONSE_{uuid.uuid4().hex}"]
TRACE = "0123456789abcdef0123456789abcdef"
SPAN = "0123456789abcdef"
RUN = "synthetic-canonical-attempt-1"
ENV = {**os.environ, "LANGFUSE_PUBLIC_KEY": "synthetic", "LANGFUSE_SECRET_KEY": "synthetic", "OTEL_GATEWAY_AUDIENCE": "collector-test", "OTEL_GATEWAY_ALLOWED_CALLERS": "allowed@example.test", "OTEL_GATEWAY_ENDPOINT": "http://127.0.0.1:1"}
PROCESSES = []
SERVERS = []
BACKENDS = ("datadog", "grafana", "both")
BACKEND = "datadog"


def ops_exporters(grpc, http, grafana=None):
    """The selected backend overlay's exporters, retargeted at loopback sinks."""
    bounded = {"timeout": "500ms", "sending_queue": {"queue_size": 2}, "retry_on_failure": {"initial_interval": "100ms", "max_interval": "200ms", "max_elapsed_time": "1s"}}
    exporters = {}
    if BACKEND in ("datadog", "both"):
        exporters["otlp/datadog"] = {"endpoint": f"127.0.0.1:{grpc}", **bounded}
    if BACKEND in ("grafana", "both"):
        grpc, http = grafana or (grpc, http)
        exporters.update({"otlp/tempo": {"endpoint": f"127.0.0.1:{grpc}", **bounded}, "otlphttp/loki": {"endpoint": f"http://127.0.0.1:{http}", **bounded}})
    return exporters


def port():
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


def write(name, value):
    path = OUT / name
    path.write_text(json.dumps(value, indent=2))
    return path


def attr(key, value):
    return {"key": key, "value": {"intValue": str(value)} if isinstance(value, int) else {"stringValue": value}}


def attrs(**values):
    return [attr(k, v) for k, v in values.items()]


def fixture():
    content = " / ".join(MARKERS)
    resource = {"attributes": attrs(**{"service.name": "physician-worker", "service.version": "synthetic-sha", "deployment.environment.name": "staging", "gen_ai.prompt": content, "unexpected.resource": content, "error.message": content})}
    scope = {"name": "coda-telemetry", "attributes": attrs(**{"unexpected.scope": content})}
    common = {"traceId": TRACE, "spanId": SPAN, "startTimeUnixNano": str(time.time_ns()), "endTimeUnixNano": str(time.time_ns() + 1000), "kind": 3}
    generation = {**common, "name": "gen_ai.generation", "attributes": attrs(**{"coda.run_id": RUN, "coda.work_id": "work-1", "coda.predecessor_run_id": "attempt-0", "coda.job": "physician_chat", "coda.outcome": "retry", "coda.republished": 2, "coda.rehearsed_hold_ms": 20000, "error.type": "model_call", "gen_ai.system": "openai", "gen_ai.request.model": "test-model", "gen_ai.usage.input_tokens": 12, "gen_ai.usage.output_tokens": 34, "langfuse.observation.type": "generation", "langfuse.observation.input": MARKERS[0], "langfuse.observation.output": MARKERS[1], "gen_ai.input.messages": content, "unexpected.span": content, "exception.message": content, "exception.stacktrace": content, "error.message": content}), "status": {"code": 2, "message": content}, "events": [{"name": content, "timeUnixNano": str(time.time_ns()), "attributes": attrs(**{"error.type": "model_call", "exception.message": content, "gen_ai.completion": content, "unexpected.event": content})}], "links": [{"traceId": "fedcba9876543210fedcba9876543210", "spanId": "fedcba9876543210"}]}
    http = {**common, "spanId": "0123456789abcdee", "name": content, "attributes": attrs(**{"http.request.method": "POST", "http.route": "/chat", "http.response.status_code": 500})}
    traces = {"resourceSpans": [{"resource": resource, "scopeSpans": [{"scope": scope, "spans": [generation, http]}]}]}
    logs = {"resourceLogs": [{"resource": resource, "scopeLogs": [{"scope": scope, "logRecords": [{"timeUnixNano": str(time.time_ns()), "traceId": TRACE, "spanId": SPAN, "severityNumber": 17, "severityText": content, "body": {"kvlistValue": {"values": attrs(**{"message": content, "nested_response": MARKERS[1]})}}, "attributes": attrs(**{"coda.run_id": RUN, "coda.job": "physician_chat", "error.type": "model_call", "unexpected.log": content, "error.message": content, "gen_ai.prompt": content})}]}]}]}
    return traces, logs


def launch(name, *configs):
    handle = (OUT / f"{name}.log").open("w")
    telemetry = write(f"{name}-telemetry.json", {"service": {"telemetry": {"metrics": {"level": "none"}}}})
    process = subprocess.Popen(["otelcol-contrib", *[arg for config in (*configs, telemetry) for arg in ("--config", str(config))]], env=ENV, stdout=handle, stderr=subprocess.STDOUT)
    PROCESSES.append((process, handle))
    return process


def wait_for(predicate, message, seconds=15):
    deadline = time.monotonic() + seconds
    while time.monotonic() < deadline:
        if predicate():
            return
        time.sleep(0.05)
    raise AssertionError(message)


def listening(number):
    try:
        with socket.create_connection(("127.0.0.1", number), timeout=0.1):
            return True
    except OSError:
        return False


def post(number, signal, payload, token=None):
    request = urllib.request.Request(f"http://127.0.0.1:{number}/v1/{signal}", json.dumps(payload).encode(), {"Content-Type": "application/json", **({"Authorization": f"Bearer {token}"} if token else {})})
    try:
        with urllib.request.urlopen(request, timeout=3) as response:
            assert not json.loads(response.read() or b"{}").get("partialSuccess"), "partial rejection"
            return response.status
    except urllib.error.HTTPError as error:
        return error.code


class Mock(http.server.BaseHTTPRequestHandler):
    def log_message(self, *_):
        pass

    def do_GET(self):
        if self.path == "/.well-known/openid-configuration":
            value = {"issuer": self.server.issuer, "jwks_uri": self.server.issuer + "/keys", "authorization_endpoint": self.server.issuer + "/authorize", "token_endpoint": self.server.issuer + "/token", "id_token_signing_alg_values_supported": ["RS256"]}
        elif self.path == "/keys":
            value = {"keys": [self.server.jwk]}
        else:
            self.send_error(404)
            return
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(json.dumps(value).encode())

    def do_POST(self):
        data = self.rfile.read(int(self.headers["Content-Length"]))
        if self.server.fail:
            self.send_error(503)
            return
        assert self.headers.get("Authorization") == "Basic " + base64.b64encode(b"synthetic:synthetic").decode()
        payload = json.loads(data)
        self.server.received.append(payload)
        write(f"{self.server.label}-{len(self.server.received)}.json", payload)
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(b"{}")


def mock(label, fail=False):
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Mock)
    server.label, server.fail, server.received = label, fail, []
    server.issuer = f"http://127.0.0.1:{server.server_port}"
    threading.Thread(target=server.serve_forever, daemon=True).start()
    SERVERS.append(server)
    return server


def sink(label):
    """Ops-backend stand-in: OTLP gRPC (Datadog Agent, Tempo) and OTLP/HTTP (Loki)."""
    grpc, http = port(), port()
    path = OUT / f"{label}.jsonl"
    config = write(f"{label}-config.json", {"receivers": {"otlp": {"protocols": {"grpc": {"endpoint": f"127.0.0.1:{grpc}"}, "http": {"endpoint": f"127.0.0.1:{http}"}}}}, "exporters": {"file": {"path": str(path), "flush_interval": "100ms"}}, "service": {"pipelines": {signal: {"receivers": ["otlp"], "exporters": ["file"]} for signal in ("traces", "logs")}}})
    launch(label, config)
    wait_for(lambda: listening(grpc) and listening(http), f"{BACKEND}-shaped OTLP sink did not start")
    return (grpc, http), path


def gateway(label, ops, lf, issuer=None, langfuse_enabled=True, grafana=None):
    """Production file order, backend overlay last; a test-only JSON overlay then moves ports."""
    http, grpc, health = port(), port(), port()
    overlay = {"extensions": {"health_check": {"endpoint": f"127.0.0.1:{health}"}}, "receivers": {"otlp": {"protocols": {"http": {"endpoint": f"127.0.0.1:{http}"}, "grpc": {"endpoint": f"127.0.0.1:{grpc}"}}}}, "processors": {"batch": {"timeout": "100ms", "send_batch_size": 1}}, "exporters": {**ops_exporters(*ops, grafana=grafana), "otlphttp/langfuse": {"endpoint": lf.issuer, "encoding": "json", "compression": "none", "timeout": "500ms", "sending_queue": {"queue_size": 2}, "retry_on_failure": {"initial_interval": "100ms", "max_interval": "200ms", "max_elapsed_time": "1s"}}}}
    configs = [ROOT / "dev/observability/collector.yaml"]
    if issuer:
        configs.append(ROOT / "dev/observability/collector.gke.yaml")
        overlay["extensions"]["oidc"] = {"issuer_url": issuer.issuer}
    if not langfuse_enabled:
        assert issuer is not None
        configs.append(ROOT / "dev/observability/collector.gke-no-langfuse.yaml")
    configs.append(ROOT / f"dev/observability/collector.{BACKEND}.yaml")
    configs.append(write(f"{label}-overlay.json", overlay))
    process = launch(label, *configs)
    wait_for(lambda: listening(http) or process.poll() is not None, "gateway did not start")
    assert process.poll() is None, (OUT / f"{label}.log").read_text()
    return http, health


def read_sink(path):
    return path.read_text() if path.exists() else ""


def test_privacy():
    ops, path = sink(f"{BACKEND}-privacy-ops")
    lf = mock(f"{BACKEND}-privacy-langfuse")
    http, _ = gateway(f"{BACKEND}-privacy", ops, lf)
    traces, logs = fixture()
    write(f"{BACKEND}-input-traces.json", traces)
    write(f"{BACKEND}-input-logs.json", logs)
    assert post(http, "traces", traces) == 200
    assert post(http, "logs", logs) == 200
    wait_for(lambda: "logRecords" in read_sink(path) and lf.received, "missing positive ingestion controls")
    data = read_sink(path)
    assert all(marker not in data for marker in MARKERS), f"{BACKEND} content leak"
    payloads = [json.loads(line) for line in data.splitlines()]
    spans = [span for p in payloads for r in p.get("resourceSpans", []) for s in r["scopeSpans"] for span in s["spans"]]
    records = [log for p in payloads for r in p.get("resourceLogs", []) for s in r["scopeLogs"] for log in s["logRecords"]]
    assert len(spans) == 2 and len(records) == 1
    gen = next(s for s in spans if s["name"] == "gen_ai.generation")
    fields = {a["key"]: a["value"] for a in gen["attributes"]}
    for key, expected in {"coda.run_id": RUN, "coda.predecessor_run_id": "attempt-0", "coda.republished": "2", "coda.rehearsed_hold_ms": "20000", "gen_ai.request.model": "test-model", "gen_ai.usage.input_tokens": "12", "gen_ai.usage.output_tokens": "34", "error.type": "model_call"}.items():
        assert expected in fields[key].values(), (key, fields)
    assert gen["status"]["code"] == 2 and not gen["status"].get("message")
    assert gen["traceId"] == TRACE and gen["spanId"] == SPAN and gen["links"]
    assert gen["events"][0]["name"] == "operational.event"
    assert records[0]["traceId"] == TRACE and records[0]["severityNumber"] == 17
    lf_spans = [s for p in lf.received for r in p["resourceSpans"] for scope in r["scopeSpans"] for s in scope["spans"]]
    assert len(lf_spans) == 1 and lf_spans[0]["name"] == "gen_ai.generation"
    lf_attrs = {a["key"]: a["value"] for a in lf_spans[0]["attributes"]}
    assert lf_attrs["langfuse.observation.input"]["stringValue"] == MARKERS[0]
    assert lf_attrs["langfuse.observation.output"]["stringValue"] == MARKERS[1]
    print(f"PASS {BACKEND} privacy: 2 ops spans + 1 correlated log; 1 Langfuse generation, exact prompt/response preserved; unique markers absent from entire {BACKEND} payload")


def test_operational_failure(service="duet-api", operation="duet.privacy_recovery.sweep", event="duet.privacy_recovery.failed", classification=None):
    classification = classification or {"step": "recover", "error_class": "database"}
    ops, path = sink(f"{BACKEND}-{event}-ops")
    lf = mock(f"{BACKEND}-{event}-langfuse")
    http, _ = gateway(f"{BACKEND}-{event}", ops, lf)
    traces, logs = fixture()
    span = traces["resourceSpans"][0]["scopeSpans"][0]["spans"][0]
    span["name"] = operation
    span["attributes"] = attrs(**classification)
    traces["resourceSpans"][0]["scopeSpans"][0]["spans"] = [span]
    record = logs["resourceLogs"][0]["scopeLogs"][0]["logRecords"][0]
    record["attributes"] = attrs(**{"event": event, **classification, "error.message": MARKERS[0]})
    for payload, key in [(traces, "resourceSpans"), (logs, "resourceLogs")]:
        payload[key][0]["resource"]["attributes"] = attrs(**{"service.name": service})
    assert post(http, "traces", traces) == 200
    assert post(http, "logs", logs) == 200
    wait_for(lambda: "logRecords" in read_sink(path) and "resourceSpans" in read_sink(path), f"{event} did not reach {BACKEND} sink")
    data = read_sink(path)
    payloads = [json.loads(line) for line in data.splitlines()]
    spans = [s for p in payloads for r in p.get("resourceSpans", []) for scope in r["scopeSpans"] for s in scope["spans"]]
    records = [r for p in payloads for resource in p.get("resourceLogs", []) for scope in resource["scopeLogs"] for r in scope["logRecords"]]
    assert spans[0]["name"] == operation, spans
    assert spans[0]["status"]["code"] == 2
    assert records[0]["traceId"] == spans[0]["traceId"] == TRACE
    assert records[0]["spanId"] == spans[0]["spanId"] == SPAN
    assert records[0]["severityNumber"] == 17
    fields = {a["key"]: a["value"] for a in records[0]["attributes"]}
    for key, value in {"event": event, **classification}.items():
        assert fields[key]["stringValue"] == value
    assert all(marker not in data for marker in MARKERS)
    print(f"PASS {BACKEND} {event}: named error span and correlated safe log survive collector without content")


def test_outage(destination):
    """destination is "langfuse" or "ops" (the selected backend) being down."""
    label = f"{BACKEND}-{destination}-outage"
    lf = mock(f"{label}-langfuse", fail=destination == "langfuse")
    ops, path = sink(f"{label}-ops") if destination == "langfuse" else ((port(), port()), None)
    http, health = gateway(label, ops, lf)
    traces, _ = fixture()
    start = time.monotonic()
    statuses = [post(http, "traces", traces) for _ in range(30)]
    duration = time.monotonic() - start
    assert statuses == [200] * 30 and duration < 5, (statuses, duration)
    with urllib.request.urlopen(f"http://127.0.0.1:{health}", timeout=1) as response:
        assert response.status == 200
    wait_for(lambda: bool(lf.received) if destination == "ops" else bool(read_sink(path)), "healthy downstream blocked by failed sibling")
    wait_for(lambda: "sending queue is full" in (OUT / f"{label}.log").read_text(), "outage did not exercise bounded queue overflow")
    print(f"PASS {BACKEND} {destination} outage: 30/30 receiver requests accepted in {duration:.3f}s; queue overflow exercised; healthy sibling exported and collector health stayed 200")


def b64(data):
    return base64.urlsafe_b64encode(data).decode().rstrip("=")


def test_auth(langfuse_enabled=True):
    issuer = mock("oidc")
    key = OUT / "synthetic-oidc-key.pem"
    subprocess.run(["openssl", "genpkey", "-algorithm", "RSA", "-pkeyopt", "rsa_keygen_bits:2048", "-out", str(key)], check=True, capture_output=True)
    modulus = subprocess.check_output(["openssl", "rsa", "-in", str(key), "-modulus", "-noout"], text=True).strip().split("=")[1]
    issuer.jwk = {"kty": "RSA", "kid": "test", "use": "sig", "alg": "RS256", "n": b64(bytes.fromhex(modulus)), "e": "AQAB"}

    def token(**changes):
        claims = {"iss": issuer.issuer, "aud": "collector-test", "sub": "synthetic-subject", "email": "allowed@example.test", "iat": int(time.time()), "exp": int(time.time()) + 300, **changes}
        message = (b64(json.dumps({"alg": "RS256", "kid": "test"}).encode()) + "." + b64(json.dumps(claims).encode())).encode()
        signature = subprocess.run(["openssl", "dgst", "-sha256", "-sign", str(key)], input=message, capture_output=True, check=True).stdout
        return message.decode() + "." + b64(signature)

    label = f"{BACKEND}-auth" if langfuse_enabled else f"{BACKEND}-auth-no-langfuse"
    ops, path = sink(f"{label}-ops")
    lf = mock(f"{label}-langfuse")
    http, _ = gateway(label, ops, lf, issuer, langfuse_enabled)
    traces, logs = fixture()
    for payload, signal in [(traces, "traces"), (logs, "logs")]:
        for credential in [None, "invalid-token", token(aud="wrong"), token(exp=1), token(iss="http://wrong.invalid")]:
            assert post(http, signal, payload, credential) == 401
        # A valid but unlisted identity cannot spoof the processor-owned field.
        group = payload["resourceSpans"][0]["scopeSpans"][0]["spans"] if signal == "traces" else payload["resourceLogs"][0]["scopeLogs"][0]["logRecords"]
        for item in group:
            item.setdefault("attributes", []).append(attr("coda.ingest.caller", "allowed@example.test"))
        assert post(http, signal, payload, token(email="denied@example.test")) == 200
    time.sleep(0.5)
    assert not read_sink(path).strip() and not lf.received, "denied caller reached sink"
    assert post(http, "traces", traces, token()) == 200
    assert post(http, "logs", logs, token()) == 200
    wait_for(lambda: "logRecords" in read_sink(path) and "resourceSpans" in read_sink(path) and (lf.received or not langfuse_enabled), "authorized caller failed positive ingestion control")
    data = read_sink(path)
    assert "coda.ingest.caller" not in data
    assert all(marker not in data for marker in MARKERS), f"authenticated {BACKEND} content leak"
    if not langfuse_enabled:
        time.sleep(0.5)
        assert not lf.received, "disabled Langfuse received a generation"
    print(f"PASS {label}: missing/malformed/wrong-audience/expired/wrong-issuer rejected (10 HTTP 401s); unlisted email + forged caller dropped; authorized traces/logs exported without content; Langfuse enabled={langfuse_enabled}")


def test_fanout(unavailable=None):
    label = f"both-{unavailable or 'healthy'}"
    dd, dd_path = sink(f"{label}-dd") if unavailable != "datadog" else ((port(), port()), None)
    gf, gf_path = sink(f"{label}-grafana") if unavailable != "grafana" else ((port(), port()), None)
    lf = mock(f"{label}-langfuse", fail=unavailable == "langfuse")
    http, health = gateway(label, dd, lf, grafana=gf)
    traces, logs = fixture()
    started = time.monotonic()
    for _ in range(30 if unavailable else 1):
        assert post(http, "traces", traces) == 200
        assert post(http, "logs", logs) == 200
    elapsed = time.monotonic() - started
    assert elapsed < 5, elapsed
    for path in (dd_path, gf_path):
        if path is None:
            continue
        wait_for(lambda: "resourceSpans" in read_sink(path) and "logRecords" in read_sink(path), "healthy destination did not export both signals")
        data = read_sink(path)
        assert all(marker not in data for marker in MARKERS), "fanout content leak"
        # The file exporter can be appending another batch: parse only
        # newline-terminated records from this snapshot.
        payloads = [json.loads(line) for line in data.split("\n")[:-1]]
        spans = [s for p in payloads for r in p.get("resourceSpans", []) for scope in r["scopeSpans"] for s in scope["spans"]]
        records = [r for p in payloads for resource in p.get("resourceLogs", []) for scope in resource["scopeLogs"] for r in scope["logRecords"]]
        assert any(s["traceId"] == TRACE and s["spanId"] == SPAN for s in spans)
        assert any(r["traceId"] == TRACE and r["spanId"] == SPAN for r in records)
    if unavailable != "langfuse":
        wait_for(lambda: bool(lf.received), "Langfuse blocked")
        assert MARKERS[0] in json.dumps(lf.received)
    if unavailable:
        wait_for(lambda: "sending queue is full" in (OUT / f"{label}.log").read_text(), "bounded overflow not exercised")
    with urllib.request.urlopen(f"http://127.0.0.1:{health}", timeout=1) as response:
        assert response.status == 200
    print(f"PASS both unavailable={unavailable}: sanitized correlated traces/logs at each healthy sink; independent Langfuse; bounded ingest {elapsed:.3f}s; healthy collector")


def run_backend(backend):
    global BACKEND
    BACKEND = backend
    print(f"== backend overlay: collector.{backend}.yaml", flush=True)
    if backend == "both":
        for unavailable in (None, "datadog", "grafana", "langfuse"):
            test_fanout(unavailable)
        return
    test_privacy()
    test_operational_failure()
    test_operational_failure("physician-sample", "physician.receipt_compaction", "physician.receipt_compaction.failed", {"error.type": "database"})
    test_outage("langfuse")
    test_outage("ops")
    test_auth()
    public_key = ENV.pop("LANGFUSE_PUBLIC_KEY")
    secret_key = ENV.pop("LANGFUSE_SECRET_KEY")
    try:
        test_auth(langfuse_enabled=False)
    finally:
        ENV.update(LANGFUSE_PUBLIC_KEY=public_key, LANGFUSE_SECRET_KEY=secret_key)


def main():
    backends = sys.argv[1:] or list(BACKENDS)
    assert all(b in BACKENDS for b in backends), f"backends must be among {BACKENDS}"
    version = subprocess.check_output(["otelcol-contrib", "--version"], text=True).strip()
    revision = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    policies = {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in (ROOT / "dev/observability").glob("*.yaml")}
    write("manifest.json", {"revision": revision, "backends": backends, "policy_sha256": policies, "collector": version, "markers": MARKERS, "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest()})
    print(f"{version}; revision {revision}; artifacts: {OUT}", flush=True)
    try:
        gke = [("collector.yaml", "collector.gke.yaml", *no_lf, f"collector.{b}.yaml") for b in BACKENDS for no_lf in ((), ("collector.gke-no-langfuse.yaml",))]
        local = [("collector.yaml", f"collector.{b}.yaml") for b in ("debug", *BACKENDS)]
        for configs in [("collector.yaml",), *local, *gke, ("sidecar.yaml",)]:
            result = subprocess.run(["otelcol-contrib", "validate", *[arg for name in configs for arg in ("--config", str(ROOT / "dev/observability" / name))]], env=ENV, capture_output=True, text=True)
            (OUT / ("validate-" + "-".join(configs) + ".log")).write_text(result.stdout + result.stderr)
            assert result.returncode == 0, result.stderr
        print("PASS config validation: shared base, local debug/datadog/grafana, GKE datadog/grafana with and without Langfuse, and sidecar")
        for backend in backends:
            run_backend(backend)
    finally:
        for process, handle in reversed(PROCESSES):
            process.terminate()
            try:
                process.wait(timeout=8)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
            handle.close()
        for server in SERVERS:
            server.shutdown()
        # This is a generated test-only signing key, never a deployment secret.
        (OUT / "synthetic-oidc-key.pem").unlink(missing_ok=True)


if __name__ == "__main__":
    main()
