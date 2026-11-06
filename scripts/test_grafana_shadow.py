#!/usr/bin/env python3
"""Render real Terraform locals; verify Grafana provisioning on isolated Docker.

No network, real endpoints, keys, or Slack. Reuse one disposable database to
prove optional provisioning is deleted when disabled. Docker image must exist.
"""
import json
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
    "cluster_location": "us-east1", "cluster_name": "synthetic", "network_name": "synthetic",
    "otel_gateway_ip": "10.82.0.10", "otel_gateway_source_ranges": ["10.80.0.0/24"],
    "otel_gateway_audience": "synthetic", "otel_gateway_allowed_callers": ["synthetic@synthetic-test.iam.gserviceaccount.com"],
    "telemetry_backend": "both",
}


def run(*args, **kwargs):
    result = subprocess.run(args, capture_output=True, text=True, **kwargs)
    if result.returncode:
        raise RuntimeError(f"{args}: {result.stderr}")
    return result.stdout


def provision(enabled):
    variables = {**VARS, "grafana_test_notifications_enabled": enabled,
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
    expression = "jsonencode({data=local.grafana_components.grafana.config_data, paths=local.grafana_components.grafana.config_items})"
    result = run("tofu", f"-chdir={harness}", "console", f"-var-file={vf}", input=expression)
    config = json.loads(json.loads(result))
    folder = OUT / ("enabled" if enabled else "disabled")
    for key, value in config["data"].items():
        path = folder / config["paths"][key]
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(value)
    return folder


def api(path):
    return json.loads(run("docker", "exec", NAME, "wget", "-qO-", f"http://admin:admin@127.0.0.1:3000{path}"))


def start(folder, volume):
    run("docker", "run", "-d", "--name", NAME, "--network", "none",
        "-e", "GF_ANALYTICS_REPORTING_ENABLED=false", "-e", "GF_ANALYTICS_CHECK_FOR_UPDATES=false",
        "-e", "GF_ANALYTICS_CHECK_FOR_PLUGIN_UPDATES=false", "-e", "GF_SECURITY_ADMIN_PASSWORD=admin",
        "-e", "GRAFANA_SLACK_WEBHOOK_URL=http://127.0.0.1:1/synthetic-only",
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


def stop(label):
    result = subprocess.run(["docker", "logs", NAME], capture_output=True, text=True)
    (OUT / f"{label}.log").write_text(result.stdout + result.stderr)
    subprocess.run(["docker", "rm", "-f", NAME], check=False, capture_output=True)


def main():
    volume = NAME + "-data"
    print(f"revision {run('git', 'rev-parse', 'HEAD').strip()}; artifacts {OUT}", flush=True)
    try:
        for enabled in (True, False):
            label = "enabled" if enabled else "disabled"
            start(provision(enabled), volume)
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
            assert len([r for r in rules if r["uid"] in ("coda-readiness-shadow", "coda-dependency-shadow", "coda-signal-freshness-shadow")]) == (3 if enabled else 0)
            if enabled:
                assert sorted(policy["routes"][0]["object_matchers"]) == sorted([["qualification", "=", "synthetic-notification-test"], ["alertname", "=", "Coda synthetic notification test"]])
            else:
                assert len(policy["routes"]) == 1
            print(f"PASS {label}: real rendered rules/datasources/contact/policy reconciled; operational catch-all muted", flush=True)
            stop(label)
    finally:
        stop("cleanup")
        subprocess.run(["docker", "volume", "rm", volume], check=False, capture_output=True)


if __name__ == "__main__":
    main()
