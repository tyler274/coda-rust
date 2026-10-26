# synthetic_hospital patient 2809

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-21 (Thu) | outpatient | Neurology |
| 1 | 2021-07-15 | 2026-05-06 (Wed) | outpatient | Orthopedic Surgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Graphic Designer",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother: Hypothyroidism",
  "Father: Type 2 Diabetes Mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "75 mcg daily",
   "name": "Levothyroxine"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 7)",
  "Cholecystectomy (age 35)"
 ],
 "chronic_conditions": [
  "Hypothyroidism",
  "Obesity"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Carpal tunnel syndrome"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-11-21, outpatient, Neurology

Chief complaint: Progressive numbness and tingling in her right hand for the past three months, worse at night and interfering with work as a graphic designer

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive numbness and tingling in her right hand for the past three months, worse at night and interfering with work as a graphic designer

HISTORY OF PRESENT ILLNESS:
She works as a graphic designer and spends several hours daily at a keyboard. She describes intermittent burning and paresthesias that affect the thumb, index, middle, and half of the ring finger on the right hand. The symptoms are worse at night and occasionally wake her from sleep. She also reports dropping objects more frequently over the last few weeks. She denies neck pain, trauma, or weakness in the other limb.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism
- Obesity

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)
- Cholecystectomy (age 35)

MEDICATIONS:
Home Medications:
- Levothyroxine 75 mcg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Type 2 Diabetes Mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graphic Designer.

PHYSICAL EXAMINATION:
On physical exam, sensation is decreased over the palmar surface of the thumb and index finger, with mild thenar muscle flattening. Thumb abduction and opposition are weak. Tinel's and Phalen's signs are not elicited. The rest of the neurologic exam is normal.

## Encounter 1: source 2021-07-15, practice 2026-05-06, outpatient, Orthopedic Surgery

Chief complaint: Worsening numbness and tingling in her right hand with new difficulty opening jars and holding objects due to reduced grip strength

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Orthopedic Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening numbness and tingling in her right hand with new difficulty opening jars and holding objects due to reduced grip strength

HISTORY OF PRESENT ILLNESS:
The patient with hypothyroidism and obesity returns for hand symptoms after a 2020 Neurology visit. Symptoms worsen at night and sometimes awaken her from sleep. She reports difficulty opening jars and holding a coffee mug due to reduced grip strength.

PAST MEDICAL HISTORY:
Active Problem List:
- Carpal tunnel syndrome (diagnosed 2020-01-15)
- Hypothyroidism
- Obesity

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)
- Cholecystectomy (age 35)

MEDICATIONS:
Home Medications:
- Levothyroxine 75 mcg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Type 2 Diabetes Mellitus

SOCIAL HISTORY:
The patient works as a data analyst and spends several hours each day typing.

PHYSICAL EXAMINATION:
On examination, sensation is decreased over the palmar aspect of the thumb, index, and middle fingers. There is atrophy of the muscles at the base of the thumb. Light tapping over the inner wrist reproduces her symptoms, and holding her wrists in forced flexion for 30 seconds also triggers the same discomfort.
