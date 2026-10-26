# synthetic_hospital patient 2230

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare Part B.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-05-25 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2023-08-28 (Mon) | outpatient | Internal Medicine |
| 2 | 2020-07-15 | 2023-11-20 (Mon) | outpatient | Hematology & Oncology |
| 3 | 2021-01-15 | 2024-05-23 (Thu) | ed | Emergency Medicine |
| 4 | 2022-07-15 | 2025-11-18 (Tue) | outpatient | Hematology & Oncology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare Part B",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Social, 2-3 glasses of wine per week",
 "family_history": [
  "Father: Hypertension, Type 2 Diabetes Mellitus, died of myocardial infarction at age 76",
  "Mother: Osteoporosis, alive at age 91",
  "Paternal uncle: Multiple myeloma diagnosed at age 71 (currently in remission)",
  "Sibling: Older brother with hypertension and hyperlipidemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 12 years ago, 25 pack-year history",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "40 mg PO daily at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "0.4 mg PO daily at bedtime",
   "name": "Tamsulosin"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (age 54)",
  "Bilateral cataract surgery (age 66)",
  "Transurethral resection of the prostate (age 62)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Benign Prostatic Hyperplasia",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Multiple myeloma or metastatic bone disease requiring intravenous bisphosphonates"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2023-05-25, outpatient, Internal Medicine

Chief complaint: Two-month history of fatigue, lower back pain, and unintentional weight loss

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of fatigue, lower back pain, and unintentional weight loss

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 2-month history of

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Benign Prostatic Hyperplasia
- Hyperlipidemia

His past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 54)
- Bilateral cataract surgery (age 66)
- Transurethral resection of the prostate (age 62)

MEDICATIONS:
He takes controlled with hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily at bedtime
- Tamsulosin 0.4 mg PO daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Type 2 Diabetes Mellitus, died of myocardial infarction at age 76
- Mother: Osteoporosis, alive at age 91
- Paternal uncle: Multiple myeloma diagnosed at age 71 (currently in remission)
- Sibling: Older brother with hypertension and hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 25 pack-year history. Alcohol: Social, 2-3 glasses of wine per week. Occupation: Retired high school mathematics teacher.

REVIEW OF SYSTEMS:
He denies fever, chills, or night sweats.

PHYSICAL EXAMINATION:
On physical examination, he appears pale and has tenderness over the lumbar vertebrae.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 9.2 g/dL (Normal: 13.8-17.2 g/dL)
- Calcium: 11.5 mg/dL (Normal: 8.5-10.2 mg/dL)
- Creatinine: 2.1 mg/dL (Normal: 0.6-1.3 mg/dL)
- Serum protein electrophoresis: M-spike present
- Serum albumin: 3.0 g/dL (Normal: 3.5-5.0 g/dL)
- Total protein: 8.5 g/dL (Normal: 6.4-8.3 g/dL)

IMAGING:
A skeletal survey shows several lytic lesions in the lumbar vertebrae and pelvis.

## Encounter 1: source 2020-04-15, practice 2023-08-28, outpatient, Internal Medicine

Chief complaint: Three-month history of worsening lower back pain and 10-pound weight loss

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of worsening lower back pain and 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
The 68-year-old male with hypertension, T2DM, BPH, and hyperlipidemia returns for follow-up of lower back pain and weight loss. He describes the pain as a constant, dull ache that intensifies with movement and is not relieved by over-the-counter analgesics. He also reports a 10-pound weight loss over the last two months, which he attributes to a lack of appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple myeloma or metastatic bone disease requiring intravenous bisphosphonates (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Benign Prostatic Hyperplasia
- Hyperlipidemia

His past medical history is significant for hypertension, controlled with hydrochlorothiazide, and benign prostatic hyperplasia.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 54)
- Bilateral cataract surgery (age 66)
- Transurethral resection of the prostate (age 62)

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily at bedtime
- Tamsulosin 0.4 mg PO daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Type 2 Diabetes Mellitus, died of myocardial infarction at age 76
- Mother: Osteoporosis, alive at age 91
- Paternal uncle: Multiple myeloma diagnosed at age 71 (currently in remission)
- Sibling: Older brother with hypertension and hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 25 pack-year history. Alcohol: Social, 2-3 glasses of wine per week. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, he is noted to have mild pallor and tenderness over the lumbar vertebrae.

LABORATORY DATA:
Laboratory tests reveal a normocytic anemia and elevated total protein with a normal albumin level.

IMAGING:
An x-ray of the lumbar spine shows several punched-out osteolytic lesions.

## Encounter 2: source 2020-07-15, practice 2023-11-20, outpatient, Hematology & Oncology

Chief complaint: Fatigue, diffuse bone pain involving ribs and back, and unintentional weight loss

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Amara Okafor, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, diffuse bone pain involving ribs and back, and unintentional weight loss

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old with hypertension, T2DM, BPH, and hyperlipidemia who returns following visits in January and April 2020, presenting with a 3-month history of fatigue, bone pain, and unintentional weight loss, with persistent dull pain in lower back and ribs that worsens with movement.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple myeloma or metastatic bone disease requiring intravenous bisphosphonates (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Benign Prostatic Hyperplasia
- Hyperlipidemia

His past medical history includes hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 54)
- Bilateral cataract surgery (age 66)
- Transurethral resection of the prostate (age 62)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily at bedtime
- Tamsulosin 0.4 mg PO daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Type 2 Diabetes Mellitus, died of myocardial infarction at age 76
- Mother: Osteoporosis, alive at age 91
- Paternal uncle: Multiple myeloma diagnosed at age 71 (currently in remission)
- Sibling: Older brother with hypertension and hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 25 pack-year history. Alcohol: Social, 2-3 glasses of wine per week. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, he appears pale and has tenderness over his lumbar vertebrae.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 9.8 g/dL (Normal: 13.8-17.2 g/dL)
- Calcium: 11.2 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum creatinine: 2.1 mg/dL (Normal: 0.6-1.3 mg/dL)
- Serum albumin: 3.2 g/dL (Normal: 3.5-5.0 g/dL)
- Total protein: 8.9 g/dL (Normal: 6.4-8.3 g/dL)
- Urinalysis: Bence Jones protein positive

IMAGING:
A skeletal survey reveals multiple lytic lesions throughout the axial skeleton.

ASSESSMENT AND PLAN:
Based on the clinical presentation and laboratory findings,

## Encounter 3: source 2021-01-15, practice 2024-05-23, ed, Emergency Medicine

Chief complaint: Increasing fatigue, constipation, low mood, and severe back pain

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Patterson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing fatigue, constipation, low mood, and severe back pain

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department with ongoing symptoms, having been evaluated in Internal Medicine and Hematology & Oncology over the preceding year. He presents with a 2-month history of increasing fatigue, constipation, and low mood. He also reports back pain that has progressively worsened over the same period.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple myeloma or metastatic bone disease requiring intravenous bisphosphonates (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Benign Prostatic Hyperplasia
- Hyperlipidemia

His past medical history includes hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 54)
- Bilateral cataract surgery (age 66)
- Transurethral resection of the prostate (age 62)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily at bedtime
- Tamsulosin 0.4 mg PO daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Type 2 Diabetes Mellitus, died of myocardial infarction at age 76
- Mother: Osteoporosis, alive at age 91
- Paternal uncle: Multiple myeloma diagnosed at age 71 (currently in remission)
- Sibling: Older brother with hypertension and hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 25 pack-year history. Alcohol: Social, 2-3 glasses of wine per week. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, he appears lethargic. His abdomen is non-distended, non-tender, and bowel sounds are normal. Digital rectal examination reveals hard stool in the rectal vault. Neurological examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 12.1 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum albumin: 4.0 g/dL (normal: 3.5-5.0 g/dL)
- Serum creatinine: 1.2 mg/dL (normal: 0.6-1.3 mg/dL)
- Hemoglobin: 9.8 g/dL (normal: 13.8-17.2 g/dL)
- Serum protein electrophoresis: M-spike present

ASSESSMENT AND PLAN:
Given the patient's presentation and laboratory findings,

## Encounter 4: source 2022-07-15, practice 2025-11-18, outpatient, Hematology & Oncology

Chief complaint: One-month history of persistent mid-back pain with ankle swelling and frothy urine

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Amara Okafor, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
One-month history of persistent mid-back pain with ankle swelling and frothy urine

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with hypertension, type 2 diabetes mellitus, benign prostatic hyperplasia, and hyperlipidemia who returns for evaluation, having presented previously in January 2020, April 2020, July 2020, and January 2021 for similar complaints of fatigue, bone pain, and unintentional weight loss. He states the pain is worse at night and does not improve with rest. He has noted swelling in his ankles over the past few weeks and reports his urine appears "frothy." He denies any recent falls, trauma, or heavy lifting. His back pain persists despite NSAIDs, and he has required frequent clinic visits for worsening fatigue. On further discussion, he reports a 5-kg unintentional weight loss over the past 2 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple myeloma or metastatic bone disease requiring intravenous bisphosphonates (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Benign Prostatic Hyperplasia
- Hyperlipidemia

His medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 54)
- Bilateral cataract surgery (age 66)
- Transurethral resection of the prostate (age 62)

MEDICATIONS:
He takes he takes lisinopril and atorvastatin.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Tamsulosin 0.4 mg PO daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Type 2 Diabetes Mellitus, died of myocardial infarction at age 76
- Mother: Osteoporosis, alive at age 91
- Paternal uncle: Multiple myeloma diagnosed at age 71 (currently in remission)
- Sibling: Older brother with hypertension and hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 25 pack-year history. Alcohol: Social, 2-3 glasses of wine per week. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral pitting edema of the lower extremities and an S4 heart sound on cardiac auscultation.

LABORATORY DATA:
Urinalysis reveals 4+ protein with no red blood cells or casts.
Further evaluation with serum protein electrophoresis shows an M-protein spike in the gamma region. Serum free light chain assay demonstrates a marked increase in kappa light chains.
Labs are drawn and confirm the presence of a monoclonal gammopathy, and urine protein electrophoresis reveals Bence-Jones proteins.

IMAGING:
Lumbar spine X-rays reveal diffuse osteopenia with multiple punched-out lesions in the thoracic vertebrae.
echocardiography reveals a thickened left ventricle with preserved ejection fraction

PATHOLOGY:
A renal biopsy is performed, which shows Congo red–positive deposits that display apple-green birefringence under polarized light.

OTHER STUDIES:
His ECG is normal

PLAN:
He is scheduled for bone marrow biopsy to confirm the underlying plasma cell disorder.
