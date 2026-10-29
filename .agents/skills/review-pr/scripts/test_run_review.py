"""Exercise the launcher's process/output contract, without calling real models."""

import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


LAUNCHER = Path(__file__).with_name("run_review.py")
FAKE_CLI = r'''
import hashlib, json, os, sys
from pathlib import Path
name = Path(sys.argv[0]).name
root = Path(os.environ["TEST_ROOT"])
if name == "gh":
    counter = root / "reads"
    counter.write_text(str(int(counter.read_text()) + 1 if counter.exists() else 1))
    reads_path = root / "pr-reads.json"
    reads = json.loads(reads_path.read_text()) if reads_path.exists() else {}
    endpoint = sys.argv[-1]
    count = reads[endpoint] = reads.get(endpoint, 0) + 1
    reads_path.write_text(json.dumps(reads))
    _, owner, repository, _, number = endpoint.split("/")
    number = int(number)
    case = os.environ.get("CASE")
    secondary = number == 8
    if ((secondary and (case == "secondary_intake_failure" or (case == "secondary_read_failure" and count > 1)))
            or (not secondary and case == "primary_read_failure" and count > 1)):
        print("PR unavailable", file=sys.stderr)
        sys.exit(1)
    head = os.environ.get("HEAD_SHA", "a" * 40)
    if count > 1 and (case == "stale" or (secondary and case == "secondary_head")):
        head = "b" * 40
    base = "f" * 40 if secondary and count > 1 and case == "secondary_base" else "c" * 40
    merged = secondary and count > 1 and case == "secondary_closed"
    print(json.dumps({"html_url": f"https://github.com/{owner}/{repository}/pull/{number}",
                     "number": number, "title": "Correct ownership", "body": f"Purpose for {owner}/{repository}#{number}",
                     "state": "closed" if merged else "open", "merged": merged,
                     "base": {"ref": "main", "sha": base},
                     "head": {"sha": head}}))
    sys.exit(0)
prompt = sys.stdin.read()
with (root / "calls.jsonl").open("a") as output:
    output.write(json.dumps({"engine": name, "argv": sys.argv[1:], "prompt": prompt,
                             "cwd": os.getcwd()}) + "\n")
case = os.environ.get("CASE", "success")
if case == "exit_failure":
    print("Authentication failed", file=sys.stderr)
    sys.exit(7)
report = "CHANGES_REQUIRED: One contract violation. See owner.py:12."
session = "unexpected-new-session" if case == "wrong_session" else f"test-{name}-session"
if name == "codex":
    run_dir = Path(sys.argv[sys.argv.index("--output-last-message") + 1]).parent
else:
    config = json.loads(sys.argv[sys.argv.index("--mcp-config") + 1])
    run_dir = Path(config["mcpServers"]["reviewer"]["args"][-1])
run = json.loads((run_dir / "run.json").read_text())
if case != "missing_access":
    with (run_dir / "tool-calls.jsonl").open("a") as tools:
        if "PUBLICATION TURN" in prompt:
            draft = json.loads(Path(run["approved_publication"]["draft_path"]).read_text())
            tools.write(json.dumps({"tool": "publish", "pr_url": draft["pr_url"],
                "url": "https://github.com/example/project/pull/7#pullrequestreview-99",
                "author": "coda-pr-reviewer[bot]", "reviewed_head_sha": draft["head_sha"],
                "draft_id": run["approved_publication"]["draft_id"],
                "reviewer_session_id": run["resume_session_id"]}) + "\n")
        else:
            for target in run["targets"]:
                parts = target["url"].split("/")
                repository = "/".join(parts[3:5])
                tools.write(json.dumps({"tool": "github_read", "repository": repository,
                    "endpoint": f"repos/{repository}/pulls/{parts[-1]}"}) + "\n")
                tools.write(json.dumps({"tool": "read_file", "repository": repository,
                    "path": "owner.py", "ref": target["head_sha"], "blob_sha": "e" * 40}) + "\n")
            target = run["targets"][0]
            draft = {"pr_url": target["url"], "head_sha": target["head_sha"],
                "base_sha": target["base_sha"], "kind": "review", "body": report,
                "comments": [], "reply_to": None, "event": "COMMENT"}
            draft_id = hashlib.sha256(json.dumps(draft, sort_keys=True,
                separators=(",", ":")).encode()).hexdigest()
            folder = run_dir / "publications"
            folder.mkdir(exist_ok=True)
            (folder / f"{draft_id}.json").write_text(json.dumps(draft, indent=2) + "\n")
if name == "codex":
    print(json.dumps({"type": "thread.started", "thread_id": session}))
    if case == "native_failure":
        print(json.dumps({"type": "turn.failed", "error": {"message": "Access unavailable"}}))
    else:
        print(json.dumps({"type": "turn.completed", "usage": {}}))
        if case != "missing_report":
            Path(sys.argv[sys.argv.index("--output-last-message") + 1]).write_text(report)
else:
    print(json.dumps({"type": "result", "subtype": "error_during_execution" if case == "native_failure" else "success",
                     "is_error": case == "native_failure", "session_id": session,
                     "result": "" if case == "missing_report" else report}))
'''


class LauncherContract(unittest.TestCase):
    def run_case(self, engine, case="success", extra=(), app_config=False):
        scratch = Path.cwd() / "work"
        scratch.mkdir(exist_ok=True)
        temporary = tempfile.TemporaryDirectory(prefix="review-launcher-test-", dir=scratch)
        self.addCleanup(temporary.cleanup)
        root = Path(temporary.name)
        binary = root / "bin"
        binary.mkdir()
        for name in ("gh", "codex", "claude"):
            program = binary / name
            program.write_text(f"#!{sys.executable}\n" + FAKE_CLI)
            program.chmod(0o755)
        context = root / "context.md"
        context.write_text("User decision: preserve the existing workflow.\nSource: supplied request.")
        config = root / "reviewer-app.json"
        config.write_text(json.dumps({"app_id": 1, "installation_id": 2,
                                      "private_key_command": ["key-helper"]}))
        output = root / "run"
        env = dict(os.environ, PATH=str(binary) + os.pathsep + os.environ["PATH"],
                   TEST_ROOT=str(root), CASE=case)
        result = subprocess.run(
            [sys.executable, str(LAUNCHER), "--pr", "https://github.com/example/project/pull/7",
             "--workspace", str(root), "--engine", engine, "--output-dir", str(output),
             "--context-file", str(context),
             *(["--github-app-config", str(config)] if app_config else []), *extra],
            capture_output=True, text=True, env=env, timeout=20,
        )
        return result, json.loads((output / "run.json").read_text()), root

    def resume_case(self, root, prior="run", output="followup", case="success", extra=(), head=None):
        env = dict(os.environ, PATH=str(root / "bin") + os.pathsep + os.environ["PATH"],
                   TEST_ROOT=str(root), CASE=case, HEAD_SHA=head or "d" * 40)
        result = subprocess.run(
            [sys.executable, str(LAUNCHER), "--resume-from", str(root / prior / "run.json"),
             "--output-dir", str(root / output), *extra],
            capture_output=True, text=True, env=env, timeout=20,
        )
        record = root / output / "run.json"
        return result, json.loads(record.read_text()) if record.exists() else None

    def assert_read_only(self, call):
        args = call["argv"]
        if call["engine"] == "codex":
            self.assertEqual(args[args.index("--sandbox") + 1], "read-only")
            self.assertIn('approval_policy="never"', args)
            mcp = next(value for value in args if value.startswith("mcp_servers="))
            self.assertIn("reviewer_tools.py", mcp)
            self.assertIn("default_tools_approval_mode", mcp)
        else:
            self.assertIn("--strict-mcp-config", args)
            self.assertEqual(args[args.index("--permission-mode") + 1], "dontAsk")
            self.assertEqual(args[args.index("--permission-prompts") + 1], "none")
            self.assertEqual(args[args.index("--tools") + 1], "Read,Glob,Grep")
            config = json.loads(args[args.index("--mcp-config") + 1])
            self.assertIn("reviewer_tools.py", config["mcpServers"]["reviewer"]["args"][0])
            self.assertEqual(
                args[args.index("--allowedTools") + 1].split(","),
                ["mcp__reviewer__github_read", "mcp__reviewer__read_file",
                 "mcp__reviewer__prepare_publication", "mcp__reviewer__publish"],
            )

    def test_fresh_process_receives_pr_and_sourced_context_and_returns_full_review(self):
        for engine in ("codex", "claude"):
            with self.subTest(engine=engine):
                result, record, root = self.run_case(engine)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertEqual(record["status"], "completed")
                calls = [json.loads(line) for line in (root / "calls.jsonl").read_text().splitlines()]
                self.assertEqual(len(calls), 1)
                call = calls[0]
                self.assert_read_only(call)
                self.assertEqual(call["engine"], engine)
                self.assertEqual(call["cwd"], str(root))
                self.assertEqual(call["argv"][0], "exec" if engine == "codex" else "-p")
                for forbidden in ("resume", "fork", "--last", "--continue", "--resume", "--fork-session", "--from-pr"):
                    self.assertNotIn(forbidden, call["argv"])
                self.assertIn("User decision: preserve the existing workflow.", call["prompt"])
                self.assertIn("https://github.com/example/project/pull/7", call["prompt"])
                self.assertIn("a" * 40, call["prompt"])
                self.assertEqual(Path(record["review_path"]).read_text(),
                                 "CHANGES_REQUIRED: One contract violation. See owner.py:12.")
                self.assertEqual(record["session_id"], f"test-{engine}-session")

    def test_followups_resume_the_recorded_reviewer_and_refresh_the_target(self):
        for engine in ("codex", "claude"):
            with self.subTest(engine=engine):
                result, first, root = self.run_case(engine, extra=("--model", "chosen-reviewer", "--effort", "high"))
                self.assertEqual(result.returncode, 0, result.stderr)
                for prior, output in (("run", "followup"), ("followup", "second-followup")):
                    result, record = self.resume_case(root, prior=prior, output=output,
                                                     extra=("--context-file", str(root / "context.md")))
                    self.assertEqual(result.returncode, 0, result.stderr)
                    self.assertEqual(record["status"], "completed")
                    self.assertEqual(record["session_id"], first["session_id"])
                    self.assertEqual(record["resume_session_id"], first["session_id"])
                    self.assertEqual(record["engine"], engine)
                    self.assertEqual(record["model"], "chosen-reviewer")
                    self.assertEqual(record["effort"], "high")
                    self.assertEqual(record["targets"][0]["head_sha"], "d" * 40)
                    self.assertEqual(record["resume_from"], str(root / prior / "run.json"))
                    call = json.loads((root / "calls.jsonl").read_text().splitlines()[-1])
                    flag = "resume" if engine == "codex" else "--resume"
                    self.assertEqual(call["argv"][call["argv"].index(flag) + 1], first["session_id"])
                    self.assertEqual(call["argv"][call["argv"].index("--model") + 1], "chosen-reviewer")
                    self.assertEqual(call["cwd"], str(root))
                    self.assert_read_only(call)
                    for forbidden in ("--last", "--continue", "fork", "--fork-session", "--from-pr"):
                        self.assertNotIn(forbidden, call["argv"])
                    self.assertIn("d" * 40, call["prompt"])
                    self.assertIn("User decision: preserve the existing workflow.", call["prompt"])
                    self.assertTrue(Path(record["review_path"]).read_text())
                self.assertEqual(len((root / "calls.jsonl").read_text().splitlines()), 3)

    def test_invalid_resume_target_never_launches_a_reviewer(self):
        for condition in ("missing_id", "unfinished", "wrong_engine", "wrong_pr"):
            with self.subTest(condition=condition):
                result, first, root = self.run_case("codex")
                self.assertEqual(result.returncode, 0, result.stderr)
                extra = ()
                if condition == "missing_id":
                    first.pop("session_id")
                elif condition == "unfinished":
                    first["status"] = "running"
                elif condition == "wrong_engine":
                    extra = ("--engine", "claude")
                else:
                    extra = ("--pr", "https://github.com/example/project/pull/8")
                (root / "run" / "run.json").write_text(json.dumps(first))
                result, record = self.resume_case(root, extra=extra)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(len((root / "calls.jsonl").read_text().splitlines()), 1)
                self.assertTrue(record is None or record["status"] == "failed")

    def test_resume_failure_or_changed_session_does_not_fall_back_to_a_fresh_review(self):
        for engine in ("codex", "claude"):
            for case in ("exit_failure", "wrong_session"):
                with self.subTest(engine=engine, case=case):
                    result, first, root = self.run_case(engine)
                    self.assertEqual(result.returncode, 0, result.stderr)
                    result, record = self.resume_case(root, case=case)
                    self.assertEqual(result.returncode, 1)
                    self.assertEqual(record["status"], "failed")
                    self.assertTrue(record["error"])
                    self.assertEqual(len((root / "calls.jsonl").read_text().splitlines()), 2)

    def test_explicit_engine_model_and_effort_reach_the_child(self):
        for engine in ("codex", "claude"):
            with self.subTest(engine=engine):
                result, record, root = self.run_case(engine, extra=("--model", "chosen-model", "--effort", "high"))
                self.assertEqual(result.returncode, 0, result.stderr)
                args = json.loads((root / "calls.jsonl").read_text())["argv"]
                self.assertEqual(args[args.index("--model") + 1], "chosen-model")
                if engine == "codex":
                    self.assertIn('model_reasoning_effort="high"', args)
                else:
                    self.assertEqual(args[args.index("--effort") + 1], "high")
                self.assertEqual(record["model"], "chosen-model")

    def test_failed_or_empty_native_results_never_become_completed_reviews(self):
        for engine in ("codex", "claude"):
            for case in ("exit_failure", "native_failure", "missing_report"):
                with self.subTest(engine=engine, case=case):
                    result, record, root = self.run_case(engine, case)
                    self.assertEqual(result.returncode, 1)
                    self.assertEqual(record["status"], "failed")
                    self.assertTrue(record["error"])
                    self.assertEqual((root / "reads").read_text(), "1")

    def test_changed_remote_revision_keeps_report_but_marks_it_stale(self):
        result, record, root = self.run_case("codex", "stale")
        self.assertEqual(result.returncode, 3)
        self.assertEqual(record["status"], "stale")
        self.assertEqual(record["targets"][0]["head_sha"], "a" * 40)
        self.assertEqual(record["latest_targets"][0]["head_sha"], "b" * 40)
        self.assertTrue(Path(record["review_path"]).read_text())

    def test_batch_uses_one_reviewer_and_resumes_all_targets_across_repositories(self):
        urls = ["https://github.com/example/project/pull/7", "https://github.com/example/consumer/pull/7"]
        for engine in ("codex", "claude"):
            with self.subTest(engine=engine):
                result, first, root = self.run_case(engine, extra=("--pr", urls[1]))
                self.assertEqual(result.returncode, 0, result.stderr)
                calls = (root / "calls.jsonl").read_text().splitlines()
                self.assertEqual(len(calls), 1)
                prompt = json.loads(calls[0])["prompt"]
                for url in urls:
                    self.assertTrue(url in prompt, f"Reviewer did not receive target {url}")
                self.assertEqual([target["url"] for target in first["targets"]], urls)
                self.assertEqual([target["url"] for target in first["latest_targets"]], urls)
                self.assert_read_only(json.loads(calls[0]))
                result, resumed = self.resume_case(root)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertEqual(resumed["session_id"], first["session_id"])
                self.assertEqual([target["url"] for target in resumed["targets"]], urls)
                self.assertEqual([target["head_sha"] for target in resumed["targets"]], ["d" * 40] * 2)
                self.assertEqual(resumed["previous_targets"], first["targets"])
                self.assertEqual(resumed["changed_targets"], [])
                calls = (root / "calls.jsonl").read_text().splitlines()
                self.assertEqual(len(calls), 2)
                call = json.loads(calls[-1])
                self.assert_read_only(call)
                flag = "resume" if engine == "codex" else "--resume"
                self.assertEqual(call["argv"][call["argv"].index(flag) + 1], first["session_id"])
                self.assertEqual(set(json.loads((root / "pr-reads.json").read_text()).values()), {4})

    def test_secondary_batch_target_can_prevent_a_clean_result(self):
        second = "https://github.com/example/project/pull/8"
        cases = [("secondary_head", "stale", 3), ("secondary_base", "stale", 3),
                 ("secondary_closed", "stale", 3), ("secondary_read_failure", "freshness_unverified", 3),
                 ("primary_read_failure", "freshness_unverified", 3),
                 ("secondary_intake_failure", "failed", 1)]
        for case, status, code in cases:
            with self.subTest(case=case):
                result, record, root = self.run_case("codex", case, extra=("--pr", second))
                self.assertEqual(result.returncode, code, result.stderr)
                self.assertEqual(record["status"], status)
                if status == "failed":
                    self.assertFalse((root / "calls.jsonl").exists())
                    self.assertIn(second, record["error"])
                else:
                    self.assertEqual(record["latest_targets"][0]["head_sha"], "a" * 40)
                    self.assertEqual(len((root / "calls.jsonl").read_text().splitlines()), 1)
                    self.assertTrue(Path(record["review_path"]).read_text())
                    if status == "stale":
                        self.assertEqual(record["changed_targets"], [second])
                    else:
                        first = "https://github.com/example/project/pull/7"
                        missing, available = (first, second) if case == "primary_read_failure" else (second, first)
                        self.assertIn(missing, record["freshness_errors"])
                        self.assertEqual([target["url"] for target in record["latest_targets"]], [available])

    def test_batch_resume_does_not_silently_drop_a_target(self):
        result, first, root = self.run_case("codex", extra=("--pr", "https://github.com/example/project/pull/8"))
        self.assertEqual(result.returncode, 0, result.stderr)
        result, record = self.resume_case(root, extra=("--pr", "https://github.com/example/project/pull/7"))
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(record["status"], "failed")
        self.assertEqual(len((root / "calls.jsonl").read_text().splitlines()), 1)

    def test_legacy_single_pr_run_can_still_resume_its_reviewer(self):
        result, first, root = self.run_case("codex")
        self.assertEqual(result.returncode, 0, result.stderr)
        first["target"] = first.pop("targets")[0]
        (root / "run" / "run.json").write_text(json.dumps(first))
        result, resumed = self.resume_case(root)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(resumed["session_id"], first["session_id"])
        self.assertEqual([target["url"] for target in resumed["targets"]], [first["target"]["url"]])

    def test_dry_run_prepares_request_without_invoking_a_model(self):
        result, record, root = self.run_case("claude", extra=("--dry-run",))
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(record["status"], "prepared")
        self.assertTrue((root / "run" / "request.md").is_file())
        self.assertFalse((root / "calls.jsonl").exists())

    def test_missing_direct_reviewer_access_cannot_be_reported_as_completed(self):
        result, record, _ = self.run_case("codex", "missing_access")
        self.assertEqual(result.returncode, 3)
        self.assertEqual(record["status"], "access_unverified")
        self.assertEqual(record["missing_direct_access"],
                         ["https://github.com/example/project/pull/7"])

    def test_approved_exact_draft_is_published_by_the_same_reviewer_session(self):
        for engine in ("codex", "claude"):
            with self.subTest(engine=engine):
                result, first, root = self.run_case(engine, app_config=True)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertEqual(len(first["publication_drafts"]), 1)
                draft = first["publication_drafts"][0]
                result, published = self.resume_case(
                    root, extra=("--approve-publication", draft), head="a" * 40)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertEqual(published["session_id"], first["session_id"])
                self.assertEqual(published["resume_session_id"], first["session_id"])
                self.assertEqual(len(published["github_publications"]), 1)
                receipt = published["github_publications"][0]
                self.assertEqual(receipt["author"], "coda-pr-reviewer[bot]")
                self.assertEqual(receipt["reviewer_session_id"], first["session_id"])
                call = json.loads((root / "calls.jsonl").read_text().splitlines()[-1])
                self.assertIn("PUBLICATION TURN", call["prompt"])
                self.assertIn(receipt["draft_id"], call["prompt"])

    def test_changed_draft_is_refused_before_the_reviewer_runs(self):
        result, first, root = self.run_case("codex", app_config=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        draft = Path(first["publication_drafts"][0])
        value = json.loads(draft.read_text())
        value["body"] = "Changed after approval"
        draft.write_text(json.dumps(value))
        result, record = self.resume_case(
            root, extra=("--approve-publication", str(draft)))
        self.assertNotEqual(result.returncode, 0)
        self.assertIsNone(record)
        self.assertEqual(len((root / "calls.jsonl").read_text().splitlines()), 1)

    def test_resume_dry_run_does_not_send_a_followup(self):
        result, first, root = self.run_case("claude")
        self.assertEqual(result.returncode, 0, result.stderr)
        result, record = self.resume_case(root, extra=("--dry-run",))
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(record["status"], "prepared")
        self.assertEqual(record["resume_session_id"], first["session_id"])
        self.assertEqual(len((root / "calls.jsonl").read_text().splitlines()), 1)


if __name__ == "__main__":
    unittest.main()
