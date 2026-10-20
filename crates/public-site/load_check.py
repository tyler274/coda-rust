#!/usr/bin/env python3
"""Loopback-only release load proof. Run inside `nix develop`; never targets a URL."""
import argparse
import gzip
import hashlib
import http.client
import json
import os
from pathlib import Path
import re
import resource
import signal
import socket
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]


def read_optional(path):
    try:
        return Path(path).read_text().strip()
    except OSError:
        return None


def request(port, path, encoding="identity"):
    connection = http.client.HTTPConnection("127.0.0.1", port, timeout=10)
    try:
        connection.request("GET", path, headers={"Accept-Encoding": encoding})
        response = connection.getresponse()
        return response.status, dict(response.getheaders()), response.read()
    finally:
        connection.close()


def snapshot(pid):
    stat = Path(f"/proc/{pid}/stat").read_text().split()
    status = Path(f"/proc/{pid}/status").read_text()
    return {
        "cpu_seconds": (int(stat[13]) + int(stat[14])) / os.sysconf("SC_CLK_TCK"),
        "rss_kib": int(re.search(r"VmRSS:\s+(\d+)", status)[1]),
        "threads": int(re.search(r"Threads:\s+(\d+)", status)[1]),
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=ROOT / "target/release/public-site")
    parser.add_argument("--output", type=Path, default=ROOT / "target/public-site-load")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    args.output.mkdir(parents=True, exist_ok=True)
    soft, hard = resource.getrlimit(resource.RLIMIT_NOFILE)
    if soft < 4096:
        resource.setrlimit(resource.RLIMIT_NOFILE, (min(4096, hard), hard))
    assert resource.getrlimit(resource.RLIMIT_NOFILE)[0] >= 4096, "Need >=4096 open files"
    # Only the Nix tool PATH survives. In particular no Doppler secrets, proxies,
    # Access credentials, concurrency overrides, or model/data config is inherited.
    clean_env = {"PATH": os.environ["PATH"]}
    cgroup = next(line[3:] for line in Path("/proc/self/cgroup").read_text().splitlines() if line.startswith("0::"))
    cgroup_dir = Path("/sys/fs/cgroup") / cgroup.lstrip("/")
    metadata = {
        "cgroup": cgroup,
        "timestamp_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "kernel": list(os.uname()),
        "cpu_model": next(line.split(":", 1)[1].strip() for line in Path("/proc/cpuinfo").read_text().splitlines() if line.startswith("model name")),
        "logical_cpus": os.cpu_count(),
        "cpu_affinity": sorted(os.sched_getaffinity(0)),
        "meminfo": read_optional("/proc/meminfo"),
        "cpu_max": read_optional(cgroup_dir / "cpu.max"),
        "memory_max": read_optional(cgroup_dir / "memory.max"),
        "loadavg_before": list(os.getloadavg()),
        "fd_limits": resource.getrlimit(resource.RLIMIT_NOFILE),
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "oha_version": subprocess.check_output(["oha", "--version"], env=clean_env, text=True).strip(),
        "configuration": "production; default concurrency/timeout/Tokio workers; HTTP/1.1 keep-alive; gzip; no TLS/CDN; client and server share host",
        "checks": {},
    }
    for environment in ("production", "staging"):
        # Reserve a fresh local port; never connect unless our child remains alive.
        with socket.socket() as reservation:
            reservation.bind(("127.0.0.1", 0))
            port = reservation.getsockname()[1]
        env = dict(clean_env, APP_ENV=environment, PUBLIC_SITE_LISTEN_ADDR=f"127.0.0.1:{port}")
        with (args.output / f"{environment}-server.log").open("w") as log:
            server = subprocess.Popen([str(binary)], env=env, stdout=log, stderr=log)
            try:
                deadline = time.monotonic() + 120
                while True:
                    assert server.poll() is None, f"Server exited: {log.name}"
                    try:
                        status, headers, body = request(port, "/")
                        break
                    except (OSError, http.client.HTTPException):
                        assert time.monotonic() < deadline, "Startup timed out"
                        time.sleep(0.1)
                if environment == "staging":
                    for path in ("/", asset, "/api/health", "/missing"):
                        status, headers, body = request(port, path)
                        assert status == 403, (path, status)
                        assert "no-store" in headers["cache-control"]
                        assert "noindex" in headers["x-robots-tag"]
                    metadata["staging_deny_all"] = "403/no-store/noindex for page, hashed asset, health, missing path; no credentials or upstream calls"
                    continue
                assert status == 200 and b"<main" in body and b"<title" in body
                asset = re.search(rb'href="(/a/[^\"]+\.css)"', body)[1].decode()
                for name, path in (("home", "/"), ("asset", asset)):
                    status, headers, identity = request(port, path)
                    assert status == 200
                    status, headers, compressed = request(port, path, "gzip")
                    assert status == 200 and headers["content-encoding"] == "gzip"
                    assert gzip.decompress(compressed) == identity
                    assert "public" in headers["cache-control"]
                    content_type = "text/html" if name == "home" else "text/css"
                    cache_directive = "s-maxage=3600" if name == "home" else "immutable"
                    assert content_type in headers["content-type"]
                    assert cache_directive in headers["cache-control"]
                    metadata["checks"][name] = {"path": path, "headers": headers, "identity_bytes": len(identity), "gzip_bytes": len(compressed), "identity_sha256": hashlib.sha256(identity).hexdigest()}
                    command = ["oha", "--no-tui", "--output-format", "json", "--stats-success-breakdown", "--http-version", "1.1", "--disable-compression", "-H", "Accept-Encoding: gzip", "-c", "1000", "-z", "30s", "-w", "-t", "10s", f"http://127.0.0.1:{port}{path}"]
                    before = snapshot(server.pid)
                    usage_before = resource.getrusage(resource.RUSAGE_CHILDREN)
                    result = subprocess.run(command, env=clean_env, text=True, capture_output=True, check=True, timeout=60)
                    usage_after = resource.getrusage(resource.RUSAGE_CHILDREN)
                    after = snapshot(server.pid)
                    (args.output / f"{name}.json").write_text(result.stdout)
                    report = json.loads(result.stdout)
                    # Keep overload/error counts as evidence, never raise limits to
                    # turn a failed target into a passing benchmark. All-200 runs
                    # must account for exactly the expected compressed payload.
                    statuses = report["statusCodeDistribution"]
                    if set(statuses) == {"200"} and not report["errorDistribution"]:
                        assert report["summary"]["totalData"] == statuses["200"] * len(compressed)
                    metadata["checks"][name].update(command=command, server_before=before, server_after=after, client_cpu_seconds=usage_after.ru_utime + usage_after.ru_stime - usage_before.ru_utime - usage_before.ru_stime)
                    status, _, after_body = request(port, path, "gzip")
                    assert status == 200 and after_body == compressed, "Post-load response changed"
                    print(name, json.dumps(report, indent=2), flush=True)
            finally:
                if server.poll() is None:
                    server.send_signal(signal.SIGTERM)
                    try:
                        server.wait(timeout=20)
                    except subprocess.TimeoutExpired:
                        server.kill()
                        server.wait()
                metadata[f"{environment}_exit_code"] = server.returncode
    metadata["loadavg_after"] = list(os.getloadavg())
    (args.output / "metadata.json").write_text(json.dumps(metadata, indent=2) + "\n")
    print("Results:", args.output)
    print("Staging negative checks:", metadata["staging_deny_all"])
    print("Child server exit codes:", metadata["production_exit_code"], metadata["staging_exit_code"])


if __name__ == "__main__":
    main()
