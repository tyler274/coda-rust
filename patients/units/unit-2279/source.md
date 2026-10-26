# synthetic_hospital patient 2279

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 62 at first encounter, sex M, Non-Hispanic White, Private insurance (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-08 (Tue) | ed | Emergency Medicine |
| 1 | 2020-01-15 | 2025-07-17 (Thu) | icu | Critical Care Medicine |
| 2 | 2020-07-15 | 2026-01-06 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance (employer-sponsored)",
 "occupation": "Regional Sales Manager",
 "alcohol_use": "Occasional (1-2 drinks per week)",
 "family_history": [
  "Father: Type 2 diabetes mellitus with diabetic nephropathy, deceased at age 68 from myocardial infarction",
  "Mother: Hypertension, chronic kidney disease stage 3",
  "Sibling: Elder brother with coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 40 pack-year history",
 "home_medications": [
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin Glargine"
  },
  {
   "dose": "Sliding scale with meals",
   "name": "Insulin Lispro"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (2005)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Chronic kidney disease stage 3b",
  "Hypertension",
  "Dyslipidemia"
 ],
 "age_at_first_encounter": 62
}
```

Primary diagnoses: ["Candidemia", "Legionnaires' disease"]
Comorbidities: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Type 2 diabetes mellitus", "Tobacco use disorder"]

## Encounter 0: source 2020-01-15, practice 2025-07-08, ed, Emergency Medicine

Chief complaint: Two days of fever, watery diarrhea, and fatigue, followed by dry cough and mild shortness of breath after recent hotel stay

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two days of fever, watery diarrhea, and fatigue, followed by dry cough and mild shortness of breath after recent hotel stay

HISTORY OF PRESENT ILLNESS:
He recently returned from a 4-day stay at a hotel for a business convention.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Chronic kidney disease stage 3b
- Hypertension
- Dyslipidemia

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2005)

MEDICATIONS:
Home Medications:
- Insulin Glargine 24 units subcutaneous at bedtime
- Insulin Lispro Sliding scale with meals
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus with diabetic nephropathy, deceased at age 68 from myocardial infarction
- Mother: Hypertension, chronic kidney disease stage 3
- Sibling: Elder brother with coronary artery disease

SOCIAL HISTORY:
40-pack-year smoking history

VITAL SIGNS:
Temperature is 39.1°C (102.4°F), BP is 126/76 mmHg, pulse is 98/min, and O2 saturation is 94% on room air

PHYSICAL EXAMINATION:
Pulmonary auscultation reveals crackles in the right lower lung field

LABORATORY DATA:
Sodium 126 mEq/L, AST 68 U/L, ALT 70 U/L, and WBC count 15,400/mm³ with neutrophil predominance

IMAGING:
Chest x-ray shows patchy infiltrates in the right middle and lower lobes

OTHER STUDIES:
Sputum Gram stain reveals many segmented neutrophils but no visible organisms

## Encounter 1: source 2020-01-15, practice 2025-07-17, icu, Critical Care Medicine

Chief complaint: New fever of 38.9°C, tachycardia, and hypotension on day 10 of hospitalization despite initial improvement

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. James Okonkwo, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
New fever of 38.9°C, tachycardia, and hypotension on day 10 of hospitalization despite initial improvement

HISTORY OF PRESENT ILLNESS:
A 62-year-old male with T2DM, CKD stage 3b, HTN, and dyslipidemia presents after evaluation for fever, diarrhea, and respiratory symptoms. He has been on broad-spectrum antibiotics, requiring central venous catheter placement for vasopressor support. On day 10 of admission he develops new fever of 38.9°C (102°F), tachycardia, and hypotension despite initial improvement.

PAST MEDICAL HISTORY:
Active Problem List:
- Legionnaires' disease (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Chronic kidney disease stage 3b
- Hypertension
- Dyslipidemia

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2005)

MEDICATIONS:
Home Medications:
- Insulin Glargine 24 units subcutaneous at bedtime
- Insulin Lispro Sliding scale with meals
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus with diabetic nephropathy, deceased at age 68 from myocardial infarction
- Mother: Hypertension, chronic kidney disease stage 3
- Sibling: Elder brother with coronary artery disease

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history. Alcohol: Occasional (1-2 drinks per week). Occupation: Regional Sales Manager.

PHYSICAL EXAMINATION:
His physical examination is notable for no new focal signs of infection.

LABORATORY DATA:
Laboratory studies show:

- White blood cell count: 14,000/mm³
- Hemoglobin: 10.2 g/dL
- Platelet count: 100,000/mm³
- Creatinine: 2.1 mg/dL (elevated from baseline of 1.6 mg/dL)
- Blood cultures are drawn and within 48 hours, all bottles grow Candida albicans.

## Encounter 2: source 2020-07-15, practice 2026-01-06, ed, Emergency Medicine

Chief complaint: Two-day history of fever, productive cough, and progressive shortness of breath

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of fever, productive cough, and progressive shortness of breath

PAST MEDICAL HISTORY:
Active Problem List:
- Legionnaires' disease (diagnosed 2020-01-15)
- Candidemia (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Chronic kidney disease stage 3b
- Hypertension
- Dyslipidemia

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2005)

MEDICATIONS:
Home Medications:
- Insulin Glargine 24 units subcutaneous at bedtime
- Insulin Lispro Sliding scale with meals
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus with diabetic nephropathy, deceased at age 68 from myocardial infarction
- Mother: Hypertension, chronic kidney disease stage 3
- Sibling: Elder brother with coronary artery disease

SOCIAL HISTORY:
Smoking

VITAL SIGNS:
Temperature 100 F, BP 120/80, HR 90, RR 20

PHYSICAL EXAMINATION:
Bronchial breath sounds are auscultated and there is dullness to percussion over the right lower lung field

IMAGING:
Chest x-ray shows a right lower lobe consolidation
