#!/usr/bin/env python3
"""Run the named verification boundary; keep logs outside tracked source."""
import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import platform
import re
import subprocess

ROOT = Path(__file__).resolve().parent.parent
LOCK = json.loads((ROOT / "verification/verus.json").read_text())
MACHINE = f"{platform.system()}-{platform.machine()}"
ASSET = LOCK["assets"].get(MACHINE)
VERUS = ROOT / "target/verification/tools" / LOCK["version"] / f"verus-{ASSET['name'] if ASSET else 'unsupported'}" / "verus"


def run(command, log, *, cwd=ROOT, env=None, expect_failure=None):
    print("Running:", " ".join(map(str, command)), flush=True)
    with log.open("w") as output:
        result = subprocess.run(list(map(str, command)), cwd=cwd, env=env,
                                stdout=output, stderr=subprocess.STDOUT)
    text = log.read_text()
    if expect_failure:
        valid = result.returncode != 0 and expect_failure in text
        valid = valid and "could not compile" not in text
    else:
        valid = result.returncode == 0
        if list(command[:2]) == ["cargo", "test"]:
            valid = valid and any(int(n) > 0 for n in re.findall(r"test result: ok\. (\d+) passed", text))
    if not valid:
        print(text[-12000:])
        raise SystemExit(f"FAILED: see {log}")
    print(f"{'Expected failure confirmed' if expect_failure else 'Passed'}: {log}", flush=True)
    return text


def formal(out, spec):
    print(f"Formal source: {spec}", flush=True)
    if not VERUS.is_file():
        raise SystemExit("Run python3 verification/install-verus.py first.")
    version = run([VERUS, "--version"], out / "verus-version.log")
    if LOCK["version"] not in version:
        raise SystemExit("Verifier does not match verification/verus.json")
    run([VERUS, spec, "--no-cheating"], out / "formal.log")


def stateright(out, chart):
    print(f"Stateright chart: {chart}", flush=True)
    env = {**os.environ, "CARGO_TARGET_DIR": str(ROOT / "target/verification/stateright")}
    run(["cargo", "run", "--locked", "--release", "--quiet", "--manifest-path",
         "verification/stateright/Cargo.toml", "--", chart, "check"], out / "stateright.log", env=env)
    print((out / "stateright.log").read_text().strip(), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("check", choices=["formal", "stateright"])
    parser.add_argument("--out", type=Path, help="new evidence directory; defaults to ignored target/verification/runs")
    parser.add_argument("--spec", type=Path, help="Verus source to check; required for formal")
    parser.add_argument("--chart", help="chart registered in verification/stateright, such as physician/sign-in; required for stateright")
    args = parser.parse_args()
    if args.check == "formal" and (args.spec is None or not args.spec.is_file()):
        parser.error("formal requires --spec pointing to an existing Verus source file")
    if args.check == "stateright" and not args.chart:
        parser.error("stateright requires --chart")
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
    out = (args.out or ROOT / "target/verification/runs" / f"{stamp}-{args.check}").resolve()
    out.mkdir(parents=True, exist_ok=False)
    print(f"Evidence: {out}", flush=True)
    if args.check == "formal":
        formal(out, args.spec.resolve())
    else:
        stateright(out, args.chart)
