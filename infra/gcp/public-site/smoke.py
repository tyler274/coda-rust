#!/usr/bin/env python3
"""Local image contract proof; no cloud credentials or deployment actions.

Run from the workspace root in Nix:
  nix develop --command python infra/gcp/public-site/smoke.py public-site:deployment-check
"""
import hashlib
from html.parser import HTMLParser
import json
import re
import subprocess
import sys
import time
from urllib.error import HTTPError, URLError
from urllib.parse import urljoin
from urllib.request import urlopen


def docker(*args):
    return subprocess.check_output(["docker", *args], text=True).strip()


class Document(HTMLParser):
    def __init__(self):
        super().__init__()
        self.styles = []
        self.modules = []
        self.canonical = []
        self.description = []
        self.tags = []

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        self.tags.append(tag)
        if tag == "link" and attrs.get("rel") == "stylesheet":
            self.styles.append(attrs["href"])
        if tag == "link" and attrs.get("rel") == "canonical":
            self.canonical.append(attrs["href"])
        if tag == "meta" and attrs.get("name") == "description":
            self.description.append(attrs.get("content", ""))
        if tag == "script" and attrs.get("type") == "module":
            self.modules.append(attrs["src"])


def fetch(base, path):
    try:
        response = urlopen(urljoin(base, path), timeout=5)
    except HTTPError as error:
        response = error
    with response:
        return response.status, response.headers, response.read()


def container(image, environment):
    return docker(
        "run", "-d", "--read-only", "--cap-drop=ALL",
        "--security-opt=no-new-privileges", "--memory=512m", "--cpus=1",
        "-p", "127.0.0.1::8080", "-e", f"APP_ENV={environment}",
        "-e", "RELEASE_SHA=" + "a" * 40,
        "-e", "PHYSICIAN_APP_URL=https://physician.example.test", image,
    )


def await_ready(cid):
    info = json.loads(docker("inspect", cid))[0]
    port = info["NetworkSettings"]["Ports"]["8080/tcp"][0]["HostPort"]
    base = f"http://127.0.0.1:{port}/"
    deadline = time.monotonic() + 90
    while time.monotonic() < deadline:
        try:
            fetch(base, "/healthz")
            return base
        except (URLError, TimeoutError, ConnectionError):
            state = json.loads(docker("inspect", cid))[0]["State"]
            assert state["Running"], docker("logs", cid)
            time.sleep(0.2)
    raise AssertionError("Image did not become reachable within 90 seconds")


def stop(cid):
    started = time.monotonic()
    docker("stop", "--time=10", cid)
    state = json.loads(docker("inspect", cid))[0]["State"]
    elapsed = time.monotonic() - started
    assert state["ExitCode"] == 0 and not state["OOMKilled"], state
    assert elapsed < 10, f"SIGTERM shutdown took {elapsed:.2f}s"
    print(f"SIGTERM: clean exit 0 in {elapsed:.2f}s")


def main():
    image = sys.argv[1] if len(sys.argv) > 1 else "public-site:deployment-check"
    info = json.loads(docker("image", "inspect", image))[0]
    config = info["Config"]
    assert config["User"] == "65532:65532", config["User"]
    assert config["Entrypoint"] == ["/public-site"], config["Entrypoint"]
    env = dict(item.split("=", 1) for item in config["Env"])
    assert env["PUBLIC_SITE_LISTEN_ADDR"] == "0.0.0.0:8080", env
    assert set(env) <= {"PATH", "SSL_CERT_FILE", "PUBLIC_SITE_LISTEN_ADDR"}, list(env)
    assert not config.get("Volumes"), config.get("Volumes")
    print(f"Image ID: {info['Id']}; size: {info['Size']} bytes; user: {config['User']}")
    print(f"Image environment names: {', '.join(sorted(env))}; no secret-bearing build args supplied")

    cid = container(image, "production")
    try:
        base = await_ready(cid)
        status, headers, body = fetch(base, "/")
        assert status == 200 and "text/html" in headers["Content-Type"]
        html = body.decode()
        document = Document()
        document.feed(html)
        assert re.search(r"<title>[^<]+</title>", html)
        assert document.canonical and document.description[0]
        assert "main" in document.tags and "footer" in document.tags
        assert "https://physician.example.test" in html
        assert len(body) > 10000, len(body)
        print(f"Homepage: 200, {len(body)} bytes, title/canonical/description/main/footer/sign-in valid")
        assert len(document.styles) >= 5, document.styles
        for path in document.styles:
            status, headers, css = fetch(base, path)
            assert path.startswith("/a/") and status == 200 and css, path
            assert "text/css" in headers["Content-Type"], path
            assert "immutable" in headers["Cache-Control"], path
        print(f"CSS: {len(document.styles)} content-addressed stylesheets served with immutable caching")
        assert len(document.modules) == 1, document.modules
        bootstrap = document.modules[0]
        assert bootstrap.startswith("/a/") and bootstrap.endswith("/motion/bootstrap.js")
        status, headers, js = fetch(base, bootstrap)
        assert status == 200 and "javascript" in headers["Content-Type"]
        assert b"./public_site_motion.js" in js
        glue = urljoin(bootstrap, "./public_site_motion.js")
        status, headers, js = fetch(base, glue)
        assert status == 200 and "javascript" in headers["Content-Type"]
        assert b"public_site_motion_bg.wasm" in js and b"import.meta.url" in js
        wasm = urljoin(glue, "./public_site_motion_bg.wasm")
        status, headers, body = fetch(base, wasm)
        assert status == 200 and headers["Content-Type"] == "application/wasm"
        assert body.startswith(b"\x00asm") and "immutable" in headers["Cache-Control"]
        print(f"Motion: sibling bootstrap/JS/WASM served; wasm {len(body)} bytes, sha256 {hashlib.sha256(body).hexdigest()}")
        for path in ["/healthz", "/readyz"]:
            assert fetch(base, path)[0] == 200, path
        status, headers, body = fetch(base, "/api/health")
        health = json.loads(body)
        assert status == 200 and health["ok"] and health["env"] == "production"
        assert health["release_sha"] == "a" * 40, health
        assert fetch(base, "/does-not-exist")[0] == 404
        print("Health/readiness: 200, release/env correct; unknown route: 404; container binding reachable")
        stop(cid)
    finally:
        docker("rm", "-f", cid)

    cid = container(image, "staging")
    try:
        base = await_ready(cid)
        for path in ["/", bootstrap, glue, wasm, document.styles[0], "/api/health", "/healthz", "/does-not-exist"]:
            status, headers, body = fetch(base, path)
            assert status in (401, 403), (path, status)
            assert "no-store" in headers["Cache-Control"], path
            assert "noindex" in headers["X-Robots-Tag"], path
        print("Staging without credentials: pages, CSS, JS/WASM, health and 404 deny access with no-store/noindex")
        stop(cid)
    finally:
        docker("rm", "-f", cid)


if __name__ == "__main__":
    main()
