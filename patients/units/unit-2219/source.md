# synthetic_hospital patient 2219

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-05 (Thu) | ed | Emergency Medicine |
| 1 | 2020-10-15 | 2025-09-04 (Thu) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2026-05-30 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfa drugs (rash)"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "Occasional social use, 1-2 drinks per month",
 "family_history": [
  "Father: Myocardial infarction at age 70, hypertension",
  "Mother: Type 2 diabetes mellitus, chronic heart failure, died at age 81",
  "Brother: Hypertension, coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 8 years ago, 25 pack-year history",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "Appendectomy (age 22)",
  "Right knee arthroscopy (age 54)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 diabetes mellitus",
  "Chronic heart failure with reduced ejection fraction",
  "Obesity",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Acute decompensated heart failure"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2024-12-05, ed, Emergency Medicine

Chief complaint: progressive dyspnea on exertion and bilateral lower extremity edema

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Ananya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive dyspnea on exertion and bilateral lower extremity edema

HISTORY OF PRESENT ILLNESS:
2-month history of progressive dyspnea on exertion and bilateral lower extremity edema. He denies chest pain, fever, or cough.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 diabetes mellitus
- Chronic heart failure with reduced ejection fraction
- Obesity
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Appendectomy (age 22)
- Right knee arthroscopy (age 54)

MEDICATIONS:
Metformin, lisinopril, and hydrochlorothiazide

ALLERGIES:
Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Myocardial infarction at age 70, hypertension
- Mother: Type 2 diabetes mellitus, chronic heart failure, died at age 81
- Brother: Hypertension, coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired postal worker.

VITAL SIGNS:
Blood pressure is 145/90 mmHg, heart rate is 88 beats per minute, and respiratory rate is 20 breaths per minute

PHYSICAL EXAMINATION:
Cardiac auscultation reveals an S3 gallop, and there is jugular venous distension. Pulmonary examination shows decreased breath sounds at the bases bilaterally without crackles. There is 2+ pitting edema up to the mid-shins.

LABORATORY DATA:
Thoracentesis is performed, and pleural fluid analysis reveals the following:

- Pleural fluid protein: 1.5 g/dL
- Serum protein: 6.8 g/dL
- Pleural fluid LDH: 110 U/L
- Serum LDH: 200 U/L
- Pleural fluid pH: 7.45

IMAGING:
Chest X-ray demonstrates small bilateral pleural effusions.

## Encounter 1: source 2020-10-15, practice 2025-09-04, ed, Emergency Medicine

Chief complaint: progressive dyspnea on exertion and orthopnea

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive dyspnea on exertion and orthopnea

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with chronic heart failure with reduced ejection fraction returns to the ED nine months after prior visit for dyspnea with a 2-week history of progressive dyspnea on exertion and orthopnea. He also reports a recent increase in his waist circumference despite no significant change in his diet or weight.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute decompensated heart failure (diagnosed 2020-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Chronic heart failure with reduced ejection fraction
- Obesity
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Appendectomy (age 22)
- Right knee arthroscopy (age 54)

MEDICATIONS:
His medications include metformin, lisinopril, and hydrochlorothiazide.

ALLERGIES:
Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Myocardial infarction at age 70, hypertension
- Mother: Type 2 diabetes mellitus, chronic heart failure, died at age 81
- Brother: Hypertension, coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired postal worker.

VITAL SIGNS:
On physical examination, his blood pressure is 150/90 mmHg, heart rate is 102 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Examination reveals jugular venous distension, bilateral lung crackles, and pitting edema of the lower extremities. An S3 heart sound is auscultated.

LABORATORY DATA:
Laboratory results are as follows:

- Serum B-type Natriuretic Peptide (BNP): 900 pg/mL (normal <100 pg/mL)
- Serum creatinine: 1.2 mg/dL
- Serum potassium: 4.5 mEq/L
- Serum sodium: 140 mEq/L
- Fasting blood glucose: 160 mg/dL

IMAGING:
Chest X-ray shows cardiomegaly and pulmonary congestion.

OTHER STUDIES:
Electrocardiogram (ECG) demonstrates left ventricular hypertrophy.

## Encounter 2: source 2021-07-15, practice 2026-05-30, ed, Emergency Medicine

Chief complaint: progressive shortness of breath, orthopnea, and bilateral lower extremity swelling

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath, orthopnea, and bilateral lower extremity swelling

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with chronic heart failure returns to the emergency department for recurrent symptoms after two similar episodes, presenting with a 2-day history of progressive shortness of breath, orthopnea, and bilateral lower extremity swelling. He denies any recent fever, cough, or contact with sick individuals.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute decompensated heart failure (diagnosed 2020-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Chronic heart failure with reduced ejection fraction
- Obesity
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Appendectomy (age 22)
- Right knee arthroscopy (age 54)

MEDICATIONS:
Metformin, lisinopril, and hydrochlorothiazide

ALLERGIES:
Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Myocardial infarction at age 70, hypertension
- Mother: Type 2 diabetes mellitus, chronic heart failure, died at age 81
- Brother: Hypertension, coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired postal worker.

VITAL SIGNS:
Blood pressure is 160/90 mmHg, heart rate is 102 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 88% on room air

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated. Auscultation of the lungs reveals bilateral rales halfway up both lung fields. Cardiac examination shows an S3 gallop. Extremities are notable for 2+ pitting edema up to the mid-shins.

LABORATORY DATA:
Serum sodium: 138 mEq/L
- Serum potassium: 4.5 mEq/L
- Serum creatinine: 1.2 mg/dL
- B-type natriuretic peptide (BNP): 900 pg/mL
- White blood cell count: 7,500 cells/mm^3
- Hemoglobin: 14 g/dL
- Platelet count: 250,000 cells/mm^3

IMAGING:
Chest x-ray demonstrates bilateral pulmonary infiltrates.
