# synthetic_hospital patient 2161

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 47 at first encounter, sex M, Hispanic or Latino, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-09 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-09-15 | 2025-12-30 (Tue) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Warehouse forklift operator (evening shift)",
 "alcohol_use": "Rare (1-2 drinks per month)",
 "family_history": [
  "Hypertension, father",
  "Type 2 diabetes mellitus, mother"
 ],
 "race_ethnicity": "Hispanic or Latino",
 "smoking_status": "Former smoker, 8 pack-years, quit 2 years ago",
 "home_medications": [],
 "surgical_history": [
  "Appendectomy, age 22"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 47
}
```

Primary diagnoses: ["Acute pharyngitis"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-05-09, outpatient, Internal Medicine

Chief complaint: Sore throat for almost a week

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Sore throat for almost a week

HISTORY OF PRESENT ILLNESS:
He presents to the primary care clinic at 7:45 PM on a Friday evening, fifteen minutes after the clinic’s scheduled closing time. The staff has already begun shutting down for the night, and the front desk receptionist informs the physician that the patient insisted on being seen. When approached, the patient says, "I know this is late, but I just got off work and I've had this sore throat for almost a week. I can't miss any more hours or I'll risk losing my job. I'm sure it's just strep, and I'd really appreciate it if you could take a quick look."

PAST MEDICAL HISTORY:
Review of his chart reveals that he has shown up without appointments on several prior occasions for minor concerns such as back pain and sinus congestion. He is otherwise healthy

PAST SURGICAL HISTORY:
- Appendectomy, age 22

MEDICATIONS:
He takes no regular medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension, father
- Type 2 diabetes mellitus, mother

SOCIAL HISTORY:
Just got off work and I can't miss any more hours or I'll risk losing my job

PHYSICAL EXAMINATION:
The patient appears well and is in no respiratory distress.

## Encounter 1: source 2020-09-15, practice 2025-12-30, outpatient, Internal Medicine

Chief complaint: Insistence on evaluation after clinic closing; patient states 'I know this...' regarding his condition

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Insistence on evaluation after clinic closing; patient states 'I know this...' regarding his condition

PAST MEDICAL HISTORY:
Active Problem List:
- Acute pharyngitis (diagnosed 2020-01-15)

Review of his chart reveals that he has shown up without appointments on several prior occasions for minor concerns such as back pain and sinus congestion. He is otherwise healthy.

PAST SURGICAL HISTORY:
- Appendectomy, age 22

MEDICATIONS:
He takes no regular medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension, father
- Type 2 diabetes mellitus, mother

SOCIAL HISTORY:
Smoking: Former smoker, 8 pack-years, quit 2 years ago. Alcohol: Rare (1-2 drinks per month). Occupation: Warehouse forklift operator (evening shift).

PHYSICAL EXAMINATION:
The patient appears well and is in no respiratory distress.
