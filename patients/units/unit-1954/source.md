# synthetic_hospital patient 1954

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Private PPO (Aetna).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-06-29 (Thu) | outpatient | Neurology |
| 1 | 2022-07-15 | 2025-12-25 (Thu) | outpatient | Neurosurgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private PPO (Aetna)",
 "occupation": "Elementary school music teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Father with neurofibromatosis type 2 (bilateral vestibular schwannomas)",
  "Paternal grandmother with early-onset deafness"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25 mg PO PRN dizziness",
   "name": "Meclizine"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "Excision of cutaneous neurofibroma, right forearm, age 22"
 ],
 "chronic_conditions": [
  "Neurofibromatosis type 2"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Neurofibromatosis type 2", "Spinal meningioma"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2023-06-29, outpatient, Neurology

Chief complaint: progressive hearing loss in her right ear and recent onset of dizziness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive hearing loss in her right ear and recent onset of dizziness

HISTORY OF PRESENT ILLNESS:
A 28-year-old woman presents to the clinic with a 6-month history of progressive hearing loss in her right ear and recent onset of dizziness. She denies otalgia, otorrhea, or a history of ototoxic medication use.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurofibromatosis type 2

Her past medical history is significant for removal of a cutaneous neurofibroma at age 22.

PAST SURGICAL HISTORY:
- Excision of cutaneous neurofibroma, right forearm, age 22

MEDICATIONS:
Home Medications:
- Meclizine 25 mg PO PRN dizziness
- Multivitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Family history reveals that her father had surgery for a spinal tumor and her paternal uncle has a history of "brain tumors."

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school music teacher.

PHYSICAL EXAMINATION:
On physical examination, she has decreased hearing on the right side as confirmed by Rinne and Weber tests.

IMAGING:
MRI of the brain with gadolinium contrast reveals a 1.5 cm mass at the right cerebellopontine angle.

OTHER STUDIES:
An audiogram shows sensorineural hearing loss in the right ear.

## Encounter 1: source 2022-07-15, practice 2025-12-25, outpatient, Neurosurgery

Chief complaint: intermittent urinary incontinence and progressive weakness of the legs

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Marcus Johnson, MD
Department: Neurosurgery
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent urinary incontinence and progressive weakness of the legs

HISTORY OF PRESENT ILLNESS:
Patient with NF2 returns, reporting a 3-month history of intermittent urinary incontinence and progressive weakness of the legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurofibromatosis type 2 (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Excision of cutaneous neurofibroma, right forearm, age 22

MEDICATIONS:
Home Medications:
- Meclizine 25 mg PO PRN dizziness
- Multivitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
She mentions that her mother had a serious operation for a hearing issue many years ago and that she was diagnosed for a chromosome 22 disorder, although she doesn't recall the specifics beyond being remembering the number 22.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school music teacher.

PHYSICAL EXAMINATION:
Relative afferent pupillary defect is negative. Babinksi sign is positive with 3+ left ankle reflexes on the left.

IMAGING:
An MRI shows a 1-cm mass at the level of the lumbar spine.
