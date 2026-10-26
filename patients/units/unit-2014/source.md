# synthetic_hospital patient 2014

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic White, Employer-sponsored commercial health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-04-13 (Wed) | outpatient | Cardiology |
| 1 | 2020-04-15 | 2022-07-11 (Mon) | outpatient | Rheumatology |
| 2 | 2020-07-15 | 2022-09-27 (Tue) | outpatient | Internal Medicine |
| 3 | 2021-01-15 | 2023-03-31 (Fri) | outpatient | Internal Medicine |
| 4 | 2021-07-15 | 2023-10-05 (Thu) | outpatient | Endocrinology |
| 5 | 2022-01-15 | 2024-04-15 (Mon) | outpatient | Oncology |
| 6 | 2022-07-15 | 2024-10-14 (Mon) | outpatient | Nephrology |
| 7 | 2023-07-15 | 2025-09-26 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Employer-sponsored commercial health insurance",
 "occupation": "Maintenance engineer at a large metropolitan hotel",
 "alcohol_use": "None",
 "family_history": [
  "Father: Type 2 diabetes mellitus, hypertension",
  "Mother: Hypertension, hyperlipidemia",
  "Paternal uncle: Tophaceous gout",
  "Maternal grandfather: Lung cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
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
   "dose": "300 mg PO daily",
   "name": "Allopurinol"
  }
 ],
 "surgical_history": [
  "Mediastinoscopy with lymph node biopsy"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (15-year history)",
  "Hypertension",
  "Hyperlipidemia",
  "Tophaceous Gout",
  "Diabetic Nephropathy",
  "Stage IIIB Non-Small Cell Lung Cancer"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Legionnaires' disease", "Stage IIIB Non-Small Cell Lung Cancer", "Diabetic nephropathy", "Tophaceous gout", "Hyperlipidemia"]
Comorbidities: ["History of recent pneumonia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-04-13, outpatient, Cardiology

Chief complaint: Routine check-up for cardiovascular risk management

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up for cardiovascular risk management

HISTORY OF PRESENT ILLNESS:
He has no complaints

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension
- Hyperlipidemia
- Tophaceous Gout
- Diabetic Nephropathy
- Stage IIIB Non-Small Cell Lung Cancer

PAST SURGICAL HISTORY:
- Mediastinoscopy with lymph node biopsy

MEDICATIONS:
He takes currently taking metformin and lisinopril.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Allopurinol 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His father had a myocardial infarction at the age of 60

SOCIAL HISTORY:
He quit smoking 5 years ago and does not consume alcohol

VITAL SIGNS:
BMI is 29 kg/m^2, blood pressure is 135/85 mmHg, and pulse is 78 bpm

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Laboratory results are as follows:

- Total cholesterol: 210 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 160 mg/dL
- Triglycerides: 150 mg/dL
- HbA1c: 7.2%

ASSESSMENT AND PLAN:
Based on the Pooled Cohort Equations cardiovascular risk calculator,

## Encounter 1: source 2020-04-15, practice 2022-07-11, outpatient, Rheumatology

Chief complaint: Progressive pain and swelling in right hand over 3 months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Okafor, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive pain and swelling in right hand over 3 months

HISTORY OF PRESENT ILLNESS:
The patient with established tophaceous gout returns three months after his Cardiology visit. He reports that the pain is worse in the morning and improves with activity throughout the day. He has a history of episodic severe pain in his right big toe, which he describes as "excruciating" and "burning," occurring several times over the past two years. These episodes usually resolve within a week with ibuprofen. He denies any recent trauma or fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension
- Tophaceous Gout
- Diabetic Nephropathy
- Stage IIIB Non-Small Cell Lung Cancer

His medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Mediastinoscopy with lymph node biopsy

MEDICATIONS:
Current medications include lisinopril and metformin.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Allopurinol 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, hyperlipidemia
- Paternal uncle: Tophaceous gout
- Maternal grandfather: Lung cancer

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: None. Occupation: Maintenance engineer at a large metropolitan hotel.

PHYSICAL EXAMINATION:
On physical examination, you note firm, asymmetric nodules around the distal interphalangeal joints of his right hand. His right first metatarsophalangeal joint is swollen and tender to palpation.

LABORATORY DATA:
Laboratory results are as follows:

- Serum uric acid: 9.2 mg/dL (normal: 3.5-7.2 mg/dL)
- Erythrocyte sedimentation rate (ESR): 30 mm/hr (normal: 0-22 mm/hr)
- C-reactive protein (CRP): 2.5 mg/dL (normal: <0.8 mg/dL)
- Complete blood count (CBC): within normal limits
- Renal function tests: within normal limits

## Encounter 2: source 2020-07-15, practice 2022-09-27, outpatient, Internal Medicine

Chief complaint: Follow-up of chronic conditions with dietary compliance concerns

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up of chronic conditions with dietary compliance concerns

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of chronic conditions including diabetic nephropathy, following recent Rheumatology evaluation for tophaceous gout. He has not been consistent with his sodium restriction but reports no other complaints. Blood pressure readings over the past year have ranged between 138–144/82–90 mm Hg.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Tophaceous gout (diagnosed 2020-04-15)
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension
- Diabetic Nephropathy
- Stage IIIB Non-Small Cell Lung Cancer

He was diagnosed with type 2 diabetes mellitus 6 years ago and hypertension 2 years ago.

PAST SURGICAL HISTORY:
- Mediastinoscopy with lymph node biopsy

MEDICATIONS:
He takes metformin and aspirin daily.

Home Medications:
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Allopurinol 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, hyperlipidemia
- Paternal uncle: Tophaceous gout
- Maternal grandfather: Lung cancer

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: None. Occupation: Maintenance engineer at a large metropolitan hotel.

VITAL SIGNS:
Today, his blood pressure is 148/86 mm Hg and pulse is 76/min.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory studies show a serum creatinine of 1.1 mg/dL and urinalysis reveals a protein +1 level and no glucose.

## Encounter 3: source 2021-01-15, practice 2023-03-31, outpatient, Internal Medicine

Chief complaint: Follow-up visit while participating in antihypertensive clinical trial

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit while participating in antihypertensive clinical trial

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up six months after his prior Internal Medicine visit, with ongoing management of diabetes, hypertension, hyperlipidemia, gout, and lung cancer. He is currently participating in a double-blind, placebo-controlled clinical trial evaluating the efficacy of a new antihypertensive medication. His blood pressure has been well-controlled on metoprolol and lisinopril. He reports no new symptoms. The patient is curious about the randomization process of the clinical trial he is enrolled in.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Tophaceous gout (diagnosed 2020-04-15)
- Diabetic nephropathy (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension
- Stage IIIB Non-Small Cell Lung Cancer

PAST SURGICAL HISTORY:
- Mediastinoscopy with lymph node biopsy

MEDICATIONS:
His current medications include metoprolol, lisinopril, and the study medication.

Home Medications:
- Metformin 1000 mg PO twice daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Allopurinol 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, hyperlipidemia
- Paternal uncle: Tophaceous gout
- Maternal grandfather: Lung cancer

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: None. Occupation: Maintenance engineer at a large metropolitan hotel.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

## Encounter 4: source 2021-07-15, practice 2023-10-05, outpatient, Endocrinology

Chief complaint: Routine follow-up with new onset fatigue and frothy urine

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with new onset fatigue and frothy urine

HISTORY OF PRESENT ILLNESS:
The patient returns for diabetic nephropathy follow-up, reporting increasing fatigue and frothy urine over the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Tophaceous gout (diagnosed 2020-04-15)
- Diabetic nephropathy (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension
- Stage IIIB Non-Small Cell Lung Cancer

PAST SURGICAL HISTORY:
- Mediastinoscopy with lymph node biopsy

MEDICATIONS:
He takes metformin.

Home Medications:
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Allopurinol 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, hyperlipidemia
- Paternal uncle: Tophaceous gout
- Maternal grandfather: Lung cancer

SOCIAL HISTORY:
He follows a strict diet

VITAL SIGNS:
His blood pressure is 145/90 mmHg.

PHYSICAL EXAMINATION:
Physical examination reveals mild bilateral lower extremity edema. His eyes show signs of proliferative diabetic retinopathy.

LABORATORY DATA:
Serum creatinine: 2.1 mg/dL (normal: 0.6-1.2 mg/dL)
- Blood urea nitrogen (BUN): 36 mg/dL (normal: 7-20 mg/dL)
- Urinalysis: 2+ protein
- Urine albumin-to-creatinine ratio: 400 mg/g

## Encounter 5: source 2022-01-15, practice 2024-04-15, outpatient, Oncology

Chief complaint: Persistent cough and unintentional weight loss of 10 kg over 3 months

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Robert Kim, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough and unintentional weight loss of 10 kg over 3 months

HISTORY OF PRESENT ILLNESS:
The patient returns for oncology evaluation with new symptoms, with recent endocrinology follow-up for diabetic nephropathy and established tophaceous gout. The patient complains of a persistent cough and unintentional weight loss of 10 kg over the past 3 months. The patient expresses his interest in participating in a clinical trial for a new treatment he heard about on the news.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Tophaceous gout (diagnosed 2020-04-15)
- Diabetic nephropathy (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension
- Stage IIIB Non-Small Cell Lung Cancer

He was recently diagnosed with stage IIIB non-small cell lung cancer (NSCLC) and is currently not on any treatment.

PAST SURGICAL HISTORY:
- Mediastinoscopy with lymph node biopsy

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Allopurinol 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, hyperlipidemia
- Paternal uncle: Tophaceous gout
- Maternal grandfather: Lung cancer

SOCIAL HISTORY:
He is a former smoker with a 30-pack-year history but quit 5 years ago.

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic. Auscultation of the lungs reveals decreased breath sounds in the right lower lobe. The rest of the examination is unremarkable.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 12.5 g/dL
- White blood cell count: 7,000/mm^3
- Platelets: 250,000/mm^3
- Fasting blood glucose: 160 mg/dL
- Creatinine: 1.0 mg/dL
- AST: 35 U/L
- ALT: 30 U/L

IMAGING:
His recent chest CT scan showed a 4 cm mass in the right lower lobe of the lung with mediastinal lymph node involvement.

ASSESSMENT AND PLAN:
The patient inquires about the potential benefits and risks of joining a phase III clinical trial for a new targeted therapy for NSCLC compared to the standard chemotherapy regimen.

## Encounter 6: source 2022-07-15, practice 2024-10-14, outpatient, Nephrology

Chief complaint: Routine follow-up with worsening fatigue and frothy urine

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Amara Okafor, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with worsening fatigue and frothy urine

HISTORY OF PRESENT ILLNESS:
The patient with long-standing diabetes and recently diagnosed Stage IIIB NSCLC returns to Nephrology Clinic for follow-up. He reports increasing fatigue and frothy urine over the past few months. His diabetes has been poorly controlled with a recent HbA1c of 9.2%, and his blood pressure has been persistently elevated despite adherence to his medication regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Tophaceous gout (diagnosed 2020-04-15)
- Diabetic nephropathy (diagnosed 2020-07-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension

PAST SURGICAL HISTORY:
- Mediastinoscopy with lymph node biopsy

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Allopurinol 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, hyperlipidemia
- Paternal uncle: Tophaceous gout
- Maternal grandfather: Lung cancer

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: None. Occupation: Maintenance engineer at a large metropolitan hotel.

PHYSICAL EXAMINATION:
Bilateral lower extremity edema

LABORATORY DATA:
Urinalysis shows 3+ proteinuria, and serum laboratory tests reveal the following:

- Serum creatinine: 2.1 mg/dL (Normal: 0.6-1.3 mg/dL)
- Blood urea nitrogen (BUN): 36 mg/dL (Normal: 7-20 mg/dL)
- Fasting glucose: 198 mg/dL (Normal: 70-99 mg/dL)
- Urine albumin-to-creatinine ratio: 400 mg/g (Normal: <30 mg/g)

## Encounter 7: source 2023-07-15, practice 2025-09-26, ed, Emergency Medicine

Chief complaint: High-grade fever, productive cough, shortness of breath, diarrhea, and confusion

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
High-grade fever, productive cough, shortness of breath, diarrhea, and confusion

HISTORY OF PRESENT ILLNESS:
This 61-year-old male with Type 2 Diabetes Mellitus, diabetic nephropathy, and Stage IIIB Lung Cancer presents to the emergency department with a 5-day history of high-grade fever, cough productive of purulent sputum, and shortness of breath. He also reports diarrhea and confusion over the past two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Tophaceous gout (diagnosed 2020-04-15)
- Diabetic nephropathy (diagnosed 2020-07-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus (15-year history)
- Hypertension

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Mediastinoscopy with lymph node biopsy

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Allopurinol 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypertension, hyperlipidemia
- Paternal uncle: Tophaceous gout
- Maternal grandfather: Lung cancer

SOCIAL HISTORY:
He works as a maintenance engineer in a large hotel.

VITAL SIGNS:
On physical examination, his temperature is 39.2°C (102.5°F), blood pressure is 135/85 mmHg, pulse is 102 beats/min, and respiratory rate is 22 breaths/min.

PHYSICAL EXAMINATION:
Chest auscultation reveals bilateral crackles.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 15,000/µL (80% neutrophils)
- Hemoglobin: 13.5 g/dL
- Platelet count: 250,000/µL
- Sodium: 130 mEq/L
- Creatinine: 1.2 mg/dL
- Liver enzymes: Mildly elevated
- Urine antigen test for Legionella: Pending

IMAGING:
Chest X-ray shows patchy infiltrates in both lower lobes.
