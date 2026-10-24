#!/usr/bin/env python3
"""The authoring agent's tool for one unit (a patient, or a family authored together).

    python3 -I scripts/patient-bank/unit.py brief   <unit>   seed, targets, paths
    python3 -I scripts/patient-bank/unit.py check   <unit>   merge parts, check, count; ACCEPTED writes the ledger
    python3 -I scripts/patient-bank/unit.py preview <unit>   render and write chart.txt for reading

Drafts live in target/patient-bank/drafts/<unit>/ (story.md, part-NN.json).
An accepted ledger is written to target/patient-bank/ledgers/<unit>.json, the
only input `build_bank.py` renders. See authoring-guide.md for the process.
"""

from __future__ import annotations

import argparse
import base64
import json
import os
import sys
from datetime import datetime, timedelta
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import ledger as ledger_check  # noqa: E402
import render  # noqa: E402
import terms  # noqa: E402
from validate import future_mentions  # noqa: E402

ROOT = HERE.parent.parent / "target" / "patient-bank"
AS_OF = "2026-08-27T12:00"
MODEL = "claude-opus-5-5 (Claude Code workflow subagent)"
FLOOR = 0.8  # share of each target a chart must reach
HEADER = ("given", "family", "birth_date", "address_line", "phone", "email", "marital_status", "summary")


def vocabulary(catalog: dict) -> str:
    def block(title: str, names) -> str:
        return f"### {title}\n" + "\n".join(f"- {n}" for n in names) + "\n"
    panels = "\n".join(f"- {k}: {', '.join(v['members'])}" for k, v in catalog["panels"].items())
    standalone = sorted(set(catalog["labs"]) - {m for v in catalog["panels"].values() for m in v["members"]})
    return "\n".join([
        "## Catalog (use these labels exactly)",
        block("Conditions (ICD-10-CM)", sorted(catalog["conditions"])),
        block("Medications (RxNorm clinical drug names)", sorted(catalog["medications"])),
        "### Lab panels and their members\n" + panels + "\n",
        block("Standalone labs (panel: null)", standalone),
        block("Surveys", sorted(catalog["surveys"])),
        block("Social history items", sorted(catalog["social"])),
        block("Reports (imaging, ECG and other studies)", sorted(catalog["reports"])),
        block("Procedures and visit codes", sorted(catalog["procedures"])),
        block("Allergens", sorted(catalog["allergens"])),
        block("Allergy reactions", sorted(catalog["reactions"])),
    ])


def load_unit(uid: str, seeds: Path) -> dict:
    for u in json.loads(seeds.read_text())["units"]:
        if u["unit"] == uid:
            return u
    sys.exit(f"unknown unit {uid}")


def merge(drafts: Path) -> tuple[dict, list[str]]:
    errors: list[str] = []
    patients: dict[str, dict] = {}
    parts = sorted(drafts.glob("part-*.json"))
    if not parts:
        return {"patients": []}, [f"no part-*.json files in {drafts}"]
    for part in parts:
        try:
            data = json.loads(part.read_text())
        except json.JSONDecodeError as exc:
            errors.append(f"{part.name}: invalid JSON ({exc})")
            continue
        for p in data.get("patients", []):
            into = patients.setdefault(p.get("mrn"), {"mrn": p.get("mrn"), "events": []})
            for k in HEADER:
                if k in p and k not in into:
                    into[k] = p[k]
            for ev in p.get("events", []):
                ev["_part"] = part.name
                into["events"].append(ev)
    for p in patients.values():
        p["events"].sort(key=lambda e: e.get("start") or "")
        for k in ("given", "family", "birth_date"):
            if k not in p:
                errors.append(f"{p['mrn']}: {k} missing (put the patient fields in part-01.json)")
    return {"patients": list(patients.values())}, errors


def strip(led: dict) -> dict:
    return {"patients": [{**p, "events": [{k: v for k, v in e.items() if k != "_part"} for e in p["events"]]}
                         for p in led["patients"]]}


def rendered(unit: dict, led: dict, catalog: dict, drafts: Path) -> tuple[dict, dict]:
    merged = drafts / "merged"
    merged.mkdir(exist_ok=True)
    (merged / "ledger.json").write_text(json.dumps(strip(led), ensure_ascii=False))
    extra = terms.resolve_extras(merged, catalog)
    r = render.Renderer(catalog, extra)
    by = {p["mrn"]: p for p in strip(led)["patients"]}
    charts = {m["mrn"]: r.render(m, by[m["mrn"]], AS_OF)[0] for m in unit["members"] if m["mrn"] in by}
    return charts, extra


def cmd_brief(unit: dict, drafts: Path) -> None:
    vocab = ROOT / "vocabulary.md"
    members = [{k: v for k, v in m.items() if k != "age_at_as_of"} | {"age_on_2026_08_27": m["age_at_as_of"]}
               for m in unit["members"]]
    print(f"unit {unit['unit']} ({unit['kind']}, {len(members)} patient{'s' if len(members) > 1 else ''})")
    if len(members) > 1:
        print("Author these patients together; they are a family and their charts must agree.")
    print("\nSeed:\n" + json.dumps(members, indent=1))
    print("\nTargets (the check requires at least 80% of each):")
    for m in unit["members"]:
        print(f"  {m['mrn']}: {m['target_documents']} documents, {m['target_observations']} observations, "
              f"about {m['years_with_practice']} years with the practice")
    print(f"\nGuide:      {HERE / 'authoring-guide.md'}")
    print(f"Vocabulary: {vocab}")
    print(f"Write to:   {drafts}/story.md, {drafts}/part-01.json, part-02.json, ...")


def cmd_check(unit: dict, drafts: Path, catalog: dict) -> int:
    led, errors = merge(drafts)
    errs, warnings = ledger_check.check(unit, strip(led), catalog)
    where = {f"{p['mrn']}/{e.get('id')}": e["_part"] for p in led["patients"] for e in p["events"]}
    errors += [f"[{where.get(e.split(':')[0], '?')}] {e}" if e.split(":")[0] in where else e for e in errs]
    counts = {}
    if not errors:
        charts, extra = rendered(unit, led, catalog, drafts)
        warnings += [f"left text-only (lookup failed): {x}" for x in extra["rejected"]]
        for m in unit["members"]:
            chart = charts.get(m["mrn"])
            if not chart:
                continue
            docs, obs = len(chart["DocumentReference"]), len(chart["Observation"])
            counts[m["mrn"]] = {f: len(v) for f, v in chart.items()}
            for what, have, want in (("documents", docs, m["target_documents"]),
                                     ("observations", obs, m["target_observations"])):
                if have < FLOOR * want:
                    errors.append(f"{m['mrn']}: {have} {what}, target {want} (need at least {int(FLOOR * want)})")
            warnings += [msg for _, msg in future_mentions(chart)]
    for mrn, c in counts.items():
        print(f"{mrn}: " + ", ".join(f"{k} {v}" for k, v in c.items()))
    for w in warnings:
        print("warning:", w)
    for e in errors:
        print("ERROR:", e)
    if errors:
        print(f"\n{len(errors)} errors. Fix them in the part files and run check again.")
        return 1
    out = ROOT / "ledgers" / f"{unit['unit']}.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    tmp = out.with_suffix(".tmp")
    tmp.write_text(json.dumps({"unit": unit["unit"], "model": MODEL, "warnings": warnings, **strip(led)},
                              indent=1, ensure_ascii=False))
    os.replace(tmp, out)
    print(f"\nACCEPTED -> {out}")
    if warnings:
        print("Review the warnings above; future-mention warnings are often real chronology leaks.")
    return 0


def local(utc: str) -> str:
    """UTC instant as the practice's Eastern wall clock, the way the chart UI shows it."""
    if len(utc) <= 10:
        return utc
    return datetime.fromisoformat(utc).astimezone(render.EASTERN).strftime("%Y-%m-%d %H:%M")


def released(row: dict) -> str:
    issued, eff = row.get("issued"), row.get("effectiveDateTime")
    if issued and eff and datetime.fromisoformat(issued) - datetime.fromisoformat(eff) >= timedelta(hours=6):
        return f" (resulted {local(issued)})"
    return ""


def chart_text(chart: dict) -> str:
    out = []
    p = chart["Patient"][0]
    out.append(f"{p['name'][0]['given'][0]} {p['name'][0]['family']} | {p['gender']} | born {p['birthDate']} | "
               f"{p['address'][0].get('city')}")
    out.append("\n## Problem list")
    for c in sorted(chart["Condition"], key=lambda r: r.get("recordedDate", "")):
        out.append(f"  recorded {local(c.get('recordedDate', ''))[:10]} onset {local(c.get('onsetDateTime', ''))[:10]} "
                   f"{c['clinicalStatus']['coding'][0]['code']}: {c['code'].get('text')}"
                   + (f" (resolved {local(c['abatementDateTime'])[:10]})" if c.get("abatementDateTime") else ""))
    out.append("\n## Medications")
    for m in sorted(chart["MedicationRequest"], key=lambda r: r["authoredOn"]):
        dose = (m.get("dosageInstruction") or [{}])[0].get("text", "")
        out.append(f"  {local(m['authoredOn'])[:10]} {m['status']}: {m['medicationCodeableConcept'].get('text')} | {dose}")
    out.append("\n## Allergies")
    out += [f"  {a['code'].get('text')}" for a in chart["AllergyIntolerance"]] or ["  none recorded"]
    timeline: list[tuple[str, str]] = []
    for o in chart["Observation"]:
        if o.get("hasMember"):
            continue
        name = o["code"].get("text") or o["code"]["coding"][0]["display"]
        if "valueQuantity" in o:
            v = f"{o['valueQuantity']['value']} {o['valueQuantity'].get('unit', '')}".strip()
        elif "component" in o:
            v = "/".join(str(c["valueQuantity"]["value"]) for c in o["component"])
        else:
            v = o.get("valueString") or (o.get("valueCodeableConcept") or {}).get("text") or "-"
        flag = (o.get("interpretation") or [{}])[0].get("coding", [{}])[0].get("code", "")
        timeline.append((local(o.get("effectiveDateTime", "")),
                         f"obs  {name} {v}{' ' + flag if flag in ('H', 'L') else ''}{released(o)}"))
    for r in chart["DiagnosticReport"]:
        text = base64.b64decode(r["presentedForm"][0]["data"]).decode() if r.get("presentedForm") else ""
        timeline.append((local(r.get("effectiveDateTime", "")), f"REPORT {r['code'].get('text')} [{r['status']}]{released(r)}"
                         + (f"\n{text.strip()}" if text else "")))
    for x in chart["Procedure"]:
        timeline.append((local(x.get("performedDateTime", "")), f"proc {x['code'].get('text')}"))
    for d in chart["DocumentReference"]:
        text = base64.b64decode(d["content"][0]["attachment"]["data"]).decode()
        timeline.append((local(d["date"]), f"DOCUMENT {d['type'].get('text')}\n{text.strip()}"))
    out.append("\n## Chart, oldest first (Eastern time; results show when they were released)")
    day = None
    for when, line in sorted(timeline, key=lambda t: t[0]):
        if when[:10] != day:
            day = when[:10]
            out.append(f"\n=== {day}")
        out.append(line if not line.startswith(("DOCUMENT", "REPORT")) else "\n" + line)
    return "\n".join(out) + "\n"


def cmd_preview(unit: dict, drafts: Path, catalog: dict) -> int:
    led, errors = merge(drafts)
    if errors:
        print("\n".join("ERROR: " + e for e in errors))
        return 1
    charts, _ = rendered(unit, led, catalog, drafts)
    path = drafts / "chart.txt"
    path.write_text("\n\n".join(chart_text(c) for c in charts.values()))
    print(f"wrote {path} ({path.stat().st_size // 1024} KB). Read it start to end.")
    return 0


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("command", choices=["brief", "check", "preview"])
    ap.add_argument("unit")
    ap.add_argument("--seeds", type=Path, default=ROOT / "seeds.json")
    a = ap.parse_args()
    catalog = render.load_catalog()
    vocab = ROOT / "vocabulary.md"
    text = vocabulary(catalog)
    if not vocab.exists() or vocab.read_text() != text:  # follows the catalog; agents share it
        tmp = vocab.with_name(f"vocabulary.{os.getpid()}.tmp")
        tmp.write_text(text)
        os.replace(tmp, vocab)
    unit = load_unit(a.unit, a.seeds)
    drafts = ROOT / "drafts" / a.unit
    drafts.mkdir(parents=True, exist_ok=True)
    if a.command == "brief":
        cmd_brief(unit, drafts)
    elif a.command == "check":
        sys.exit(cmd_check(unit, drafts, catalog))
    else:
        sys.exit(cmd_preview(unit, drafts, catalog))


if __name__ == "__main__":
    main()
