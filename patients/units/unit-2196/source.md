# synthetic_hospital patient 2196

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare Part A and B with supplemental Blue Cross Blue Shield.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-07-14 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2022-01-26 (Wed) | outpatient | Urology |
| 2 | 2021-07-15 | 2023-01-16 (Mon) | outpatient | Hematology & Oncology |
| 3 | 2022-03-15 | 2023-09-27 (Wed) | outpatient | Hematology & Oncology |
| 4 | 2022-09-15 | 2024-03-20 (Wed) | ed | Emergency Medicine |
| 5 | 2023-03-15 | 2024-09-11 (Wed) | inpatient | Cardiology |
| 6 | 2023-09-15 | 2025-03-29 (Sat) | icu | Internal Medicine |
| 7 | 2024-05-15 | 2025-11-24 (Mon) | outpatient | Hematology & Oncology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin (rash)",
  "Iodinated contrast (hives)"
 ],
 "insurance": "Medicare Part A and B with supplemental Blue Cross Blue Shield",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Social, 1-2 glasses of wine per week",
 "family_history": [
  "Father: Myocardial infarction at age 72, hypertension",
  "Mother: Type 2 diabetes mellitus, osteoporosis",
  "Paternal uncle: Multiple myeloma (diagnosed age 70)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 10 years ago (20 pack-year history)",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Furosemide"
  },
  {
   "dose": "12.5 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Cardiac catheterization with drug-eluting stent placement (5 years ago)",
  "Left inguinal hernia repair (age 45)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Diabetic Nephropathy",
  "Chronic Kidney Disease Stage 3",
  "Coronary Artery Disease status post Myocardial Infarction (5 years prior)",
  "Heart Failure with Reduced Ejection Fraction",
  "Benign Prostatic Hyperplasia"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Multiple myeloma or metastatic bone disease requiring intravenous bisphosphonates", "Essential hypertension", "Hyperkalemia", "Recurrent benign prostatic hyperplasia", "Ventricular Tachycardia", "Heart failure with reduced ejection fraction", "Multiple myeloma"]
Comorbidities: ["Stage 1 chronic kidney disease", "Anemia", "Essential hypertension", "Type 2 diabetes mellitus", "Malignancy-associated hypercalcemia"]

## Encounter 0: source 2020-01-15, practice 2021-07-14, outpatient, Internal Medicine

Chief complaint: Routine follow-up visit two weeks after initiation of antihypertensive therapy.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit two weeks after initiation of antihypertensive therapy.

HISTORY OF PRESENT ILLNESS:
He is started on lisinopril during a routine primary care visit. He reports no side effects and notes that his blood pressure at home has improved.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Diabetic Nephropathy
- Chronic Kidney Disease Stage 3
- Coronary Artery Disease status post Myocardial Infarction (5 years prior)
- Heart Failure with Reduced Ejection Fraction
- Benign Prostatic Hyperplasia

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (5 years ago)
- Left inguinal hernia repair (age 45)

MEDICATIONS:
His current medications include lisinopril, metformin, and atorvastatin.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Furosemide 40 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Penicillin (rash), Iodinated contrast (hives)

FAMILY HISTORY:
- Father: Myocardial infarction at age 72, hypertension
- Mother: Type 2 diabetes mellitus, osteoporosis
- Paternal uncle: Multiple myeloma (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His blood pressure is 126/76 mm Hg and pulse is 68/min.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Serum creatinine is mildly increased from baseline (1.1 mg/dL to 1.3 mg/dL), and potassium is 4.8 mEq/L.

## Encounter 1: source 2020-07-15, practice 2022-01-26, outpatient, Urology

Chief complaint: Six-month history of nocturia, difficulty initiating urination, and weak urinary stream.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Okafor, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of nocturia, difficulty initiating urination, and weak urinary stream.

HISTORY OF PRESENT ILLNESS:
Six months following his internal medicine visit for hypertension management, this 68-year-old male with known benign prostatic hyperplasia presents with a 6-month history of nocturia, difficulty initiating urination, and a weak urinary stream. He denies any fever, dysuria, or hematuria.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Diabetic Nephropathy
- Chronic Kidney Disease Stage 3
- Coronary Artery Disease status post Myocardial Infarction (5 years prior)
- Heart Failure with Reduced Ejection Fraction
- Benign Prostatic Hyperplasia

His past medical history is significant for hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (5 years ago)
- Left inguinal hernia repair (age 45)

MEDICATIONS:
He takes hydrochlorothiazide, metformin.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Penicillin (rash), Iodinated contrast (hives)

FAMILY HISTORY:
- Father: Myocardial infarction at age 72, hypertension
- Mother: Type 2 diabetes mellitus, osteoporosis
- Paternal uncle: Multiple myeloma (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Blood pressure is 135/85 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
He has a non-tender, distended bladder. Digital rectal examination reveals a symmetrically enlarged, firm, and non-tender prostate.

LABORATORY DATA:
- Serum creatinine: 2.1 mg/dL (increased from 1.2 mg/dL one year ago)
- Blood urea nitrogen (BUN): 36 mg/dL
- Serum potassium: 4.5 mEq/L
- Urinalysis: No hematuria, pyuria, or proteinuria

IMAGING:
A renal ultrasound is performed, which shows bilateral hydronephrosis without evidence of stones.

## Encounter 2: source 2021-07-15, practice 2023-01-16, outpatient, Hematology & Oncology

Chief complaint: Three-month history of worsening fatigue and persistent back pain worse at night, with difficulty walking long distances.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of worsening fatigue and persistent back pain worse at night, with difficulty walking long distances.

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with HTN, T2DM, CKD-3, CAD, HFrEF, and BPH, seen in Internal Medicine and Urology, who reports three months of worsening fatigue and back pain, difficulty walking long distances, lower back pain worse at night and unrelieved by rest, and denies recent trauma, fever, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Diabetic Nephropathy
- Chronic Kidney Disease Stage 3
- Coronary Artery Disease status post Myocardial Infarction (5 years prior)
- Heart Failure with Reduced Ejection Fraction
- Benign Prostatic Hyperplasia

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (5 years ago)
- Left inguinal hernia repair (age 45)

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Penicillin (rash), Iodinated contrast (hives)

FAMILY HISTORY:
- Father: Myocardial infarction at age 72, hypertension
- Mother: Type 2 diabetes mellitus, osteoporosis
- Paternal uncle: Multiple myeloma (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
Pallor and diffuse bony tenderness over the lumbar spine. Neurologic examination is normal.

LABORATORY DATA:
Hemoglobin                                    9.8 g/dL
Hematocrit                                      30%
MCV                                               88 fL
Serum calcium                               11.2 mg/dL
Serum creatinine                            2.0 mg/dL
Serum total protein                         9.2 g/dL

## Encounter 3: source 2022-03-15, practice 2023-09-27, outpatient, Hematology & Oncology

Chief complaint: Three-month history of fatigue, bone pain, and recurrent bacterial infections including sinusitis and bronchitis.

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Priya Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of fatigue, bone pain, and recurrent bacterial infections including sinusitis and bronchitis.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of multiple myeloma. He reports having had multiple episodes of sinusitis and bronchitis, which were treated with antibiotics.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-07-15)
- Multiple myeloma (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Diabetic Nephropathy
- Chronic Kidney Disease Stage 3
- Coronary Artery Disease status post Myocardial Infarction (5 years prior)
- Heart Failure with Reduced Ejection Fraction
- Benign Prostatic Hyperplasia

He has a past medical history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (5 years ago)
- Left inguinal hernia repair (age 45)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Furosemide 40 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Penicillin (rash), Iodinated contrast (hives)

FAMILY HISTORY:
- Father: Myocardial infarction at age 72, hypertension
- Mother: Type 2 diabetes mellitus, osteoporosis
- Paternal uncle: Multiple myeloma (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, he is found to have pallor and tenderness over his lumbar vertebrae.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 9.8 g/dL (Normal: 13.8-17.2 g/dL)
- Calcium: 11.2 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum creatinine: 1.9 mg/dL (Normal: 0.6-1.3 mg/dL)
- Total protein: 8.9 g/dL (Normal: 6.4-8.3 g/dL)
- Albumin: 3.2 g/dL (Normal: 3.5-5.0 g/dL)
- Serum protein electrophoresis: M-spike present
- Urine protein electrophoresis: Bence Jones proteins present

## Encounter 4: source 2022-09-15, practice 2024-03-20, ed, Emergency Medicine

Chief complaint: Two-month history of progressive fatigue, shortness of breath with mild exertion, and bilateral lower extremity swelling.

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-month history of progressive fatigue, shortness of breath with mild exertion, and bilateral lower extremity swelling.

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with chronic kidney disease, heart failure, and multiple myeloma presents with a two-month history of progressive fatigue, shortness of breath with mild exertion, and swelling in his lower legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-07-15)
- Multiple myeloma (diagnosed 2021-07-15)
- Multiple myeloma or metastatic bone disease requiring intravenous bisphosphonates (diagnosed 2022-03-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Diabetic Nephropathy
- Chronic Kidney Disease Stage 3
- Coronary Artery Disease status post Myocardial Infarction (5 years prior)
- Heart Failure with Reduced Ejection Fraction
- Benign Prostatic Hyperplasia

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (5 years ago)
- Left inguinal hernia repair (age 45)

MEDICATIONS:
He takes lisinopril, metoprolol, furosemide, and a recently added agent for symptom control.

Home Medications:
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Penicillin (rash), Iodinated contrast (hives)

FAMILY HISTORY:
- Father: Myocardial infarction at age 72, hypertension
- Mother: Type 2 diabetes mellitus, osteoporosis
- Paternal uncle: Multiple myeloma (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
BP is 102/60 mmHg, HR 54

PHYSICAL EXAMINATION:
Bibasilar crackles with 1+ pitting edema in both ankles

LABORATORY DATA:
Serum potassium of 5.8 mEq/L and creatinine of 2.1 mg/dL

OTHER STUDIES:
ECG shows peaked T waves and prolonged PR interval

## Encounter 5: source 2023-03-15, practice 2024-09-11, inpatient, Cardiology

Chief complaint: Acute onset of shortness of breath, orthopnea, and bilateral lower extremity edema with 5 kg weight gain over one week.

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Emily Washington, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Acute onset of shortness of breath, orthopnea, and bilateral lower extremity edema with 5 kg weight gain over one week.

HISTORY OF PRESENT ILLNESS:
Patient with heart failure returns with dyspnea that has worsened over the past week. He reports a weight gain of 5 kg during this period.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-07-15)
- Multiple myeloma (diagnosed 2021-07-15)
- Multiple myeloma or metastatic bone disease requiring intravenous bisphosphonates (diagnosed 2022-03-15)
- Hyperkalemia (diagnosed 2022-09-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Diabetic Nephropathy
- Chronic Kidney Disease Stage 3
- Coronary Artery Disease status post Myocardial Infarction (5 years prior)
- Heart Failure with Reduced Ejection Fraction
- Benign Prostatic Hyperplasia

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (5 years ago)
- Left inguinal hernia repair (age 45)

MEDICATIONS:
His medications include metformin, lisinopril, and hydrochlorothiazide.

Home Medications:
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Penicillin (rash), Iodinated contrast (hives)

FAMILY HISTORY:
- Father: Myocardial infarction at age 72, hypertension
- Mother: Type 2 diabetes mellitus, osteoporosis
- Paternal uncle: Multiple myeloma (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On physical examination, his blood pressure is 160/90 mmHg, heart rate is 110 bpm, respiratory rate is 22 breaths/min, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated. Auscultation of the lungs reveals bilateral rales halfway up both lung fields. Cardiac examination shows a laterally displaced apical impulse and a holosystolic murmur at the apex that radiates to the axilla. Extremities are notable for 2+ pitting edema up to the mid-shins.

LABORATORY DATA:
Laboratory Values:
- Serum creatinine: 1.8 mg/dL
- B-type natriuretic peptide (BNP): 900 pg/mL
- Serum potassium: 4.5 mEq/L
- Serum sodium: 135 mEq/L

IMAGING:
Chest X-ray shows cardiomegaly and pulmonary congestion. An echocardiogram reveals left ventricular hypertrophy, dilated left ventricle with an ejection fraction of 30%, and mitral valve leaflets with normal structure.

## Encounter 6: source 2023-09-15, practice 2025-03-29, icu, Internal Medicine

Chief complaint: Diabetic foot ulcer with fever, tachycardia, and ventricular tachycardia on telemetry.

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. Aisha Johnson, MD
Department: Internal Medicine
Visit Type: icu

CHIEF COMPLAINT:
Diabetic foot ulcer with fever, tachycardia, and ventricular tachycardia on telemetry.

HISTORY OF PRESENT ILLNESS:
The patient is a 71-year-old male with diabetes, heart failure, and multiple myeloma, hospitalized six months prior for heart failure, who is admitted for management of a diabetic foot ulcer. On hospital day 2, the patient develops a fever of 38.5°C, tachycardia with a heart rate of 110 bpm, and a blood pressure of 135/85 mmHg. Over the next 24 hours, the telemetry monitor alarms frequently for ventricular tachycardia.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-07-15)
- Multiple myeloma (diagnosed 2021-07-15)
- Multiple myeloma or metastatic bone disease requiring intravenous bisphosphonates (diagnosed 2022-03-15)
- Hyperkalemia (diagnosed 2022-09-15)
- Heart failure with reduced ejection fraction (diagnosed 2023-03-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Diabetic Nephropathy
- Chronic Kidney Disease Stage 3
- Coronary Artery Disease status post Myocardial Infarction (5 years prior)
- Benign Prostatic Hyperplasia

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (5 years ago)
- Left inguinal hernia repair (age 45)

MEDICATIONS:
He takes metformin, lisinopril, and atorvastatin.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Furosemide 40 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Penicillin (rash), Iodinated contrast (hives)

FAMILY HISTORY:
- Father: Myocardial infarction at age 72, hypertension
- Mother: Type 2 diabetes mellitus, osteoporosis
- Paternal uncle: Multiple myeloma (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Fever of 38.5°C, tachycardia with a heart rate of 110 bpm, and a blood pressure of 135/85 mmHg

PHYSICAL EXAMINATION:
New 2/6 systolic ejection murmur best heard at the right upper sternal border

LABORATORY DATA:
White blood cell count: 12,000 /µL
- Hemoglobin: 11.2 g/dL
- Blood urea nitrogen: 31 mg/dL
- Serum creatinine: 2.1 mg/dL (baseline 1.8 mg/dL)
- C-reactive protein: 18 mg/L
- Blood cultures: Pending

OTHER STUDIES:
Brief episodes of non-sustained ventricular tachycardia (NSVT), which the patient is asymptomatic for

PLAN:
A telemetry monitor is attached, and the patient is placed on broad-spectrum antibiotics. The primary team is considering adjusting the telemetry alarm settings to reduce the frequency of alarms.

## Encounter 7: source 2024-05-15, practice 2025-11-24, outpatient, Hematology & Oncology

Chief complaint: Three-month history of fatigue, back pain, and unintentional weight loss requiring chronic intravenous bisphosphonate therapy.

ENCOUNTER NOTE
Date: 2024-05-15
Provider: Dr. Priya Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of fatigue, back pain, and unintentional weight loss requiring chronic intravenous bisphosphonate therapy.

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with a history of multiple myeloma undergoing chronic intravenous bisphosphonate therapy who returns to the Hematology and Oncology clinic for continued management. He presents with a three-month history of fatigue, back pain, and unintentional weight loss. He reports a dull, persistent ache in his lower back that does not improve with over-the-counter analgesics. He denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-07-15)
- Multiple myeloma (diagnosed 2021-07-15)
- Multiple myeloma or metastatic bone disease requiring intravenous bisphosphonates (diagnosed 2022-03-15)
- Hyperkalemia (diagnosed 2022-09-15)
- Heart failure with reduced ejection fraction (diagnosed 2023-03-15)
- Ventricular Tachycardia (diagnosed 2023-09-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Diabetic Nephropathy
- Chronic Kidney Disease Stage 3
- Coronary Artery Disease status post Myocardial Infarction (5 years prior)
- Benign Prostatic Hyperplasia

His past medical history includes hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (5 years ago)
- Left inguinal hernia repair (age 45)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Furosemide 40 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Penicillin (rash), Iodinated contrast (hives)

FAMILY HISTORY:
- Father: Myocardial infarction at age 72, hypertension
- Mother: Type 2 diabetes mellitus, osteoporosis
- Paternal uncle: Multiple myeloma (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, he has pallor and tenderness over the lumbar vertebrae.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 9.8 g/dL (Normal: 13.8-17.2 g/dL)
- Calcium: 11.2 mg/dL (Normal: 8.5-10.2 mg/dL)
- Creatinine: 1.9 mg/dL (Normal: 0.6-1.3 mg/dL)
- Serum protein electrophoresis: M-spike present
