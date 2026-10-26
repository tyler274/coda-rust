# synthetic_hospital patient 2026

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-07-12 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-05-15 | 2021-11-10 (Wed) | outpatient | Internal Medicine |
| 2 | 2020-09-15 | 2022-03-15 (Tue) | outpatient | Internal Medicine |
| 3 | 2021-03-15 | 2022-09-08 (Thu) | outpatient | Cardiology |
| 4 | 2021-04-15 | 2022-10-24 (Mon) | outpatient | Cardiology |
| 5 | 2022-01-15 | 2023-07-24 (Mon) | outpatient | Internal Medicine |
| 6 | 2023-01-15 | 2024-07-25 (Thu) | outpatient | Internal Medicine |
| 7 | 2025-01-15 | 2026-07-13 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Office Manager",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "Father: Myocardial infarction at age 62, hypertension",
  "Mother: Type 2 diabetes mellitus, hypertension",
  "Brother: Gout"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 10 cigarettes per day",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "10 mg PO BID",
   "name": "Enalapril"
  },
  {
   "dose": "300 mg PO daily",
   "name": "Allopurinol"
  },
  {
   "dose": "0.6 mg PO daily",
   "name": "Colchicine"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Emergency sigmoid colectomy with end colostomy for perforated diverticulitis with peritonitis"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Heart Failure with Reduced Ejection Fraction",
  "Gout",
  "Diverticulosis",
  "Mixed Hyperlipidemia"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Essential hypertension", "Mixed hyperlipidemia", "Simple renal cyst", "Perforated diverticulitis with peritonitis", "Angiotensin-converting enzyme inhibitor therapy (Enalapril)", "Heart failure with reduced ejection fraction", "Gout"]
Comorbidities: ["Essential hypertension", "Cardiovascular disease risk factors"]

## Encounter 0: source 2020-01-15, practice 2021-07-12, outpatient, Internal Medicine

Chief complaint: Routine follow-up for hypertension management with lifestyle modifications

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for hypertension management with lifestyle modifications

HISTORY OF PRESENT ILLNESS:
Currently managed with lifestyle modifications

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Heart Failure with Reduced Ejection Fraction
- Gout
- Diverticulosis
- Mixed Hyperlipidemia

PAST SURGICAL HISTORY:
- Emergency sigmoid colectomy with end colostomy for perforated diverticulitis with peritonitis

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Enalapril 10 mg PO BID
- Allopurinol 300 mg PO daily
- Colchicine 0.6 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, hypertension
- Brother: Gout

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day. Alcohol: Social, 2-3 drinks per week. Occupation: Office Manager.

VITAL SIGNS:
His blood pressure today is 160/100 mmHg.

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
His laboratory values are as follows:

- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.0 mg/dL
- Fasting blood glucose: 98 mg/dL
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 120 mg/dL
- HDL cholesterol: 40 mg/dL

OTHER STUDIES:
A randomized controlled trial is conducted to evaluate the effectiveness of a new antihypertensive drug, Zetapress, compared to a placebo in reducing the incidence of stroke in patients with stage 2 hypertension. The study includes 500 patients, with 250 receiving Zetapress and 250 receiving a placebo. After 5 years, the incidence of stroke in the Zetapress group is 10%, while in the placebo group it is 20%. The mean systolic blood pressure reduction in the Zetapress group is 25 mmHg, compared to 10 mmHg in the placebo group.

ASSESSMENT AND PLAN:
Based on the study results and the patient's clinical profile,

## Encounter 1: source 2020-05-15, practice 2021-11-10, outpatient, Internal Medicine

Chief complaint: Routine health check-up with elevated blood pressure noted at recent health fair

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health check-up with elevated blood pressure noted at recent health fair

HISTORY OF PRESENT ILLNESS:
The patient with HFrEF and diabetes returns for blood pressure follow-up after January 2020 lifestyle modifications. He reports no complaints but notes blood pressure was elevated on two separate occasions at a recent health fair.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Heart Failure with Reduced Ejection Fraction
- Gout
- Diverticulosis
- Mixed Hyperlipidemia

His past medical history is significant for type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Emergency sigmoid colectomy with end colostomy for perforated diverticulitis with peritonitis

MEDICATIONS:
He takes metformin.

Home Medications:
- Enalapril 10 mg PO BID
- Allopurinol 300 mg PO daily
- Colchicine 0.6 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
He has a family history of coronary artery disease

SOCIAL HISTORY:
He smokes half a pack of cigarettes per day

VITAL SIGNS:
His BMI is 32 kg/m^2

PHYSICAL EXAMINATION:
On physical examination, his blood pressure is 152/96 mmHg, heart rate is 78 bpm, and the rest of the exam is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Serum chemistry panel:
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.1 mg/dL
- Glomerular filtration rate: >60 mL/min/1.73m^2
- Hemoglobin/Hematocrit: 15 g/dL/45%
- Urinalysis: Normal
- Lipid panel:
- Total cholesterol: 220 mg/dL
- LDL cholesterol: 145 mg/dL
- HDL cholesterol: 35 mg/dL
- Triglycerides: 250 mg/dL
- Fasting serum glucose: 160 mg/dL
- Hemoglobin A1c: 7.8%

## Encounter 2: source 2020-09-15, practice 2022-03-15, outpatient, Internal Medicine

Chief complaint: Routine clinic visit for cardiovascular risk assessment and medication compliance review

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine clinic visit for cardiovascular risk assessment and medication compliance review

HISTORY OF PRESENT ILLNESS:
The patient with Type 2 Diabetes, Hypertension, HFrEF, and Mixed Hyperlipidemia returns for cardiovascular risk assessment following prior hypertension management. He has been compliant with his medications and reports no new symptoms, feeling generally well. The patient is concerned about his own risk for heart disease and asks if there is anything else he can do to reduce his risk.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Heart Failure with Reduced Ejection Fraction
- Gout
- Diverticulosis
- Mixed Hyperlipidemia

PAST SURGICAL HISTORY:
- Emergency sigmoid colectomy with end colostomy for perforated diverticulitis with peritonitis

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Enalapril 10 mg PO BID
- Allopurinol 300 mg PO daily
- Colchicine 0.6 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
The patient's brother recently had a myocardial infarction at the age of 58

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day. Alcohol: Social, 2-3 drinks per week. Occupation: Office Manager.

VITAL SIGNS:
His blood pressure is 132/84 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
His laboratory results are as follows:

- Hemoglobin A1c: 7.2%
- Fasting blood glucose: 158 mg/dL
- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 200 mg/dL

PLAN:
After reviewing his current treatment regimen and lifestyle, you consider adding an additional medication to address his cardiovascular risk

## Encounter 3: source 2021-03-15, practice 2022-09-08, outpatient, Cardiology

Chief complaint: Follow-up after initiation of valsartan for heart failure management

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Aisha Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up after initiation of valsartan for heart failure management

HISTORY OF PRESENT ILLNESS:
Following prior encounters for hypertension and cardiovascular risk assessment, the patient returns for heart failure follow-up two weeks after initiating valsartan to reduce afterload and improve cardiac remodeling, reporting improved energy levels.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-09-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Heart Failure with Reduced Ejection Fraction
- Gout
- Diverticulosis

PAST SURGICAL HISTORY:
- Emergency sigmoid colectomy with end colostomy for perforated diverticulitis with peritonitis

MEDICATIONS:
He takes valsartan.

Home Medications:
- Metformin 1000 mg PO BID
- Enalapril 10 mg PO BID
- Allopurinol 300 mg PO daily
- Colchicine 0.6 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, hypertension
- Brother: Gout

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day. Alcohol: Social, 2-3 drinks per week. Occupation: Office Manager.

REVIEW OF SYSTEMS:
He denies dizziness, leg swelling, or palpitations

VITAL SIGNS:
Blood pressure of 124/74 mm Hg

LABORATORY DATA:
Potassium level of 4.9 mEq/L and serum creatinine of 1.3 mg/dL (baseline 1.1)

ASSESSMENT AND PLAN:
The physician explains that the medication does not block the production of angiotensin II, but rather interferes with its receptor signaling. As a result, various changes in the renin-angiotensin-aldosterone system are expected.

## Encounter 4: source 2021-04-15, practice 2022-10-24, outpatient, Cardiology

Chief complaint: Follow-up for improved exercise tolerance and reduced shortness of breath

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Aisha Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up for improved exercise tolerance and reduced shortness of breath

HISTORY OF PRESENT ILLNESS:
59-year-old male with heart failure and hypertension returns for follow-up two weeks later reporting improved exercise tolerance and reduced shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-09-15)
- Angiotensin-converting enzyme inhibitor therapy (Enalapril) (diagnosed 2021-03-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Heart Failure with Reduced Ejection Fraction
- Gout
- Diverticulosis

PAST SURGICAL HISTORY:
- Emergency sigmoid colectomy with end colostomy for perforated diverticulitis with peritonitis

MEDICATIONS:
He is started on enalapril.

Home Medications:
- Metformin 1000 mg PO BID
- Allopurinol 300 mg PO daily
- Colchicine 0.6 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, hypertension
- Brother: Gout

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day. Alcohol: Social, 2-3 drinks per week. Occupation: Office Manager.

VITAL SIGNS:
His blood pressure is 122/74 mm Hg

PHYSICAL EXAMINATION:
Physical examination reveals no jugular venous distension or peripheral edema

LABORATORY DATA:
His laboratory values show a stable serum creatinine of 1.2 mg/dL and potassium of 4.7 mEq/L

ASSESSMENT AND PLAN:
The physician explains that the medication reduces the work of the left ventricle by altering vascular tone and modifying renal arteriolar resistance.

## Encounter 5: source 2022-01-15, practice 2023-07-24, outpatient, Internal Medicine

Chief complaint: Routine follow-up with increased gout flares over past two months

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with increased gout flares over past two months

HISTORY OF PRESENT ILLNESS:
The patient, with established gout and prior heart failure management, returns for routine follow-up. He complains of a recent increase in gout flares over the past two months.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-09-15)
- Angiotensin-converting enzyme inhibitor therapy (Enalapril) (diagnosed 2021-03-15)
- Heart failure with reduced ejection fraction (diagnosed 2021-04-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Gout
- Diverticulosis

He also has a history of hypertension,.

PAST SURGICAL HISTORY:
- Emergency sigmoid colectomy with end colostomy for perforated diverticulitis with peritonitis

MEDICATIONS:
His current medications include allopurinol and colchicine. for which he takes hydrochlorothiazide.

Home Medications:
- Metformin 1000 mg PO BID
- Enalapril 10 mg PO BID
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, hypertension
- Brother: Gout

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day. Alcohol: Social, 2-3 drinks per week. Occupation: Office Manager.

REVIEW OF SYSTEMS:
He denies any chest pain, palpitations, or shortness of breath.

VITAL SIGNS:
His blood pressure today is 142/88 mmHg.

PHYSICAL EXAMINATION:
His physical examination is unremarkable except for mild tenderness over the metatarsophalangeal joint of his right big toe.

LABORATORY DATA:
Laboratory values are as follows:

- Serum uric acid: 8.5 mg/dL (normal: 3.5-7.2 mg/dL)
- Serum creatinine: 1.0 mg/dL (normal: 0.6-1.2 mg/dL)
- Serum potassium: 4.2 mEq/L (normal: 3.5-5.0 mEq/L)
- Serum sodium: 140 mEq/L (normal: 135-145 mEq/L)

ASSESSMENT AND PLAN:
Given the patient's medical history and current medication regimen,

## Encounter 6: source 2023-01-15, practice 2024-07-25, outpatient, Internal Medicine

Chief complaint: Two-month history of dull, aching left flank pain without hematuria or fever

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Thompson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of dull, aching left flank pain without hematuria or fever

HISTORY OF PRESENT ILLNESS:
The patient with comorbidities returns. He denies any hematuria, fever, or urinary symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-09-15)
- Angiotensin-converting enzyme inhibitor therapy (Enalapril) (diagnosed 2021-03-15)
- Heart failure with reduced ejection fraction (diagnosed 2021-04-15)
- Gout (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Diverticulosis

His past medical history is significant for hypertension, which is well-controlled with lisinopril.

PAST SURGICAL HISTORY:
- Emergency sigmoid colectomy with end colostomy for perforated diverticulitis with peritonitis

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Metformin 1000 mg PO BID
- Enalapril 10 mg PO BID
- Allopurinol 300 mg PO daily
- Colchicine 0.6 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, hypertension
- Brother: Gout

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day. Alcohol: Social, 2-3 drinks per week. Occupation: Office Manager.

VITAL SIGNS:
Blood pressure is 132/85 mmHg

PHYSICAL EXAMINATION:
On physical examination, there is mild tenderness to palpation in the left flank region. No abdominal masses are palpable, and the remainder of the examination is unremarkable.

IMAGING:
A renal ultrasound is performed, which reveals a 3 cm left renal mass with the following characteristics: a single thin-walled cyst without septations, clear fluid content, and no evidence of solid components or enhancement after administration of contrast material.

ASSESSMENT AND PLAN:
Based on the clinical scenario and ultrasound findings,

## Encounter 7: source 2025-01-15, practice 2026-07-13, ed, Emergency Medicine

Chief complaint: Worsening left lower quadrant abdominal pain, fever, and chills for three days

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Elena Vasquez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening left lower quadrant abdominal pain, fever, and chills for three days

HISTORY OF PRESENT ILLNESS:
This 63-year-old with diverticulosis, seen in 2023 for left flank pain, presents to ED with a 3-day history of worsening left lower quadrant abdominal pain, fever, and chills.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-09-15)
- Angiotensin-converting enzyme inhibitor therapy (Enalapril) (diagnosed 2021-03-15)
- Heart failure with reduced ejection fraction (diagnosed 2021-04-15)
- Gout (diagnosed 2022-01-15)
- Simple renal cyst (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Diverticulosis

PAST SURGICAL HISTORY:
- Emergency sigmoid colectomy with end colostomy for perforated diverticulitis with peritonitis

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Enalapril 10 mg PO BID
- Allopurinol 300 mg PO daily
- Colchicine 0.6 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, hypertension
- Brother: Gout

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day. Alcohol: Social, 2-3 drinks per week. Occupation: Office Manager.

REVIEW OF SYSTEMS:
He denies nausea, vomiting, or changes in bowel habits.

VITAL SIGNS:
His temperature is 38.7°C, blood pressure is 135/85 mmHg, pulse is 100 bpm, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals localized tenderness and guarding in the left lower quadrant.

LABORATORY DATA:
Laboratory tests show leukocytosis with a left shift.

IMAGING:
A CT scan of the abdomen and pelvis with contrast demonstrates a 4 cm abscess associated with sigmoid diverticulitis.

ASSESSMENT AND PLAN:
After 48 hours of treatment, the patient's fever persists and repeat imaging shows no change in the size of the abscess.

The patient is started on intravenous antibiotics.
