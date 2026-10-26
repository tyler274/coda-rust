# synthetic_hospital patient 2175

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-30 (Wed) | outpatient | Neurology |
| 1 | 2020-07-15 | 2025-10-27 (Mon) | follow_up | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer (currently on medical leave)",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "No family history of Parkinson disease or movement disorders",
  "Father with hypertension",
  "Mother alive and well"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1/2 pack per day for 20 years",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["MPTP-induced parkinsonism"]
Comorbidities: ["Unspecified substance intoxication"]

## Encounter 0: source 2020-01-15, practice 2025-04-30, outpatient, Neurology

Chief complaint: 3-month history of progressive motor symptoms including stiffness, difficulty initiating movement, and resting tremor

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of progressive motor symptoms including stiffness, difficulty initiating movement, and resting tremor

HISTORY OF PRESENT ILLNESS:
A 42-year-old man is brought to the clinic by his brother due to a 3-month history of progressive motor symptoms. He complains of increasing stiffness in his limbs, difficulty initiating movement, and occasional resting tremor in his right hand. He reports a general sense of slowed movement and a shuffling gait that has made it harder to keep up with work at the factory.

PAST MEDICAL HISTORY:
The patient has no significant past medical history

MEDICATIONS:
He takes no prescription medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- No family history of Parkinson disease or movement disorders
- Father with hypertension
- Mother alive and well

SOCIAL HISTORY:
When asked about drug use, he initially denies any, but his brother later mentions that the patient has recently used an unfamiliar street drug.

PHYSICAL EXAMINATION:
Neurologic examination reveals masked facies, cogwheel rigidity, and mild bradykinesia. There is no postural instability or ocular movement abnormality.

## Encounter 1: source 2020-07-15, practice 2025-10-27, follow_up, Neurology

Chief complaint: Follow-up evaluation of progressive motor symptoms and response to therapy

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up evaluation of progressive motor symptoms and response to therapy

HISTORY OF PRESENT ILLNESS:
The patient returns to the neurology clinic for follow-up evaluation six months after his initial presentation and is brought by his brother due to a 3-month history of progressive motor symptoms. He complains of increasing stiffness in his limbs, difficulty initiating movement, and occasional resting tremor in his right hand. He reports a general sense of slowed movement and a shuffling gait that has made it harder to keep up with work at the factory.

PAST MEDICAL HISTORY:
Active Problem List:
- MPTP-induced parkinsonism (diagnosed 2020-01-15)

MEDICATIONS:
He takes no prescription medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- No family history of Parkinson disease or movement disorders
- Father with hypertension
- Mother alive and well

SOCIAL HISTORY:
When asked about drug use, he initially denies any, but his brother later mentions that the patient has recently used an unfamiliar street drug.

PHYSICAL EXAMINATION:
Neurologic examination reveals masked facies, cogwheel rigidity, and mild bradykinesia. There is no postural instability or ocular movement abnormality.
