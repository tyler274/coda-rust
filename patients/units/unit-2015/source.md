# synthetic_hospital patient 2015

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic White, Private commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-10-23 (Fri) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2021-04-30 (Fri) | outpatient | Gastroenterology |
| 2 | 2021-01-15 | 2021-11-01 (Mon) | outpatient | Internal Medicine |
| 3 | 2022-01-15 | 2022-10-27 (Thu) | outpatient | Nephrology |
| 4 | 2023-01-15 | 2023-10-30 (Mon) | outpatient | Internal Medicine |
| 5 | 2023-07-15 | 2024-05-09 (Thu) | outpatient | Internal Medicine |
| 6 | 2024-07-15 | 2025-05-07 (Wed) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2025-10-29 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private commercial insurance",
 "occupation": "Office manager",
 "alcohol_use": "None",
 "family_history": [
  "Father with type 2 diabetes mellitus",
  "Mother with essential hypertension",
  "Paternal grandfather with chronic kidney disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Bilateral laser photocoagulation for diabetic retinopathy"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus (15-year duration)",
  "Hypertension",
  "Chronic kidney disease stage 2",
  "Diabetic retinopathy status post bilateral laser photocoagulation",
  "Obesity",
  "History of gout",
  "Internal hemorrhoids",
  "Constipation"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Diabetic nephropathy", "Gout", "Type 2 diabetes mellitus", "Hemodynamically unstable wide-complex tachycardia", "Secondary hyperaldosteronism", "Internal hemorrhoids", "Isolated oculomotor nerve palsy (microvascular ischemia)"]
Comorbidities: ["Essential hypertension", "Obesity", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-10-23, outpatient, Endocrinology

Chief complaint: Routine follow-up visit for type 2 diabetes mellitus management.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit for type 2 diabetes mellitus management.

HISTORY OF PRESENT ILLNESS:
He has been managing his diabetes with metformin and a balanced diet. He denies any symptoms of polyuria, polydipsia, or unexplained weight loss. His blood pressure is consistently around 128/82 mmHg.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Chronic kidney disease stage 2
- Diabetic retinopathy status post bilateral laser photocoagulation
- Obesity
- History of gout
- Internal hemorrhoids
- Constipation

He does not have a history of hypertension.

PAST SURGICAL HISTORY:
- Bilateral laser photocoagulation for diabetic retinopathy

MEDICATIONS:
His current medications include metformin and atorvastatin.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Lisinopril 20 mg PO daily
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with essential hypertension
- Paternal grandfather with chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office manager.

PHYSICAL EXAMINATION:
On physical examination, his BMI is 30 kg/m^2, and there are no signs of peripheral edema.

LABORATORY DATA:
His most recent laboratory results are as follows:

- Hemoglobin A1c: 6.8%
- Serum creatinine: 1.0 mg/dL
- Urine albumin-to-creatinine ratio (ACR): 20 mg/g

## Encounter 1: source 2020-07-15, practice 2021-04-30, outpatient, Gastroenterology

Chief complaint: Intermittent streaks of bright red blood on toilet paper and sensation of rectal fullness.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent streaks of bright red blood on toilet paper and sensation of rectal fullness.

HISTORY OF PRESENT ILLNESS:
The patient, with history of internal hemorrhoids and constipation, presents to Gastroenterology six months after his routine Endocrinology visit. Over the past 8 months, he has noticed a feeling of fullness in the rectum but no significant pain. He has a long-standing history of constipation and often strains to pass hard stools.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Chronic kidney disease stage 2
- Diabetic retinopathy status post bilateral laser photocoagulation
- Obesity
- History of gout
- Internal hemorrhoids
- Constipation

His medical history includes type 2 diabetes mellitus and hypertension, both controlled with medication.

PAST SURGICAL HISTORY:
- Bilateral laser photocoagulation for diabetic retinopathy

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 24 units subcutaneous at bedtime
- Lisinopril 20 mg PO daily
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
He denies family history of colorectal cancer.

SOCIAL HISTORY:
He does not drink alcohol or smoke.

REVIEW OF SYSTEMS:
He denies weight loss, melena, abdominal pain

PHYSICAL EXAMINATION:
On examination, his abdomen is soft and nontender. Rectal exam does not reveal any tender nodules, but anoscopy shows engorged vascular cushions protruding into the lumen above the level of the dentate line that do not cause pain when touched.

## Encounter 2: source 2021-01-15, practice 2021-11-01, outpatient, Internal Medicine

Chief complaint: Follow-up visit after three gout attacks in the past year, with recent right knee episode treated with corticosteroids.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Jennifer Okafor, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit after three gout attacks in the past year, with recent right knee episode treated with corticosteroids.

HISTORY OF PRESENT ILLNESS:
The patient returns for gout follow-up following recent Endocrinology and Gastroenterology visits. He reports a recent right knee episode treated with oral corticosteroids with complete symptom resolution and currently has no joint pain or swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Internal hemorrhoids (diagnosed 2020-07-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Chronic kidney disease stage 2
- Diabetic retinopathy status post bilateral laser photocoagulation
- Obesity
- History of gout
- Constipation

Past medical history includes obesity, hypertension, and chronic kidney disease stage 2.

PAST SURGICAL HISTORY:
- Bilateral laser photocoagulation for diabetic retinopathy

MEDICATIONS:
He takes hydrochlorothiazide, lisinopril, and amlodipine.

Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 24 units subcutaneous at bedtime
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with essential hypertension
- Paternal grandfather with chronic kidney disease

SOCIAL HISTORY:
He does not consume alcohol or sugary beverages.

LABORATORY DATA:
Laboratory testing reveals a serum uric acid level of 9.8 mg/dL.

## Encounter 3: source 2022-01-15, practice 2022-10-27, outpatient, Nephrology

Chief complaint: Routine follow-up with increasing fatigue and frothy urine over the past few months.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with increasing fatigue and frothy urine over the past few months.

HISTORY OF PRESENT ILLNESS:
This 60-year-old male with 15-year type 2 diabetes mellitus and chronic kidney disease stage 2 returns for nephrology follow-up following prior Endocrinology and Internal Medicine visits. He reports increasing fatigue and frothy urine over the past few months. His diabetes is managed with metformin and insulin, though he admits to occasional non-compliance with his medications and diet. Blood pressure remains difficult to control, averaging 150/95 mmHg despite treatment with lisinopril.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Internal hemorrhoids (diagnosed 2020-07-15)
- Gout (diagnosed 2021-01-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Chronic kidney disease stage 2
- Diabetic retinopathy status post bilateral laser photocoagulation
- Obesity
- Constipation

He also has a history of bilateral laser photocoagulation for diabetic retinopathy.

PAST SURGICAL HISTORY:
- Bilateral laser photocoagulation for diabetic retinopathy

MEDICATIONS:
He takes metformin, insulin, lisinopril.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with essential hypertension
- Paternal grandfather with chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office manager.

PHYSICAL EXAMINATION:
On physical examination, his BMI is 32 kg/m^2, and he has mild pitting edema in both lower extremities. Fundoscopic examination reveals dot hemorrhages and hard exudates.

LABORATORY DATA:
- Serum creatinine: 2.1 mg/dL (normal: 0.6-1.3 mg/dL)
- Urine albumin-to-creatinine ratio: 400 mg/g (normal: 40 mg/dL for men)

## Encounter 4: source 2023-01-15, practice 2023-10-30, outpatient, Internal Medicine

Chief complaint: Follow-up visit while participating in a double-blind clinical trial for a new antihypertensive medication.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit while participating in a double-blind clinical trial for a new antihypertensive medication.

HISTORY OF PRESENT ILLNESS:
The patient with longstanding diabetes mellitus and hypertension, seen in Nephrology, returns while participating in a double-blind, placebo-controlled clinical trial for a new antihypertensive medication. He reports no new symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Internal hemorrhoids (diagnosed 2020-07-15)
- Gout (diagnosed 2021-01-15)
- Diabetic nephropathy (diagnosed 2022-01-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Chronic kidney disease stage 2
- Diabetic retinopathy status post bilateral laser photocoagulation
- Obesity
- Constipation

PAST SURGICAL HISTORY:
- Bilateral laser photocoagulation for diabetic retinopathy

MEDICATIONS:
His blood pressure has been well-controlled on metoprolol and lisinopril, and his diabetes is managed with metformin.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with essential hypertension
- Paternal grandfather with chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office manager.

VITAL SIGNS:
On examination, his blood pressure is 132/84 mmHg, heart rate is 78 bpm, and BMI is 30 kg/m^2.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
Laboratory values are as follows:

- Fasting plasma glucose: 138 mg/dL
- HbA1c: 7.2%
- Serum creatinine: 1.0 mg/dL
- LDL cholesterol: 100 mg/dL
- Serum potassium: 4.5 mEq/L

OTHER STUDIES:
The investigator of the clinical trial informs you that the interim analysis shows no significant difference in the baseline characteristics between the treatment and placebo groups.

## Encounter 5: source 2023-07-15, practice 2024-05-09, outpatient, Internal Medicine

Chief complaint: Persistent headaches and episodes of dizziness for the past two months.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Jennifer Okafor, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent headaches and episodes of dizziness for the past two months.

HISTORY OF PRESENT ILLNESS:
The patient is a 61-year-old male with T2DM, hypertension, CKD, and recent trial participation. He presents with persistent headaches and episodes of dizziness for the past two months.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Internal hemorrhoids (diagnosed 2020-07-15)
- Gout (diagnosed 2021-01-15)
- Diabetic nephropathy (diagnosed 2022-01-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Chronic kidney disease stage 2
- Diabetic retinopathy status post bilateral laser photocoagulation
- Obesity
- Constipation

PAST SURGICAL HISTORY:
- Bilateral laser photocoagulation for diabetic retinopathy

MEDICATIONS:
He mentions that he has been adherent to his antihypertensive regimen, which includes hydrochlorothiazide and amlodipine.

Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 24 units subcutaneous at bedtime
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with essential hypertension
- Paternal grandfather with chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office manager.

VITAL SIGNS:
His blood pressure today is 162/98 mmHg.

PHYSICAL EXAMINATION:
His physical examination is unremarkable except for bilateral pedal edema.

LABORATORY DATA:
Laboratory results are as follows:

- Serum creatinine: 2.1 mg/dL (elevated)
- Serum potassium: 4.9 mEq/L
- Plasma renin activity: Elevated
- Serum aldosterone: Elevated

## Encounter 6: source 2024-07-15, practice 2025-05-07, ed, Emergency Medicine

Chief complaint: Sudden onset of double vision and drooping of the left eyelid that began this morning.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of double vision and drooping of the left eyelid that began this morning.

HISTORY OF PRESENT ILLNESS:
This 62-year-old male with type 2 diabetes mellitus complicated by diabetic retinopathy and chronic kidney disease presents to the emergency department with sudden onset of double vision and drooping of his left eyelid that began this morning. He denies any headache, eye pain, or recent trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Internal hemorrhoids (diagnosed 2020-07-15)
- Gout (diagnosed 2021-01-15)
- Diabetic nephropathy (diagnosed 2022-01-15)
- Secondary hyperaldosteronism (diagnosed 2023-07-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Chronic kidney disease stage 2
- Diabetic retinopathy status post bilateral laser photocoagulation
- Obesity
- Constipation

PAST SURGICAL HISTORY:
- Bilateral laser photocoagulation for diabetic retinopathy

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 24 units subcutaneous at bedtime
- Lisinopril 20 mg PO daily
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with essential hypertension
- Paternal grandfather with chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office manager.

VITAL SIGNS:
His blood pressure is 150/95 mmHg, and his heart rate is 88 beats per minute.

PHYSICAL EXAMINATION:
Physical examination reveals ptosis of the left eyelid, and the left eye is deviated downward and laterally. The pupil of the left eye is 3 mm in diameter and reacts normally to light and accommodation. The right eye examination is unremarkable.

LABORATORY DATA:
His fasting blood glucose is 280 mg/dL, and his HbA1c is 9.2%.

## Encounter 7: source 2025-01-15, practice 2025-10-29, ed, Emergency Medicine

Chief complaint: Palpitations and lightheadedness that started while mowing the lawn 30 minutes ago.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Maria Santos, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Palpitations and lightheadedness that started while mowing the lawn 30 minutes ago.

HISTORY OF PRESENT ILLNESS:
This diabetic man presents with palpitations and lightheadedness that started while he was mowing the lawn 30 minutes ago. He denies chest pain, shortness of breath, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Internal hemorrhoids (diagnosed 2020-07-15)
- Gout (diagnosed 2021-01-15)
- Diabetic nephropathy (diagnosed 2022-01-15)
- Secondary hyperaldosteronism (diagnosed 2023-07-15)
- Isolated oculomotor nerve palsy (microvascular ischemia) (diagnosed 2024-07-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Chronic kidney disease stage 2
- Diabetic retinopathy status post bilateral laser photocoagulation
- Obesity
- Constipation

PAST SURGICAL HISTORY:
- Bilateral laser photocoagulation for diabetic retinopathy

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with essential hypertension
- Paternal grandfather with chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office manager.

VITAL SIGNS:
On examination, his blood pressure is 90/60 mmHg, heart rate is 180 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
He appears anxious and diaphoretic. His jugular venous pressure is not elevated, and there are no additional heart sounds or murmurs. Lung fields are clear to auscultation.

LABORATORY DATA:
IV access is established, and labs are drawn. The patient's lab values are as follows:

- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.1 mg/dL
- Glucose: 182 mg/dL
- Troponin: Not detectable

OTHER STUDIES:
The ECG shows a regular wide-complex tachycardia without P waves.
