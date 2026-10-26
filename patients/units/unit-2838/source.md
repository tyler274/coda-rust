# synthetic_hospital patient 2838

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex F, Non-Hispanic White, Medicare with supplemental Plan G.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-24 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2025-09-10 (Wed) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-02-25 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare with supplemental Plan G",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "Rare, less than 1 drink per month",
 "family_history": [
  "Mother with osteoporosis and hip fracture at age 82",
  "Father with hypertension and end-stage renal disease",
  "Paternal grandmother with gastroesophageal reflux disease and Barrett esophagus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 20 years ago, 8 pack-year history",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Omeprazole"
  },
  {
   "dose": "600 mg PO three times daily with meals",
   "name": "Calcium carbonate"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy, age 46",
  "Laparoscopic cholecystectomy, age 54"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Gastroesophageal reflux disease",
  "Chronic kidney disease, stage 3a",
  "Osteoporosis",
  "History of recurrent urinary tract infections"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Severe Clostridioides difficile infection without toxic megacolon", "Hypomagnesemia", "Milk-alkali syndrome"]
Comorbidities: ["Gastroesophageal reflux disease", "Essential hypertension", "Acute kidney injury"]

## Encounter 0: source 2020-01-15, practice 2025-02-24, outpatient, Internal Medicine

Chief complaint: Muscle cramps and weakness for the past two weeks, more pronounced in lower extremities and progressively worsening, without chest pain or palpitations.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Muscle cramps and weakness for the past two weeks, more pronounced in lower extremities and progressively worsening, without chest pain or palpitations.

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with complaints of muscle cramps and weakness for the past two weeks. She reports that the symptoms are more pronounced in her lower extremities and have been progressively worsening. She denies any chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Gastroesophageal reflux disease
- Chronic kidney disease, stage 3a
- Osteoporosis
- History of recurrent urinary tract infections

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy, age 46
- Laparoscopic cholecystectomy, age 54

MEDICATIONS:
Her current medications include lisinopril, hydrochlorothiazide, and omeprazole.

Home Medications:
- Calcium carbonate 600 mg PO three times daily with meals

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with osteoporosis and hip fracture at age 82
- Father with hypertension and end-stage renal disease
- Paternal grandmother with gastroesophageal reflux disease and Barrett esophagus

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago, 8 pack-year history. Alcohol: Rare, less than 1 drink per month. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination reveals normal vital signs, and no edema is noted. Deep tendon reflexes are diminished in the lower extremities.

LABORATORY DATA:
Laboratory results are as follows:

- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum calcium: 9.2 mg/dL
- Serum magnesium: 1.2 mg/dL (Normal range: 1.7-2.2 mg/dL)
- Serum creatinine: 1.0 mg/dL

OTHER STUDIES:
An ECG shows normal sinus rhythm with prolonged QT interval.

## Encounter 1: source 2020-07-15, practice 2025-09-10, ed, Emergency Medicine

Chief complaint: Two-week history of confusion, constipation, and generalized weakness in a patient with poor medication recall.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of confusion, constipation, and generalized weakness in a patient with poor medication recall.

HISTORY OF PRESENT ILLNESS:
This 68-year-old female with hypertension, GERD, CKD 3a, and osteoporosis, previously seen in January 2020 for muscle cramps, presents to the emergency department with a two-week history of confusion, constipation, and generalized weakness. She does not have a PCP and cannot recall her medication dosages.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypomagnesemia (diagnosed 2020-01-15)
- Hypertension
- Gastroesophageal reflux disease
- Chronic kidney disease, stage 3a
- Osteoporosis
- History of recurrent urinary tract infections

She has a past medical history of osteoporosis and chronic heartburn.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy, age 46
- Laparoscopic cholecystectomy, age 54

MEDICATIONS:
She takes over-the-counter calcium carbonate tablets regularly.

Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Omeprazole 20 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with osteoporosis and hip fracture at age 82
- Father with hypertension and end-stage renal disease
- Paternal grandmother with gastroesophageal reflux disease and Barrett esophagus

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago, 8 pack-year history. Alcohol: Rare, less than 1 drink per month. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Blood pressure is 140/85 mmHg, heart rate is 88 beats per minute

PHYSICAL EXAMINATION:
She is oriented to person only. Her abdomen is non-distended, non-tender, and bowel sounds are present.

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 12.5 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum creatinine: 2.1 mg/dL (normal: 0.6-1.1 mg/dL)
- Blood urea nitrogen (BUN): 45 mg/dL (normal: 7-20 mg/dL)
- Arterial blood gas (ABG): pH 7.52 (normal: 7.35-7.45), HCO3 32 mmol/L (normal: 22-26 mmol/L)
- Serum phosphorus: 2.9 mg/dL (normal: 2.5-4.5 mg/dL)

## Encounter 2: source 2021-01-15, practice 2026-02-25, ed, Emergency Medicine

Chief complaint: Three-day history of watery diarrhea, abdominal cramping, and low-grade fever with 5-6 unformed bowel movements per day.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three-day history of watery diarrhea, abdominal cramping, and low-grade fever with 5-6 unformed bowel movements per day.

HISTORY OF PRESENT ILLNESS:
The patient with history of milk-alkali syndrome and hypomagnesemia returns for evaluation. She reports having 5-6 unformed bowel movements per day.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypomagnesemia (diagnosed 2020-01-15)
- Milk-alkali syndrome (diagnosed 2020-07-15)
- Hypertension
- Gastroesophageal reflux disease
- Chronic kidney disease, stage 3a
- Osteoporosis
- History of recurrent urinary tract infections

Her past medical history includes hypertension, chronic kidney disease, and a recent hospitalization for a urinary tract infection, for which she completed a 7-day course of ciprofloxacin two weeks ago.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy, age 46
- Laparoscopic cholecystectomy, age 54

MEDICATIONS:
She has been taking omeprazole for the past year for gastroesophageal reflux disease.

Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Calcium carbonate 600 mg PO three times daily with meals

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with osteoporosis and hip fracture at age 82
- Father with hypertension and end-stage renal disease
- Paternal grandmother with gastroesophageal reflux disease and Barrett esophagus

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago, 8 pack-year history. Alcohol: Rare, less than 1 drink per month. Occupation: Retired elementary school teacher.

VITAL SIGNS:
On examination, her temperature is 38.2°C (100.8°F), blood pressure is 135/85 mmHg, and heart rate is 92 beats per minute.

PHYSICAL EXAMINATION:
Abdominal examination reveals mild diffuse tenderness without rebound or guarding.

LABORATORY DATA:
Her laboratory tests show:

- White blood cell count: 14,000/mm³ (normal: 4,500-11,000/mm³)
- Serum creatinine: 1.8 mg/dL (baseline 1.4 mg/dL)
- Negative stool cultures for Salmonella, Shigella, and Campylobacter
