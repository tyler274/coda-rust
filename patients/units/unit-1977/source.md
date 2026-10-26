# synthetic_hospital patient 1977

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic White, Commercial high-deductible health plan with limited prescription coverage.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-01-25 (Tue) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2022-04-25 (Mon) | outpatient | Internal Medicine |
| 2 | 2020-10-15 | 2022-10-17 (Mon) | outpatient | Internal Medicine |
| 3 | 2021-07-15 | 2023-07-21 (Fri) | outpatient | Endocrinology |
| 4 | 2022-04-15 | 2024-04-22 (Mon) | outpatient | Cardiology |
| 5 | 2022-10-15 | 2024-10-21 (Mon) | outpatient | Cardiology |
| 6 | 2023-01-15 | 2025-01-27 (Mon) | outpatient | Cardiology |
| 7 | 2024-01-15 | 2026-01-12 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial high-deductible health plan with limited prescription coverage",
 "occupation": "Construction foreman",
 "alcohol_use": "Social alcohol use, 1-2 drinks per week",
 "family_history": [
  "Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 62",
  "Mother: Hyperlipidemia, ischemic stroke at age 70",
  "One sibling with type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 0.5 packs per day for 30 years (30 pack-years)",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (15-year duration)",
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Hyperlipidemia", "Diabetic nephropathy", "Heart failure with reduced ejection fraction", "Chronic Kidney Disease due to Diabetic Nephropathy", "Peripheral arterial disease", "Stable Angina Pectoris (Medical Management)"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-01-25, outpatient, Internal Medicine

Chief complaint: Routine follow-up for chronic hyperlipidemia

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic hyperlipidemia

HISTORY OF PRESENT ILLNESS:
He reports no new symptoms and is adherent to his medication regimen

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Metformin and lisinopril

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 62
- Mother: Hyperlipidemia, ischemic stroke at age 70
- One sibling with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day for 30 years (30 pack-years). Alcohol: Social alcohol use, 1-2 drinks per week. Occupation: Construction foreman.

VITAL SIGNS:
His blood pressure is 132/84 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin A1c: 7.2%
- Fasting plasma glucose: 154 mg/dL
- Total cholesterol: 210 mg/dL
- LDL cholesterol: 130 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 150 mg/dL

ASSESSMENT AND PLAN:
The patient is interested in starting a new medication to further lower his cardiovascular risk. He inquires about the effectiveness of adding a statin to his current regimen. You recall a recent randomized controlled trial comparing the addition of atorvastatin to standard therapy versus standard therapy alone in patients with type 2 diabetes. The study reported a difference in mean LDL cholesterol reduction of 40 mg/dL with a 95% confidence interval (CI) of 35 to 45 mg/dL and a 99% CI of 34 to 46 mg/dL. Based on this information,

## Encounter 1: source 2020-04-15, practice 2022-04-25, outpatient, Internal Medicine

Chief complaint: Stage 2 hypertension management

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Stage 2 hypertension management

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes metformin and atorvastatin.

Home Medications:
- Lisinopril 10 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 62
- Mother: Hyperlipidemia, ischemic stroke at age 70
- One sibling with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day for 30 years (30 pack-years). Alcohol: Social alcohol use, 1-2 drinks per week. Occupation: Construction foreman.

VITAL SIGNS:
Baseline SBP was 160 mmHg, and after 6 months, his SBP was 130 mmHg

## Encounter 2: source 2020-10-15, practice 2022-10-17, outpatient, Internal Medicine

Chief complaint: Routine check-up for chronic kidney disease management

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up for chronic kidney disease management

HISTORY OF PRESENT ILLNESS:
This diabetic returns for CKD care following visits for hypertension and hyperlipidemia. He reports no current symptoms but mentions that he has been noncompliant with his antihypertensive medications for the past six months due to financial constraints.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension

He has a history of type 2 diabetes mellitus and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His current medications include metformin, atorvastatin, and a multivitamin.

Home Medications:
- Lisinopril 10 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 62
- Mother: Hyperlipidemia, ischemic stroke at age 70
- One sibling with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day for 30 years (30 pack-years). Alcohol: Social alcohol use, 1-2 drinks per week. Occupation: Construction foreman.

VITAL SIGNS:
His blood pressure is 162/98 mmHg.

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for mild bilateral lower extremity edema.

LABORATORY DATA:
Laboratory results are as follows:

- Serum creatinine: 2.1 mg/dL (increased from 1.4 mg/dL one year ago)
- Blood urea nitrogen (BUN): 36 mg/dL
- Serum potassium: 4.9 mEq/L
- Serum sodium: 140 mEq/L
- Urinalysis: Proteinuria (1+), no glucose, no ketones, no blood

## Encounter 3: source 2021-07-15, practice 2023-07-21, outpatient, Endocrinology

Chief complaint: Increased fatigue and unintentional weight loss over 3 months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Increased fatigue and unintentional weight loss over 3 months

HISTORY OF PRESENT ILLNESS:
He returns for follow-up of his diabetes and CKD. He reports increased fatigue and a recent unintentional weight loss of 5 kg over the past 3 months. He denies any changes in diet or exercise.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Chronic Kidney Disease due to Diabetic Nephropathy (diagnosed 2020-10-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His current medications include metformin, simvastatin, and lisinopril.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 62
- Mother: Hyperlipidemia, ischemic stroke at age 70
- One sibling with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day for 30 years (30 pack-years). Alcohol: Social alcohol use, 1-2 drinks per week. Occupation: Construction foreman.

VITAL SIGNS:
His blood pressure today is 142/88 mmHg.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral pedal edema.

LABORATORY DATA:
Laboratory results are as follows:

- Serum creatinine: 2.1 mg/dL (increased from 1.4 mg/dL six months ago)
- Urine albumin-to-creatinine ratio: 400 mg/g
- Hemoglobin A1c: 7.5%
- Fasting blood glucose: 154 mg/dL
- Serum potassium: 4.8 mEq/L

## Encounter 4: source 2022-04-15, practice 2024-04-22, outpatient, Cardiology

Chief complaint: Persistent fatigue and left calf cramping with walking

ENCOUNTER NOTE
Date: 2022-04-15
Provider: Dr. David Kim, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue and left calf cramping with walking

HISTORY OF PRESENT ILLNESS:
The patient with diabetes, hypertension and CKD returns for evaluation. The symptoms resolve with rest and have gradually worsened over the past year. He has no chest pain or shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Chronic Kidney Disease due to Diabetic Nephropathy (diagnosed 2020-10-15)
- Diabetic nephropathy (diagnosed 2021-07-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension

Medical history includes type 2 diabetes mellitus, hypertension, and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He is currently taking lisinopril and metformin but stopped his lipid-lowering medication several months ago due to muscle soreness.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 62
- Mother: Hyperlipidemia, ischemic stroke at age 70
- One sibling with type 2 diabetes mellitus

SOCIAL HISTORY:
He smokes half a pack of cigarettes daily and has done so for over 30 years.

VITAL SIGNS:
His blood pressure is 144/88 mm Hg, and pulse is 78/min.

PHYSICAL EXAMINATION:
On physical exam, there are decreased dorsalis pedis pulses bilaterally and evidence of dry, scaly skin on the left foot. Fundoscopic exam shows mild arteriolar narrowing, but no hemorrhages or AV-nicking.

PLAN:
A decision is made to reinitiate statin therapy to reduce his cardiovascular risk.

## Encounter 5: source 2022-10-15, practice 2024-10-21, outpatient, Cardiology

Chief complaint: Progressive exertional chest pain over 3 months

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. David Kim, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive exertional chest pain over 3 months

HISTORY OF PRESENT ILLNESS:
The patient with T2DM, HTN, and HLD returns with exertional chest pain. He describes the pain as a pressure-like sensation in the center of his chest that occurs with walking up hills and resolves with rest. He denies any pain at rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Chronic Kidney Disease due to Diabetic Nephropathy (diagnosed 2020-10-15)
- Diabetic nephropathy (diagnosed 2021-07-15)
- Peripheral arterial disease (diagnosed 2022-04-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His current medications include metformin, lisinopril, and atorvastatin.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 62
- Mother: Hyperlipidemia, ischemic stroke at age 70
- One sibling with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day for 30 years (30 pack-years). Alcohol: Social alcohol use, 1-2 drinks per week. Occupation: Construction foreman.

VITAL SIGNS:
His blood pressure is 135/85 mmHg, and his heart rate is 78 bpm.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
His fasting lipid panel is as follows:

- Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 150 mg/dL

OTHER STUDIES:
An electrocardiogram shows no acute changes.
The patient is scheduled for a stress test, which reveals reversible ischemia.

PLAN:
He is referred to a cardiologist who recommends a randomized controlled trial (RCT) comparing a new anti-anginal medication with a placebo. The study design includes blinding of both participants and researchers.

## Encounter 6: source 2023-01-15, practice 2025-01-27, outpatient, Cardiology

Chief complaint: Follow-up visit for stable angina management

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit for stable angina management

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up with diabetic nephropathy and peripheral arterial disease. He reports a 3-month history of progressive exertional chest pain that resolves with rest. He denies any symptoms of heart failure.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Chronic Kidney Disease due to Diabetic Nephropathy (diagnosed 2020-10-15)
- Diabetic nephropathy (diagnosed 2021-07-15)
- Peripheral arterial disease (diagnosed 2022-04-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His current medications include metformin, lisinopril, and atorvastatin.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 62
- Mother: Hyperlipidemia, ischemic stroke at age 70
- One sibling with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day for 30 years (30 pack-years). Alcohol: Social alcohol use, 1-2 drinks per week. Occupation: Construction foreman.

VITAL SIGNS:
Blood pressure of 135/85 mmHg, heart rate of 78 bpm

PHYSICAL EXAMINATION:
No cardiac murmurs

OTHER STUDIES:
An electrocardiogram shows no significant ST-T wave changes. He undergoes a stress test, which demonstrates reversible ischemia.

PLAN:
He is referred for a double-blind randomized controlled trial comparing a new anti-anginal medication with placebo. The primary outcome is the time to onset of angina during a standardized exercise test. The study coordinators, patients, and data analysts are blinded to the treatment assignments.

## Encounter 7: source 2024-01-15, practice 2026-01-12, ed, Emergency Medicine

Chief complaint: Progressive exertional dyspnea and bilateral ankle swelling

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive exertional dyspnea and bilateral ankle swelling

HISTORY OF PRESENT ILLNESS:
This 62-year-old male with longstanding Type 2 Diabetes Mellitus, chronic kidney disease, and known cardiovascular disease presents with progressive exertional dyspnea and bilateral ankle swelling. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Chronic Kidney Disease due to Diabetic Nephropathy (diagnosed 2020-10-15)
- Diabetic nephropathy (diagnosed 2021-07-15)
- Peripheral arterial disease (diagnosed 2022-04-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 62
- Mother: Hyperlipidemia, ischemic stroke at age 70
- One sibling with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day for 30 years (30 pack-years). Alcohol: Social alcohol use, 1-2 drinks per week. Occupation: Construction foreman.

VITAL SIGNS:
On physical examination, his blood pressure is 140/85 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
He has jugular venous distension, bilateral basal crackles on lung auscultation, and 2+ pitting edema in both ankles.

LABORATORY DATA:
His laboratory values are as follows:

- Serum sodium: 138 mEq/L
- Serum potassium: 4.5 mEq/L
- Serum creatinine: 1.2 mg/dL
- B-type natriuretic peptide (BNP): 580 pg/mL
- Fasting blood glucose: 160 mg/dL
- Hemoglobin A1c: 7.8%

IMAGING:
An echocardiogram shows left ventricular hypertrophy with an ejection fraction of 45%.
