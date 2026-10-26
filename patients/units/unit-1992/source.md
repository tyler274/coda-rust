# synthetic_hospital patient 1992

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-11-26 (Thu) | outpatient | Gastroenterology |
| 1 | 2020-07-15 | 2021-05-21 (Fri) | outpatient | Psychiatry |
| 2 | 2021-03-15 | 2022-01-24 (Mon) | outpatient | Cardiology |
| 3 | 2021-11-15 | 2022-09-27 (Tue) | outpatient | Hematology & Oncology |
| 4 | 2022-07-15 | 2023-05-23 (Tue) | outpatient | Psychiatry |
| 5 | 2023-07-15 | 2024-05-25 (Sat) | ed | Emergency Medicine |
| 6 | 2024-07-15 | 2025-05-23 (Fri) | icu | Emergency Medicine |
| 7 | 2025-07-15 | 2026-05-28 (Thu) | inpatient | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Commercial sales representative",
 "alcohol_use": "Social alcohol use, typically 1-2 drinks on weekends; reports occasional episodes of heavier consumption with meals",
 "family_history": [
  "Father with Type 2 diabetes mellitus and coronary artery disease",
  "Mother with hypertension",
  "Brother with kidney stones"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 10 cigarettes per day",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg daily",
   "name": "Omeprazole"
  },
  {
   "dose": "5 mg daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg daily",
   "name": "Chlorthalidone"
  },
  {
   "dose": "20 mg nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "as needed for heartburn",
   "name": "Calcium carbonate antacid"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Gastroesophageal reflux disease",
  "Hyperlipidemia",
  "Obstructive sleep apnea",
  "Primary hyperparathyroidism",
  "History of recurrent nephrolithiasis"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Depressive disorder due to hypercalcemia", "Obstructive sleep apnea", "Gastroesophageal reflux disease", "Ureterolithiasis", "Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall", "Esophageal rupture with pleural effusion"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension", "Gastroesophageal reflux disease", "Obesity", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-11-26, outpatient, Gastroenterology

Chief complaint: Six-month history of intermittent burning retrosternal chest pain occurring after meals and when lying down.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of intermittent burning retrosternal chest pain occurring after meals and when lying down.

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of intermittent chest pain. The pain is described as a burning sensation located retrosternally, which often occurs after eating, especially when lying down, and occasionally wakes him from sleep. He denies exertional chest pain. He also reports a chronic cough and recent onset of hoarseness.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Gastroesophageal reflux disease
- Hyperlipidemia
- Obstructive sleep apnea
- Primary hyperparathyroidism
- History of recurrent nephrolithiasis

His past medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Omeprazole 20 mg daily
- Amlodipine 5 mg daily
- Chlorthalidone 25 mg daily
- Atorvastatin 20 mg nightly
- Calcium carbonate antacid as needed for heartburn

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His father had a myocardial infarction at age 55.

SOCIAL HISTORY:
He smokes 10 cigarettes a day and drinks alcohol socially.

VITAL SIGNS:
On physical examination, his blood pressure is 140/85 mmHg, heart rate is 78 beats per minute, and BMI is 30 kg/m^2

PHYSICAL EXAMINATION:
Cardiac examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal:
- Hemoglobin: 14.2 g/dL
- Fasting glucose: 102 mg/dL
- Total cholesterol: 220 mg/dL
- LDL cholesterol: 140 mg/dL
- HDL cholesterol: 35 mg/dL
- Triglycerides: 200 mg/dL

## Encounter 1: source 2020-07-15, practice 2021-05-21, outpatient, Psychiatry

Chief complaint: Persistent fatigue, difficulty concentrating, and low mood for the past six months.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Morrison, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, difficulty concentrating, and low mood for the past six months.

HISTORY OF PRESENT ILLNESS:
This 45-year-old male with gastroesophageal reflux disease and obstructive sleep apnea returns six months after his gastroenterology visit. He reports that his sleep is frequently interrupted, and he often wakes up feeling as if he hasn't slept at all. His wife has noticed that he snores loudly and occasionally seems to stop breathing while asleep.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Obstructive sleep apnea
- Primary hyperparathyroidism
- History of recurrent nephrolithiasis

The patient has a history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His current medications include metformin and lisinopril.

Home Medications:
- Omeprazole 20 mg daily
- Amlodipine 5 mg daily
- Chlorthalidone 25 mg daily
- Atorvastatin 20 mg nightly
- Calcium carbonate antacid as needed for heartburn

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with Type 2 diabetes mellitus and coronary artery disease
- Mother with hypertension
- Brother with kidney stones

SOCIAL HISTORY:
He denies any recent stressors, alcohol use, or recreational drug use.

VITAL SIGNS:
His blood pressure is 138/86 mmHg

PHYSICAL EXAMINATION:
BMI is 32 kg/m^2, and neck circumference is 43 cm. The rest of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin A1c: 7.2%
- Thyroid-Stimulating Hormone (TSH): 2.5 mIU/L
- Serum Sodium: 140 mEq/L
- Serum Creatinine: 1.0 mg/dL

## Encounter 2: source 2021-03-15, practice 2022-01-24, outpatient, Cardiology

Chief complaint: Daytime sleepiness and fatigue with witnessed apneic events during sleep.

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Daytime sleepiness and fatigue with witnessed apneic events during sleep.

HISTORY OF PRESENT ILLNESS:
The patient with hypertension returns for further evaluation of fatigue. His wife reports that he snores loudly and has witnessed him stop breathing during sleep.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (diagnosed 2020-01-15)
- Obstructive sleep apnea (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Primary hyperparathyroidism
- History of recurrent nephrolithiasis

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes despite adherence to a three-drug regimen including a thiazide diuretic, a calcium channel blocker, and an ACE inhibitor.

Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Omeprazole 20 mg daily
- Amlodipine 5 mg daily
- Chlorthalidone 25 mg daily
- Atorvastatin 20 mg nightly
- Calcium carbonate antacid as needed for heartburn

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with Type 2 diabetes mellitus and coronary artery disease
- Mother with hypertension
- Brother with kidney stones

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day. Alcohol: Social alcohol use, typically 1-2 drinks on weekends; reports occasional episodes of heavier consumption with meals. Occupation: Commercial sales representative.

VITAL SIGNS:
The patient has a BMI of 35 kg/m^2 and neck circumference of 17 inches. His blood pressure is 145/95 mmHg

PHYSICAL EXAMINATION:
Physical examination reveals crowded oropharynx and no other abnormalities.

OTHER STUDIES:
Overnight polysomnography shows an apnea-hypopnea index of 35 events per hour.

## Encounter 3: source 2021-11-15, practice 2022-09-27, outpatient, Hematology & Oncology

Chief complaint: Six-month history of morning headaches and chronic fatigue without witnessed snoring.

ENCOUNTER NOTE
Date: 2021-11-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of morning headaches and chronic fatigue without witnessed snoring.

HISTORY OF PRESENT ILLNESS:
The patient returns following sleep evaluations in 2020 and 2021. He reports a 6-month history of fatigue and headaches, particularly in the morning. He denies snoring or any witnessed apneic events. His wife has not observed any unusual sleeping behaviors.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (diagnosed 2020-01-15)
- Obstructive sleep apnea (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Primary hyperparathyroidism
- History of recurrent nephrolithiasis

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Omeprazole 20 mg daily
- Amlodipine 5 mg daily
- Chlorthalidone 25 mg daily
- Atorvastatin 20 mg nightly
- Calcium carbonate antacid as needed for heartburn

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with Type 2 diabetes mellitus and coronary artery disease
- Mother with hypertension
- Brother with kidney stones

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day. Alcohol: Social alcohol use, typically 1-2 drinks on weekends; reports occasional episodes of heavier consumption with meals. Occupation: Commercial sales representative.

VITAL SIGNS:
Blood pressure is 145/90 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
BMI is 32 kg/m^2. His neck circumference is 43 cm.

LABORATORY DATA:
Hemoglobin: 18.5 g/dL (Normal: 13.8-17.2 g/dL)
- Hematocrit: 54.5% (Normal: 40.7-50.3%)
- Erythropoietin: Elevated
- Fasting glucose: 146 mg/dL
- HbA1c: 7.2%

## Encounter 4: source 2022-07-15, practice 2023-05-23, outpatient, Psychiatry

Chief complaint: Worsening mood, daytime fatigue, and irritability over the past six months at wife's request.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Lisa Wong, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening mood, daytime fatigue, and irritability over the past six months at wife's request.

HISTORY OF PRESENT ILLNESS:
The patient is a 47-year-old male with primary hyperparathyroidism and obstructive sleep apnea who returns to the Psychiatry clinic for re-evaluation of neuropsychiatric symptoms. He was initially evaluated in this department in July 2020 for similar complaints and has since undergone cardiology and hematology assessments for persistent fatigue. He states that he "feels slower than usual" and finds it hard to concentrate at work. He denies suicidal ideation or anhedonia but admits to feeling persistently tired and irritable. He reports multiple nighttime awakenings, and his wife states that she often wakes him because he "stops breathing". He reports that this has happened for years and that he "had gotten used to it". He has no prior psychiatric history. His appetite is unchanged, but he reports morning headaches and dry mouth upon awakening. He has a sedentary lifestyle and gained 20 lb. (9 kg) over the past year.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (diagnosed 2020-01-15)
- Obstructive sleep apnea (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Primary hyperparathyroidism
- History of recurrent nephrolithiasis

Past medical history includes hypertension and gastroesophageal reflux disease.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes lisinopril and omeprazole.

Home Medications:
- Metformin 1000 mg twice daily
- Amlodipine 5 mg daily
- Chlorthalidone 25 mg daily
- Atorvastatin 20 mg nightly
- Calcium carbonate antacid as needed for heartburn

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with Type 2 diabetes mellitus and coronary artery disease
- Mother with hypertension
- Brother with kidney stones

SOCIAL HISTORY:
He drinks 1–2 alcoholic beverages on weekends and does not use illicit substances.

VITAL SIGNS:
His BMI is 34 kg/m² and blood pressure is 144/92 mm Hg.

PHYSICAL EXAMINATION:
Physical examination shows a protruding abdomen, a thick neck and a small, receding jaw. Cardiopulmonary and neurologic exams are normal.

LABORATORY DATA:
Laboratory studies, including TSH and CBC, are within normal limits.

OTHER STUDIES:
The patient scores 10 on the PHQ-9 depression screening tool (normal: less than 4).

## Encounter 5: source 2023-07-15, practice 2024-05-25, ed, Emergency Medicine

Chief complaint: Sudden onset of severe left flank pain radiating to the groin with nausea and hematuria.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe left flank pain radiating to the groin with nausea and hematuria.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (diagnosed 2020-01-15)
- Obstructive sleep apnea (diagnosed 2020-07-15)
- Depressive disorder due to hypercalcemia (diagnosed 2022-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Primary hyperparathyroidism
- History of recurrent nephrolithiasis

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Omeprazole 20 mg daily
- Amlodipine 5 mg daily
- Chlorthalidone 25 mg daily
- Atorvastatin 20 mg nightly
- Calcium carbonate antacid as needed for heartburn

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with Type 2 diabetes mellitus and coronary artery disease
- Mother with hypertension
- Brother with kidney stones

SOCIAL HISTORY:
Diet is high in sodium and he rarely drinks water

VITAL SIGNS:
BMI is 32 kg/m^2, blood pressure is 145/90 mmHg

PHYSICAL EXAMINATION:
On physical examination, he is in distress and constantly moving to find a comfortable position. Abdomen is soft with moderate left costovertebral angle tenderness.

LABORATORY DATA:
Urinalysis reveals hematuria and the presence of calcium oxalate crystals. Serum calcium: 10.2 mg/dL (normal: 8.5-10.2 mg/dL). Serum phosphate: 3.5 mg/dL (normal: 2.5-4.5 mg/dL). Serum parathyroid hormone (PTH): 65 pg/mL (normal: 10-65 pg/mL). Serum creatinine: 1.1 mg/dL (normal: 0.6-1.2 mg/dL). Urine pH: 5.5.

IMAGING:
A non-contrast CT scan of the abdomen and pelvis shows a 6 mm radiopaque stone at the left ureterovesical junction.

## Encounter 6: source 2024-07-15, practice 2025-05-23, icu, Emergency Medicine

Chief complaint: Sudden severe tearing chest pain radiating to the back following vomiting after heavy meal and alcohol consumption.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden severe tearing chest pain radiating to the back following vomiting after heavy meal and alcohol consumption.

HISTORY OF PRESENT ILLNESS:
The patient is a 49-year-old male with gastroesophageal reflux disease diagnosed in 2020 and recurrent nephrolithiasis last seen in 2023 who presents with acute chest pain. He reports the pain began shortly after intense vomiting following a large meal and alcohol consumption, describing it as sharp and tearing.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (diagnosed 2020-01-15)
- Obstructive sleep apnea (diagnosed 2020-07-15)
- Depressive disorder due to hypercalcemia (diagnosed 2022-07-15)
- Ureterolithiasis (diagnosed 2023-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Primary hyperparathyroidism
- History of recurrent nephrolithiasis

His medical history is significant for chronic gastroesophageal reflux disease (GERD).

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes for which he takes omeprazole.

Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Amlodipine 5 mg daily
- Chlorthalidone 25 mg daily
- Atorvastatin 20 mg nightly
- Calcium carbonate antacid as needed for heartburn

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with Type 2 diabetes mellitus and coronary artery disease
- Mother with hypertension
- Brother with kidney stones

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day. Alcohol: Social alcohol use, typically 1-2 drinks on weekends; reports occasional episodes of heavier consumption with meals. Occupation: Commercial sales representative.

VITAL SIGNS:
His vital signs are as follows: blood pressure 150/90 mmHg, heart rate 110 bpm, respiratory rate 22 breaths/min, and temperature 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
On examination, the patient appears distressed and diaphoretic. Chest auscultation reveals a crunching sound synchronous with the heartbeat. Subcutaneous emphysema is palpated in the neck.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 18,000 /μL
- Hemoglobin: 13.5 g/dL
- Blood urea nitrogen: 22 mg/dL
- Serum creatinine: 1.1 mg/dL
- Arterial blood gas:
- pH: 7.45
- PaCO2: 32 mmHg
- PaO2: 80 mmHg
- HCO3-: 24 mEq/L

IMAGING:
A chest X-ray shows mediastinal widening and a small left pleural effusion.

## Encounter 7: source 2025-07-15, practice 2026-05-28, inpatient, Emergency Medicine

Chief complaint: Sudden severe pressure-like chest pain radiating to the left arm with diaphoresis and shortness of breath.

ENCOUNTER NOTE
Date: 2025-07-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden severe pressure-like chest pain radiating to the left arm with diaphoresis and shortness of breath.

HISTORY OF PRESENT ILLNESS:
The patient is a 50-year-old male with a history of gastroesophageal reflux disease and prior esophageal rupture who presents with chest pain. He describes the pain as "pressure-like" and rates it 9 out of 10. He also reports shortness of breath and diaphoresis. His previous physician diagnosed him with gastroesophageal reflux disease (GERD) several months ago, and he attributes these symptoms to a severe episode of GERD.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (diagnosed 2020-01-15)
- Obstructive sleep apnea (diagnosed 2020-07-15)
- Depressive disorder due to hypercalcemia (diagnosed 2022-07-15)
- Ureterolithiasis (diagnosed 2023-07-15)
- Esophageal rupture with pleural effusion (diagnosed 2024-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Primary hyperparathyroidism
- History of recurrent nephrolithiasis

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His current medications include metformin, lisinopril, and as-needed antacids.

Home Medications:
- Omeprazole 20 mg daily
- Amlodipine 5 mg daily
- Chlorthalidone 25 mg daily
- Atorvastatin 20 mg nightly
- Calcium carbonate antacid as needed for heartburn

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with Type 2 diabetes mellitus and coronary artery disease
- Mother with hypertension
- Brother with kidney stones

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day. Alcohol: Social alcohol use, typically 1-2 drinks on weekends; reports occasional episodes of heavier consumption with meals. Occupation: Commercial sales representative.

VITAL SIGNS:
On examination, his blood pressure is 160/90 mmHg, heart rate is 102 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
His cardiac examination reveals a regular rhythm without murmurs, rubs, or gallops. Lung fields are clear to auscultation.

LABORATORY DATA:
His laboratory values are as follows:

- Troponin I: 0.5 ng/mL (normal <0.04 ng/mL)
- Creatinine: 1.2 mg/dL (normal 0.6-1.2 mg/dL)
- Hemoglobin A1c: 7.8% (normal <5.7% for non-diabetics)
- Total cholesterol: 220 mg/dL (normal <200 mg/dL)

OTHER STUDIES:
An electrocardiogram (ECG) shows ST-segment elevations in leads II, III, and aVF.
