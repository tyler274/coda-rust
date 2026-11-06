"""Exercise CI selection using real Git histories and the runner-facing CLI."""

import copy
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

import yaml


SCRIPT = Path(__file__).with_name("ci_scope.py")
WORKFLOW = SCRIPT.parents[1] / ".github/workflows/ci.yml"


class ScopeTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.repo = self.root / "repo"
        self.repo.mkdir()
        self.git("init", "-q")
        self.git("config", "user.name", "CI test")
        self.git("config", "user.email", "ci-test@example.invalid")
        self.write("README.md", "initial\n")
        self.write("crates/app/src/lib.rs", "pub fn initial() {}\n")
        self.base = self.commit()

    def git(self, *args):
        return subprocess.check_output(["git", *args], cwd=self.repo, stderr=subprocess.PIPE).decode().strip()

    def write(self, path, text):
        file = self.repo / path
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_text(text)

    def commit(self):
        self.git("add", "-A")
        # Fixtures use Git objects directly, independent of developer hooks.
        tree = self.git("write-tree")
        parent = subprocess.run(["git", "rev-parse", "--verify", "HEAD"], cwd=self.repo,
                                capture_output=True, text=True)
        args = ["commit-tree", tree, "-m", "fixture"]
        if parent.returncode == 0:
            args += ["-p", parent.stdout.strip()]
        sha = self.git(*args)
        self.git("update-ref", "HEAD", sha)
        return sha

    def scope(self, name="pull_request", base=None, head=None, event=None):
        if event is None:
            base, head = base or self.base, head or self.git("rev-parse", "HEAD")
            event = ({"pull_request": {"base": {"sha": base}, "head": {"sha": head}}}
                     if name == "pull_request" else {"before": base, "after": head})
        payload = self.root / "event.json"
        output = self.root / "output"
        payload.write_text(json.dumps(event))
        output.write_text("")
        env = dict(os.environ, GITHUB_EVENT_NAME=name, GITHUB_EVENT_PATH=str(payload),
                   GITHUB_OUTPUT=str(output), GITHUB_STEP_SUMMARY=str(self.root / "summary"))
        result = subprocess.run([sys.executable, str(SCRIPT), "select"], cwd=self.repo,
                                env=env, capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        lines = output.read_text().splitlines()
        self.assertEqual([line.split("=")[0] for line in lines], ["full", "deps"])
        self.last_deps = lines[1]
        return lines[0]

    def test_scope_matches_approved_categories_not_file_size(self):
        cases = {
            "README.md": False, "AGENTS.md": False, "CLAUDE.md": False,
            "book/src/long-document.md": False, "book/book.toml": False,
            "intent/physician.md": False, "intent/nested/decision.md": False,
            "verification/README.md": False,
            "verification/physician/sign-in/README.md": False,
            "infra/gcp/staging/README.md": False,
            ".agents/skills/review-pr/scripts/run_review.py": False,
            ".claude/skills/review-pr": False,
            "skills/clinical/SKILL.md": True, "crates/app/src/lib.rs": True,
            "Cargo.lock": True, "flake.nix": True, "infra/gcp/main.tf": True,
            ".github/workflows/ci.yml": True,
            "verification/replay.mjs": True, "infra/gcp/staging/notes.md": True,
            "intent/runtime.json": True, ".claude/launch.json": True,
            "scripts/check.sh": True, "new-area/README.md": True,
        }
        for path, full in cases.items():
            with self.subTest(path=path):
                self.git("reset", "--hard", self.base)
                self.write(path, "changed\n")
                self.commit()
                self.assertEqual(self.scope(), f"full={str(full).lower()}")

    def test_dependency_changes_report_deps(self):
        cases = {
            "Cargo.lock": True, "Cargo.toml": True, "crates/app/Cargo.toml": True,
            "flake.lock": True, "flake.nix": True, "nix/packages.nix": True,
            "crates/app/src/lib.rs": False, "scripts/check.sh": False,
            "book/src/page.md": False, "crates/app/Cargo.toml.md": False,
        }
        for path, deps in cases.items():
            with self.subTest(path=path):
                self.git("reset", "--hard", self.base)
                self.write(path, "changed\n")
                self.commit()
                self.scope()
                self.assertEqual(self.last_deps, f"deps={str(deps).lower()}")
        # An uncertain comparison runs everything but does not guess cold.
        self.scope("workflow_dispatch", event={})
        self.assertEqual(self.last_deps, "deps=false")

    def test_complete_pr_and_push_include_earlier_code_change(self):
        self.write("crates/app/src/lib.rs", "pub fn changed() {}\n")
        self.commit()
        self.write("README.md", "last commit only changes docs\n")
        self.commit()
        for event in ("pull_request", "push"):
            with self.subTest(event=event):
                self.assertEqual(self.scope(event), "full=true")

    def test_pr_uses_merge_base_not_unrelated_base_branch_changes(self):
        self.git("checkout", "-qb", "base-advance")
        self.write("crates/app/src/other.rs", "pub fn other() {}\n")
        newer_base = self.commit()
        self.git("checkout", "-qb", "docs-change", self.base)
        self.write("README.md", "docs change\n")
        self.commit()
        self.assertEqual(self.scope(base=newer_base), "full=false")

    def test_renames_and_deletions_cover_both_paths(self):
        cases = [("crates/app/src/lib.rs", "book/moved.md", True),
                 ("README.md", "crates/moved.rs", True),
                 ("README.md", "book/moved.md", False),
                 ("README.md", None, False),
                 ("crates/app/src/lib.rs", None, True)]
        for old, new, full in cases:
            with self.subTest(old=old, new=new):
                self.git("reset", "--hard", self.base)
                if new:
                    (self.repo / new).parent.mkdir(parents=True, exist_ok=True)
                    self.git("mv", old, new)
                else:
                    self.git("rm", old)
                self.commit()
                self.assertEqual(self.scope(), f"full={str(full).lower()}")

    def test_many_files_and_unusual_names_do_not_truncate_comparison(self):
        for index in range(350):
            self.write(f"book/page-{index}.md", "doc\n")
        self.write("book/name with\nnewline.md", "doc\n")
        self.commit()
        self.assertEqual(self.scope(), "full=false")
        self.write("z-product/source.rs", "code\n")
        self.commit()
        self.assertEqual(self.scope(), "full=true")

    def test_uncertain_comparison_and_manual_dispatch_select_full(self):
        self.write("README.md", "docs change\n")
        head = self.commit()
        for event, payload in [
            ("push", {"before": "0" * 40, "after": head}),
            ("push", {"before": "1" * 40, "after": head}),
            ("pull_request", {}), ("pull_request", None),
            ("workflow_dispatch", {}), ("unknown", {}),
        ]:
            with self.subTest(event=event, payload=payload):
                # Explicit null input is different from the helper's default.
                if payload is None:
                    payload = {"pull_request": None}
                self.assertEqual(self.scope(event, event=payload), "full=true")
        self.assertEqual(self.scope("push", base=head, head=head), "full=true")


class ResultTests(unittest.TestCase):
    def setUp(self):
        self.jobs = yaml.safe_load(WORKFLOW.read_text())["jobs"]
        self.dependencies = self.jobs["result"]["needs"]

    def needs(self, full):
        result = {job: {"result": "success" if full else "skipped"} for job in self.dependencies}
        result["scope"] = {"result": "success", "outputs": {"full": str(full).lower()}}
        result["lightweight"] = {"result": "success"}
        return result

    def check(self, needs, success):
        result = subprocess.run([sys.executable, str(SCRIPT), "result"],
                                env=dict(os.environ, CI_NEEDS=json.dumps(needs)),
                                capture_output=True, text=True)
        self.assertEqual(result.returncode == 0, success, result.stdout + result.stderr)

    def test_selected_checks_must_pass_even_when_actions_skips_dependents(self):
        for full in (False, True):
            needs = self.needs(full)
            self.check(needs, True)
            selected = self.dependencies if full else ("scope", "lightweight")
            for job in selected:
                for state in ("failure", "cancelled", "skipped"):
                    with self.subTest(full=full, job=job, state=state):
                        changed = copy.deepcopy(needs)
                        changed[job]["result"] = state
                        self.check(changed, False)
                missing = copy.deepcopy(needs)
                del missing[job]
                self.check(missing, False)
        for flag in (None, "", "unknown"):
            needs = self.needs(False)
            needs["scope"]["outputs"]["full"] = flag
            self.check(needs, False)

    def test_gate_covers_every_validation_job_and_deployment_consumes_gate(self):
        # This is a workflow topology contract, not a copy of the job inventory.
        # The cache publisher follows images and must not delay the gate.
        validators = {name for name, job in self.jobs.items()
                      if name not in {"result", "nix-cache-publish"} and not name.startswith("deploy-")}
        self.assertEqual(set(self.dependencies), validators)
        self.assertEqual(self.jobs["result"]["if"], "always()")
        for name, job in self.jobs.items():
            if name.startswith("deploy-"):
                self.assertIn("result", job["needs"], name)


if __name__ == "__main__":
    unittest.main()
