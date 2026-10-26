# synthetic_hospital patient 2223

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-10-29 (Thu) | outpatient | Neurology |
| 1 | 2020-04-15 | 2021-01-18 (Mon) | outpatient | Cardiology |
| 2 | 2020-10-15 | 2021-08-02 (Mon) | outpatient | Gastroenterology |
| 3 | 2021-04-15 | 2022-01-29 (Sat) | ed | Emergency Medicine |
| 4 | 2022-01-15 | 2022-11-01 (Tue) | outpatient | Rheumatology |
| 5 | 2023-01-15 | 2023-10-28 (Sat) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2024-10-26 (Sat) | inpatient | Cardiology |
| 7 | 2025-01-15 | 2025-11-04 (Tue) | icu | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Hydrochlorothiazide (angioedema and electrolyte abnormalities)"
 ],
 "insurance": "Medicare",
 "occupation": "Retired accountant",
 "alcohol_use": "Abstinent (formerly occasional)",
 "family_history": [
  "Father: Myocardial infarction at age 62, hypertension",
  "Mother: Type 2 diabetes mellitus, stroke at age 70",
  "No family history of amyloidosis or autoimmune disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
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
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Furosemide"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention (drug-eluting stent) to LAD",
  "Diagnostic cardiac catheterization"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Coronary Artery Disease",
  "Chronic Heart Failure with reduced ejection fraction",
  "Chronic Kidney Disease Stage 3b",
  "Systemic AL Amyloidosis",
  "History of ischemic stroke with mild residual left-sided weakness",
  "Thoracic spinal stenosis"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Acute decompensated heart failure", "Hepatic dysfunction", "AL amyloidosis", "Polymyositis", "Thoracic spinal stenosis", "Hydrochlorothiazide adverse reaction", "Oropharyngeal dysphagia"]
Comorbidities: ["Acute decompensated heart failure", "Essential hypertension", "Type 2 diabetes mellitus", "Coronary Artery Disease (Antiplatelet Therapy)"]

## Encounter 0: source 2020-01-15, practice 2020-10-29, outpatient, Neurology

Chief complaint: Lower back pain radiating down posterior thighs for 6 months, worse with walking and standing

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Lower back pain radiating down posterior thighs for 6 months, worse with walking and standing

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of lower back pain that radiates down the posterior aspect of both thighs. He reports that the pain is exacerbated by walking and standing for prolonged periods. He finds some relief when he sits or leans forward while shopping with a cart.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic Heart Failure with reduced ejection fraction
- Chronic Kidney Disease Stage 3b
- Systemic AL Amyloidosis
- History of ischemic stroke with mild residual left-sided weakness
- Thoracic spinal stenosis

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (drug-eluting stent) to LAD
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Metformin 1000 mg PO twice daily
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Hydrochlorothiazide (angioedema and electrolyte abnormalities)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, stroke at age 70
- No family history of amyloidosis or autoimmune disease

SOCIAL HISTORY:
Smoking: Never. Alcohol: Abstinent (formerly occasional). Occupation: Retired accountant.

PHYSICAL EXAMINATION:
On physical examination, there is no loss of lower extremity strength, and peripheral pulses are intact. Straight leg raise test is negative bilaterally.

IMAGING:
An MRI of the lumbar spine reveals narrowing of the spinal canal with associated degenerative changes in the facet joints and hypertrophy of the ligamentum flavum.

## Encounter 1: source 2020-04-15, practice 2021-01-18, outpatient, Cardiology

Chief complaint: Increasing fatigue, dyspnea on exertion, and bilateral lower extremity swelling over two months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue, dyspnea on exertion, and bilateral lower extremity swelling over two months

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with systemic AL amyloidosis and chronic heart failure presents three months after Neurology evaluation for thoracic spinal stenosis. He reports increasing fatigue, dyspnea on exertion, and bilateral lower extremity swelling over two months. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Thoracic spinal stenosis (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic Heart Failure with reduced ejection fraction
- Chronic Kidney Disease Stage 3b
- Systemic AL Amyloidosis
- History of ischemic stroke with mild residual left-sided weakness

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (drug-eluting stent) to LAD
- Diagnostic cardiac catheterization

MEDICATIONS:
His medications include lisinopril, amlodipine, and furosemide.

Home Medications:
- Metformin 1000 mg PO twice daily
- Aspirin 81 mg PO daily

ALLERGIES:
Hydrochlorothiazide (angioedema and electrolyte abnormalities)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, stroke at age 70
- No family history of amyloidosis or autoimmune disease

SOCIAL HISTORY:
Smoking: Never. Alcohol: Abstinent (formerly occasional). Occupation: Retired accountant.

VITAL SIGNS:
On physical examination, his blood pressure is 150/90 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a low-pitched diastolic rumble at the apex. Jugular venous pressure is elevated. There is 2+ pitting edema up to the mid-shins bilaterally.

LABORATORY DATA:
Laboratory findings are as follows:
- Serum creatinine: 2.1 mg/dL (elevated)
- Blood urea nitrogen (BUN): 45 mg/dL (elevated)
- Serum albumin: 3.2 g/dL (low)
- Total protein: 8.5 g/dL (high)
- Urinalysis: Proteinuria, no hematuria or casts

IMAGING:
An echocardiogram shows increased ventricular wall thickness with normal chamber size and impaired diastolic filling without significant valvular abnormalities.

## Encounter 2: source 2020-10-15, practice 2021-08-02, outpatient, Gastroenterology

Chief complaint: Difficulty swallowing with coughing and choking, especially with liquids, over six months

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Emily Johnson, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty swallowing with coughing and choking, especially with liquids, over six months

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with systemic AL amyloidosis and thoracic spinal stenosis, recently evaluated by Cardiology and Neurology, who presents to Gastroenterology with a 6-month history of difficulty swallowing, which he describes as a problem initiating swallows. He reports frequent coughing and choking when eating, especially with liquids. He also mentions occasional nasal regurgitation.

PAST MEDICAL HISTORY:
Active Problem List:
- Thoracic spinal stenosis (diagnosed 2020-01-15)
- AL amyloidosis (diagnosed 2020-04-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic Heart Failure with reduced ejection fraction
- Chronic Kidney Disease Stage 3b
- Systemic AL Amyloidosis
- History of ischemic stroke with mild residual left-sided weakness

His past medical history includes poorly controlled diabetes mellitus type 2, hypertension, and a recent stroke with mild residual left-sided weakness.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (drug-eluting stent) to LAD
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Metformin 1000 mg PO twice daily
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Hydrochlorothiazide (angioedema and electrolyte abnormalities)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, stroke at age 70
- No family history of amyloidosis or autoimmune disease

SOCIAL HISTORY:
Smoking: Never. Alcohol: Abstinent (formerly occasional). Occupation: Retired accountant.

PHYSICAL EXAMINATION:
On physical examination, there is a slight slurring of speech and reduced gag reflex.

IMAGING:
A videofluoroscopic modified barium swallow study shows delayed swallow initiation, pooling of contrast in the valleculae, and penetration of contrast into the laryngeal vestibule without aspiration.

## Encounter 3: source 2021-04-15, practice 2022-01-29, ed, Emergency Medicine

Chief complaint: Progressive dyspnea, orthopnea, and lower extremity swelling for one week with concern for medication reaction

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive dyspnea, orthopnea, and lower extremity swelling for one week with concern for medication reaction

PAST MEDICAL HISTORY:
Active Problem List:
- Thoracic spinal stenosis (diagnosed 2020-01-15)
- AL amyloidosis (diagnosed 2020-04-15)
- Oropharyngeal dysphagia (diagnosed 2020-10-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic Heart Failure with reduced ejection fraction
- Chronic Kidney Disease Stage 3b
- Systemic AL Amyloidosis
- History of ischemic stroke with mild residual left-sided weakness

His medical history includes hypertension and coronary artery disease.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (drug-eluting stent) to LAD
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Metformin 1000 mg PO twice daily
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Hydrochlorothiazide (angioedema and electrolyte abnormalities)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, stroke at age 70
- No family history of amyloidosis or autoimmune disease

SOCIAL HISTORY:
Smoking: Never. Alcohol: Abstinent (formerly occasional). Occupation: Retired accountant.

VITAL SIGNS:
Temperature is 37.0°C (98.6°F), blood pressure is 135/78 mm Hg, pulse is 88/min, and respiratory rate is 20/min

PHYSICAL EXAMINATION:
Jugular venous distention is noted. Cardiac auscultation reveals an S3 gallop. Lung examination shows bilateral crackles. There is 2+ pitting edema in the lower extremities.

IMAGING:
A chest X-ray shows pulmonary edema.

PLAN:
He is started on a combination of medications for heart failure, including a diuretic that inhibits the Na⁺-K⁺-2Cl⁻ cotransporter in the thick ascending limb of the loop of Henle.

## Encounter 4: source 2022-01-15, practice 2022-11-01, outpatient, Rheumatology

Chief complaint: Two-month history of difficulty rising from chairs and climbing stairs with progressive fatigue

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Lisa Patel, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of difficulty rising from chairs and climbing stairs with progressive fatigue

HISTORY OF PRESENT ILLNESS:
The patient with AL amyloidosis and prior dyspnea evaluations presents with two-month difficulty rising from chairs and climbing stairs. He denies recent trauma but reports increased fatigue and occasional exertional dyspnea.

PAST MEDICAL HISTORY:
Active Problem List:
- Thoracic spinal stenosis (diagnosed 2020-01-15)
- AL amyloidosis (diagnosed 2020-04-15)
- Oropharyngeal dysphagia (diagnosed 2020-10-15)
- Hydrochlorothiazide adverse reaction (diagnosed 2021-04-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic Heart Failure with reduced ejection fraction
- Chronic Kidney Disease Stage 3b
- Systemic AL Amyloidosis
- History of ischemic stroke with mild residual left-sided weakness

Medical history is significant for type 2 diabetes mellitus and hypertension, both well controlled.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (drug-eluting stent) to LAD
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Metformin 1000 mg PO twice daily
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Hydrochlorothiazide (angioedema and electrolyte abnormalities)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, stroke at age 70
- No family history of amyloidosis or autoimmune disease

SOCIAL HISTORY:
He does not use tobacco, alcohol, or illicit drugs.

PHYSICAL EXAMINATION:
Physical examination shows symmetric weakness in the deltoid, biceps, and quadriceps muscles bilaterally. There is no rash, sensory loss, or joint swelling. Deep tendon reflexes are preserved.

LABORATORY DATA:
Laboratory results show a creatine kinase level of 3,200 U/L (normal: 22–198), and erythrocyte sedimentation rate is 72 mm/hr.

## Encounter 5: source 2023-01-15, practice 2023-10-28, ed, Emergency Medicine

Chief complaint: Two-day history of progressive shortness of breath and bilateral ankle swelling with recent severe chest pain

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of progressive shortness of breath and bilateral ankle swelling with recent severe chest pain

HISTORY OF PRESENT ILLNESS:
Patient with systemic AL amyloidosis and chronic heart failure returns to the emergency department. He reports a recent episode of severe chest pain that resolved spontaneously.

PAST MEDICAL HISTORY:
Active Problem List:
- Thoracic spinal stenosis (diagnosed 2020-01-15)
- AL amyloidosis (diagnosed 2020-04-15)
- Oropharyngeal dysphagia (diagnosed 2020-10-15)
- Hydrochlorothiazide adverse reaction (diagnosed 2021-04-15)
- Polymyositis (diagnosed 2022-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic Heart Failure with reduced ejection fraction
- Chronic Kidney Disease Stage 3b
- Systemic AL Amyloidosis
- History of ischemic stroke with mild residual left-sided weakness

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (drug-eluting stent) to LAD
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Metformin 1000 mg PO twice daily
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Hydrochlorothiazide (angioedema and electrolyte abnormalities)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, stroke at age 70
- No family history of amyloidosis or autoimmune disease

SOCIAL HISTORY:
Smoking: Never. Alcohol: Abstinent (formerly occasional). Occupation: Retired accountant.

VITAL SIGNS:
On examination, his blood pressure is 90/60 mmHg, heart rate is 110 bpm, respiratory rate is 22 breaths/min, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
He has jugular venous distension, crackles at the lung bases, and 2+ pitting edema in both ankles.

LABORATORY DATA:
Laboratory results are as follows:

- Aspartate aminotransferase (AST): 2500 U/L
- Alanine aminotransferase (ALT): 2300 U/L
- Total bilirubin: 1.2 mg/dL
- Alkaline phosphatase (ALP): 100 U/L
- International normalized ratio (INR): 1.1

OTHER STUDIES:
An ECG shows ST-segment depressions in the anterior leads.

## Encounter 6: source 2024-01-15, practice 2024-10-26, inpatient, Cardiology

Chief complaint: Three-month history of progressive exertional dyspnea and bilateral ankle swelling

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Washington, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Three-month history of progressive exertional dyspnea and bilateral ankle swelling

HISTORY OF PRESENT ILLNESS:
The patient with systemic AL amyloidosis and chronic heart failure with reduced ejection fraction returns for evaluation of recurrent symptoms. He presents with a 3-month history of progressive exertional dyspnea and bilateral ankle swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Thoracic spinal stenosis (diagnosed 2020-01-15)
- AL amyloidosis (diagnosed 2020-04-15)
- Oropharyngeal dysphagia (diagnosed 2020-10-15)
- Hydrochlorothiazide adverse reaction (diagnosed 2021-04-15)
- Polymyositis (diagnosed 2022-01-15)
- Hepatic dysfunction (diagnosed 2023-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic Heart Failure with reduced ejection fraction
- Chronic Kidney Disease Stage 3b
- Systemic AL Amyloidosis
- History of ischemic stroke with mild residual left-sided weakness

He has a past medical history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (drug-eluting stent) to LAD
- Diagnostic cardiac catheterization

MEDICATIONS:
His medications include metformin, lisinopril, and hydrochlorothiazide.

Home Medications:
- Amlodipine 5 mg PO daily
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Hydrochlorothiazide (angioedema and electrolyte abnormalities)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, stroke at age 70
- No family history of amyloidosis or autoimmune disease

SOCIAL HISTORY:
He does not smoke and consumes alcohol occasionally.

VITAL SIGNS:
On physical examination, his blood pressure is 145/90 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
He has jugular venous distension, crackles at the lung bases, and 2+ pitting edema in both ankles. His heart sounds include a third heart sound (S3).

LABORATORY DATA:
Laboratory values are as follows:

- Serum sodium: 138 mEq/L
- Serum potassium: 4.5 mEq/L
- Serum creatinine: 1.2 mg/dL
- B-type natriuretic peptide (BNP): 900 pg/mL
- Fasting blood glucose: 160 mg/dL
- Hemoglobin A1c: 7.2%

IMAGING:
Chest X-ray shows cardiomegaly and pulmonary congestion.

OTHER STUDIES:
Electrocardiogram (ECG) reveals left ventricular hypertrophy.

## Encounter 7: source 2025-01-15, practice 2025-11-04, icu, Cardiology

Chief complaint: Acute increasing shortness of breath over two days with orthopnea requiring three pillows

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Garcia, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Acute increasing shortness of breath over two days with orthopnea requiring three pillows

HISTORY OF PRESENT ILLNESS:
This 73-year-old with AL amyloidosis and heart failure returns with symptoms similar to one year prior. He reports needing 3 pillows to sleep comfortably and has had difficulty walking more than a block due to fatigue and breathlessness. He has not had chest pain, fever, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Thoracic spinal stenosis (diagnosed 2020-01-15)
- AL amyloidosis (diagnosed 2020-04-15)
- Oropharyngeal dysphagia (diagnosed 2020-10-15)
- Hydrochlorothiazide adverse reaction (diagnosed 2021-04-15)
- Polymyositis (diagnosed 2022-01-15)
- Hepatic dysfunction (diagnosed 2023-01-15)
- Acute decompensated heart failure (diagnosed 2024-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Chronic Heart Failure with reduced ejection fraction
- Chronic Kidney Disease Stage 3b
- Systemic AL Amyloidosis
- History of ischemic stroke with mild residual left-sided weakness

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (drug-eluting stent) to LAD
- Diagnostic cardiac catheterization

MEDICATIONS:
He takes lisinopril, amlodipine, and metformin.

Home Medications:
- Furosemide 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Hydrochlorothiazide (angioedema and electrolyte abnormalities)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, stroke at age 70
- No family history of amyloidosis or autoimmune disease

SOCIAL HISTORY:
Smoking: Never. Alcohol: Abstinent (formerly occasional). Occupation: Retired accountant.

VITAL SIGNS:
Temperature is 36.8°C (98.2°F), blood pressure is 155/95 mm Hg, pulse is 95/min, and respirations are 22/min. Pulse oximetry shows 93% on room air.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a normal S1 and S2 with a third heart sound (S3). Bibasilar crackles are heard on lung auscultation, and there is jugular venous distension but no peripheral edema. Fundoscopy reveals arteriolar narrowing without papilledema.
