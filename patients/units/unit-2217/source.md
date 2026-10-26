# synthetic_hospital patient 2217

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic Black, Medicare and Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-06-19 (Mon) | outpatient | Nephrology |
| 1 | 2020-04-15 | 2023-10-02 (Mon) | outpatient | Pulmonology |
| 2 | 2020-09-15 | 2024-02-19 (Mon) | outpatient | Cardiology |
| 3 | 2021-03-15 | 2024-08-21 (Wed) | ed | Emergency Medicine |
| 4 | 2021-09-15 | 2025-02-21 (Fri) | ed | Emergency Medicine |
| 5 | 2022-03-15 | 2025-08-25 (Mon) | ed | Emergency Medicine |
| 6 | 2022-05-15 | 2025-11-02 (Sun) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-06-29 (Mon) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare and Medicaid",
 "occupation": "Retired warehouse supervisor, currently on disability",
 "alcohol_use": "Former heavy alcohol use (6-8 beers daily), currently abstinent and denies current use",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Stroke at age 70",
  "Sibling: Type 2 Diabetes Mellitus",
  "Family history of chronic kidney disease"
 ],
 "race_ethnicity": "Non-Hispanic Black",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "24 units subcutaneously at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "10 mg orally daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg orally daily",
   "name": "Amlodipine"
  },
  {
   "dose": "1000 mg orally twice daily (noted to be discontinued upon ESRD diagnosis)",
   "name": "Metformin"
  },
  {
   "dose": "81 mg orally daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg orally daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "40 mg orally daily",
   "name": "Furosemide"
  },
  {
   "dose": "800 mg orally three times daily with meals",
   "name": "Sevelamer carbonate"
  },
  {
   "dose": "0.25 mcg orally daily",
   "name": "Calcitriol"
  },
  {
   "dose": "5 mg orally twice daily (for history of DVT)",
   "name": "Apixaban"
  }
 ],
 "surgical_history": [
  "Cardiac catheterization with drug-eluting stent placement (prior to encounters)",
  "Left radiocephalic arteriovenous fistula creation for hemodialysis access"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (15-year duration)",
  "Hypertension (20-year history, poorly controlled)",
  "Chronic Kidney Disease Stage 4 progressing to End-Stage Renal Disease on hemodialysis",
  "Secondary Hyperparathyroidism",
  "Heart Failure with Reduced Ejection Fraction",
  "History of Myocardial Infarction (remote)",
  "History of Deep Vein Thrombosis (left popliteal vein)",
  "Obesity Hypoventilation Syndrome",
  "Class III Obesity"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["End-stage renal disease on hemodialysis", "Secondary hyperparathyroidism", "Deep vein thrombosis of left popliteal vein", "Hyperkalemia", "Heart failure with reduced ejection fraction", "Acute myocardial infarction", "High-output heart failure due to arteriovenous fistula", "Obesity hypoventilation syndrome"]
Comorbidities: ["Medication noncompliance", "Morbid obesity", "Type 2 diabetes mellitus", "Chronic kidney disease stage 4", "Bilateral ankle edema", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2023-06-19, outpatient, Nephrology

Chief complaint: Progressive fatigue, bone pain, and generalized weakness over several months with difficulty walking up stairs due to thigh discomfort.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, bone pain, and generalized weakness over several months with difficulty walking up stairs due to thigh discomfort.

HISTORY OF PRESENT ILLNESS:
He also reports difficulty walking up stairs due to thigh discomfort. He has not attended routine nephrology follow-up appointments for the past year due to relocation.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension (20-year history, poorly controlled)
- Chronic Kidney Disease Stage 4 progressing to End-Stage Renal Disease on hemodialysis
- Secondary Hyperparathyroidism
- Heart Failure with Reduced Ejection Fraction
- History of Myocardial Infarction (remote)
- History of Deep Vein Thrombosis (left popliteal vein)
- Obesity Hypoventilation Syndrome
- Class III Obesity

Past medical history includes type 2 diabetes mellitus and hypertension diagnosed 15 years ago.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (prior to encounters)
- Left radiocephalic arteriovenous fistula creation for hemodialysis access

MEDICATIONS:
Medications include insulin glargine, lisinopril, and amlodipine.

Home Medications:
- Metformin 1000 mg orally twice daily (noted to be discontinued upon ESRD diagnosis)
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Furosemide 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily
- Apixaban 5 mg orally twice daily (for history of DVT)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Stroke at age 70
- Sibling: Type 2 Diabetes Mellitus
- Family history of chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy alcohol use (6-8 beers daily), currently abstinent and denies current use. Occupation: Retired warehouse supervisor, currently on disability.

VITAL SIGNS:
On examination, temperature is 36.8°C (98.2°F), pulse is 74/min, and blood pressure is 146/84 mmHg. BMI is 31 kg/m².

PHYSICAL EXAMINATION:
Cardiopulmonary examination is unremarkable. Musculoskeletal examination reveals diffuse bony tenderness over both femurs and proximal muscle weakness in the thighs bilaterally.

LABORATORY DATA:
Recent labs show elevated serum creatinine consistent with chronic kidney disease stage 4.

## Encounter 1: source 2020-04-15, practice 2023-10-02, outpatient, Pulmonology

Chief complaint: Daytime fatigue, morning headaches, and frequent nighttime awakenings to urinate over 9 months; wife reports loud snoring and apneic episodes.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Daytime fatigue, morning headaches, and frequent nighttime awakenings to urinate over 9 months; wife reports loud snoring and apneic episodes.

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old male with End-Stage Renal Disease on hemodialysis and secondary hyperparathyroidism, seen in Nephrology three months ago for bone pain and generalized weakness, who presents with a 9-month history of daytime fatigue, morning headaches, and frequent nighttime awakenings to urinate. He reports waking up 4 to 5 times every night to void and often feels unrefreshed despite a full night's sleep. His wife mentions that he snores loudly and sometimes stops breathing during sleep. Over the past year, he has gained 16 kg (35 lb) without any changes in diet.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension (20-year history, poorly controlled)
- Chronic Kidney Disease Stage 4 progressing to End-Stage Renal Disease on hemodialysis
- Heart Failure with Reduced Ejection Fraction
- History of Myocardial Infarction (remote)
- History of Deep Vein Thrombosis (left popliteal vein)
- Obesity Hypoventilation Syndrome
- Class III Obesity

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (prior to encounters)
- Left radiocephalic arteriovenous fistula creation for hemodialysis access

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Lisinopril 10 mg orally daily
- Amlodipine 5 mg orally daily
- Metformin 1000 mg orally twice daily (noted to be discontinued upon ESRD diagnosis)
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Furosemide 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily
- Apixaban 5 mg orally twice daily (for history of DVT)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Stroke at age 70
- Sibling: Type 2 Diabetes Mellitus
- Family history of chronic kidney disease

SOCIAL HISTORY:
He has no history of smoking or alcohol use.

VITAL SIGNS:
Vitals reveal a blood pressure of 158/92 mm Hg, heart rate of 94/min, respiratory rate of 20/min, and BMI of 41 kg/m². Pulse oximetry on room air is 91%.

PHYSICAL EXAMINATION:
Physical examination shows mild pedal edema and no thyroid enlargement.

OTHER STUDIES:
ECG reveals right axis deviation and signs of right ventricular hypertrophy.

## Encounter 2: source 2020-09-15, practice 2024-02-19, outpatient, Cardiology

Chief complaint: Progressive shortness of breath on exertion over 6 months, now unable to walk more than one city block without stopping, with evening ankle swelling.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Williams, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath on exertion over 6 months, now unable to walk more than one city block without stopping, with evening ankle swelling.

HISTORY OF PRESENT ILLNESS:
Patient returns for follow-up with heart failure, ESRD on dialysis, and obesity hypoventilation syndrome following nephrology and pulmonology visits. He reports progressive dyspnea on exertion over the past 6 months. He can no longer walk more than one city block without dyspnea and occasionally needs to stop to catch his breath on stairs. He denies chest pain but notes ankle swelling by evening.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism (diagnosed 2020-01-15)
- Obesity hypoventilation syndrome (diagnosed 2020-04-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension (20-year history, poorly controlled)
- Chronic Kidney Disease Stage 4 progressing to End-Stage Renal Disease on hemodialysis
- Heart Failure with Reduced Ejection Fraction
- History of Myocardial Infarction (remote)
- History of Deep Vein Thrombosis (left popliteal vein)
- Class III Obesity

His past medical history is significant for hypertension, but he has not been taking medications consistently.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (prior to encounters)
- Left radiocephalic arteriovenous fistula creation for hemodialysis access

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Lisinopril 10 mg orally daily
- Amlodipine 5 mg orally daily
- Metformin 1000 mg orally twice daily (noted to be discontinued upon ESRD diagnosis)
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Furosemide 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily
- Apixaban 5 mg orally twice daily (for history of DVT)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Stroke at age 70
- Sibling: Type 2 Diabetes Mellitus
- Family history of chronic kidney disease

SOCIAL HISTORY:
He drinks alcohol occasionally but does not smoke or use illicit drugs.

PHYSICAL EXAMINATION:
On examination, he appears mildly dyspneic. Jugular venous pressure is mildly elevated. Cardiac auscultation reveals a third heart sound at the apex. Bibasilar crackles are heard on lung exam, and there is trace bilateral lower extremity pitting edema.

IMAGING:
A chest x-ray reveals a markedly enlarged cardiac silhouette and mild pulmonary vascular congestion.

PLAN:
Echocardiogram is scheduled.

## Encounter 3: source 2021-03-15, practice 2024-08-21, ed, Emergency Medicine

Chief complaint: Worsening fatigue, anorexia, and muscle weakness over 2 days; denies chest pain, vomiting, or diarrhea.

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Emily Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening fatigue, anorexia, and muscle weakness over 2 days; denies chest pain, vomiting, or diarrhea.

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with ESRD on hemodialysis and prior cardiology and nephrology evaluations comes to the emergency department with a 2-day history of worsening fatigue, anorexia, and muscle weakness. He denies chest pain, vomiting, or diarrhea.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism (diagnosed 2020-01-15)
- Obesity hypoventilation syndrome (diagnosed 2020-04-15)
- Heart failure with reduced ejection fraction (diagnosed 2020-09-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension (20-year history, poorly controlled)
- Chronic Kidney Disease Stage 4 progressing to End-Stage Renal Disease on hemodialysis
- History of Myocardial Infarction (remote)
- History of Deep Vein Thrombosis (left popliteal vein)
- Class III Obesity

He has a long-standing history of type II diabetes mellitus, hypertension, and stage 4 chronic kidney disease, and he has been nonadherent with medical follow-up for over a year.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (prior to encounters)
- Left radiocephalic arteriovenous fistula creation for hemodialysis access

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Lisinopril 10 mg orally daily
- Amlodipine 5 mg orally daily
- Metformin 1000 mg orally twice daily (noted to be discontinued upon ESRD diagnosis)
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Furosemide 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily
- Apixaban 5 mg orally twice daily (for history of DVT)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Stroke at age 70
- Sibling: Type 2 Diabetes Mellitus
- Family history of chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy alcohol use (6-8 beers daily), currently abstinent and denies current use. Occupation: Retired warehouse supervisor, currently on disability.

VITAL SIGNS:
His temperature is 36.8°C (98.2°F), blood pressure is 158/96 mm Hg, and heart rate is 86/min.

PHYSICAL EXAMINATION:
On physical examination, he appears fatigued but alert. Cardiopulmonary examination is unremarkable. There is mild bilateral pitting edema of the ankles and dullness to percussion over the costovertebral angles bilaterally. Neurologic examination shows 4/5 muscle strength in the lower extremities with delayed relaxation of deep tendon reflexes. He is oriented to time and place.

LABORATORY DATA:
Laboratory testing reveals a serum creatinine of 5.4 mg/dL and blood urea nitrogen of 81 mg/dL. Arterial blood gas on room air shows pH 7.28, and serum sodium is 138 mEq/L. Anion gap is calculated to be elevated.

PLAN:
After stabilization, he is admitted to the nephrology service.

## Encounter 4: source 2021-09-15, practice 2025-02-21, ed, Emergency Medicine

Chief complaint: Progressive shortness of breath and leg swelling over 2 weeks with orthopnea requiring two pillows and paroxysmal nocturnal dyspnea.

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath and leg swelling over 2 weeks with orthopnea requiring two pillows and paroxysmal nocturnal dyspnea.

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male on maintenance hemodialysis with chronic heart failure who returns with progressive symptoms. He was previously evaluated for similar exertional dyspnea and lower extremity edema. Over the past 2 weeks, he has had increasing difficulty climbing stairs and has started sleeping on two pillows at night to breathe more comfortably. He also reports waking up abruptly at night feeling short of breath. For the past 5 days, he has noticed his shoes fitting tighter and his ankles becoming puffy by the evening. He denies chest pain, cough, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism (diagnosed 2020-01-15)
- Obesity hypoventilation syndrome (diagnosed 2020-04-15)
- Heart failure with reduced ejection fraction (diagnosed 2020-09-15)
- End-stage renal disease on hemodialysis (diagnosed 2021-03-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension (20-year history, poorly controlled)
- Chronic Kidney Disease Stage 4 progressing to End-Stage Renal Disease on hemodialysis
- History of Myocardial Infarction (remote)
- History of Deep Vein Thrombosis (left popliteal vein)
- Class III Obesity

His medical history includes long-standing hypertension and a remote myocardial infarction.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (prior to encounters)
- Left radiocephalic arteriovenous fistula creation for hemodialysis access

MEDICATIONS:
He is non-adherent to his medications due to cost concerns.

Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Lisinopril 10 mg orally daily
- Amlodipine 5 mg orally daily
- Metformin 1000 mg orally twice daily (noted to be discontinued upon ESRD diagnosis)
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Furosemide 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily
- Apixaban 5 mg orally twice daily (for history of DVT)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Stroke at age 70
- Sibling: Type 2 Diabetes Mellitus
- Family history of chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy alcohol use (6-8 beers daily), currently abstinent and denies current use. Occupation: Retired warehouse supervisor, currently on disability.

VITAL SIGNS:
On examination, temperature is 36.9°C (98.4°F), blood pressure is 102/66 mm Hg, heart rate is 92/min, and respiratory rate is 20/min with an oxygen saturation of 93% on room air.

PHYSICAL EXAMINATION:
Jugular venous pressure (JVP) is 5cm above the sternal angle. Bibasilar crackles are heard on auscultation of the lungs. Cardiac examination reveals an S3 gallop without murmurs. There is bilateral pitting edema to the mid-shins.

OTHER STUDIES:
Pulmonary artery catheterization shows a mean pulmonary capillary wedge pressure (PCWP) of 26 mm Hg (normal: 6–12 mm Hg).

## Encounter 5: source 2022-03-15, practice 2025-08-25, ed, Emergency Medicine

Chief complaint: Swollen, painful left calf that started 2 days ago and has progressively worsened; denies recent trauma or prolonged immobility.

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Robert Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Swollen, painful left calf that started 2 days ago and has progressively worsened; denies recent trauma or prolonged immobility.

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with end-stage renal disease on maintenance hemodialysis and history of left popliteal deep vein thrombosis presents with lower extremity complaints. He reports that the symptoms started 2 days ago and have progressively worsened. He denies any recent trauma or prolonged periods of immobility.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism (diagnosed 2020-01-15)
- Obesity hypoventilation syndrome (diagnosed 2020-04-15)
- Heart failure with reduced ejection fraction (diagnosed 2020-09-15)
- End-stage renal disease on hemodialysis (diagnosed 2021-03-15)
- High-output heart failure due to arteriovenous fistula (diagnosed 2021-09-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension (20-year history, poorly controlled)
- Chronic Kidney Disease Stage 4 progressing to End-Stage Renal Disease on hemodialysis
- History of Myocardial Infarction (remote)
- History of Deep Vein Thrombosis (left popliteal vein)
- Class III Obesity

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (prior to encounters)
- Left radiocephalic arteriovenous fistula creation for hemodialysis access

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Lisinopril 10 mg orally daily
- Amlodipine 5 mg orally daily
- Metformin 1000 mg orally twice daily (noted to be discontinued upon ESRD diagnosis)
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Furosemide 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily
- Apixaban 5 mg orally twice daily (for history of DVT)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Stroke at age 70
- Sibling: Type 2 Diabetes Mellitus
- Family history of chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy alcohol use (6-8 beers daily), currently abstinent and denies current use. Occupation: Retired warehouse supervisor, currently on disability.

PHYSICAL EXAMINATION:
On physical examination, the left calf is erythematous, warm, and tender to palpation. There is pitting edema extending from the calf to the ankle. The circumference of the left calf is 3 cm greater than the right.

IMAGING:
Doppler ultrasound of the lower extremities reveals a deep vein thrombosis (DVT) in the left popliteal vein.

## Encounter 6: source 2022-05-15, practice 2025-11-02, ed, Emergency Medicine

Chief complaint: Weakness and palpitations that started 2 hours ago.

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Emily Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Weakness and palpitations that started 2 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism (diagnosed 2020-01-15)
- Obesity hypoventilation syndrome (diagnosed 2020-04-15)
- Heart failure with reduced ejection fraction (diagnosed 2020-09-15)
- End-stage renal disease on hemodialysis (diagnosed 2021-03-15)
- High-output heart failure due to arteriovenous fistula (diagnosed 2021-09-15)
- Deep vein thrombosis of left popliteal vein (diagnosed 2022-03-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension (20-year history, poorly controlled)
- Chronic Kidney Disease Stage 4 progressing to End-Stage Renal Disease on hemodialysis
- History of Myocardial Infarction (remote)
- History of Deep Vein Thrombosis (left popliteal vein)
- Class III Obesity

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (prior to encounters)
- Left radiocephalic arteriovenous fistula creation for hemodialysis access

MEDICATIONS:
He reports compliance with his medications, including metformin, lisinopril, and amlodipine.

Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Furosemide 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily
- Apixaban 5 mg orally twice daily (for history of DVT)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Stroke at age 70
- Sibling: Type 2 Diabetes Mellitus
- Family history of chronic kidney disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy alcohol use (6-8 beers daily), currently abstinent and denies current use. Occupation: Retired warehouse supervisor, currently on disability.

VITAL SIGNS:
Blood pressure is 150/90 mmHg, heart rate is 55 bpm

LABORATORY DATA:
His current laboratory values are as follows:

- Serum potassium: 7.2 mEq/L
- Serum creatinine: 3.5 mg/dL
- Blood urea nitrogen (BUN): 45 mg/dL
- Serum glucose: 180 mg/dL
- Serum sodium: 138 mEq/L

OTHER STUDIES:
Peaked T waves on the ECG

## Encounter 7: source 2023-01-15, practice 2026-06-29, outpatient, Internal Medicine

Chief complaint: Comes to the clinic for routine follow-up.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Comes to the clinic for routine follow-up.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism (diagnosed 2020-01-15)
- Obesity hypoventilation syndrome (diagnosed 2020-04-15)
- Heart failure with reduced ejection fraction (diagnosed 2020-09-15)
- End-stage renal disease on hemodialysis (diagnosed 2021-03-15)
- High-output heart failure due to arteriovenous fistula (diagnosed 2021-09-15)
- Deep vein thrombosis of left popliteal vein (diagnosed 2022-03-15)
- Hyperkalemia (diagnosed 2022-05-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension (20-year history, poorly controlled)
- Chronic Kidney Disease Stage 4 progressing to End-Stage Renal Disease on hemodialysis
- History of Myocardial Infarction (remote)
- History of Deep Vein Thrombosis (left popliteal vein)
- Class III Obesity

He has a 20-year history of poorly controlled hypertension. He has no known cardiovascular or renal disease.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (prior to encounters)
- Left radiocephalic arteriovenous fistula creation for hemodialysis access

MEDICATIONS:
He is prescribed amlodipine and hydrochlorothiazide but admits he frequently skips his medications

Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Lisinopril 10 mg orally daily
- Metformin 1000 mg orally twice daily (noted to be discontinued upon ESRD diagnosis)
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Furosemide 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily
- Apixaban 5 mg orally twice daily (for history of DVT)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Stroke at age 70
- Sibling: Type 2 Diabetes Mellitus
- Family history of chronic kidney disease

SOCIAL HISTORY:
Drinks six to eight beers daily

REVIEW OF SYSTEMS:
He denies chest pain, dyspnea, or leg swelling

VITAL SIGNS:
Blood pressure today is 174/100 mm Hg, consistent with prior measurements from multiple visits

PHYSICAL EXAMINATION:
Funduscopic examination reveals arteriovenous nicking and copper wiring of the retinal vessels. Physical examination is otherwise unremarkable

LABORATORY DATA:
His last basic metabolic panel showed mild creatinine elevation
