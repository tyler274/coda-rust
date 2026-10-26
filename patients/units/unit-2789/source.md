# synthetic_hospital patient 2789

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 48 at first encounter, sex F, Non-Hispanic White, Private insurance through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-31 (Thu) | follow_up | Nephrology |
| 1 | 2021-01-15 | 2026-08-05 (Wed) | outpatient | Nephrology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance through employer",
 "occupation": "Administrative Assistant",
 "alcohol_use": "Social (1-2 glasses of wine per week)",
 "family_history": [
  "Mother: Rheumatoid arthritis",
  "Father: Hypertension, chronic kidney disease",
  "Maternal aunt: Type 1 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "1000 mg IV every 6 months (maintenance infusion)",
   "name": "Rituximab"
  },
  {
   "dose": "600 mg PO TID with meals",
   "name": "Calcium carbonate"
  },
  {
   "dose": "2000 units PO daily",
   "name": "Cholecalciferol"
  },
  {
   "dose": "500 mg PO PRN pain (maximum 3 grams daily)",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Percutaneous kidney biopsy (3 months ago)",
  "Functional endoscopic sinus surgery (5 years ago)"
 ],
 "chronic_conditions": [
  "Granulomatosis with Polyangiitis (GPA)",
  "Chronic kidney disease, Stage 3b secondary to ANCA-associated vasculitis and prior chronic NSAID use",
  "Nephrogenic diabetes insipidus",
  "Chronic lower back pain",
  "History of diffuse alveolar hemorrhage (resolved)",
  "History of rapidly progressive glomerulonephritis (resolved)"
 ],
 "age_at_first_encounter": 48
}
```

Primary diagnoses: ["Chronic kidney disease", "Nephrogenic Diabetes Insipidus"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-07-31, follow_up, Nephrology

Chief complaint: Follow-up after hospitalization 3 months ago for hemoptysis and hematuria

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Nephrology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up after hospitalization 3 months ago for hemoptysis and hematuria

HISTORY OF PRESENT ILLNESS:
She was diagnosed with a systemic vasculitis and started on immunosuppressive therapy. Since discharge, she has been chronically fatigued, has lost 5 pounds, and reports vague joint pain in her knees and wrists. She also reports occasional constipation and has had two episodes of cramping abdominal pain in the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Granulomatosis with Polyangiitis (GPA)
- Chronic kidney disease, Stage 3b secondary to ANCA-associated vasculitis and prior chronic NSAID use
- Nephrogenic diabetes insipidus
- Chronic lower back pain
- History of diffuse alveolar hemorrhage (resolved)
- History of rapidly progressive glomerulonephritis (resolved)

PAST SURGICAL HISTORY:
- Percutaneous kidney biopsy (3 months ago)
- Functional endoscopic sinus surgery (5 years ago)

MEDICATIONS:
She takes prednisone and rituximab.

Home Medications:
- Calcium carbonate 600 mg PO TID with meals
- Cholecalciferol 2000 units PO daily
- Acetaminophen 500 mg PO PRN pain (maximum 3 grams daily)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Hypertension, chronic kidney disease
- Maternal aunt: Type 1 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Administrative Assistant.

REVIEW OF SYSTEMS:
No fevers, night sweats, or recent infections

VITAL SIGNS:
Vital signs are normal

PHYSICAL EXAMINATION:
Dry oral mucosa and mild pallor

LABORATORY DATA:
Stable renal insufficiency, unchanged from her prior admission. Stool guaiac testing is negative.

ASSESSMENT AND PLAN:
She notes an unexpected discrepancy between active and storage forms

She is started on a low-phosphate diet and calcium acetate. The physician orders a panel of vitamin D metabolites

## Encounter 1: source 2021-01-15, practice 2026-08-05, outpatient, Nephrology

Chief complaint: Chronic fatigue and increased thirst

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Patel, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic fatigue and increased thirst

HISTORY OF PRESENT ILLNESS:
She returns for follow-up one year after prior hospitalization for hemoptysis and hematuria. She also reports frequent urination, including waking up multiple times at night to void. She denies drinking excessive fluids intentionally but says she always feels thirsty.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic kidney disease (diagnosed 2020-01-15)
- Granulomatosis with Polyangiitis (GPA)
- Chronic kidney disease, Stage 3b secondary to ANCA-associated vasculitis and prior chronic NSAID use
- Nephrogenic diabetes insipidus
- Chronic lower back pain
- History of diffuse alveolar hemorrhage (resolved)
- History of rapidly progressive glomerulonephritis (resolved)

She has a long history of chronic back pain. Her medical history is otherwise unremarkable,.

PAST SURGICAL HISTORY:
- Percutaneous kidney biopsy (3 months ago)
- Functional endoscopic sinus surgery (5 years ago)

MEDICATIONS:
She has taken over-the-counter ibuprofen daily for the past 25 years. and she is not taking any prescription medications.

Home Medications:
- Prednisone 10 mg PO daily
- Rituximab 1000 mg IV every 6 months (maintenance infusion)
- Calcium carbonate 600 mg PO TID with meals
- Cholecalciferol 2000 units PO daily
- Acetaminophen 500 mg PO PRN pain (maximum 3 grams daily)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Hypertension, chronic kidney disease
- Maternal aunt: Type 1 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Administrative Assistant.

VITAL SIGNS:
Vitals are within normal limits,

PHYSICAL EXAMINATION:
Her physical exam is unremarkable.

LABORATORY DATA:
Laboratory testing shows serum sodium of 150 mEq/L and mild renal insufficiency.

OTHER STUDIES:
A water deprivation test is performed, followed by administration of exogenous antidiuretic hormone (ADH). Her urine output and urine osmolality are monitored during the test.
