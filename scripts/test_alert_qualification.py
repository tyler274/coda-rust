#!/usr/bin/env python3
"""Offline notification rehearsal contracts; no provider calls or credentials."""

import contextlib
import copy
import importlib.util
import io
import json
from pathlib import Path
import threading
import unittest
from unittest.mock import patch
import urllib.request

ROOT = Path(__file__).resolve().parents[1]


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, ROOT / path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


qualification = load("qualification", "dev/observability/qualify-alerts.py")
sink = load("sink", "dev/observability/grafana/webhook-sink.py")


def observations(route="datadog"):
    # Invented local test data, never a claim of actual provider/Slack observation.
    fields = [
        ("signal_failure", "00:00.123456789", "synthetic-fixture"),
        ("pending", "01:00.123456789", "evaluator-history"),
        ("firing", "03:00.123456789", "evaluator-history"),
        ("provider_notified", "03:01.123456789", "provider-metadata"),
        ("slack_alert", "03:02.123456790", "human-confirmed-slack"),
        ("signal_restored", "04:00.123456789", "synthetic-fixture"),
        ("recovered", "06:00.123456789", "evaluator-history"),
        ("slack_recovery", "06:02.123456789", "human-confirmed-slack"),
    ]
    return [dict(route=route, kind=kind, at=f"2026-01-01T00:{time}Z",
                 source=source, group="staging/qualification-test/unavailable",
                 evidence="fixture-only.json", confirmed_by="fixture-operator",
                 message_ref="fixture-message-not-real") for kind, time, source in fields]


def gaps(events, route="datadog"):
    return qualification.assess(events)["routes"][route]["gaps"]


class QualificationTests(unittest.TestCase):
    def test_default_is_offline_with_actionable_unconfigured_targets(self):
        with patch("socket.socket", side_effect=AssertionError("network forbidden")):
            output = io.StringIO()
            with contextlib.redirect_stdout(output):
                self.assertEqual(qualification.main([]), 0)
            result = json.loads(output.getvalue())
        self.assertFalse(result["live_qualified"])
        self.assertEqual(len(result["configuration_gaps"]), 27)
        self.assertEqual(result["policy_to_review"]["maximum_reminders"], 2)
        self.assertIn("three consecutive", " ".join(result["steps"]["datadog"]))
        self.assertIn("outside-cluster", " ".join(result["steps"]["cloud-monitoring"]))
        self.assertIn("shadow", " ".join(result["steps"]["grafana-test"]))

    def test_live_refused_before_reading_files_or_using_network(self):
        with patch.object(Path, "read_text", side_effect=AssertionError("must gate first")), patch("socket.socket", side_effect=AssertionError("network forbidden")):
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                qualification.main(["--live"])
            self.assertEqual(error.exception.code, 2)

    def test_primary_scope_is_fixed(self):
        contract = json.loads(qualification.FIXTURE.read_text())
        for field, value in (("environment", "production"), ("primary_channel", "#elsewhere")):
            changed = copy.deepcopy(contract)
            changed[field] = value
            with self.assertRaises(ValueError):
                qualification.rehearsal(changed)

    def test_exact_timestamps_and_latency_not_rounded(self):
        events = observations()
        result = qualification.assess(events)
        route = result["routes"]["datadog"]
        self.assertEqual(route["gaps"], [])
        self.assertEqual(route["records"], events)
        self.assertEqual(route["reported_latencies"]["failure_to_slack_seconds"], "182.000000001")
        self.assertFalse(result["live_qualified"])
        self.assertTrue(result["routes"]["cloud-monitoring"]["gaps"])

    def test_provider_metadata_never_substitutes_for_slack(self):
        events = [event for event in observations() if event["kind"] not in {"slack_alert", "slack_recovery"}]
        self.assertIn("slack_alert: require exactly one observation per exercise", gaps(events))
        events = observations()
        events[4]["source"] = "provider-metadata"
        self.assertTrue(any("human-confirmed" in gap for gap in gaps(events)))
        events[4]["source"] = "local-webhook-sink"
        self.assertTrue(any("human-confirmed" in gap for gap in gaps(events)))

    def test_evaluator_states_cannot_be_inferred_from_samples(self):
        events = observations()
        events[2]["source"] = "synthetic-fixture"
        self.assertTrue(any("actual evaluator history" in gap for gap in gaps(events)))

    def test_duplicate_group_or_receipt_and_missing_evidence(self):
        events = observations()
        events[-1]["group"] = "other-group"
        events[-1]["evidence"] = ""
        events.append(copy.deepcopy(events[-1]))
        problems = gaps(events)
        self.assertTrue(any("group changed" in gap for gap in problems))
        self.assertTrue(any("missing inspectable" in gap for gap in problems))
        self.assertIn("slack_recovery: require exactly one observation per exercise", problems)

    def test_clock_order_and_utc_required(self):
        events = observations()
        events.reverse()
        with self.assertRaisesRegex(ValueError, "chronological"):
            qualification.assess(events)
        with self.assertRaisesRegex(ValueError, "UTC"):
            qualification.instant("2026-01-01T00:00:00")
        events = observations()
        events[4]["kind"], events[5]["kind"] = events[5]["kind"], events[4]["kind"]
        self.assertTrue(any("order violated" in gap for gap in gaps(events)))

    def test_reminder_interval_and_count(self):
        events = observations()
        for minute in (7, 8, 9):
            event = dict(events[4], kind="reminder", at=f"2026-01-01T00:0{minute}:00Z")
            events.append(event)
        self.assertIn("more than two reminders", gaps(events))
        self.assertIn("reminder earlier than 15 minutes", gaps(events))
        self.assertIn("reminder received at or after evaluator recovery", gaps(events))

    def test_maintenance_receipt_and_missing_cleanup_fail(self):
        events = observations()
        events.insert(0, dict(events[0], kind="maintenance_start", at="2026-01-01T00:00:00Z"))
        problems = gaps(events)
        self.assertIn("notification received during maintenance", problems)
        self.assertIn("maintenance not ended", problems)

    def test_exercise_bound(self):
        events = observations()
        events[-1]["at"] = "2026-01-01T01:00:00Z"
        self.assertIn("exercise exceeded 40-minute bound", gaps(events))

    def test_unknown_route_rejected(self):
        with self.assertRaisesRegex(ValueError, "unknown observation route"):
            qualification.assess(observations("unknown"))

    def test_sink_records_local_receipt_not_slack(self):
        sink.RECEIVED.clear()
        server = sink.http.server.ThreadingHTTPServer(("127.0.0.1", 0), sink.Sink)
        thread = threading.Thread(target=server.serve_forever)
        thread.start()
        try:
            url = f"http://127.0.0.1:{server.server_port}"
            request = urllib.request.Request(url + "/qualification-test", b'{"text":"synthetic"}', {"Content-Type": "application/json"})
            with contextlib.redirect_stdout(io.StringIO()):
                with urllib.request.urlopen(request, timeout=3) as response:
                    self.assertEqual(response.read(), b"ok")
            with urllib.request.urlopen(url + "/received", timeout=3) as response:
                receipt = json.load(response)[0]
            self.assertEqual(receipt["payload"], {"text": "synthetic"})
            self.assertEqual(receipt["path"], "/qualification-test")
            self.assertEqual(receipt["source"], "local-webhook-sink")
            self.assertFalse(receipt["slack_receipt_confirmed"])
            qualification.instant(receipt["received_at"])
        finally:
            server.shutdown()
            server.server_close()
            thread.join()


if __name__ == "__main__":
    unittest.main()
