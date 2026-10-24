#!/usr/bin/env python3
"""Validate the patient bank in bulk and write a report.

    python3 scripts/patient-bank/validate.py --bank target/patient-bank/bank \
        --seeds target/patient-bank/seeds.json --schema /path/to/fhir.schema.json \
        [--jobs 8] [--report target/patient-bank/validation.json]

Checks, per patient (errors fail the bank; warnings are counted for review):
  schema      every resource against the official HL7 FHIR R4 4.0.1 JSON schema
  ndjson      one resource type per file, FHIR ids, unique ids, UTC timestamps
  references  subject/patient, DiagnosticReport.result, Observation.hasMember resolve
              to the same patient
  terminology every coding is the catalog's verified code and display, or a
              render-time verified extra; text-only concepts are counted
  timeline    clinical times after birth and before the bank time; issued >= effective;
              onset <= recorded; abatement after onset; stated note ages match birthDate
  realism     no duplicate note bodies, no scaffold words, no repeated vital tuples
  snapshots   the chart rendered as of three earlier instants contains nothing dated
              at or after the instant, every snapshot row exists in the full chart,
              and medication/problem statuses are the ones true at that instant
  future      note text mentioning a diagnosis or drug that the chart only records
              later (lexical; warning, reviewed by reading)
"""

from __future__ import annotations

import argparse
import base64
import json
import random
import re
import sys
from collections import Counter
from concurrent.futures import ProcessPoolExecutor
from datetime import date
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import render  # noqa: E402

AS_OF_UTC = "2026-08-27T16:00:00+00:00"
ID_RE = re.compile(r"^[A-Za-z0-9\-.]{1,64}$")
UTC_RE = re.compile(r"^\d{4}-\d{2}-\d{2}(T\d{2}:\d{2}:\d{2}\+00:00)?$")
SCAFFOLD = re.compile(r"\b(synthetic|fictional|generated|template|simulation|test patient|answer key|lorem)\b", re.I)
AGE_RE = re.compile(r"\b(\d{2,3})[- ](?:year|yr)s?[- ]old\b", re.I)
KIN = re.compile(r"\b(mother|father|sister|brother|son|daughter|wife|husband|spouse|partner|grand\w*|aunt|"
                 r"uncle|cousin|friend|at age|in her|in his|niece|nephew)\b", re.I)
TIME_FIELDS = ("onsetDateTime", "recordedDate", "abatementDateTime", "authoredOn", "effectiveDateTime",
               "issued", "performedDateTime", "date")
STOP = {"disease", "disorder", "unspecified", "chronic", "of", "with", "without", "the", "and", "or", "in",
        "acute", "primary", "history", "due", "to", "status", "post", "type", "syndrome", "stage", "a",
        "mild", "moderate", "severe", "single", "episode", "recurrent", "other", "lower", "upper", "left",
        "right", "bilateral", "extremity", "extremities", "limb", "use", "current", "long-term", "term",
        "presence", "personal", "family", "gene", "positive", "encounter", "for", "general", "adult",
        "medical", "examination", "normal", "supervision", "pregnancy", "complications", "foot", "knee", "hip"}
SYNONYMS = {
    "Essential hypertension": ["hypertension", "htn"],
    "Hyperlipidemia": ["hyperlipidemia", "dyslipidemia"], "Mixed hyperlipidemia": ["hyperlipidemia"],
    "Type 2 diabetes mellitus without complications": ["type 2 diabetes", "t2dm", "diabetes mellitus"],
    "Type 2 diabetes mellitus with hyperglycemia": ["type 2 diabetes", "t2dm"],
    "Prediabetes": ["prediabetes", "pre-diabetes"],
    "Paroxysmal atrial fibrillation": ["atrial fibrillation", "afib", "a-fib"],
    "Persistent atrial fibrillation": ["atrial fibrillation", "afib"],
    "Chronic obstructive pulmonary disease": ["copd"], "Coronary artery disease": ["coronary artery disease", "cad"],
    "Obstructive sleep apnea": ["sleep apnea", "osa"], "Hypothyroidism": ["hypothyroidism"],
    "Gastroesophageal reflux disease without esophagitis": ["gerd", "reflux"],
    "Generalized anxiety disorder": ["generalized anxiety", "gad"],
    "Heart failure with reduced ejection fraction": ["heart failure", "hfref"],
    "Heart failure with preserved ejection fraction": ["heart failure", "hfpef"],
}


def b64text(data: str) -> str:
    return base64.b64decode(data).decode("utf-8", "replace")


def condition_terms(label: str) -> list[str]:
    if label in SYNONYMS:
        return SYNONYMS[label]
    if label.lower().startswith(("history of", "family history", "status post", "long-term", "presence",
                                 "encounter", "supervision")):
        return []
    words = [w for w in re.findall(r"[a-z][a-z\-]+", label.lower()) if w not in STOP and len(w) > 4]
    return [label.lower()] + ([" ".join(words)] if len(words) >= 2 else [])


def future_mentions(chart: dict[str, list[dict]]) -> list[tuple[str, str]]:
    """Note text naming a diagnosis or drug the chart only records later (lexical)."""
    found: list[tuple[str, str]] = []
    docs = sorted(chart["DocumentReference"], key=lambda r: r["date"])
    first_recorded: dict[str, tuple[str, list[str]]] = {}
    for r in chart["Condition"]:
        label = r["code"].get("text", "")
        when = r.get("recordedDate", "")
        onset = r.get("onsetDateTime", "")
        known = min(when, onset) if onset else when
        first_recorded.setdefault(label, (known, condition_terms(label)))
    first_drug: dict[str, str] = {}
    for r in chart["MedicationRequest"]:
        words = re.findall(r"[a-z][a-z-]+", r["medicationCodeableConcept"].get("text", "").lower())
        ing = words[0] if words else ""
        if len(ing) > 4:
            first_drug[ing] = min(first_drug.get(ing, "9999"), r["authoredOn"])
    for doc in docs:
        text = b64text(doc["content"][0]["attachment"]["data"]).lower()
        for label, (known, terms_) in first_recorded.items():
            if known[:10] > doc["date"][:10]:
                for t in terms_:
                    if t and re.search(r"\b" + re.escape(t) + r"\b", text):
                        found.append(("future_condition_mention", f"future condition: {doc['id']} ({doc['date'][:10]}) mentions {t!r}, recorded {known[:10]}"))
                        break
        for ing, started in first_drug.items():
            if started[:10] > doc["date"][:10] and re.search(r"\b" + re.escape(ing) + r"\b", text):
                found.append(("future_drug_mention", f"future drug: {doc['id']} ({doc['date'][:10]}) mentions {ing!r}, started {started[:10]}"))
    return found


def load_chart(d: Path) -> dict[str, list[dict]]:
    chart = {}
    for fam in render.FAMILIES:
        p = d / f"{fam}.ndjson"
        chart[fam] = [json.loads(l) for l in p.read_text().splitlines()] if p.exists() else []
    return chart


def init_worker(schema_path: str, catalog: dict, extra: dict) -> None:
    global VALIDATORS, CATALOG, EXTRA, KNOWN
    import jsonschema
    root = json.loads(Path(schema_path).read_text())
    VALIDATORS = {}
    for fam in render.FAMILIES:
        sub = {k: v for k, v in root.items() if k not in ("oneOf", "discriminator")}
        sub["$ref"] = f"#/definitions/{fam}"
        VALIDATORS[fam] = jsonschema.Draft6Validator(sub)
    CATALOG, EXTRA = catalog, extra
    KNOWN = set()
    for sec in ("conditions", "medications", "labs", "panels", "surveys", "social", "reports", "procedures",
                "allergens", "reactions"):
        for e in catalog[sec].values():
            KNOWN.add((e["system"], e["code"], e.get("display")))
    for sec in ("conditions", "medications"):
        for e in extra.get(sec, {}).values():
            sysurl = catalog["systems"]["icd10cm" if sec == "conditions" else "rxnorm"]
            KNOWN.add((sysurl, e["code"], e["display"]))
    for code, disp, _ in [render.BP, render.SBP, render.DBP]:
        KNOWN.add(("http://loinc.org", code, disp))
    for code, disp, *_ in render.VITALS.values():
        KNOWN.add(("http://loinc.org", code, disp))
    for code, disp in render.DOC_TYPE_CODES.values():
        KNOWN.add(("http://loinc.org", code, disp))


def codings(cc: dict | None):
    for c in (cc or {}).get("coding", []) or []:
        yield c


def check_patient(args) -> dict:
    mrn, bank, seed, ledger_path, external = args
    d = Path(bank) / "patients" / mrn
    chart = load_chart(d)
    errors: list[str] = []
    warn: Counter = Counter()
    notes: list[str] = []
    stats: Counter = Counter()
    ids: dict[str, str] = {}
    # ndjson + schema
    for fam, rows in chart.items():
        raw = (d / f"{fam}.ndjson").read_text().splitlines() if (d / f"{fam}.ndjson").exists() else []
        if len(raw) != len(rows):
            errors.append(f"{fam}: line/row mismatch")
        for r in rows:
            stats[fam] += 1
            if r.get("resourceType") != fam:
                errors.append(f"{fam}: contains {r.get('resourceType')}")
            if not ID_RE.match(r.get("id", "")):
                errors.append(f"{fam}/{r.get('id')}: bad id")
            if r["id"] in ids:
                errors.append(f"duplicate id {r['id']}")
            ids[r["id"]] = fam
            for e in VALIDATORS[fam].iter_errors(r):
                errors.append(f"schema {fam}/{r['id']}: {e.message[:160]} at {'/'.join(map(str, e.absolute_path))}")
                break
            for k in TIME_FIELDS:
                v = r.get(k)
                if isinstance(v, str) and not UTC_RE.match(v) and not (fam == "Patient"):
                    errors.append(f"{fam}/{r['id']}: {k} {v} not UTC +00:00")
    if external:
        return {"mrn": mrn, "errors": errors, "warnings": dict(warn), "stats": dict(stats), "notes": notes,
                "external": True}
    patient = chart["Patient"][0]
    birth = date.fromisoformat(patient["birthDate"])
    # references
    for fam, rows in chart.items():
        for r in rows:
            ref = (r.get("subject") or r.get("patient") or {}).get("reference")
            if fam != "Patient" and ref != f"Patient/{mrn}":
                errors.append(f"{fam}/{r['id']}: subject {ref}")
            for key in ("result", "hasMember"):
                for x in r.get(key, []) or []:
                    t, _, i = x["reference"].partition("/")
                    if ids.get(i) != t:
                        errors.append(f"{fam}/{r['id']}: {key} {x['reference']} unresolved")
    # terminology
    for fam, rows in chart.items():
        for r in rows:
            for field in ("code", "medicationCodeableConcept", "type"):
                cc = r.get(field)
                if not isinstance(cc, dict):
                    continue
                cs = list(codings(cc))
                if fam in ("Condition", "MedicationRequest", "Observation", "Procedure", "AllergyIntolerance",
                           "DiagnosticReport") and field != "type":
                    stats[f"{fam}:{'coded' if cs else 'text-only'}"] += 1
                for c in cs:
                    if c.get("system") == "http://www.ama-assn.org/go/cpt":
                        if not any(e["code"] == c["code"] for e in CATALOG["procedures"].values()):
                            errors.append(f"{fam}/{r['id']}: CPT {c['code']} not in catalog")
                        continue
                    if (c.get("system"), c.get("code"), c.get("display")) not in KNOWN:
                        errors.append(f"{fam}/{r['id']}: unverified coding {c.get('system')}|{c.get('code')}|{c.get('display')}")
    # timeline
    for fam, rows in chart.items():
        if fam == "Patient":
            continue
        for r in rows:
            for k in TIME_FIELDS:
                v = r.get(k)
                if not isinstance(v, str):
                    continue
                if v >= AS_OF_UTC:
                    errors.append(f"{fam}/{r['id']}: {k} {v} not before bank time")
                if v[:10] < patient["birthDate"]:
                    errors.append(f"{fam}/{r['id']}: {k} {v} before birth")
            if r.get("issued") and r.get("effectiveDateTime") and r["issued"] < r["effectiveDateTime"]:
                errors.append(f"{fam}/{r['id']}: issued before effective")
            if fam == "Condition":
                if r.get("onsetDateTime", "")[:10] > r.get("recordedDate", "9")[:10]:
                    errors.append(f"Condition/{r['id']}: onset after recorded")
                if r.get("abatementDateTime") and r["abatementDateTime"] < r.get("onsetDateTime", ""):
                    errors.append(f"Condition/{r['id']}: abatement before onset")
    # documents
    docs = sorted(chart["DocumentReference"], key=lambda r: r["date"])
    bodies: dict[str, str] = {}
    for doc in docs:
        text = b64text(doc["content"][0]["attachment"]["data"])
        body = text.split("\n\n", 1)[1] if "\n\n" in text else text
        if body in bodies:
            errors.append(f"DocumentReference/{doc['id']}: duplicate body of {bodies[body]}")
        bodies[body] = doc["id"]
        if SCAFFOLD.search(body):
            errors.append(f"DocumentReference/{doc['id']}: scaffold word {SCAFFOLD.search(body).group(0)!r}")
        on = date.fromisoformat(doc["date"][:10])
        actual = on.year - birth.year - ((on.month, on.day) < (birth.month, birth.day))
        for m in AGE_RE.finditer(body):
            if int(m.group(1)) != actual and not KIN.search(body[max(0, m.start() - 40):m.end() + 10]):
                warn["note_age_mismatch"] += 1
                notes.append(f"age: {doc['id']} says {m.group(1)}, is {actual} ({doc['date'][:10]})")
    # repeated vital tuples
    tuples = Counter()
    for r in chart["Observation"]:
        if r["code"]["coding"][0]["code"] == "85354-9":
            tuples[(r["component"][0]["valueQuantity"]["value"], r["component"][1]["valueQuantity"]["value"])] += 1
    for t, n in tuples.items():
        if n >= 4:
            warn["repeated_bp_tuple"] += 1
    # future facts in note text
    for kind, msg in future_mentions(chart):
        warn[kind] += 1
        notes.append(msg)
    # snapshots
    led = json.loads(Path(ledger_path).read_text())
    pled = next(p for p in led["patients"] if p["mrn"] == mrn)
    r = render.Renderer(CATALOG, EXTRA)
    starts = sorted(e["start"] for e in pled["events"])
    rng = random.Random(mrn)
    full_ids = {x["id"]: x for rows in chart.values() for x in rows}
    differs = 0
    if len(starts) >= 3:
        for _ in range(3):
            cut = starts[rng.randint(1, len(starts) - 1)]
            cut_utc = render.to_utc(cut)
            snap, _ = r.render(seed, pled, cut)
            for fam, rows in snap.items():
                for x in rows:
                    if x["id"] not in full_ids:
                        errors.append(f"snapshot {cut}: {fam}/{x['id']} absent from full chart")
                        continue
                    for k in TIME_FIELDS:
                        v = x.get(k)
                        if isinstance(v, str) and fam != "Patient" and v >= cut_utc:
                            errors.append(f"snapshot {cut}: {fam}/{x['id']} {k} {v} not before cutoff")
                    full = full_ids[x["id"]]
                    if fam == "MedicationRequest" and x["status"] != full["status"]:
                        differs += 1
                        if x["status"] == "stopped":
                            errors.append(f"snapshot {cut}: {x['id']} stopped before its stop")
                    if fam == "Condition" and x.get("abatementDateTime") != full.get("abatementDateTime"):
                        differs += 1
                        if x.get("abatementDateTime"):
                            errors.append(f"snapshot {cut}: {x['id']} abated before its abatement")
                    if fam in ("Observation", "DiagnosticReport") and x["status"] != full["status"]:
                        differs += 1
            stats["snapshots"] += 1
    stats["snapshot_status_differences"] = differs
    return {"mrn": mrn, "errors": errors, "warnings": dict(warn), "stats": dict(stats), "notes": notes[:50]}


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--bank", type=Path, required=True)
    ap.add_argument("--seeds", type=Path, required=True)
    ap.add_argument("--schema", type=Path, required=True)
    ap.add_argument("--jobs", type=int, default=8)
    ap.add_argument("--report", type=Path)
    a = ap.parse_args()
    import terms
    catalog = render.load_catalog()
    manifest = json.loads((a.bank / "manifest.json").read_text())
    extra = terms.resolve_extras(a.bank / "ledgers", catalog)
    seeds = {m["mrn"]: (m, u["unit"]) for u in json.loads(a.seeds.read_text())["units"] for m in u["members"]}
    work = []
    for p in manifest["patients"]:
        if p["unit"] == "roger":
            work.append((p["mrn"], str(a.bank), None, None, True))
        else:
            seed, unit = seeds[p["mrn"]]
            work.append((p["mrn"], str(a.bank), seed, str(a.bank / "ledgers" / f"{unit}.json"), False))
    results = []
    with ProcessPoolExecutor(max_workers=a.jobs, initializer=init_worker,
                             initargs=(str(a.schema), catalog, extra)) as pool:
        for res in pool.map(check_patient, work, chunksize=4):
            results.append(res)
    errors = sum(len(r["errors"]) for r in results)
    warn = Counter()
    stats = Counter()
    for r in results:
        warn.update(r["warnings"])
        stats.update(r["stats"])
    # panel checks
    panel_errors = []
    sizes = {}
    prev = None
    mrns = {p["mrn"] for p in manifest["patients"]}
    fam_of = {p["mrn"]: p.get("family_id") for p in manifest["patients"]}
    for f in sorted((a.bank / "panels").glob("panel-*.json"), key=lambda p: int(p.stem.split("-")[1])):
        pan = json.loads(f.read_text())
        mem = set(pan["members"])
        sizes[f.stem] = len(mem)
        if len(mem) != pan["requested"]:
            panel_errors.append(f"{f.stem}: {len(mem)} members, requested {pan['requested']}")
        if not mem <= mrns:
            panel_errors.append(f"{f.stem}: unknown members")
        if prev is not None and not prev <= mem:
            panel_errors.append(f"{f.stem}: does not contain the smaller panel")
        for fid in {fam_of[m] for m in mem if fam_of.get(m)}:
            together = {m for m, x in fam_of.items() if x == fid}
            if not together <= mem:
                panel_errors.append(f"{f.stem}: splits {fid}")
        prev = mem
    report = {
        "bank": str(a.bank), "patients": len(results),
        "patients_with_errors": sum(1 for r in results if r["errors"]),
        "errors": errors + len(panel_errors), "panel_errors": panel_errors, "panels": sizes,
        "warnings": dict(warn), "stats": dict(stats),
        "terminology_extras": {"conditions": len(extra["conditions"]), "medications": len(extra["medications"]),
                               "rejected": extra["rejected"]},
        "failures": [r for r in results if r["errors"]][:200],
        "review_notes": {r["mrn"]: r["notes"] for r in results if r["notes"]},
    }
    out = a.report or (a.bank.parent / "validation.json")
    out.write_text(json.dumps(report, indent=1, ensure_ascii=False) + "\n")
    print(json.dumps({k: report[k] for k in ("patients", "patients_with_errors", "errors", "panels", "warnings",
                                             "stats", "panel_errors")}, indent=1))
    print("report:", out)
    sys.exit(1 if report["errors"] else 0)


if __name__ == "__main__":
    main()
