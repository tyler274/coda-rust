#!/usr/bin/env python3
"""Collect GitHub Actions attempt timings and render offline HTML/SVG reports.

Only gh api accesses GitHub. Import Collector with an injected api callable for
recorded data. History's limit counts workflow runs; every attempt is included.
"""

import argparse
from collections import defaultdict
from dataclasses import dataclass
from datetime import datetime, timezone
from html import escape
import json
import os
from pathlib import Path
import re
import statistics
import subprocess
import sys
from urllib.parse import quote


FULL_JOBS = {
    "ci", "cargo-deny", "Offline cloud foundation contracts",
    "Locked Nix images (build, run, migrate; no push)",
    "Fresh Postgres migrations and tests",
    "Collector config and observability OpenTofu",
}
LIGHT_JOB = "Documentation and engineering skills"
# Cache setup and cleanup take about a second; 10s or more means a stall.
SETUP_STEPS = {"Read private Nix cache", "Remove private cache credentials"}
SLOW_SETUP_SECONDS = 10
METHOD = (
    "Validation elapsed = attempt start to last completed validation job; "
    "deployment jobs are excluded. Job/step durations are execution time, not "
    "queue time, and parallel jobs must not be summed as wall time. Incomplete, "
    "skipped and missing/invalid timestamps are omitted, never counted as zero. "
    "An in-progress CI result job is explicitly omitted (including its steps). "
    "Baseline uses only completed, successful full push/pull_request first attempts "
    "with complete validation timings. Manual, light, cancelled, failed and "
    "incomplete attempts remain separate. Reruns are separate samples, not "
    "independent revisions; a partial rerun contains only that attempt's jobs. "
    "Scope is inferred from executed job names."
)


def gh_api(endpoint):
    """Authenticated API request; preserve gh failures instead of empty data."""
    result = subprocess.run(
        ["gh", "api", endpoint], check=True, capture_output=True, text=True,
    )
    return json.loads(result.stdout)


class Collector:
    def __init__(self, repo, api=gh_api):
        if not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", repo):
            raise ValueError("repo must be OWNER/REPO")
        self.repo = repo
        self.root = f"repos/{repo}/actions"
        self.raw = {}
        def record(endpoint):
            payload = api(endpoint)
            self.raw[endpoint] = payload
            return payload
        self.api = record

    def pages(self, endpoint, key, limit=None):
        items = []
        page = 1
        size = min(100, limit) if limit else 100
        while limit is None or len(items) < limit:
            separator = "&" if "?" in endpoint else "?"
            payload = self.api(f"{endpoint}{separator}per_page={size}&page={page}")
            batch = payload[key]
            items.extend(batch)
            if len(batch) < size or len(items) >= payload.get("total_count", float("inf")):
                break
            page += 1
        return items[:limit] if limit else items

    def run(self, run_id, attempt=None):
        if run_id < 1 or (attempt is not None and attempt < 1):
            raise ValueError("run ID and attempt must be positive")
        base = f"{self.root}/runs/{run_id}"
        if attempt is None:
            attempt = self.api(base)["run_attempt"]
        # Pin metadata and jobs to the same attempt, even if a rerun starts now.
        run = self.api(f"{base}/attempts/{attempt}")
        jobs = self.pages(f"{base}/attempts/{attempt}/jobs", "jobs")
        return analyze(self.repo, run, jobs)

    def history(self, limit=100, workflow="ci.yml"):
        if limit < 1:
            raise ValueError("limit must be positive")
        runs = self.pages(
            f"{self.root}/workflows/{quote(workflow, safe='')}/runs", "workflow_runs", limit,
        )
        return [self.run(run["id"], attempt) for run in runs
                for attempt in range(1, run.get("run_attempt", 1) + 1)]


def timestamp(value):
    if not value:
        return None
    try:
        result = datetime.fromisoformat(value.replace("Z", "+00:00"))
        # GitHub sometimes uses year 0001 for a job that never started.
        if result.year < 2000 or result.tzinfo is None:
            return None
        return result
    except (ValueError, TypeError):
        return None


def interval(item):
    if item.get("status") != "completed" or item.get("conclusion") == "skipped":
        return None
    start, end = timestamp(item.get("started_at")), timestamp(item.get("completed_at"))
    return (start, end) if start and end and end >= start else None


def deployment(job):
    return job["name"].lower().startswith("deploy")


def executed(job):
    return job.get("conclusion") != "skipped" and timestamp(job.get("started_at")) is not None


@dataclass
class Timing:
    label: str
    start: datetime
    end: datetime
    conclusion: str

    @property
    def seconds(self):
        return (self.end - self.start).total_seconds()


@dataclass
class ReportRun:
    run_id: int
    attempt: int
    title: str
    url: str
    start: datetime | None
    sha: str
    event: str
    status: str
    conclusion: str
    scope: str
    validation: list
    deployments: list
    steps: list
    deployment_steps: list
    omitted: list
    elapsed: float | None
    complete: bool

    @property
    def cohort(self):
        mode = "manual" if self.event == "workflow_dispatch" else self.event
        state = self.conclusion if self.status == "completed" else self.status
        rerun = " / rerun" if self.attempt > 1 else ""
        return f"{mode} / {self.scope} / {state}{rerun}"

    @property
    def baseline(self):
        return (self.attempt == 1 and self.status == "completed" and self.conclusion == "success"
                and self.scope == "full" and self.event in {"push", "pull_request"}
                and self.complete and self.elapsed is not None)

    @property
    def label(self):
        return f"{self.run_id} attempt {self.attempt}"


def analyze(repo, run, jobs):
    validation, deployments, steps, deployment_steps, omitted = [], [], [], [], []
    complete = True
    names = {j["name"] for j in jobs if executed(j) and not deployment(j)}
    scope = "full" if names & FULL_JOBS else "light" if LIGHT_JOB in names else "unknown"
    for job in jobs:
        times = interval(job)
        if times is None:
            reason = job.get("conclusion") or job.get("status") or "unknown"
            if job.get("status") == "completed" and reason != "skipped":
                reason += "; missing/invalid timestamps"
            omitted.append(f"{job['name']}: {reason}")
            if not deployment(job) and job.get("conclusion") != "skipped":
                complete = False
            continue
        row = Timing(job["name"], *times, job.get("conclusion") or "unknown")
        (deployments if deployment(job) else validation).append(row)
        for step in job.get("steps", []):
            step_times = interval(step)
            if step_times:
                target = deployment_steps if deployment(job) else steps
                target.append(Timing(f"{job['name']} / {step['name']}", *step_times,
                                     step.get("conclusion") or "unknown"))
            else:
                omitted.append(f"{job['name']} / {step['name']}: "
                               f"{step.get('conclusion') or step.get('status') or 'unknown'}; "
                               "no completed timing")
    start = timestamp(run.get("run_started_at"))
    end = max((row.end for row in validation), default=None)
    elapsed = (end - start).total_seconds() if start and end and end >= start else None
    attempt = run.get("run_attempt", 1)
    return ReportRun(
        run["id"], attempt, run.get("display_title", "CI"),
        f"https://github.com/{repo}/actions/runs/{run['id']}/attempts/{attempt}",
        start, run.get("head_sha", "unknown"), run.get("event", "unknown"),
        run["status"], run.get("conclusion") or "pending", scope,
        validation, deployments, steps, deployment_steps, omitted, elapsed, complete,
    )


def duration(seconds):
    return "unavailable" if seconds is None else f"{seconds / 60:.2f} min"


def chart(title, rows):
    """Inline accessible SVG. Rows: (label, offset seconds, duration, URL)."""
    if not rows:
        return f"<h3>{escape(title)}</h3><p>No completed timings.</p>"
    width, left, plot, row_height = 1100, 490, 510, 27
    extent = max((offset + seconds for _, offset, seconds, _ in rows), default=1) or 1
    height = 44 + len(rows) * row_height
    parts = [f'<h3>{escape(title)}</h3><svg role="img" aria-label="{escape(title)}" '
             f'viewBox="0 0 {width} {height}" xmlns="http://www.w3.org/2000/svg">',
             f'<title>{escape(title)} — minutes</title>']
    for i in range(6):
        x = left + plot * i / 5
        parts.append(f'<path d="M{x} 24V{height}" stroke="#d9e1e8"/>'
                     f'<text x="{x}" y="17" text-anchor="middle">{extent*i/300:.1f}</text>')
    for index, (label, offset, seconds, url) in enumerate(rows):
        y = 32 + index * row_height
        label = str(label)
        full = escape(f"{label}: {duration(seconds)}; start +{duration(offset)}")
        short = escape(label if len(label) <= 65 else label[:62] + "…")
        parts.append(f'<a href="{escape(url, quote=True)}"><title>{full}</title>'
                     f'<text x="0" y="{y+14}">{short}</text>'
                     f'<rect x="{left+offset/extent*plot:.2f}" y="{y}" '
                     f'width="{max(seconds/extent*plot, 1):.2f}" height="19" rx="3" fill="#2563eb"/>'
                     f'<text x="{left+(offset+seconds)/extent*plot+5:.2f}" y="{y+14}">'
                     f'{seconds/60:.2f}</text></a>')
    parts.append("</svg>")
    return "".join(parts)


def inline_code(text):
    # API job names are untrusted Markdown; use inline code with no backticks/newlines.
    return "`" + text.replace("`", "'").replace("\n", " ") + "`"


def slow_setup(run):
    return [row for row in run.steps + run.deployment_steps
            if row.label.rsplit(" / ", 1)[-1] in SETUP_STEPS and row.seconds >= SLOW_SETUP_SECONDS]


def markdown(runs):
    baseline = [r.elapsed for r in runs if r.baseline]
    lines = ["## CI timing report", "", METHOD, "",
             f"Collected {len(runs)} attempt(s); baseline n={len(baseline)}."]
    if baseline:
        lines.append(f"Full successful routine validation: median {duration(statistics.median(baseline))}; "
                     f"range {duration(min(baseline))}–{duration(max(baseline))}.")
    lines += ["", "### Attempts"]
    for run in runs:
        partial = " (partial snapshot)" if not run.complete else ""
        lines.append(f"- [{run.label}]({run.url}) — {run.cohort}; "
                     f"validation {duration(run.elapsed)}{partial}; revision `{run.sha}`.")
        if run.omitted:
            lines.append(f"  - Omitted: {inline_code('; '.join(run.omitted))}.")
        slow = slow_setup(run)
        if slow:
            text = "; ".join(f"{row.label} {row.seconds:.0f}s" for row in slow)
            lines.append(f"  - Warning: slow cache setup (≥{SLOW_SETUP_SECONDS}s): {inline_code(text)}.")
        if run.deployments:
            lines.append("  - Deployment execution (separate): " + "; ".join(
                f"{duration(row.seconds)}" for row in run.deployments) + ".")
    return "\n".join(lines) + "\n"


def render_html(runs, repo):
    e = escape
    generated = datetime.now(timezone.utc).isoformat(timespec="seconds")
    parts = ['<!doctype html><html lang="en"><meta charset="utf-8">'
             '<meta name="viewport" content="width=device-width,initial-scale=1">'
             '<title>CI timing report</title><style>'
             'body{font:16px system-ui;color:#17243b;background:#f6f8fb;margin:2rem auto;max-width:1200px;padding:0 1rem}'
             'section,details{background:white;padding:1rem;margin:1rem 0;border:1px solid #dae1ea;border-radius:8px}'
             'svg{width:100%;min-width:800px}svg text{font:12px system-ui;fill:#17243b}'
             '.chart{overflow-x:auto}a{color:#1d4ed8}summary{cursor:pointer}li{margin:.4rem 0}'
             '</style><body>', f'<h1>CI timing report — {e(repo)}</h1><p>Generated {e(generated)}</p>',
             f'<p>{e(METHOD)}</p>']
    baseline = [r.elapsed for r in runs if r.baseline]
    parts.append(f'<p>{len(runs)} attempts; baseline n={len(baseline)}'
                 + (f'; median <strong>{duration(statistics.median(baseline))}</strong>' if baseline else '')
                 + '.</p>')
    cohorts = defaultdict(list)
    for run in sorted(runs, key=lambda r: (r.start.isoformat() if r.start else "", r.run_id, r.attempt)):
        cohorts[run.cohort].append(run)
    parts.append('<h2>History by cohort</h2><p>Oldest first in each cohort. Bars link to the exact attempt.</p>')
    for cohort, members in sorted(cohorts.items()):
        rows = [(r.label + (" (partial)" if not r.complete else ""), 0, r.elapsed, r.url)
                for r in members if r.elapsed is not None]
        parts.append('<section class="chart">' + chart(cohort, rows) +
                     f'<p>{len(members)} attempts; {len(members)-len(rows)} without validation timing.</p></section>')
    parts.append('<h2>Attempt details</h2>')
    for run in runs:
        parts.append(f'<details{" open" if len(runs) == 1 else ""}><summary>{e(run.label)} — '
                     f'{e(run.cohort)} — {e(run.title)}</summary><p><a href="{e(run.url)}">GitHub attempt</a>; '
                     f'revision <code>{e(run.sha)}</code>; started {e(str(run.start))}; '
                     f'validation {duration(run.elapsed)}{" (partial snapshot)" if not run.complete else ""}.</p>')
        for title, timings in [("Validation jobs", run.validation), ("Deployment jobs (separate)", run.deployments)]:
            origin = run.start or min((t.start for t in timings), default=None)
            rows = [(t.label, max(0, (t.start-origin).total_seconds()), t.seconds, run.url) for t in timings]
            parts.append('<div class="chart">' + chart(title + " — minutes from attempt start", rows) + '</div>')
        for title, steps in [("Validation steps", run.steps), ("Deployment steps (separate)", run.deployment_steps)]:
            parts.append('<div class="chart">' + chart(title + " — execution minutes (longest first)", [
                (t.label, 0, t.seconds, run.url) for t in sorted(steps, key=lambda t: t.seconds, reverse=True)
            ]) + '</div>')
        if run.omitted:
            parts.append('<h3>Omitted timings</h3><ul>' + ''.join(f'<li>{e(s)}</li>' for s in run.omitted) + '</ul>')
        parts.append('</details>')
    parts.append('</body></html>')
    return '\n'.join(parts)


def positive(value):
    number = int(value)
    if number < 1:
        raise argparse.ArgumentTypeError("must be positive")
    return number


def main(argv=None, api=gh_api):
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    for mode in ("history", "run"):
        sub = commands.add_parser(mode)
        sub.add_argument("--repo", default=os.environ.get("GITHUB_REPOSITORY", "Coda-Health/coda-rust"))
        sub.add_argument("--output", type=Path, default=Path(f"ci-timings-{mode}.html"))
        sub.add_argument("--summary", type=Path, help="Markdown destination (default: output with .md suffix); appended for GITHUB_STEP_SUMMARY")
        if mode == "history":
            sub.add_argument("--limit", type=positive, default=100, help="workflow runs, including all their attempts")
            sub.add_argument("--workflow", default="ci.yml")
        else:
            sub.add_argument("--run-id", type=positive, default=os.environ.get("GITHUB_RUN_ID"))
            sub.add_argument("--attempt", type=positive, default=os.environ.get("GITHUB_RUN_ATTEMPT"))
    args = parser.parse_args(argv)
    if args.command == "run" and args.run_id is None:
        parser.error("run requires --run-id or GITHUB_RUN_ID")
    summary = args.summary or args.output.with_suffix(".md")
    if summary.resolve() in {args.output.resolve(), args.output.with_suffix('.json').resolve()} or args.output.suffix == '.json':
        parser.error("HTML output, JSON data and Markdown summary paths must differ")
    try:
        collector = Collector(args.repo, api)
        runs = (collector.history(args.limit, args.workflow) if args.command == "history"
                else [collector.run(args.run_id, args.attempt)])
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(render_html(runs, args.repo), encoding="utf-8")
        args.output.with_suffix('.json').write_text(
            json.dumps(collector.raw, indent=2) + '\n', encoding='utf-8')
        summary.parent.mkdir(parents=True, exist_ok=True)
        mode = "a" if str(summary) == os.environ.get("GITHUB_STEP_SUMMARY") else "w"
        with summary.open(mode, encoding="utf-8") as handle:
            handle.write(markdown(runs))
    except (OSError, ValueError, KeyError, subprocess.CalledProcessError) as error:
        print(f"ci_timings: {error}", file=sys.stderr)
        if isinstance(error, subprocess.CalledProcessError) and error.stderr:
            print(error.stderr, file=sys.stderr)
        return 1
    print(f"HTML/SVG report: {args.output}\nMarkdown summary: {summary}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
