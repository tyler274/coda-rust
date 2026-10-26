# synthetic_hospital patient 1989

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Aetna Better Health (employer-sponsored PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-07-07 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2022-01-04 (Tue) | outpatient | Endocrinology |
| 2 | 2021-07-15 | 2023-01-03 (Tue) | outpatient | Orthopedics |
| 3 | 2022-07-15 | 2024-01-01 (Mon) | outpatient | Cardiology |
| 4 | 2023-07-15 | 2025-01-06 (Mon) | outpatient | Endocrinology |
| 5 | 2024-01-15 | 2025-06-24 (Tue) | ed | Emergency Medicine |
| 6 | 2024-07-15 | 2026-01-01 (Thu) | outpatient | Internal Medicine |
| 7 | 2025-01-15 | 2026-07-06 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Aetna Better Health (employer-sponsored PPO)",
 "occupation": "Carpenter (self-employed)",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Myocardial infarction at age 64, Hypertension",
  "Mother: Type 2 Diabetes Mellitus, Dyslipidemia",
  "Maternal uncle: Stroke at age 70"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 18 pack-years, quit 3 months ago",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
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
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Hyperlipidemia",
  "Coronary Artery Disease"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Poorly controlled diabetes mellitus", "Right oculomotor nerve palsy", "Renal tubular acidosis", "Ulnar neuropathy at the elbow (Cubital tunnel syndrome)", "Essential hypertension", "Stable Angina Pectoris (Medical Management)"]
Comorbidities: ["Poorly controlled diabetes mellitus", "Essential hypertension", "Hyperlipidemia", "Obesity"]

## Encounter 0: source 2020-01-15, practice 2021-07-07, outpatient, Internal Medicine

Chief complaint: Routine health check-up after elevated blood pressure was noted at a recent health fair on two separate occasions.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health check-up after elevated blood pressure was noted at a recent health fair on two separate occasions.

HISTORY OF PRESENT ILLNESS:
He has no complaints but mentions that during a recent health fair, his blood pressure was noted to be elevated on two separate occasions.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Coronary Artery Disease

His past medical history is significant for type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes for which he takes metformin.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
He has a family history of stroke

SOCIAL HISTORY:
He smokes half a pack of cigarettes per day

VITAL SIGNS:
On examination, his blood pressure is 162/98 mmHg, heart rate is 78 bpm, and BMI is 32 kg/m^2.

PHYSICAL EXAMINATION:
His cardiovascular examination is unremarkable, and there is no evidence of end-organ damage.

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
- Fasting serum glucose: 126 mg/dL
- Hemoglobin A1c: 7.2%

## Encounter 1: source 2020-07-15, practice 2022-01-04, outpatient, Endocrinology

Chief complaint: Management of poorly controlled type 2 diabetes mellitus despite adherence to dietary modifications and regular exercise for the past 6 months.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Management of poorly controlled type 2 diabetes mellitus despite adherence to dietary modifications and regular exercise for the past 6 months.

HISTORY OF PRESENT ILLNESS:
The patient with diabetes, hypertension, hyperlipidemia, and CAD returns to endocrinology following January 2020 internal medicine care despite adherence to dietary modifications and regular exercise for past 6 months. He was diagnosed 2 years ago and initially managed with metformin, which he tolerates well. He has no history of hypoglycemic episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Coronary Artery Disease

His medical history is notable for hypertension and dyslipidemia.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His current medications are metformin, lisinopril, and atorvastatin.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64, Hypertension
- Mother: Type 2 Diabetes Mellitus, Dyslipidemia
- Maternal uncle: Stroke at age 70

SOCIAL HISTORY:
He does not smoke or drink alcohol.

VITAL SIGNS:
BMI of 31 kg/m² and blood pressure of 134/78 mm Hg

PHYSICAL EXAMINATION:
Fundoscopic examination reveals no diabetic retinopathy. Neurologic examination is unremarkable.

LABORATORY DATA:
Hemoglobin A1c of 8.2%, fasting plasma glucose of 165 mg/dL, and a serum creatinine of 0.8 mg/dL

PLAN:
The decision is made to add an oral hypoglycemic agent that increases pancreatic β-cell insulin secretion by binding to and inhibiting ATP-dependent potassium channels on the β-cell membrane.

## Encounter 2: source 2021-07-15, practice 2023-01-03, outpatient, Orthopedics

Chief complaint: Several months of worsening weakness in his dominant right hand causing difficulty with carpentry tools and dropping small objects.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Several months of worsening weakness in his dominant right hand causing difficulty with carpentry tools and dropping small objects.

HISTORY OF PRESENT ILLNESS:
The patient is a 57-year-old male with coronary artery disease, hyperlipidemia, hypertension, and type 2 diabetes mellitus previously evaluated in Internal Medicine and Endocrinology for blood pressure and glycemic management, who presents with several months of worsening weakness in his dominant right hand. He works as a carpenter and has noticed increasing difficulty using his tools, particularly when trying to hold small nails between his fingers or when stabilizing pieces of wood that require hand dexterity. He frequently drops screws or washers, especially when trying to pick them up with his ring and little fingers. Over time, he has also observed a slow but progressive shrinkage of the muscles between his fingers and around the base of his little finger. He initially thought it was due to aging and overuse, but now it is beginning to interfere with his work and daily functioning. He denies pain, trauma, or systemic symptoms but does report intermittent numbness and tingling in the medial hand that worsens with prolonged elbow flexion or when resting his arm on a hard surface. He sleeps with his arms bent under his pillow, and he sometimes notices hand numbness in the morning that resolves after a few minutes. At work, he frequently leans his elbows against counters and tool benches. He has tried wearing a wrist splint at night without significant benefit. He has not seen any change in his grip strength while shaking hands or squeezing with his palm but finds tasks requiring finger spreading or pinching between fingers increasingly difficult. He also notes trouble playing guitar, especially forming chords that require abducting and adducting individual fingers. He has no symptoms in his left hand and no known history of neck problems.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Poorly controlled diabetes mellitus (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Coronary Artery Disease

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64, Hypertension
- Mother: Type 2 Diabetes Mellitus, Dyslipidemia
- Maternal uncle: Stroke at age 70

SOCIAL HISTORY:
He does not smoke and drinks socially

PHYSICAL EXAMINATION:
On physical exam, there is marked wasting of the hypothenar eminence and the first dorsal interosseous muscle on the right. The hand shows mild clawing of the fourth and fifth digits at rest. Strength testing reveals weakness in finger abduction and adduction, most noticeable when asked to spread his fingers against resistance or hold a piece of paper between the fingers. Sensation is reduced over the palmar and dorsal surfaces of the fifth digit and the medial half of the fourth digit. Grip strength is mildly reduced but wrist flexion, elbow strength, and forearm pronation/supination are intact. Tinel sign is positive at the right elbow.

ASSESSMENT AND PLAN:
The physician explains that the pattern of sensory and motor findings indicates damage to a specific nerve supplying most of the intrinsic hand muscles.

## Encounter 3: source 2022-07-15, practice 2024-01-01, outpatient, Cardiology

Chief complaint: Intermittent chest pain described as pressure-like sensation over the left chest radiating to the left arm during morning walks for the past two months.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James O'Brien, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent chest pain described as pressure-like sensation over the left chest radiating to the left arm during morning walks for the past two months.

HISTORY OF PRESENT ILLNESS:
This patient with coronary artery disease, hypertension, and diabetes presents to cardiology with chest pain. The pain is described as a pressure-like sensation over the left chest, radiating to the left arm, and occurs during his morning walks. It typically resolves with rest. He denies any symptoms at rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Poorly controlled diabetes mellitus (diagnosed 2020-07-15)
- Ulnar neuropathy at the elbow (Cubital tunnel syndrome) (diagnosed 2021-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Coronary Artery Disease

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His current medications include lisinopril and atorvastatin.

Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64, Hypertension
- Mother: Type 2 Diabetes Mellitus, Dyslipidemia
- Maternal uncle: Stroke at age 70

SOCIAL HISTORY:
Smoking: Former smoker, 18 pack-years, quit 3 months ago. Alcohol: Social (1-2 drinks per week). Occupation: Carpenter (self-employed).

VITAL SIGNS:
His blood pressure is 138/86 mmHg, heart rate is 78 bpm, and BMI is 30 kg/m^2.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

IMAGING:
Subsequent coronary angiography reveals a 70% stenosis of the left anterior descending artery.

OTHER STUDIES:
An exercise stress test shows ST-segment depression in the anterior leads at high levels of exertion.

PLAN:
The patient is enrolled in a clinical trial comparing a new anti-anginal medication to placebo. After 6 months, the trial results show a p-value of 0.008 for the difference in exercise tolerance between the new medication and placebo.

## Encounter 4: source 2023-07-15, practice 2025-01-06, outpatient, Endocrinology

Chief complaint: Routine follow-up for diabetes management with persistent suboptimal glycemic control despite adherence to metformin and lifestyle modifications.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for diabetes management with persistent suboptimal glycemic control despite adherence to metformin and lifestyle modifications.

HISTORY OF PRESENT ILLNESS:
The patient returns to Endocrinology for ongoing diabetes management with established hypertension and coronary artery disease. He was diagnosed with diabetes 5 years ago and has been on metformin 1000 mg twice daily. Despite adherence to metformin and lifestyle modifications, his glycemic control has been suboptimal.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Poorly controlled diabetes mellitus (diagnosed 2020-07-15)
- Ulnar neuropathy at the elbow (Cubital tunnel syndrome) (diagnosed 2021-07-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2022-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Coronary Artery Disease

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His current medications include aspirin, atorvastatin, and metoprolol.

Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64, Hypertension
- Mother: Type 2 Diabetes Mellitus, Dyslipidemia
- Maternal uncle: Stroke at age 70

SOCIAL HISTORY:
Smoking: Former smoker, 18 pack-years, quit 3 months ago. Alcohol: Social (1-2 drinks per week). Occupation: Carpenter (self-employed).

REVIEW OF SYSTEMS:
He denies any chest pain, dyspnea, or recent changes in weight.

VITAL SIGNS:
His blood pressure is 132/86 mmHg, heart rate is 78 bpm, and BMI is 32 kg/m^2.

LABORATORY DATA:
Recent laboratory results are as follows:

- Hemoglobin A1c: 8.2%
- Fasting plasma glucose: 172 mg/dL
- Total cholesterol: 184 mg/dL
- HDL cholesterol: 42 mg/dL
- LDL cholesterol: 100 mg/dL
- Serum creatinine: 1.0 mg/dL
- Urine albumin-to-creatinine ratio: 30 mg/g

## Encounter 5: source 2024-01-15, practice 2025-06-24, ed, Emergency Medicine

Chief complaint: Fatigue and muscle weakness that has been worsening over the past two weeks with occasional palpitations.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fatigue and muscle weakness that has been worsening over the past two weeks with occasional palpitations.

HISTORY OF PRESENT ILLNESS:
This 60-year-old male with diabetes and coronary artery disease presents with fatigue and muscle weakness that has been worsening over the past two weeks. He also reports occasional palpitations but denies chest pain, dyspnea, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Poorly controlled diabetes mellitus (diagnosed 2020-07-15)
- Ulnar neuropathy at the elbow (Cubital tunnel syndrome) (diagnosed 2021-07-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2022-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Coronary Artery Disease

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes metformin, insulin glargine, and lisinopril.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64, Hypertension
- Mother: Type 2 Diabetes Mellitus, Dyslipidemia
- Maternal uncle: Stroke at age 70

SOCIAL HISTORY:
Smoking: Former smoker, 18 pack-years, quit 3 months ago. Alcohol: Social (1-2 drinks per week). Occupation: Carpenter (self-employed).

VITAL SIGNS:
Blood pressure is 142/88 mmHg, heart rate is 98 bpm

PHYSICAL EXAMINATION:
Decreased skin turgor

LABORATORY DATA:
- Serum sodium: 138 mEq/L
- Serum potassium: 6.2 mEq/L
- Serum chloride: 105 mEq/L
- Serum bicarbonate: 18 mEq/L
- Blood urea nitrogen: 29 mg/dL
- Serum creatinine: 1.9 mg/dL
- Serum glucose: 198 mg/dL
- Urine pH: 6.0
- Plasma renin activity: Low
- Plasma aldosterone concentration: Low

## Encounter 6: source 2024-07-15, practice 2026-01-01, outpatient, Internal Medicine

Chief complaint: Routine follow-up for long-standing diabetes mellitus with ongoing challenges in glycemic control.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for long-standing diabetes mellitus with ongoing challenges in glycemic control.

HISTORY OF PRESENT ILLNESS:
This patient with coronary artery disease and hypertension returns for diabetes follow-up, diagnosed with diabetes 5 years ago and managing it with metformin and dietary modifications.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Poorly controlled diabetes mellitus (diagnosed 2020-07-15)
- Ulnar neuropathy at the elbow (Cubital tunnel syndrome) (diagnosed 2021-07-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2022-07-15)
- Renal tubular acidosis (diagnosed 2024-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Coronary Artery Disease

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His current medications include metformin and lisinopril.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for coronary artery disease in his father, who had a myocardial infarction at age 55.

SOCIAL HISTORY:
Smoking: Former smoker, 18 pack-years, quit 3 months ago. Alcohol: Social (1-2 drinks per week). Occupation: Carpenter (self-employed).

REVIEW OF SYSTEMS:
He denies any chest pain, shortness of breath, or leg pain on exertion.

VITAL SIGNS:
On physical examination, his blood pressure is 132/84 mmHg, heart rate is 78 bpm, and BMI is 31 kg/m^2.

PHYSICAL EXAMINATION:
His cardiovascular examination is unremarkable, and there are no signs of peripheral arterial disease.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin A1c: 7.2%
- Total cholesterol: 210 mg/dL
- LDL cholesterol: 130 mg/dL
- HDL cholesterol: 40 mg/dL
- Triglycerides: 150 mg/dL

## Encounter 7: source 2025-01-15, practice 2026-07-06, ed, Emergency Medicine

Chief complaint: Abrupt onset of double vision that started this morning.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Abrupt onset of double vision that started this morning.

HISTORY OF PRESENT ILLNESS:
The patient, with longstanding type 2 diabetes and prior emergency department evaluation, is brought to the emergency department because of double vision that started abruptly this morning.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Poorly controlled diabetes mellitus (diagnosed 2020-07-15)
- Ulnar neuropathy at the elbow (Cubital tunnel syndrome) (diagnosed 2021-07-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2022-07-15)
- Renal tubular acidosis (diagnosed 2024-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Coronary Artery Disease

He has a history of poorly controlled hypertension and diabetes mellitus.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64, Hypertension
- Mother: Type 2 Diabetes Mellitus, Dyslipidemia
- Maternal uncle: Stroke at age 70

SOCIAL HISTORY:
Smoking: Former smoker, 18 pack-years, quit 3 months ago. Alcohol: Social (1-2 drinks per week). Occupation: Carpenter (self-employed).

PHYSICAL EXAMINATION:
On examination, his right eye is deviated down and out at rest, and he is unable to adduct or elevate that eye. The right pupil is dilated and poorly reactive to light. He has intact sensation over the face and normal visual acuity. His lacrimal reflex is preserved bilaterally. Neurologic examination is otherwise unremarkable.

ASSESSMENT AND PLAN:
The physician suspects a cranial nerve palsy.
