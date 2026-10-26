# synthetic_hospital patient 2827

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex F, Non-Hispanic Black, Medicare with supplemental commercial plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-10-05 (Mon) | ed | Emergency Medicine |
| 1 | 2020-04-15 | 2020-12-21 (Mon) | outpatient | Nephrology |
| 2 | 2020-07-15 | 2021-03-23 (Tue) | outpatient | Cardiology |
| 3 | 2021-07-15 | 2022-03-27 (Sun) | ed | Emergency Medicine |
| 4 | 2022-07-15 | 2023-03-27 (Mon) | outpatient | Ophthalmology |
| 5 | 2023-07-15 | 2024-04-04 (Thu) | ed | Emergency Medicine |
| 6 | 2024-07-15 | 2025-04-03 (Thu) | ed | Emergency Medicine |
| 7 | 2025-07-15 | 2026-03-27 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental commercial plan",
 "occupation": "Retired administrative assistant",
 "alcohol_use": "Rare, 1-2 glasses of wine monthly",
 "family_history": [
  "Type 2 Diabetes Mellitus (maternal grandmother, mother)",
  "Hypertension (father, brother)",
  "End-stage renal disease (maternal uncle)",
  "Myocardial infarction (father, age 68)"
 ],
 "race_ethnicity": "Non-Hispanic Black",
 "smoking_status": "Former smoker, quit 12 years ago, 20 pack-year history",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Furosemide"
  },
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "18 units subcutaneously at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "500 mg PO every 6 hours as needed for joint pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Open cholecystectomy",
  "Left knee arthroscopy with debridement (10 years prior)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Chronic Kidney Disease Stage 3b",
  "Osteoarthritis",
  "Dyslipidemia",
  "Obesity (Class II)",
  "Gastroesophageal reflux disease with paraesophageal hiatal hernia"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Tricuspid Regurgitation", "Secondary hyperparathyroidism of renal origin", "Herpes Zoster Ophthalmicus", "Iliofemoral Deep Vein Thrombosis", "Deep vein thrombosis", "Superficial surgical wound dehiscence", "Pseudogout (Acute Calcium Pyrophosphate Dihydrate Crystal Deposition Disease)", "Paraesophageal hiatal hernia"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus", "Delayed management of life-threatening respiratory failure"]

## Encounter 0: source 2020-01-15, practice 2020-10-05, ed, Emergency Medicine

Chief complaint: Serosanguinous discharge from surgical wound 10 days after open cholecystectomy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Serosanguinous discharge from surgical wound 10 days after open cholecystectomy

HISTORY OF PRESENT ILLNESS:
She presents to the emergency department 10 days postoperative from an open cholecystectomy

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3b
- Osteoarthritis
- Dyslipidemia
- Obesity (Class II)
- Gastroesophageal reflux disease with paraesophageal hiatal hernia

Her medical history is significant for type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
- Open cholecystectomy
- Left knee arthroscopy with debridement (10 years prior)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Furosemide 40 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 18 units subcutaneously at bedtime
- Atorvastatin 40 mg PO daily
- Acetaminophen 500 mg PO every 6 hours as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (maternal grandmother, mother)
- Hypertension (father, brother)
- End-stage renal disease (maternal uncle)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 20 pack-year history. Alcohol: Rare, 1-2 glasses of wine monthly. Occupation: Retired administrative assistant.

REVIEW OF SYSTEMS:
She denies fever, chills, or abdominal pain.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On examination, the wound edges are separated, revealing a small amount of subcutaneous fat but no visible bowel or fascia. The surrounding skin is mildly erythematous but not warm to the touch. There is no crepitus, and the patient does not exhibit signs of hemodynamic instability. Her abdominal examination is otherwise unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 11,000 /µL
- Hemoglobin: 13.2 g/dL
- Blood glucose: 180 mg/dL
- C-reactive protein: 15 mg/L (normal <10 mg/L)

## Encounter 1: source 2020-04-15, practice 2020-12-21, outpatient, Nephrology

Chief complaint: Follow-up visit after recent hospitalization for uremia symptoms with ongoing fatigue and pruritus

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Okonkwo, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit after recent hospitalization for uremia symptoms with ongoing fatigue and pruritus

HISTORY OF PRESENT ILLNESS:
The patient is a 65-year-old female with CKD Stage 3b returning for follow-up after a recent hospitalization for symptoms of uremia. She reports ongoing fatigue and pruritus.

PAST MEDICAL HISTORY:
Active Problem List:
- Superficial surgical wound dehiscence (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3b
- Osteoarthritis
- Dyslipidemia
- Obesity (Class II)
- Gastroesophageal reflux disease with paraesophageal hiatal hernia

She has a long-standing history of type II diabetes mellitus.

PAST SURGICAL HISTORY:
- Open cholecystectomy
- Left knee arthroscopy with debridement (10 years prior)

MEDICATIONS:
She is currently on multiple antihypertensive medications, a statin, and insulin.

Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Furosemide 40 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 18 units subcutaneously at bedtime
- Atorvastatin 40 mg PO daily
- Acetaminophen 500 mg PO every 6 hours as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (maternal grandmother, mother)
- Hypertension (father, brother)
- End-stage renal disease (maternal uncle)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 20 pack-year history. Alcohol: Rare, 1-2 glasses of wine monthly. Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory studies reveal a serum creatinine of 4.3 mg/dL and a blood urea nitrogen (BUN) level of 67 mg/dL.

## Encounter 2: source 2020-07-15, practice 2021-03-23, outpatient, Cardiology

Chief complaint: Progressive fatigue, lower extremity swelling, and exertional dyspnea over several months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, lower extremity swelling, and exertional dyspnea over several months

HISTORY OF PRESENT ILLNESS:
This patient with CKD and recent nephrology follow-up presents with dyspnea. She has experienced increasing dyspnea with minimal exertion over the past few months and now has difficulty climbing a flight of stairs.

PAST MEDICAL HISTORY:
Active Problem List:
- Superficial surgical wound dehiscence (diagnosed 2020-01-15)
- Secondary hyperparathyroidism of renal origin (diagnosed 2020-04-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3b
- Osteoarthritis
- Dyslipidemia
- Obesity (Class II)
- Gastroesophageal reflux disease with paraesophageal hiatal hernia

PAST SURGICAL HISTORY:
- Open cholecystectomy
- Left knee arthroscopy with debridement (10 years prior)

MEDICATIONS:
Medications include lisinopril, furosemide, and metformin.

Home Medications:
- Amlodipine 5 mg PO daily
- Insulin glargine 18 units subcutaneously at bedtime
- Atorvastatin 40 mg PO daily
- Acetaminophen 500 mg PO every 6 hours as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (maternal grandmother, mother)
- Hypertension (father, brother)
- End-stage renal disease (maternal uncle)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 20 pack-year history. Alcohol: Rare, 1-2 glasses of wine monthly. Occupation: Retired administrative assistant.

VITAL SIGNS:
On examination, her temperature is 37.0°C (98.6°F), blood pressure is 130/80 mm Hg, pulse is 88/min, and respirations are 18/min.

PHYSICAL EXAMINATION:
Jugular venous distention is present to the angle of the mandible. There is a pulsatile liver and 2+ bilateral pitting edema. Cardiac auscultation reveals a murmur.

OTHER STUDIES:
ECG shows right axis deviation and right bundle branch block.

## Encounter 3: source 2021-07-15, practice 2022-03-27, ed, Emergency Medicine

Chief complaint: Sudden onset severe pain, redness, and swelling in left knee over 2 days

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset severe pain, redness, and swelling in left knee over 2 days

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old female with a history of Osteoarthritis, Type 2 Diabetes Mellitus, and Chronic Kidney Disease Stage 3b, with prior Nephrology and Cardiology evaluations over the past year, who presents to the emergency department with a 2-day history of severe pain, redness, and swelling in her left knee. She reports that the pain began suddenly and has progressively worsened. She denies any recent trauma or injury to the knee.

PAST MEDICAL HISTORY:
Active Problem List:
- Superficial surgical wound dehiscence (diagnosed 2020-01-15)
- Secondary hyperparathyroidism of renal origin (diagnosed 2020-04-15)
- Tricuspid Regurgitation (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3b
- Osteoarthritis
- Dyslipidemia
- Obesity (Class II)
- Gastroesophageal reflux disease with paraesophageal hiatal hernia

Her medical history is significant for hypertension, type 2 diabetes mellitus, and osteoarthritis.

PAST SURGICAL HISTORY:
- Open cholecystectomy
- Left knee arthroscopy with debridement (10 years prior)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Furosemide 40 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 18 units subcutaneously at bedtime
- Atorvastatin 40 mg PO daily
- Acetaminophen 500 mg PO every 6 hours as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (maternal grandmother, mother)
- Hypertension (father, brother)
- End-stage renal disease (maternal uncle)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 20 pack-year history. Alcohol: Rare, 1-2 glasses of wine monthly. Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, her left knee is warm, erythematous, and swollen with decreased range of motion due to pain.

LABORATORY DATA:
Aspiration of the knee joint yields a yellow, cloudy fluid. Synovial fluid analysis reveals:

- WBC count: 18,000/mm³ (reference range: <200/mm³)
- Neutrophils: 75% (reference range: <25%)
- Crystals: Rhomboid-shaped, weakly positively birefringent

IMAGING:
Plain radiography of the knee shows chondrocalcinosis.

## Encounter 4: source 2022-07-15, practice 2023-03-27, outpatient, Ophthalmology

Chief complaint: Painful vesicular rash on right forehead and periorbital area with preceding burning pain

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Johnson, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful vesicular rash on right forehead and periorbital area with preceding burning pain

HISTORY OF PRESENT ILLNESS:
This 67-year-old female with type 2 diabetes mellitus, chronic kidney disease stage 3b, and history of tricuspid regurgitation and pseudogout presents with facial pain and rash. She reports that the pain began as a burning sensation 5 days ago before the rash appeared. The patient denies any recent illness or trauma. She has no history of similar episodes in the past.

PAST MEDICAL HISTORY:
Active Problem List:
- Superficial surgical wound dehiscence (diagnosed 2020-01-15)
- Secondary hyperparathyroidism of renal origin (diagnosed 2020-04-15)
- Tricuspid Regurgitation (diagnosed 2020-07-15)
- Pseudogout (Acute Calcium Pyrophosphate Dihydrate Crystal Deposition Disease) (diagnosed 2021-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3b
- Osteoarthritis
- Dyslipidemia
- Obesity (Class II)
- Gastroesophageal reflux disease with paraesophageal hiatal hernia

Her medical history is significant for type 2 diabetes mellitus and hypertension, which are well-controlled.

PAST SURGICAL HISTORY:
- Open cholecystectomy
- Left knee arthroscopy with debridement (10 years prior)

MEDICATIONS:
She takes with metformin and lisinopril, respectively.

Home Medications:
- Amlodipine 5 mg PO daily
- Furosemide 40 mg PO daily
- Insulin glargine 18 units subcutaneously at bedtime
- Atorvastatin 40 mg PO daily
- Acetaminophen 500 mg PO every 6 hours as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (maternal grandmother, mother)
- Hypertension (father, brother)
- End-stage renal disease (maternal uncle)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 20 pack-year history. Alcohol: Rare, 1-2 glasses of wine monthly. Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, you note a vesicular rash in the distribution of the right ophthalmic branch of the trigeminal nerve. Fluorescein staining of the cornea reveals dendriform ulcers. Her visual acuity is decreased in the right eye.

## Encounter 5: source 2023-07-15, practice 2024-04-04, ed, Emergency Medicine

Chief complaint: Acute left calf swelling and throbbing pain after 12-hour international flight

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute left calf swelling and throbbing pain after 12-hour international flight

HISTORY OF PRESENT ILLNESS:
This 68-year-old female with chronic kidney disease stage 3b, obesity class II, and prior lower extremity swelling presents with acute left calf complaints. She describes the pain as throbbing, worsened with walking, and accompanied by tightness in the lower leg. She has no history of recent trauma but reports that she recently returned from a 12-hour international flight two days ago. She denies chest pain, shortness of breath, or hemoptysis.

PAST MEDICAL HISTORY:
Active Problem List:
- Superficial surgical wound dehiscence (diagnosed 2020-01-15)
- Secondary hyperparathyroidism of renal origin (diagnosed 2020-04-15)
- Tricuspid Regurgitation (diagnosed 2020-07-15)
- Pseudogout (Acute Calcium Pyrophosphate Dihydrate Crystal Deposition Disease) (diagnosed 2021-07-15)
- Herpes Zoster Ophthalmicus (diagnosed 2022-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3b
- Osteoarthritis
- Dyslipidemia
- Obesity (Class II)
- Gastroesophageal reflux disease with paraesophageal hiatal hernia

She has a history of hypertension and type 2 diabetes mellitus, both of which are well controlled on medication.

PAST SURGICAL HISTORY:
- Open cholecystectomy
- Left knee arthroscopy with debridement (10 years prior)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Furosemide 40 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 18 units subcutaneously at bedtime
- Atorvastatin 40 mg PO daily
- Acetaminophen 500 mg PO every 6 hours as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (maternal grandmother, mother)
- Hypertension (father, brother)
- End-stage renal disease (maternal uncle)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 20 pack-year history. Alcohol: Rare, 1-2 glasses of wine monthly. Occupation: Retired administrative assistant.

VITAL SIGNS:
Vital signs are stable

PHYSICAL EXAMINATION:
Physical examination reveals unilateral left leg swelling with pitting edema extending up to the knee. The left calf is tender to palpation, and there is a positive Homan sign. The remainder of the physical exam is unremarkable.

LABORATORY DATA:
Laboratory studies, including baseline coagulation parameters, are within normal limits.

IMAGING:
Doppler ultrasound of the left lower extremity demonstrates non-compressibility of the proximal femoral vein consistent with acute deep vein thrombosis (DVT).

PLAN:
Given her risk of progression to pulmonary embolism, the medical team initiates treatment with fondaparinux. The patient tolerates the therapy well without evidence of bleeding, and her leg pain improves with continued treatment and compression stockings.

## Encounter 6: source 2024-07-15, practice 2025-04-03, ed, Emergency Medicine

Chief complaint: Sudden severe chest pain and shortness of breath with positional worsening

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden severe chest pain and shortness of breath with positional worsening

HISTORY OF PRESENT ILLNESS:
This 69-year-old female with a history of gastroesophageal reflux disease with paraesophageal hiatal hernia and prior deep vein thrombosis presents to the emergency department with a 4-hour history of sudden onset severe chest pain and shortness of breath. She describes the pain as sharp and worse when lying down, with some relief upon sitting up. She also reports a history of intermittent dysphagia and postprandial fullness for the past year, which she attributed to her gastroesophageal reflux disease.

PAST MEDICAL HISTORY:
Active Problem List:
- Superficial surgical wound dehiscence (diagnosed 2020-01-15)
- Secondary hyperparathyroidism of renal origin (diagnosed 2020-04-15)
- Tricuspid Regurgitation (diagnosed 2020-07-15)
- Pseudogout (Acute Calcium Pyrophosphate Dihydrate Crystal Deposition Disease) (diagnosed 2021-07-15)
- Herpes Zoster Ophthalmicus (diagnosed 2022-07-15)
- Iliofemoral Deep Vein Thrombosis (diagnosed 2023-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3b
- Osteoarthritis
- Dyslipidemia
- Obesity (Class II)
- Gastroesophageal reflux disease with paraesophageal hiatal hernia

Her past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Open cholecystectomy
- Left knee arthroscopy with debridement (10 years prior)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Furosemide 40 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 18 units subcutaneously at bedtime
- Atorvastatin 40 mg PO daily
- Acetaminophen 500 mg PO every 6 hours as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (maternal grandmother, mother)
- Hypertension (father, brother)
- End-stage renal disease (maternal uncle)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 20 pack-year history. Alcohol: Rare, 1-2 glasses of wine monthly. Occupation: Retired administrative assistant.

VITAL SIGNS:
On examination, her blood pressure is 150/90 mmHg, heart rate is 98 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Her abdomen is soft and non-tender, but she has decreased breath sounds at the left lung base.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 9,000/mm^3
- Platelets: 250,000/mm^3
- Serum glucose: 180 mg/dL
- Serum creatinine: 1.0 mg/dL
- Alanine aminotransferase (ALT): 30 U/L
- Aspartate aminotransferase (AST): 28 U/L

IMAGING:
A chest X-ray reveals a retrocardiac air-fluid level.

## Encounter 7: source 2025-07-15, practice 2026-03-27, ed, Emergency Medicine

Chief complaint: Recurrent acute swelling and pain in left calf with throbbing sensation

ENCOUNTER NOTE
Date: 2025-07-15
Provider: Dr. Robert Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent acute swelling and pain in left calf with throbbing sensation

HISTORY OF PRESENT ILLNESS:
This 70-year-old female with a history of Type 2 Diabetes Mellitus, Chronic Kidney Disease Stage 3b, Obesity Class II, and prior iliofemoral deep vein thrombosis presents to the emergency department with acute swelling and pain in her left calf that began suddenly earlier this morning. She describes the pain as throbbing, worsened with walking, and accompanied by tightness in the lower leg. She has no history of recent trauma but reports that she recently returned from a 12-hour international flight two days ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Superficial surgical wound dehiscence (diagnosed 2020-01-15)
- Secondary hyperparathyroidism of renal origin (diagnosed 2020-04-15)
- Tricuspid Regurgitation (diagnosed 2020-07-15)
- Pseudogout (Acute Calcium Pyrophosphate Dihydrate Crystal Deposition Disease) (diagnosed 2021-07-15)
- Herpes Zoster Ophthalmicus (diagnosed 2022-07-15)
- Iliofemoral Deep Vein Thrombosis (diagnosed 2023-07-15)
- Paraesophageal hiatal hernia (diagnosed 2024-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3b
- Osteoarthritis
- Dyslipidemia
- Obesity (Class II)
- Gastroesophageal reflux disease with paraesophageal hiatal hernia

She has a history of hypertension and type 2 diabetes mellitus, both of which are well controlled on medication.

PAST SURGICAL HISTORY:
- Open cholecystectomy
- Left knee arthroscopy with debridement (10 years prior)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Furosemide 40 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 18 units subcutaneously at bedtime
- Atorvastatin 40 mg PO daily
- Acetaminophen 500 mg PO every 6 hours as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (maternal grandmother, mother)
- Hypertension (father, brother)
- End-stage renal disease (maternal uncle)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 20 pack-year history. Alcohol: Rare, 1-2 glasses of wine monthly. Occupation: Retired administrative assistant.

REVIEW OF SYSTEMS:
She denies chest pain, shortness of breath, or hemoptysis.

VITAL SIGNS:
Vital signs are stable

PHYSICAL EXAMINATION:
Physical examination reveals unilateral left leg swelling with pitting edema extending up to the knee. The left calf is tender to palpation, and there is a positive Homan sign.
and the remainder of the physical exam is unremarkable.

LABORATORY DATA:
Laboratory studies, including baseline coagulation parameters, are within normal limits.

IMAGING:
Doppler ultrasound of the left lower extremity demonstrates non-compressibility of the proximal femoral vein consistent with acute deep vein thrombosis (DVT).

PLAN:
Given her risk of progression to pulmonary embolism, the medical team initiates treatment with fondaparinux. The patient tolerates the therapy well without evidence of bleeding, and her leg pain improves with continued treatment and compression stockings.
