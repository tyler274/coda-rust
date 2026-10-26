# synthetic_hospital patient 2854

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 78 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-20 (Fri) | ed | Emergency Medicine |
| 1 | 2020-06-15 | 2025-05-20 (Tue) | ed | Emergency Medicine |
| 2 | 2021-06-15 | 2026-05-30 (Sat) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Father: Type 2 Diabetes Mellitus, Hypertension",
  "Mother: Chronic Kidney Disease, Cerebrovascular Accident",
  "Sister: Type 2 Diabetes Mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25 mg oral daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "10 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "500 mg oral twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg oral daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy (1995)",
  "Laparoscopic cholecystectomy (2004)",
  "Appendectomy (1962)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Chronic Kidney Disease"
 ],
 "age_at_first_encounter": 78
}
```

Primary diagnoses: ["Nonexertional heat stroke", "Acute kidney injury", "Acute pyelonephritis"]
Comorbidities: ["Acute kidney injury", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2024-12-20, ed, Emergency Medicine

Chief complaint: Fever, chills, and right-sided flank pain for 2 days

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, chills, and right-sided flank pain for 2 days

HISTORY OF PRESENT ILLNESS:
A 68-year-old woman with a history of type 2 diabetes mellitus presents to the emergency department with a 2-day history of fever, chills, and right-sided flank pain. She reports nausea and has vomited twice today. She has not been able to keep down oral fluids or medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (1995)
- Laparoscopic cholecystectomy (2004)
- Appendectomy (1962)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily
- Lisinopril 10 mg oral daily
- Metformin 500 mg oral twice daily
- Atorvastatin 20 mg oral daily
- Aspirin 81 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Hypertension
- Mother: Chronic Kidney Disease, Cerebrovascular Accident
- Sister: Type 2 Diabetes Mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her temperature is 38.9°C (102°F), blood pressure is 100/60 mmHg, pulse is 110/min, and respiratory rate is 22/min

PHYSICAL EXAMINATION:
Physical examination reveals costovertebral angle tenderness on the right

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 15,000/mm³ (Normal: 4,500-11,000/mm³)
- Blood urea nitrogen (BUN): 28 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 1.8 mg/dL (Normal: 0.6-1.1 mg/dL)
- Urinalysis: Pyuria, bacteriuria, and positive leukocyte esterase

OTHER STUDIES:
A urine culture is pending

## Encounter 1: source 2020-06-15, practice 2025-05-20, ed, Emergency Medicine

Chief complaint: Weakness and dizziness with decreased oral intake and watery diarrhea

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. Carlos Mendez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Weakness and dizziness with decreased oral intake and watery diarrhea

HISTORY OF PRESENT ILLNESS:
This 78-year-old female with hypertension, type 2 diabetes mellitus, and chronic kidney disease returns to the emergency department five months after treatment for acute pyelonephritis. She presents with a 2-day history of weakness and dizziness. She reports a poor appetite over the past week and has had several episodes of watery diarrhea.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute pyelonephritis (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease

She has a past medical history of hypertension.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (1995)
- Laparoscopic cholecystectomy (2004)
- Appendectomy (1962)

MEDICATIONS:
She takes for which she takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg oral daily
- Metformin 500 mg oral twice daily
- Atorvastatin 20 mg oral daily
- Aspirin 81 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Hypertension
- Mother: Chronic Kidney Disease, Cerebrovascular Accident
- Sister: Type 2 Diabetes Mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Blood pressure is 100/60 mmHg, heart rate is 105 beats per minute

PHYSICAL EXAMINATION:
On examination, she appears lethargic. Her mucous membranes are dry, and her skin turgor is decreased.

LABORATORY DATA:
Laboratory results are as follows:

- Serum creatinine: 2.1 mg/dL (increased from a baseline of 0.9 mg/dL)
- Blood urea nitrogen (BUN): 50 mg/dL
- Serum potassium: 4.2 mEq/L
- Serum sodium: 140 mEq/L
- Urine specific gravity: 1.030
- Urine sediment: unremarkable

## Encounter 2: source 2021-06-15, practice 2026-05-30, icu, Emergency Medicine

Chief complaint: Altered mental status and collapse during summer heat wave

ENCOUNTER NOTE
Date: 2021-06-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Altered mental status and collapse during summer heat wave

HISTORY OF PRESENT ILLNESS:
A 79-year-old female with HTN, T2DM, CKD, and prior pyelonephritis and AKI is brought by family to the emergency department during a heatwave after being found confused and lying on the floor at her home which lacks air conditioning.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute pyelonephritis (diagnosed 2020-01-15)
- Acute kidney injury (diagnosed 2020-06-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease

Her past medical history includes hypertension, type 2 diabetes mellitus, and chronic kidney disease.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (1995)
- Laparoscopic cholecystectomy (2004)
- Appendectomy (1962)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg oral daily
- Lisinopril 10 mg oral daily
- Metformin 500 mg oral twice daily
- Atorvastatin 20 mg oral daily
- Aspirin 81 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, Hypertension
- Mother: Chronic Kidney Disease, Cerebrovascular Accident
- Sister: Type 2 Diabetes Mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
On examination, her temperature is 41.1 C (106 F), heart rate is 118/min, respiratory rate is 26/min, and blood pressure is 90/60 mmHg

PHYSICAL EXAMINATION:
She is disoriented to time and place. Her skin is hot and dry to the touch.

LABORATORY DATA:
Laboratory studies are as follows:

- Sodium: 152 mEq/L
- Potassium: 4.0 mEq/L
- Creatinine: 2.1 mg/dL (baseline 1.5 mg/dL)
- Blood urea nitrogen: 40 mg/dL
- Platelet count: 90,000/mm^3
- Prothrombin time: 16 seconds (INR 1.4)
- Partial thromboplastin time: 40 seconds
- D-dimer: Elevated
- Arterial blood gas: pH 7.31, PaCO2 28 mmHg, PaO2 90 mmHg, HCO3 18 mEq/L

IMAGING:
Chest X-ray shows bilateral infiltrates consistent with pulmonary edema.
