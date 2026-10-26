# synthetic_hospital patient 2322

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, White, non-Hispanic, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-01-31 (Wed) | outpatient | Neurology |
| 1 | 2022-01-15 | 2026-02-06 (Fri) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman, currently on disability",
 "alcohol_use": "None",
 "family_history": [
  "Father: Myocardial infarction at age 67, deceased",
  "Mother: Ischemic stroke at age 74",
  "Brother: Opioid use disorder",
  "Sister: Hypertension"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Former smoker, quit 2018, 35 pack-year history",
 "home_medications": [
  {
   "dose": "10 mg PO q6h PRN severe pain",
   "name": "Oxycodone"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "300 mg PO TID",
   "name": "Gabapentin"
  }
 ],
 "surgical_history": [
  "L4-L5 lumbar discectomy 2017",
  "L4-L5 posterior spinal fusion 2019",
  "Left knee arthroscopy 2015"
 ],
 "chronic_conditions": [
  "Opioid use disorder, severe",
  "Chronic lower back pain with radiculopathy",
  "Hypertension",
  "Hyperlipidemia",
  "Nicotine dependence, in sustained remission"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Hypoxic-ischemic encephalopathy", "Anterograde amnesia due to hippocampal injury"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-01-31, outpatient, Neurology

Chief complaint: Memory loss

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Memory loss

HISTORY OF PRESENT ILLNESS:
3 weeks after surviving a cardiac arrest. He was successfully resuscitated after 8 minutes of pulselessness and spent 2 weeks in the intensive care unit. Since discharge, he has difficulty forming new memories and often repeats the same questions minutes after being given the answers. He recalls the names of his children and major life events but cannot remember events that happened earlier the same day.

PAST MEDICAL HISTORY:
Active Problem List:
- Opioid use disorder, severe
- Chronic lower back pain with radiculopathy
- Hypertension
- Hyperlipidemia
- Nicotine dependence, in sustained remission

PAST SURGICAL HISTORY:
- L4-L5 lumbar discectomy 2017
- L4-L5 posterior spinal fusion 2019
- Left knee arthroscopy 2015

MEDICATIONS:
Home Medications:
- Oxycodone 10 mg PO q6h PRN severe pain
- Atorvastatin 40 mg PO daily
- Lisinopril 20 mg PO daily
- Aspirin 81 mg PO daily
- Gabapentin 300 mg PO TID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 67, deceased
- Mother: Ischemic stroke at age 74
- Brother: Opioid use disorder
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 2018, 35 pack-year history. Alcohol: None. Occupation: Retired construction foreman, currently on disability.

PHYSICAL EXAMINATION:
Neurologic exam is unremarkable except for mild inattentiveness.

## Encounter 1: source 2022-01-15, practice 2026-02-06, icu, Critical Care Medicine

Chief complaint: Found unresponsive in his home after a suspected opioid overdose

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Found unresponsive in his home after a suspected opioid overdose

HISTORY OF PRESENT ILLNESS:
This patient with opioid use disorder and prior hippocampal injury is intubated and resuscitated in the emergency department. After 48 hours, he remains in coma.

PAST MEDICAL HISTORY:
Active Problem List:
- Anterograde amnesia due to hippocampal injury (diagnosed 2020-01-15)
- Opioid use disorder, severe
- Chronic lower back pain with radiculopathy
- Hypertension
- Hyperlipidemia
- Nicotine dependence, in sustained remission

PAST SURGICAL HISTORY:
- L4-L5 lumbar discectomy 2017
- L4-L5 posterior spinal fusion 2019
- Left knee arthroscopy 2015

MEDICATIONS:
Home Medications:
- Oxycodone 10 mg PO q6h PRN severe pain
- Atorvastatin 40 mg PO daily
- Lisinopril 20 mg PO daily
- Aspirin 81 mg PO daily
- Gabapentin 300 mg PO TID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 67, deceased
- Mother: Ischemic stroke at age 74
- Brother: Opioid use disorder
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 2018, 35 pack-year history. Alcohol: None. Occupation: Retired construction foreman, currently on disability.

PHYSICAL EXAMINATION:
Neurologic examination shows no purposeful movement

LABORATORY DATA:
Arterial blood gas reveals a pH of 7.11, pCO₂ of 68 mm Hg, and pO₂ of 41 mm Hg.

IMAGING:
Head CT shows no signs of hemorrhage or infarct.

OTHER STUDIES:
EEG demonstrates diffuse slowing
