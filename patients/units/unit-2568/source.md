# synthetic_hospital patient 2568

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex M, Non-Hispanic White, Employer-sponsored PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-26 (Sun) | ed | Emergency Medicine |
| 1 | 2021-01-15 | 2026-02-02 (Mon) | icu | Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored PPO",
 "occupation": "Software Developer",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Father with hypertension",
  "Mother with hypothyroidism",
  "No family history of sudden cardiac death or malignant hyperthermia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Tonsillectomy at age 8"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Exertional heat stroke", "Hyperchloremic metabolic acidosis due to excess normal saline infusion"]
Comorbidities: ["Hypotension"]

## Encounter 0: source 2020-01-15, practice 2025-01-26, ed, Emergency Medicine

Chief complaint: Found unresponsive in his apartment after taking a large dose of narcotics.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found unresponsive in his apartment after taking a large dose of narcotics.

HISTORY OF PRESENT ILLNESS:
Earlier in the evening, he admitted to taking a large dose of narcotics. They report he appeared euphoric at first but became progressively more drowsy.

PAST SURGICAL HISTORY:
- Tonsillectomy at age 8

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension
- Mother with hypothyroidism
- No family history of sudden cardiac death or malignant hyperthermia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Software Developer.

VITAL SIGNS:
Respiratory rate of 6/min. Blood pressure is 88/56 mm Hg, and pulse is 118/min.

PHYSICAL EXAMINATION:
On arrival, he is somnolent and responds only to painful stimuli. His breathing is shallow and irregular. Pupils are constricted and minimally reactive to light.

## Encounter 1: source 2021-01-15, practice 2026-02-02, icu, Critical Care

Chief complaint: Collapsing while running a marathon on a hot summer day.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Collapsing while running a marathon on a hot summer day.

HISTORY OF PRESENT ILLNESS:
The 29-year-old man with a prior emergency department encounter one year ago for unresponsiveness following narcotic ingestion is brought to the emergency department during the summer after collapsing while running a marathon. Witnesses say he had been sweating heavily but suddenly became confused and then lost consciousness.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperchloremic metabolic acidosis due to excess normal saline infusion (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Tonsillectomy at age 8

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension
- Mother with hypothyroidism
- No family history of sudden cardiac death or malignant hyperthermia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Software Developer.

VITAL SIGNS:
Temperature is 41.3°C (106.3°F), pulse is 118/min, and blood pressure is 88/60 mm Hg

PHYSICAL EXAMINATION:
On arrival, he is lethargic and disoriented. Skin is hot and dry. He has no signs of trauma or infection. He does not respond to verbal commands but withdraws from painful stimuli.

LABORATORY DATA:
Laboratory studies and a blood smear are shown:
Hemoglobin                                 13.9 g/dL
Leukocytes                                 13,400/mm³
Platelets                                     160,000/mm³
Serum urea nitrogen                 42 mg/dL
Creatinine                                   2.1 mg/dL
AST                                             720 U/L
ALT                                             640 U/L
