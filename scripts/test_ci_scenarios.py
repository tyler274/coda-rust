"""Check the scenario runner's receipt rules and its view of the duet-api suites."""

import importlib.util
import json
from pathlib import Path
import re
import tempfile
import unittest
from unittest import mock

REPO = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("scenarios", REPO / "scripts/scenarios.py")
scenarios = importlib.util.module_from_spec(spec)
spec.loader.exec_module(scenarios)


def receipt(steps, gaps):
    with tempfile.TemporaryDirectory() as directory:
        path = Path(directory) / "receipt.json"
        written = scenarios.Receipt(path)
        for passed in steps:
            written.step("P1", "step", "actor", passed, "observed")
        for _ in range(gaps):
            written.gap("P1", "capability", "why", "owner")
        written.write()
        return json.loads(path.read_text())


class ReceiptOutcome(unittest.TestCase):
    def test_missing_capability_is_incomplete_not_complete(self):
        self.assertEqual(receipt([True], 1)["outcome"], "incomplete")

    def test_failed_step_outranks_missing(self):
        data = receipt([True, False], 1)
        self.assertEqual((data["outcome"], data["counts"]["failed"]), ("failed", 1))

    def test_no_steps_is_not_a_pass(self):
        self.assertEqual(receipt([], 0)["outcome"], "failed")

    def test_complete_only_with_steps_and_no_gaps(self):
        data = receipt([True], 0)
        self.assertEqual(data["outcome"], "complete")
        self.assertEqual(data["counts"]["real_model_steps"], 0)
        self.assertEqual(data["steps"][0]["evidence"], "controlled")


class DuetSuites(unittest.TestCase):
    def test_runner_names_exactly_the_tests_in_its_suites(self):
        found = set()
        for suite in scenarios.D1_SUITES:
            source = (REPO / f"crates/duet-api/tests/{suite}.rs").read_text()
            found |= set(re.findall(r"#\[tokio::test[^\]]*\]\s*(?:#\[[^\]]*\]\s*)*async fn (\w+)", source))
        self.assertEqual(found, set(scenarios.D1_TESTS))


class RunnerFailures(unittest.TestCase):
    def test_nonzero_cargo_exit_fails_even_when_every_test_reports_ok(self):
        stdout = "".join(f"test {name} ... ok\n" for name in scenarios.D1_TESTS)
        completed = scenarios.subprocess.CompletedProcess([], 101, stdout, "")
        with tempfile.TemporaryDirectory() as directory, \
                mock.patch.object(scenarios.subprocess, "run", return_value=completed):
            written = scenarios.Receipt(Path(directory) / "receipt.json")
            scenarios.d1_suites(written, {"postgres": {"url": "postgres://x"}}, Path(directory) / "d1")
        self.assertEqual([s["step"] for s in written.steps if not s["passed"]], ["cargo test exit"])

    def test_readiness_file_mid_write_is_retried(self):
        process = mock.Mock(**{"poll.return_value": None})
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "stack" / "stack.json"
            path.parent.mkdir()
            path.write_text('{"ready_at"')
            reads = iter(['{"ready_at"', json.dumps({"ready_at": "now"})])
            with mock.patch.object(Path, "read_text", lambda self: next(reads)), \
                    mock.patch.object(scenarios.time, "sleep"):
                self.assertEqual(scenarios.wait_ready(process, Path(directory)), {"ready_at": "now"})

    def test_empty_selection_is_a_usage_error(self):
        with mock.patch.object(scenarios.sys, "argv", ["scenarios.py", "--only", ""]), \
                self.assertRaises(SystemExit) as raised:
            scenarios.main()
        self.assertEqual(raised.exception.code, 2)


if __name__ == "__main__":
    unittest.main()
