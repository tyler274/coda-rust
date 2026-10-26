# synthetic_hospital patient 2781

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex F, Non-Hispanic White, Private PPO (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-07 (Thu) | outpatient | Cardiology |
| 1 | 2020-07-15 | 2025-05-09 (Fri) | outpatient | Endocrinology |
| 2 | 2021-01-15 | 2025-11-10 (Mon) | outpatient | General Surgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO (employer-sponsored)",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Medullary thyroid carcinoma (diagnosed age 48)",
  "Paternal aunt: Pheochromocytoma",
  "Mother: Essential hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Cesarean section (2010)",
  "Tonsillectomy (1990)"
 ],
 "chronic_conditions": [
  "Paroxysmal hypertension"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Pheochromocytoma"]
Comorbidities: ["Secondary hypertension", "Multiple endocrine neoplasia type 2"]

## Encounter 0: source 2020-01-15, practice 2024-11-07, outpatient, Cardiology

Chief complaint: Episodic sensations of racing heartbeat, pounding headaches, and flushing occurring unpredictably with increased fatigue

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodic sensations of racing heartbeat, pounding headaches, and flushing occurring unpredictably with increased fatigue

HISTORY OF PRESENT ILLNESS:
That occur unpredictably and last for several minutes. She reports increased fatigue and occasional difficulty sleeping.

PAST MEDICAL HISTORY:
Active Problem List:
- Paroxysmal hypertension

She has no chronic illnesses.

PAST SURGICAL HISTORY:
- Cesarean section (2010)
- Tonsillectomy (1990)

MEDICATIONS:
She does not take medications

ALLERGIES:
NKDA

FAMILY HISTORY:
Her father died of a “thyroid problem” in his 40s, but no records are available.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: High school mathematics teacher.

REVIEW OF SYSTEMS:
She denies chest pain, vision changes, or weight loss.

VITAL SIGNS:
Physical examination today reveals a blood pressure of 120/76 mm Hg. However, when she transitions from sitting to standing, her blood pressure is remeasured at 198/102 mm Hg.

LABORATORY DATA:
Routine labs are within normal limits except for elevated plasma metanephrines.

OTHER STUDIES:
ECG and chest radiograph are unremarkable.

## Encounter 1: source 2020-07-15, practice 2025-05-09, outpatient, Endocrinology

Chief complaint: Palpable neck mass and complaints of diarrhea

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Okonkwo, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Palpable neck mass and complaints of diarrhea

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Paroxysmal hypertension

PAST SURGICAL HISTORY:
- Cesarean section (2010)
- Tonsillectomy (1990)

ALLERGIES:
NKDA

FAMILY HISTORY:
With a family history of multiple endocrine neoplasia type 2A (MEN 2A). She reports that her father had thyroid cancer and her sister was recently diagnosed with pheochromocytoma.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: High school mathematics teacher.

VITAL SIGNS:
Her blood pressure is 145/90 mmHg and heart rate is 98 bpm.

PHYSICAL EXAMINATION:
On physical examination, a firm, nontender nodule is palpated in the middle third of the thyroid gland.

LABORATORY DATA:
Laboratory tests reveal:

- Serum calcium: 10.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum calcitonin: 500 pg/mL (normal: <10 pg/mL)
- CEA (carcinoembryonic antigen): 10 ng/mL (normal: <5 ng/mL)

ASSESSMENT AND PLAN:
Given the patient's family history and clinical findings, you suspect medullary thyroid cancer (MTC).

Before proceeding with thyroidectomy,

## Encounter 2: source 2021-01-15, practice 2025-11-10, outpatient, General Surgery

Chief complaint: Scheduled for surgical resection of right adrenal mass with ongoing paroxysmal hypertension episodes

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Rodriguez, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Scheduled for surgical resection of right adrenal mass with ongoing paroxysmal hypertension episodes

HISTORY OF PRESENT ILLNESS:
The patient returns for preoperative evaluation following prior Cardiology and Endocrinology consultations over the past year for management of paroxysmal hypertension. She is scheduled for surgical resection of a right adrenal mass discovered during evaluation of paroxysmal hypertension. She reports episodes of pounding headache, palpitations, and diaphoresis that occur several times per week and last about 20–30 minutes. Blood pressure readings at home have shown intermittent spikes above 180/100 mm Hg, but she is otherwise normotensive between episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Pheochromocytoma (diagnosed 2020-01-15)
- Paroxysmal hypertension

PAST SURGICAL HISTORY:
- Cesarean section (2010)
- Tonsillectomy (1990)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Medullary thyroid carcinoma (diagnosed age 48)
- Paternal aunt: Pheochromocytoma
- Mother: Essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: High school mathematics teacher.

LABORATORY DATA:
Plasma and urinary catecholamines and metanephrines are markedly elevated.

IMAGING:
Imaging confirms a unilateral adrenal tumor with features concerning for pheochromocytoma.

ASSESSMENT AND PLAN:
Over the next few days, her blood pressure declines steadily without significant orthostasis, and her episodes of palpitations become less intense.

She is admitted for preoperative preparation and started on phenoxybenzamine.
