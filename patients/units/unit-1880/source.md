# synthetic_hospital patient 1880

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-04-19 (Fri) | outpatient | Endocrinology |
| 1 | 2021-07-15 | 2025-10-03 (Fri) | outpatient | Endocrinology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO",
 "occupation": "Administrative data specialist",
 "alcohol_use": "Minimal (<1 drink per week)",
 "family_history": [
  "Maternal aunt: Primary hyperparathyroidism",
  "Father: Recurrent nephrolithiasis",
  "Mother: Hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Morbid obesity (BMI 43 kg/m\u00b2)"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Primary hyperparathyroidism"]
Comorbidities: ["Parathyroid adenoma"]

## Encounter 0: source 2020-01-15, practice 2024-04-19, outpatient, Endocrinology

Chief complaint: Asymptomatic hypercalcemia detected on routine laboratory monitoring with known parathyroid adenoma

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Asymptomatic hypercalcemia detected on routine laboratory monitoring with known parathyroid adenoma

HISTORY OF PRESENT ILLNESS:
She was previously asymptomatic but had mild hypercalcemia on routine bloodwork. Her serum calcium had been consistently elevated, and intact PTH levels were above normal. She was deemed a poor surgical candidate due to other comorbidities and was started on a drug called cinacalcet.

PAST MEDICAL HISTORY:
Active Problem List:
- Morbid obesity (BMI 43 kg/m²)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Cinacalcet

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt: Primary hyperparathyroidism
- Father: Recurrent nephrolithiasis
- Mother: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Minimal (<1 drink per week). Occupation: Administrative data specialist.

ASSESSMENT AND PLAN:
Cinacalcet is a calcimimetic that acts by potentiating the effect of calcium on the calcium-sensing receptors (CaSR) in the parathyroid glands.

Two weeks after starting the medication, repeat labs are ordered to evaluate treatment response.

## Encounter 1: source 2021-07-15, practice 2025-10-03, outpatient, Endocrinology

Chief complaint: Persistent fatigue, generalized body aches, frequent urination, and difficulty concentrating over the past 6 months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, generalized body aches, frequent urination, and difficulty concentrating over the past 6 months

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of parathyroid adenoma identified in January 2020. Over the past 6 months, she has also noticed difficulty concentrating at work and occasional constipation. She denies fever, weight loss, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Morbid obesity (BMI 43 kg/m²)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt: Primary hyperparathyroidism
- Father: Recurrent nephrolithiasis
- Mother: Hypothyroidism

SOCIAL HISTORY:
She drinks minimal alcohol and does not use tobacco or recreational drugs

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Her creatinine is within normal limits. Further testing reveals an elevated intact parathyroid hormone level

IMAGING:
A neck ultrasound shows a 1.4-cm hypoechoic lesion adjacent to the left thyroid lobe. A nuclear medicine scan demonstrates increased uptake in the same region.
