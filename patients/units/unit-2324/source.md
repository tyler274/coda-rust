# synthetic_hospital patient 2324

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 2 | 2020-09-15 | 2025-02-07 (Fri) | ed | Emergency Medicine |
| 5 | 2021-05-15 | 2025-10-13 (Mon) | icu | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin (hives)",
  "Shellfish (anaphylaxis)"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "Occasional, 1-2 drinks per month",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Ovarian cancer at age 71",
  "Sibling: Hypertension and hyperlipidemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 12 years ago (30 pack-year history)",
 "home_medications": [
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "25 mg twice daily",
   "name": "Metoprolol tartrate"
  },
  {
   "dose": "20 mg daily",
   "name": "Rosuvastatin"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "0.4 mg sublingual as needed for chest pain",
   "name": "Nitroglycerin"
  }
 ],
 "surgical_history": [
  "Staging laparotomy with splenectomy (1986)",
  "Mediastinoscopy with lymph node biopsy (1986)",
  "Central venous port placement and removal (1986)"
 ],
 "chronic_conditions": [
  "Coronary artery disease",
  "History of Hodgkin lymphoma, stage IIA, in remission for 38 years",
  "Radiation-induced heart disease",
  "Hypertension",
  "Hyperlipidemia",
  "Type 2 diabetes mellitus"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Cardiac tamponade"]
Comorbidities: []

## Encounter 2: source 2020-09-15, practice 2025-02-07, ed, Emergency Medicine

Chief complaint: Acute onset of dyspnea and chest pain that is worse when lying flat

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of dyspnea and chest pain that is worse when lying flat

HISTORY OF PRESENT ILLNESS:
This 65-year-old male with radiation-induced heart disease and coronary artery disease following Hodgkin lymphoma treatment 38 years ago presents to the emergency department with acute onset of dyspnea and chest pain that is worse when lying flat. He denies any recent trauma or illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Coronary artery disease
- History of Hodgkin lymphoma, stage IIA, in remission for 38 years
- Radiation-induced heart disease
- Hypertension
- Hyperlipidemia
- Type 2 diabetes mellitus

PAST SURGICAL HISTORY:
- Staging laparotomy with splenectomy (1986)
- Mediastinoscopy with lymph node biopsy (1986)
- Central venous port placement and removal (1986)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Metoprolol tartrate 25 mg twice daily
- Rosuvastatin 20 mg daily
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Nitroglycerin 0.4 mg sublingual as needed for chest pain

ALLERGIES:
Penicillin (hives), Shellfish (anaphylaxis)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Ovarian cancer at age 71
- Sibling: Hypertension and hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (30 pack-year history). Alcohol: Occasional, 1-2 drinks per month. Occupation: Retired postal worker.

VITAL SIGNS:
His blood pressure is 90/60 mmHg, heart rate is 110 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated, and heart sounds are distant on auscultation. Lung fields are clear.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 14.2 g/dL
- White blood cell count: 7.8 x 10^3/uL
- Platelets: 230 x 10^3/uL
- Creatinine: 1.0 mg/dL
- B-type natriuretic peptide (BNP): 80 pg/mL

IMAGING:
Chest x-ray reveals a normal cardiac silhouette. Echocardiography demonstrates a small pericardial effusion with right ventricular diastolic collapse.

OTHER STUDIES:
Electrocardiogram shows low voltage QRS complexes and electrical alternans.

## Encounter 5: source 2021-05-15, practice 2025-10-13, icu, Cardiology

Chief complaint: Sudden hypotension and tachycardia on hospital day 3

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Aisha Patel, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Sudden hypotension and tachycardia on hospital day 3

HISTORY OF PRESENT ILLNESS:
This patient with radiation-induced heart disease and prior cardiac tamponade is admitted with chest pain and diagnosed with anterior ST-elevation myocardial infarction. On hospital day 3, he suddenly becomes hypotensive and tachycardic.

PAST MEDICAL HISTORY:
Active Problem List:
- Cardiac tamponade (diagnosed 2020-09-15)
- Coronary artery disease
- History of Hodgkin lymphoma, stage IIA, in remission for 38 years
- Radiation-induced heart disease
- Hypertension
- Hyperlipidemia
- Type 2 diabetes mellitus

PAST SURGICAL HISTORY:
- Staging laparotomy with splenectomy (1986)
- Mediastinoscopy with lymph node biopsy (1986)
- Central venous port placement and removal (1986)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Metoprolol tartrate 25 mg twice daily
- Rosuvastatin 20 mg daily
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Nitroglycerin 0.4 mg sublingual as needed for chest pain

ALLERGIES:
Penicillin (hives), Shellfish (anaphylaxis)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Ovarian cancer at age 71
- Sibling: Hypertension and hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (30 pack-year history). Alcohol: Occasional, 1-2 drinks per month. Occupation: Retired postal worker.

PHYSICAL EXAMINATION:
There is an elevation of jugular venous pressure. His lungs are clear on auscultation. Heart sounds are distant. There is no peripheral edema.

OTHER STUDIES:
An ECG is shown.
