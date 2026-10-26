# synthetic_hospital patient 2000

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex M, Non-Hispanic White, Employer-sponsored commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-27 (Sat) | ed | Emergency Medicine |
| 1 | 2020-01-15 | 2025-09-27 (Sat) | inpatient | Cardiothoracic Surgery |
| 2 | 2020-07-15 | 2026-04-05 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored commercial PPO",
 "occupation": "Registered Nurse - Urgent Care Clinic",
 "alcohol_use": "Occasional social use (1-2 drinks per month)",
 "family_history": [
  "Father with abdominal aortic aneurysm",
  "Paternal grandfather with sudden cardiac death at age 50",
  "Mother with hypertension and type 2 diabetes"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day for 30 years (30 pack-years)",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Hypertension"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Aortic dissection", "Type A Aortic Dissection"]
Comorbidities: ["Aortic Regurgitation", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2025-09-27, ed, Emergency Medicine

Chief complaint: sudden onset of severe chest pain radiating to the back after lifting heavy furniture

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe chest pain radiating to the back after lifting heavy furniture

HISTORY OF PRESENT ILLNESS:
The pain is described as tearing in nature and began while the patient was lifting heavy furniture.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with abdominal aortic aneurysm
- Paternal grandfather with sudden cardiac death at age 50
- Mother with hypertension and type 2 diabetes

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 30 years (30 pack-years). Alcohol: Occasional social use (1-2 drinks per month). Occupation: Registered Nurse - Urgent Care Clinic.

VITAL SIGNS:
His blood pressure is 180/100 mmHg, heart rate is 110 bpm, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
He appears diaphoretic and anxious. On examination, you note a faint diastolic murmur. His pulses are palpable but there is a discrepancy in blood pressure between the right and left arms.

IMAGING:
Chest X-ray reveals a widened mediastinum.

OTHER STUDIES:
The ECG shows nonspecific ST and T wave changes.

ASSESSMENT AND PLAN:
The patient is currently hemodynamically stable.

## Encounter 1: source 2020-01-15, practice 2025-09-27, inpatient, Cardiothoracic Surgery

Chief complaint: ongoing tearing chest pain radiating to the back, admitted for definitive management

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. James Wilson, MD
Department: Cardiothoracic Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Ongoing tearing chest pain radiating to the back, admitted for definitive management

HISTORY OF PRESENT ILLNESS:
The patient with hypertension returns for definitive management of tearing pain that began 2 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Type A Aortic Dissection (diagnosed 2020-01-15)
- Hypertension

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with abdominal aortic aneurysm
- Paternal grandfather with sudden cardiac death at age 50
- Mother with hypertension and type 2 diabetes

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 30 years (30 pack-years). Alcohol: Occasional social use (1-2 drinks per month). Occupation: Registered Nurse - Urgent Care Clinic.

VITAL SIGNS:
His blood pressure is 180/100 mmHg, heart rate is 110 bpm, and he is diaphoretic.

PHYSICAL EXAMINATION:
Physical examination reveals a diastolic murmur best heard over the right sternal border. His pulses are palpable and symmetric in all extremities.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 14 g/dL
- White blood cell count: 12,000/mm^3
- Platelets: 250,000/mm^3
- Serum creatinine: 1.1 mg/dL
- Troponin I: <0.04 ng/mL (normal)

IMAGING:
A chest X-ray reveals a widened mediastinum.

OTHER STUDIES:
An electrocardiogram shows no evidence of ischemia or infarction.

## Encounter 2: source 2020-07-15, practice 2026-04-05, ed, Emergency Medicine

Chief complaint: sudden-onset severe upper back pain radiating between his scapulae

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset severe upper back pain radiating between his scapulae

HISTORY OF PRESENT ILLNESS:
This 52-year-old male with prior Type A aortic dissection presents with similar symptoms. He describes the pain as tearing and worse than anything he has ever felt before. He is diaphoretic and reports that he initially thought he was having a heart attack.

PAST MEDICAL HISTORY:
Active Problem List:
- Type A Aortic Dissection (diagnosed 2020-01-15)
- Aortic dissection (diagnosed 2020-01-15)
- Hypertension

He has a history of hypertension.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with abdominal aortic aneurysm
- Paternal grandfather with sudden cardiac death at age 50
- Mother with hypertension and type 2 diabetes

SOCIAL HISTORY:
He recently began working as a nurse in a busy urgent care clinic. and is a long-term smoker.

VITAL SIGNS:
His blood pressure is 188/92 mm Hg, heart rate is 106/min, respiratory rate is 20/min, temperature is 36.8°C (98.2°F), and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
On examination, he appears anxious and uncomfortable.

IMAGING:
A chest X-ray is shown.

OTHER STUDIES:
An ECG shows nonspecific ST-T changes
