#!/usr/bin/env python3
"""Render authored ledgers into the patient bank, its panels and manifest.

    python3 scripts/patient-bank/build_bank.py --seeds target/patient-bank/seeds.json \
        --ledgers target/patient-bank/ledgers --out target/patient-bank/bank \
        [--roger DIR]   # include the five Coda Agents Roger patients verbatim

Output:
  bank/patients/<MRN>/<Family>.ndjson   one chart per patient, as of 2026-08-27
  bank/panels/panel-<N>.json            nested panels (100 ⊂ 300 ⊂ 500 ⊂ 1000)
  bank/manifest.json                    every patient with lineage, counts and hashes
  bank/ledgers/<unit>.json              the authored source each chart was rendered from

`materialize.py` turns a panel (optionally as of an earlier instant) into one
multi-patient NDJSON directory in the Roger fixture layout.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import random
import shutil
import sys
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import render  # noqa: E402
import terms  # noqa: E402

AS_OF = "2026-08-27T12:00"
PANEL_SIZES = (100, 300, 500, 1000)
PANEL_SEED = 20261024


def write_ndjson(path: Path, rows: list[dict]) -> None:
    with path.open("w") as f:
        for r in rows:
            f.write(json.dumps(r, ensure_ascii=False, separators=(",", ":")) + "\n")


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def roger_patients(roger: Path) -> dict[str, dict[str, list[dict]]]:
    charts: dict[str, dict[str, list[dict]]] = {}
    for fam in render.FAMILIES:
        for line in (roger / f"{fam}.ndjson").read_text().splitlines():
            r = json.loads(line)
            ref = r["id"] if fam == "Patient" else (r.get("subject") or r.get("patient"))["reference"].split("/")[1]
            charts.setdefault(ref, {f: [] for f in render.FAMILIES})[fam].append(r)
    return charts


def panels(units: list[dict], mrns: set[str]) -> dict[int, list[str]]:
    """Nested panels that keep authoring units (families) together.

    Units are shuffled with a fixed seed, then stratified by complexity so each
    prefix has the bank's mix; a family enters a panel whole.
    """
    rng = random.Random(PANEL_SEED)
    order = units[:]
    rng.shuffle(order)
    by = {"light": [], "moderate": [], "complex": []}
    for u in order:
        tier = max((m["complexity"] for m in u["members"]), key=["light", "moderate", "complex"].index)
        by[tier].append(u)
    total = sum(len(v) for v in by.values())
    share = {k: len(v) / total for k, v in by.items()}
    taken = {k: 0 for k in by}
    sequence = []
    while any(taken[k] < len(by[k]) for k in by):
        # pick the tier furthest below its share so far
        n = max(1, len(sequence))
        k = min((k for k in by if taken[k] < len(by[k])), key=lambda k: taken[k] / n - share[k])
        sequence.append(by[k][taken[k]])
        taken[k] += 1
    flat: list[list[str]] = [[m["mrn"] for m in u["members"] if m["mrn"] in mrns] for u in sequence]
    flat = [g for g in flat if g]
    result: dict[int, list[str]] = {}
    for size in PANEL_SIZES:
        members: list[str] = []
        for g in flat:
            if len(members) + len(g) <= size:  # a family enters whole or waits
                members.extend(g)
            if len(members) == size:
                break
        result[size] = members
    for small, large in zip(PANEL_SIZES, PANEL_SIZES[1:]):
        assert set(result[small]) <= set(result[large]), f"panel-{small} not nested in panel-{large}"
    return result


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--seeds", type=Path, required=True)
    ap.add_argument("--ledgers", type=Path, required=True)
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--roger", type=Path, help="Roger fixture internal_resources directory to include verbatim")
    a = ap.parse_args()
    catalog = render.load_catalog()
    seeds = json.loads(a.seeds.read_text())
    units = seeds["units"]
    extra = terms.resolve_extras(a.ledgers, catalog)
    r = render.Renderer(catalog, extra)
    if a.out.exists():
        shutil.rmtree(a.out)
    (a.out / "patients").mkdir(parents=True)
    (a.out / "ledgers").mkdir()
    (a.out / "panels").mkdir()
    manifest = {"as_of": AS_OF, "format": "FHIR R4 4.0.1 NDJSON, one directory per patient",
                "families": list(render.FAMILIES), "patients": [], "units": []}
    totals: Counter = Counter()
    coding: Counter = Counter()
    all_units = []
    if a.roger:
        roger_charts = roger_patients(a.roger)
        roger_units = []
        for mrn, chart in sorted(roger_charts.items()):
            d = a.out / "patients" / mrn
            d.mkdir()
            for fam, rows in chart.items():
                write_ndjson(d / f"{fam}.ndjson", rows)
                totals[fam] += len(rows)
            p = chart["Patient"][0]
            manifest["patients"].append({
                "mrn": mrn, "name": f"{p['name'][0]['given'][0]} {p['name'][0]['family']}",
                "lineage": {"source": "coda-agents docs/local-product-fixtures (Roger practice)",
                            "authoring": "authored through coda-gym's chart flow; copied byte-for-byte",
                            "clinical_review": "not physician reviewed"},
                "unit": "roger", "complexity": "fixture",
                "counts": {f: len(v) for f, v in chart.items()},
                "sha256": {f: sha256_file(d / f"{f}.ndjson") for f in render.FAMILIES}})
            roger_units.append({"unit": "roger", "kind": "individual",
                                "members": [{"mrn": mrn, "complexity": "complex"}]})
        all_units.extend(roger_units)
    for unit in units:
        lp = a.ledgers / f"{unit['unit']}.json"
        if not lp.exists():
            continue
        led = json.loads(lp.read_text())
        shutil.copyfile(lp, a.out / "ledgers" / lp.name)
        by_mrn = {p["mrn"]: p for p in led["patients"]}
        manifest["units"].append({"unit": unit["unit"], "kind": unit["kind"],
                                  "members": [m["mrn"] for m in unit["members"]],
                                  "model": led.get("model"), "ledger_sha256": sha256_file(lp)})
        all_units.append(unit)
        for seed in unit["members"]:
            chart, receipt = r.render(seed, by_mrn[seed["mrn"]], AS_OF)
            d = a.out / "patients" / seed["mrn"]
            d.mkdir()
            for fam, rows in chart.items():
                write_ndjson(d / f"{fam}.ndjson", rows)
                totals[fam] += len(rows)
                for row in rows:
                    cc = row.get("code") or row.get("medicationCodeableConcept")
                    if fam in ("Condition", "MedicationRequest", "Observation", "Procedure",
                               "AllergyIntolerance", "DiagnosticReport"):
                        coding[(fam, "coded" if cc and cc.get("coding") else "text")] += 1
            manifest["patients"].append({
                "mrn": seed["mrn"], "name": f"{seed['given']} {seed['family']}", "sex": seed["sex"],
                "birth_date": seed["birth_date"], "town": seed["town"], "complexity": seed["complexity"],
                "unit": unit["unit"], "family_id": seed.get("family_id"),
                "relationships": seed.get("relationships", []),
                "lineage": {"source": "authored for the Coda synthetic patient bank",
                            "seed": "scripts/patient-bank/design_panel.py (seed 20261024)",
                            "author_model": led.get("model"),
                            "authoring_guide": "scripts/patient-bank/authoring-guide.md",
                            "clinical_review": "not physician reviewed; generated, not clinically validated"},
                "counts": {f: len(v) for f, v in chart.items()},
                "render_receipt": {"filled_panel_values": len(receipt.filled), "derived_values": len(receipt.derived),
                                   "uncoded": receipt.uncoded, "skipped": receipt.skipped},
                "sha256": {f: sha256_file(d / f"{f}.ndjson") for f in render.FAMILIES}})
    mrns = {p["mrn"] for p in manifest["patients"]}
    pan = panels(all_units, mrns)
    for size, members in pan.items():
        (a.out / "panels" / f"panel-{size}.json").write_text(json.dumps(
            {"size": len(members), "requested": size, "nested_in": [s for s in PANEL_SIZES if s > size],
             "members": members}, indent=1) + "\n")
    manifest["totals"] = dict(totals)
    manifest["coding"] = {f"{k[0]}:{k[1]}": v for k, v in sorted(coding.items())}
    manifest["patient_count"] = len(manifest["patients"])
    manifest["panels"] = {str(k): len(v) for k, v in pan.items()}
    (a.out / "manifest.json").write_text(json.dumps(manifest, indent=1, ensure_ascii=False) + "\n")
    print(json.dumps({"patients": manifest["patient_count"], "totals": manifest["totals"],
                      "panels": manifest["panels"], "coding": manifest["coding"]}, indent=1))


if __name__ == "__main__":
    main()
