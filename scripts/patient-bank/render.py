#!/usr/bin/env python3
"""Translate authored patient ledgers into strict FHIR R4 NDJSON.

The ledger is the clinical source of truth; this module only encodes it, in the
shape of the Coda Agents Roger fixture and coda-gym's chart-build contract. It
takes an as-of instant, so the bank chart (as of the practice's current time) and
any earlier snapshot come from the same code: events and records after the
instant do not exist yet, and statuses are those true at the instant.

Code here never chooses a diagnosis, drug or result that the ledger did not
state. It does fill unstated members of an ordered lab panel with values inside
the reference range, and it computes derived values (eGFR, anion gap, LDL,
non-HDL, BMI, indices) from stated ones; each such Observation is listed in the
patient's render receipt.
"""

from __future__ import annotations

import base64
import hashlib
import json
import math
import random
import re
from dataclasses import dataclass, field
from datetime import date, datetime, timedelta, timezone
from pathlib import Path
from zoneinfo import ZoneInfo

HERE = Path(__file__).resolve().parent
EASTERN = ZoneInfo("America/New_York")
MRN_SYSTEM = "urn:oid:2.16.840.1.113883.19.5"
UCUM = "http://unitsofmeasure.org"
OBS_CAT = "http://terminology.hl7.org/CodeSystem/observation-category"
V2_0074 = "http://terminology.hl7.org/CodeSystem/v2-0074"
FAMILIES = ("Patient", "Condition", "MedicationRequest", "Observation", "DiagnosticReport",
            "DocumentReference", "Procedure", "AllergyIntolerance")

VITALS = {  # coda-gym authoring/references/vital_fields.md
    "hr": ("8867-4", "Heart rate", "Heart Rate", "/min", "/min"),
    "rr": ("9279-1", "Respiratory rate", "Respiratory Rate", "/min", "/min"),
    "temp_c": ("8310-5", "Body temperature", "Temperature", "Cel", "C"),
    "spo2": ("59408-5", "Oxygen saturation in Arterial blood by Pulse oximetry", "Oxygen Saturation", "%", "%"),
    "weight_kg": ("29463-7", "Body weight", "Weight", "kg", "kg"),
    "height_cm": ("8302-2", "Body height", "Height", "cm", "cm"),
    "bmi": ("39156-5", "Body mass index (BMI) [Ratio]", "Body Mass Index", "kg/m2", "kg/m2"),
    "peak_flow": ("33452-4", "Maximum expiratory gas flow Respiratory system airway by Peak flow meter",
                  "Peak Expiratory Flow", "L/min", "L/min"),
}
BP = ("85354-9", "Blood pressure panel with all children optional", "Blood Pressure")
SBP = ("8480-6", "Systolic blood pressure", "Systolic Blood Pressure")
DBP = ("8462-4", "Diastolic blood pressure", "Diastolic Blood Pressure")

DOC_TYPE_CODES = {  # accurate LOINC document classes only; others stay text-only
    "History and Physical": ("34117-2", "History and physical note"),
    "Discharge Summary": ("18842-5", "Discharge summary"),
    "Consults": ("11488-4", "Consult note"),
}

MARITAL = {"married": ("M", "Married"), "single": ("S", "Never Married"), "divorced": ("D", "Divorced"),
           "widowed": ("W", "Widowed"), "partnered": ("T", "Domestic partner"),
           "separated": ("L", "Legally Separated")}
LANG = {"English": "en", "Spanish": "es", "Vietnamese": "vi", "Mandarin": "zh", "Korean": "ko",
        "Khmer": "km", "Hindi": "hi", "Portuguese": "pt", "Russian": "ru", "Haitian Creole": "ht",
        "Polish": "pl"}
RACE = {"White": "2106-3", "Black or African American": "2054-5", "Asian": "2028-9",
        "Other Race": "2131-1"}
ETHNICITY = {"Hispanic or Latino": "2135-2", "Not Hispanic or Latino": "2186-5"}
US_CORE = "http://hl7.org/fhir/us/core/StructureDefinition"
CDC_RACE = "urn:oid:2.16.840.1.113883.6.238"

# Normal ranges used to fill unstated panel members: (low, high, decimals).
FILL = {
    "White Blood Cells": (4.5, 9.5, 1),
    "Hemoglobin": ((13.8, 16.6), (12.2, 15.2), 1), "Red Cell Distribution Width": (11.8, 14.0, 1),
    "Platelets": (170, 360, 0), "Mean Platelet Volume": (8.0, 11.0, 1), "Mean Corpuscular Volume": (84, 96, 0),
    "Neutrophils %": (48, 68, 0), "Lymphocytes %": (22, 38, 0), "Monocytes %": (4, 9, 0),
    "Eosinophils %": (0.5, 5, 0), "Basophils %": (0, 1, 0),
    "Sodium": (137, 143, 0), "Potassium": (3.7, 4.8, 1), "Chloride": (99, 106, 0), "Bicarbonate": (23, 29, 0),
    "Blood Urea Nitrogen": (9, 20, 0), "Creatinine": ((0.75, 1.15), (0.55, 0.95), 2), "Glucose": (78, 104, 0),
    "Calcium": (8.9, 10.1, 1), "Albumin": (3.9, 4.8, 1), "Total Protein": (6.4, 7.8, 1),
    "Aspartate Aminotransferase": (14, 32, 0), "Alanine Aminotransferase": (11, 34, 0),
    "Alkaline Phosphatase": (48, 110, 0), "Bilirubin, Total": (0.3, 1.0, 1), "Bilirubin, Direct": (0.0, 0.3, 1),
    "High Density Lipoprotein": ((40, 58), (50, 72), 0), "Triglycerides": (65, 145, 0),
    "Low Density Lipoprotein": (85, 125, 0), "Iron": (65, 150, 0), "Total Iron-Binding Capacity": (260, 380, 0),
    "Ferritin": ((40, 260), (20, 140), 0), "Prothrombin Time": (11.2, 13.2, 1),
    "Prothrombin Time-International Normalized Ratio": (0.9, 1.1, 1),
    "Urine Specific Gravity": (1.008, 1.024, 3), "Urine pH": (5.0, 7.0, 1),
    "Urine White Blood Cells": (0, 2, 0), "Urine Red Blood Cells": (0, 2, 0),
}
FILL_TEXT = {"Urine Color": "Yellow", "Urine Clarity": "Clear", "Urine Protein": "Negative",
             "Urine Glucose": "Negative", "Urine Ketones": "Negative", "Urine Blood": "Negative",
             "Urine Leukocyte Esterase": "Negative", "Urine Nitrite": "Negative"}
DIFFERENTIAL = ("Neutrophils %", "Lymphocytes %", "Monocytes %", "Eosinophils %", "Basophils %")
DERIVED = {"Estimated Glomerular Filtration Rate (Creatinine)", "Anion Gap", "Non-HDL Cholesterol",
           "Total Cholesterol", "Hematocrit", "Red Blood Cells", "Mean Corpuscular Hemoglobin",
           "Mean Corpuscular Hemoglobin Concentration", "Absolute Neutrophil Count",
           "Transferrin Saturation"}


def load_catalog() -> dict:
    return json.loads((HERE / "catalog.json").read_text())


def to_utc(local: str) -> str:
    """Eastern wall-clock 'YYYY-MM-DDTHH:MM' -> 'YYYY-MM-DDTHH:MM:SS+00:00'."""
    dt = datetime.fromisoformat(local[:16]).replace(tzinfo=EASTERN)
    return dt.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%S+00:00")


def local_dt(local: str) -> datetime:
    return datetime.fromisoformat(local[:16]).replace(tzinfo=EASTERN)


def fmt_local(dt: datetime) -> str:
    return dt.astimezone(EASTERN).strftime("%Y-%m-%dT%H:%M")


def last_updated(utc: str) -> str:
    return utc[:10] + "T00:00:00+00:00"


def opaque(prefix: str, *parts: str) -> str:
    return f"{prefix}-{hashlib.sha256('|'.join(parts).encode()).hexdigest()[:12]}"


def b64(text: str) -> str:
    return base64.b64encode(text.encode("utf-8")).decode("ascii")


def age_on(birth: date, on: date) -> int:
    return on.year - birth.year - ((on.month, on.day) < (birth.month, birth.day))


def _percent(m: re.Match) -> str:
    pct = float(m.group(1)) * (100 if m.group(2).upper() == "MG/MG" else 0.1)
    return f"{pct:g}%"


def friendly_drug(name: str) -> str:
    name = re.sub(r"^(NDA\d+ )?(\d+ ACTUAT |[\d.]+ ML )?", "", name)  # application number, pack size, pen volume
    if re.search(r"\b(Topical|Cream|Gel|Ointment|Lotion)\b", name, re.I):  # topical strengths read as percent
        name = re.sub(r"(\d*\.?\d+) (MG/MG|MG/ML)\b", _percent, name)
    text = re.sub(r"\b(MG|MCG|ML|UNT|MEQ|ACTUAT|HR|GM)\b", lambda m: m.group(1).lower(), name)
    return re.sub(r"\b(Oral|Tablet|Capsule|Extended|Release|Delayed|Topical|Cream|Ointment|Gel|Injector|"
                  r"Pen|Auto-Injector|Solution|Spray|Nasal|Inhaler|Metered|Dose|Dry|Powder|Sublingual|"
                  r"Film|Disintegrating|Ophthalmic|Vaginal|Intrauterine|System|Injection|Transdermal|"
                  r"Suspension|Inhalation|Pack)\b", lambda m: m.group(1).lower(), text)


def ckd_epi_2021(scr: float, age: int, female: bool) -> int:
    k, a = (0.7, -0.241) if female else (0.9, -0.302)
    egfr = 142 * min(scr / k, 1) ** a * max(scr / k, 1) ** -1.200 * 0.9938 ** age * (1.012 if female else 1)
    return int(round(egfr))


@dataclass
class Receipt:
    filled: list[str] = field(default_factory=list)
    derived: list[str] = field(default_factory=list)
    uncoded: list[str] = field(default_factory=list)
    pending: list[str] = field(default_factory=list)
    skipped: list[str] = field(default_factory=list)


class Renderer:
    def __init__(self, catalog: dict, extra_terms: dict | None = None) -> None:
        self.c = catalog
        self.extra = extra_terms or {"conditions": {}, "medications": {}}

    # ---- terminology -------------------------------------------------------
    def condition_code(self, item: dict, receipt: Receipt) -> dict:
        label = item["label"]
        entry = self.c["conditions"].get(label)
        if entry:
            return {"coding": [{"system": entry["system"], "code": entry["code"], "display": entry["display"]}],
                    "text": label}
        extra = self.extra["conditions"].get(f"{label}|{item.get('icd10', '')}")
        if extra:
            return {"coding": [{"system": self.c["systems"]["icd10cm"], "code": extra["code"],
                                "display": extra["display"]}], "text": label}
        receipt.uncoded.append(f"Condition:{label}")
        return {"text": label}

    def medication_concept(self, item: dict, receipt: Receipt) -> tuple[dict, str]:
        name = item.get("drug")
        if name and name in self.c["medications"]:
            e = self.c["medications"][name]
            return ({"coding": [{"system": e["system"], "code": e["code"], "display": e["display"]}],
                     "text": friendly_drug(name)}, e["ingredient"])
        text = name or item.get("drug_text") or "unspecified medication"
        extra = self.extra["medications"].get(text)
        if extra:
            return ({"coding": [{"system": self.c["systems"]["rxnorm"], "code": extra["code"],
                                 "display": extra["display"]}], "text": text}, extra.get("ingredient", text))
        receipt.uncoded.append(f"MedicationRequest:{text}")
        return {"text": text}, text.split()[0].lower()

    # ---- patient ------------------------------------------------------------
    def patient(self, seed: dict, led: dict) -> dict:
        mrn = seed["mrn"]
        res = {
            "resourceType": "Patient", "id": mrn,
            "meta": {"versionId": "1", "lastUpdated": "2026-08-27T00:00:00+00:00"},
            "identifier": [{"system": MRN_SYSTEM, "value": mrn[3:]}],
            "name": [{"use": "official", "family": seed["family"], "given": [seed["given"]]}],
            "gender": seed["sex"], "birthDate": seed["birth_date"],
            "telecom": [{"system": "phone", "value": seed["phone"], "use": "mobile"}],
            "address": [{"use": "home", "line": [led.get("address_line") or "1 Main Street"],
                         "city": seed["town"], "state": "MA", "postalCode": seed["postal_code"], "country": "US"}],
        }
        if seed.get("email"):
            res["telecom"].append({"system": "email", "value": seed["email"], "use": "home"})
        ms = MARITAL.get((led.get("marital_status") or "").lower())
        if ms:
            res["maritalStatus"] = {"coding": [{"system": "http://terminology.hl7.org/CodeSystem/v3-MaritalStatus",
                                                "code": ms[0], "display": ms[1]}], "text": ms[1]}
        lang = seed.get("preferred_language", "English")
        if lang != "English" and lang in LANG:
            res["communication"] = [{"language": {"coding": [{"system": "urn:ietf:bcp:47", "code": LANG[lang],
                                                              "display": lang}], "text": lang}, "preferred": True}]
        ext = []
        race = seed.get("race")
        if race and race != "Declined":
            sub = [{"url": "text", "valueString": race}]
            if race in RACE:
                sub.insert(0, {"url": "ombCategory", "valueCoding": {"system": CDC_RACE, "code": RACE[race],
                                                                     "display": race}})
            ext.append({"url": f"{US_CORE}/us-core-race", "extension": sub})
        eth = seed.get("ethnicity")
        if eth in ETHNICITY:
            ext.append({"url": f"{US_CORE}/us-core-ethnicity", "extension": [
                {"url": "ombCategory", "valueCoding": {"system": CDC_RACE, "code": ETHNICITY[eth], "display": eth}},
                {"url": "text", "valueString": eth}]})
        if ext:
            res["extension"] = ext
        return res

    # ---- main ---------------------------------------------------------------
    def render(self, seed: dict, led: dict, as_of: str) -> tuple[dict[str, list[dict]], Receipt]:
        """Return resources by family as of `as_of` (Eastern 'YYYY-MM-DDTHH:MM')."""
        cutoff = local_dt(as_of)
        mrn = seed["mrn"]
        subject = {"reference": f"Patient/{mrn}"}
        female = seed["sex"] == "female"
        birth = date.fromisoformat(seed["birth_date"])
        receipt = Receipt()
        out: dict[str, list[dict]] = {f: [] for f in FAMILIES}
        out["Patient"].append(self.patient(seed, led))
        rng_base = int(hashlib.sha256(mrn.encode()).hexdigest()[:8], 16)
        baseline: dict[str, float] = {}
        events = sorted(led["events"], key=lambda e: e["start"])
        events = [e for e in events if local_dt(e["start"]) < cutoff]

        # Problem list: diagnosis or history items, resolved by a later action.
        resolved_at: dict[str, str] = {}
        for ev in events:
            for item in ev.get("conditions", []):
                if item.get("action") == "resolved":
                    when = item.get("date") or ev["start"]
                    if local_dt(when) < cutoff:
                        resolved_at.setdefault(item["label"], when)
        seen_conditions: set[str] = set()
        # Medication orders: started here or recorded as historical; stopped later.
        stops: dict[str, str] = {}
        for ev in events:
            for item in ev.get("medications", []):
                if item.get("action") == "stop":
                    key = item.get("drug") or item.get("drug_text")
                    when = item.get("date") or ev["start"]
                    if local_dt(when) < cutoff:
                        stops.setdefault(key, when)
        height = None

        for ev in events:
            eid = ev["id"]
            start = ev["start"]
            start_utc = to_utc(start)
            for i, item in enumerate(ev.get("conditions", [])):
                if item.get("action") not in ("diagnosed", "history"):
                    continue
                label = item["label"]
                if label in seen_conditions:
                    continue  # one problem-list row per concept
                seen_conditions.add(label)
                res = {"resourceType": "Condition", "id": opaque("cond", mrn, eid, label),
                       "meta": {"versionId": "1", "lastUpdated": last_updated(start_utc)},
                       "clinicalStatus": None,
                       "verificationStatus": {"coding": [{"system": "http://terminology.hl7.org/CodeSystem/condition-ver-status",
                                                          "code": "confirmed", "display": "Confirmed"}]},
                       "category": [{"coding": [{"system": "http://terminology.hl7.org/CodeSystem/condition-category",
                                                 "code": "problem-list-item", "display": "Problem List Item"}],
                                     "text": "Problem List Item"}],
                       "code": self.condition_code(item, receipt), "subject": subject}
                onset = item.get("onset")
                if onset and onset[:10] < start[:10]:
                    res["onsetDateTime"] = onset[:10] if len(onset) <= 10 else to_utc(onset)
                else:
                    res["onsetDateTime"] = start_utc
                res["recordedDate"] = start_utc
                if label in resolved_at and resolved_at[label] >= start:
                    res["clinicalStatus"] = {"coding": [{"system": "http://terminology.hl7.org/CodeSystem/condition-clinical",
                                                         "code": "resolved", "display": "Resolved"}]}
                    res["abatementDateTime"] = to_utc(resolved_at[label])
                else:
                    res["clinicalStatus"] = {"coding": [{"system": "http://terminology.hl7.org/CodeSystem/condition-clinical",
                                                         "code": "active", "display": "Active"}]}
                if item.get("note"):
                    res["note"] = [{"text": item["note"]}]
                out["Condition"].append(res)

            for i, item in enumerate(ev.get("medications", [])):
                if item.get("action") not in ("start", "historical"):
                    continue
                key = item.get("drug") or item.get("drug_text")
                concept, _ = self.medication_concept(item, receipt)
                authored = item.get("date") or start
                res = {"resourceType": "MedicationRequest", "id": opaque("medrx", mrn, eid, str(i), key),
                       "meta": {"versionId": "1", "lastUpdated": last_updated(to_utc(authored))},
                       "status": "active", "intent": "order", "medicationCodeableConcept": concept,
                       "subject": subject, "authoredOn": to_utc(authored)}
                if item.get("action") == "historical":
                    res["reportedBoolean"] = True
                stop = stops.get(key)
                if stop and stop >= authored:
                    res["status"] = "stopped"
                    res["statusReason"] = {"text": f"Discontinued {stop[:10]}"}
                elif item.get("course") == "short":
                    if cutoff - local_dt(authored) > timedelta(days=21):
                        res["status"] = "completed"
                dose = {}
                sig = item.get("sig") or item.get("frequency") or ""
                dose["text"] = sig
                if item.get("frequency"):
                    dose["timing"] = {"code": {"text": item["frequency"]}}
                d = item.get("dose")
                if isinstance(d, dict) and isinstance(d.get("value"), (int, float)):
                    dose["doseAndRate"] = [{"doseQuantity": {"value": d["value"], "unit": d.get("unit", "")}}]
                if item.get("route"):
                    dose["route"] = {"text": item["route"]}
                if any(v for v in dose.values()):
                    res["dosageInstruction"] = [dose]
                if item.get("reason"):
                    res["reasonCode"] = [{"text": item["reason"]}]
                out["MedicationRequest"].append(res)

            for i, item in enumerate(ev.get("allergies", [])):
                entry = self.c["allergens"].get(item.get("substance"))
                code = ({"coding": [{"system": entry["system"], "code": entry["code"], "display": entry["display"]}],
                         "text": item["substance"]} if entry else {"text": item.get("substance", "Unknown")})
                if not entry:
                    receipt.uncoded.append(f"AllergyIntolerance:{item.get('substance')}")
                res = {"resourceType": "AllergyIntolerance", "id": opaque("allergy", mrn, eid, str(i)),
                       "meta": {"versionId": "1", "lastUpdated": last_updated(start_utc)},
                       "clinicalStatus": {"coding": [{"system": "http://terminology.hl7.org/CodeSystem/allergyintolerance-clinical",
                                                      "code": "active", "display": "Active"}]},
                       "verificationStatus": {"coding": [{"system": "http://terminology.hl7.org/CodeSystem/allergyintolerance-verification",
                                                          "code": "confirmed", "display": "Confirmed"}]},
                       "type": item.get("type", "allergy") if item.get("type") in ("allergy", "intolerance") else "allergy",
                       "category": [entry["category"] if entry else "medication"],
                       "criticality": item.get("criticality", "low") if item.get("criticality") in ("low", "high", "unable-to-assess") else "low",
                       "code": code, "patient": subject, "recordedDate": start_utc}
                rx = self.c["reactions"].get(item.get("reaction"))
                if rx or item.get("reaction"):
                    man = ({"coding": [{"system": rx["system"], "code": rx["code"], "display": rx["display"]}],
                            "text": item["reaction"]} if rx else {"text": item["reaction"]})
                    reaction = {"manifestation": [man]}
                    if item.get("severity") in ("mild", "moderate", "severe"):
                        reaction["severity"] = item["severity"]
                    res["reaction"] = [reaction]
                out["AllergyIntolerance"].append(res)

            vit_sets = ev.get("vitals") or []
            if isinstance(vit_sets, dict):
                vit_sets = [vit_sets]
            for vi, vit in enumerate(vit_sets):  # a stay records several timed sets
                tag = (eid,) if len(vit_sets) == 1 and not vit.get("time") else (eid, str(vi))
                if vit.get("time"):
                    if local_dt(vit["time"]) >= cutoff:
                        continue
                    when = to_utc(vit["time"])
                else:
                    when = to_utc(fmt_local(local_dt(start) + timedelta(minutes=4 + rng_base % 9)))
                obs_cat = [{"coding": [{"system": OBS_CAT, "code": "vital-signs", "display": "Vital Signs"}]}]
                if isinstance(vit.get("bp"), list) and len(vit["bp"]) == 2:
                    out["Observation"].append({
                        "resourceType": "Observation", "id": opaque("obs", mrn, *tag, "bp"),
                        "meta": {"versionId": "1", "lastUpdated": last_updated(when)}, "status": "final",
                        "category": obs_cat, "code": {"coding": [{"system": "http://loinc.org", "code": BP[0], "display": BP[1]}], "text": BP[2]},
                        "subject": subject, "effectiveDateTime": when,
                        "component": [
                            {"code": {"coding": [{"system": "http://loinc.org", "code": SBP[0], "display": SBP[1]}], "text": SBP[2]},
                             "valueQuantity": {"value": vit["bp"][0], "unit": "mmHg", "system": UCUM, "code": "mm[Hg]"}},
                            {"code": {"coding": [{"system": "http://loinc.org", "code": DBP[0], "display": DBP[1]}], "text": DBP[2]},
                             "valueQuantity": {"value": vit["bp"][1], "unit": "mmHg", "system": UCUM, "code": "mm[Hg]"}}]})
                if isinstance(vit.get("height_cm"), (int, float)):
                    height = vit["height_cm"]
                values = {k: v for k, v in vit.items() if k != "time"}
                if "bmi" not in values and isinstance(values.get("weight_kg"), (int, float)) and height:
                    values["bmi"] = round(values["weight_kg"] / (height / 100) ** 2, 1)
                    receipt.derived.append(f"{':'.join(tag)}:Body Mass Index")
                for key, (code, disp, text, ucum, unit) in VITALS.items():
                    v = values.get(key)
                    if not isinstance(v, (int, float)):
                        continue
                    out["Observation"].append({
                        "resourceType": "Observation", "id": opaque("obs", mrn, *tag, key),
                        "meta": {"versionId": "1", "lastUpdated": last_updated(when)}, "status": "final",
                        "category": obs_cat,
                        "code": {"coding": [{"system": "http://loinc.org", "code": code, "display": disp}], "text": text},
                        "subject": subject, "effectiveDateTime": when,
                        "valueQuantity": {"value": v, "unit": unit, "system": UCUM, "code": ucum}})

            for i, panel_item in enumerate(ev.get("labs", [])):
                self._labs(out, receipt, mrn, subject, eid, i, panel_item, ev, cutoff, female, birth,
                           rng_base, baseline)

            for i, s in enumerate(ev.get("surveys", [])):
                entry = self.c["surveys"].get(s.get("survey"))
                if not entry or not isinstance(s.get("value"), (int, float)):
                    receipt.skipped.append(f"{eid}:survey:{s.get('survey')}")
                    continue
                when = to_utc(s.get("date") or start)
                out["Observation"].append({
                    "resourceType": "Observation", "id": opaque("obs", mrn, eid, "survey", str(i)),
                    "meta": {"versionId": "1", "lastUpdated": last_updated(when)}, "status": "final",
                    "category": [{"coding": [{"system": OBS_CAT, "code": "survey", "display": "Survey"}]}],
                    "code": {"coding": [{"system": "http://loinc.org", "code": entry["code"], "display": entry["display"]}], "text": s["survey"]},
                    "subject": subject, "effectiveDateTime": when,
                    "valueQuantity": {"value": s["value"], "unit": "{score}", "system": UCUM, "code": "{score}"}})
            for i, s in enumerate(ev.get("social", [])):
                entry = self.c["social"].get(s.get("item"))
                if not entry or not s.get("value"):
                    receipt.skipped.append(f"{eid}:social:{s.get('item')}")
                    continue
                out["Observation"].append({
                    "resourceType": "Observation", "id": opaque("obs", mrn, eid, "social", str(i)),
                    "meta": {"versionId": "1", "lastUpdated": last_updated(start_utc)}, "status": "final",
                    "category": [{"coding": [{"system": OBS_CAT, "code": "social-history", "display": "Social History"}]}],
                    "code": {"coding": [{"system": "http://loinc.org", "code": entry["code"], "display": entry["display"]}], "text": s["item"]},
                    "subject": subject, "effectiveDateTime": start_utc,
                    "valueCodeableConcept": {"text": str(s["value"])}})

            for i, rep in enumerate(ev.get("reports", [])):
                self._report(out, receipt, mrn, subject, eid, i, rep, ev, cutoff)

            for i, p in enumerate(ev.get("procedures", [])):
                entry = self.c["procedures"].get(p.get("procedure"))
                when = p.get("performed") or start
                if local_dt(when) >= cutoff:
                    continue
                if entry:
                    coding = {"system": entry["system"], "code": entry["code"]}
                    if entry.get("display"):
                        coding["display"] = entry["display"]
                    code = {"coding": [coding], "text": p["procedure"]}
                else:
                    receipt.uncoded.append(f"Procedure:{p.get('procedure')}")
                    code = {"text": p.get("procedure", "Procedure")}
                out["Procedure"].append({
                    "resourceType": "Procedure", "id": opaque("proc", mrn, eid, str(i)),
                    "meta": {"versionId": "1", "lastUpdated": last_updated(to_utc(when))}, "status": "completed",
                    "code": code, "subject": subject, "performedDateTime": to_utc(when)})
            for i, imm in enumerate(ev.get("immunizations", [])):
                when = imm.get("date") or start
                if local_dt(when) >= cutoff:
                    continue
                entry = self.c["procedures"]["Immunization administration"]
                out["Procedure"].append({
                    "resourceType": "Procedure", "id": opaque("proc", mrn, eid, "imm", str(i)),
                    "meta": {"versionId": "1", "lastUpdated": last_updated(to_utc(when))}, "status": "completed",
                    "code": {"coding": [{"system": entry["system"], "code": entry["code"], "display": entry["display"]}],
                             "text": f"{imm.get('vaccine', 'Vaccine')} vaccine administration"},
                    "subject": subject, "performedDateTime": to_utc(when)})

            for i, doc in enumerate(ev.get("documents", [])):
                when = doc.get("date") or start
                if local_dt(when) >= cutoff:
                    receipt.pending.append(f"{eid}:document after cutoff")
                    continue
                d = local_dt(when).date()
                header = (f"{doc.get('title') or doc.get('type')}\n"
                          f"Date: {d.strftime('%m/%d/%Y')}\n"
                          f"Patient: {seed['given']} {seed['family']} | MRN: {mrn[3:]} | Age: {age_on(birth, d)}\n"
                          f"Service: {ev.get('service') or 'Internal Medicine — Primary Care'}\n\n")
                text = header + doc["body"].strip() + "\n"
                dtype = doc.get("type") or "Progress Notes"
                type_cc: dict = {"text": dtype}
                for key, (code, disp) in DOC_TYPE_CODES.items():
                    if dtype == key or dtype.endswith(key):
                        type_cc["coding"] = [{"system": "http://loinc.org", "code": code, "display": disp}]
                out["DocumentReference"].append({
                    "resourceType": "DocumentReference", "id": opaque("docref", mrn, eid, str(i)),
                    "meta": {"versionId": "1", "lastUpdated": last_updated(to_utc(when))},
                    "status": "current", "docStatus": "preliminary" if doc.get("status") == "preliminary" else "final",
                    "type": type_cc,
                    "category": [{"coding": [{"system": "http://hl7.org/fhir/us/core/CodeSystem/us-core-documentreference-category",
                                              "code": "clinical-note", "display": "Clinical Note"}]}],
                    "subject": subject, "date": to_utc(when), "description": doc.get("title") or dtype,
                    "content": [{"attachment": {"contentType": "text/plain; charset=utf-8", "data": b64(text)}}]})
        return out, receipt

    # ---- labs ---------------------------------------------------------------
    def _value(self, label: str, female: bool, rng: random.Random, baseline: dict) -> float | None:
        spec = FILL.get(label)
        if spec is None:
            return None
        lo, hi, dec = spec
        if isinstance(lo, tuple):
            lo, hi = (hi if female else lo)
        if label not in baseline:
            baseline[label] = rng.uniform(lo + (hi - lo) * 0.2, hi - (hi - lo) * 0.2)
        v = baseline[label] + rng.gauss(0, (hi - lo) * 0.15)
        v = min(max(v, lo), hi)
        return round(v, dec) if dec else int(round(v))

    def _quantity(self, label: str, value: float) -> dict:
        unit = self.c["labs"][label].get("unit")
        q: dict = {"value": value}
        if unit:
            q.update({"unit": unit, "system": UCUM, "code": unit})
        return q

    def _ref_range(self, label: str, female: bool) -> list[dict] | None:
        r = self.c["labs"][label].get("range")
        if not r:
            return None
        lo = r.get("low_f") if female and r.get("low_f") not in (None, "") else r.get("low")
        hi = r.get("high_f") if female and r.get("high_f") not in (None, "") else r.get("high")
        try:
            lo, hi = float(lo), float(hi)
        except (TypeError, ValueError):
            return None
        unit = self.c["labs"][label].get("unit")
        rr: dict = {"low": {"value": lo}, "high": {"value": hi}}
        if unit:
            for k in ("low", "high"):
                rr[k].update({"unit": unit, "system": UCUM, "code": unit})
        return [rr]

    def _labs(self, out, receipt, mrn, subject, eid, i, item, ev, cutoff, female, birth, rng_base, baseline):
        collected = item.get("collected") or ev["start"]
        if local_dt(collected) >= cutoff:
            return
        panel_name = item.get("panel")
        panel = self.c["panels"].get(panel_name) if panel_name else None
        stated = {r["test"]: r for r in item.get("results", []) if r.get("test")}
        unknown = [t for t in stated if t not in self.c["labs"]]
        for t in unknown:
            receipt.skipped.append(f"{eid}:lab:{t}")
        stated = {t: r for t, r in stated.items() if t in self.c["labs"]}
        slow = any(t in ("Urine Culture", "Blood Culture") for t in stated)
        rng = random.Random(rng_base + int(hashlib.sha256(f"{eid}{i}".encode()).hexdigest()[:6], 16))
        delay = timedelta(hours=46 + rng.randint(0, 20)) if slow else timedelta(minutes=95 + rng.randint(0, 300))
        issued_local = local_dt(item["resulted"]) if item.get("resulted") else local_dt(collected) + delay
        pending = issued_local >= cutoff
        eff = to_utc(collected)
        issued = to_utc(fmt_local(issued_local))
        values: dict[str, dict] = {}
        members = panel["members"] if panel else list(stated)
        for t in members:
            r = stated.get(t)
            if r is not None:
                values[t] = r
        if panel:
            age = age_on(birth, local_dt(collected).date())
            num = lambda t: values[t].get("value") if t in values and isinstance(values[t].get("value"), (int, float)) else None  # noqa: E731
            diff_open = [t for t in DIFFERENTIAL if t in members and t not in values]
            if "Mean Corpuscular Volume" in members and "Mean Corpuscular Volume" not in values \
                    and num("Hematocrit") and num("Red Blood Cells"):
                values["Mean Corpuscular Volume"] = {"value": int(round(num("Hematocrit") / num("Red Blood Cells") * 10)), "_derived": True}
            for t in members:
                if t in values or t in DERIVED or t in diff_open:
                    continue
                if t in FILL_TEXT:
                    values[t] = {"text": FILL_TEXT[t], "_filled": True}
                    continue
                v = self._value(t, female, rng, baseline)
                if v is not None:
                    values[t] = {"value": v, "_filled": True}
            if diff_open:
                # unstated differential members share what the stated ones leave of 100%
                room = 100 - sum(num(t) or 0 for t in DIFFERENTIAL if t in values)
                share = {t: self._value(t, female, rng, baseline) or 0 for t in diff_open}
                total = sum(share.values()) or 1
                fill = {t: max(0, int(round(room * v / total))) for t, v in share.items()}
                biggest = max(fill, key=lambda t: share[t])
                fill[biggest] = max(0, fill[biggest] + int(round(room)) - sum(fill.values()))
                for t, v in fill.items():
                    values[t] = {"value": v, "_filled": True}
            # Derived members, computed only from values present in this draw. Red-cell
            # indices come from one consistent set: Hgb, MCV and MCHC give Hct and RBC.
            hgb, mcv = num("Hemoglobin"), num("Mean Corpuscular Volume")
            hct, rbc = num("Hematocrit"), num("Red Blood Cells")
            if hgb and mcv:
                if hct is None and rbc is not None:
                    hct = round(rbc * mcv / 10, 1)
                if hct is None:
                    mchc = baseline.setdefault("_mchc", rng.uniform(32.6, 34.2)) + rng.uniform(-0.3, 0.3)
                    hct = round(hgb / mchc * 100, 1)
                if rbc is None:
                    rbc = round(hct / mcv * 10, 2)
                for t, v in (("Hematocrit", hct), ("Red Blood Cells", rbc)):
                    if t in members and t not in values:
                        values[t] = {"value": v, "_derived": True}
            if "Mean Corpuscular Hemoglobin" in members and "Mean Corpuscular Hemoglobin" not in values \
                    and num("Hemoglobin") and num("Red Blood Cells"):
                values["Mean Corpuscular Hemoglobin"] = {"value": round(num("Hemoglobin") / num("Red Blood Cells") * 10, 1), "_derived": True}
            if "Mean Corpuscular Hemoglobin Concentration" in members and "Mean Corpuscular Hemoglobin Concentration" not in values \
                    and num("Hemoglobin") and num("Hematocrit"):
                values["Mean Corpuscular Hemoglobin Concentration"] = {"value": round(num("Hemoglobin") / num("Hematocrit") * 100, 1), "_derived": True}
            if "Absolute Neutrophil Count" in members and "Absolute Neutrophil Count" not in values \
                    and num("White Blood Cells") and num("Neutrophils %"):
                values["Absolute Neutrophil Count"] = {"value": round(num("White Blood Cells") * num("Neutrophils %") / 100, 1), "_derived": True}
            egfr = "Estimated Glomerular Filtration Rate (Creatinine)"
            if egfr in members and egfr not in values and num("Creatinine"):
                values[egfr] = {"value": ckd_epi_2021(num("Creatinine"), age, female), "_derived": True}
            if "Anion Gap" in members and "Anion Gap" not in values and all(num(t) for t in ("Sodium", "Chloride", "Bicarbonate")):
                values["Anion Gap"] = {"value": int(round(num("Sodium") - num("Chloride") - num("Bicarbonate"))), "_derived": True}
            if "Total Cholesterol" in members and "Total Cholesterol" not in values \
                    and all(num(t) for t in ("Low Density Lipoprotein", "High Density Lipoprotein", "Triglycerides")):
                values["Total Cholesterol"] = {"value": int(round(num("Low Density Lipoprotein") + num("High Density Lipoprotein")
                                                                  + num("Triglycerides") / 5)), "_derived": True}
            if "Non-HDL Cholesterol" in members and "Non-HDL Cholesterol" not in values \
                    and num("Total Cholesterol") and num("High Density Lipoprotein"):
                values["Non-HDL Cholesterol"] = {"value": int(round(num("Total Cholesterol") - num("High Density Lipoprotein"))), "_derived": True}
            if "Transferrin Saturation" in members and "Transferrin Saturation" not in values \
                    and num("Iron") and num("Total Iron-Binding Capacity"):
                values["Transferrin Saturation"] = {"value": int(round(num("Iron") / num("Total Iron-Binding Capacity") * 100)), "_derived": True}
        lab_cat = [{"coding": [{"system": OBS_CAT, "code": "laboratory", "display": "Laboratory"}]}]
        refs = []
        for t in members:
            r = values.get(t)
            if r is None:
                continue
            entry = self.c["labs"][t]
            oid = opaque("obs", mrn, eid, str(i), t)
            res = {"resourceType": "Observation", "id": oid,
                   "meta": {"versionId": "1", "lastUpdated": last_updated(eff)},
                   "status": "registered" if pending else "final", "category": lab_cat,
                   "code": {"coding": [{"system": entry["system"], "code": entry["code"], "display": entry["display"]}], "text": t},
                   "subject": subject, "effectiveDateTime": eff}
            if not pending:
                res["issued"] = issued
                if isinstance(r.get("value"), (int, float)):
                    res["valueQuantity"] = self._quantity(t, r["value"])
                    rr = self._ref_range(t, female)
                    if rr:
                        res["referenceRange"] = rr
                        lo, hi = rr[0]["low"]["value"], rr[0]["high"]["value"]
                        flag = "H" if r["value"] > hi else "L" if r["value"] < lo else None
                        if flag:
                            res["interpretation"] = [{"coding": [{"system": "http://terminology.hl7.org/CodeSystem/v3-ObservationInterpretation",
                                                                  "code": flag, "display": "High" if flag == "H" else "Low"}]}]
                elif r.get("text"):
                    res["valueCodeableConcept"] = {"text": str(r["text"])}
                else:
                    continue
            if r.get("_filled"):
                receipt.filled.append(f"{eid}:{t}")
            if r.get("_derived"):
                receipt.derived.append(f"{eid}:{t}")
            out["Observation"].append(res)
            refs.append({"reference": f"Observation/{oid}"})
        if pending:
            receipt.pending.append(f"{eid}:{panel_name or 'lab'} pending at cutoff")
        if panel and refs:
            code = {"coding": [{"system": panel["system"], "code": panel["code"], "display": panel["display"]}], "text": panel_name}
            rid = opaque("diagrep", mrn, eid, str(i))
            dr = {"resourceType": "DiagnosticReport", "id": rid,
                  "meta": {"versionId": "1", "lastUpdated": last_updated(eff)},
                  "status": "registered" if pending else "final",
                  "category": [{"coding": [{"system": V2_0074, "code": "LAB", "display": "Laboratory"}]}],
                  "code": code, "subject": subject, "effectiveDateTime": eff, "result": refs}
            if not pending:
                dr["issued"] = issued
            out["DiagnosticReport"].append(dr)
            po = {"resourceType": "Observation", "id": opaque("obs-panel", mrn, eid, str(i)),
                  "meta": {"versionId": "1", "lastUpdated": last_updated(eff)},
                  "status": "registered" if pending else "final", "category": lab_cat, "code": code,
                  "subject": subject, "effectiveDateTime": eff, "hasMember": refs}
            if not pending:
                po["issued"] = issued
            out["Observation"].append(po)

    def _report(self, out, receipt, mrn, subject, eid, i, rep, ev, cutoff):
        performed = rep.get("performed") or ev["start"]
        if local_dt(performed) >= cutoff:
            return
        entry = self.c["reports"].get(rep.get("study"))
        rng = random.Random(int(hashlib.sha256(f"{mrn}{eid}{i}".encode()).hexdigest()[:8], 16))
        slow = any(w in (rep.get("study") or "").lower() for w in ("pathology", "cytology"))
        delay = timedelta(days=3 + rng.randint(0, 5), hours=rng.randint(0, 8)) if slow \
            else timedelta(minutes=50 + rng.randint(0, 600))
        issued_local = local_dt(rep["issued"]) if rep.get("issued") else local_dt(performed) + delay
        pending = issued_local >= cutoff
        if entry:
            code = {"coding": [{"system": entry["system"], "code": entry["code"], "display": entry["display"]}], "text": rep["study"]}
            cat = {"system": V2_0074, "code": entry["category"]["code"], "display": entry["category"]["display"]}
        else:
            receipt.uncoded.append(f"DiagnosticReport:{rep.get('study')}")
            code = {"text": rep.get("study", "Diagnostic study")}
            cat = {"system": V2_0074, "code": "OTH", "display": "Other"}
        eff = to_utc(performed)
        dr = {"resourceType": "DiagnosticReport", "id": opaque("diagrep", mrn, eid, "rep", str(i)),
              "meta": {"versionId": "1", "lastUpdated": last_updated(eff)},
              "status": "registered" if pending else "final", "category": [{"coding": [cat]}],
              "code": code, "subject": subject, "effectiveDateTime": eff}
        if pending:
            receipt.pending.append(f"{eid}:{rep.get('study')} pending at cutoff")
        else:
            dr["issued"] = to_utc(fmt_local(issued_local))
            if rep.get("conclusion"):
                dr["conclusion"] = rep["conclusion"]
            dr["presentedForm"] = [{"contentType": "text/plain; charset=utf-8", "data": b64(rep["text"].strip() + "\n")}]
        out["DiagnosticReport"].append(dr)
