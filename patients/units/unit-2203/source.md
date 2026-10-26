# synthetic_hospital patient 2203

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare Part A and B.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-30 (Sun) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2025-02-26 (Wed) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2025-12-31 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare Part A and B",
 "occupation": "Retired postal clerk",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Myocardial infarction at age 61",
  "Mother: Type 2 diabetes mellitus",
  "Paternal grandfather: Ischemic stroke at age 68"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (quit 2010, 25 pack-years)",
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
  "Cholecystectomy (2005)",
  "Inguinal hernia repair (2012)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Acute ischemic stroke"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2024-06-30, ed, Emergency Medicine

Chief complaint: sudden onset of left-sided weakness and slurred speech that began two hours ago

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of left-sided weakness and slurred speech that began two hours ago

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia

His past medical history includes hypertension, type 2 diabetes mellitus, and hyperlipidemia.

PAST SURGICAL HISTORY:
- Cholecystectomy (2005)
- Inguinal hernia repair (2012)

MEDICATIONS:
He has been compliant with his medications, which include lisinopril, metformin, and atorvastatin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Type 2 diabetes mellitus
- Paternal grandfather: Ischemic stroke at age 68

SOCIAL HISTORY:
Smoking: Former smoker (quit 2010, 25 pack-years). Alcohol: Social (1-2 drinks per week). Occupation: Retired postal clerk.

VITAL SIGNS:
His blood pressure is 155/90 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 breaths per minute

PHYSICAL EXAMINATION:
Neurological examination reveals decreased strength in the left upper and lower extremities, left facial droop, and dysarthria.

LABORATORY DATA:
His fasting lipid panel from one month ago is as follows:

- Total cholesterol: 190 mg/dL
- LDL cholesterol: 100 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 150 mg/dL

IMAGING:
A non-contrast head CT scan shows no evidence of hemorrhage.

## Encounter 1: source 2020-09-15, practice 2025-02-26, ed, Emergency Medicine

Chief complaint: sudden onset of left-sided weakness and slurred speech that began 2 hours ago

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of left-sided weakness and slurred speech that began 2 hours ago

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with hypertension, type 2 diabetes mellitus, hyperlipidemia, and a prior ischemic stroke in January 2020 presents to the emergency department with sudden onset of left-sided weakness and slurred speech that began 2 hours ago. He was last seen normal 3 hours prior to arrival. His wife reports that he has been compliant with his medications, which include metformin, lisinopril, and atorvastatin.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute ischemic stroke (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Cholecystectomy (2005)
- Inguinal hernia repair (2012)

MEDICATIONS:
Metformin, lisinopril, and atorvastatin

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Type 2 diabetes mellitus
- Paternal grandfather: Ischemic stroke at age 68

SOCIAL HISTORY:
Smoking: Former smoker (quit 2010, 25 pack-years). Alcohol: Social (1-2 drinks per week). Occupation: Retired postal clerk.

VITAL SIGNS:
On examination, his blood pressure is 155/90 mmHg, heart rate is 78 beats per minute, and he is afebrile

PHYSICAL EXAMINATION:
Neurological examination reveals left facial droop, left arm and leg weakness, and dysarthria.

LABORATORY DATA:
- Glucose: 180 mg/dL
- Creatinine: 1.2 mg/dL
- INR: 1.0
- Platelet count: 250,000/µL

IMAGING:
A noncontrast head CT scan shows no evidence of hemorrhage.

## Encounter 2: source 2021-07-15, practice 2025-12-31, ed, Emergency Medicine

Chief complaint: confusion and slurred speech

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Confusion and slurred speech

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with hypertension, type 2 diabetes mellitus, and hyperlipidemia who returns to the emergency department 10 months after his last stroke. He is brought by his wife due to confusion and slurred speech that began 2 hours ago. She reports he has been adherent to medications with no recent illnesses.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute ischemic stroke (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Cholecystectomy (2005)
- Inguinal hernia repair (2012)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Type 2 diabetes mellitus
- Paternal grandfather: Ischemic stroke at age 68

SOCIAL HISTORY:
Smoking: Former smoker (quit 2010, 25 pack-years). Alcohol: Social (1-2 drinks per week). Occupation: Retired postal clerk.

VITAL SIGNS:
His temperature is 37.0°C (98.6°F), blood pressure is 160/90 mmHg, pulse is 88/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
On examination, the patient is alert but appears anxious. Neurological examination reveals expressive aphasia and right facial droop.

IMAGING:
A non-contrast head CT scan shows no hemorrhage or mass effect.
