"""Verify out-of-catalog conditions and drugs that authors proposed.

A proposed ICD-10-CM code is accepted only when NLM Clinical Tables returns it;
its official display replaces the author's. A drug written as text is accepted
only when RxNav resolves it to a clinical or branded drug whose name contains
the same ingredient word and every strength number in the text. Anything else
stays text-only and is counted as uncoded. Lookups share the catalog cache.
"""

from __future__ import annotations

import json
import re
import urllib.parse
from pathlib import Path

from build_catalog import Lookup


def _numbers(text: str) -> set[str]:
    return {n.rstrip("0").rstrip(".") if "." in n else n for n in re.findall(r"\d+(?:\.\d+)?", text)}


def resolve_drug(look: Lookup, text: str) -> dict | None:
    url = "https://rxnav.nlm.nih.gov/REST/approximateTerm.json?" + urllib.parse.urlencode(
        {"term": text, "maxEntries": 5})
    try:
        cands = look.get(url).get("approximateGroup", {}).get("candidate", []) or []
    except Exception:
        return None
    first = re.findall(r"[a-z]+", text.lower())
    if not first:
        return None
    for c in cands:
        rxcui = c.get("rxcui")
        if not rxcui:
            continue
        props = look.get(f"https://rxnav.nlm.nih.gov/REST/rxcui/{rxcui}/properties.json").get("properties") or {}
        if props.get("tty") not in ("SCD", "SBD", "GPCK", "BPCK"):
            continue
        name = props.get("name", "")
        if first[0] in name.lower() and _numbers(text) <= _numbers(name):
            return {"code": rxcui, "display": name, "ingredient": first[0],
                    "receipt": "NLM RxNav approximateTerm + properties"}
    return None


def resolve_extras(ledgers: Path, catalog: dict) -> dict:
    look = Lookup(offline=False)
    out = {"conditions": {}, "medications": {}, "rejected": []}
    for path in sorted(ledgers.glob("*.json")):
        led = json.loads(path.read_text())
        for p in led.get("patients", []):
            for ev in p.get("events", []):
                for c in ev.get("conditions", []):
                    label, code = c.get("label"), c.get("icd10")
                    if label in catalog["conditions"] or not code:
                        continue
                    key = f"{label}|{code}"
                    if key in out["conditions"]:
                        continue
                    display = look.icd10(code.strip().upper())
                    if display:
                        out["conditions"][key] = {"code": code.strip().upper(), "display": display,
                                                  "receipt": "NLM Clinical Tables ICD-10-CM"}
                    else:
                        out["rejected"].append(f"condition {label!r} {code}")
                for m in ev.get("medications", []):
                    text = m.get("drug_text")
                    if not text or text in out["medications"]:
                        continue
                    hit = resolve_drug(look, text)
                    if hit:
                        out["medications"][text] = hit
                    else:
                        out["rejected"].append(f"drug {text!r}")
    look.save()
    return out
