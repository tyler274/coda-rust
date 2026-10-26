# synthetic_hospital patient 2011

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex M, Hispanic/Latino, Medicare (primary, ESRD-eligible) with Medicaid secondary.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-12-14 (Wed) | outpatient | Cardiology |
| 1 | 2020-04-15 | 2023-03-13 (Mon) | outpatient | Nephrology |
| 2 | 2021-01-15 | 2023-12-10 (Sun) | ed | Emergency Medicine |
| 3 | 2022-01-15 | 2024-12-14 (Sat) | ed | Emergency Medicine |
| 4 | 2023-01-15 | 2025-12-12 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (primary, ESRD-eligible) with Medicaid secondary",
 "occupation": "Manufacturing technician at automotive parts facility",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father: Type 2 diabetes mellitus, myocardial infarction at age 54",
  "Mother: Hypertension, stroke at age 68",
  "Brother: Coronary artery disease"
 ],
 "race_ethnicity": "Hispanic/Latino",
 "smoking_status": "Former smoker, 30 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "4000 units IV 3x weekly (with dialysis)",
   "name": "Epoetin alfa"
  },
  {
   "dose": "667 mg PO with meals",
   "name": "Calcium acetate"
  },
  {
   "dose": "800 mg PO with meals",
   "name": "Sevelamer carbonate"
  }
 ],
 "surgical_history": [
  "Left upper extremity arteriovenous fistula creation",
  "Cardiac catheterization with drug-eluting stent placement to LAD (2 years prior)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus with diabetic nephropathy",
  "Hypertension",
  "Coronary Artery Disease status post myocardial infarction (2 years prior)",
  "Heart Failure with Reduced Ejection Fraction",
  "End-Stage Renal Disease on Maintenance Hemodialysis (initiated 2 years prior)",
  "Anemia of Chronic Kidney Disease"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["End-stage renal disease on hemodialysis", "Heart failure with reduced ejection fraction", "COVID-19", "Acute ischemic stroke", "Stable Angina Pectoris"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-12-14, outpatient, Cardiology

Chief complaint: Progressive exertional chest pain over the past 2 months, described as pressure-like sensation in the center of the chest occurring during daily walks and subsiding with rest.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive exertional chest pain over the past 2 months, described as pressure-like sensation in the center of the chest occurring during daily walks and subsiding with rest.

HISTORY OF PRESENT ILLNESS:
He describes the pain as a pressure-like sensation in the center of his chest, which occurs during his daily walks and subsides with rest. He denies any symptoms at rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus with diabetic nephropathy
- Hypertension
- Coronary Artery Disease status post myocardial infarction (2 years prior)
- Heart Failure with Reduced Ejection Fraction
- End-Stage Renal Disease on Maintenance Hemodialysis (initiated 2 years prior)
- Anemia of Chronic Kidney Disease

PAST SURGICAL HISTORY:
- Left upper extremity arteriovenous fistula creation
- Cardiac catheterization with drug-eluting stent placement to LAD (2 years prior)

MEDICATIONS:
His current medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Insulin glargine 24 units subcutaneous at bedtime
- Epoetin alfa 4000 units IV 3x weekly (with dialysis)
- Calcium acetate 667 mg PO with meals
- Sevelamer carbonate 800 mg PO with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
His father had a myocardial infarction at age 60.

SOCIAL HISTORY:
He has a 30-pack-year smoking history but quit 5 years ago.

VITAL SIGNS:
His blood pressure is 142/88 mmHg, heart rate is 78 bpm, and BMI is 31 kg/m^2

PHYSICAL EXAMINATION:
Cardiac examination reveals a regular rhythm without murmurs, rubs, or gallops.

LABORATORY DATA:
His fasting lipid panel is as follows:

- Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 150 mg/dL

OTHER STUDIES:
An electrocardiogram shows no acute changes.

## Encounter 1: source 2020-04-15, practice 2023-03-13, outpatient, Nephrology

Chief complaint: Follow-up visit for end-stage renal disease with reports of declining quality of life, persistent fatigue, and difficulty managing fluid restrictions despite optimal dialysis.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit for end-stage renal disease with reports of declining quality of life, persistent fatigue, and difficulty managing fluid restrictions despite optimal dialysis.

HISTORY OF PRESENT ILLNESS:
This patient with ESRD returns after cardiology evaluation. Despite optimal dialysis treatment, he reports a declining quality of life, persistent fatigue, and difficulty managing his fluid restrictions.

PAST MEDICAL HISTORY:
Active Problem List:
- Stable Angina Pectoris (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus with diabetic nephropathy
- Hypertension
- Coronary Artery Disease status post myocardial infarction (2 years prior)
- Heart Failure with Reduced Ejection Fraction
- End-Stage Renal Disease on Maintenance Hemodialysis (initiated 2 years prior)
- Anemia of Chronic Kidney Disease

He has been on hemodialysis for the past 2 years due to end-stage renal disease (ESRD).

PAST SURGICAL HISTORY:
- Left upper extremity arteriovenous fistula creation
- Cardiac catheterization with drug-eluting stent placement to LAD (2 years prior)

MEDICATIONS:
His current medications include erythropoietin, calcium acetate, and sevelamer.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Lisinopril 10 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Insulin glargine 24 units subcutaneous at bedtime
- Epoetin alfa 4000 units IV 3x weekly (with dialysis)
- Sevelamer carbonate 800 mg PO with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
His sister, who has no significant medical history, has offered to be a living kidney donor.

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Occasional (1-2 drinks per month). Occupation: Manufacturing technician at automotive parts facility.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Recent laboratory values are as follows:
- Hemoglobin: 9.8 g/dL (normal: 13.8-17.2 g/dL)
- Calcium: 8.9 mg/dL (normal: 8.5-10.2 mg/dL)
- Phosphorus: 5.5 mg/dL (normal: 2.5-4.5 mg/dL)
- PTH: 300 pg/mL (normal: 15-65 pg/mL)

## Encounter 2: source 2021-01-15, practice 2023-12-10, ed, Emergency Medicine

Chief complaint: Progressive exertional dyspnea and bilateral ankle swelling over the past 6 months with acute worsening.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive exertional dyspnea and bilateral ankle swelling over the past 6 months with acute worsening.

HISTORY OF PRESENT ILLNESS:
The patient with known heart failure with reduced ejection fraction and end-stage renal disease on maintenance hemodialysis, previously evaluated for stable angina and dialysis management, presents to the emergency department with a 6-month history of progressive exertional dyspnea and bilateral ankle swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Stable Angina Pectoris (diagnosed 2020-01-15)
- End-stage renal disease on hemodialysis (diagnosed 2020-04-15)
- Type 2 Diabetes Mellitus with diabetic nephropathy
- Hypertension
- Coronary Artery Disease status post myocardial infarction (2 years prior)
- Heart Failure with Reduced Ejection Fraction
- End-Stage Renal Disease on Maintenance Hemodialysis (initiated 2 years prior)
- Anemia of Chronic Kidney Disease

He has a past medical history of hypertension, type 2 diabetes mellitus, and a myocardial infarction 2 years ago.

PAST SURGICAL HISTORY:
- Left upper extremity arteriovenous fistula creation
- Cardiac catheterization with drug-eluting stent placement to LAD (2 years prior)

MEDICATIONS:
He reports adherence to his medication regimen, which includes metoprolol, lisinopril, aspirin, and metformin.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Insulin glargine 24 units subcutaneous at bedtime
- Epoetin alfa 4000 units IV 3x weekly (with dialysis)
- Calcium acetate 667 mg PO with meals
- Sevelamer carbonate 800 mg PO with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 54
- Mother: Hypertension, stroke at age 68
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Occasional (1-2 drinks per month). Occupation: Manufacturing technician at automotive parts facility.

VITAL SIGNS:
On physical examination, his blood pressure is 150/90 mmHg, heart rate is 78 bpm, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
He has jugular venous distension, crackles at the lung bases, and 2+ pitting edema in both ankles.

LABORATORY DATA:
Laboratory findings are as follows:
- Serum sodium: 138 mEq/L
- Serum potassium: 4.5 mEq/L
- Serum creatinine: 1.2 mg/dL
- Fasting blood glucose: 158 mg/dL
- B-type natriuretic peptide (BNP): 580 pg/mL

IMAGING:
- Chest X-ray: Cardiomegaly with cephalization of pulmonary vasculature

## Encounter 3: source 2022-01-15, practice 2024-12-14, ed, Emergency Medicine

Chief complaint: Fever, dry cough, myalgia, and worsening shortness of breath for the past 5 days after exposure to coworkers with confirmed respiratory virus infection.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, dry cough, myalgia, and worsening shortness of breath for the past 5 days after exposure to coworkers with confirmed respiratory virus infection.

HISTORY OF PRESENT ILLNESS:
This 59-year-old male with a history of Type 2 Diabetes Mellitus with diabetic nephropathy, Coronary Artery Disease status post myocardial infarction, Heart Failure with Reduced Ejection Fraction, and End-Stage Renal Disease on Maintenance Hemodialysis, previously evaluated in this emergency department approximately one year ago for progressive exertional dyspnea, presents to the emergency department with fever, dry cough, myalgia, and worsening shortness of breath for the past 5 days. He notes that several coworkers recently became ill with similar symptoms, and he was notified that one of them tested positive for a novel respiratory virus. He has not received any vaccinations in the past year.

PAST MEDICAL HISTORY:
Active Problem List:
- Stable Angina Pectoris (diagnosed 2020-01-15)
- End-stage renal disease on hemodialysis (diagnosed 2020-04-15)
- Heart failure with reduced ejection fraction (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus with diabetic nephropathy
- Hypertension
- Coronary Artery Disease status post myocardial infarction (2 years prior)
- End-Stage Renal Disease on Maintenance Hemodialysis (initiated 2 years prior)
- Anemia of Chronic Kidney Disease

He has a history of type 2 diabetes and hypertension.

PAST SURGICAL HISTORY:
- Left upper extremity arteriovenous fistula creation
- Cardiac catheterization with drug-eluting stent placement to LAD (2 years prior)

MEDICATIONS:
Home Medications:
- Metoprolol succinate 50 mg PO daily
- Lisinopril 10 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Insulin glargine 24 units subcutaneous at bedtime
- Epoetin alfa 4000 units IV 3x weekly (with dialysis)
- Calcium acetate 667 mg PO with meals
- Sevelamer carbonate 800 mg PO with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 54
- Mother: Hypertension, stroke at age 68
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: Occasional (1-2 drinks per month). Occupation: Manufacturing technician at automotive parts facility.

VITAL SIGNS:
On exam, his temperature is 38.5°C (101.3°F), heart rate is 110/min, respiratory rate is 24/min, and oxygen saturation is 89% on room air.

PHYSICAL EXAMINATION:
He appears fatigued and dyspneic with diffuse crackles on lung auscultation.

LABORATORY DATA:
Nasopharyngeal PCR testing confirms infection with the viral agent responsible for a global pandemic during 2020 and 2021.

IMAGING:
Chest imaging shows bilateral ground-glass opacities.

## Encounter 4: source 2023-01-15, practice 2025-12-12, ed, Emergency Medicine

Chief complaint: Sudden onset of left-sided weakness and difficulty speaking that began 45 minutes ago.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of left-sided weakness and difficulty speaking that began 45 minutes ago.

HISTORY OF PRESENT ILLNESS:
This 60-year-old male with ESRD on maintenance hemodialysis, CAD s/p MI, and HFrEF presents to the emergency department. The son denies recent surgeries, head trauma, or anticoagulant use.

PAST MEDICAL HISTORY:
Active Problem List:
- Stable Angina Pectoris (diagnosed 2020-01-15)
- End-stage renal disease on hemodialysis (diagnosed 2020-04-15)
- Heart failure with reduced ejection fraction (diagnosed 2021-01-15)
- COVID-19 (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus with diabetic nephropathy
- Hypertension
- Coronary Artery Disease status post myocardial infarction (2 years prior)
- End-Stage Renal Disease on Maintenance Hemodialysis (initiated 2 years prior)
- Anemia of Chronic Kidney Disease

The patient has a history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Left upper extremity arteriovenous fistula creation
- Cardiac catheterization with drug-eluting stent placement to LAD (2 years prior)

MEDICATIONS:
His medications include lisinopril and metformin.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Insulin glargine 24 units subcutaneous at bedtime
- Epoetin alfa 4000 units IV 3x weekly (with dialysis)
- Calcium acetate 667 mg PO with meals
- Sevelamer carbonate 800 mg PO with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 54
- Mother: Hypertension, stroke at age 68
- Brother: Coronary artery disease

SOCIAL HISTORY:
His son speaks limited English and there is no immediate access to a professional interpreter. The nearest interpreter is 30 minutes away.

VITAL SIGNS:
Blood pressure of 185/95 mmHg, heart rate of 88 beats per minute, and irregularly irregular pulse

PHYSICAL EXAMINATION:
On examination, the patient is alert but anxious. He has expressive aphasia and left facial droop.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 14 g/dL
- Platelet count: 250,000/mm^3
- INR: 1.0
- Creatinine: 1.2 mg/dL

IMAGING:
A non-contrast head CT scan is performed urgently and shows no hemorrhage.
