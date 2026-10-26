# synthetic_hospital patient 2212

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-10-16 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2025-12-07 (Sun) | ed | Emergency Medicine |
| 2 | 2020-10-15 | 2026-07-04 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Father: Myocardial infarction at age 61",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Ischemic stroke at age 65"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-year history, quit 2 years ago",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (2005)",
  "Left inguinal hernia repair (2012)"
 ],
 "chronic_conditions": [
  "Hypertension (poorly controlled)",
  "Type 2 Diabetes Mellitus",
  "Hyperlipidemia",
  "Nicotine dependence, in sustained remission"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Atrial fibrillation with rapid ventricular response", "Lacunar stroke"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2025-10-16, outpatient, Internal Medicine

Chief complaint: Intermittent palpitations and mild dyspnea on exertion for two months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent palpitations and mild dyspnea on exertion for two months

HISTORY OF PRESENT ILLNESS:
He reports experiencing intermittent palpitations and mild dyspnea on exertion for the past two months. He denies chest pain, dizziness, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension (poorly controlled)
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Nicotine dependence, in sustained remission

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2005)
- Left inguinal hernia repair (2012)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Type 2 diabetes mellitus
- Brother: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history, quit 2 years ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired postal worker.

VITAL SIGNS:
On physical examination, his blood pressure is 142/88 mmHg, heart rate is irregularly irregular at 98 bpm, and oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
His lungs are clear to auscultation, and there is no lower extremity edema.

LABORATORY DATA:
His laboratory values are as follows:

- Hemoglobin: 14.2 g/dL
- Platelet count: 250,000/mm^3
- INR: 1.0
- Creatinine: 1.1 mg/dL
- Estimated Glomerular Filtration Rate (eGFR): 60 mL/min/1.73m^2

OTHER STUDIES:
An electrocardiogram shows atrial fibrillation with a ventricular rate of 100 bpm.

ASSESSMENT AND PLAN:
Based on the CHA2DS2-VASc score,

## Encounter 1: source 2020-03-15, practice 2025-12-07, ed, Emergency Medicine

Chief complaint: Acute onset of palpitations and shortness of breath while gardening

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of palpitations and shortness of breath while gardening

HISTORY OF PRESENT ILLNESS:
The patient with hypertension and type 2 diabetes mellitus returns two months after his prior atrial fibrillation diagnosis with symptoms that started 2 hours ago while gardening. He denies chest pain, dizziness, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation with rapid ventricular response (diagnosed 2020-01-15)
- Hypertension (poorly controlled)
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Nicotine dependence, in sustained remission

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2005)
- Left inguinal hernia repair (2012)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Type 2 diabetes mellitus
- Brother: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history, quit 2 years ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired postal worker.

VITAL SIGNS:
On examination, his blood pressure is 90/60 mmHg, heart rate is 170 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
His jugular venous pressure is not elevated. Lung examination reveals no crackles, and there is no lower extremity edema.

LABORATORY DATA:
His laboratory values are as follows:

- Hemoglobin: 14 g/dL
- White blood cell count: 7.5 x 10^9/L
- Platelets: 250 x 10^9/L
- Sodium: 140 mmol/L
- Potassium: 4.2 mmol/L
- Creatinine: 1.0 mg/dL
- Glucose: 180 mg/dL

OTHER STUDIES:
An electrocardiogram shows atrial fibrillation with a rapid ventricular response but no ST-segment or T-wave abnormalities.

## Encounter 2: source 2020-10-15, practice 2026-07-04, ed, Emergency Medicine

Chief complaint: Sudden onset of left-sided weakness

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of left-sided weakness

HISTORY OF PRESENT ILLNESS:
This patient with hypertension, type 2 diabetes, and prior atrial fibrillation presents to the emergency department with sudden onset of left-sided weakness. He denies any headache, visual changes, or speech difficulties.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation with rapid ventricular response (diagnosed 2020-01-15)
- Hypertension (poorly controlled)
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Nicotine dependence, in sustained remission

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2005)
- Left inguinal hernia repair (2012)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Type 2 diabetes mellitus
- Brother: Ischemic stroke at age 65

SOCIAL HISTORY:
A 30-pack-year smoking history

VITAL SIGNS:
His blood pressure is 180/95 mmHg, heart rate is 78 bpm, and he is afebrile.

PHYSICAL EXAMINATION:
Physical examination reveals a left-sided facial droop and left upper and lower extremity weakness with a power of 3/5. Sensation is intact, and there are no cerebellar signs.

LABORATORY DATA:
His fasting lipid profile is as follows:

- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 160 mg/dL
- Triglycerides: 250 mg/dL

IMAGING:
A non-contrast head CT scan shows no hemorrhage or mass effect.
