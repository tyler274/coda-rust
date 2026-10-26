# synthetic_hospital patient 1896

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-21 (Mon) | outpatient | Urology |
| 1 | 2020-07-15 | 2026-01-12 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Occasional, 1-2 drinks per month",
 "family_history": [
  "Maternal history of nephrolithiasis",
  "Paternal hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [
  "Recurrent urinary tract infections"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Urolithiasis (Renal Colic)"]
Comorbidities: ["Urinary tract infection"]

## Encounter 0: source 2020-01-15, practice 2025-07-21, outpatient, Urology

Chief complaint: Intermittent right-sided flank pain and cloudy, foul-smelling urine occurring over the past few months with persistent recurrent UTIs

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent right-sided flank pain and cloudy, foul-smelling urine occurring over the past few months with persistent recurrent UTIs

HISTORY OF PRESENT ILLNESS:
She reports multiple episodes of urinary tract infections during the past year, often treated with antibiotics but never fully resolving. She denies fever, chills, or hematuria.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent urinary tract infections

MEDICATIONS:
She is not on any medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal history of nephrolithiasis
- Paternal hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional, 1-2 drinks per month. Occupation: Elementary school teacher.

VITAL SIGNS:
Vital signs are within normal limits

PHYSICAL EXAMINATION:
Abdominal examination reveals mild right costovertebral angle tenderness

IMAGING:
An x-ray is shown below

## Encounter 1: source 2020-07-15, practice 2026-01-12, ed, Emergency Medicine

Chief complaint: Sudden-onset severe right flank pain radiating to the groin with associated nausea and hematuria

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset severe right flank pain radiating to the groin with associated nausea and hematuria

HISTORY OF PRESENT ILLNESS:
The patient returns after January urology evaluation for recurrent UTIs, describing sharp and severe pain and reporting associated nausea and hematuria.

PAST MEDICAL HISTORY:
Active Problem List:
- Urolithiasis (Renal Colic) (diagnosed 2020-01-15)
- Recurrent urinary tract infections

She has a history of recurrent urinary tract infections.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal history of nephrolithiasis
- Paternal hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional, 1-2 drinks per month. Occupation: Elementary school teacher.

VITAL SIGNS:
Her temperature is 37.8°C (100°F), blood pressure is 130/85 mm Hg, and heart rate is 98/min.

LABORATORY DATA:
Urinalysis shows alkaline urine with numerous white blood cells.

IMAGING:
A non-contrast CT scan of the abdomen is shown below.
