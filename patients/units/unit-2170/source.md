# synthetic_hospital patient 2170

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Vietnamese American, Blue Cross Blue Shield PPO (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-08-30 (Wed) | inpatient | Pulmonology |
| 1 | 2022-07-15 | 2026-02-25 (Wed) | inpatient | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO (employer-sponsored)",
 "occupation": "Warehouse inventory supervisor",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Myocardial infarction at age 62, hypertension",
  "Mother: Type 2 diabetes mellitus",
  "Sibling: Asthma"
 ],
 "race_ethnicity": "Vietnamese American",
 "smoking_status": "Former smoker, 12 pack-years, quit 3 years ago",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "History of pulmonary tuberculosis (completed treatment 5 years prior)",
  "Hypertension",
  "Gastroesophageal reflux disease"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Constrictive pericarditis", "Pulmonary aspergillosis"]
Comorbidities: ["History of pulmonary tuberculosis"]

## Encounter 0: source 2020-01-15, practice 2023-08-30, inpatient, Pulmonology

Chief complaint: Recurrent cough productive of blood-streaked sputum with pleuritic chest pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: inpatient

CHIEF COMPLAINT:
Recurrent cough productive of blood-streaked sputum with pleuritic chest pain

HISTORY OF PRESENT ILLNESS:
He has also experienced unintentional weight loss, intermittent shortness of breath, and pleuritic chest pain over the past several months. He denies recent travel, sick contacts, or night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- History of pulmonary tuberculosis (completed treatment 5 years prior)
- Hypertension
- Gastroesophageal reflux disease

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus
- Sibling: Asthma

SOCIAL HISTORY:
Smoking: Former smoker, 12 pack-years, quit 3 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Warehouse inventory supervisor.

PHYSICAL EXAMINATION:
On examination, he is afebrile, appears thin, and has scattered crackles over the upper lung fields.

LABORATORY DATA:
A sputum sample is sent for microbiologic analysis.

IMAGING:
A chest radiograph shows a cavitary lesion in the right upper lobe containing a round mass that moves with changes in patient position.

## Encounter 1: source 2022-07-15, practice 2026-02-25, inpatient, Cardiology

Chief complaint: Progressive dyspnea on exertion, fatigue, and swelling of the abdomen and lower extremities

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James Rodriguez, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion, fatigue, and swelling of the abdomen and lower extremities

HISTORY OF PRESENT ILLNESS:
This 47-year-old male with prior pulmonary tuberculosis and 2020 hospitalization for cough with hemoptysis presents with 6-month history of progressive dyspnea on exertion, fatigue, and swelling of abdomen and lower extremities. He reports 10-pound weight gain over last two months.

PAST MEDICAL HISTORY:
Active Problem List:
- Pulmonary aspergillosis (diagnosed 2020-01-15)
- History of pulmonary tuberculosis (completed treatment 5 years prior)
- Hypertension
- Gastroesophageal reflux disease

His past medical history is significant for pulmonary tuberculosis treated 5 years ago.

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus
- Sibling: Asthma

SOCIAL HISTORY:
Smoking: Former smoker, 12 pack-years, quit 3 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Warehouse inventory supervisor.

VITAL SIGNS:
Blood pressure is 130/80 mmHg, heart rate is 88 bpm

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated. A pericardial knock is heard on auscultation. Abdominal examination reveals ascites.

IMAGING:
An echocardiogram shows thickened pericardium with normal systolic function and impaired ventricular filling.
