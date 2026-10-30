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
from urllib.request import Request, urlopen

# kytos public/homepage/site.js at the reviewed snapshot commit (provenance.json).
ORIGINAL_SITE_JS_SHA256 = "672b93cee690408b072033feb643a1e15dfe70d7022a5d5c2b5d79482bf59b80"


def docker(*args):
    return subprocess.check_output(["docker", *args], text=True).strip()


class Document(HTMLParser):
    def __init__(self):
        super().__init__()
        self.styles = []
        self.modules = []
        self.scripts = []
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
        elif tag == "script" and "src" in attrs:
            self.scripts.append(attrs["src"])


def fetch(base, path, headers=None):
    try:
        response = urlopen(Request(urljoin(base, path), headers=headers or {}), timeout=5)
    except HTTPError as error:
        response = error
    with response:
        return response.status, response.headers, response.read()


def container(image, environment, physician_app_url=None):
    extra_env = []
    if physician_app_url:
        # The physician upstream alone decides whether the site offers sign-in.
        extra_env += ["-e", f"PHYSICIAN_APP_URL={physician_app_url}"]
    if environment == "staging":
        # Startup requires the direct Cloud Run signed-header audience. Local
        # image tests never substitute a trust endpoint or fetch cloud tokens.
        extra_env += [
            "-e", "PHYSICIAN_PROXY_SERVICE_ACCOUNT=public-site@fixture.iam.gserviceaccount.com",
            "-e", "PUBLIC_SITE_IAP_AUDIENCE=/projects/134570912572/locations/us-east1/services/public-site-staging",
        ]
    return docker(
        "run", "-d", "--read-only", "--cap-drop=ALL",
        "--security-opt=no-new-privileges", "--memory=512m", "--cpus=1",
        "-p", "127.0.0.1::8080", "-e", f"APP_ENV={environment}",
        "-e", "RELEASE_SHA=" + "a" * 40, *extra_env, image,
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
        assert headers["Strict-Transport-Security"] == "max-age=63072000", headers
        html = body.decode()
        document = Document()
        document.feed(html)
        assert re.search(r"<title>[^<]+</title>", html)
        assert document.canonical and document.description[0]
        assert "main" in document.tags and "footer" in document.tags
        assert 'href="/physician/"' not in html and "Sign in" not in html
        assert len(body) > 10000, len(body)
        for path in ["/physician", "/physician/today"]:
            assert fetch(base, path)[0] == 404, path
        print(f"Homepage: 200, {len(body)} bytes, title/canonical/description/main/footer valid; no physician upstream, so no Sign in and /physician 404")
        # The homepage system: original styles, frame reservation and the no-script reveal.
        assert [path.rsplit("/", 1)[1] for path in document.styles] == [
            "chrome.css", "site.css", "stage.css", "duet.css", "noscript.css"], document.styles
        for path in document.styles:
            status, headers, css = fetch(base, path)
            assert path.startswith("/a/") and status == 200 and css, path
            assert "text/css" in headers["Content-Type"], path
            assert "immutable" in headers["Cache-Control"], path
        print(f"CSS: {len(document.styles)} content-addressed stylesheets served with immutable caching")
        assert len(document.modules) == 1, document.modules
        chrome = document.modules[0]
        assert chrome.startswith("/a/") and chrome.endswith("/chrome.js")
        status, headers, js = fetch(base, chrome)
        assert status == 200 and "javascript" in headers["Content-Type"] and b"scrolled" in js
        assert len(document.scripts) == 2, document.scripts
        motion, story = document.scripts
        assert motion.startswith("/a/") and motion.endswith("/homepage/site.js")
        assert story.startswith("/a/") and story.endswith("/homepage/duet.js")
        status, headers, js = fetch(base, story)
        assert status == 200 and "javascript" in headers["Content-Type"] and b"duet-story" in js
        status, headers, js = fetch(base, motion)
        assert status == 200 and "javascript" in headers["Content-Type"]
        assert "immutable" in headers["Cache-Control"]
        assert hashlib.sha256(js).hexdigest() == ORIGINAL_SITE_JS_SHA256
        print(f"Motion: chrome.js, the original site.js and the site-owned duet.js served; site.js sha256 {ORIGINAL_SITE_JS_SHA256}")
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

    # With an upstream the same image offers sign-in, without exposing the upstream.
    cid = container(image, "local", "http://127.0.0.1:8790")
    try:
        base = await_ready(cid)
        status, _, body = fetch(base, "/")
        html = body.decode()
        assert status == 200 and 'href="/physician/">Sign in</a>' in html
        assert "127.0.0.1:8790" not in html
        print("Physician upstream configured: Sign in links to /physician/; upstream origin absent from HTML")
        stop(cid)
    finally:
        docker("rm", "-f", cid)

    cid = container(image, "staging", "https://physician.example.test")
    try:
        base = await_ready(cid)
        for path in ["/", "/physician", "/physician/", "/physician/today", chrome, motion, document.styles[0], "/api/health", "/healthz", "/does-not-exist"]:
            status, headers, body = fetch(base, path)
            assert status in (401, 403), (path, status)
            assert "no-store" in headers["Cache-Control"], path
            assert "noindex" in headers["X-Robots-Tag"], path
        for credentials in [
            {"x-goog-authenticated-user-email": "accounts.google.com:engineer@example.test",
             "x-goog-authenticated-user-id": "accounts.google.com:123"},
            {"cf-access-client-id": "id", "cf-access-client-secret": "secret"},
            {"cf-access-jwt-assertion": "legacy"},
            {"x-goog-iap-jwt-assertion": "not-a-signed-jwt"},
            {"x-coda-staging-token": "probe"},
        ]:
            for path in ["/", "/api/health", "/readyz"]:
                status, headers, _ = fetch(base, path, credentials)
                assert status == 403 and "no-store" in headers["Cache-Control"], path
                assert "noindex" in headers["X-Robots-Tag"], path
        print("Staging: missing/invalid IAP, unsigned identity and legacy credentials denied with no-store/noindex")
        print("Signed allow/rotation proof uses Rust public local fixtures; live IAP member/nonmember qualification remains separate")
        stop(cid)
    finally:
        docker("rm", "-f", cid)


if __name__ == "__main__":
    main()
