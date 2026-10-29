"""Exercise the reviewer MCP boundary without a live GitHub repository."""

import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SERVER = Path(__file__).with_name("reviewer_tools.py")
FAKE_GH = r'''
import base64, json, os, sys
from pathlib import Path

root = Path(os.environ["TEST_ROOT"])
method = sys.argv[sys.argv.index("--method") + 1]
endpoint = sys.argv[sys.argv.index("--method") + 2]
payload = json.load(sys.stdin) if "--input" in sys.argv else None
with (root / "github-calls.jsonl").open("a") as calls:
    calls.write(json.dumps({"method": method, "endpoint": endpoint, "payload": payload,
                            "token": os.environ.get("GH_TOKEN")}) + "\n")
head = os.environ.get("REMOTE_HEAD", "a" * 40)
if endpoint == "repos/example/project/pulls/7" and method == "GET":
    value = {"state": "open", "base": {"sha": "c" * 40}, "head": {"sha": head}}
elif endpoint.startswith("repos/example/project/contents/owner.py"):
    value = {"encoding": "base64", "content": base64.b64encode(b"first\nsecond\n").decode(),
             "sha": "e" * 40}
elif endpoint in ("repos/example/project/pulls/comments/200",
                  "repos/example/project/pulls/comments/201"):
    comment_id = int(endpoint.rsplit("/", 1)[1])
    value = {"id": comment_id,
             "in_reply_to_id": 200 if comment_id == 201 else None,
             "user": {"login": "author" if comment_id == 201 else "coda-pr-reviewer[bot]"},
             "pull_request_url": "https://api.github.com/repos/example/project/pulls/7"}
else:
    print(f"Unexpected fake GitHub call: {method} {endpoint}", file=sys.stderr)
    sys.exit(2)
print(json.dumps(value))
'''

FAKE_HTTP = r'''
import json, os
from pathlib import Path
from urllib.error import HTTPError, URLError
import urllib.request

class Response:
    def __init__(self, value):
        self.value = json.dumps(value).encode()
    def __enter__(self):
        return self
    def __exit__(self, *args):
        return False
    def read(self):
        return self.value

def fake_urlopen(request, timeout=0):
    root = Path(os.environ["TEST_ROOT"])
    endpoint = request.full_url.removeprefix("https://api.github.com/")
    method = request.get_method()
    payload = json.loads(request.data) if request.data else None
    authorization = request.get_header("Authorization")
    with (root / "github-calls.jsonl").open("a") as calls:
        calls.write(json.dumps({"method": method, "endpoint": endpoint, "payload": payload,
                                "app_bearer": bool(authorization and
                                                   authorization.startswith("Bearer "))}) + "\n")
    if endpoint == "app":
        value = {"slug": "coda-pr-reviewer"}
    elif endpoint == "app/installations/2/access_tokens":
        value = {"token": "installation-token"}
    elif endpoint == "repos/example/project/pulls/7" and method == "GET":
        value = {"state": "open", "base": {"sha": "c" * 40},
                 "head": {"sha": os.environ.get("REMOTE_HEAD", "a" * 40)}}
    elif endpoint == "repos/example/project/pulls/7/reviews" and method == "POST":
        failure = os.environ.get("POST_FAILURE")
        if failure == "transport":
            raise URLError("connection reset")
        if failure:
            raise HTTPError(request.full_url, int(failure), "simulated refusal", {}, None)
        (root / "published.json").write_text(json.dumps(payload))
        value = {"id": 99}
    elif endpoint == "repos/example/project/pulls/7/reviews/99":
        published = json.loads((root / "published.json").read_text())
        value = {"id": 99,
                 "html_url": "https://github.com/example/project/pull/7#pullrequestreview-99",
                 "user": {"login": "coda-pr-reviewer[bot]"}, "body": published["body"],
                 "commit_id": published["commit_id"],
                 "state": "APPROVED" if os.environ.get("MISMATCH_STATE") else "COMMENTED"}
    elif endpoint.startswith("repos/example/project/pulls/7/reviews/99/comments?"):
        value = json.loads((root / "published.json").read_text())["comments"]
    elif endpoint == "repos/example/project/pulls/7/comments/200/replies" and method == "POST":
        (root / "published-reply.json").write_text(json.dumps(payload))
        value = {"id": 300}
    elif endpoint == "repos/example/project/pulls/comments/300":
        published = json.loads((root / "published-reply.json").read_text())
        value = {"id": 300,
                 "html_url": "https://github.com/example/project/pull/7#discussion_r300",
                 "user": {"login": "coda-pr-reviewer[bot]"}, "body": published["body"],
                 "in_reply_to_id": 200}
    else:
        raise AssertionError(f"Unexpected fake GitHub HTTP call: {method} {endpoint}")
    return Response(value)

urllib.request.urlopen = fake_urlopen
'''


class ReviewerToolProtocol(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="reviewer-tools-test-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        binary = self.root / "bin"
        binary.mkdir()
        gh = binary / "gh"
        gh.write_text(f"#!{sys.executable}\n" + FAKE_GH)
        gh.chmod(0o755)
        key = binary / "key-helper"
        key.write_text("#!/bin/sh\nprintf 'fake-private-key'")
        key.chmod(0o755)
        openssl = binary / "openssl"
        openssl.write_text("#!/bin/sh\nprintf 'fake-signature'")
        openssl.chmod(0o755)
        (self.root / "sitecustomize.py").write_text(FAKE_HTTP)
        self.config = self.root / "app.json"
        self.config.write_text(json.dumps({"app_id": 1, "installation_id": 2,
                                           "private_key_command": [str(key)]}))
        self.env = dict(os.environ, PATH=str(binary) + os.pathsep + os.environ["PATH"],
                        PYTHONPATH=str(self.root), TEST_ROOT=str(self.root))
        self.env.pop("GH_TOKEN", None)
        self.env.pop("GITHUB_TOKEN", None)
        self.run_dir = self.root / "run"
        self.run_dir.mkdir()
        self.record = {
            "targets": [{"url": "https://github.com/example/project/pull/7",
                         "head_sha": "a" * 40, "base_sha": "c" * 40}],
            "resume_session_id": "same-reviewer-session",
        }
        self.save_record()

    def save_record(self):
        (self.run_dir / "run.json").write_text(json.dumps(self.record))

    def start(self):
        process = subprocess.Popen(
            [sys.executable, str(SERVER), "--run-dir", str(self.run_dir)],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
            text=True, env=self.env,
        )
        self.addCleanup(self.stop, process)
        return process

    def stop(self, process):
        if process.poll() is None:
            process.terminate()
        try:
            process.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            process.kill()
            process.communicate(timeout=5)

    def call(self, process, request_id, method, params=None):
        message = {"jsonrpc": "2.0", "id": request_id, "method": method}
        if params is not None:
            message["params"] = params
        process.stdin.write(json.dumps(message) + "\n")
        process.stdin.flush()
        return json.loads(process.stdout.readline())

    def tool(self, process, request_id, name, arguments):
        response = self.call(process, request_id, "tools/call",
                             {"name": name, "arguments": arguments})
        return response["result"]

    def prepare_draft(self):
        process = self.start()
        initialized = self.call(process, 1, "initialize", {"protocolVersion": "2024-11-05"})
        self.assertEqual(initialized["result"]["serverInfo"]["name"], "reviewer")
        listed = self.call(process, 2, "tools/list")["result"]["tools"]
        self.assertEqual({tool["name"] for tool in listed},
                         {"github_read", "read_file", "prepare_publication", "publish"})
        read_tool = next(tool for tool in listed if tool["name"] == "github_read")
        self.assertIn("start exactly with repos/OWNER/REPO/", read_tool["description"])
        malformed = self.tool(process, 20, "github_read", {
            "repository": "example/project", "endpoint": "pulls/7"})
        self.assertTrue(malformed["isError"])
        self.assertIn("start exactly with repos/example/project/", malformed["content"][0]["text"])
        read = self.tool(process, 3, "github_read", {
            "repository": "example/project", "endpoint": "repos/example/project/pulls/7"})
        self.assertNotIn("isError", read)
        file_result = self.tool(process, 4, "read_file", {
            "repository": "example/project", "path": "owner.py", "ref": "a" * 40})
        file_value = json.loads(file_result["content"][0]["text"])
        self.assertEqual(file_value["content"], "1: first\n2: second")
        prepared = self.tool(process, 5, "prepare_publication", {
            "pr_url": "https://github.com/example/project/pull/7",
            "body": "Reviewer-owned assessment", "kind": "review", "event": "COMMENT"})
        value = json.loads(prepared["content"][0]["text"])
        refused = self.tool(process, 6, "publish", {"draft_id": value["draft_id"]})
        self.assertTrue(refused["isError"])
        self.stop(process)
        return value

    def prepare_reply(self, reply_to=201):
        process = self.start()
        prepared = self.tool(process, 1, "prepare_publication", {
            "pr_url": "https://github.com/example/project/pull/7",
            "body": "Reviewer disposition after the author reply",
            "kind": "reply", "reply_to": reply_to})
        self.stop(process)
        return json.loads(prepared["content"][0]["text"])

    def approve(self, prepared):
        self.record.update(github_app_config=str(self.config), approved_publication={
            "draft_path": prepared["draft_path"], "draft_id": prepared["draft_id"]})
        self.save_record()

    def test_direct_reads_and_exact_approved_publication_round_trip(self):
        prepared = self.prepare_draft()
        self.approve(prepared)
        process = self.start()
        published = self.tool(process, 7, "publish", {"draft_id": prepared["draft_id"]})
        receipt = json.loads(published["content"][0]["text"])
        self.assertEqual(receipt["author"], "coda-pr-reviewer[bot]")
        self.assertEqual(receipt["reviewer_session_id"], "same-reviewer-session")
        repeated = self.tool(process, 8, "publish", {"draft_id": prepared["draft_id"]})
        self.assertEqual(json.loads(repeated["content"][0]["text"]), receipt)
        calls = [json.loads(line) for line in (self.root / "github-calls.jsonl").read_text().splitlines()]
        writes = [call for call in calls if call["method"] == "POST"
                  and call["endpoint"].endswith("/reviews")]
        self.assertEqual(len(writes), 1)
        self.assertTrue(writes[0]["app_bearer"])
        self.assertEqual(writes[0]["payload"]["body"], "Reviewer-owned assessment")
        logged = [json.loads(line) for line in (self.run_dir / "tool-calls.jsonl").read_text().splitlines()]
        self.assertEqual([entry["tool"] for entry in logged],
                         ["github_read", "github_read", "read_file", "publish", "publish"])

    def test_changed_pr_and_uncertain_prior_attempt_refuse_a_write(self):
        prepared = self.prepare_draft()
        self.approve(prepared)
        self.env["REMOTE_HEAD"] = "b" * 40
        process = self.start()
        changed = self.tool(process, 7, "publish", {"draft_id": prepared["draft_id"]})
        self.assertTrue(changed["isError"])
        self.assertIn("PR changed", changed["content"][0]["text"])
        self.stop(process)
        self.env.pop("REMOTE_HEAD")
        Path(prepared["draft_path"]).with_suffix(".attempt.json").write_text("{}")
        process = self.start()
        uncertain = self.tool(process, 8, "publish", {"draft_id": prepared["draft_id"]})
        self.assertTrue(uncertain["isError"])
        self.assertIn("uncertain", uncertain["content"][0]["text"])
        calls = [json.loads(line) for line in (self.root / "github-calls.jsonl").read_text().splitlines()]
        self.assertFalse(any(call["method"] == "POST" and call["endpoint"].endswith("/reviews")
                             for call in calls))

    def test_mismatched_saved_event_is_not_accepted_as_a_receipt(self):
        prepared = self.prepare_draft()
        self.approve(prepared)
        self.env["MISMATCH_STATE"] = "1"
        process = self.start()
        result = self.tool(process, 7, "publish", {"draft_id": prepared["draft_id"]})
        self.assertTrue(result["isError"])
        self.assertIn("did not match", result["content"][0]["text"])
        self.assertFalse(Path(prepared["draft_path"]).with_suffix(".receipt.json").exists())

    def test_reply_to_author_child_is_canonicalized_and_published_on_root_thread(self):
        prepared = self.prepare_reply(reply_to=201)
        draft = json.loads(Path(prepared["draft_path"]).read_text())
        self.assertEqual(draft["reply_to"], 200)
        self.assertEqual(prepared["publication"]["reply_to"], 200)
        self.approve(prepared)
        process = self.start()
        result = self.tool(process, 2, "publish", {"draft_id": prepared["draft_id"]})
        receipt = json.loads(result["content"][0]["text"])
        self.assertEqual(receipt["url"],
                         "https://github.com/example/project/pull/7#discussion_r300")
        calls = [json.loads(line) for line in
                 (self.root / "github-calls.jsonl").read_text().splitlines()]
        write = next(call for call in calls if call["method"] == "POST"
                     and call["endpoint"].endswith("/replies"))
        self.assertEqual(write["endpoint"],
                         "repos/example/project/pulls/7/comments/200/replies")

    def test_definite_http_refusal_clears_attempt_and_allows_corrected_retry(self):
        prepared = self.prepare_draft()
        self.approve(prepared)
        attempt = Path(prepared["draft_path"]).with_suffix(".attempt.json")
        self.env["POST_FAILURE"] = "422"
        process = self.start()
        refused = self.tool(process, 7, "publish", {"draft_id": prepared["draft_id"]})
        self.assertTrue(refused["isError"])
        self.assertIn("HTTP 422", refused["content"][0]["text"])
        self.assertFalse(attempt.exists())
        self.stop(process)
        self.env.pop("POST_FAILURE")
        process = self.start()
        retried = self.tool(process, 8, "publish", {"draft_id": prepared["draft_id"]})
        self.assertNotIn("isError", retried)
        self.assertTrue(Path(prepared["draft_path"]).with_suffix(".receipt.json").exists())

    def test_ambiguous_http_failures_keep_attempt_and_block_retry(self):
        for failure in ("500", "408", "transport"):
            with self.subTest(failure=failure):
                prepared = self.prepare_draft()
                self.approve(prepared)
                attempt = Path(prepared["draft_path"]).with_suffix(".attempt.json")
                attempt.unlink(missing_ok=True)
                self.env["POST_FAILURE"] = failure
                process = self.start()
                failed = self.tool(process, 7, "publish", {"draft_id": prepared["draft_id"]})
                self.assertTrue(failed["isError"])
                self.assertTrue(attempt.exists())
                self.stop(process)
                self.env.pop("POST_FAILURE")
                calls_before = len((self.root / "github-calls.jsonl").read_text().splitlines())
                process = self.start()
                blocked = self.tool(process, 8, "publish", {"draft_id": prepared["draft_id"]})
                self.assertTrue(blocked["isError"])
                self.assertIn("uncertain", blocked["content"][0]["text"])
                calls_after = len((self.root / "github-calls.jsonl").read_text().splitlines())
                self.assertEqual(calls_after, calls_before)
                self.stop(process)


if __name__ == "__main__":
    unittest.main()
