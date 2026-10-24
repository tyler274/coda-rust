#!/usr/bin/env python3
"""Write one panel as a multi-patient NDJSON directory in the Roger fixture layout.

    python3 scripts/patient-bank/materialize.py --bank target/patient-bank/bank \
        --seeds target/patient-bank/seeds.json --panel 100 --out target/patient-bank/out/panel-100 \
        [--as-of 2025-03-01T09:00]   # Eastern wall clock; default is the bank time

At the bank time the per-patient charts are concatenated as they are. With
--as-of, each chart is re-rendered from its ledger by the same renderer, so the
directory holds only what the practice knew at that instant: later records are
absent, later stops and resolutions have not happened, and results not yet back
are "registered". Roger fixture patients are copied only at the bank time.
"""

from __future__ import annotations

import argparse
import json
import shutil
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import render  # noqa: E402
import terms  # noqa: E402
from build_bank import AS_OF, write_ndjson  # noqa: E402


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--bank", type=Path, required=True)
    ap.add_argument("--seeds", type=Path, required=True)
    ap.add_argument("--panel", type=int, required=True)
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--as-of", default=AS_OF)
    a = ap.parse_args()
    if a.as_of > AS_OF:
        sys.exit(f"--as-of {a.as_of} is after the bank time {AS_OF}")
    manifest = json.loads((a.bank / "manifest.json").read_text())
    members = json.loads((a.bank / "panels" / f"panel-{a.panel}.json").read_text())["members"]
    by_mrn = {p["mrn"]: p for p in manifest["patients"]}
    seeds = {m["mrn"]: m for u in json.loads(a.seeds.read_text())["units"] for m in u["members"]}
    rerender = a.as_of != AS_OF
    if rerender:
        catalog = render.load_catalog()
        r = render.Renderer(catalog, terms.resolve_extras(a.bank / "ledgers", catalog))
    rows: dict[str, list[dict]] = {f: [] for f in render.FAMILIES}
    skipped = []
    for mrn in members:
        p = by_mrn[mrn]
        if not rerender:
            for fam in render.FAMILIES:
                path = a.bank / "patients" / mrn / f"{fam}.ndjson"
                rows[fam].extend(json.loads(l) for l in path.read_text().splitlines())
            continue
        if p["unit"] == "roger":
            skipped.append(mrn)
            continue
        led = json.loads((a.bank / "ledgers" / f"{p['unit']}.json").read_text())
        pled = next(x for x in led["patients"] if x["mrn"] == mrn)
        if min(e["start"] for e in pled["events"]) >= a.as_of:
            skipped.append(mrn)  # not yet a patient of the practice
            continue
        chart, _ = r.render(seeds[mrn], pled, a.as_of)
        for fam in render.FAMILIES:
            rows[fam].extend(chart[fam])
    if a.out.exists():
        shutil.rmtree(a.out)
    a.out.mkdir(parents=True)
    for fam in render.FAMILIES:
        write_ndjson(a.out / f"{fam}.ndjson", rows[fam])
    summary = {"panel": a.panel, "as_of": a.as_of, "patients": len(rows["Patient"]),
               "not_included": skipped, "counts": {f: len(v) for f, v in rows.items()}}
    (a.out / "materialized.json").write_text(json.dumps(summary, indent=1) + "\n")
    print(json.dumps(summary, indent=1))


if __name__ == "__main__":
    main()
