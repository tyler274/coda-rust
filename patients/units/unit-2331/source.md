# synthetic_hospital patient 2331

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 64 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield PPO (transitioning to Medicare).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-09-08 (Sun) | inpatient | Cardiology |
| 1 | 2021-07-15 | 2026-02-23 (Mon) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin (anaphylaxis)",
  "Sulfa drugs (rash)"
 ],
 "insurance": "Blue Cross Blue Shield PPO (transitioning to Medicare)",
 "occupation": "Retired police officer",
 "alcohol_use": "Occasional, 1-2 drinks per week",
 "family_history": [
  "Father: Myocardial infarction at age 68, Type 2 diabetes mellitus",
  "Mother: Hip fracture due to osteoporosis at age 75",
  "Brother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit at age 60",
 "home_medications": [
  {
   "dose": "5 mg daily (adjusted for INR)",
   "name": "Warfarin sodium"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "70 mg weekly",
   "name": "Alendronate"
  },
  {
   "dose": "600 mg three times daily",
   "name": "Calcium carbonate"
  },
  {
   "dose": "2000 units daily",
   "name": "Cholecalciferol (Vitamin D3)"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Mechanical aortic valve replacement (age 62)",
  "Left knee arthroscopy (age 45)",
  "Tonsillectomy (age 8)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Status post mechanical aortic valve replacement",
  "Osteoporosis",
  "Dyslipidemia"
 ],
 "age_at_first_encounter": 64
}
```

Primary diagnoses: ["Prerenal acute kidney injury secondary to hypovolemia"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-09-08, inpatient, Cardiology

Chief complaint: Hospitalized for management of infective endocarditis

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Hospitalized for management of infective endocarditis

HISTORY OF PRESENT ILLNESS:
On hospital day 5, his urine output declines significantly, and labs show a rising creatinine from a baseline of 0.8 mg/dL to 2.1 mg/dL.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Status post mechanical aortic valve replacement
- Osteoporosis
- Dyslipidemia

PAST SURGICAL HISTORY:
- Mechanical aortic valve replacement (age 62)
- Left knee arthroscopy (age 45)
- Tonsillectomy (age 8)

MEDICATIONS:
He was started empirically on intravenous vancomycin and gentamicin.

Home Medications:
- Warfarin sodium 5 mg daily (adjusted for INR)
- Metformin 1000 mg twice daily
- Lisinopril 10 mg daily
- Atorvastatin 40 mg daily
- Alendronate 70 mg weekly
- Calcium carbonate 600 mg three times daily
- Cholecalciferol (Vitamin D3) 2000 units daily
- Aspirin 81 mg daily

ALLERGIES:
Penicillin (anaphylaxis), Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, Type 2 diabetes mellitus
- Mother: Hip fracture due to osteoporosis at age 75
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit at age 60. Alcohol: Occasional, 1-2 drinks per week. Occupation: Retired police officer.

VITAL SIGNS:
His temperature is 37.2°C (99.0°F), blood pressure is 124/76 mm Hg, and pulse is 84/min.

PHYSICAL EXAMINATION:
He has no rash, edema, or signs of volume overload.

LABORATORY DATA:
Urinalysis is bland, and there is no mention of hematuria or proteinuria.

## Encounter 1: source 2021-07-15, practice 2026-02-23, inpatient, Internal Medicine

Chief complaint: Fever, productive cough, and shortness of breath

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Fever, productive cough, and shortness of breath

HISTORY OF PRESENT ILLNESS:
This 68-year-old man with mechanical aortic valve replacement, Osteoporosis, and 2020 hospitalization for infective endocarditis is admitted to the hospital for management of a complicated femoral fracture. On hospital day 5, he develops fever, productive cough, and shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Prerenal acute kidney injury secondary to hypovolemia (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Status post mechanical aortic valve replacement
- Osteoporosis
- Dyslipidemia

PAST SURGICAL HISTORY:
- Mechanical aortic valve replacement (age 62)
- Left knee arthroscopy (age 45)
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Warfarin sodium 5 mg daily (adjusted for INR)
- Metformin 1000 mg twice daily
- Lisinopril 10 mg daily
- Atorvastatin 40 mg daily
- Alendronate 70 mg weekly
- Calcium carbonate 600 mg three times daily
- Cholecalciferol (Vitamin D3) 2000 units daily
- Aspirin 81 mg daily

ALLERGIES:
Penicillin (anaphylaxis), Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, Type 2 diabetes mellitus
- Mother: Hip fracture due to osteoporosis at age 75
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit at age 60. Alcohol: Occasional, 1-2 drinks per week. Occupation: Retired police officer.

VITAL SIGNS:
Temperature is 38.5°C (101.3°F), blood pressure is 128/76 mm Hg, pulse is 98/min, respiratory rate is 20/min, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Chest auscultation reveals crackles in the right lower lung field.

LABORATORY DATA:
Sputum culture grows Pseudomonas aeruginosa .
Prior to therapy, his serum creatinine was 0.9 mg/dL; on day 7 of treatment, it rises to 2.1 mg/dL. Serum potassium is 5.2 mEq/L.

IMAGING:
Chest x-ray shows a new right lower lobe infiltrate

PLAN:
The patient is started on a combination of antibiotics.
