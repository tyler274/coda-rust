#!/usr/bin/env python3
"""Design the synthetic adult primary-care panel: one seed per patient.

A seed fixes who the patient is before anyone writes their chart: MRN, sex, birth
year, town, household, how long they have been with the practice, how complex
their history is, and a few clinical anchors drawn from adult primary-care
prevalence. Family members are designed together so their charts can agree.
The author model writes the history; this script only guarantees spread and
reproducibility (fixed seed, no clock or network).

    python3 scripts/patient-bank/design_panel.py --count 995 --first-mrn 6 \
        --out target/patient-bank/seeds.json
"""

from __future__ import annotations

import argparse
import json
import random
import re
import sys
from datetime import date, timedelta
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import names  # noqa: E402

SEED = 20261024
AS_OF = "2026-08-27"  # Roger practice simulation time (coda-agents fixture)

TOWNS = [  # Pioneer Valley and nearby, where the Roger fixture patients live
    ("Northampton", "01060", 14), ("Florence", "01062", 6), ("Easthampton", "01027", 9),
    ("Amherst", "01002", 10), ("Hadley", "01035", 4), ("Holyoke", "01040", 10),
    ("South Hadley", "01075", 6), ("Hatfield", "01038", 2), ("Williamsburg", "01096", 2),
    ("Southampton", "01073", 2), ("Westhampton", "01027", 1), ("Leeds", "01053", 2),
    ("Haydenville", "01039", 1), ("Belchertown", "01007", 4), ("Granby", "01033", 2),
    ("Sunderland", "01375", 2), ("Deerfield", "01342", 2), ("Greenfield", "01301", 4),
    ("Westfield", "01085", 4), ("Chicopee", "01020", 4), ("Springfield", "01109", 3),
    ("Ludlow", "01056", 2), ("Chesterfield", "01012", 1), ("Goshen", "01032", 1),
]

# Adult primary-care age mix (years) -> weight.
AGE_BANDS = [((18, 29), 14), ((30, 39), 15), ((40, 49), 15), ((50, 59), 17),
             ((60, 69), 18), ((70, 79), 14), ((80, 94), 7)]

# Race / ethnicity mix loosely reflecting Hampshire and Hampden counties.
# (race label, ethnicity, preferred language weights)
ANCESTRY = [
    ("White", "Not Hispanic or Latino", 66),
    ("White", "Hispanic or Latino", 6),
    ("Other Race", "Hispanic or Latino", 7),   # e.g. Puerto Rican heritage, race unrecorded
    ("Black or African American", "Not Hispanic or Latino", 6),
    ("Asian", "Not Hispanic or Latino", 7),
    ("Two or more races", "Not Hispanic or Latino", 4),
    ("Declined", "Declined", 4),
]

# Chronic anchors with rough adult prevalence, age-dependent multipliers, sex.
# (anchor, base weight, min age, sex or None)
ANCHORS = [
    ("hypertension", 30, 30, None), ("hyperlipidemia", 28, 35, None),
    ("obesity", 22, 18, None), ("prediabetes", 12, 30, None),
    ("type 2 diabetes", 11, 30, None), ("depression", 12, 18, None),
    ("anxiety disorder", 12, 18, None), ("hypothyroidism", 7, 25, "female"),
    ("hypothyroidism", 2, 30, "male"), ("GERD", 10, 25, None),
    ("osteoarthritis", 12, 50, None), ("chronic low back pain", 9, 25, None),
    ("asthma", 8, 18, None), ("COPD", 5, 50, None), ("tobacco use", 9, 18, None),
    ("atrial fibrillation", 4, 60, None), ("coronary artery disease", 5, 55, None),
    ("heart failure", 2, 65, None), ("chronic kidney disease", 6, 55, None),
    ("obstructive sleep apnea", 7, 35, None), ("migraine", 7, 18, "female"),
    ("migraine", 2, 18, "male"), ("insomnia", 5, 25, None),
    ("benign prostatic hyperplasia", 10, 55, "male"), ("osteoporosis", 8, 60, "female"),
    ("gout", 3, 40, "male"), ("allergic rhinitis", 10, 18, None),
    ("ADHD", 4, 18, None), ("alcohol use disorder", 3, 22, None),
    ("opioid use disorder in recovery", 1, 22, None), ("cannabis use", 4, 18, None),
    ("cancer survivorship", 4, 45, None), ("breast cancer survivorship", 3, 45, "female"),
    ("prostate cancer history", 3, 60, "male"), ("rheumatoid arthritis", 1, 30, None),
    ("psoriasis", 2, 18, None), ("inflammatory bowel disease", 1, 18, None),
    ("irritable bowel syndrome", 5, 18, None), ("celiac disease", 1, 18, None),
    ("hepatitis C (treated or untreated)", 1, 25, None), ("HIV on treatment", 1, 25, None),
    ("PTSD", 3, 18, None), ("bipolar disorder", 1, 18, None),
    ("vitamin D deficiency", 5, 18, None), ("iron deficiency anemia", 3, 18, "female"),
    ("polycystic ovary syndrome", 3, 18, "female"), ("menopause symptoms", 7, 45, "female"),
    ("peripheral neuropathy", 3, 50, None), ("mild cognitive impairment or dementia", 4, 70, None),
    ("Parkinson disease", 1, 60, None), ("glaucoma or cataract", 6, 60, None),
    ("hearing loss", 6, 60, None), ("peripheral arterial disease", 2, 60, None),
    ("history of venous thromboembolism", 2, 30, None), ("thyroid nodule", 2, 30, None),
    ("kidney stones", 2, 25, None), ("chronic hepatitis B", 1, 25, None),
    ("multiple sclerosis", 1, 20, "female"), ("fibromyalgia", 2, 25, "female"),
    ("skin cancer history", 3, 50, None), ("diverticular disease", 3, 50, None),
    ("recurrent urinary tract infections", 3, 18, "female"),
]

# Episodic texture that real primary-care charts accumulate.
EPISODES = [
    "an upper respiratory infection or sinusitis visit", "a COVID-19 illness",
    "a musculoskeletal injury (sprain, strain or minor fracture)", "a urinary tract infection",
    "a skin lesion evaluated or removed", "an emergency department visit",
    "a hospital admission", "an elective surgery or procedure", "a medication side effect",
    "a dermatology, cardiology, GI or other specialist consultation",
    "screening colonoscopy, mammography or other age-appropriate screening",
    "an abnormal lab or imaging finding needing follow-up", "a portal message thread",
    "a telephone triage call", "a pregnancy, miscarriage or fertility care (if applicable)",
    "a mental-health crisis or counseling referral", "a fall", "a travel or vaccine visit",
    "an outside record arriving from a prior physician or hospital",
]

# Realistic chart imperfections (applied to a minority of patients).
TEXTURE = [
    "an allergy or adverse reaction documented only in one note and never added to the allergy list",
    "the patient stopped a medication on their own, documented in a phone note, while the order remains active",
    "an outside medication list that disagrees with the practice list",
    "a recommended follow-up (imaging, referral, lab) that was never completed",
    "a gap of more than two years without visits",
    "a result reviewed and communicated through a portal message",
    "a specialist changed a medication between primary-care visits",
    "conflicting family-history details across notes",
    "a duplicate or outdated problem-list entry",
    "a patient-reported outside event (urgent care visit, outside lab) with no records received",
]

COMPLEXITY = {  # targets measured against coda-gym charts (median ~108 notes, ~320 observations)
    "light": {"years": (5, 8), "documents": (40, 80), "observations": (200, 500)},
    "moderate": {"years": (8, 15), "documents": (80, 150), "observations": (500, 1200)},
    "complex": {"years": (12, 20), "documents": (150, 300), "observations": (1000, 2500)},
}

OCCUPATIONS = [
    "teacher", "nurse", "graduate student", "line cook", "software developer",
    "home health aide", "dairy farmer", "UPS driver", "accountant", "carpenter", "librarian",
    "college professor", "warehouse associate", "hair stylist", "small business owner", "electrician", "social worker", "bus driver", "barista",
    "physical therapist", "musician", "state employee", "landscaper",
    "pharmacy technician", "unemployed", "on disability", "paralegal", "veterinary technician",
    "construction laborer", "restaurant owner", "data analyst", "chef",
    "custodian", "bank teller", "artist", "clergy", "police officer", "firefighter",
    "medical assistant", "orchard worker", "childcare provider",
    "real estate agent", "auto mechanic", "homemaker", "undergraduate student",
]

INSURANCE = [("commercial employer plan", 46), ("Medicare", 22), ("Medicare Advantage", 8),
             ("MassHealth (Medicaid)", 14), ("Health Connector marketplace plan", 6),
             ("uninsured, self-pay", 2), ("VA plus Medicare", 2)]


def weighted(rng: random.Random, items):
    total = sum(w for *_, w in items)
    pick = rng.uniform(0, total)
    for item in items:
        pick -= item[-1]
        if pick <= 0:
            return item
    return items[-1]


def anchors_for(rng: random.Random, age: int, sex: str, complexity: str) -> list[str]:
    n = {"light": rng.choice([0, 0, 1, 1, 2]), "moderate": rng.choice([1, 2, 2, 3]),
         "complex": rng.choice([3, 4, 4, 5, 6])}[complexity]
    pool = []
    for name, w, min_age, s in ANCHORS:
        if age < min_age or (s and s != sex):
            continue
        scale = 1 + max(0, age - 45) / 25 if name in {
            "hypertension", "hyperlipidemia", "type 2 diabetes", "osteoarthritis",
            "chronic kidney disease", "atrial fibrillation", "coronary artery disease"} else 1
        pool.append((name, w * scale))
    chosen: list[str] = []
    while pool and len(chosen) < n:
        name = weighted(rng, pool)[0]
        pool = [p for p in pool if p[0] != name]
        if name not in chosen:
            chosen.append(name)
    return chosen


def complexity_for(rng: random.Random, age: int) -> str:
    if age < 35:
        w = [("light", 62), ("moderate", 33), ("complex", 5)]
    elif age < 60:
        w = [("light", 30), ("moderate", 52), ("complex", 18)]
    else:
        w = [("light", 12), ("moderate", 50), ("complex", 38)]
    return weighted(rng, w)[0]


def person(rng: random.Random, mrn: int, age: int | None = None, sex: str | None = None,
           town=None, ancestry=None) -> dict:
    if age is None:
        lo, hi = weighted(rng, AGE_BANDS)[0]
        age = rng.randint(lo, hi)
    sex = sex or ("female" if rng.random() < 0.53 else "male")
    town = town or weighted(rng, TOWNS)
    ancestry = ancestry or weighted(rng, ANCESTRY)
    complexity = complexity_for(rng, age)
    span = COMPLEXITY[complexity]
    wanted = rng.randint(*span["years"])
    years = min(wanted, max(1, age - 17))  # an adult practice: joined at 18 at the earliest
    scale = max(0.3, years / wanted)  # fewer years, proportionally fewer records
    birth_year = int(AS_OF[:4]) - age - (1 if rng.random() < 0.6 else 0)
    insurance = weighted(rng, [i for i in INSURANCE if not (i[0].startswith("Medicare") and age < 63)
                               and not (i[0] == "VA plus Medicare" and age < 65)])[0]
    if age >= 66 and rng.random() < 0.85:
        insurance = rng.choice(["Medicare", "Medicare", "Medicare Advantage", "Medicare with a supplement plan"])
    language = "English"
    if ancestry[1] == "Hispanic or Latino" and rng.random() < 0.3:
        language = "Spanish"
    elif ancestry[0] == "Asian" and rng.random() < 0.25:
        language = rng.choice(["Vietnamese", "Mandarin", "Korean", "Khmer", "Hindi"])
    elif rng.random() < 0.02:
        language = rng.choice(["Portuguese", "Russian", "Haitian Creole", "Polish"])
    episodes = rng.sample(EPISODES, k={"light": 4, "moderate": 7, "complex": 10}[complexity])
    texture = rng.sample(TEXTURE, k={"light": 1, "moderate": 2, "complex": 3}[complexity])
    return {
        "mrn": f"MRN{mrn:08d}",
        "sex": sex,
        "age_at_as_of": age,
        "birth_year": birth_year,
        "town": town[0],
        "postal_code": town[1],
        "race": ancestry[0],
        "ethnicity": ancestry[1],
        "preferred_language": language,
        "insurance": insurance,
        "occupation_hint": rng.choice(OCCUPATIONS) if age < 66 else rng.choice(
            ["retired teacher", "retired machinist", "retired nurse", "retired postal worker",
             "retired engineer", "retired farmer", "retired bookkeeper", "part-time library volunteer"]),
        "complexity": complexity,
        "years_with_practice": years,
        "target_documents": int(rng.randint(*span["documents"]) * scale),
        "target_observations": int(rng.randint(*span["observations"]) * scale),
        "anchors": anchors_for(rng, age, sex, complexity),
        "episodes": episodes,
        "chart_texture": texture,
    }


def design(count: int, first_mrn: int) -> list[dict]:
    rng = random.Random(SEED)
    units: list[dict] = []
    mrn = first_mrn
    remaining = count
    unit_no = 0
    while remaining > 0:
        unit_no += 1
        roll = rng.random()
        size = 1
        kind = None
        if remaining >= 2 and roll < 0.09:
            kind, size = "couple", 2
        elif remaining >= 2 and roll < 0.12:
            kind, size = "parent and adult child", 2
        elif remaining >= 3 and roll < 0.135:
            kind, size = "parents and adult child", 3
        elif remaining >= 2 and roll < 0.15:
            kind, size = "adult siblings", 2
        members = []
        if kind is None:
            members.append(person(rng, mrn))
        else:
            town = weighted(rng, TOWNS)
            anc = weighted(rng, ANCESTRY)
            if kind == "couple":
                a = rng.randint(28, 85)
                sexes = rng.choice([("female", "male"), ("female", "male"), ("female", "male"),
                                    ("female", "female"), ("male", "male")])
                members.append(person(rng, mrn, a, sexes[0], town, anc))
                members.append(person(rng, mrn + 1, max(18, min(94, a + rng.randint(-6, 6))), sexes[1], town, anc))
                rel = [("spouse", 0, 1)]
            elif kind == "parent and adult child":
                p = rng.randint(50, 88)
                members.append(person(rng, mrn, p, None, town, anc))
                members.append(person(rng, mrn + 1, max(18, p - rng.randint(22, 36)), None,
                                      town if rng.random() < 0.4 else None, anc))
                rel = [("parent", 0, 1)]
            elif kind == "parents and adult child":
                p = rng.randint(48, 80)
                members.append(person(rng, mrn, p, "female", town, anc))
                members.append(person(rng, mrn + 1, min(94, p + rng.randint(-3, 5)), "male", town, anc))
                members.append(person(rng, mrn + 2, max(18, p - rng.randint(22, 32)), None, town, anc))
                rel = [("spouse", 0, 1), ("parent", 0, 2), ("parent", 1, 2)]
            else:
                a = rng.randint(25, 80)
                members.append(person(rng, mrn, a, None, None, anc))
                members.append(person(rng, mrn + 1, max(18, a + rng.randint(-8, 8)), None, None, anc))
                rel = [("sibling", 0, 1)]
            fam = f"family-{unit_no:04d}"
            for m in members:
                m["family_id"] = fam
                m["relationships"] = []
            # Each entry names the other person's role relative to this patient;
            # for ("parent", i, j), member i is the parent of member j.
            for r, i, j in rel:
                forward, back = {"spouse": ("spouse", "spouse"), "parent": ("child", "parent"),
                                 "sibling": ("sibling", "sibling")}[r]
                members[i]["relationships"].append({"mrn": members[j]["mrn"], "relation": forward})
                members[j]["relationships"].append({"mrn": members[i]["mrn"], "relation": back})
        units.append({"unit": f"unit-{unit_no:04d}", "kind": kind or "individual", "members": members})
        mrn += len(members)
        remaining -= len(members)
    return units


def heritage(m: dict) -> str:
    if m["ethnicity"] == "Hispanic or Latino":
        return "hispanic"
    if m["race"] == "Asian":
        return "asian"
    if m["race"] == "Black or African American":
        return "black"
    if m["preferred_language"] in ("Russian", "Polish"):
        return "slavic"
    return "general"


def assign_identity(units: list[dict]) -> None:
    """Give every patient a unique fictional name, birth date, phone and email."""
    rng = random.Random(SEED + 1)
    as_of = date.fromisoformat(AS_OF)
    used_names: set[tuple[str, str]] = set()
    used_phones: set[str] = set()
    used_emails: set[str] = set()
    for unit in units:
        shared_family = None
        for m in unit["members"]:
            h = heritage(m)
            age = m["age_at_as_of"]
            latest = as_of.replace(year=as_of.year - age)
            earliest = as_of.replace(year=as_of.year - age - 1) + timedelta(days=1)
            dob = earliest + timedelta(days=rng.randint(0, (latest - earliest).days))
            m["birth_date"] = dob.isoformat()
            m["birth_year"] = dob.year
            era = "older" if dob.year < 1960 else "middle" if dob.year < 1990 else "younger"
            origin = None
            if h == "asian":
                origin = names.LANGUAGE_ORIGIN.get(m["preferred_language"]) or rng.choice(sorted(names.ASIAN_ORIGINS))
                if shared_family:
                    origin = unit.setdefault("_origin", origin)
                unit.setdefault("_origin", origin)
                origin = unit["_origin"]
            for _ in range(500):
                if origin:
                    f_names, m_names, fam_names = names.ASIAN_ORIGINS[origin]
                    given = rng.choice(f_names if m["sex"] == "female" else m_names)
                elif h != "general" and rng.random() < 0.85:
                    given = rng.choice(names.GIVEN_HERITAGE[h][m["sex"]])
                else:
                    given = rng.choice(names.GIVEN[(m["sex"], era)])
                if shared_family and (unit["kind"] != "couple" or rng.random() < 0.7):
                    family = shared_family
                else:
                    pool = (names.ASIAN_ORIGINS[origin][2] if origin else
                            names.FAMILY[h] if h != "general" and rng.random() < 0.85 else names.FAMILY["general"])
                    family = rng.choice(pool)
                if (given, family) not in used_names:
                    break
            used_names.add((given, family))
            shared_family = shared_family or family
            m["given"], m["family"] = given, family
            while True:
                phone = f"+1-{rng.choice(names.AREA_CODES)}-555-01{rng.randint(0, 99):02d}"
                if phone not in used_phones:
                    break
            used_phones.add(phone)
            m["phone"] = phone
            if rng.random() < 0.82:
                base = re.sub(r"[^a-z]", "", given.lower()) + "." + re.sub(r"[^a-z]", "", family.lower())
                email, n = f"{base}@example.test", 2
                while email in used_emails:
                    email, n = f"{base}{n}@example.test", n + 1
                used_emails.add(email)
                m["email"] = email


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--count", type=int, required=True)
    ap.add_argument("--first-mrn", type=int, default=6)
    ap.add_argument("--out", type=Path, required=True)
    a = ap.parse_args()
    units = design(a.count, a.first_mrn)
    assign_identity(units)
    for u in units:
        u.pop("_origin", None)
    a.out.parent.mkdir(parents=True, exist_ok=True)
    a.out.write_text(json.dumps({"seed": SEED, "as_of": AS_OF, "units": units}, indent=1) + "\n")
    n = sum(len(u["members"]) for u in units)
    print(f"{n} patients in {len(units)} authoring units -> {a.out}")


if __name__ == "__main__":
    main()
