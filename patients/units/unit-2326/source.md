# synthetic_hospital patient 2326

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 74 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-06 (Wed) | outpatient | Neurology |
| 1 | 2021-01-15 | 2025-10-21 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfa drugs (rash)"
 ],
 "insurance": "Medicare",
 "occupation": "Retired electrical engineer",
 "alcohol_use": "None",
 "family_history": [
  "Father: Alzheimer disease (onset age 76, deceased)",
  "Mother: Ischemic stroke (age 80)",
  "Paternal uncle: Intracerebral hemorrhage (age 72)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 20 years ago (30 pack-years)",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Donepezil"
  },
  {
   "dose": "10 mg PO twice daily",
   "name": "Memantine"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "80 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Right total knee arthroplasty (2018)",
  "Cataract extraction with intraocular lens implantation, bilateral (2020)"
 ],
 "chronic_conditions": [
  "Alzheimer disease dementia",
  "Hypertension",
  "Hyperlipidemia",
  "History of ischemic stroke (6 months prior to initial encounter)"
 ],
 "age_at_first_encounter": 74
}
```

Primary diagnoses: ["Cerebral amyloid angiopathy", "Neuropathic pain syndrome"]
Comorbidities: ["History of stroke"]

## Encounter 0: source 2020-01-15, practice 2024-11-06, outpatient, Neurology

Chief complaint: Severe, persistent burning pain and allodynia involving his entire left arm and leg

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Severe, persistent burning pain and allodynia involving his entire left arm and leg

HISTORY OF PRESENT ILLNESS:
He presents with severe, persistent burning pain and allodynia involving his entire left arm and leg that began several weeks ago. He reports no history of trauma or prior neuropathy. He now has difficulty sleeping due to the intensity of the pain and describes it as "burning and electric."

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease dementia
- Hypertension
- Hyperlipidemia
- History of ischemic stroke (6 months prior to initial encounter)

PAST SURGICAL HISTORY:
- Right total knee arthroplasty (2018)
- Cataract extraction with intraocular lens implantation, bilateral (2020)

MEDICATIONS:
Home Medications:
- Donepezil 10 mg PO daily
- Memantine 10 mg PO twice daily
- Amlodipine 5 mg PO daily
- Atorvastatin 80 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Alzheimer disease (onset age 76, deceased)
- Mother: Ischemic stroke (age 80)
- Paternal uncle: Intracerebral hemorrhage (age 72)

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (30 pack-years). Alcohol: None. Occupation: Retired electrical engineer.

PHYSICAL EXAMINATION:
Neurological examination reveals normal muscle strength and deep tendon reflexes in all extremities. Sensory testing shows marked hypoesthesia to all modalities over the left side, while the right side is intact. There is no evidence of muscle atrophy, fasciculations, or abnormal movements.

## Encounter 1: source 2021-01-15, practice 2025-10-21, ed, Emergency Medicine

Chief complaint: New left-sided weakness and confusion

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
New left-sided weakness and confusion

HISTORY OF PRESENT ILLNESS:
This 75-year-old male with Alzheimer disease dementia and prior ischemic stroke returns one year after neurology evaluation for neuropathic pain syndrome. He is brought by his daughter after she noticed new left-sided weakness and confusion. He has had multiple falls over the past year with rapidly declining memory.

PAST MEDICAL HISTORY:
Active Problem List:
- Neuropathic pain syndrome (diagnosed 2020-01-15)
- Alzheimer disease dementia
- Hypertension
- Hyperlipidemia
- History of ischemic stroke (6 months prior to initial encounter)

The patient has a history of Alzheimer disease. She also mentions that he had a prior stroke six months ago.

PAST SURGICAL HISTORY:
- Right total knee arthroplasty (2018)
- Cataract extraction with intraocular lens implantation, bilateral (2020)

MEDICATIONS:
Home Medications:
- Donepezil 10 mg PO daily
- Memantine 10 mg PO twice daily
- Amlodipine 5 mg PO daily
- Atorvastatin 80 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Alzheimer disease (onset age 76, deceased)
- Mother: Ischemic stroke (age 80)
- Paternal uncle: Intracerebral hemorrhage (age 72)

SOCIAL HISTORY:
He lives independently

VITAL SIGNS:
His blood pressure is 120/75 mm Hg, heart rate is 72/min, respiratory rate is 14/min, and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
On examination, he has decreased strength in the left upper and lower extremities and is disoriented to time.
