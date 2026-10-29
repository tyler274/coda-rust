#!/usr/bin/env python3
"""Scoped GitHub read and approved-publication tools for the independent reviewer."""
import argparse
import base64
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import time
from urllib.error import HTTPError, URLError
from urllib.parse import unquote, urlsplit
from urllib.request import Request, urlopen


class GitHubRequestError(RuntimeError):
    """Preserve whether GitHub conclusively refused a request."""

    def __init__(self, message, status=None):
        super().__init__(message)
        self.status = status

    @property
    def definite_refusal(self):
        return self.status is not None and 400 <= self.status < 500 and self.status != 408


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def github(endpoint, method="GET", payload=None, token=None, hostname="github.com"):
    if token:
        if hostname != "github.com":
            raise ValueError("Reviewer App authentication currently supports github.com only")
        data = json.dumps(payload).encode() if payload is not None else None
        request = Request(
            "https://api.github.com/" + endpoint.lstrip("/"), data=data, method=method,
            headers={"Accept": "application/vnd.github+json",
                     "Authorization": f"Bearer {token}",
                     "Content-Type": "application/json",
                     "User-Agent": "coda-pr-reviewer",
                     "X-GitHub-Api-Version": "2022-11-28"},
        )
        try:
            with urlopen(request, timeout=60) as response:
                body = response.read()
        except HTTPError as error:
            raise GitHubRequestError(
                f"GitHub {method} failed for {endpoint.split('?')[0]} with HTTP {error.code}; "
                "inspect access and current remote state before retrying",
                status=error.code,
            ) from None
        except URLError:
            raise GitHubRequestError(
                f"GitHub {method} failed for {endpoint.split('?')[0]}; "
                "inspect network access and current remote state before retrying"
            ) from None
        return json.loads(body) if body.strip() else None
    command = ["gh", "api", "--hostname", hostname, "--method", method, endpoint]
    if payload is not None:
        command += ["--input", "-"]
    result = subprocess.run(command, input=json.dumps(payload) if payload is not None else None,
                            text=True, capture_output=True, timeout=60)
    if result.returncode:
        # Never echo auth commands, environment values, or token responses.
        raise RuntimeError(f"GitHub {method} failed for {endpoint.split('?')[0]}; inspect access and current remote state before retrying")
    return json.loads(result.stdout) if result.stdout.strip() else None


def app_token(config_path, repository):
    config = json.loads(Path(config_path).read_text())
    if not isinstance(config.get("app_id"), int) or not isinstance(config.get("installation_id"), int):
        raise ValueError("Reviewer App config requires integer app_id and installation_id")
    command = config["private_key_command"]
    if not isinstance(command, list) or not command or not all(isinstance(x, str) for x in command):
        raise ValueError("private_key_command must be an argv array")
    key = subprocess.run(command, capture_output=True, timeout=30)
    if key.returncode:
        raise RuntimeError("Could not retrieve reviewer App private key")
    encode = lambda b: base64.urlsafe_b64encode(b).rstrip(b"=")
    header = encode(json.dumps({"alg": "RS256", "typ": "JWT"}).encode())
    claims = encode(json.dumps({"iat": int(time.time()) - 30, "exp": int(time.time()) + 300,
                                "iss": str(config["app_id"])}).encode())
    message = header + b"." + claims
    with tempfile.NamedTemporaryFile() as pem:
        pem.write(key.stdout)
        pem.flush()
        signature = subprocess.run(["openssl", "dgst", "-sha256", "-sign", pem.name],
                                   input=message, capture_output=True, timeout=10)
    if signature.returncode:
        raise RuntimeError("Could not sign reviewer App JWT")
    jwt = (message + b"." + encode(signature.stdout)).decode()
    app = github("app", token=jwt)
    expected_bot = app["slug"] + "[bot]"
    # GitHub restricts this token to the selected installed repository and permissions.
    response = github(f"app/installations/{int(config['installation_id'])}/access_tokens", "POST",
                      {"repositories": [repository.split('/')[1]],
                       "permissions": {"pull_requests": "write", "contents": "read"}}, jwt)
    return response["token"], expected_bot


class ReviewerTools:
    def __init__(self, run_dir):
        self.run_dir = Path(run_dir).resolve()
        self.run = json.loads((self.run_dir / "run.json").read_text())
        self.targets = {t["url"]: t for t in self.run["targets"]}
        self.repositories = {}
        for url in self.targets:
            parsed = urlsplit(url)
            self.repositories["/".join(parsed.path.strip("/").split("/")[:2])] = parsed.hostname

    def target(self, url):
        if url not in self.targets:
            raise ValueError("PR is outside this review's target set")
        parsed = urlsplit(url)
        parts = parsed.path.strip("/").split("/")
        return self.targets[url], parsed.hostname, "/".join(parts[:2]), parts[3]

    def read(self, endpoint, repository):
        if repository not in self.repositories:
            raise ValueError("Repository is outside this review's target set")
        decoded = unquote(endpoint)
        prefix = f"repos/{repository}/"
        if (not endpoint.startswith(prefix) or not decoded.startswith(prefix)
                or any(x in decoded.split('?')[0].split('/') for x in (".", ".."))
                or "\\" in decoded or "#" in decoded):
            raise ValueError(
                f"Endpoint must start exactly with repos/{repository}/ and stay within that target repository"
            )
        value = github(endpoint, hostname=self.repositories[repository])
        self.record_access("github_read", {"repository": repository, "endpoint": endpoint})
        return value

    def file(self, repository, path, ref, start_line=1, end_line=None):
        from urllib.parse import quote
        if start_line < 1 or (end_line is not None and end_line < start_line):
            raise ValueError("Invalid line range")
        value = self.read(f"repos/{repository}/contents/{quote(path, safe='/')}?ref={quote(ref, safe='')}", repository)
        if not isinstance(value, dict) or value.get("encoding") != "base64":
            raise ValueError("Not a readable text file; use git tree/blob endpoints to inspect this object")
        text = base64.b64decode(value["content"]).decode("utf-8")
        lines = text.splitlines()
        stop = min(end_line or len(lines), len(lines))
        self.record_access("read_file", {"repository": repository, "path": path, "ref": ref,
                                         "blob_sha": value["sha"]})
        return {"path": path, "ref": ref, "blob_sha": value["sha"], "total_lines": len(lines),
                "start_line": start_line, "end_line": stop,
                "content": "\n".join(f"{i + 1}: {line}" for i, line in enumerate(lines)
                                      if start_line <= i + 1 <= stop)}

    def record_access(self, tool, evidence):
        with (self.run_dir / "tool-calls.jsonl").open("a") as stream:
            stream.write(json.dumps({"tool": tool, **evidence}) + "\n")

    def prepare(self, pr_url, body, kind="review", comments=None, reply_to=None, event="COMMENT"):
        target, host, repo, number = self.target(pr_url)
        if not body.strip():
            raise ValueError("Publication body must not be empty")
        if kind not in ("review", "reply") or event not in ("COMMENT", "APPROVE", "REQUEST_CHANGES"):
            raise ValueError("Unsupported publication")
        comments = comments or []
        if kind != "review" and (comments or event != "COMMENT"):
            raise ValueError("Only reviews accept inline comments or a review event")
        for comment in comments:
            if (set(comment) != {"path", "line", "side", "body"}
                    or not isinstance(comment["line"], int) or comment["line"] < 1
                    or comment["side"] not in ("LEFT", "RIGHT") or not comment["body"].strip()):
                raise ValueError("Inline comments require path, positive line, LEFT/RIGHT side, and body")
        if kind == "reply":
            if not isinstance(reply_to, int) or reply_to < 1:
                raise ValueError("Replies require a GitHub review comment ID")
            parent = github(f"repos/{repo}/pulls/comments/{reply_to}", hostname=host)
            if parent["pull_request_url"].rstrip('/').split('/')[-1] != number:
                raise ValueError("Reply target belongs to another PR")
            reply_to = parent.get("in_reply_to_id") or parent["id"]
        elif reply_to is not None:
            raise ValueError("reply_to applies only to replies")
        draft = {"pr_url": pr_url, "head_sha": target["head_sha"], "base_sha": target["base_sha"],
                 "kind": kind, "body": body, "comments": comments, "reply_to": reply_to, "event": event}
        draft_id = digest(draft)
        folder = self.run_dir / "publications"
        folder.mkdir(exist_ok=True)
        path = folder / f"{draft_id}.json"
        if not path.exists():
            path.write_text(json.dumps(draft, indent=2) + "\n")
        return {"draft_id": draft_id, "draft_path": str(path), "publication": draft,
                "status": "awaiting_user_approval", "instruction": "Return this exact draft for user approval; do not publish yet."}

    def publish(self, draft_id):
        approval = self.run.get("approved_publication")
        if not approval or approval["draft_id"] != draft_id:
            raise ValueError("No user-approved publication for this draft in this turn")
        draft = json.loads(Path(approval["draft_path"]).read_text())
        if digest(draft) != draft_id:
            raise ValueError("Approved draft changed; obtain approval again")
        target, host, repo, number = self.target(draft["pr_url"])
        if host != "github.com":
            raise ValueError("Reviewer App publisher currently supports github.com only")
        receipt_path = Path(approval["draft_path"]).with_suffix(".receipt.json")
        attempt_path = Path(approval["draft_path"]).with_suffix(".attempt.json")
        if receipt_path.exists():
            receipt = json.loads(receipt_path.read_text())
            self.record_access("publish", receipt)
            return receipt
        if attempt_path.exists():
            raise ValueError("Prior write outcome is uncertain; reconcile on GitHub before another publication")
        current = github(f"repos/{repo}/pulls/{number}", hostname=host)
        if (current["head"]["sha"] != draft["head_sha"] or current["base"]["sha"] != draft["base_sha"]
                or current["state"] != "open"):
            raise ValueError("PR changed since review; resume assessment and approve an updated draft")
        config = self.run.get("github_app_config")
        if not config:
            raise ValueError("Reviewer GitHub App is not configured; no personal-account fallback")
        token, bot = app_token(config, repo)
        if draft["kind"] == "review":
            endpoint = f"repos/{repo}/pulls/{number}/reviews"
            payload = {"body": draft["body"], "commit_id": draft["head_sha"],
                       "event": draft["event"], "comments": draft["comments"]}
        elif draft["kind"] == "reply":
            endpoint = f"repos/{repo}/pulls/{number}/comments/{draft['reply_to']}/replies"
            payload = {"body": draft["body"]}
        attempt_path.write_text(json.dumps({"endpoint": endpoint, "draft_id": draft_id}) + "\n")
        try:
            response = github(endpoint, "POST", payload, token)
        except GitHubRequestError as error:
            if error.definite_refusal:
                attempt_path.unlink()
            raise
        read_endpoint = (f"repos/{repo}/pulls/{number}/reviews/{response['id']}"
                         if draft["kind"] == "review"
                         else f"repos/{repo}/pulls/comments/{response['id']}")
        saved = github(read_endpoint, token=token)
        matches = saved["user"]["login"] == bot and saved["body"] == draft["body"]
        if draft["kind"] == "review":
            expected_state = {"COMMENT": "COMMENTED", "APPROVE": "APPROVED",
                              "REQUEST_CHANGES": "CHANGES_REQUESTED"}[draft["event"]]
            matches = (matches and saved.get("commit_id") == draft["head_sha"]
                       and saved.get("state") == expected_state)
            comments = []
            page = 1
            while True:
                batch = github(
                    f"repos/{repo}/pulls/{number}/reviews/{response['id']}/comments?per_page=100&page={page}",
                    token=token,
                )
                comments.extend(batch)
                if len(batch) < 100:
                    break
                page += 1
            fields = ("path", "line", "side", "body")
            matches = matches and [
                {field: comment.get(field) for field in fields} for comment in comments
            ] == draft["comments"]
        else:
            matches = matches and saved.get("in_reply_to_id") == draft["reply_to"]
        if not matches:
            raise RuntimeError(
                "Published review did not match the approved author, body, revision, event, "
                "inline comments, or reply target; inspect GitHub before proceeding"
            )
        receipt = {"pr_url": draft["pr_url"], "url": saved["html_url"], "author": bot,
                   "reviewed_head_sha": draft["head_sha"], "draft_id": draft_id,
                   "reviewer_session_id": self.run["resume_session_id"]}
        receipt_path.write_text(json.dumps(receipt, indent=2) + "\n")
        self.record_access("publish", receipt)
        return receipt


STRING = {"type": "string"}

def tool(name, description, properties, required, readonly):
    return {"name": name, "description": description,
            "inputSchema": {"type": "object", "properties": properties, "required": required,
                            "additionalProperties": False},
            "annotations": {"readOnlyHint": readonly, "destructiveHint": False,
                            "openWorldHint": True}}


TOOLS = [
    tool("github_read", "Read GitHub REST data directly: PRs, diffs via files, discussion, trees, commits, checks. The endpoint must start exactly with repos/OWNER/REPO/ for the selected target repository. Page lists explicitly using ?page= and per_page=. Only GET is supported.",
         {"repository": STRING, "endpoint": STRING}, ["repository", "endpoint"], True),
    tool("read_file", "Read a full repository file at an exact commit independently of the author. Optional line range for large files. Use github_read git/trees to discover paths.",
         {"repository": STRING, "path": STRING, "ref": STRING, "start_line": {"type": "integer"}, "end_line": {"type": "integer"}},
         ["repository", "path", "ref"], True),
    tool("prepare_publication", "Save your own exact review or follow-up reply for human approval. This does not post to GitHub. Return the draft unchanged to the user.",
         {"pr_url": STRING, "body": STRING, "kind": {"enum": ["review", "reply"]},
          "event": {"enum": ["COMMENT", "APPROVE", "REQUEST_CHANGES"]},
          "comments": {"type": "array", "items": {"type": "object",
              "properties": {"path": STRING, "line": {"type": "integer", "minimum": 1},
                             "side": {"enum": ["LEFT", "RIGHT"]}, "body": STRING},
              "required": ["path", "line", "side", "body"], "additionalProperties": False}},
          "reply_to": {"type": "integer"}}, ["pr_url", "body"], False),
    tool("publish", "Publish your previously approved exact draft as the reviewer bot and read it back. Available approval is bound by launcher to this resumed reviewer and draft. Never posts as a personal user.",
         {"draft_id": STRING}, ["draft_id"], False),
]


def serve(run_dir):
    owner = ReviewerTools(run_dir)
    methods = {"github_read": owner.read, "read_file": owner.file,
               "prepare_publication": owner.prepare, "publish": owner.publish}
    for line in sys.stdin:
        request = json.loads(line)
        if "id" not in request:
            continue
        try:
            method = request["method"]
            if method == "initialize":
                result = {"protocolVersion": request["params"]["protocolVersion"],
                          "capabilities": {"tools": {}}, "serverInfo": {"name": "reviewer", "version": "1.0"}}
            elif method == "ping":
                result = {}
            elif method == "tools/list":
                result = {"tools": TOOLS}
            elif method == "tools/call":
                try:
                    value = methods[request["params"]["name"]](**request["params"].get("arguments", {}))
                    result = {"content": [{"type": "text", "text": json.dumps(value)}]}
                except Exception as error:
                    result = {"isError": True, "content": [{"type": "text", "text": str(error)}]}
            else:
                raise ValueError(f"Unsupported method: {method}")
            response = {"jsonrpc": "2.0", "id": request["id"], "result": result}
        except Exception as error:
            response = {"jsonrpc": "2.0", "id": request["id"], "error": {"code": -32602, "message": str(error)}}
        print(json.dumps(response), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-dir", required=True)
    serve(parser.parse_args().run_dir)
