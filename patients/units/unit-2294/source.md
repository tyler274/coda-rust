# synthetic_hospital patient 2294

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-03 (Mon) | outpatient | Pulmonology |
| 1 | 2020-04-15 | 2022-12-30 (Fri) | outpatient | Internal Medicine |
| 2 | 2021-03-15 | 2023-11-28 (Tue) | outpatient | Oncology |
| 3 | 2022-03-15 | 2024-11-28 (Thu) | outpatient | Pulmonology |
| 4 | 2022-09-15 | 2025-06-02 (Mon) | ed | Emergency Medicine |
| 5 | 2023-03-15 | 2025-12-07 (Sun) | ed | Emergency Medicine |
| 6 | 2023-10-15 | 2026-07-01 (Wed) | ed | Emergency Medicine |
| 7 | 2023-11-15 | 2026-08-01 (Sat) | icu | Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "None",
 "family_history": [
  "Father: Lung cancer diagnosed at age 72",
  "Mother: Chronic obstructive pulmonary disease",
  "Brother: Myocardial infarction at age 68"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 50 pack-year history, quit 2 years ago",
 "home_medications": [
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for shortness of breath",
   "name": "Albuterol"
  },
  {
   "dose": "250/50 mcg inhaled twice daily",
   "name": "Fluticasone/Salmeterol"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "No prior surgeries",
  "Pending elective left inguinal hernia repair"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD)",
  "Idiopathic pulmonary fibrosis",
  "Stage IIIB non-small cell lung cancer",
  "History of ischemic stroke",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Uncomplicated parapneumonic effusion", "Syndrome of inappropriate antidiuretic hormone secretion (SIADH)", "Idiopathic Pulmonary Fibrosis", "Tobacco use disorder", "Stage IIIB Non-Small Cell Lung Cancer", "Complicated parapneumonic effusion", "Atrioventricular Nodal Re-entrant Tachycardia", "Ventilator-associated pneumonia"]
Comorbidities: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Chronic obstructive pulmonary disease", "Hypoxemia", "Tobacco use disorder"]

## Encounter 0: source 2020-01-15, practice 2022-10-03, outpatient, Pulmonology

Chief complaint: Worsening fatigue, exertional dyspnea, and dry cough over 8 months with unintentional weight loss

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening fatigue, exertional dyspnea, and dry cough over 8 months with unintentional weight loss

HISTORY OF PRESENT ILLNESS:
He has difficulty walking up a flight of stairs without stopping to catch his breath. He reports a persistent dry cough and a 4-kg unintentional weight loss during this time.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD)
- Idiopathic pulmonary fibrosis
- Stage IIIB non-small cell lung cancer
- History of ischemic stroke
- Tobacco use disorder

PAST SURGICAL HISTORY:
- No prior surgeries
- Pending elective left inguinal hernia repair

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer diagnosed at age 72
- Mother: Chronic obstructive pulmonary disease
- Brother: Myocardial infarction at age 68

SOCIAL HISTORY:
He has a 50-pack-year smoking history and worked as a construction foreman until retirement 3 years ago.

REVIEW OF SYSTEMS:
He denies wheezing, chest tightness, orthopnea, or paroxysmal nocturnal dyspnea.

VITAL SIGNS:
His temperature is 36.7°C (98.1°F), pulse is 84/min, respiratory rate is 20/min, and oxygen saturation is 91% on room air.

PHYSICAL EXAMINATION:
Lung examination reveals fine, dry bibasilar crackles and digital clubbing. Cardiac examination is unremarkable.

IMAGING:
Chest x-ray reveals a decreased lung volume with reticular markings predominantly in the lower lobes. High-resolution CT reveals subpleural and basilar-predominant honeycombing and reticulations.

OTHER STUDIES:
Pulmonary function testing shows reduced total lung capacity, decreased diffusion capacity for carbon monoxide (DLCO), and a preserved FEV1/FVC ratio. Despite poor lung compliance and low oxygenation, he is found to have no evidence of dynamic airway collapse on spirometry or imaging.

PLAN:
The patient is started on antifibrotic therapy and referred for lung transplant evaluation.

## Encounter 1: source 2020-04-15, practice 2022-12-30, outpatient, Internal Medicine

Chief complaint: Preoperative evaluation for elective left inguinal hernia repair

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Preoperative evaluation for elective left inguinal hernia repair

HISTORY OF PRESENT ILLNESS:
The patient returns for preoperative evaluation for elective left inguinal hernia repair in the setting of known chronic obstructive pulmonary disease and recently diagnosed idiopathic pulmonary fibrosis. He reports a productive cough with white sputum for the past several months, which he attributes to his COPD. He denies fever, chills, or chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD)
- Stage IIIB non-small cell lung cancer
- History of ischemic stroke
- Tobacco use disorder

PAST SURGICAL HISTORY:
- No prior surgeries
- Pending elective left inguinal hernia repair

MEDICATIONS:
His medications include tiotropium, albuterol as needed, and a fluticasone/salmeterol inhaler.

Home Medications:
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer diagnosed at age 72
- Mother: Chronic obstructive pulmonary disease
- Brother: Myocardial infarction at age 68

SOCIAL HISTORY:
45-pack-year smoking history. His last cigarette was 2 days ago.

VITAL SIGNS:
On physical examination, his blood pressure is 135/85 mmHg, heart rate is 78 beats per minute, respiratory rate is 18 breaths per minute, and oxygen saturation is 94% on room air

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals diffuse wheezes and prolonged expiration. The remainder of the examination is unremarkable.

## Encounter 2: source 2021-03-15, practice 2023-11-28, outpatient, Oncology

Chief complaint: Follow-up visit after being offered chemotherapy and radiation therapy for lung cancer

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Elena Rodriguez, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit after being offered chemotherapy and radiation therapy for lung cancer

HISTORY OF PRESENT ILLNESS:
He returns for follow-up after being offered chemoradiation following pulmonary evaluation and appears cachectic, reporting a 10-pound weight loss over the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Tobacco use disorder (diagnosed 2020-04-15)
- Chronic obstructive pulmonary disease (COPD)
- Stage IIIB non-small cell lung cancer
- History of ischemic stroke

PAST SURGICAL HISTORY:
- No prior surgeries
- Pending elective left inguinal hernia repair

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer diagnosed at age 72
- Mother: Chronic obstructive pulmonary disease
- Brother: Myocardial infarction at age 68

SOCIAL HISTORY:
He has a 50-pack-year smoking history and quit smoking two years ago.

VITAL SIGNS:
His blood pressure is 130/85 mmHg, heart rate is 102 bpm, and temperature is 37.1°C.

PHYSICAL EXAMINATION:
On examination, he is tachypneic with an oxygen saturation of 92% on room air and has decreased breath sounds in the right lower lobe.

LABORATORY DATA:
- Hemoglobin: 12.5 g/dL
- White blood cell count: 7,000/mm^3
- Platelets: 250,000/mm^3
- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.0 mg/dL
- Blood urea nitrogen (BUN): 14 mg/dL
- Glucose: 110 mg/dL

ASSESSMENT AND PLAN:
The patient expresses a desire to refuse the recommended treatment, stating he prefers to avoid the side effects.

## Encounter 3: source 2022-03-15, practice 2024-11-28, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath and left-sided chest pain with productive cough

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath and left-sided chest pain with productive cough

HISTORY OF PRESENT ILLNESS:
The patient with idiopathic pulmonary fibrosis and Stage IIIB non-small cell lung cancer returns to the pulmonology clinic with a 3-week history of progressive shortness of breath, left-sided chest pain that worsens with deep breaths, and a productive cough. He denies fever or chills.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Tobacco use disorder (diagnosed 2020-04-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2021-03-15)
- Chronic obstructive pulmonary disease (COPD)
- History of ischemic stroke

PAST SURGICAL HISTORY:
- No prior surgeries
- Pending elective left inguinal hernia repair

MEDICATIONS:
His medications include tiotropium and albuterol inhalers.

Home Medications:
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer diagnosed at age 72
- Mother: Chronic obstructive pulmonary disease
- Brother: Myocardial infarction at age 68

SOCIAL HISTORY:
40-pack-year smoking history

VITAL SIGNS:
On physical examination, his temperature is 37.2°C, blood pressure is 135/85 mmHg, heart rate is 92 beats per minute, and respiratory rate is 22 breaths per minute. Oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Chest examination reveals decreased breath sounds, decreased tactile fremitus, and dullness to percussion at the left lung base. The remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory Values:
- White Blood Cell Count: 9,000/uL
- Hemoglobin: 14 g/dL
- Platelet Count: 250,000/uL
- Blood Urea Nitrogen: 18 mg/dL
- Serum Creatinine: 1.0 mg/dL
- Serum Albumin: 3.5 g/dL

IMAGING:
A chest X-ray shows a left-sided pleural effusion.

## Encounter 4: source 2022-09-15, practice 2025-06-02, ed, Emergency Medicine

Chief complaint: Acute worsening shortness of breath, fever, and left-sided pleuritic chest pain

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Emily Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute worsening shortness of breath, fever, and left-sided pleuritic chest pain

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with COPD, idiopathic pulmonary fibrosis, Stage IIIB non-small cell lung cancer, and prior parapneumonic effusion returns with a 4-day history of increasing shortness of breath, productive cough with yellowish sputum, and fever. He also reports left-sided pleuritic chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Tobacco use disorder (diagnosed 2020-04-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2021-03-15)
- Uncomplicated parapneumonic effusion (diagnosed 2022-03-15)
- Chronic obstructive pulmonary disease (COPD)
- History of ischemic stroke

PAST SURGICAL HISTORY:
- No prior surgeries
- Pending elective left inguinal hernia repair

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer diagnosed at age 72
- Mother: Chronic obstructive pulmonary disease
- Brother: Myocardial infarction at age 68

SOCIAL HISTORY:
Smoking: Former smoker, 50 pack-year history, quit 2 years ago. Alcohol: None. Occupation: Retired construction foreman.

VITAL SIGNS:
Temperature is 38.6°C, blood pressure is 135/85 mmHg, heart rate is 102 beats per minute, and respiratory rate is 24 breaths per minute

PHYSICAL EXAMINATION:
Decreased breath sounds and dullness to percussion at the left lung base

LABORATORY DATA:
Leukocytosis with a left shift

IMAGING:
Chest radiograph shows a left lower lobe consolidation with a blunted left costophrenic angle

OTHER STUDIES:
Thoracentesis is performed, and pleural fluid analysis reveals the following:

- Appearance: Cloudy
- pH: 7.10
- Glucose: 30 mg/dL
- LDH: 1000 U/L (serum LDH 300 U/L)
- Protein: 4.5 g/dL (serum protein 7.0 g/dL)
- White blood cells: 20,000/mm³ with 90% neutrophils
- Gram stain: Gram-positive cocci in clusters

## Encounter 5: source 2023-03-15, practice 2025-12-07, ed, Emergency Medicine

Chief complaint: Sudden onset shortness of breath and confusion

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Michael O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset shortness of breath and confusion

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Tobacco use disorder (diagnosed 2020-04-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2021-03-15)
- Uncomplicated parapneumonic effusion (diagnosed 2022-03-15)
- Complicated parapneumonic effusion (diagnosed 2022-09-15)
- Chronic obstructive pulmonary disease (COPD)
- History of ischemic stroke

He has a history of chronic obstructive pulmonary disease and was recently hospitalized for pneumonia.

PAST SURGICAL HISTORY:
- No prior surgeries
- Pending elective left inguinal hernia repair

MEDICATIONS:
He is currently taking inhaled albuterol and tiotropium.

Home Medications:
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer diagnosed at age 72
- Mother: Chronic obstructive pulmonary disease
- Brother: Myocardial infarction at age 68

SOCIAL HISTORY:
Smoking: Former smoker, 50 pack-year history, quit 2 years ago. Alcohol: None. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On exam, the patient is tachypneic and mildly disoriented. He has diffuse wheezes on auscultation.

LABORATORY DATA:
Arterial blood gas shows hypoxemia and mild respiratory alkalosis.

OTHER STUDIES:
ECG findings are shown below.

## Encounter 6: source 2023-10-15, practice 2026-07-01, ed, Emergency Medicine

Chief complaint: Generalized tonic-clonic seizure at home with preceding confusion and gait unsteadiness

ENCOUNTER NOTE
Date: 2023-10-15
Provider: Dr. Aisha Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Generalized tonic-clonic seizure at home with preceding confusion and gait unsteadiness

HISTORY OF PRESENT ILLNESS:
Patient with NSCLC and stroke presents. Over the past week he has been increasingly forgetful, irritable, and unsteady when walking.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Tobacco use disorder (diagnosed 2020-04-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2021-03-15)
- Uncomplicated parapneumonic effusion (diagnosed 2022-03-15)
- Complicated parapneumonic effusion (diagnosed 2022-09-15)
- Atrioventricular Nodal Re-entrant Tachycardia (diagnosed 2023-03-15)
- Chronic obstructive pulmonary disease (COPD)
- History of ischemic stroke

His medical history is notable for chronic obstructive pulmonary disease.

PAST SURGICAL HISTORY:
- No prior surgeries
- Pending elective left inguinal hernia repair

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer diagnosed at age 72
- Mother: Chronic obstructive pulmonary disease
- Brother: Myocardial infarction at age 68

SOCIAL HISTORY:
50-pack-year smoking history

VITAL SIGNS:
Temperature is 37°C (98.6°F), blood pressure is 124/78 mm Hg, pulse is 82/min, and respirations are 16/min

PHYSICAL EXAMINATION:
Physical examination reveals mild confusion and difficulty with tandem gait. No focal neurological deficits are noted. Cardiopulmonary examination shows scattered wheezes without crackles.

LABORATORY DATA:
Laboratory studies show:
Serum sodium        116 mEq/L
Serum potassium      4.3 mEq/L
Serum osmolality      254 mOsm/kg
Urine osmolality        570 mOsm/kg
Urine sodium          64 mEq/L

IMAGING:
A CT scan of the head shows no acute hemorrhage or mass lesion.

## Encounter 7: source 2023-11-15, practice 2026-08-01, icu, Critical Care

Chief complaint: Fever with increased respiratory secretions and oxygen requirements

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. David Kim, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Fever with increased respiratory secretions and oxygen requirements

HISTORY OF PRESENT ILLNESS:
The patient returns to ICU with respiratory decline in the setting of IPF and malignancy. Respiratory secretions have increased in volume and purulence, and he now requires a higher FiO2 to maintain oxygen saturation above 92%.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Tobacco use disorder (diagnosed 2020-04-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2021-03-15)
- Uncomplicated parapneumonic effusion (diagnosed 2022-03-15)
- Complicated parapneumonic effusion (diagnosed 2022-09-15)
- Atrioventricular Nodal Re-entrant Tachycardia (diagnosed 2023-03-15)
- Syndrome of inappropriate antidiuretic hormone secretion (SIADH) (diagnosed 2023-10-15)
- Chronic obstructive pulmonary disease (COPD)
- History of ischemic stroke

PAST SURGICAL HISTORY:
- No prior surgeries
- Pending elective left inguinal hernia repair

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer diagnosed at age 72
- Mother: Chronic obstructive pulmonary disease
- Brother: Myocardial infarction at age 68

SOCIAL HISTORY:
Smoking: Former smoker, 50 pack-year history, quit 2 years ago. Alcohol: None. Occupation: Retired construction foreman.

LABORATORY DATA:
His white blood cell count has risen from 9,000 to 15,000 cells/mm³ over the past 48 hours.

IMAGING:
A chest x-ray reveals new bilateral infiltrates.

OTHER STUDIES:
Sputum culture is pending.
