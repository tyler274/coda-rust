# synthetic_hospital patient 2210

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-19 (Wed) | outpatient | Cardiology |
| 1 | 2020-07-15 | 2023-04-06 (Thu) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2024-04-08 (Mon) | ed | Emergency Medicine |
| 3 | 2022-07-15 | 2025-04-18 (Fri) | ed | Emergency Medicine |
| 4 | 2023-07-15 | 2026-04-12 (Sun) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal service worker",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Father: Coronary artery disease with myocardial infarction at age 62",
  "Mother: Type 2 diabetes mellitus and hypertension",
  "Older brother: Hyperlipidemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "50 mg daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "40 mg daily",
   "name": "Furosemide"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention with drug-eluting stent placement"
 ],
 "chronic_conditions": [
  "Coronary artery disease",
  "Hypertension",
  "Type 2 diabetes mellitus",
  "Hyperlipidemia",
  "Heart failure with reduced ejection fraction",
  "Chronic kidney disease stage 3",
  "Ischemic cardiomyopathy",
  "Status post ST-segment elevation myocardial infarction",
  "Left ventricular aneurysm"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Cardiac arrest", "Heart failure with reduced ejection fraction", "Ischemic cardiomyopathy with left ventricular aneurysm", "Hyponatremia"]
Comorbidities: ["Old myocardial infarction", "Stress hyperglycemia", "Essential hypertension", "Coronary Artery Disease (Antiplatelet Therapy)", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-10-19, outpatient, Cardiology

Chief complaint: Progressive exertional dyspnea and fatigue over the past three months.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive exertional dyspnea and fatigue over the past three months.

HISTORY OF PRESENT ILLNESS:
He reports a 3-month history of progressive exertional dyspnea and orthopnea. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Coronary artery disease
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Heart failure with reduced ejection fraction
- Chronic kidney disease stage 3
- Ischemic cardiomyopathy
- Status post ST-segment elevation myocardial infarction
- Left ventricular aneurysm

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement

MEDICATIONS:
His medications include aspirin, metoprolol, lisinopril, and atorvastatin.

Home Medications:
- Metoprolol succinate 50 mg daily
- Metformin 1000 mg twice daily
- Furosemide 40 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Coronary artery disease with myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus and hypertension
- Older brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired postal service worker.

VITAL SIGNS:
On physical examination, his blood pressure is 130/80 mmHg, heart rate is 78 bpm, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a holosystolic murmur at the apex radiating to the axilla. Lung auscultation is clear. Extremities show no edema.

IMAGING:
An echocardiogram reveals a dilated left ventricle with an area of akinesis in the anterior wall and septum, and a reduced ejection fraction of 35%.

OTHER STUDIES:
An ECG shows persistent ST-segment elevation in the anterior leads and deep Q waves.

## Encounter 1: source 2020-07-15, practice 2023-04-06, ed, Emergency Medicine

Chief complaint: Worsening shortness of breath over the past two days with orthopnea and lower extremity edema.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening shortness of breath over the past two days with orthopnea and lower extremity edema.

HISTORY OF PRESENT ILLNESS:
The patient with ischemic cardiomyopathy and left ventricular aneurysm, last evaluated six months ago for progressive dyspnea, reports that he now feels winded walking across his bedroom and has had to sleep upright in a chair the past two nights due to breathlessness when lying down. He has also noticed swelling in his feet and lower legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Ischemic cardiomyopathy with left ventricular aneurysm (diagnosed 2020-01-15)
- Coronary artery disease
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Heart failure with reduced ejection fraction
- Chronic kidney disease stage 3
- Ischemic cardiomyopathy
- Status post ST-segment elevation myocardial infarction
- Left ventricular aneurysm

His medical history includes coronary artery disease with a prior myocardial infarction, chronic systolic heart failure, type 2 diabetes mellitus, and hyperlipidemia.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement

MEDICATIONS:
He admits to running out of his medications, including furosemide and lisinopril, about a week ago.

Home Medications:
- Aspirin 81 mg daily
- Metoprolol succinate 50 mg daily
- Atorvastatin 40 mg daily
- Metformin 1000 mg twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Coronary artery disease with myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus and hypertension
- Older brother: Hyperlipidemia

SOCIAL HISTORY:
He does not smoke or drink alcohol.

VITAL SIGNS:
His blood pressure is 102/68 mm Hg, heart rate is 106/min, and respiratory rate is 26/min.

PHYSICAL EXAMINATION:
On examination, he appears tired and mildly confused. Jugular venous distension is noted to the angle of the mandible. Lung auscultation reveals bilateral crackles throughout the lung bases. Cardiac exam shows a laterally displaced apical impulse and an S3 gallop. There is moderate pitting edema in both lower extremities.

IMAGING:
Chest x-ray reveals an enlarged cardiac silhouette with pulmonary vascular congestion and Kerley B lines.

OTHER STUDIES:
ECG shows sinus tachycardia with Q waves in the inferior leads.

PLAN:
He is placed on oxygen and intravenous diuretics. A pulmonary artery catheter is inserted to assess hemodynamics.

## Encounter 2: source 2021-07-15, practice 2024-04-08, ed, Emergency Medicine

Chief complaint: Sudden onset severe shortness of breath and progressive ankle swelling over the past week.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset severe shortness of breath and progressive ankle swelling over the past week.

HISTORY OF PRESENT ILLNESS:
68-year-old male with ischemic cardiomyopathy and left ventricular aneurysm, returning with similar symptoms, presents with severe shortness of breath that developed suddenly over the past several hours. He reports progressive swelling of his ankles over the last week and recent difficulty lying flat at night.

PAST MEDICAL HISTORY:
Active Problem List:
- Ischemic cardiomyopathy with left ventricular aneurysm (diagnosed 2020-01-15)
- Heart failure with reduced ejection fraction (diagnosed 2020-07-15)
- Coronary artery disease
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Chronic kidney disease stage 3
- Ischemic cardiomyopathy
- Status post ST-segment elevation myocardial infarction
- Left ventricular aneurysm

He has a history of long-standing hypertension and coronary artery disease, and he was recently hospitalized for a myocardial infarction.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Metoprolol succinate 50 mg daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily
- Metformin 1000 mg twice daily
- Furosemide 40 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Coronary artery disease with myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus and hypertension
- Older brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired postal service worker.

VITAL SIGNS:
Respiratory rate of 32/min and oxygen saturation of 86% on room air. Blood pressure is 168/102 mm Hg, and heart rate is 112/min.

PHYSICAL EXAMINATION:
On examination, he appears anxious and diaphoretic. Jugular venous distension is present, and auscultation of the chest reveals diffuse crackles bilaterally. Cardiac examination shows a third heart sound.

IMAGING:
Chest x-ray demonstrates diffuse alveolar infiltrates and bilateral pleural effusions.

ASSESSMENT AND PLAN:
Acute decompensated heart failure with pulmonary edema

He is started on nesiritide. Treatment is started with nesiritide, a drug that is structurally identical to endogenous human B-type natriuretic peptide.

## Encounter 3: source 2022-07-15, practice 2025-04-18, ed, Emergency Medicine

Chief complaint: Acute confusion and disorientation noted by neighbor, with reduced oral intake due to nocturia.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Elena Vasquez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute confusion and disorientation noted by neighbor, with reduced oral intake due to nocturia.

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with ischemic cardiomyopathy and heart failure with reduced ejection fraction previously evaluated in 2020 and 2021 for cardiac decompensation, now presenting with acute confusion. He is disoriented to time and place but able to state his name. He has no family with him and appears to have difficulty recalling his medical history. His neighbor reports he has been drinking less water lately due to fear of going to the bathroom at night.

PAST MEDICAL HISTORY:
Active Problem List:
- Ischemic cardiomyopathy with left ventricular aneurysm (diagnosed 2020-01-15)
- Heart failure with reduced ejection fraction (diagnosed 2020-07-15)
- Coronary artery disease
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Chronic kidney disease stage 3
- Ischemic cardiomyopathy
- Status post ST-segment elevation myocardial infarction
- Left ventricular aneurysm

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Aspirin 81 mg daily
- Metoprolol succinate 50 mg daily
- Furosemide 40 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Coronary artery disease with myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus and hypertension
- Older brother: Hyperlipidemia

SOCIAL HISTORY:
The patient has no advance directive or designated surrogate decision-maker on file.

VITAL SIGNS:
On examination, his blood pressure is 160/90 mmHg, heart rate is 88 beats per minute, respiratory rate is 18 breaths per minute, and temperature is 37.1°C (98.8°F)

PHYSICAL EXAMINATION:
His skin is warm and dry. Neurological examination reveals no focal deficits.

LABORATORY DATA:
Laboratory tests reveal:

- Serum sodium: 118 mEq/L
- Serum potassium: 4.5 mEq/L
- Serum chloride: 88 mEq/L
- Serum bicarbonate: 24 mEq/L
- Serum glucose: 180 mg/dL
- Serum creatinine: 2.1 mg/dL (baseline 1.9 mg/dL)
- Serum osmolality: 250 mOsm/kg

Urinalysis shows low urine sodium.

## Encounter 4: source 2023-07-15, practice 2026-04-12, icu, Critical Care Medicine

Chief complaint: Found unresponsive at home after earlier complaints of chest pain and shortness of breath.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James O'Brien, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Found unresponsive at home after earlier complaints of chest pain and shortness of breath.

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with coronary artery disease, hypertension, and progressive ischemic cardiomyopathy following left ventricular aneurysm with recurrent heart failure exacerbations. He is brought to the emergency department by ambulance after his wife found him unresponsive at home. His wife reports that he complained of shortness of breath and chest pain earlier in the day but refused to seek medical attention.

PAST MEDICAL HISTORY:
Active Problem List:
- Ischemic cardiomyopathy with left ventricular aneurysm (diagnosed 2020-01-15)
- Heart failure with reduced ejection fraction (diagnosed 2020-07-15)
- Hyponatremia (diagnosed 2022-07-15)
- Coronary artery disease
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Chronic kidney disease stage 3
- Ischemic cardiomyopathy
- Status post ST-segment elevation myocardial infarction
- Left ventricular aneurysm

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement

MEDICATIONS:
He takes aspirin, metoprolol, and metformin.

Home Medications:
- Metoprolol succinate 50 mg daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily
- Furosemide 40 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Coronary artery disease with myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus and hypertension
- Older brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired postal service worker.

PHYSICAL EXAMINATION:
On arrival, he is pulseless and apneic. The cardiac monitor shows organized electrical activity without a detectable pulse.

LABORATORY DATA:
His initial laboratory values are as follows:

- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 1.1 mg/dL
- Glucose: 180 mg/dL
- Troponin I: Not yet available
- Arterial blood gas (on 100% O2): pH 7.1, PaCO2 60 mmHg, PaO2 500 mmHg

PLAN:
The patient is intubated, and cardiopulmonary resuscitation (CPR) is initiated. An intravenous line is established, and epinephrine is administered.
