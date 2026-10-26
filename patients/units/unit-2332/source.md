# synthetic_hospital patient 2332

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 76 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-16 (Mon) | outpatient | Neurology |
| 1 | 2021-07-15 | 2026-06-12 (Fri) | outpatient | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Occasional social use (1-2 glasses of wine per month)",
 "family_history": [
  "Father: Alzheimer disease, onset age 78, deceased age 82",
  "Mother: Hypertension, stroke at age 80",
  "One older brother with Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 20 years ago (30 pack-year history)",
 "home_medications": [
  {
   "dose": "5 mg orally daily",
   "name": "Amlodipine"
  },
  {
   "dose": "20 mg orally daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg orally daily",
   "name": "Aspirin"
  },
  {
   "dose": "500 mg orally as needed for knee pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Cataract extraction with intraocular lens placement (right eye), 2019",
  "Cataract extraction with intraocular lens placement (left eye), 2020",
  "Inguinal hernia repair (left), 2015"
 ],
 "chronic_conditions": [
  "Essential hypertension",
  "Hyperlipidemia",
  "Osteoarthritis (bilateral knees)",
  "History of cerebral small vessel disease"
 ],
 "age_at_first_encounter": 76
}
```

Primary diagnoses: ["Normal pressure hydrocephalus"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-12-16, outpatient, Neurology

Chief complaint: progressive difficulty walking and recent urinary incontinence

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty walking and recent urinary incontinence

HISTORY OF PRESENT ILLNESS:
His daughter reports that he has also become forgetful over the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension
- Hyperlipidemia
- Osteoarthritis (bilateral knees)
- History of cerebral small vessel disease

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement (right eye), 2019
- Cataract extraction with intraocular lens placement (left eye), 2020
- Inguinal hernia repair (left), 2015

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg orally daily
- Atorvastatin 20 mg orally daily
- Aspirin 81 mg orally daily
- Acetaminophen 500 mg orally as needed for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alzheimer disease, onset age 78, deceased age 82
- Mother: Hypertension, stroke at age 80
- One older brother with Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (30 pack-year history). Alcohol: Occasional social use (1-2 glasses of wine per month). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
He walks with short, shuffling steps and has a wide-based, magnetic gait. On exam, he is alert and oriented but shows impaired recall and difficulty with executive tasks.

IMAGING:
MRI of the brain shows enlarged ventricles out of proportion to cortical atrophy, with normal sulcal size. There is no evidence of mass lesion or obstructive pathology.

## Encounter 1: source 2021-07-15, practice 2026-06-12, outpatient, Neurology

Chief complaint: progressively worsening memory and difficulty walking over the past 3 months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressively worsening memory and difficulty walking over the past 3 months

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up evaluation of progressive neurological symptoms, having been previously assessed in January 2020 for gait difficulty and urinary incontinence. She reports that he has become increasingly forgetful, often misplacing items and repeating questions. He has also had multiple falls, and she has noticed he sometimes urinates on himself before reaching the bathroom. He denies headaches, nausea, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Normal pressure hydrocephalus (diagnosed 2020-01-15)
- Essential hypertension
- Hyperlipidemia
- Osteoarthritis (bilateral knees)
- History of cerebral small vessel disease

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement (right eye), 2019
- Cataract extraction with intraocular lens placement (left eye), 2020
- Inguinal hernia repair (left), 2015

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg orally daily
- Atorvastatin 20 mg orally daily
- Aspirin 81 mg orally daily
- Acetaminophen 500 mg orally as needed for knee pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alzheimer disease, onset age 78, deceased age 82
- Mother: Hypertension, stroke at age 80
- One older brother with Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (30 pack-year history). Alcohol: Occasional social use (1-2 glasses of wine per month). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On examination, the patient walks with short, hesitant steps and needs assistance to maintain balance. Neurologic examination reveals mild cognitive impairment and impaired tandem gait but no focal neurological deficits.
