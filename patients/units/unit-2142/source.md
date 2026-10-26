# synthetic_hospital patient 2142

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 40 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-04-24 (Wed) | outpatient | Internal Medicine |
| 1 | 2022-01-15 | 2026-04-29 (Wed) | outpatient | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Unemployed former office administrator",
 "alcohol_use": "Social, 1-2 drinks per month",
 "family_history": [
  "Father: Huntington disease (onset age 42, deceased age 58)",
  "Mother: Hypothyroidism",
  "Paternal grandfather: 'Nervous disorder' (suspected Huntington disease)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "0.5 mg PO twice daily as needed for anxiety",
   "name": "Clonazepam"
  }
 ],
 "surgical_history": [
  "Appendectomy (age 16)"
 ],
 "chronic_conditions": [
  "Major depressive disorder",
  "Generalized anxiety disorder",
  "Essential hypertension"
 ],
 "age_at_first_encounter": 40
}
```

Primary diagnoses: ["Huntington disease"]
Comorbidities: ["Risk of Alzheimer's disease"]

## Encounter 0: source 2020-01-15, practice 2024-04-24, outpatient, Internal Medicine

Chief complaint: Brought to the clinic by his wife due to personality changes and unusual movements.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Brought to the clinic by his wife due to personality changes and unusual movements.

HISTORY OF PRESENT ILLNESS:
Over the past year, he has become increasingly irritable and forgetful. He recently lost his job due to difficulties concentrating and inappropriate behavior with coworkers.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder
- Generalized anxiety disorder
- Essential hypertension

PAST SURGICAL HISTORY:
- Appendectomy (age 16)

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Lisinopril 10 mg PO daily
- Clonazepam 0.5 mg PO twice daily as needed for anxiety

ALLERGIES:
NKDA

FAMILY HISTORY:
His father died in his 50s from a similar condition.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Unemployed former office administrator.

PHYSICAL EXAMINATION:
Neurologic examination shows purposeless, involuntary movements of his arms and facial grimacing.

IMAGING:
MRI of the brain is performed.

## Encounter 1: source 2022-01-15, practice 2026-04-29, outpatient, Neurology

Chief complaint: Abnormal movements and behavioral changes.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Okafor, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Abnormal movements and behavioral changes.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of symptoms initially assessed in 2020 and is brought to the clinic by his wife because of several months of abnormal movements and behavioral changes. His wife reports recent forgetfulness, irritability, and impulsive behavior.

PAST MEDICAL HISTORY:
Active Problem List:
- Huntington disease (diagnosed 2020-01-15)
- Major depressive disorder
- Generalized anxiety disorder
- Essential hypertension

PAST SURGICAL HISTORY:
- Appendectomy (age 16)

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Lisinopril 10 mg PO daily
- Clonazepam 0.5 mg PO twice daily as needed for anxiety

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is significant for a similar disorder in his father.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: Unemployed former office administrator.

PHYSICAL EXAMINATION:
On exam, he has continuous, involuntary, irregular, purposeless movements of his arms and face.

IMAGING:
MRI shows atrophy of a basal ganglia structure.
