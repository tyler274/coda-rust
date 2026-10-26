# synthetic_hospital patient 1914

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-06-03 (Fri) | outpatient | Dermatology |
| 1 | 2024-01-15 | 2026-06-10 (Wed) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Paternal grandmother with type 1 diabetes mellitus",
  "Father with vitiligo",
  "Maternal aunt with Hodgkin lymphoma (age 32)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [
  "Vitiligo"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Malignancy-associated hypercalcemia", "Vitiligo"]
Comorbidities: ["Family history of diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-06-03, outpatient, Dermatology

Chief complaint: Evaluation of progressively spreading pale patches on her face, neck, and dorsal hands over the past 18 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of progressively spreading pale patches on her face, neck, and dorsal hands over the past 18 months

HISTORY OF PRESENT ILLNESS:
She reports no associated itching, pain, or preceding trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitiligo

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Mentions a strong family history of autoimmune thyroid disease.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination reveals multiple irregular depigmented macules with sharp borders on sun-exposed skin. The lesions fluoresce bright white under Wood lamp examination. Sensory function over the lesions is intact, and dermoscopy reveals perifollicular pigmentation.

## Encounter 1: source 2024-01-15, practice 2026-06-10, outpatient, Internal Medicine

Chief complaint: Routine annual health examination, no complaints

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Marcus Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine annual health examination, no complaints

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with vitiligo diagnosed in Dermatology in 2020 returns for routine annual health examination. She has no complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitiligo (diagnosed 2020-01-15)

MEDICATIONS:
Currently not on any medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her family history is significant for her father having had "high calcium levels" without any complications

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 11.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum parathyroid hormone (PTH): 65 pg/mL (normal: 10-65 pg/mL)
- Serum creatinine: 0.9 mg/dL (normal: 0.6-1.1 mg/dL)
- 24-hour urine calcium: 100 mg (normal: 100-300 mg/day)
- 24-hour urine creatinine: 1.2 g/day (normal: 1.0-1.5 g/day)
