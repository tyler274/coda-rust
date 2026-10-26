# synthetic_hospital patient 2201

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 71 at first encounter, sex M, Non-Hispanic White, Medicare Part A and B.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-09 (Wed) | ed | Emergency Medicine |
| 1 | 2021-01-15 | 2025-10-11 (Sat) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2026-03-31 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare Part A and B",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Rare, 1-2 glasses of wine on holidays",
 "family_history": [
  "Father: Myocardial infarction at age 66, hypertension",
  "Mother: Type 2 diabetes mellitus, stroke at age 74",
  "Older brother: Coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 8 years ago, 30 pack-year history",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO once daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg PO once daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "50 mg PO once daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "40 mg PO once daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO once daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Inguinal hernia repair (age 65)",
  "Cardiac catheterization with coronary angiography (age 70)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Heart Failure with Reduced Ejection Fraction",
  "Chronic Kidney Disease Stage 3a"
 ],
 "age_at_first_encounter": 71
}
```

Primary diagnoses: ["Acute Decompensated Heart Failure with Pulmonary Edema", "Acute decompensated heart failure", "Acute kidney injury"]
Comorbidities: ["Acute kidney injury", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2024-10-09, ed, Emergency Medicine

Chief complaint: Dizziness and decreased urine output for the past two days

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Dizziness and decreased urine output for the past two days

HISTORY OF PRESENT ILLNESS:
Over the last week, he has had several episodes of diarrhea and poor oral intake. He does not take nonsteroidal anti-inflammatory drugs or other nephrotoxic medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Heart Failure with Reduced Ejection Fraction
- Chronic Kidney Disease Stage 3a

He has a history of hypertension and type 2 diabetes mellitus but has otherwise been well.

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 65)
- Cardiac catheterization with coronary angiography (age 70)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO once daily
- Hydrochlorothiazide 25 mg PO once daily
- Metoprolol succinate 50 mg PO once daily
- Atorvastatin 40 mg PO once daily
- Aspirin 81 mg PO once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 66, hypertension
- Mother: Type 2 diabetes mellitus, stroke at age 74
- Older brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 30 pack-year history. Alcohol: Rare, 1-2 glasses of wine on holidays. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His blood pressure is 96/54 mmHg, pulse is 108/min, respirations are 20/min, and temperature is 36.9 °C (98.4 °F).

PHYSICAL EXAMINATION:
On arrival, he appears lethargic, with dry mucous membranes and delayed capillary refill. Cardiac and pulmonary examinations are unremarkable. Abdominal examination shows no tenderness or costovertebral angle pain.

LABORATORY DATA:
Laboratory testing demonstrates elevated creatinine.

## Encounter 1: source 2021-01-15, practice 2025-10-11, ed, Emergency Medicine

Chief complaint: Worsening shortness of breath, orthopnea, and bilateral lower extremity edema

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening shortness of breath, orthopnea, and bilateral lower extremity edema

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with hypertension, type 2 diabetes mellitus, heart failure with reduced ejection fraction, and chronic kidney disease stage 3a returns to this emergency department one year following an evaluation for acute kidney injury, with worsening shortness of breath, orthopnea, and bilateral lower extremity edema over the past week. He reports a weight gain of 5 kg during this period.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Heart Failure with Reduced Ejection Fraction
- Chronic Kidney Disease Stage 3a

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 65)
- Cardiac catheterization with coronary angiography (age 70)

MEDICATIONS:
His medications include metformin, lisinopril, and hydrochlorothiazide.

Home Medications:
- Metoprolol succinate 50 mg PO once daily
- Atorvastatin 40 mg PO once daily
- Aspirin 81 mg PO once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 66, hypertension
- Mother: Type 2 diabetes mellitus, stroke at age 74
- Older brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 30 pack-year history. Alcohol: Rare, 1-2 glasses of wine on holidays. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On physical examination, his blood pressure is 160/90 mmHg, heart rate is 102 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated, and auscultation of the lungs reveals bilateral crackles halfway up both lung fields. Cardiac examination shows an S3 gallop. Extremities are notable for 2+ pitting edema up to the mid-shins.

LABORATORY DATA:
Laboratory results are as follows:
- Serum sodium: 134 mEq/L
- Serum potassium: 5.2 mEq/L
- Serum creatinine: 1.8 mg/dL (baseline 1.2 mg/dL)
- B-type natriuretic peptide (BNP): 900 pg/mL

IMAGING:
- Chest X-ray: Cardiomegaly with cephalization of pulmonary vasculature and Kerley B lines

## Encounter 2: source 2021-07-15, practice 2026-03-31, ed, Emergency Medicine

Chief complaint: Acute onset of shortness of breath and a productive cough with pink, frothy sputum

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of shortness of breath and a productive cough with pink, frothy sputum

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with heart failure and chronic kidney disease, hospitalized six months ago for decompensated heart failure, returns to the emergency department with acute onset of shortness of breath and a productive cough with pink, frothy sputum that began earlier in the day. He reports a 2-week history of increasing lower extremity edema and paroxysmal nocturnal dyspnea.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Acute decompensated heart failure (diagnosed 2021-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Heart Failure with Reduced Ejection Fraction
- Chronic Kidney Disease Stage 3a

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 65)
- Cardiac catheterization with coronary angiography (age 70)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO once daily
- Hydrochlorothiazide 25 mg PO once daily
- Metoprolol succinate 50 mg PO once daily
- Atorvastatin 40 mg PO once daily
- Aspirin 81 mg PO once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 66, hypertension
- Mother: Type 2 diabetes mellitus, stroke at age 74
- Older brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 30 pack-year history. Alcohol: Rare, 1-2 glasses of wine on holidays. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On examination, he is tachypneic with a respiratory rate of 30 breaths per minute, blood pressure is 170/100 mmHg, heart rate is 110 beats per minute, and oxygen saturation is 88% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral rales halfway up both lung fields. Jugular venous pressure is elevated.

LABORATORY DATA:
Laboratory Values:
- Serum B-type natriuretic peptide (BNP): 900 pg/mL (normal <100 pg/mL)
- Serum creatinine: 1.4 mg/dL (normal 0.6-1.2 mg/dL)
- Blood urea nitrogen (BUN): 28 mg/dL (normal 7-20 mg/dL)
- Serum potassium: 4.5 mEq/L (normal 3.5-5.0 mEq/L)
- Arterial blood gas (ABG) on room air: pH 7.32, PaCO2 45 mmHg, PaO2 58 mmHg, HCO3 24 mEq/L

IMAGING:
The chest X-ray shows cardiomegaly and cephalization of pulmonary vasculature.

OTHER STUDIES:
An ECG indicates sinus tachycardia without ischemic changes.
