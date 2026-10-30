"""Exercise proof.py against a scratch repository and a local bare remote."""

import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


PROOF = Path(__file__).with_name("proof.py")


class ProofTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name) / "repo"
        self.root.mkdir()
        self.git("init", "--quiet", "--initial-branch=feature/x")
        self.git("config", "user.email", "agent@codahealth.test")
        self.git("config", "user.name", "Agent")
        (self.root / "code.txt").write_text("broken\n")
        self.git("add", ".")
        self.git("commit", "--quiet", "-m", "base")
        self.base = self.git("rev-parse", "HEAD")
        self.bundle = self.root / "target/verification/proof/feature-x"

    def git(self, *args, cwd=None):
        return subprocess.run(["git", *args], cwd=cwd or self.root, check=True,
                              capture_output=True, text=True).stdout.strip()

    def proof(self, *args, code=0):
        result = subprocess.run([sys.executable, str(PROOF), *args], cwd=self.root,
                                capture_output=True, text=True, env=os.environ)
        self.assertEqual(result.returncode, code, result.stdout + result.stderr)
        return result

    def manifest(self):
        return json.loads((self.bundle / "manifest.json").read_text())

    def test_run_records_output_exit_and_revision(self):
        self.proof("run", "--claim", "AC1", "--label", "says hi", "--", "sh", "-c", "echo hi")
        self.proof("run", "--claim", "AC2", "--", "sh", "-c", "echo no; exit 3", code=3)
        first, second = self.manifest()["entries"]
        self.assertEqual((first["exit"], first["ok"], first["revision"]), (0, True, self.base))
        self.assertEqual((self.bundle / first["files"][0]).read_text(), "hi\n")
        self.assertEqual((second["exit"], second["ok"]), (3, False))
        self.proof("run", "--claim", "AC3", "--", "no-such-command", code=127)

    def test_expect_requires_matching_output(self):
        self.proof("run", "--claim", "AC1", "--expect", r"failed\|5\|5", "--",
                   "sh", "-c", "echo 'failed|4|4'", code=1)
        self.assertFalse(self.manifest()["entries"][0]["expect_matched"])
        self.proof("run", "--claim", "AC1", "--expect", r"failed\|5\|5", "--", "echo", "failed|5|5")

    def test_failure_before_the_fix_runs_new_test_at_base(self):
        (self.root / "code.txt").write_text("fixed\n")
        (self.root / "check.sh").write_text("grep -q fixed code.txt\n")
        self.git("add", ".")
        self.git("commit", "--quiet", "-m", "fix")
        self.proof("run", "--claim", "AC1", "--expect-fail", "--at", self.base, "--with", "check.sh",
                   "--", "sh", "check.sh")
        self.proof("run", "--claim", "AC1", "--", "sh", "check.sh")
        before, after = self.manifest()["entries"]
        self.assertEqual((before["at"], before["ok"], after["ok"]), (self.base, True, True))
        self.assertNotIn("proof-at-", self.git("worktree", "list"))
        self.proof("run", "--claim", "AC1", "--expect-fail", "--", "sh", "check.sh", code=1)

    def test_status_reports_stale_and_gaps(self):
        self.proof("run", "--claim", "AC1", "--", "true")
        self.proof("gap", "--claim", "AC2", "--reason", "needs staging")
        self.proof("status")
        (self.root / "code.txt").write_text("changed\n")
        self.git("commit", "--quiet", "-am", "later")
        status = self.proof("status", code=1).stdout
        self.assertIn("STALE", status)
        self.assertIn("GAP: needs staging", status)

    def test_check_refuses_credentials_and_warns_on_real_email(self):
        clean = Path(self.tmp.name) / "clean.json"
        clean.write_text('{"email": "physician@codahealth.test", "outcome": "accepted"}')
        self.proof("add", "--claim", "AC1", str(clean))
        self.proof("check")
        leaks = {
            "bearer.log": "Authorization: Bearer abcdefghijklmnopqrstuvwxyz012345",
            "session.json": '{"session": "s3cr3t-session-value-123456"}',
            "jwt.txt": "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.signature",
        }
        for name, text in leaks.items():
            with self.subTest(name=name):
                path = Path(self.tmp.name) / name
                path.write_text(text)
                self.proof("add", "--claim", "AC1", str(path))
                self.assertIn("refused", self.proof("check", code=1).stderr)
                (self.bundle / "manifest.json").write_text(json.dumps(
                    {**self.manifest(), "entries": self.manifest()["entries"][:1]}))
        person = Path(self.tmp.name) / "person.txt"
        person.write_text("seen by jane.doe@gmail.com")
        self.proof("add", "--claim", "AC1", str(person))
        self.assertIn("not a synthetic account", self.proof("check").stderr)

    def test_check_refuses_oversized_files(self):
        big = Path(self.tmp.name) / "big.png"
        big.write_bytes(b"\0" * (2 * 1024 * 1024 + 1))
        self.proof("add", "--claim", "AC1", str(big))
        self.assertIn("file limit", self.proof("check", code=1).stderr)

    def test_add_copies_a_run_folder(self):
        run = Path(self.tmp.name) / "physician-scale-stack-1"
        (run / "logs").mkdir(parents=True)
        (run / "stack.json").write_text("{}")
        (run / "logs/site-a.log").write_text("ready\n")
        self.proof("add", "--claim", "AC3", "--label", "scale stack", str(run))
        entry = self.manifest()["entries"][0]
        self.assertEqual(entry["kind"], "run-dir")
        self.assertEqual(entry["files"], ["01-physician-scale-stack-1/logs/site-a.log",
                                          "01-physician-scale-stack-1/stack.json"])

    def test_publish_pushes_bundle_outside_branches_and_render_links_it(self):
        remote = Path(self.tmp.name) / "remote.git"
        self.git("init", "--quiet", "--bare", str(remote))
        self.git("remote", "add", "origin", str(remote))
        shot = Path(self.tmp.name) / "page.png"
        shot.write_bytes(b"\x89PNG")
        self.proof("run", "--claim", "AC1", "--label", "unit tests", "--", "echo", "14 passed")
        self.proof("add", "--claim", "AC2", "--label", "page", str(shot))
        self.proof("gap", "--claim", "AC3", "--reason", "needs staging")
        self.proof("publish", code=1)
        self.proof("publish", "--web-base", "https://github.com/o/r")
        first = self.manifest()["published"]
        head = self.git("rev-parse", "HEAD")
        files = self.git("ls-tree", "-r", "--name-only", first["commit"], cwd=remote).splitlines()
        self.assertIn(f"{head[:10]}/01-unit-tests.log", files)
        self.assertIn(f"{head[:10]}/02-page.png", files)
        self.assertEqual(self.git("for-each-ref", "refs/heads", cwd=remote), "")

        section = self.proof("render").stdout
        base = f"https://github.com/o/r/blob/{first['commit']}/{head[:10]}"
        self.assertIn(f"| AC1 | `unit tests` ([log]({base}/01-unit-tests.log)) | Pass (exit 0) |", section)
        self.assertIn(f"![AC2: page]({base}/02-page.png?raw=true)", section)
        self.assertIn("| AC3 | None | Gap: needs staging |", section)
        self.assertIn("$ echo '14 passed'\n14 passed\n# exit 0", section)

        self.proof("publish", "--web-base", "https://github.com/o/r")
        second = self.manifest()["published"]["commit"]
        self.assertEqual(self.git("rev-parse", f"{second}^", cwd=remote), first["commit"])

    def test_render_marks_stale_entries_and_uses_local_paths_before_publish(self):
        self.proof("run", "--claim", "AC1", "--", "true")
        (self.root / "code.txt").write_text("changed\n")
        section = self.proof("render").stdout
        self.assertIn("with uncommitted changes", section)
        self.assertIn(str(self.bundle / "01-true.log"), section)


if __name__ == "__main__":
    unittest.main()
