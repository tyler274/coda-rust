"""Recorded API-shaped fixtures; no GitHub/network access."""

from copy import deepcopy
import io
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch
from contextlib import redirect_stderr, redirect_stdout

import ci_timings as timings


RUN = {
    "id": 42, "run_attempt": 1, "display_title": "A <safe> change",
    "head_sha": "abc123", "event": "pull_request", "status": "completed",
    "conclusion": "success", "run_started_at": "2026-11-04T10:00:00Z",
}


def job(name="ci", start="2026-11-04T10:00:10Z", end="2026-11-04T10:02:00Z", **fields):
    value = {"name": name, "status": "completed", "conclusion": "success",
             "started_at": start, "completed_at": end, "steps": []}
    value.update(fields)
    return value


JOBS = [
    job("Documentation and engineering skills", end="2026-11-04T10:01:00Z"),
    job(steps=[job("Build", end="2026-11-04T10:01:30Z")]),
    job("CI result", start="2026-11-04T10:02:01Z", end="2026-11-04T10:02:05Z"),
    job("Deploy staging", start="2026-11-04T10:02:06Z", end="2026-11-04T10:08:00Z"),
]


class FixtureAPI:
    def __init__(self, responses):
        self.responses = responses
        self.calls = []

    def __call__(self, endpoint):
        self.calls.append(endpoint)
        if endpoint not in self.responses:
            raise AssertionError(f"Unexpected API request: {endpoint}")
        return deepcopy(self.responses[endpoint])


class CollectorTests(unittest.TestCase):
    def test_only_gh_api_and_propagates_failure(self):
        with patch("ci_timings.subprocess.run") as run:
            run.return_value.stdout = '{"ok": true}'
            self.assertEqual(timings.gh_api("repos/o/r/actions"), {"ok": True})
            run.assert_called_once_with(["gh", "api", "repos/o/r/actions"], check=True,
                                        capture_output=True, text=True)
            run.side_effect = subprocess.CalledProcessError(1, "gh", stderr="denied")
            with self.assertRaises(subprocess.CalledProcessError):
                timings.gh_api("repos/o/r/actions")

    def test_paginated_jobs_and_explicit_attempt(self):
        base = "repos/o/r/actions/runs/42/attempts/2"
        api = FixtureAPI({base: dict(RUN, run_attempt=2),
                          base + "/jobs?per_page=100&page=1": {"jobs": [job()] * 100, "total_count": 101},
                          base + "/jobs?per_page=100&page=2": {"jobs": [job("CI result")], "total_count": 101}})
        result = timings.Collector("o/r", api).run(42, 2)
        self.assertEqual(result.attempt, 2)
        self.assertEqual(len(result.validation), 101)
        self.assertTrue(result.url.endswith("/42/attempts/2"))
        self.assertEqual(len(api.calls), 3)

    def test_latest_attempt_pins_metadata_and_jobs(self):
        base = "repos/o/r/actions/runs/42"
        api = FixtureAPI({base: {"run_attempt": 3}, base + "/attempts/3": dict(RUN, run_attempt=3),
                          base + "/attempts/3/jobs?per_page=100&page=1": {"jobs": JOBS}})
        self.assertEqual(timings.Collector("o/r", api).run(42).attempt, 3)
        self.assertEqual(len(api.calls), 3)

    def test_history_fetches_every_attempt(self):
        root = "repos/o/r/actions"
        responses = {root + "/workflows/ci.yml/runs?per_page=1&page=1": {
            "workflow_runs": [dict(RUN, run_attempt=2)], "total_count": 8}}
        for attempt in (1, 2):
            base = f"{root}/runs/42/attempts/{attempt}"
            responses[base] = dict(RUN, run_attempt=attempt,
                                   conclusion="failure" if attempt == 1 else "success")
            responses[base + "/jobs?per_page=100&page=1"] = {"jobs": JOBS}
        results = timings.Collector("o/r", FixtureAPI(responses)).history(1)
        self.assertEqual([r.attempt for r in results], [1, 2])
        self.assertEqual([r.baseline for r in results], [False, False])
        self.assertIn("rerun", results[1].cohort)

    def test_pagination_keeps_page_size_constant_for_nonmultiple_limit(self):
        api = FixtureAPI({"items?per_page=100&page=1": {"items": list(range(100)), "total_count": 180},
                          "items?per_page=100&page=2": {"items": list(range(100, 180)), "total_count": 180}})
        self.assertEqual(timings.Collector("o/r", api).pages("items", "items", 101), list(range(101)))

    def test_empty_page_without_total_ends_pagination(self):
        api = FixtureAPI({"items?per_page=100&page=1": {"items": []}})
        self.assertEqual(timings.Collector("o/r", api).pages("items", "items"), [])

    def test_invalid_repo_and_limits(self):
        for repo in ("o/r/extra", "../x/y", "o r/repo"):
            with self.assertRaises(ValueError):
                timings.Collector(repo)
        collector = timings.Collector("o/r", FixtureAPI({}))
        with self.assertRaises(ValueError):
            collector.history(0)
        with self.assertRaises(ValueError):
            collector.run(42, 0)


class AnalysisTests(unittest.TestCase):
    def test_validation_does_not_include_deployment_or_sum_parallel_jobs(self):
        result = timings.analyze("o/r", RUN, JOBS)
        self.assertEqual(result.elapsed, 125)
        self.assertEqual(len(result.validation), 3)
        self.assertEqual(len(result.deployments), 1)
        self.assertEqual(result.steps[0].seconds, 80)
        self.assertTrue(result.baseline)

    def test_running_result_is_explicitly_omitted_including_steps(self):
        jobs = deepcopy(JOBS)
        jobs[2] = job("CI result", status="in_progress", conclusion=None, end=None,
                      steps=[job("Collect report")])
        result = timings.analyze("o/r", dict(RUN, status="in_progress", conclusion=None), jobs)
        self.assertEqual(result.elapsed, 120)
        self.assertEqual(result.omitted, ["CI result: in_progress"])
        self.assertEqual(len(result.steps), 1)
        self.assertFalse(result.complete)
        self.assertFalse(result.baseline)
        self.assertIn("partial snapshot", timings.markdown([result]))

    def test_light_cancelled_manual_failed_and_unknown_separate(self):
        cases = [
            (RUN, [job(timings.LIGHT_JOB), job("ci", conclusion="skipped")], "light"),
            (dict(RUN, conclusion="cancelled"), JOBS, "full"),
            (dict(RUN, conclusion="failure"), JOBS, "full"),
            (dict(RUN, event="workflow_dispatch"), JOBS, "full"),
            (RUN, [job("Something else")], "unknown"),
            (dict(RUN, conclusion="failure"),
             [job("Fresh Postgres migrations and tests (shard 2)")], "full"),
        ]
        cohorts = set()
        for run, jobs, scope in cases:
            with self.subTest(run=run, scope=scope):
                result = timings.analyze("o/r", run, jobs)
                self.assertEqual(result.scope, scope)
                self.assertFalse(result.baseline)
                cohorts.add(result.cohort)
        self.assertEqual(len(cohorts), 5)

    def test_invalid_missing_skipped_times_are_not_zero(self):
        for value in (None, "bad", "0001-01-01T00:00:00Z", "2026-11-04T10:00:00"):
            self.assertIsNone(timings.timestamp(value))
        for item in (job(end=None), job(end="2026-11-03T00:00:00Z"),
                     job(conclusion="skipped"), job(status="queued")):
            self.assertIsNone(timings.interval(item))
        result = timings.analyze("o/r", RUN, [job(end=None)])
        self.assertIsNone(result.elapsed)
        self.assertFalse(result.baseline)
        self.assertIn("missing/invalid timestamps", result.omitted[0])

    def test_deployment_steps_are_separate(self):
        result = timings.analyze("o/r", RUN, JOBS + [job("Deploy production", steps=[job("Publish")])])
        self.assertEqual(len(result.steps), 1)
        self.assertEqual(result.deployment_steps[0].label, "Deploy production / Publish")
        self.assertIn("Deployment steps (separate)", timings.render_html([result], "o/r"))

    def test_missing_attempt_start_prevents_baseline(self):
        result = timings.analyze("o/r", dict(RUN, run_started_at=None), JOBS)
        self.assertIsNone(result.elapsed)
        self.assertFalse(result.baseline)
        self.assertIn("Validation jobs", timings.render_html([result], "o/r"))

    def test_slow_cache_setup_is_flagged(self):
        steps = [job("Read private Nix cache", end="2026-11-04T10:00:25Z"),
                 job("Remove private cache credentials", end="2026-11-04T10:00:11Z"),
                 job("Build", end="2026-11-04T10:01:30Z")]
        result = timings.analyze("o/r", RUN, [job("Fresh Postgres migrations and tests", steps=steps)])
        report = timings.markdown([result])
        self.assertIn("Warning: slow cache setup", report)
        self.assertIn("Fresh Postgres migrations and tests / Read private Nix cache 15s", report)
        self.assertNotIn("Remove private cache credentials", report)
        self.assertNotIn("Warning", timings.markdown([timings.analyze("o/r", RUN, JOBS)]))
        hostile = timings.analyze("o/r", RUN, [job("a`b\nc", steps=steps[:1])])
        self.assertIn("`a'b c / Read private Nix cache 15s`", timings.markdown([hostile]))

    def test_skipped_steps_reported_and_zero_duration_valid(self):
        result = timings.analyze("o/r", RUN, [job(steps=[job("skip", conclusion="skipped"),
            job("instant", start="2026-11-04T10:00:10Z", end="2026-11-04T10:00:10Z")])])
        self.assertEqual(result.steps[0].seconds, 0)
        self.assertIn("skip: skipped", result.omitted[0])


class OutputTests(unittest.TestCase):
    def test_self_contained_escaped_html_and_attempt_links(self):
        run = timings.analyze("o/r", RUN, JOBS + [job('<script>alert("x")</script>')])
        html = timings.render_html([run], "o/r")
        self.assertIn("<svg", html)
        self.assertIn("&lt;script&gt;", html)
        self.assertIn("A &lt;safe&gt; change", html)
        self.assertNotIn("<script", html)
        self.assertNotIn('src="https://', html)
        self.assertIn(run.url, html)
        self.assertIn("Deployment jobs (separate)", html)
        self.assertIn(run.url, timings.markdown([run]))

    def test_empty_history(self):
        self.assertIn("baseline n=0", timings.render_html([], "o/r"))
        self.assertIn("Collected 0 attempt(s)", timings.markdown([]))

    def test_cli_writes_report_and_appends_step_summary(self):
        base = "repos/o/r/actions/runs/42/attempts/1"
        api = FixtureAPI({base: RUN, base + "/jobs?per_page=100&page=1": {"jobs": JOBS}})
        with tempfile.TemporaryDirectory() as directory:
            output, summary = Path(directory) / "nested/report.html", Path(directory) / "step.md"
            summary.write_text("Existing summary\n")
            with patch.dict(os.environ, {"GITHUB_REPOSITORY": "o/r", "GITHUB_RUN_ID": "42",
                                         "GITHUB_RUN_ATTEMPT": "1", "GITHUB_STEP_SUMMARY": str(summary)}), redirect_stdout(io.StringIO()):
                self.assertEqual(timings.main(["run", "--output", str(output), "--summary", str(summary)], api), 0)
            self.assertIn("<svg", output.read_text())
            raw = json.loads(output.with_suffix('.json').read_text())
            self.assertEqual(raw[base], RUN)
            self.assertEqual(raw[base + '/jobs?per_page=100&page=1']['jobs'], JOBS)
            self.assertTrue(summary.read_text().startswith("Existing summary\n"))
            self.assertIn("https://github.com/o/r/actions/runs/42/attempts/1", summary.read_text())

    def test_cli_history_default_summary_path(self):
        api = FixtureAPI({"repos/o/r/actions/workflows/ci.yml/runs?per_page=2&page=1": {"workflow_runs": []}})
        with tempfile.TemporaryDirectory() as directory, redirect_stdout(io.StringIO()):
            output = Path(directory) / "history.html"
            self.assertEqual(timings.main(["history", "--repo", "o/r", "--limit", "2", "--output", str(output)], api), 0)
            self.assertTrue(output.with_suffix(".md").exists())

    def test_cli_failure_returns_nonzero_without_fake_report(self):
        def fail(_):
            raise subprocess.CalledProcessError(1, "gh", stderr="permission denied")
        with tempfile.TemporaryDirectory() as directory, redirect_stderr(io.StringIO()) as stderr:
            output = Path(directory) / "report.html"
            self.assertEqual(timings.main(["history", "--output", str(output)], fail), 1)
            self.assertFalse(output.exists())
            self.assertIn("permission denied", stderr.getvalue())

    def test_cli_rejects_same_output_and_summary(self):
        with redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            timings.main(["history", "--output", "same.md", "--summary", "same.md"], FixtureAPI({}))


if __name__ == "__main__":
    unittest.main()
