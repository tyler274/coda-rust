# synthetic_hospital patient 1978

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic Black, Blue Cross Blue Shield (Private).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-05-24 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2021-11-26 (Fri) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2022-11-21 (Mon) | outpatient | Ophthalmology |
| 3 | 2022-07-15 | 2023-11-27 (Mon) | outpatient | Pulmonology |
| 4 | 2023-07-15 | 2024-11-21 (Thu) | outpatient | Cardiology |
| 5 | 2024-01-15 | 2025-05-23 (Fri) | outpatient | Cardiology |
| 6 | 2024-07-15 | 2025-12-01 (Mon) | outpatient | Cardiology |
| 7 | 2024-10-15 | 2026-02-26 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield (Private)",
 "occupation": "Inventory Control Specialist",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Type 2 diabetes mellitus, myocardial infarction at age 62",
  "Mother: Hypertension, stroke at age 70",
  "Brother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic Black",
 "smoking_status": "Former smoker, 15 pack-years, quit 2019",
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
   "dose": "20 mg PO at bedtime",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus (15-year duration)",
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Heart failure with preserved ejection fraction", "Proliferative diabetic retinopathy", "Heart failure with reduced ejection fraction", "Type 2 diabetes mellitus", "Obstructive sleep apnea", "Essential hypertension", "Acute decompensated heart failure"]
Comorbidities: ["Essential hypertension", "Left ventricular hypertrophy", "Obesity", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-05-24, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine follow-up

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine follow-up

HISTORY OF PRESENT ILLNESS:
He has been managing his diabetes with metformin and a balanced diet. He reports no new symptoms and is adherent to his medication.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 20 mg PO at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 62
- Mother: Hypertension, stroke at age 70
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 2019. Alcohol: Social, 1-2 drinks per week. Occupation: Inventory Control Specialist.

VITAL SIGNS:
His blood pressure is 132/86 mmHg, and his body mass index is 30 kg/m^2.

PHYSICAL EXAMINATION:
His feet are intact with palpable pulses and no signs of ulceration.

LABORATORY DATA:
Recent laboratory results are as follows:

- Hemoglobin A1c: 7.2%
- Serum creatinine: 1.1 mg/dL
- Urinalysis: No glucose, no ketones, no blood
- Random urine albumin-to-creatinine ratio: Pending

## Encounter 1: source 2020-07-15, practice 2021-11-26, outpatient, Internal Medicine

Chief complaint: routine follow-up for hypertension management

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for hypertension management

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old male with Type 2 diabetes mellitus of 15 years' duration returning for hypertension management, last seen six months ago for routine diabetes follow-up. He was diagnosed with hypertension 6 months ago. Despite adherence to lifestyle modifications including dietary changes and increased physical activity, his blood pressure remains elevated. His last visit 2 months ago showed a blood pressure of 158/94 mmHg.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Hyperlipidemia

He has a history of type 2 diabetes mellitus and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His current medication includes metformin 1000 mg twice daily and atorvastatin 20 mg at bedtime.

Home Medications:
- Lisinopril 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 62
- Mother: Hypertension, stroke at age 70
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 2019. Alcohol: Social, 1-2 drinks per week. Occupation: Inventory Control Specialist.

VITAL SIGNS:
Today, his blood pressure is 162/98 mmHg.

PHYSICAL EXAMINATION:
His physical examination is unremarkable except for obesity. His BMI is 32 kg/m^2.

LABORATORY DATA:
Laboratory values are as follows:

- Fasting glucose: 138 mg/dL
- HbA1c: 7.2%
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 110 mg/dL
- HDL cholesterol: 40 mg/dL
- Triglycerides: 250 mg/dL
- Serum creatinine: 1.0 mg/dL
- Estimated GFR: >60 mL/min/1.73m^2

## Encounter 2: source 2021-07-15, practice 2022-11-21, outpatient, Ophthalmology

Chief complaint: gradually worsening vision over the past year

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening vision over the past year

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old male with a 15-year history of type 2 diabetes mellitus who presents for ophthalmologic evaluation following recent internal medicine follow-up for his chronic conditions. He has not had recent eye examinations and reports that his glasses no longer help him see clearly. He occasionally notices dark spots drifting in his field of vision, and at night he struggles to distinguish objects in dim light. There is no headache, jaw pain, or limb weakness, and he has not experienced acute eye pain. He denies trauma, ocular infections,

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-07-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Hyperlipidemia

He was diagnosed with diabetes mellitus 15 years ago and admits that his blood glucose levels have often been poorly controlled despite multiple medications.

PAST SURGICAL HISTORY:
Prior surgery.

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 20 mg PO at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 62
- Mother: Hypertension, stroke at age 70
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 2019. Alcohol: Social, 1-2 drinks per week. Occupation: Inventory Control Specialist.

VITAL SIGNS:
His blood pressure is 148/92 mmHg, pulse is 78/min, and BMI is 31 kg/m².

PHYSICAL EXAMINATION:
Visual fields are constricted bilaterally, and intraocular pressure is normal.

LABORATORY DATA:
Laboratory evaluation reveals a hemoglobin A1c of 9.6%.

## Encounter 3: source 2022-07-15, practice 2023-11-27, outpatient, Pulmonology

Chief complaint: fatigue, daytime sleepiness, and a recent episode of palpitations

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Priya Patel, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, daytime sleepiness, and a recent episode of palpitations

HISTORY OF PRESENT ILLNESS:
Patient with diabetes and proliferative retinopathy presents with fatigue and daytime sleepiness. He reports his wife observed him snoring loudly and having periods of apnea during sleep. He denies chest pain, dizziness, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-07-15)
- Proliferative diabetic retinopathy (diagnosed 2021-07-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 62
- Mother: Hypertension, stroke at age 70
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 2019. Alcohol: Social, 1-2 drinks per week. Occupation: Inventory Control Specialist.

VITAL SIGNS:
On physical examination, his blood pressure is 145/90 mmHg, heart rate is irregularly irregular at 110 bpm, and BMI is 32 kg/m^2.

PHYSICAL EXAMINATION:
Neck circumference is 43 cm. Cardiac auscultation reveals an irregular rhythm without murmurs, and lung fields are clear.

OTHER STUDIES:
An electrocardiogram shows atrial fibrillation with a rapid ventricular response. Overnight polysomnography is scheduled and demonstrates an apnea-hypopnea index of 22 events per hour.

## Encounter 4: source 2023-07-15, practice 2024-11-21, outpatient, Cardiology

Chief complaint: presents to the clinic for a follow-up visit with increasing fatigue and shortness of breath on exertion

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James Wilson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a follow-up visit with increasing fatigue and shortness of breath on exertion

HISTORY OF PRESENT ILLNESS:
The patient with type 2 diabetes, hypertension, and prior OSA returns for follow-up with increasing fatigue and shortness of breath on exertion over the past 6 months. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-07-15)
- Proliferative diabetic retinopathy (diagnosed 2021-07-15)
- Obstructive sleep apnea (diagnosed 2022-07-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 62
- Mother: Hypertension, stroke at age 70
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 2019. Alcohol: Social, 1-2 drinks per week. Occupation: Inventory Control Specialist.

VITAL SIGNS:
On physical examination, his blood pressure is 138/86 mmHg, heart rate is 78 bpm, and BMI is 32 kg/m^2.

PHYSICAL EXAMINATION:
Auscultation of the heart reveals a soft S4 gallop.

LABORATORY DATA:
Laboratory tests show:

- Hemoglobin A1c: 7.2%
- Fasting glucose: 154 mg/dL
- Serum creatinine: 1.1 mg/dL
- BNP: 280 pg/mL

IMAGING:
The patient is referred for an echocardiogram, which shows left ventricular diastolic dysfunction with preserved ejection fraction.

OTHER STUDIES:
ECG: Left ventricular hypertrophy

ASSESSMENT AND PLAN:
The patient is concerned about the implications of his condition and asks about the reliability of the echocardiogram findings.

## Encounter 5: source 2024-01-15, practice 2025-05-23, outpatient, Cardiology

Chief complaint: progressive dyspnea on exertion and fatigue

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Wilson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion and fatigue

HISTORY OF PRESENT ILLNESS:
The patient returns to the Cardiology clinic for follow-up of his heart failure and longstanding type 2 diabetes mellitus with proliferative retinopathy. He presents complaining of progressive dyspnea on exertion and fatigue over the past 6 months. He also reports a recent episode of paroxysmal nocturnal dyspnea. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-07-15)
- Proliferative diabetic retinopathy (diagnosed 2021-07-15)
- Obstructive sleep apnea (diagnosed 2022-07-15)
- Heart failure with preserved ejection fraction (diagnosed 2023-07-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His medications include metformin, lisinopril, and hydrochlorothiazide.

Home Medications:
- Atorvastatin 20 mg PO at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 62
- Mother: Hypertension, stroke at age 70
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 2019. Alcohol: Social, 1-2 drinks per week. Occupation: Inventory Control Specialist.

VITAL SIGNS:
On physical examination, his blood pressure is 130/85 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals an S3 gallop and bibasilar crackles are noted on lung auscultation. Extremities show 1+ bilateral lower extremity edema.

IMAGING:
A subsequent echocardiogram reveals a left ventricular ejection fraction of 30%. Based on these findings,

OTHER STUDIES:
An ECG shows left ventricular hypertrophy.

## Encounter 6: source 2024-07-15, practice 2025-12-01, outpatient, Cardiology

Chief complaint: presents to the clinic for a routine follow-up with worsening fatigue and shortness of breath

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. James Wilson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine follow-up with worsening fatigue and shortness of breath

HISTORY OF PRESENT ILLNESS:
The patient returns for cardiology follow-up with progression of symptoms evaluated in July 2023 and January 2024. He complains of increasing fatigue and shortness of breath on exertion over the past 6 months. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-07-15)
- Proliferative diabetic retinopathy (diagnosed 2021-07-15)
- Obstructive sleep apnea (diagnosed 2022-07-15)
- Heart failure with preserved ejection fraction (diagnosed 2023-07-15)
- Heart failure with reduced ejection fraction (diagnosed 2024-01-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 62
- Mother: Hypertension, stroke at age 70
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 2019. Alcohol: Social, 1-2 drinks per week. Occupation: Inventory Control Specialist.

VITAL SIGNS:
On physical examination, his blood pressure is 142/88 mmHg, heart rate is 78 bpm, and BMI is 32 kg/m^2.

PHYSICAL EXAMINATION:
Lung auscultation reveals bibasilar crackles. Cardiac examination shows an S4 gallop. No lower extremity edema is noted.

LABORATORY DATA:
- Hemoglobin A1c: 7.2%
- Serum creatinine: 1.4 mg/dL
- B-type natriuretic peptide (BNP): 580 pg/mL

IMAGING:
- Chest X-ray: Mild cardiomegaly with no acute disease

PLAN:
The patient is enrolled in a factorial design study to evaluate the effects of two interventions on heart failure outcomes. He is randomized to receive either a beta-blocker or a placebo, and either an ACE inhibitor or an ARB.

## Encounter 7: source 2024-10-15, practice 2026-02-26, ed, Emergency Medicine

Chief complaint: progressive shortness of breath, orthopnea, and lower extremity edema

ENCOUNTER NOTE
Date: 2024-10-15
Provider: Dr. Emily Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath, orthopnea, and lower extremity edema

HISTORY OF PRESENT ILLNESS:
The patient with Type 2 diabetes mellitus and heart failure, decompensated three months prior, presents to the emergency department with progressive shortness of breath, orthopnea, and lower extremity edema over the past two weeks. He reports a 10-pound weight gain during this period.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-07-15)
- Proliferative diabetic retinopathy (diagnosed 2021-07-15)
- Obstructive sleep apnea (diagnosed 2022-07-15)
- Heart failure with preserved ejection fraction (diagnosed 2023-07-15)
- Heart failure with reduced ejection fraction (diagnosed 2024-01-15)
- Acute decompensated heart failure (diagnosed 2024-07-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes metformin, lisinopril, and hydrochlorothiazide.

Home Medications:
- Atorvastatin 20 mg PO at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 62
- Mother: Hypertension, stroke at age 70
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 2019. Alcohol: Social, 1-2 drinks per week. Occupation: Inventory Control Specialist.

VITAL SIGNS:
Blood pressure is 150/90 mmHg, heart rate is 102 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 90% on room air

PHYSICAL EXAMINATION:
Cardiac auscultation reveals an S3 gallop. Lung auscultation demonstrates bibasilar crackles. Extremities show 2+ pitting edema up to the mid-shins.

LABORATORY DATA:
Serum BNP: 900 pg/mL (normal <100 pg/mL)
- Serum creatinine: 1.2 mg/dL (normal 0.6-1.2 mg/dL)
- Serum potassium: 4.5 mEq/L (normal 3.5-5.0 mEq/L)
- Serum sodium: 138 mEq/L (normal 135-145 mEq/L)

IMAGING:
Chest X-ray shows cardiomegaly and pulmonary congestion. An echocardiogram reveals a left ventricular ejection fraction of 30%.
