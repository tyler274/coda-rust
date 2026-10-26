# synthetic_hospital patient 2335

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 78 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2026-03-30 (Mon) | outpatient | Cardiology |
| 1 | 2020-02-15 | 2026-05-01 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired civil engineer",
 "alcohol_use": "Social, 1-2 glasses of wine per week",
 "family_history": [
  "Father: Coronary artery disease, myocardial infarction at age 82",
  "Mother: Hypertension, osteoporosis",
  "No family history of sudden cardiac death or pacemaker implantation"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 25 years ago (15 pack-years)",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 78
}
```

Primary diagnoses: ["Third-degree atrioventricular block"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2026-03-30, outpatient, Cardiology

Chief complaint: Intermittent episodes of dizziness and fatigue for the past 3 weeks, worse when rising from bed or walking

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent episodes of dizziness and fatigue for the past 3 weeks, worse when rising from bed or walking

HISTORY OF PRESENT ILLNESS:
He says the symptoms are worse when he tries to get up from bed or when walking. He is no longer able to perform moderate physical activity due to near-fainting spells. He denies chest pain, palpitations, or shortness of breath.

PAST MEDICAL HISTORY:
He has no known medical history and

MEDICATIONS:
He takes no medications.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 82
- Mother: Hypertension, osteoporosis
- No family history of sudden cardiac death or pacemaker implantation

SOCIAL HISTORY:
Smoking: Former smoker, quit 25 years ago (15 pack-years). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired civil engineer.

VITAL SIGNS:
His pulse is 35/min, respirations are 15/min, and supine blood pressure is 102/44 mm Hg.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a regular but very slow rhythm. Neurologic exam is non-focal.

OTHER STUDIES:
An ECG is performed and shown below.

## Encounter 1: source 2020-02-15, practice 2026-05-01, ed, Emergency Medicine

Chief complaint: Collapsing at home

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Collapsing at home

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department one month after cardiology evaluation. He is brought in after collapsing at home and reports several weeks of fatigue, lightheadedness, and intermittent episodes of blurred vision.

PAST MEDICAL HISTORY:
Active Problem List:
- Third-degree atrioventricular block (diagnosed 2020-01-15)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 82
- Mother: Hypertension, osteoporosis
- No family history of sudden cardiac death or pacemaker implantation

SOCIAL HISTORY:
Smoking: Former smoker, quit 25 years ago (15 pack-years). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired civil engineer.

VITAL SIGNS:
On examination, he has a HR of 31 and BP of 100/60

PHYSICAL EXAMINATION:
He is alert but slow to respond. Cardiopulmonary examination is unremarkable.

OTHER STUDIES:
An ECG is shown.
