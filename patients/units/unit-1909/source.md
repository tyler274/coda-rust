# synthetic_hospital patient 1909

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-08-20 (Tue) | outpatient | Internal Medicine |
| 1 | 2021-03-15 | 2025-10-21 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Senior Marketing Consultant",
 "alcohol_use": "Social drinker (2-3 drinks on weekends)",
 "family_history": [
  "Mother with rheumatoid arthritis",
  "Paternal grandmother with epilepsy",
  "Father with type 2 diabetes"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "750 mg twice daily",
   "name": "Levetiracetam"
  },
  {
   "dose": "20 mg weekly (taken on Fridays)",
   "name": "Methotrexate"
  },
  {
   "dose": "1 mg daily (except Fridays)",
   "name": "Folic acid"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction"
 ],
 "chronic_conditions": [
  "Generalized epilepsy",
  "Rheumatoid arthritis"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Methotrexate-induced folate deficiency"]
Comorbidities: ["Alcohol use", "Inadequate food intake"]

## Encounter 0: source 2020-01-15, practice 2024-08-20, outpatient, Internal Medicine

Chief complaint: Fatigue, mild exertional dyspnea, and difficulty concentrating at work over the past month

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, mild exertional dyspnea, and difficulty concentrating at work over the past month

HISTORY OF PRESENT ILLNESS:
She reports a recent decrease in appetite due to a demanding work schedule.

PAST MEDICAL HISTORY:
Active Problem List:
- Generalized epilepsy
- Rheumatoid arthritis

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

MEDICATIONS:
She takes medication.

Home Medications:
- Levetiracetam 750 mg twice daily
- Methotrexate 20 mg weekly (taken on Fridays)
- Folic acid 1 mg daily (except Fridays)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Paternal grandmother with epilepsy
- Father with type 2 diabetes

SOCIAL HISTORY:
Drinks socially

PHYSICAL EXAMINATION:
On examination, she appears pale but has no focal neurologic deficits. Romberg testing is normal.

LABORATORY DATA:
Hemoglobin is 9.0 g/dL. MCV is 113 fL. Homocysteine is elevated.

## Encounter 1: source 2021-03-15, practice 2025-10-21, ed, Emergency Medicine

Chief complaint: Severe fatigue, worsening shortness of breath with minimal exertion, and near-syncope

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. James Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe fatigue, worsening shortness of breath with minimal exertion, and near-syncope

HISTORY OF PRESENT ILLNESS:
This 35-year-old female with generalized epilepsy and rheumatoid arthritis presents to the emergency department with recurrent symptoms as in January 2020. She reports fatigue, mild exertional dyspnea, difficulty concentrating at work over the past month, and a recent decrease in appetite due to a demanding work schedule.

PAST MEDICAL HISTORY:
Active Problem List:
- Methotrexate-induced folate deficiency (diagnosed 2020-01-15)
- Generalized epilepsy
- Rheumatoid arthritis

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

MEDICATIONS:
She takes she takes medication.

Home Medications:
- Levetiracetam 750 mg twice daily
- Methotrexate 20 mg weekly (taken on Fridays)
- Folic acid 1 mg daily (except Fridays)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Paternal grandmother with epilepsy
- Father with type 2 diabetes

SOCIAL HISTORY:
She drinks socially

PHYSICAL EXAMINATION:
On examination, she appears pale but has no focal neurologic deficits. Romberg testing is normal.

LABORATORY DATA:
Hemoglobin is 9.0 g/dL. MCV is 113 fL. Homocysteine is elevated.
