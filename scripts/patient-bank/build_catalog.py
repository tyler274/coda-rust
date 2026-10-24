#!/usr/bin/env python3
"""Verify the candidate terminology and write the authoring catalog.

Reads `catalog_seed.py`, looks every code up in its official service, and writes
`catalog.json`: the canonical display, the clean label used as `code.text`, and a
receipt naming the service. Responses are cached in `terminology-cache.json` so a
rebuild is reproducible offline. Codes the service does not confirm are reported
and left out; nothing is guessed.

    python3 scripts/patient-bank/build_catalog.py [--offline] [--roger-fixture DIR]
"""

from __future__ import annotations

import argparse
import json
import os
import sys
import urllib.parse
import urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import catalog_seed as seed  # noqa: E402

CACHE = HERE / "terminology-cache.json"
OUT = HERE / "catalog.json"
GYM_LABS = HERE / "gym-lab-ranges.json"

ICD = "http://hl7.org/fhir/sid/icd-10-cm"
LOINC = "http://loinc.org"
RXNORM = "http://www.nlm.nih.gov/research/umls/rxnorm"
SNOMED = "http://snomed.info/sct"
CPT = "http://www.ama-assn.org/go/cpt"
HCPCS = "https://www.cms.gov/Medicare/Coding/HCPCSReleaseCodeSets"

V2_0074 = {
    "RAD": "Radiology", "EC": "Electrocardiac (e.g., EKG, EEC, Holter)",
    "CUS": "Cardiac Ultrasound", "GE": "Genetics", "SP": "Surgical Pathology",
    "CP": "Cytopathology", "PF": "Pulmonary Function", "EN": "Electroneuro (EEG, EMG,EP,PSG)",
    "OUS": "OB Ultrasound", "AU": "Audiology", "LAB": "Laboratory",
}


class Lookup:
    def __init__(self, offline: bool) -> None:
        self.offline = offline
        self.cache = json.loads(CACHE.read_text()) if CACHE.exists() else {}

    def get(self, url: str, accept: str = "application/json"):
        if url in self.cache:
            return self.cache[url]
        if self.offline:
            raise RuntimeError(f"offline and not cached: {url}")
        req = urllib.request.Request(url, headers={"Accept": accept, "User-Agent": "coda-patient-bank"})
        with urllib.request.urlopen(req, timeout=60) as resp:
            value = json.load(resp)
        self.cache[url] = value
        return value

    def save(self) -> None:
        # authoring agents share the cache: keep their entries, never leave a partial file
        try:
            self.cache = json.loads(CACHE.read_text()) | self.cache
        except (FileNotFoundError, json.JSONDecodeError):
            pass
        tmp = CACHE.with_name(f"{CACHE.name}.{os.getpid()}.tmp")
        tmp.write_text(json.dumps(self.cache, indent=0, sort_keys=True))
        os.replace(tmp, CACHE)

    def hcpcs(self, code: str) -> str | None:
        url = "https://clinicaltables.nlm.nih.gov/api/hcpcs/v3/search?" + urllib.parse.urlencode(
            {"sf": "code", "terms": code, "df": "code,display", "maxList": 10})
        for found, name in self.get(url)[3]:
            if found == code:
                return name
        return None

    def icd10(self, code: str) -> str | None:
        url = "https://clinicaltables.nlm.nih.gov/api/icd10cm/v3/search?" + urllib.parse.urlencode(
            {"sf": "code", "terms": code, "maxList": 50})
        for found, name in self.get(url)[3]:
            if found == code:
                return name
        return None

    def loinc(self, code: str) -> str | None:
        url = "https://clinicaltables.nlm.nih.gov/api/loinc_items/v3/search?" + urllib.parse.urlencode(
            {"terms": code, "df": "LOINC_NUM,LONG_COMMON_NAME", "maxList": 50})
        for found, name in self.get(url)[3]:
            if found == code:
                return name
        return None

    def snomed(self, code: str) -> str | None:
        url = "https://tx.fhir.org/r4/CodeSystem/$lookup?" + urllib.parse.urlencode(
            {"system": SNOMED, "code": code})
        try:
            params = self.get(url, "application/fhir+json")
        except Exception:
            return None
        for p in params.get("parameter", []):
            if p.get("name") == "display":
                return p.get("valueString")
        return None

    def rxnorm_drugs(self, ingredient: str) -> list[dict]:
        url = "https://rxnav.nlm.nih.gov/REST/drugs.json?" + urllib.parse.urlencode({"name": ingredient})
        groups = self.get(url).get("drugGroup", {}).get("conceptGroup", []) or []
        return [c for g in groups if g.get("tty") in ("SCD", "GPCK") for c in g.get("conceptProperties", [])]


ROGER_FIXTURE = "submodules/coda-agents/docs/local-product-fixtures/internal_resources"


def build(offline: bool, roger: Path) -> int:
    look = Lookup(offline)
    problems: list[str] = []
    catalog: dict = {"systems": {"icd10cm": ICD, "loinc": LOINC, "rxnorm": RXNORM, "snomed": SNOMED, "cpt": CPT, "hcpcs": HCPCS}}
    gym: dict = {}
    for row in (json.loads(GYM_LABS.read_text()) if GYM_LABS.exists() else []):
        gym.setdefault(row["label"], row)  # first row wins: blood before urine, BMP before ABG
        if row["category"] == "Urine Studies":
            gym.setdefault("Urine " + row["label"], row)

    conditions = {}
    for label, code in seed.CONDITIONS.items():
        display = look.icd10(code)
        if display is None:
            problems.append(f"condition {label!r}: ICD-10-CM {code} not found")
            continue
        conditions[label] = {"system": ICD, "code": code, "display": display,
                             "receipt": "NLM Clinical Tables ICD-10-CM"}
    catalog["conditions"] = conditions

    meds = {}
    for ingredient, strengths, form in seed.MEDICATIONS:
        scds = look.rxnorm_drugs(ingredient)
        wanted = strengths or [None]
        for strength in wanted:
            match = []
            for c in scds:
                name = c["name"]
                if form.lower() not in name.lower():
                    continue
                if strength is not None and f" {strength} " not in f" {name} ":
                    continue
                match.append(c)
            # Prefer the shortest exact SCD name (no packaging qualifiers).
            match.sort(key=lambda c: (len(c["name"]), c["name"]))
            if not match:
                problems.append(f"medication {ingredient} {strength} {form}: no SCD")
                continue
            chosen = match[0]
            meds[chosen["name"]] = {"system": RXNORM, "code": chosen["rxcui"], "display": chosen["name"],
                                    "ingredient": ingredient, "receipt": "NLM RxNav drugs.json (SCD)"}
    for ingredient, exact in seed.EXACT_MEDICATIONS:
        found = [c for c in look.rxnorm_drugs(ingredient) if c["name"] == exact]
        if not found:
            problems.append(f"medication exact {exact!r}: not an RxNorm SCD/GPCK name")
            continue
        meds[exact] = {"system": RXNORM, "code": found[0]["rxcui"], "display": exact,
                       "ingredient": ingredient, "receipt": "NLM RxNav drugs.json (exact name)"}
    catalog["medications"] = meds

    labs = {}
    for label, (code, unit) in seed.LABS.items():
        display = look.loinc(code)
        if display is None:
            problems.append(f"lab {label!r}: LOINC {code} not found")
            continue
        entry = {"system": LOINC, "code": code, "display": display, "unit": unit,
                 "receipt": "NLM Clinical Tables LOINC"}
        g = gym.get(label)
        if g:
            entry["range"] = {k: g[k] for k in ("low", "high", "low_f", "high_f") if g.get(k) not in (None, "None")}
            entry["range_source"] = "coda-gym lab_reference_ranges.xlsx"
        labs[label] = entry
    catalog["labs"] = labs

    panels = {}
    for label, (code, members) in seed.PANELS.items():
        display = look.loinc(code)
        missing = [m for m in members if m not in labs]
        if display is None or missing:
            problems.append(f"panel {label!r}: LOINC {code} found={display is not None} missing={missing}")
            continue
        panels[label] = {"system": LOINC, "code": code, "display": display, "members": members}
    catalog["panels"] = panels

    for name, table in (("surveys", seed.SURVEYS), ("social", seed.SOCIAL)):
        out = {}
        for label, value in table.items():
            code, unit = value if isinstance(value, tuple) else (value, None)
            display = look.loinc(code)
            if display is None:
                problems.append(f"{name} {label!r}: LOINC {code} not found")
                continue
            out[label] = {"system": LOINC, "code": code, "display": display, "unit": unit}
        catalog[name] = out

    reports = {}
    for label, (code, section) in seed.REPORTS.items():
        display = look.loinc(code)
        if display is None:
            problems.append(f"report {label!r}: LOINC {code} not found")
            continue
        reports[label] = {"system": LOINC, "code": code, "display": display,
                          "category": {"code": section, "display": V2_0074[section]}}
    catalog["reports"] = reports

    # CPT has no public verification service. Codes harvested from the Coda
    # Agents Roger fixture keep that fixture's display; recalled codes carry no
    # display rather than an unverified one.
    fixture = roger / "Procedure.ndjson"
    fixture_display = {}
    if not fixture.exists():
        problems.append(f"Roger fixture not found at {fixture}; pass --roger-fixture")
    else:
        for line in fixture.read_text().splitlines():
            coding = json.loads(line)["code"]["coding"][0]
            fixture_display[coding["code"]] = coding["display"]
    procedures = {}
    for label, (code, prov) in seed.PROCEDURES.items():
        entry = {"system": CPT, "code": code, "provenance": prov}
        if prov == "hcpcs":  # HCPCS Level II is verifiable: NLM Clinical Tables
            display = look.hcpcs(code)
            if display is None:
                problems.append(f"procedure {label!r}: HCPCS {code} not found")
                continue
            entry = {"system": HCPCS, "code": code, "display": display, "provenance": "NLM Clinical Tables HCPCS"}
        elif prov == "fixture":
            if code not in fixture_display:
                problems.append(f"procedure {label!r}: CPT {code} not in the Roger fixture")
                continue
            entry["display"] = fixture_display[code]
        procedures[label] = entry
    catalog["procedures"] = procedures

    allergens = {}
    for label, (code, category) in seed.ALLERGENS.items():
        display = look.snomed(code)
        if display is None:
            problems.append(f"allergen {label!r}: SNOMED {code} not found")
            continue
        allergens[label] = {"system": SNOMED, "code": code, "display": display, "category": category}
    catalog["allergens"] = allergens
    reactions = {}
    for label, code in seed.REACTIONS.items():
        display = look.snomed(code)
        if display is None:
            problems.append(f"reaction {label!r}: SNOMED {code} not found")
            continue
        reactions[label] = {"system": SNOMED, "code": code, "display": display}
    catalog["reactions"] = reactions

    look.save()
    OUT.write_text(json.dumps(catalog, indent=1, sort_keys=True, ensure_ascii=False) + "\n")
    counts = {k: len(v) for k, v in catalog.items() if k != "systems"}
    print(json.dumps(counts))
    for p in problems:
        print("PROBLEM", p)
    return 1 if problems else 0


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--offline", action="store_true")
    ap.add_argument("--roger-fixture", type=Path, default=HERE.parents[1] / ROGER_FIXTURE,
                    help="coda-agents local-product-fixtures/internal_resources directory")
    a = ap.parse_args()
    sys.exit(build(a.offline, a.roger_fixture))
