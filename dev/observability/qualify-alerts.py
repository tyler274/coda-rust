#!/usr/bin/env python3
"""Offline Stage4 rehearsal and evidence check. No network or live execution path."""

import argparse
from datetime import datetime
from decimal import Decimal
import json
from pathlib import Path
import re

FIXTURE = Path(__file__).with_name("qualification.json")
ROUTES = ("datadog", "cloud-monitoring", "grafana-test")
REQUIRED_TARGET_FIELDS = (
    "evaluator_id", "signal_id", "query_and_thresholds", "evaluation_timing",
    "notification_policy", "slack_account", "contact_id", "channel_id", "configuration_evidence",
)
STEPS = {
    "datadog": [
        "Read back the exact Slack account/channel mapping for #engineering-observability; verify contact, recovery and bounded reminder settings.",
        "After explicit approval, send one labeled provider routing test; ask a person to confirm its Slack message and timestamp. This is not evaluator proof.",
        "Use only the approved dedicated synthetic signal/target selected by the copied readiness evaluator, never a product service or its metric series.",
        "Establish healthy baseline. At recorded sample times supply three consecutive minute-spaced failures, then hold failure within the approved observation window. Read actual evaluator history through pending (if exposed), firing and notification metadata; do not infer transitions from the submitted values.",
        "Verify repeated samples retain one stable group, at most two 15-minute reminders, and no Grafana service-page duplicate. Stop at 40 minutes and restore healthy samples even on failure.",
        "Supply at least two consecutive minute-spaced successes and continue until the evaluator's actual recovery window clears. Capture evaluator recovery and independently confirmed Slack recovery.",
        "With separately approved time-bounded maintenance on only the synthetic group, repeat failure and recovery. Capture evaluation history and absence of Slack notifications over the full mute/delivery window. Test active failure across maintenance expiry; confirm normal alerting resumes. Remove the synthetic mute and restore baseline.",
    ],
    "cloud-monitoring": [
        "Read back the Stage2-owned policy, heartbeat descriptor, outside-GKE producer and verified Slack notification channel. Require a separate route independent of Datadog, Grafana and the watched collector.",
        "After explicit approval, establish a fresh dedicated synthetic heartbeat through the outside-cluster path, then stop only that heartbeat. Simulate loss of the in-cluster path by withholding only its dedicated test signal, not stopping a real component.",
        "Observe the actual managed absence evaluator after its five-minute threshold plus configured alignment/evaluation delay. Record last heartbeat, pending if exposed, incident open, provider notification metadata and human-confirmed Slack receipt.",
        "Resume the heartbeat outside GKE; capture incident closure and Slack recovery. Verify the configured notification cap and maintenance behavior separately; absence of the shared collector must not prevent this notification.",
    ],
    "grafana-test": [
        "Keep service rules in shadow/no-notification mode. Read back the separately labeled qualification-test rule and contact route, never the operational service route.",
        "After separate approval, drive its dedicated synthetic datasource series from healthy to failing through the actual Grafana rule evaluator; capture Normal, Pending and Alerting with exact times.",
        "Confirm the labeled test Slack receipt, then restore the series and capture Normal and Slack recovery. A contact-point test or local webhook receipt alone does not qualify the rule or Slack.",
    ],
}


def instant(value):
    """UTC, microsecond precision or better retained as a Decimal, never floats."""
    match = re.fullmatch(r"(\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d)(\.\d{1,9})?Z", value)
    if not match:
        raise ValueError("timestamps must be exact UTC RFC3339 (up to nanoseconds)")
    whole = datetime.fromisoformat(match[1] + "+00:00")
    epoch = datetime(1970, 1, 1, tzinfo=whole.tzinfo)
    delta = whole - epoch
    return Decimal(delta.days * 86400 + delta.seconds) + Decimal(match[2] or "0")


def rehearsal(contract):
    if contract["environment"] != "staging":
        raise ValueError("only staging qualification is in scope")
    if contract["primary_channel"] != "#engineering-observability":
        raise ValueError("primary destination must be #engineering-observability")
    gaps = []
    for route in ROUTES:
        target = contract["targets"][route]
        for field in REQUIRED_TARGET_FIELDS:
            if not target.get(field):
                gaps.append(f"{route}: supply verified {field}; do not guess")
    return {
        "mode": "offline-rehearsal", "live_qualified": False,
        "live_gate": "BLOCKED: explicit operator approval and verified targets required; this tool cannot send or apply anything",
        "configuration_gaps": gaps,
        "policy_to_review": {
            "group_by": ["environment", "synthetic_target", "symptom"],
            "reminder_interval_seconds": 900, "maximum_reminders": 2,
            "maximum_exercise_seconds": 2400,
            "recovery": "enabled, same group and route; never equate missing telemetry with recovery",
            "maintenance": "time-bounded synthetic-group mute; no initial/reminder/recovery sends during mute; explicitly verify expiry and cleanup",
        },
        "steps": STEPS,
        "required_approval": [
            "Review exact source/deployed revisions, dedicated signal, evaluator query/windows, verified contact/account/channel and separate Grafana test route.",
            "Authorize labeled routing test and bounded synthetic failure/recovery, reminders and maintenance window; name operator, UTC start/end and abort/cleanup steps.",
            "Authorize independent heartbeat-loss simulation and recovery separately. No real service failure, deployment, apply or IAM change is included.",
        ],
    }


def assess(events):
    """Check an exported observation ledger, not provider truth or Slack authenticity.

    Preserve original records and timestamps. Provider metadata is useful evidence
    but is never promoted to a human receipt. Even complete input remains unverified.
    """
    allowed = {"signal_failure", "pending", "firing", "provider_notified",
               "slack_alert", "reminder", "signal_restored", "recovered",
               "slack_recovery", "maintenance_start", "maintenance_end"}
    if not events:
        raise ValueError("empty observation ledger")
    results = {}
    for route in ROUTES:
        records = [event for event in events if event["route"] == route]
        if not records:
            continue
        times = [instant(event["at"]) for event in records]
        if times != sorted(times):
            raise ValueError(f"{route}: observations must be chronological")
        gaps = []
        kinds = {}
        groups = set()
        maintenance = False
        reminder_times = []
        for event, at in zip(records, times):
            kind = event["kind"]
            if kind not in allowed:
                raise ValueError(f"unknown observation kind: {kind}")
            groups.add(event["group"])
            kinds.setdefault(kind, []).append(event)
            if not event.get("evidence"):
                gaps.append(f"{kind}: missing inspectable evidence reference")
            if kind == "maintenance_start":
                if maintenance:
                    gaps.append("overlapping maintenance")
                maintenance = True
            elif kind == "maintenance_end":
                if not maintenance:
                    gaps.append("maintenance end without start")
                maintenance = False
            elif kind in {"slack_alert", "slack_recovery", "reminder"}:
                if maintenance:
                    gaps.append("notification received during maintenance")
                if event.get("source") != "human-confirmed-slack" or not event.get("confirmed_by") or not event.get("message_ref"):
                    gaps.append(f"{kind}: needs human-confirmed Slack receipt, confirmer and message reference")
                if kind == "reminder":
                    reminder_times.append(at)
            elif kind == "provider_notified" and event.get("source") != "provider-metadata":
                gaps.append("provider notification requires provider-metadata source")
            elif kind in {"pending", "firing", "recovered"} and event.get("source") != "evaluator-history":
                gaps.append(f"{kind}: requires actual evaluator history, not inferred sample state")
        if maintenance:
            gaps.append("maintenance not ended")
        if len(groups) > 1:
            gaps.append("group changed within exercise; possible duplicate incident")
        required = ("signal_failure", "firing", "slack_alert", "signal_restored", "recovered", "slack_recovery")
        for kind in required:
            if len(kinds.get(kind, [])) != 1:
                gaps.append(f"{kind}: require exactly one observation per exercise")
        if kinds.get("recovered") and any(at >= instant(kinds["recovered"][0]["at"]) for at in reminder_times):
            gaps.append("reminder received at or after evaluator recovery")
        if len(reminder_times) > 2:
            gaps.append("more than two reminders")
        if kinds.get("slack_alert"):
            preceding = instant(kinds["slack_alert"][0]["at"])
            for at in reminder_times:
                if at - preceding < 900:
                    gaps.append("reminder earlier than 15 minutes")
                preceding = at
        latencies = {}
        if all(len(kinds.get(kind, [])) == 1 for kind in required):
            ordered = [instant(kinds[kind][0]["at"]) for kind in required]
            if ordered != sorted(ordered):
                gaps.append("failure, firing, receipt, restore, recovery, receipt order violated")
            if any(not ordered[0] <= instant(event["at"]) <= ordered[1] for event in kinds.get("pending", [])):
                gaps.append("pending must occur between signal failure and firing")
            if ordered[-1] - ordered[0] > 2400:
                gaps.append("exercise exceeded 40-minute bound")
            for name, start, end in (
                ("failure_to_firing_seconds", 0, 1),
                ("failure_to_slack_seconds", 0, 2),
                ("firing_to_slack_seconds", 1, 2),
                ("restore_to_recovery_seconds", 3, 4),
                ("recovery_to_slack_seconds", 4, 5),
            ):
                latencies[name] = str(ordered[end] - ordered[start])
        results[route] = {"gaps": gaps, "reported_latencies": latencies,
                          "records": records}
    if any(event["route"] not in ROUTES for event in events):
        raise ValueError("unknown observation route")
    return {"mode": "offline-evidence-check", "live_qualified": False,
            "limitation": "Input assertions only. Inspect source artifacts and confirm Slack independently; maintenance, routing test, shadow isolation and watchdog independence need separate live proof.",
            "unassessed_routes": [route for route in ROUTES if route not in results],
            "routes": results}


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--contract", type=Path, default=FIXTURE)
    parser.add_argument("--observations", type=Path, help="local JSON array of observation records")
    parser.add_argument("--live", action="store_true", help="always refused: no live implementation")
    args = parser.parse_args(argv)
    if args.live:
        parser.error("live execution is blocked; explicit operator approval is required outside this offline tool")
    try:
        result = rehearsal(json.loads(args.contract.read_text()))
        if args.observations:
            result["evidence_check"] = assess(json.loads(args.observations.read_text()))
        print(json.dumps(result, indent=2))
        if args.observations:
            return int(any(route["gaps"] for route in result["evidence_check"]["routes"].values()))
        return 0
    except (ValueError, KeyError, TypeError, OSError) as error:
        parser.error(str(error))


if __name__ == "__main__":
    raise SystemExit(main())
