# synthetic_hospital patient 2137

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 38 at first encounter, sex M, Non-Hispanic White, Employer-sponsored PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-01 (Mon) | outpatient | Endocrinology |
| 1 | 2022-01-15 | 2026-06-29 (Mon) | follow_up | Urology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored PPO",
 "occupation": "Software engineer",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father with recurrent kidney stones",
  "Paternal aunt with primary hyperparathyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Tonsillectomy (age 7)"
 ],
 "chronic_conditions": [
  "Primary hyperparathyroidism",
  "Recurrent calcium nephrolithiasis"
 ],
 "age_at_first_encounter": 38
}
```

Primary diagnoses: ["Primary hyperparathyroidism", "Urolithiasis (Renal Colic)"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-07-01, outpatient, Endocrinology

Chief complaint: 1-year history of fatigue, intermittent constipation, and difficulty concentrating

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
1-year history of fatigue, intermittent constipation, and difficulty concentrating

HISTORY OF PRESENT ILLNESS:
He has also had two episodes of flank pain requiring emergency evaluation, both of which revealed calcium oxalate kidney stones. His appetite is normal, but he has unintentionally lost 4 kg (9 lb) over the past 6 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism
- Recurrent calcium nephrolithiasis

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with recurrent kidney stones
- Paternal aunt with primary hyperparathyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Software engineer.

PHYSICAL EXAMINATION:
Physical exam reveals mild muscle weakness in the upper and lower extremities, but no cranial nerve abnormalities or skin findings.

LABORATORY DATA:
Calcium                                              11.2 mg/dL
Phosphate                                            2.1 mEq/L
25-D3                                                  Normal

## Encounter 1: source 2022-01-15, practice 2026-06-29, follow_up, Urology

Chief complaint: presents to the clinic for follow-up after a recent episode of nephrolithiasis

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Marcus Johnson, MD
Department: Urology
Visit Type: follow_up

CHIEF COMPLAINT:
He presents to the clinic for follow-up after a recent episode of nephrolithiasis

HISTORY OF PRESENT ILLNESS:
The patient is a 40-year-old male with primary hyperparathyroidism diagnosed in 2020 who returns to the urology clinic for follow-up of recurrent calcium nephrolithiasis. He reports passing a calcium oxalate kidney stone two weeks ago and is now asymptomatic. He has had two prior episodes of kidney stones over the past five years, both requiring emergency department visits. His diet is balanced, and he drinks approximately 2 liters of water daily.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperparathyroidism (diagnosed 2020-01-15)
- Recurrent calcium nephrolithiasis

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is positive for kidney stones in his father.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Software engineer.

LABORATORY DATA:
Laboratory testing during his most recent episode showed elevated urinary calcium excretion with normal serum calcium and parathyroid hormone levels.

PLAN:
After evaluation, the patient is started on hydrochlorothiazide to reduce the risk of stone recurrence.
