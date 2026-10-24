"""Check an authored ledger against its seed, the catalog and chronology.

Errors make the author retry with the error list; warnings are recorded for
review. The checks are mechanical: they never judge clinical reasoning.
"""

from __future__ import annotations

import re
from datetime import date, datetime

AS_OF = "2026-08-27T12:00"
KINDS = {"establish_care", "office_visit", "annual_physical", "telehealth_visit", "nurse_visit",
         "telephone", "portal_message", "lab_only", "imaging", "specialist_visit", "urgent_care",
         "ed_visit", "hospital_admission", "procedure_visit", "outside_records", "pharmacy"}
STAMP = re.compile(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}$")
FORBIDDEN = re.compile(r"\b(synthetic|fictional|generated|template|simulation|simulated|test patient|"
                       r"answer key|placeholder|lorem)\b", re.I)
# Real care organizations and record systems a chart must not name (the guide asks for roles and
# generic places). Drug, vaccine and assay product names stay, as in real charts.
REAL_NAMES = re.compile(r"\b(MyChart|Epic|Cerner|athenahealth|"
                        r"Quest Diagnostics|LabCorp|CVS|Walgreens|Walmart|Rite Aid|Stop & Shop|Big Y|"
                        r"Baystate|Cooley Dickinson|Mercy Medical|Holyoke Medical|UMass|Mass General|Brigham|"
                        r"Dartmouth|Hartford Hospital|Yale New Haven|Valley Medical Group|Kaiser|Mayo Clinic)\b")
DIFFERENTIAL = ("Neutrophils %", "Lymphocytes %", "Monocytes %", "Eosinophils %", "Basophils %")
AGE_IN_TEXT = re.compile(r"\b(\d{2,3})[- ](?:year|yr)s?[- ]old\b", re.I)


def _stamps(ev: dict):
    yield "start", ev.get("start")
    if ev.get("end"):
        yield "end", ev["end"]
    for k in ("labs",):
        for x in ev.get(k, []):
            yield "labs.collected", x.get("collected")
    for x in ev.get("reports", []):
        yield "reports.performed", x.get("performed")
    for x in ev.get("procedures", []):
        yield "procedures.performed", x.get("performed")
    for x in ev.get("immunizations", []):
        yield "immunizations.date", x.get("date")
    for x in ev.get("documents", []):
        yield "documents.date", x.get("date")
    vit = ev.get("vitals")
    for x in (vit if isinstance(vit, list) else []):
        if x.get("time") or len(vit) > 1:
            yield "vitals.time", x.get("time")


def age_on(birth: date, on: date) -> int:
    return on.year - birth.year - ((on.month, on.day) < (birth.month, birth.day))


def check(unit: dict, ledger: dict, catalog: dict, extra_ok=lambda kind, item: False) -> tuple[list[str], list[str]]:
    errors: list[str] = []
    warnings: list[str] = []
    seeds = {m["mrn"]: m for m in unit["members"]}
    patients = ledger.get("patients")
    if not isinstance(patients, list):
        return ["top level must be {\"patients\": [...]}"], warnings
    got = {p.get("mrn") for p in patients}
    if got != set(seeds):
        errors.append(f"patients must be exactly {sorted(seeds)}; got {sorted(x for x in got if x)}")
    bodies: dict[str, str] = {}
    for p in patients:
        seed = seeds.get(p.get("mrn"))
        if not seed:
            continue
        mrn = seed["mrn"]
        birth = date.fromisoformat(seed["birth_date"])
        for k in ("given", "family", "birth_date"):
            if p.get(k) and p[k] != seed[k]:
                errors.append(f"{mrn}: {k} must be {seed[k]!r} (seed), got {p[k]!r}")
        events = p.get("events") or []
        if not events:
            errors.append(f"{mrn}: no events")
            continue
        ids = [e.get("id") for e in events]
        if len(set(ids)) != len(ids):
            errors.append(f"{mrn}: duplicate event ids")
        started: dict[str, str] = {}
        diagnosed: dict[str, str] = {}
        n_docs = 0
        for ev in events:
            eid = f"{mrn}/{ev.get('id')}"
            if ev.get("kind") not in KINDS:
                errors.append(f"{eid}: unknown kind {ev.get('kind')!r}")
            for where, stamp in _stamps(ev):
                if not isinstance(stamp, str) or not STAMP.match(stamp):
                    if stamp is not None or where == "start":
                        errors.append(f"{eid}: {where} must be 'YYYY-MM-DDTHH:MM', got {stamp!r}")
                    continue
                try:
                    datetime.fromisoformat(stamp)
                except ValueError:
                    errors.append(f"{eid}: {where} invalid date {stamp!r}")
                    continue
                if stamp >= AS_OF and where != "end":
                    errors.append(f"{eid}: {where} {stamp} is not before {AS_OF}")
                if stamp[:10] < seed["birth_date"]:
                    errors.append(f"{eid}: {where} {stamp} precedes birth {seed['birth_date']}")
            start = ev.get("start") or ""
            for c in ev.get("conditions", []):
                label = c.get("label")
                if c.get("action") not in ("diagnosed", "history", "resolved"):
                    errors.append(f"{eid}: condition {label!r} action must be diagnosed|history|resolved")
                if label not in catalog["conditions"] and not c.get("icd10") and not extra_ok("condition", c):
                    errors.append(f"{eid}: condition {label!r} is not a catalog label and has no icd10")
                if c.get("action") in ("diagnosed", "history"):
                    diagnosed.setdefault(label, start)
                    onset = c.get("onset")
                    if onset and str(onset)[:10] > start[:10]:
                        errors.append(f"{eid}: condition {label!r} onset {onset} after recording event {start[:10]}")
                elif c.get("action") == "resolved" and label not in diagnosed:
                    errors.append(f"{eid}: condition {label!r} resolved before being diagnosed")
            for m in ev.get("medications", []):
                key = m.get("drug") or m.get("drug_text")
                if m.get("drug") and m["drug"] not in catalog["medications"]:
                    errors.append(f"{eid}: drug {m['drug']!r} is not a catalog name (use drug_text instead)")
                if not key:
                    errors.append(f"{eid}: medication without drug or drug_text")
                if m.get("action") in ("start", "historical"):
                    started.setdefault(key, start)
                elif m.get("action") == "stop":
                    if key not in started:
                        errors.append(f"{eid}: medication {key!r} stopped before it was started")
                else:
                    errors.append(f"{eid}: medication {key!r} action must be start|stop|historical")
            for lab in ev.get("labs", []):
                if lab.get("panel") and lab["panel"] not in catalog["panels"]:
                    errors.append(f"{eid}: unknown panel {lab['panel']!r}")
                for r in lab.get("results", []):
                    if r.get("test") not in catalog["labs"]:
                        errors.append(f"{eid}: unknown lab test {r.get('test')!r}")
                    elif "value" not in r and "text" not in r:
                        errors.append(f"{eid}: lab {r.get('test')!r} needs value or text")
                    if lab.get("panel") in catalog["panels"] and r.get("test") not in catalog["panels"][lab["panel"]]["members"]:
                        errors.append(f"{eid}: {r.get('test')!r} is not a member of {lab['panel']!r}")
                vals = {r.get("test"): r.get("value") for r in lab.get("results", [])
                        if isinstance(r.get("value"), (int, float))}
                stated = [vals[t] for t in DIFFERENTIAL if t in vals]
                if sum(stated) > 101:
                    errors.append(f"{eid}: stated differential totals {sum(stated):g}% (over 100); "
                                  "state only the members you mean and code fills the rest to 100")
                elif len(stated) == len(DIFFERENTIAL) and not 98 <= sum(stated) <= 102:
                    errors.append(f"{eid}: differential totals {sum(stated):g}%, must total 100")
                hgb, hct, rbc, mcv = (vals.get(t) for t in ("Hemoglobin", "Hematocrit", "Red Blood Cells",
                                                            "Mean Corpuscular Volume"))
                if rbc and not mcv:
                    errors.append(f"{eid}: Red Blood Cells stated without Mean Corpuscular Volume; state Hgb and MCV "
                                  "(code derives Hct, RBC, MCH and MCHC) or state the indices together")
                if hct and rbc and mcv and abs(hct / rbc * 10 - mcv) > 3:
                    errors.append(f"{eid}: MCV {mcv} disagrees with Hct/RBC ({hct / rbc * 10:.0f})")
                implied_hct = hct or (rbc * mcv / 10 if rbc and mcv else None)
                if hgb and implied_hct and not 30 <= hgb / implied_hct * 100 <= 37:
                    errors.append(f"{eid}: Hgb {hgb} with Hct {implied_hct:.1f} gives MCHC "
                                  f"{hgb / implied_hct * 100:.1f}, outside what an analyzer reports (30-37)")
            for s in ev.get("surveys", []):
                if s.get("survey") not in catalog["surveys"]:
                    errors.append(f"{eid}: unknown survey {s.get('survey')!r}")
            for s in ev.get("social", []):
                if s.get("item") not in catalog["social"]:
                    errors.append(f"{eid}: unknown social item {s.get('item')!r}")
            for r in ev.get("reports", []):
                if r.get("study") not in catalog["reports"]:
                    errors.append(f"{eid}: unknown report study {r.get('study')!r}")
                if not (r.get("text") or "").strip():
                    errors.append(f"{eid}: report {r.get('study')!r} has no text")
                if REAL_NAMES.search(r.get("text") or ""):
                    errors.append(f"{eid}: report {r.get('study')!r} names a real organization or product "
                                  f"({REAL_NAMES.search(r['text']).group(0)})")
            for r in ev.get("procedures", []):
                if r.get("procedure") not in catalog["procedures"]:
                    errors.append(f"{eid}: unknown procedure {r.get('procedure')!r}")
            for a in ev.get("allergies", []):
                if a.get("substance") not in catalog["allergens"]:
                    warnings.append(f"{eid}: allergen {a.get('substance')!r} not in catalog (text only)")
                if a.get("reaction") and a["reaction"] not in catalog["reactions"]:
                    warnings.append(f"{eid}: reaction {a.get('reaction')!r} not in catalog (text only)")
            for d in ev.get("documents", []):
                n_docs += 1
                body = (d.get("body") or "").strip()
                if len(body) < 40:
                    errors.append(f"{eid}: document {d.get('title')!r} body too short")
                if REAL_NAMES.search(body):
                    errors.append(f"{eid}: document {d.get('title')!r} names a real organization or product "
                                  f"({REAL_NAMES.search(body).group(0)}); use a role or generic description")
                if FORBIDDEN.search(body):
                    errors.append(f"{eid}: document {d.get('title')!r} uses a forbidden word ({FORBIDDEN.search(body).group(0)})")
                if body in bodies:
                    errors.append(f"{eid}: document body duplicates {bodies[body]}")
                bodies[body] = eid
                when = d.get("date") or start
                if isinstance(when, str) and STAMP.match(when):
                    actual = age_on(birth, date.fromisoformat(when[:10]))
                    for m in AGE_IN_TEXT.finditer(body):
                        stated = int(m.group(1))
                        if abs(stated - actual) >= 1 and stated < 120:
                            ctx = body[max(0, m.start() - 40):m.end() + 10]
                            if not re.search(r"\b(mother|father|sister|brother|son|daughter|wife|husband|"
                                             r"spouse|partner|grand|aunt|uncle|cousin|friend|at age|in her|in his)\b",
                                             ctx, re.I):
                                errors.append(f"{eid}: note says {stated}-year-old but patient is {actual} on {when[:10]}")
            if ev.get("kind") in ("office_visit", "annual_physical", "establish_care", "telehealth_visit") \
                    and not ev.get("documents"):
                errors.append(f"{eid}: visit without a note")
        if n_docs < 0.6 * seed["target_documents"]:
            warnings.append(f"{mrn}: {n_docs} documents vs target {seed['target_documents']}")
        first = min(e.get("start", "9999") for e in events if e.get("kind") != "outside_records")
        expected = int(AS_OF[:4]) - seed["years_with_practice"]
        if abs(int(first[:4]) - expected) > 2:
            warnings.append(f"{mrn}: first practice event {first[:10]} vs about {expected}")
    return errors, warnings
