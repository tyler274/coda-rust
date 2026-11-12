#!/usr/bin/env python3
"""Render real Terraform locals; verify Grafana provisioning on isolated Docker.

Network-none Docker with loopback-only Prometheus and notification fixtures;
no real endpoints, keys or Slack. Reuse one disposable database to prove optional
provisioning deletion. Test-only intervals are compressed, production queries,
thresholds and route matchers are retained. The synthetic notification rule's
Loki query is the one exception: it reads the loopback fixture instead, so the
provisioned rule itself drives the test route. Docker images must already exist.
"""
import copy
import http.server
import json
import sys
import pathlib
import re
import subprocess
import time
import shutil

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUT = ROOT / "target/verification" / f"grafana-shadow-{time.time_ns()}"
OUT.mkdir(parents=True)
NAME = f"coda-shadow-test-{time.time_ns()}"
IMAGE = "docker.io/grafana/grafana:13.3.0"
VARS = {
    "environment": "staging", "project_id": "synthetic-test", "cluster_project": "synthetic-test",
    "cluster_location": "us-east1", "cluster_name": "synthetic",
    "otel_gateway_ip": "10.82.0.10", "otel_gateway_source_ranges": ["10.80.0.0/24"],
    "otel_gateway_audience": "synthetic", "otel_gateway_allowed_callers": ["synthetic@synthetic-test.iam.gserviceaccount.com"],
    "telemetry_backend": "both",
}


def run(*args, **kwargs):
    result = subprocess.run(args, capture_output=True, text=True, **kwargs)
    if result.returncode:
        raise RuntimeError(f"{args}: {result.stderr}")
    return result.stdout


def fixture_server():
    """Bounded synthetic Prometheus source and Slack-shaped loopback sink."""
    class Handler(http.server.BaseHTTPRequestHandler):
        state = {"shadow": 0, "test": 0}
        receipts = []

        def log_message(self, *_):
            pass

        def do_GET(self):
            if self.path == "/metrics":
                payload = '\n'.join([
                    f'coda_compat_readiness_consecutive_failures{{environment="staging",service="synthetic",target="synthetic"}} {4 * self.state["shadow"]}',
                    f'coda_compat_dependency_unhealthy{{environment="staging",service="synthetic",dependency="synthetic"}} {self.state["shadow"]}',
                    f'coda_compat_last_observed_seconds{{environment="staging",service="synthetic",target="synthetic"}} {time.time()}',
                    f'synthetic_alert_value{{case="shadow"}} {self.state["shadow"]}',
                    f'synthetic_alert_value{{case="test"}} {self.state["test"]}', '',
                ]).encode()
                content_type = "text/plain; version=0.0.4"
            else:
                payload = json.dumps(self.receipts).encode()
                content_type = "application/json"
            self.send_response(200)
            self.send_header("Content-Type", content_type)
            self.end_headers()
            self.wfile.write(payload)

        def do_POST(self):
            length = int(self.headers.get("Content-Length", 0))
            assert length < 65536
            value = json.loads(self.rfile.read(length))
            if self.path == "/state":
                assert set(value) == {"shadow", "test"} and all(v in (0, 1) for v in value.values())
                self.state.update(value)
            elif self.path == "/slack":
                assert len(self.receipts) < 20
                self.receipts.append({"time": time.time(), "payload": value})
            else:
                self.send_error(404)
                return
            self.send_response(200)
            self.end_headers()
            self.wfile.write(b"ok")

    http.server.ThreadingHTTPServer(("127.0.0.1", 9099), Handler).serve_forever()


def provision(enabled, label=None):
    variables = {**VARS, "grafana_test_notifications_enabled": enabled,
                 "grafana_cloud_monitoring_enabled": enabled,
                 "grafana_compatibility_metrics_url": "http://127.0.0.1:9090" if enabled else ""}
    vf = OUT / "synthetic.tfvars.json"
    vf.write_text(json.dumps(variables))
    # Copy the source root into an isolated local-backend harness. Never
    # initialize or read the deployment GCS backend, even for console.
    harness = OUT / "harness/infra/gcp/observability"
    if not harness.exists():
        shutil.copytree(ROOT / "infra/gcp/observability", harness, ignore=shutil.ignore_patterns(".terraform", "terraform.tfstate*", "env"))
        shutil.copytree(ROOT / "dev/observability", OUT / "harness/dev/observability")
        for source in harness.glob("*.tf"):
            text = source.read_text()
            if 'backend "gcs"' in text:
                source.write_text(re.sub(r'backend "gcs"\s*\{[^}]*\}', 'backend "local" {}', text))
        run("tofu", f"-chdir={harness}", "init", "-input=false")
    expression = 'jsonencode({data=local.grafana_components.grafana.config_data, paths=local.grafana_components.grafana.config_items, contact=jsondecode(jsonencode(yamldecode(local.grafana_contact_config))), rules=yamldecode(file("${local.grafana_config_dir}/compatibility-rules.yaml")), synthetic=yamldecode(local.grafana_synthetic_rules), cloud=yamldecode(local.grafana_cloud_rules)})'
    result = run("tofu", f"-chdir={harness}", "console", f"-var-file={vf}", input=expression)
    config = json.loads(json.loads(result))
    folder = OUT / (label or ("enabled" if enabled else "disabled"))
    for key, value in config["data"].items():
        path = folder / config["paths"][key]
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(value)
    if enabled:
        # JSON is YAML-compatible. Preserve rendered matchers, mute interval,
        # source queries and thresholds; only compress evaluator timings.
        contact = config["contact"]
        contact["policies"][0].update(group_wait="1s", group_interval="5s", repeat_interval="1h")
        (folder / "alerting/contact.yaml").write_text(json.dumps(contact))
        rules = config["rules"]
        group = rules["groups"][0]
        group["interval"] = "10s"
        for rule in group["rules"]:
            rule["for"] = "20s"
        template = group["rules"][0]
        # The provisioned synthetic rule keeps its title, labels and threshold.
        # Only its Loki query moves to the loopback fixture.
        synthetic = config["synthetic"]
        synthetic_group = synthetic["groups"][0]
        synthetic_group["interval"] = "10s"
        for rule in synthetic_group["rules"]:
            rule["for"] = "20s"
            rule["data"][0].update(datasourceUid="compatibility", model={"refId": "A", "instant": True, "expr": 'synthetic_alert_value{case="test"} * 4'})
        (folder / "alerting/synthetic-rules.yaml").write_text(json.dumps(synthetic))
        # Negative controls prove neither matcher alone bypasses shadow mode.
        for uid, title, labels, case in [
            ("synthetic-title-only", "Coda synthetic notification test", {}, "shadow"),
            ("synthetic-label-only", "Synthetic wrong title", {"qualification": "synthetic-notification-test"}, "shadow"),
        ]:
            rule = copy.deepcopy(template)
            rule.update(uid=uid, title=title, labels={"team": "coda", **labels})
            rule["data"][0]["model"]["expr"] = f'synthetic_alert_value{{case="{case}"}} * 4'
            group["rules"].append(rule)
        (folder / "alerting/compatibility.yaml").write_text(json.dumps(rules))
        (folder / "datasources/compatibility.yaml").write_text(json.dumps({"apiVersion": 1, "datasources": [{"name": "Compatibility metrics", "uid": "compatibility", "type": "prometheus", "access": "proxy", "url": "http://127.0.0.1:9090", "jsonData": {"httpMethod": "GET"}}]}))
    return folder, config["cloud"]


def cloud_uids(cloud):
    """The rendered cloud rule file's provisioned UIDs and the UIDs it deletes
    that it does not provision: retired rules, or every rule when disabled."""
    provisioned = [r["uid"] for g in cloud["groups"] for r in g["rules"]]
    deleted = [d["uid"] for d in cloud["deleteRules"] if d["uid"] not in provisioned]
    return provisioned, deleted


def seed_retired(volume):
    """Provision the rules a deployed Grafana could still hold under retired
    UIDs, on the volume the later passes reuse, so their deletion is real."""
    folder, cloud = provision(True, "retired")
    provisioned, retired = cloud_uids(cloud)
    assert retired, "the enabled cloud rule file names no retired UIDs"
    group = cloud["groups"][0]
    for uid in retired:
        rule = copy.deepcopy(group["rules"][0])
        rule.update(uid=uid, title=f"Retired {uid}")
        group["rules"].append(rule)
    del cloud["deleteRules"]
    (folder / "alerting/cloud-rules.yaml").write_text(json.dumps(cloud))
    start(folder, volume)
    present = {r["uid"] for r in api("/api/v1/provisioning/alert-rules")}
    assert set(retired) <= present, f"retired rules not seeded: {sorted(set(retired) - present)}"
    print(f"PASS seeded retired cloud rules {sorted(retired)} beside {len(provisioned)} current ones", flush=True)
    stop("retired")
    return retired


def api(path):
    return json.loads(run("docker", "exec", NAME, "wget", "-qO-", f"http://admin:admin@127.0.0.1:3000{path}"))


def native_query_boundary(rules):
    query = copy.deepcopy(rules[0]["data"][0]["model"])
    query["datasource"] = {"uid": "cloud-monitoring", "type": "stackdriver"}
    payload = json.dumps({"from": str(int(time.time() * 1000) - 600000),
                          "to": str(int(time.time() * 1000)), "queries": [query]})
    code = """
import urllib.request, urllib.error, sys
request = urllib.request.Request('http://127.0.0.1:3000/api/ds/query',
    data=sys.argv[1].encode(), headers={'Authorization': 'Basic YWRtaW46YWRtaW4=', 'Content-Type': 'application/json'})
try:
    response = urllib.request.urlopen(request, timeout=45)
except urllib.error.HTTPError as error:
    response = error
print(response.read().decode())
"""
    response = run("docker", "run", "--rm", "--network", f"container:{NAME}",
                   "--entrypoint", "python", "docker.io/python:3.14.10-alpine3.24",
                   "-c", code, payload)
    (OUT / "native-query-boundary.json").write_text(response)
    logs = subprocess.run(["docker", "logs", NAME], capture_output=True, text=True, check=True)
    assert "panic triggered" not in logs.stdout + logs.stderr, "native datasource panic triggered"
    result = json.loads(response)["results"]["A"]
    error = result.get("error", "")
    assert "panic" not in error.lower() and "nil pointer" not in error.lower(), error
    assert result.get("status") == 200, result
    assert all(not frame["data"]["values"] for frame in result.get("frames", [])), result
    print("PASS native model executes without migration panic; network-none returns empty frames, not authenticated evaluation proof", flush=True)


def start(folder, volume):
    run("docker", "run", "-d", "--name", NAME, "--network", "none",
        "-e", "GF_ANALYTICS_REPORTING_ENABLED=false", "-e", "GF_ANALYTICS_CHECK_FOR_UPDATES=false",
        "-e", "GF_ANALYTICS_CHECK_FOR_PLUGIN_UPDATES=false", "-e", "GF_SECURITY_ADMIN_PASSWORD=admin",
        "-e", "GRAFANA_SLACK_WEBHOOK_URL=http://127.0.0.1:9099/slack",
        "-e", "GF_UNIFIED_ALERTING_MIN_INTERVAL=10s",
        "-v", f"{folder}:/etc/grafana/provisioning:ro", "-v", f"{volume}:/var/lib/grafana", IMAGE)
    deadline = time.monotonic() + 45
    while time.monotonic() < deadline:
        try:
            if api("/api/health").get("database") == "ok":
                return
        except (RuntimeError, json.JSONDecodeError):
            pass
        time.sleep(0.2)
    raise AssertionError("Grafana failed startup")


def evaluate():
    # Both helpers share Grafana's network-none namespace. All three services
    # communicate exclusively over loopback; no port is published.
    run("docker", "run", "-d", "--name", NAME + "-fixture", "--network", f"container:{NAME}",
        "-v", f"{pathlib.Path(__file__).resolve()}:/app/scripts/fixture.py:ro",
        "--entrypoint", "python", "docker.io/python:3.14.10-alpine3.24", "/app/scripts/fixture.py", "--serve")
    prometheus = OUT / "prometheus.json"
    prometheus.write_text(json.dumps({"global": {"scrape_interval": "1s"}, "scrape_configs": [{"job_name": "synthetic", "static_configs": [{"targets": ["127.0.0.1:9099"]}]}]}))
    run("docker", "run", "-d", "--name", NAME + "-prometheus", "--network", f"container:{NAME}",
        "-v", f"{prometheus}:/etc/prometheus/prometheus.yml:ro", "docker.io/prom/prometheus:v3.16.0",
        "--config.file=/etc/prometheus/prometheus.yml", "--storage.tsdb.retention.time=1h")
    history = []

    def fixture(path, state=None):
        args = ["docker", "exec", NAME, "wget", "-qO-"]
        if state is not None:
            args.append("--post-data=" + json.dumps(state))
        result = run(*args, f"http://127.0.0.1:9099/{path}")
        return json.loads(result) if state is None else None

    def states():
        groups = api("/api/prometheus/grafana/api/v1/rules")["data"]["groups"]
        result = {r["name"] + ":" + r.get("labels", {}).get("qualification", ""): r for g in groups for r in g["rules"]}
        history.append({"time": time.time(), "rules": result})
        (OUT / "evaluation-history.json").write_text(json.dumps(history, indent=2))
        return result

    def wait(predicate, label, seconds=100):
        deadline = time.monotonic() + seconds
        while time.monotonic() < deadline:
            if predicate():
                return
            time.sleep(1)
        raise AssertionError(f"Timed out: {label}")

    def matches(name, state, qualification=""):
        rule = states().get(name + ":" + qualification, {})
        return rule.get("state") == state and rule.get("health") == "ok" and rule.get("lastEvaluation", "")[:4].isdigit() and int(rule["lastEvaluation"][:4]) > 2000

    qualified = "synthetic-notification-test"
    title = "Coda synthetic notification test"
    wait(lambda: matches("Dependency compatibility failure", "inactive"), "healthy baseline")
    assert fixture("received") == []
    fixture("state", {"shadow": 1, "test": 0})
    wait(lambda: matches("Dependency compatibility failure", "pending"), "operational pending")
    wait(lambda: matches("Dependency compatibility failure", "firing"), "operational firing")
    wait(lambda: matches(title, "firing") and matches("Synthetic wrong title", "firing", qualified), "both one-label negative controls firing")
    # Hold shadow-only firing beyond group_wait + group_interval.
    until = time.monotonic() + 12
    while time.monotonic() < until:
        assert fixture("received") == [], "shadow/one-label alert notified"
        time.sleep(1)
    fixture("state", {"shadow": 1, "test": 1})
    wait(lambda: matches(title, "pending", qualified), "qualified test pending")
    wait(lambda: matches(title, "firing", qualified), "qualified test firing")
    wait(lambda: len(fixture("received")) >= 1, "local firing receipt")
    firing = fixture("received")
    (OUT / "local-firing-receipts.json").write_text(json.dumps(firing, indent=2))
    assert len(firing) == 1 and "FIRING" in json.dumps(firing)
    assert "Synthetic wrong title" not in json.dumps(firing) and "Dependency compatibility" not in json.dumps(firing)
    fixture("state", {"shadow": 0, "test": 0})
    wait(lambda: matches(title, "inactive", qualified) and matches("Dependency compatibility failure", "inactive"), "recovery")
    wait(lambda: len(fixture("received")) >= 2, "local recovery receipt")
    receipts = fixture("received")
    (OUT / "local-receipts.json").write_text(json.dumps(receipts, indent=2))
    assert len(receipts) == 2 and "RESOLVED" in json.dumps(receipts[-1])
    for receipt in receipts:
        attachments = receipt["payload"]["attachments"]
        assert len(attachments) == 1
        assert "/coda-synthetic-notification-test/" in attachments[0]["title_link"]
        assert "qualification = synthetic-notification-test" in attachments[0]["text"]
    # Allow a notification grouping cycle after recovery as a negative control.
    until = time.monotonic() + 12
    while time.monotonic() < until:
        assert len(fixture("received")) == 2, "unexpected shadow recovery notification"
        time.sleep(1)
    print("PASS real evaluator: normal -> pending -> firing -> recovery; exact local firing/recovery receipts; operational and each single-label control remained muted", flush=True)


def stop(label):
    for helper in ("-prometheus", "-fixture"):
        logs = subprocess.run(["docker", "logs", NAME + helper], capture_output=True, text=True)
        (OUT / f"{label}{helper}.log").write_text(logs.stdout + logs.stderr)
        subprocess.run(["docker", "rm", "-fv", NAME + helper], check=False, capture_output=True)
    result = subprocess.run(["docker", "logs", NAME], capture_output=True, text=True)
    (OUT / f"{label}.log").write_text(result.stdout + result.stderr)
    subprocess.run(["docker", "rm", "-f", NAME], check=False, capture_output=True)


def main():
    volume = NAME + "-data"
    print(f"revision {run('git', 'rev-parse', 'HEAD').strip()}; artifacts {OUT}", flush=True)
    try:
        retired = seed_retired(volume)
        for enabled in (True, False):
            label = "enabled" if enabled else "disabled"
            folder, cloud = provision(enabled)
            provisioned, deleted = cloud_uids(cloud)
            assert set(retired) <= set(deleted), "a retired cloud rule UID is no longer deleted"
            start(folder, volume)
            policy = api("/api/v1/provisioning/policies")
            contacts = api("/api/v1/provisioning/contact-points")
            rules = api("/api/v1/provisioning/alert-rules")
            datasources = api("/api/datasources")
            snapshot = dict(policy=policy, contacts=contacts, rules=rules, datasources=datasources)
            (OUT / f"{label}.json").write_text(json.dumps(snapshot, indent=2))
            assert policy["receiver"] == "coda-shadow"
            assert policy["routes"][-1]["mute_time_intervals"] == ["coda-shadow-always"]
            assert (any(c["uid"] == "coda-slack-test" for c in contacts)) == enabled
            assert (any(d["uid"] == "compatibility" for d in datasources)) == enabled
            assert (any(d["uid"] == "cloud-monitoring" for d in datasources)) == enabled
            cloud_rules = [r for r in rules if r["uid"].startswith("gcp-")]
            assert sorted(r["uid"] for r in cloud_rules) == sorted(provisioned)
            assert bool(provisioned) == enabled
            assert not {r["uid"] for r in rules} & set(deleted), "a deleted cloud rule is still provisioned"
            assert all(r["labels"]["mode"] == "shadow" for r in cloud_rules)
            assert len([r for r in rules if r["uid"] in ("coda-readiness-shadow", "coda-dependency-shadow", "coda-signal-freshness-shadow")]) == (3 if enabled else 0)
            if enabled:
                assert sorted(policy["routes"][0]["object_matchers"]) == sorted([["qualification", "=", "synthetic-notification-test"], ["alertname", "=", "Coda synthetic notification test"]])
            else:
                assert len(policy["routes"]) == 1
                assert not any(r["uid"] == "coda-synthetic-notification-test" for r in rules)
            print(f"PASS {label}: real rendered rules/datasources/contact/policy reconciled ({len(provisioned)} cloud rules; {len(deleted)} deleted UIDs absent); operational catch-all muted", flush=True)
            if enabled:
                native_query_boundary(cloud_rules)
                evaluate()
            stop(label)
    finally:
        stop("cleanup")
        subprocess.run(["docker", "volume", "rm", volume], check=False, capture_output=True)


if __name__ == "__main__":
    if sys.argv[1:] == ["--serve"]:
        fixture_server()
    else:
        main()
