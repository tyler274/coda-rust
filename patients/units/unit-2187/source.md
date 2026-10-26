# synthetic_hospital patient 2187

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-04-21 (Thu) | ed | Emergency Medicine |
| 1 | 2020-04-15 | 2022-07-18 (Mon) | ed | Emergency Medicine |
| 2 | 2020-10-15 | 2023-01-31 (Tue) | ed | Emergency Medicine |
| 3 | 2021-04-15 | 2023-07-20 (Thu) | inpatient | Internal Medicine |
| 4 | 2022-01-15 | 2024-04-20 (Sat) | ed | Emergency Medicine |
| 5 | 2022-07-15 | 2024-10-22 (Tue) | inpatient | Internal Medicine |
| 6 | 2023-03-15 | 2025-06-17 (Tue) | inpatient | Internal Medicine |
| 7 | 2023-11-15 | 2026-02-28 (Sat) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Myocardial infarction at age 71",
  "Mother: Type 2 Diabetes Mellitus, stroke at age 78",
  "One sibling with Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  },
  {
   "dose": "250 mcg inhaled twice daily",
   "name": "Fluticasone propionate"
  },
  {
   "dose": "50 mcg inhaled twice daily",
   "name": "Salmeterol"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed",
   "name": "Albuterol"
  },
  {
   "dose": "50 mg by mouth daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "0.125 mg by mouth daily",
   "name": "Digoxin"
  },
  {
   "dose": "10 mg by mouth daily",
   "name": "Lisinopril"
  },
  {
   "dose": "500 mg by mouth twice daily",
   "name": "Metformin"
  },
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "81 mg by mouth daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (age 52)",
  "Left knee arthroscopy (age 45)"
 ],
 "chronic_conditions": [
  "Chronic Obstructive Pulmonary Disease (GOLD Stage IV, on home oxygen therapy)",
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Chronic Kidney Disease Stage 3 (eGFR 30-44)",
  "Atrial Fibrillation (rate-controlled)",
  "Heart Failure with preserved ejection fraction (HFpEF)"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["COPD exacerbation with acute hypercapnic respiratory failure without altered mental status", "Digoxin toxicity", "Hyperchloremic metabolic acidosis due to excess normal saline infusion", "Euthyroid sick syndrome"]
Comorbidities: ["Hypercapnic respiratory failure", "Hyponatremia", "Delirium due to medical condition", "Medication noncompliance", "Tobacco use disorder", "Chronic obstructive pulmonary disease", "Respiratory arrest", "Essential hypertension", "COPD exacerbation with acute hypercapnic respiratory failure without altered mental status"]

## Encounter 0: source 2020-01-15, practice 2022-04-21, ed, Emergency Medicine

Chief complaint: Presents to the emergency department with a 2-day history of increasing shortness of breath and a productive cough with green sputum

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Presents to the emergency department with a 2-day history of increasing shortness of breath and a productive cough with green sputum

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic Obstructive Pulmonary Disease (GOLD Stage IV, on home oxygen therapy)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease Stage 3 (eGFR 30-44)
- Atrial Fibrillation (rate-controlled)
- Heart Failure with preserved ejection fraction (HFpEF)

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 52)
- Left knee arthroscopy (age 45)

MEDICATIONS:
His medications include tiotropium and metoprolol.

Home Medications:
- Fluticasone propionate 250 mcg inhaled twice daily
- Salmeterol 50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metoprolol succinate 50 mg by mouth daily
- Digoxin 0.125 mg by mouth daily
- Lisinopril 10 mg by mouth daily
- Metformin 500 mg by mouth twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Aspirin 81 mg by mouth daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 Diabetes Mellitus, stroke at age 78
- One sibling with Hypertension

SOCIAL HISTORY:
He has a 40-pack-year smoking history and quit smoking 5 years ago.

PHYSICAL EXAMINATION:
On examination, he is tachypneic with a respiratory rate of 28 breaths per minute, oxygen saturation is 88% on room air, and he has diffuse wheezes on auscultation.

LABORATORY DATA:
Arterial blood gas on room air shows:

- pH: 7.32
- PaCO2: 60 mmHg
- PaO2: 55 mmHg
- HCO3-: 30 mEq/L

IMAGING:
Chest X-ray shows increased bronchovascular markings and no focal consolidation.

ASSESSMENT AND PLAN:
The patient is diagnosed with a COPD exacerbation likely triggered by a respiratory infection.

He is advised to be admitted for management with supplemental oxygen, antibiotics, and systemic corticosteroids. However, the patient refuses hospital admission, stating he prefers to be at home. After addressing potential concerns and ensuring the patient understands the risks of refusing treatment, he remains adamant about not being admitted.

## Encounter 1: source 2020-04-15, practice 2022-07-18, ed, Emergency Medicine

Chief complaint: Increased shortness of breath, productive cough with greenish sputum, and wheezing

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increased shortness of breath, productive cough with greenish sputum, and wheezing

HISTORY OF PRESENT ILLNESS:
This patient with Stage IV COPD on home oxygen returns to the ED with a 3-day history of increased shortness of breath, productive cough with greenish sputum, and wheezing, per January.

PAST MEDICAL HISTORY:
Active Problem List:
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2020-01-15)
- Chronic Obstructive Pulmonary Disease (GOLD Stage IV, on home oxygen therapy)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease Stage 3 (eGFR 30-44)
- Atrial Fibrillation (rate-controlled)
- Heart Failure with preserved ejection fraction (HFpEF)

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 52)
- Left knee arthroscopy (age 45)

MEDICATIONS:
He takes tiotropium inhaler daily.

Home Medications:
- Fluticasone propionate 250 mcg inhaled twice daily
- Salmeterol 50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metoprolol succinate 50 mg by mouth daily
- Digoxin 0.125 mg by mouth daily
- Lisinopril 10 mg by mouth daily
- Metformin 500 mg by mouth twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Aspirin 81 mg by mouth daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 Diabetes Mellitus, stroke at age 78
- One sibling with Hypertension

SOCIAL HISTORY:
40-pack-year smoking history

VITAL SIGNS:
Respiratory rate of 28 breaths per minute, heart rate of 110 beats per minute, blood pressure of 150/90 mmHg, temperature of 37.8°C, and oxygen saturation of 88% on room air

PHYSICAL EXAMINATION:
On examination, he is alert but in respiratory distress. Auscultation reveals diffuse expiratory wheezes and decreased breath sounds at the bases

LABORATORY DATA:
Arterial blood gas on room air shows pH 7.32, PaCO2 60 mmHg, PaO2 55 mmHg, and HCO3- 30 mEq/L

IMAGING:
Chest X-ray demonstrates hyperinflated lungs with no signs of consolidation or pneumothorax

PLAN:
The patient is started on supplemental oxygen, nebulized albuterol, and intravenous corticosteroids. Despite initial treatment, his respiratory effort remains high, and his oxygen saturation improves only to 92% on 4 L/min of oxygen via nasal cannula.

## Encounter 2: source 2020-10-15, practice 2023-01-31, ed, Emergency Medicine

Chief complaint: Increased shortness of breath with fever of 38.5°C (101.3°F)

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increased shortness of breath with fever of 38.5°C (101.3°F)

HISTORY OF PRESENT ILLNESS:
This 68-year-old with GOLD Stage IV COPD on home oxygen, seen in January and April 2020 for similar complaints, presents to the emergency department with a 3-day history of increased shortness of breath, productive cough with greenish sputum, and wheezing. He reports a fever of 38.5°C (101.3°F) at home.

PAST MEDICAL HISTORY:
Active Problem List:
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2020-01-15)
- Chronic Obstructive Pulmonary Disease (GOLD Stage IV, on home oxygen therapy)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease Stage 3 (eGFR 30-44)
- Atrial Fibrillation (rate-controlled)
- Heart Failure with preserved ejection fraction (HFpEF)

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 52)
- Left knee arthroscopy (age 45)

MEDICATIONS:
His medications include tiotropium and albuterol inhalers.

Home Medications:
- Fluticasone propionate 250 mcg inhaled twice daily
- Salmeterol 50 mcg inhaled twice daily
- Metoprolol succinate 50 mg by mouth daily
- Digoxin 0.125 mg by mouth daily
- Lisinopril 10 mg by mouth daily
- Metformin 500 mg by mouth twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Aspirin 81 mg by mouth daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 Diabetes Mellitus, stroke at age 78
- One sibling with Hypertension

SOCIAL HISTORY:
He has a 40-pack-year smoking history and quit smoking 5 years ago.

VITAL SIGNS:
With a respiratory rate of 28 breaths/min, heart rate is 110 beats/min, blood pressure is 145/90 mmHg, and oxygen saturation is 88% on room air

PHYSICAL EXAMINATION:
On examination, he is tachypneic
Lung auscultation reveals diffuse wheezes and decreased breath sounds at the bases bilaterally.

LABORATORY DATA:
Arterial blood gas on room air shows:

- pH: 7.32
- PaCO2: 50 mmHg
- PaO2: 60 mmHg
- HCO3-: 26 mEq/L

IMAGING:
Chest X-ray shows hyperinflated lungs and no signs of consolidation.

## Encounter 3: source 2021-04-15, practice 2023-07-20, inpatient, Internal Medicine

Chief complaint: Admitted to the hospital for acute exacerbation of COPD

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. James O'Brien, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Admitted to the hospital for acute exacerbation of COPD

PAST MEDICAL HISTORY:
Active Problem List:
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2020-01-15)
- Chronic Obstructive Pulmonary Disease (GOLD Stage IV, on home oxygen therapy)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease Stage 3 (eGFR 30-44)
- Atrial Fibrillation (rate-controlled)
- Heart Failure with preserved ejection fraction (HFpEF)

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 52)
- Left knee arthroscopy (age 45)

MEDICATIONS:
He is currently on tiotropium, salmeterol, and metoprolol.

Home Medications:
- Fluticasone propionate 250 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metoprolol succinate 50 mg by mouth daily
- Digoxin 0.125 mg by mouth daily
- Lisinopril 10 mg by mouth daily
- Metformin 500 mg by mouth twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Aspirin 81 mg by mouth daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 Diabetes Mellitus, stroke at age 78
- One sibling with Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
His temperature is 37.2°C (99°F), blood pressure is 150/90 mmHg, and heart rate is 102 beats per minute.

PHYSICAL EXAMINATION:
On examination, he is tachypneic with a respiratory rate of 28 breaths per minute, oxygen saturation is 88% on room air, and diffuse wheezes are heard on auscultation.

LABORATORY DATA:
Arterial blood gas shows respiratory acidosis.
Laboratory tests are ordered:

- Serum TSH: 0.4 μIU/mL (normal range: 0.5-5.0 μIU/mL)
- Free T4: 0.8 ng/dL (normal range: 0.9-1.7 ng/dL)
- Free T3: 1.8 pg/mL (normal range: 2.3-4.2 pg/mL)

ASSESSMENT AND PLAN:
Three days into his hospital stay, he remains lethargic despite improvement in his respiratory status.

The patient is started on systemic corticosteroids and antibiotics.

## Encounter 4: source 2022-01-15, practice 2024-04-20, ed, Emergency Medicine

Chief complaint: Brought to the emergency department by his daughter because of increasing confusion and lethargy over the past two days

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Elena Volkov, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department by his daughter because of increasing confusion and lethargy over the past two days

HISTORY OF PRESENT ILLNESS:
The patient with severe COPD returns with altered mental status. His daughter reports that he has been more forgetful recently, but the acute change in his mental status is new.

PAST MEDICAL HISTORY:
Active Problem List:
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2020-01-15)
- Euthyroid sick syndrome (diagnosed 2021-04-15)
- Chronic Obstructive Pulmonary Disease (GOLD Stage IV, on home oxygen therapy)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease Stage 3 (eGFR 30-44)
- Atrial Fibrillation (rate-controlled)
- Heart Failure with preserved ejection fraction (HFpEF)

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 52)
- Left knee arthroscopy (age 45)

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone propionate 250 mcg inhaled twice daily
- Salmeterol 50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metoprolol succinate 50 mg by mouth daily
- Digoxin 0.125 mg by mouth daily
- Lisinopril 10 mg by mouth daily
- Metformin 500 mg by mouth twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Aspirin 81 mg by mouth daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 Diabetes Mellitus, stroke at age 78
- One sibling with Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
His temperature is 37.2°C (99.0°F), blood pressure is 165/90 mmHg, pulse is 102/min, and respiratory rate is 22/min with an oxygen saturation of 89% on room air.

PHYSICAL EXAMINATION:
On examination, the patient is disoriented to time and place but oriented to person.
Physical examination reveals bilateral wheezes and decreased breath sounds at the lung bases.

LABORATORY DATA:
Laboratory studies are as follows:

- Sodium: 122 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 89 mEq/L
- Bicarbonate: 28 mEq/L
- Blood urea nitrogen (BUN): 35 mg/dL
- Creatinine: 1.2 mg/dL
- Glucose: 98 mg/dL
- Arterial blood gas (ABG) on room air:
- pH: 7.32
- PaCO2: 55 mmHg
- PaO2: 58 mmHg
- HCO3-: 29 mEq/L

IMAGING:
Chest X-ray shows hyperinflated lungs and flattened diaphragms without evidence of pneumonia.

PLAN:
The patient is started on supplemental oxygen, nebulized bronchodilators, and intravenous corticosteroids. Over the next 48 hours, his mental status improves significantly, and he is now alert and fully oriented. He expresses a desire to return home and manage his condition with the help of his daughter.

## Encounter 5: source 2022-07-15, practice 2024-10-22, inpatient, Internal Medicine

Chief complaint: Community-acquired pneumonia requiring admission

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Robert Kim, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Community-acquired pneumonia requiring admission

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with GOLD Stage IV COPD on home oxygen and recurrent hypercapnic respiratory failure, most recently in January 2022 with confusion, is admitted to the hospital for a community-acquired pneumonia. He is started on appropriate antibiotics and intravenous fluids. On hospital day 3, he is noted to have improved clinically and is scheduled for transition to oral antibiotics and discharge the following day. During the night shift, the patient becomes acutely confused and agitated.

PAST MEDICAL HISTORY:
Active Problem List:
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2020-01-15)
- Euthyroid sick syndrome (diagnosed 2021-04-15)
- Chronic Obstructive Pulmonary Disease (GOLD Stage IV, on home oxygen therapy)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease Stage 3 (eGFR 30-44)
- Atrial Fibrillation (rate-controlled)
- Heart Failure with preserved ejection fraction (HFpEF)

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 52)
- Left knee arthroscopy (age 45)

MEDICATIONS:
He takes metformin, lisinopril, and a sliding scale of insulin.

Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone propionate 250 mcg inhaled twice daily
- Salmeterol 50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metoprolol succinate 50 mg by mouth daily
- Digoxin 0.125 mg by mouth daily
- Insulin glargine 24 units subcutaneous at bedtime
- Aspirin 81 mg by mouth daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 Diabetes Mellitus, stroke at age 78
- One sibling with Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
There is no focal neurological deficit on examination

LABORATORY DATA:
His recent laboratory values are as follows:

- Sodium: 128 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 4.2 mEq/L (Normal: 3.5-5.1 mEq/L)
- Chloride: 96 mEq/L (Normal: 98-106 mEq/L)
- Bicarbonate: 24 mEq/L (Normal: 22-29 mEq/L)
- Blood urea nitrogen (BUN): 31 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 2.1 mg/dL (Baseline: 1.8 mg/dL)
- Glucose: 180 mg/dL (Normal: 70-99 mg/dL)

ASSESSMENT AND PLAN:
The night team notes the acute change in mental status

Plans to hand off this information to the day team

## Encounter 6: source 2023-03-15, practice 2025-06-17, inpatient, Internal Medicine

Chief complaint: Acute COPD exacerbation requiring hospitalization

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Maria Santos, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Acute COPD exacerbation requiring hospitalization

HISTORY OF PRESENT ILLNESS:
The patient is a 71-year-old male with GOLD Stage IV COPD on home oxygen and recurrent exacerbations requiring prior hospitalizations, most recently in July 2022. He presents with an acute exacerbation requiring hospitalization and is started on systemic corticosteroids and nebulized bronchodilators. On hospital day 3, he becomes acutely confused and agitated. His daughter, who has been at his bedside, reports that he has been "not himself" since earlier in the day. The nurse reports that the patient received his neighbor's medications this morning due to a chart mix-up; the neighbor is treated for heart failure with digoxin, furosemide, and potassium chloride.

PAST MEDICAL HISTORY:
Active Problem List:
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2020-01-15)
- Euthyroid sick syndrome (diagnosed 2021-04-15)
- Hyperchloremic metabolic acidosis due to excess normal saline infusion (diagnosed 2022-07-15)
- Chronic Obstructive Pulmonary Disease (GOLD Stage IV, on home oxygen therapy)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease Stage 3 (eGFR 30-44)
- Atrial Fibrillation (rate-controlled)
- Heart Failure with preserved ejection fraction (HFpEF)

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 52)
- Left knee arthroscopy (age 45)

MEDICATIONS:
He takes tiotropium, salmeterol, fluticasone, and metoprolol.

Home Medications:
- Fluticasone propionate 250 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metoprolol succinate 50 mg by mouth daily
- Digoxin 0.125 mg by mouth daily
- Lisinopril 10 mg by mouth daily
- Metformin 500 mg by mouth twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Aspirin 81 mg by mouth daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 Diabetes Mellitus, stroke at age 78
- One sibling with Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for mild bilateral wheezing on lung auscultation

LABORATORY DATA:
Laboratory results are as follows:

- Sodium: 130 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen (BUN): 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 98 mg/dL
- Calcium: 9.2 mg/dL
- Magnesium: 1.8 mg/dL
- Complete blood count (CBC): within normal limits
- Arterial blood gas (ABG): pH 7.42, PaCO2 40 mmHg, PaO2 80 mmHg

## Encounter 7: source 2023-11-15, practice 2026-02-28, inpatient, Internal Medicine

Chief complaint: Worsening shortness of breath and confusion

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. David Washington, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Worsening shortness of breath and confusion

HISTORY OF PRESENT ILLNESS:
The patient is a 71-year-old male with GOLD Stage IV COPD on home oxygen therapy who presents with confusion and respiratory symptoms. He has been hospitalized multiple times for respiratory exacerbations, most recently in March 2023 and July 2022. His daughter reports that he has had a mild cough with yellow sputum for the past week but began acting strangely earlier this morning. At baseline, he uses oxygen at home and needs to rest after climbing a single flight of stairs. Today, he became increasingly drowsy and had difficulty speaking in full sentences. He also appeared flushed and diaphoretic.

PAST MEDICAL HISTORY:
Active Problem List:
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2020-01-15)
- Euthyroid sick syndrome (diagnosed 2021-04-15)
- Hyperchloremic metabolic acidosis due to excess normal saline infusion (diagnosed 2022-07-15)
- Digoxin toxicity (diagnosed 2023-03-15)
- Chronic Obstructive Pulmonary Disease (GOLD Stage IV, on home oxygen therapy)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease Stage 3 (eGFR 30-44)
- Atrial Fibrillation (rate-controlled)
- Heart Failure with preserved ejection fraction (HFpEF)

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 52)
- Left knee arthroscopy (age 45)

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone propionate 250 mcg inhaled twice daily
- Salmeterol 50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metoprolol succinate 50 mg by mouth daily
- Digoxin 0.125 mg by mouth daily
- Lisinopril 10 mg by mouth daily
- Metformin 500 mg by mouth twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Aspirin 81 mg by mouth daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71
- Mother: Type 2 Diabetes Mellitus, stroke at age 78
- One sibling with Hypertension

SOCIAL HISTORY:
30-year smoking history

VITAL SIGNS:
Respiratory rate is 20/min, oxygen saturation is 87% on 2 L/min nasal cannula, and heart rate is 112/min

PHYSICAL EXAMINATION:
He is disoriented to time and place. There is end-expiratory wheezing bilaterally with prolonged expiratory phase.

LABORATORY DATA:
Arterial blood gas reveals pH 7.24, PCO₂ 78 mm Hg, and HCO₃⁻ 34 mEq/L.
