# synthetic_hospital patient 2004

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-07-15 | 2025-04-10 (Thu) | ed | Emergency Medicine |
| 1 | 2021-07-15 | 2026-04-10 (Fri) | icu | Cardiothoracic Surgery |
| 2 | 2021-07-15 | 2026-04-10 (Fri) | icu | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Software Development Manager",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "Father: Abdominal aortic aneurysm, repaired age 68",
  "Mother: Hypertension, type 2 diabetes",
  "Paternal uncle: Sudden cardiac death at age 54"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 25 pack-years, quit 1 year ago",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
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
  "Emergency ascending aortic replacement with valve resuspension for Type A aortic dissection",
  "Pericardial window for hemopericardium"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Dyslipidemia",
  "Tobacco use disorder, in remission"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Type A Aortic Dissection", "Cardiac tamponade", "Legionnaires' disease"]
Comorbidities: ["Essential hypertension", "Recurrent pericardial effusion (palliative management)"]

## Encounter 0: source 2020-07-15, practice 2025-04-10, ed, Emergency Medicine

Chief complaint: High-grade fever, cough productive of purulent sputum, and shortness of breath

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
High-grade fever, cough productive of purulent sputum, and shortness of breath

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department with a 3-day history of high-grade fever, cough productive of purulent sputum, and shortness of breath. He also complains of confusion, watery diarrhea, and muscle aches. He recently returned from a cruise in the Caribbean.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Dyslipidemia
- Tobacco use disorder, in remission

PAST SURGICAL HISTORY:
- Emergency ascending aortic replacement with valve resuspension for Type A aortic dissection
- Pericardial window for hemopericardium

MEDICATIONS:
Hydrochlorothiazide

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm, repaired age 68
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Sudden cardiac death at age 54

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-years, quit 1 year ago. Alcohol: Social, 2-3 drinks per week. Occupation: Software Development Manager.

VITAL SIGNS:
Temperature is 39.2°C (102.5°F), blood pressure is 130/85 mmHg, pulse is 102 beats/min, and respiratory rate is 22 breaths/min

PHYSICAL EXAMINATION:
Chest auscultation reveals bilateral crackles

LABORATORY DATA:
- White blood cell count: 15,000/mm³ (80% neutrophils)
- Hemoglobin: 14 g/dL
- Platelet count: 250,000/mm³
- Sodium: 130 mEq/L
- Creatinine: 1.1 mg/dL
- Liver enzymes: Mildly elevated
- Urinary antigen test for Legionella: Positive

IMAGING:
Chest X-ray shows patchy infiltrates in both lower lobes

## Encounter 1: source 2021-07-15, practice 2026-04-10, icu, Cardiothoracic Surgery

Chief complaint: Sudden onset of severe chest pain radiating to the back

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiothoracic Surgery
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of severe chest pain radiating to the back

HISTORY OF PRESENT ILLNESS:
Patient with hypertension, dyslipidemia, tobacco use, and Legionnaires' disease presents with pain described as tearing and constant for the past 3 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Legionnaires' disease (diagnosed 2020-07-15)
- Hypertension
- Dyslipidemia
- Tobacco use disorder, in remission

PAST SURGICAL HISTORY:
- Emergency ascending aortic replacement with valve resuspension for Type A aortic dissection
- Pericardial window for hemopericardium

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm, repaired age 68
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Sudden cardiac death at age 54

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-years, quit 1 year ago. Alcohol: Social, 2-3 drinks per week. Occupation: Software Development Manager.

VITAL SIGNS:
His blood pressure is 180/100 mmHg, heart rate is 110 bpm, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
He appears diaphoretic and in distress.
On examination, you note a diastolic murmur best heard at the left sternal border. His lungs are clear to auscultation.

IMAGING:
Chest X-ray reveals a widened mediastinum. Bedside echocardiography demonstrates an intimal flap in the ascending aorta and a small pericardial effusion without right ventricular diastolic collapse. There is no evidence of pulmonary edema.

OTHER STUDIES:
An ECG shows no ST-segment changes.

## Encounter 2: source 2021-07-15, practice 2026-04-10, icu, Cardiology

Chief complaint: Acute onset of dyspnea and chest discomfort that worsens when lying flat

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Patel, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Acute onset of dyspnea and chest discomfort that worsens when lying flat

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with hypertension, dyslipidemia, and prior aortic dissection today describes the discomfort as sharp pain radiating to his back. He denies recent illness or trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Legionnaires' disease (diagnosed 2020-07-15)
- Type A Aortic Dissection (diagnosed 2021-07-15)
- Hypertension
- Dyslipidemia
- Tobacco use disorder, in remission

PAST SURGICAL HISTORY:
- Emergency ascending aortic replacement with valve resuspension for Type A aortic dissection
- Pericardial window for hemopericardium

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm, repaired age 68
- Mother: Hypertension, type 2 diabetes
- Paternal uncle: Sudden cardiac death at age 54

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-years, quit 1 year ago. Alcohol: Social, 2-3 drinks per week. Occupation: Software Development Manager.

VITAL SIGNS:
On examination, his blood pressure is 110/70 mm Hg, heart rate is 110 bpm, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
His jugular venous pressure is elevated, and heart sounds are distant. Lung fields are clear to auscultation. During the examination, a blood pressure drop of 15 mm Hg is noted during inspiration.

IMAGING:
Chest X-ray reveals an enlarged cardiac silhouette. Echocardiography confirms the presence of a large pericardial effusion with diastolic collapse of the right ventricle.

OTHER STUDIES:
An electrocardiogram shows low voltage QRS complexes and electrical alternans.
