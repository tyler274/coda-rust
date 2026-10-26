# synthetic_hospital patient 2777

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-05-02 (Thu) | procedure | General Surgery |
| 1 | 2021-07-15 | 2025-10-20 (Mon) | outpatient | Endocrinology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "High school science teacher",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Maternal aunt with multinodular goiter",
  "Father with hypothyroidism",
  "Mother with hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Coronary angiography via femoral artery",
  "Total thyroidectomy"
 ],
 "chronic_conditions": [
  "Multinodular goiter"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Multinodular goiter", "Drug-induced hypothyroidism"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-05-02, procedure, General Surgery

Chief complaint: Progressive dysphagia and neck discomfort from enlarged multinodular goiter

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: General Surgery
Visit Type: procedure

CHIEF COMPLAINT:
Progressive dysphagia and neck discomfort from enlarged multinodular goiter

HISTORY OF PRESENT ILLNESS:
Undergoes a total thyroidectomy for a large multinodular goiter that was causing progressive dysphagia and neck discomfort

PAST MEDICAL HISTORY:
Active Problem List:
- Multinodular goiter

PAST SURGICAL HISTORY:
- Coronary angiography via femoral artery
- Total thyroidectomy

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with multinodular goiter
- Father with hypothyroidism
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: High school science teacher.

OTHER STUDIES:
The operation is completed without complications. In the surgical report, the surgeon notes deliberate ligation of the arteries supplying both the superior and inferior portions of the thyroid gland. During dissection of the upper thyroid pole, the vessel encountered was carefully ligated near its entry into the gland, with care taken to preserve adjacent neural structures. The artery supplying the superior portion of the thyroid gland originates from

ASSESSMENT AND PLAN:
Postoperatively, the patient recovers well without vocal changes or evidence of hematoma.

## Encounter 1: source 2021-07-15, practice 2025-10-20, outpatient, Endocrinology

Chief complaint: Progressive fatigue, cold intolerance, and weight gain over 3 months following recent cardiac catheterization

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, cold intolerance, and weight gain over 3 months following recent cardiac catheterization

HISTORY OF PRESENT ILLNESS:
The patient with multinodular goiter previously evaluated for dysphagia in January 2020 presents with progressive fatigue, cold intolerance, and weight gain that have developed over the past 3 months. She has noticed constipation and dry skin despite no changes in her diet or activity level. She recently underwent coronary angiography requiring iodinated contrast.

PAST MEDICAL HISTORY:
Active Problem List:
- Multinodular goiter (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Coronary angiography via femoral artery
- Total thyroidectomy

MEDICATIONS:
She has since been taking an over-the-counter cough syrup containing high levels of iodine

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is unremarkable

SOCIAL HISTORY:
She does not smoke or drink alcohol

VITAL SIGNS:
Her pulse is 58/min and regular

PHYSICAL EXAMINATION:
On physical examination, she appears tired, with a slightly enlarged, non-tender thyroid gland. Deep tendon reflexes show delayed relaxation.

LABORATORY DATA:
Elevated TSH and low free T4
