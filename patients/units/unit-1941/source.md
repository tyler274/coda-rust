# synthetic_hospital patient 1941

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-23 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2025-03-24 (Mon) | outpatient | Cardiology |
| 2 | 2020-10-15 | 2025-09-08 (Mon) | outpatient | Internal Medicine |
| 3 | 2021-07-15 | 2026-06-10 (Wed) | outpatient | Cardiology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield",
 "occupation": "High School Science Teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother with hypertension diagnosed age 32",
  "Paternal uncle with renal artery stenosis",
  "Father with ischemic stroke at age 48"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Fibromuscular dysplasia", "Renal artery stenosis"]
Comorbidities: ["Secondary hypertension due to renal artery stenosis"]

## Encounter 0: source 2020-01-15, practice 2024-12-23, outpatient, Internal Medicine

Chief complaint: Evaluation of newly diagnosed hypertension with headaches and fatigue over several weeks.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of newly diagnosed hypertension with headaches and fatigue over several weeks.

HISTORY OF PRESENT ILLNESS:
She reports headaches and fatigue over several weeks but denies chest pain, dyspnea, or palpitations.

MEDICATIONS:
She takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with hypertension diagnosed age 32
- Paternal uncle with renal artery stenosis
- Father with ischemic stroke at age 48

SOCIAL HISTORY:
She exercises regularly, does not smoke

VITAL SIGNS:
BP is 168/100 mmHg bilaterally; BMI is 23 kg/m².

PHYSICAL EXAMINATION:
On examination, a right-sided abdominal bruit is audible near the umbilicus. Fundoscopic exam is normal.

LABORATORY DATA:
Laboratory tests including renal function and electrolytes are normal.

IMAGING:
Renal angiography is shown below.

## Encounter 1: source 2020-04-15, practice 2025-03-24, outpatient, Cardiology

Chief complaint: Persistent headaches and pulsatile 'whooshing' sound in right ear for the past month.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent headaches and pulsatile 'whooshing' sound in right ear for the past month.

HISTORY OF PRESENT ILLNESS:
She returns following her January 2020 evaluation for hypertension. She also notes occasional neck stiffness and was found to have elevated blood pressure at a pharmacy visit.

PAST MEDICAL HISTORY:
Active Problem List:
- Fibromuscular dysplasia (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with hypertension diagnosed age 32
- Paternal uncle with renal artery stenosis
- Father with ischemic stroke at age 48

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: High School Science Teacher.

REVIEW OF SYSTEMS:
She has no asthma, sinus symptoms, hematuria, or skin findings.
She denies any history of fever, rash, neuropathy, or hemoptysis.

VITAL SIGNS:
Today, her blood pressure is 160/100 mmHg in both arms.

PHYSICAL EXAMINATION:
On physical examination, there is a bruit over the right carotid artery. Neurologic exam is normal.

LABORATORY DATA:
Laboratory evaluation shows high renin and aldosterone. She is negative for hepatitis B and C. Antinuclear antibody (ANA) is negative; ANCA results are unclear.

IMAGING:
CT angiography of the head and neck reveals alternating areas of stenosis and dilation in the internal carotid artery, and renal artery imaging shows a similar pattern in the renal vessels.

## Encounter 2: source 2020-10-15, practice 2025-09-08, outpatient, Internal Medicine

Chief complaint: Evaluation of persistently elevated blood pressure despite lifestyle modifications.

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of persistently elevated blood pressure despite lifestyle modifications.

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of persistently elevated blood pressure despite lifestyle modifications, following previous presentations for headaches and pulsatile whooshing sounds. She reports occasional headaches and fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- Fibromuscular dysplasia (diagnosed 2020-01-15)
- Renal artery stenosis (diagnosed 2020-04-15)

MEDICATIONS:
She is not taking any medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with hypertension diagnosed age 32
- Paternal uncle with renal artery stenosis
- Father with ischemic stroke at age 48

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: High School Science Teacher.

VITAL SIGNS:
Her blood pressure is 160/100 mm Hg in both arms

PHYSICAL EXAMINATION:
On auscultation of the abdomen, a high-pitched bruit is noted just to the right of midline

LABORATORY DATA:
Urine pregnancy test is negative

## Encounter 3: source 2021-07-15, practice 2026-06-10, outpatient, Cardiology

Chief complaint: Evaluation of persistent headaches and worsening blood pressure control despite medication adherence.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of persistent headaches and worsening blood pressure control despite medication adherence.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of persistent headaches and blood pressure management, having presented in 2020 with associated fatigue, pulsatile whooshing in the right ear, and persistent elevations despite lifestyle modifications. Despite adherence, her systolic pressures remain in the 160–170 mm Hg range.

PAST MEDICAL HISTORY:
Active Problem List:
- Fibromuscular dysplasia (diagnosed 2020-01-15)
- Renal artery stenosis (diagnosed 2020-04-15)

She was diagnosed with hypertension 2 years ago. She has no personal history of cardiovascular disease, diabetes, or hyperlipidemia.

MEDICATIONS:
Currently taking amlodipine

ALLERGIES:
NKDA

FAMILY HISTORY:
No family history of cardiovascular disease, diabetes, or hyperlipidemia

SOCIAL HISTORY:
She does not smoke or drink alcohol.

REVIEW OF SYSTEMS:
Review of systems is unremarkable except for occasional lightheadedness.

PHYSICAL EXAMINATION:
Physical examination shows no focal neurologic deficits. Bruits are audible over the flanks bilaterally.

LABORATORY DATA:
Laboratory testing is notable for a serum creatinine of 0.9 mg/dL and elevated plasma renin activity.
