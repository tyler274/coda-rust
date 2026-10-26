# synthetic_hospital patient 1870

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield (Commercial PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-10-23 (Thu) | outpatient | Family Medicine |
| 1 | 2020-07-15 | 2026-05-08 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield (Commercial PPO)",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father with type 2 diabetes mellitus",
  "Mother with hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg orally twice daily",
   "name": "Metformin"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction (age 22)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus"
 ],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Acute bacterial sinusitis"]
Comorbidities: ["Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2025-10-23, outpatient, Family Medicine

Chief complaint: persistent cough, nasal congestion, and purulent nasal discharge for the past 12 days

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Family Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough, nasal congestion, and purulent nasal discharge for the past 12 days

HISTORY OF PRESENT ILLNESS:
She reports that her symptoms began as a "common cold" but have since worsened. She denies any fever, but notes increased facial pain, especially when bending forward.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Metformin

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her temperature is 98.6°F (37°C), blood pressure is 130/85 mmHg, and pulse is 78 beats per minute.

PHYSICAL EXAMINATION:
On physical examination, you note tenderness over her maxillary sinuses and erythematous nasal mucosa with greenish nasal discharge. Otoscopic examination is normal.

## Encounter 1: source 2020-07-15, practice 2026-05-08, ed, Emergency Medicine

Chief complaint: 10-day history of facial pain, pressure, and a persistent headache

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
10-day history of facial pain, pressure, and a persistent headache

HISTORY OF PRESENT ILLNESS:
This 35-year-old female with Type 2 diabetes mellitus presents with recurrent upper respiratory symptoms similar to those documented in January 2020. She reports that her symptoms began as a "common cold" with nasal congestion and a mild cough, but over the past few days, her condition has worsened. She now has a fever of 38.5°C, purulent nasal discharge, and increased cough, especially at night. She denies any visual changes, tooth pain, or ear discharge.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute bacterial sinusitis (diagnosed 2020-01-15)
- Type 2 diabetes mellitus

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Metformin

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, you note tenderness over the maxillary sinuses and erythema of the nasal mucosa. No polyps or masses are visible. Transillumination of the sinuses is poor. Her oral examination is unremarkable, and there is no cervical lymphadenopathy.

LABORATORY DATA:
Laboratory Values:
- White blood cell count: 12,000 /µL (Normal: 4,500-11,000 /µL)
- Neutrophils: 75% (Normal: 40-60%)
- Lymphocytes: 15% (Normal: 20-40%)
- Blood glucose: 160 mg/dL (Normal fasting: 70-99 mg/dL)
