# synthetic_hospital patient 2282

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-03 (Wed) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2025-02-19 (Wed) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2026-01-02 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal service supervisor",
 "alcohol_use": "None",
 "family_history": [
  "Father: Type 2 diabetes mellitus, coronary artery disease",
  "Mother: Hypertension, chronic kidney disease",
  "Paternal uncle: End-stage renal disease on hemodialysis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 8 years ago",
 "home_medications": [
  {
   "dose": "22 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "500 mg orally twice daily",
   "name": "Metformin"
  },
  {
   "dose": "10 mg orally daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg orally daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg orally daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Cataract extraction with intraocular lens placement, right eye (2021)",
  "Inguinal hernia repair, left side (2018)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Chronic kidney disease Stage 3b",
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Hyperkalemia", "Septic arthritis of the knee", "Broad-spectrum antibiotic prophylaxis (piperacillin-tazobactam)"]
Comorbidities: ["Chronic kidney disease", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2024-07-03, ed, Emergency Medicine

Chief complaint: Fever, chills, and constant right lower quadrant abdominal pain for 2 days.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, chills, and constant right lower quadrant abdominal pain for 2 days.

HISTORY OF PRESENT ILLNESS:
He reports that the pain is constant and has been worsening. He denies nausea, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Chronic kidney disease Stage 3b
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, right eye (2021)
- Inguinal hernia repair, left side (2018)

MEDICATIONS:
Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Metformin 500 mg orally twice daily
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Aspirin 81 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, coronary artery disease
- Mother: Hypertension, chronic kidney disease
- Paternal uncle: End-stage renal disease on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: None. Occupation: Retired postal service supervisor.

VITAL SIGNS:
His temperature is 38.7°C (101.7°F), blood pressure is 145/90 mmHg, pulse is 98/min, and respiratory rate is 20/min.

PHYSICAL EXAMINATION:
Physical examination reveals tenderness to palpation in the right lower quadrant with guarding but no rebound tenderness.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 14,000/µL
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/µL
- Serum creatinine: 2.1 mg/dL (baseline 1.8 mg/dL)
- Blood urea nitrogen: 45 mg/dL
- C-reactive protein: 18 mg/L

IMAGING:
A CT scan of the abdomen and pelvis with contrast shows a thickened cecal wall and periappendiceal fat stranding.

ASSESSMENT AND PLAN:
Despite initial improvement, on hospital day 3, the patient develops a new fever and the surgical site appears erythematous and warm. Hand hygiene compliance on the unit is noted to be suboptimal.

The patient is admitted for management of acute appendicitis and started on intravenous antibiotics.

## Encounter 1: source 2020-09-15, practice 2025-02-19, ed, Emergency Medicine

Chief complaint: Three-day history of fever, chills, and acute right knee pain without trauma.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three-day history of fever, chills, and acute right knee pain without trauma.

HISTORY OF PRESENT ILLNESS:
This 65-year-old male with Type 2 diabetes mellitus, CKD Stage 3b, and prior ED visit for febrile illness in January 2020 presents to the emergency department with a 3-day history of fever, chills, and right knee pain. He denies recent trauma or insect bites. Patient was started on empiric vancomycin 2 days ago with no improvement in symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Broad-spectrum antibiotic prophylaxis (piperacillin-tazobactam) (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Chronic kidney disease Stage 3b
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, right eye (2021)
- Inguinal hernia repair, left side (2018)

MEDICATIONS:
Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Metformin 500 mg orally twice daily
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Aspirin 81 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, coronary artery disease
- Mother: Hypertension, chronic kidney disease
- Paternal uncle: End-stage renal disease on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: None. Occupation: Retired postal service supervisor.

VITAL SIGNS:
His temperature is 38.9°C (102°F), blood pressure is 135/85 mmHg, pulse is 110/min, and respiratory rate is 20/min

PHYSICAL EXAMINATION:
Physical examination reveals a swollen, erythematous, and warm right knee with limited range of motion due to pain.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 14,000/uL
- Erythrocyte sedimentation rate (ESR): 70 mm/hr
- C-reactive protein (CRP): 90 mg/L
- Blood cultures: No growth after 48 hours
- Synovial fluid analysis:
- WBC: 50,000/uL with 90% neutrophils
- Gram stain: No organisms seen
- Culture: Pending

## Encounter 2: source 2021-07-15, practice 2026-01-02, ed, Emergency Medicine

Chief complaint: Acute confusion and dizziness reported by wife over the past two days.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute confusion and dizziness reported by wife over the past two days.

HISTORY OF PRESENT ILLNESS:
This 66-year-old male with Type 2 diabetes mellitus, chronic kidney disease Stage 3b, and prior emergency department visits for intra-abdominal infection and septic arthritis presents with acute confusion and dizziness. His wife reports that he has been less responsive and not eating well for the past two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Broad-spectrum antibiotic prophylaxis (piperacillin-tazobactam) (diagnosed 2020-01-15)
- Septic arthritis of the knee (diagnosed 2020-09-15)
- Type 2 diabetes mellitus
- Chronic kidney disease Stage 3b
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, right eye (2021)
- Inguinal hernia repair, left side (2018)

MEDICATIONS:
His medications include insulin glargine, metformin, and lisinopril.

Home Medications:
- Atorvastatin 40 mg orally daily
- Aspirin 81 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, coronary artery disease
- Mother: Hypertension, chronic kidney disease
- Paternal uncle: End-stage renal disease on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: None. Occupation: Retired postal service supervisor.

VITAL SIGNS:
On examination, his blood pressure is 95/60 mmHg, heart rate is 58 bpm

PHYSICAL EXAMINATION:
He is oriented to person only. His skin is warm and dry.

LABORATORY DATA:
Laboratory results are as follows:

- Sodium: 135 mEq/L
- Potassium: 5.9 mEq/L
- Chloride: 98 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 44 mg/dL
- Creatinine: 2.8 mg/dL (baseline 1.9 mg/dL)
- Glucose: 58 mg/dL

OTHER STUDIES:
An ECG shows peaked T waves.
