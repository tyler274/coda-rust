# synthetic_hospital patient 2035

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex M, African American, Medicare (ESRD eligibility).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-26 (Sun) | ed | Emergency Medicine |
| 1 | 2021-01-15 | 2026-01-25 (Sun) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2026-07-22 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (ESRD eligibility)",
 "occupation": "Former forklift operator, currently on disability",
 "alcohol_use": "Occasional, 1-2 drinks per month",
 "family_history": [
  "Father with hypertension and myocardial infarction at age 62",
  "Mother with type 2 diabetes mellitus and ESRD",
  "Older brother with chronic kidney disease"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Former smoker, quit 8 years ago, 15 pack-year history",
 "home_medications": [
  {
   "dose": "4000 units IV three times weekly with hemodialysis",
   "name": "Epoetin alfa"
  },
  {
   "dose": "667 mg three times daily with meals",
   "name": "Calcium acetate"
  },
  {
   "dose": "10 mg daily",
   "name": "Amlodipine"
  },
  {
   "dose": "800 mg three times daily with meals",
   "name": "Sevelamer carbonate"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "50 mg daily",
   "name": "Metoprolol succinate"
  }
 ],
 "surgical_history": [
  "Left radiocephalic arteriovenous fistula creation",
  "Placement of tunneled right internal jugular dialysis catheter"
 ],
 "chronic_conditions": [
  "End-Stage Renal Disease on hemodialysis",
  "Hypertension",
  "Anemia of chronic kidney disease",
  "Secondary hyperparathyroidism"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["Uremic pericarditis", "Hyperkalemia"]
Comorbidities: ["End-stage renal disease on hemodialysis", "Anemia of chronic kidney disease", "Essential hypertension", "Hyperkalemia"]

## Encounter 0: source 2020-01-15, practice 2025-01-26, ed, Emergency Medicine

Chief complaint: Sharp, substernal chest pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sharp, substernal chest pain

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department with a 2-day history of sharp, substernal chest pain that worsens with deep breaths and lying flat. He reports a recent increase in fatigue and shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- End-Stage Renal Disease on hemodialysis
- Hypertension
- Anemia of chronic kidney disease
- Secondary hyperparathyroidism

PAST SURGICAL HISTORY:
- Left radiocephalic arteriovenous fistula creation
- Placement of tunneled right internal jugular dialysis catheter

MEDICATIONS:
Home Medications:
- Epoetin alfa 4000 units IV three times weekly with hemodialysis
- Calcium acetate 667 mg three times daily with meals
- Amlodipine 10 mg daily
- Sevelamer carbonate 800 mg three times daily with meals
- Aspirin 81 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and myocardial infarction at age 62
- Mother with type 2 diabetes mellitus and ESRD
- Older brother with chronic kidney disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 15 pack-year history. Alcohol: Occasional, 1-2 drinks per month. Occupation: Former forklift operator, currently on disability.

VITAL SIGNS:
His temperature is 37.2°C (99°F), blood pressure is 155/90 mmHg, heart rate is 98 bpm, and respiratory rate is 22 breaths/min.

PHYSICAL EXAMINATION:
Physical examination reveals a friction rub heard best at the left lower sternal border. There is no jugular venous distention, and his lungs are clear to auscultation. His extremities are without edema.

LABORATORY DATA:
Laboratory values are as follows:

- Creatinine: 6.5 mg/dL (normal: 0.6-1.3 mg/dL)
- Blood urea nitrogen (BUN): 72 mg/dL (normal: 7-20 mg/dL)
- Potassium: 5.2 mEq/L (normal: 3.5-5.0 mEq/L)
- Hemoglobin: 9.8 g/dL (normal: 13.8-17.2 g/dL for males)
- White blood cell count: 9,000/mm³ (normal: 4,500-11,000/mm³)
- Platelet count: 250,000/mm³ (normal: 150,000-450,000/mm³)

OTHER STUDIES:
An electrocardiogram (ECG) shows no ST-segment elevations or other abnormalities.

## Encounter 1: source 2021-01-15, practice 2026-01-25, ed, Emergency Medicine

Chief complaint: Weakness and palpitations

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Weakness and palpitations

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with end-stage renal disease on hemodialysis and prior uremic pericarditis presents with weakness and palpitations. He reports that his symptoms started a few hours ago and have progressively worsened. He has not been compliant with his hemodialysis sessions and missed his last two appointments.

PAST MEDICAL HISTORY:
Active Problem List:
- Uremic pericarditis (diagnosed 2020-01-15)
- End-Stage Renal Disease on hemodialysis
- Hypertension
- Anemia of chronic kidney disease
- Secondary hyperparathyroidism

PAST SURGICAL HISTORY:
- Left radiocephalic arteriovenous fistula creation
- Placement of tunneled right internal jugular dialysis catheter

MEDICATIONS:
Home Medications:
- Epoetin alfa 4000 units IV three times weekly with hemodialysis
- Calcium acetate 667 mg three times daily with meals
- Amlodipine 10 mg daily
- Sevelamer carbonate 800 mg three times daily with meals
- Aspirin 81 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and myocardial infarction at age 62
- Mother with type 2 diabetes mellitus and ESRD
- Older brother with chronic kidney disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 15 pack-year history. Alcohol: Occasional, 1-2 drinks per month. Occupation: Former forklift operator, currently on disability.

VITAL SIGNS:
His blood pressure is 150/90 mmHg, heart rate is 55 bpm

PHYSICAL EXAMINATION:
On examination, he appears lethargic

LABORATORY DATA:
Laboratory results are as follows:

- Serum potassium: 7.2 mEq/L
- Serum creatinine: 6.5 mg/dL
- Blood urea nitrogen (BUN): 85 mg/dL
- Serum sodium: 135 mEq/L
- Serum calcium: 9.0 mg/dL
- Serum bicarbonate: 20 mEq/L

OTHER STUDIES:
An ECG shows peaked T waves, a widened QRS complex, and a prolonged PR interval.

## Encounter 2: source 2021-07-15, practice 2026-07-22, ed, Emergency Medicine

Chief complaint: Shortness of breath and chest pain

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Shortness of breath and chest pain

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with end-stage renal disease on hemodialysis and prior uremic pericarditis in January 2020 presents to the emergency department with complaints of shortness of breath and chest pain that is worse when lying flat and improves when sitting up. He denies any recent trauma or illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Uremic pericarditis (diagnosed 2020-01-15)
- Hyperkalemia (diagnosed 2021-01-15)
- End-Stage Renal Disease on hemodialysis
- Hypertension
- Anemia of chronic kidney disease
- Secondary hyperparathyroidism

PAST SURGICAL HISTORY:
- Left radiocephalic arteriovenous fistula creation
- Placement of tunneled right internal jugular dialysis catheter

MEDICATIONS:
He takes erythropoietin and calcium acetate.

Home Medications:
- Epoetin alfa 4000 units IV three times weekly with hemodialysis
- Amlodipine 10 mg daily
- Sevelamer carbonate 800 mg three times daily with meals
- Aspirin 81 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and myocardial infarction at age 62
- Mother with type 2 diabetes mellitus and ESRD
- Older brother with chronic kidney disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 15 pack-year history. Alcohol: Occasional, 1-2 drinks per month. Occupation: Former forklift operator, currently on disability.

VITAL SIGNS:
His blood pressure is 110/70 mmHg, heart rate is 110 bpm, and respiratory rate is 22 breaths per minute

PHYSICAL EXAMINATION:
His jugular venous pressure is elevated, and a pericardial friction rub is auscultated. Lung examination is clear.

LABORATORY DATA:
Hemoglobin: 9.8 g/dL
Creatinine: 6.2 mg/dL
Blood Urea Nitrogen (BUN): 54 mg/dL
Potassium: 5.1 mEq/L
C-reactive protein (CRP): 12 mg/L

IMAGING:
A chest x-ray reveals an enlarged cardiac silhouette.

OTHER STUDIES:
The ECG shows sinus tachycardia with electrical alternans.
