"""Check the scenario runner's receipt rules and its view of the duet-api suites."""

import importlib.util
import json
from pathlib import Path
import re
import tempfile
import unittest

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


if __name__ == "__main__":
    unittest.main()
