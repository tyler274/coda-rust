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
sys.path.insert(0, str(SCRIPT.parent))
import ci_scope  # noqa: E402


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
        self.assertEqual([line.split("=")[0] for line in lines],
                         ["full", "deps", "mobile", "sandbox"])
        self.last_deps = lines[1]
        self.last_mobile = lines[2]
        self.last_sandbox = lines[3]
        return lines[0]

    def test_sandbox_changes_select_only_the_sandbox_job(self):
        cases = {
            # path: (full, deps, sandbox)
            "crates/workspace-tools/sandbox/src/main.rs": (False, False, True),
            "crates/workspace-tools/sandbox/Cargo.toml": (False, False, True),
            "crates/workspace-tools/sandbox/flake.nix": (False, False, True),
            "crates/workspace-tools/sandbox/flake.lock": (False, False, True),
            "crates/workspace-tools/sandbox/config/cyrene.toml": (False, False, True),
            ".github/workflows/ci.yml": (True, False, True),
            # The sandbox flake builds from the root manifest and lockfile.
            "Cargo.lock": (True, True, True),
            "Cargo.toml": (True, True, True),
            "crates/workspace-tools/src/main.rs": (True, False, False),
            "crates/app/Cargo.toml": (True, True, False),
            "book/src/page.md": (False, False, False),
        }
        for path, (full, deps, sandbox) in cases.items():
            with self.subTest(path=path):
                self.git("reset", "--hard", self.base)
                self.write(path, "changed\n")
                self.commit()
                self.assertEqual(self.scope(), f"full={str(full).lower()}")
                self.assertEqual(self.last_deps, f"deps={str(deps).lower()}")
                self.assertEqual(self.last_sandbox, f"sandbox={str(sandbox).lower()}")
        # A sandbox change beside product code still selects both.
        self.git("reset", "--hard", self.base)
        self.write("crates/workspace-tools/sandbox/Cargo.toml", "changed\n")
        self.write("crates/app/src/lib.rs", "changed\n")
        self.commit()
        self.assertEqual(self.scope(), "full=true")
        self.assertEqual((self.last_deps, self.last_sandbox), ("deps=false", "sandbox=true"))
        for event in ("workflow_dispatch", "schedule", "unknown"):
            with self.subTest(event=event):
                self.scope(event, event={})
                self.assertEqual(self.last_sandbox, "sandbox=true")

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
            "rust-toolchain.toml": True,
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

    def test_android_build_changes_select_the_emulator(self):
        cases = {
            "crates/ui/src/lib.rs": True, "crates/mobile/src/main.rs": True,
            "crates/android/src/main.rs": True, "crates/duet-client/src/lib.rs": True,
            "platform/android/ModeShellTest.kt": True, "scripts/wire-duet-android.py": True,
            "nix/android-shell.nix": True, "flake.lock": True, "Cargo.lock": True,
            ".github/workflows/ci.yml": True, ".cargo/config.toml": True, "Dioxus.toml": True,
            "platform/ios/build_bridge.rs": True,
            "crates/duet-api/src/lib.rs": False, "crates/physician-sample/src/main.rs": False,
            "crates/uix/src/lib.rs": False, "platform/ios/Info.plist": False,
            "nix/packages.nix": False, "skills/clinical/SKILL.md": False,
            "book/src/release.md": False,
        }
        for path, mobile in cases.items():
            with self.subTest(path=path):
                self.assertEqual(ci_scope.mobile_path(path), mobile)

    def test_emulator_is_manual_only_while_change_selection_is_off(self):
        self.assertFalse(ci_scope.MOBILE_ON_CHANGE)
        self.write("crates/mobile/src/main.rs", "changed\n")
        self.commit()
        for event in ("pull_request", "push"):
            with self.subTest(event=event):
                self.assertEqual(self.scope(event), "full=true")
                self.assertEqual(self.last_mobile, "mobile=false")
        for event, mobile in [("workflow_dispatch", True), ("schedule", False), ("unknown", False)]:
            with self.subTest(event=event):
                self.scope(event, event={})
                self.assertEqual(self.last_mobile, f"mobile={str(mobile).lower()}")

    def test_change_selection_runs_the_emulator_only_for_the_apk_build(self):
        self.write("crates/mobile/src/main.rs", "changed\n")
        head = self.commit()
        event = {"pull_request": {"base": {"sha": self.base}, "head": {"sha": head}}}
        cwd = os.getcwd()
        os.chdir(self.repo)
        try:
            self.assertTrue(ci_scope.classify("pull_request", event)[2])
            self.write("crates/duet-api/src/lib.rs", "changed\n")
            other = self.commit()
            event["pull_request"] = {"base": {"sha": head}, "head": {"sha": other}}
            self.assertFalse(ci_scope.classify("pull_request", event)[2])
        finally:
            os.chdir(cwd)

    def test_mobile_crate_list_check_reads_cargo_tree(self):
        root = SCRIPT.resolve().parents[1]
        def check(tree):
            return subprocess.run([sys.executable, str(SCRIPT), "check-mobile-crates"],
                                  input=tree, capture_output=True, text=True)
        listed = f"ui v0.1.0 ({root}/crates/ui)\nmobile v0.1.0 ({root}/crates/mobile) (*)\nserde v1.0.0\n"
        self.assertEqual(check(listed).returncode, 0)
        unlisted = check(listed + f"duet-new v0.1.0 ({root}/crates/duet-new)\n")
        self.assertNotEqual(unlisted.returncode, 0)
        self.assertIn("duet-new", unlisted.stderr)

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
            ("workflow_dispatch", {}), ("schedule", {}), ("unknown", {}),
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

    def needs(self, full, mobile=None, sandbox=None):
        mobile = full if mobile is None else mobile
        sandbox = full if sandbox is None else sandbox
        result = {job: {"result": "success" if full else "skipped"} for job in self.dependencies}
        result["android-emulator"] = {"result": "success" if mobile else "skipped"}
        result["sandbox"] = {"result": "success" if sandbox else "skipped"}
        result["scope"] = {"result": "success",
                           "outputs": {"full": str(full).lower(), "mobile": str(mobile).lower(),
                                       "sandbox": str(sandbox).lower()}}
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
        for output in ("full", "mobile", "sandbox"):
            for flag in (None, "", "unknown"):
                needs = self.needs(False)
                needs["scope"]["outputs"][output] = flag
                self.check(needs, False)

    def test_emulator_result_follows_mobile_not_full(self):
        self.check(self.needs(True, mobile=False), True)
        ran = self.needs(True, mobile=False)
        ran["android-emulator"]["result"] = "failure"
        self.check(ran, False)
        for state in ("failure", "cancelled", "skipped"):
            with self.subTest(state=state):
                needs = self.needs(True, mobile=True)
                needs["android-emulator"]["result"] = state
                self.check(needs, False)

    def test_sandbox_result_follows_its_own_scope(self):
        self.check(self.needs(False, sandbox=True), True)
        self.check(self.needs(True, sandbox=False), True)
        for full in (False, True):
            for state in ("failure", "cancelled", "skipped"):
                with self.subTest(full=full, state=state):
                    needs = self.needs(full, sandbox=True)
                    needs["sandbox"]["result"] = state
                    self.check(needs, False)
            ran = self.needs(full, sandbox=False)
            ran["sandbox"]["result"] = "success"
            self.check(ran, False)

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


class WarmRunTests(unittest.TestCase):
    def setUp(self):
        self.text = WORKFLOW.read_text()
        self.workflow = yaml.safe_load(self.text)
        self.jobs = self.workflow["jobs"]

    def test_weekly_schedule_and_manual_dispatch_both_trigger(self):
        triggers = self.workflow[True]  # PyYAML reads the bare `on:` key as True.
        self.assertIn("workflow_dispatch", triggers)
        self.assertEqual(len(triggers["schedule"]), 1)
        self.assertEqual(len(triggers["schedule"][0]["cron"].split()[4]), 1)

    def test_scheduled_runs_skip_browsers_and_deployments(self):
        browser = [s for s in self.jobs["ci"]["steps"] if "browser qualification" in s.get("name", "")]
        self.assertEqual(len(browser), 1)
        self.assertEqual(browser[0]["if"], "github.event_name == 'workflow_dispatch'")
        for name, job in self.jobs.items():
            if name.startswith("deploy-") or name == "nix-cache-publish":
                self.assertIn("github.event_name == 'push'", job["if"], name)

    def test_validation_cannot_dispatch_other_workflows(self):
        # GitHub's dispatch API requires Actions write. Validation executes
        # proposed source and dependencies; its token must not approve releases.
        for name, job in self.jobs.items():
            if name.startswith("deploy-") or name == "nix-cache-publish":
                continue
            with self.subTest(job=name):
                permissions = job.get("permissions", self.workflow.get("permissions", {}))
                self.assertNotEqual(permissions, "write-all")
                if isinstance(permissions, dict):
                    self.assertIn(permissions.get("actions"), (None, "read"))

    def test_postgres_saves_per_commit_and_restores_by_prefix(self):
        steps = {s.get("name"): s for s in self.jobs["postgres"]["steps"]}
        restore = steps["Restore Postgres proof build"]
        save = steps["Save Postgres proof build"]
        self.assertTrue(restore["uses"].startswith("actions/cache/restore@"))
        self.assertTrue(save["uses"].startswith("actions/cache/save@"))
        self.assertTrue(restore["with"]["key"].endswith("-${{ github.sha }}"))
        self.assertEqual(save["with"]["key"], "${{ steps.postgres-cache.outputs.cache-primary-key }}")
        self.assertEqual(save["with"]["path"], restore["with"]["path"])
        prefixes = restore["with"]["restore-keys"].strip().splitlines()
        # Each fallback is broader: same dependency set, same shard, any shard.
        for narrow, broad in zip([restore["with"]["key"], *prefixes], prefixes):
            self.assertTrue(narrow.startswith(broad), broad)
            self.assertNotEqual(narrow, broad)


if __name__ == "__main__":
    unittest.main()
