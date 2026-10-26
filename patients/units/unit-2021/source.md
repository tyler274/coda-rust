# synthetic_hospital patient 2021

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, African American, Medicare (disability) and supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-13 (Thu) | outpatient | Nephrology |
| 1 | 2020-07-15 | 2023-04-21 (Fri) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2023-10-18 (Wed) | outpatient | Neurology |
| 3 | 2021-07-15 | 2024-04-08 (Mon) | outpatient | Orthopedic Surgery |
| 4 | 2022-01-15 | 2024-10-07 (Mon) | outpatient | Neurosurgery |
| 5 | 2022-07-15 | 2025-04-04 (Fri) | outpatient | Cardiology |
| 6 | 2023-01-15 | 2025-10-16 (Thu) | ed | Emergency Medicine |
| 7 | 2023-07-15 | 2026-04-03 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (disability) and supplemental private insurance",
 "occupation": "Retired postal worker (currently disabled)",
 "alcohol_use": "None",
 "family_history": [
  "Father: Type 2 diabetes mellitus, myocardial infarction at age 60",
  "Mother: Hypertension, stroke at age 72",
  "Sibling: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Former smoker, 20 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Spironolactone"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (15-year history)",
  "Hypertension",
  "Diabetic Nephropathy",
  "Heart Failure with Reduced Ejection Fraction",
  "Multiple System Atrophy",
  "Cervical Spondylosis"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Ventricular Tachycardia", "Cervical spondylosis with myelopathy", "Hyperkalemia", "Cervical radiculopathy", "Multiple system atrophy", "Diabetic nephropathy", "Heart failure with reduced ejection fraction"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-10-13, outpatient, Nephrology

Chief complaint: Routine follow-up for chronic kidney disease with increased fatigue and frothy urine

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic kidney disease with increased fatigue and frothy urine

HISTORY OF PRESENT ILLNESS:
He reports increased fatigue and a frothy appearance to his urine.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension
- Diabetic Nephropathy
- Heart Failure with Reduced Ejection Fraction
- Multiple System Atrophy
- Cervical Spondylosis

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His diabetes has been managed with metformin and dietary modifications.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Spironolactone 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 60
- Mother: Hypertension, stroke at age 72
- Sibling: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker (currently disabled).

VITAL SIGNS:
His blood pressure is currently 145/90 mmHg.

PHYSICAL EXAMINATION:
Physical examination reveals mild bilateral lower extremity edema.

LABORATORY DATA:
- Serum creatinine: 1.8 mg/dL (increased from 1.2 mg/dL six months ago)
- Urine albumin-to-creatinine ratio: 400 mg/g
- Hemoglobin A1c: 8.5%
- Fasting blood glucose: 180 mg/dL
- Serum potassium: 4.5 mEq/L

## Encounter 1: source 2020-07-15, practice 2023-04-21, outpatient, Internal Medicine

Chief complaint: Routine follow-up with unintentional weight loss and persistent fatigue

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Okonkwo, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with unintentional weight loss and persistent fatigue

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of his diabetic nephropathy. He reports increased fatigue and recent unintentional weight loss of 5 kg over the past 3 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic nephropathy (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension
- Heart Failure with Reduced Ejection Fraction
- Multiple System Atrophy
- Cervical Spondylosis

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes metformin and dietary modifications.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Spironolactone 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 60
- Mother: Hypertension, stroke at age 72
- Sibling: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker (currently disabled).

VITAL SIGNS:
His blood pressure today is 142/88 mmHg.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral pedal edema.

LABORATORY DATA:
- Hemoglobin A1c: 7.8%
- Fasting blood glucose: 162 mg/dL
- Serum creatinine: 1.4 mg/dL (baseline 6 months ago was 1.0 mg/dL)
- Urine albumin-creatinine ratio: 150 mg/g

## Encounter 2: source 2021-01-15, practice 2023-10-18, outpatient, Neurology

Chief complaint: Progressive difficulty with balance, coordination, and autonomic dysfunction

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty with balance, coordination, and autonomic dysfunction

HISTORY OF PRESENT ILLNESS:
The patient returns for Neurology evaluation with Type 2 diabetes and nephropathy following encounters for chronic kidney disease and weight loss. He presents with a 2-year history of progressive difficulty with balance and coordination. He reports frequent falls and a shuffling gait. He has been experiencing erectile dysfunction, urinary urgency, and constipation.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic nephropathy (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension
- Heart Failure with Reduced Ejection Fraction
- Multiple System Atrophy
- Cervical Spondylosis

His past medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Spironolactone 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 60
- Mother: Hypertension, stroke at age 72
- Sibling: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker (currently disabled).

VITAL SIGNS:
His blood pressure is 145/90 mmHg while seated and 115/70 mmHg upon standing, with an associated increase in heart rate from 78 to 98 bpm.

PHYSICAL EXAMINATION:
On examination, he exhibits bradykinesia, rigidity, and postural instability.
There is no tremor noted.

## Encounter 3: source 2021-07-15, practice 2024-04-08, outpatient, Orthopedic Surgery

Chief complaint: Neck pain radiating down left arm with numbness in thumb and index finger

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Park, MD
Department: Orthopedic Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Neck pain radiating down left arm with numbness in thumb and index finger

HISTORY OF PRESENT ILLNESS:
The patient, a 59-year-old male with Type 2 Diabetes Mellitus, diabetic nephropathy, and recently diagnosed Multiple System Atrophy, presents to the Orthopedic Surgery clinic with a 6-week history of neck pain that radiates down his left arm. He describes the pain as a sharp, burning sensation that worsens with neck extension and turning his head to the left. He also reports numbness in his left thumb and index finger.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic nephropathy (diagnosed 2020-01-15)
- Multiple system atrophy (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension
- Heart Failure with Reduced Ejection Fraction
- Cervical Spondylosis

His medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Spironolactone 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 60
- Mother: Hypertension, stroke at age 72
- Sibling: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker (currently disabled).

PHYSICAL EXAMINATION:
On physical examination, there is decreased sensation to light touch over the left thumb and index finger, and weakness in the left biceps and wrist extensors. The biceps reflex is diminished on the left side.

IMAGING:
Cervical spine MRI reveals degenerative changes with osteophyte formation and foraminal narrowing at the C5-C6 level.

## Encounter 4: source 2022-01-15, practice 2024-10-07, outpatient, Neurosurgery

Chief complaint: Progressive right hand weakness with difficulty performing fine motor tasks

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Neurosurgery
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive right hand weakness with difficulty performing fine motor tasks

HISTORY OF PRESENT ILLNESS:
The patient presents with new right hand symptoms, returning six months after orthopedic evaluation for left neck pain with arm numbness, in the setting of multiple system atrophy and diabetic nephropathy. He reports 3-month progressive right hand weakness with difficulty buttoning shirts and dropping objects. He denies pain, numbness, or tingling in affected hand.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic nephropathy (diagnosed 2020-01-15)
- Multiple system atrophy (diagnosed 2021-01-15)
- Cervical radiculopathy (diagnosed 2021-07-15)
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension
- Heart Failure with Reduced Ejection Fraction
- Cervical Spondylosis

His past medical history is significant for hypertension and type 2 diabetes mellitus, which are well-controlled.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Atorvastatin 40 mg PO daily
- Spironolactone 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 60
- Mother: Hypertension, stroke at age 72
- Sibling: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker (currently disabled).

PHYSICAL EXAMINATION:
On physical examination, the patient has normal muscle bulk in both upper extremities. When asked to outstretch his arms with palms up and eyes closed, his right arm drifts downward and the palm pronates. His grip strength is 4/5 on the right and 5/5 on the left. Reflexes are 3+ in the right biceps and brachioradialis, and 2+ on the left. There is no sensory deficit noted. Babinski sign is upgoing on the right side.

LABORATORY DATA:
Laboratory values:
- Hemoglobin A1c: 6.8%
- Fasting blood glucose: 132 mg/dL
- Serum creatinine: 1.0 mg/dL
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 110 mg/dL
- HDL cholesterol: 45 mg/dL

## Encounter 5: source 2022-07-15, practice 2025-04-04, outpatient, Cardiology

Chief complaint: Progressive shortness of breath on exertion and bilateral ankle swelling

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Robert Johnson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath on exertion and bilateral ankle swelling

HISTORY OF PRESENT ILLNESS:
The patient with long-standing heart failure with reduced ejection fraction and multiple system atrophy returns presenting with progressive shortness of breath on exertion and bilateral ankle swelling over the past two months. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic nephropathy (diagnosed 2020-01-15)
- Multiple system atrophy (diagnosed 2021-01-15)
- Cervical radiculopathy (diagnosed 2021-07-15)
- Cervical spondylosis with myelopathy (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension
- Heart Failure with Reduced Ejection Fraction
- Cervical Spondylosis

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes metformin, lisinopril, and atorvastatin.

Home Medications:
- Spironolactone 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 60
- Mother: Hypertension, stroke at age 72
- Sibling: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker (currently disabled).

VITAL SIGNS:
Blood pressure is 142/88 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a third heart sound (S3) and jugular venous distension is noted. Lung fields are clear to auscultation. Extremities show 2+ pitting edema up to the mid-shins bilaterally.

LABORATORY DATA:
Serum sodium: 138 mEq/L
- Serum potassium: 4.5 mEq/L
- Serum creatinine: 1.2 mg/dL
- Fasting glucose: 158 mg/dL
- B-type natriuretic peptide (BNP): 580 pg/mL

IMAGING:
Chest X-ray: No acute cardiopulmonary process

OTHER STUDIES:
Echocardiogram shows left ventricular hypertrophy with an ejection fraction of 40%

## Encounter 6: source 2023-01-15, practice 2025-10-16, ed, Emergency Medicine

Chief complaint: Acute weakness and palpitations

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Williams, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute weakness and palpitations

HISTORY OF PRESENT ILLNESS:
The patient with long-standing diabetic nephropathy and heart failure with reduced ejection fraction presents to the ED with weakness and palpitations that started earlier today. He reports compliance with his medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic nephropathy (diagnosed 2020-01-15)
- Multiple system atrophy (diagnosed 2021-01-15)
- Cervical radiculopathy (diagnosed 2021-07-15)
- Cervical spondylosis with myelopathy (diagnosed 2022-01-15)
- Heart failure with reduced ejection fraction (diagnosed 2022-07-15)
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension
- Cervical Spondylosis

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes metformin, lisinopril, and a recently started potassium-sparing diuretic.

Home Medications:
- Atorvastatin 40 mg PO daily
- Spironolactone 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 60
- Mother: Hypertension, stroke at age 72
- Sibling: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker (currently disabled).

VITAL SIGNS:
Blood pressure is 138/86 mmHg, heart rate is 98 bpm

PHYSICAL EXAMINATION:
On examination, he appears mildly distressed.

LABORATORY DATA:
Laboratory results are as follows:

- Serum potassium: 6.5 mEq/L (normal: 3.5-5.0 mEq/L)
- Serum creatinine: 1.2 mg/dL (normal: 0.6-1.2 mg/dL)
- Blood urea nitrogen (BUN): 18 mg/dL (normal: 7-20 mg/dL)
- Serum sodium: 140 mEq/L (normal: 135-145 mEq/L)
- Serum glucose: 160 mg/dL (normal: 70-99 mg/dL)

OTHER STUDIES:
An ECG shows peaked T waves.

## Encounter 7: source 2023-07-15, practice 2026-04-03, ed, Emergency Medicine

Chief complaint: Acute palpitations and lightheadedness while exerting himself

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute palpitations and lightheadedness while exerting himself

HISTORY OF PRESENT ILLNESS:
The patient with HFrEF and prior hyperkalemia presents with palpitations and lightheadedness that started 30 minutes ago while he was mowing the lawn, denying chest pain, dyspnea, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic nephropathy (diagnosed 2020-01-15)
- Multiple system atrophy (diagnosed 2021-01-15)
- Cervical radiculopathy (diagnosed 2021-07-15)
- Cervical spondylosis with myelopathy (diagnosed 2022-01-15)
- Heart failure with reduced ejection fraction (diagnosed 2022-07-15)
- Hyperkalemia (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension
- Cervical Spondylosis

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Spironolactone 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 60
- Mother: Hypertension, stroke at age 72
- Sibling: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker (currently disabled).

VITAL SIGNS:
His blood pressure is 90/60 mmHg, heart rate is 170 beats per minute, respiratory rate is 18 breaths per minute, and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals a regular rhythm with no murmurs, rubs, or gallops, and his lungs are clear to auscultation.

LABORATORY DATA:
His blood glucose is 180 mg/dL, serum potassium is 4.2 mEq/L, and creatinine is 1.1 mg/dL.

OTHER STUDIES:
An electrocardiogram (ECG) shows a wide-complex tachycardia with no discernible P waves.

PLAN:
Intravenous access is established, and the patient is placed on a cardiac monitor.
