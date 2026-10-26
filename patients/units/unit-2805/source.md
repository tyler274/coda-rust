# synthetic_hospital patient 2805

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex F, Non-Hispanic White, Employer-sponsored preferred provider organization (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-08-04 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2026-01-29 (Thu) | outpatient | Gastroenterology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin (anaphylaxis)",
  "Iodinated contrast (urticaria)"
 ],
 "insurance": "Employer-sponsored preferred provider organization (PPO)",
 "occupation": "High school administrative assistant",
 "alcohol_use": "Social, 2-3 glasses of wine per month",
 "family_history": [
  "Father with colorectal cancer diagnosed at age 67",
  "Paternal grandfather with colorectal cancer",
  "Mother with osteoporosis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 8 years ago, 15 pack-year history",
 "home_medications": [
  {
   "dose": "5 mg oral daily",
   "name": "Amlodipine"
  },
  {
   "dose": "20 mg oral daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "500 mg oral as needed for pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Laparoscopic right hemicolectomy with primary anastomosis (5 months prior)",
  "Cholecystectomy (age 42)",
  "Appendectomy (age 16)"
 ],
 "chronic_conditions": [
  "Stage III colorectal adenocarcinoma",
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Antibiotic-associated diarrhea due to clindamycin"]
Comorbidities: ["Colorectal neoplasia"]

## Encounter 0: source 2020-01-15, practice 2025-08-04, outpatient, Internal Medicine

Chief complaint: Evaluation of colon cancer risk and screening test interpretation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of colon cancer risk and screening test interpretation

HISTORY OF PRESENT ILLNESS:
She undergoes a new screening test with sensitivity of 92% and specificity of 88%. In Population X, the prevalence of colon cancer is 3%, while in Population Y, it is 30%. The same test is used for both populations, and each population has 1,000 individuals. You are tasked with interpreting the negative predictive value (NPV) of the test for both populations. Consider the following 2x2 tables for Population X: true negatives = 854, false negatives = 2; and for Population Y: true negatives = 616, false negatives = 24.

PAST MEDICAL HISTORY:
Active Problem List:
- Stage III colorectal adenocarcinoma
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Laparoscopic right hemicolectomy with primary anastomosis (5 months prior)
- Cholecystectomy (age 42)
- Appendectomy (age 16)

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg oral daily
- Atorvastatin 20 mg oral daily
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
Penicillin (anaphylaxis), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Father with colorectal cancer diagnosed at age 67
- Paternal grandfather with colorectal cancer
- Mother with osteoporosis

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 15 pack-year history. Alcohol: Social, 2-3 glasses of wine per month. Occupation: High school administrative assistant.

## Encounter 1: source 2020-07-15, practice 2026-01-29, outpatient, Gastroenterology

Chief complaint: Two-week history of voluminous watery diarrhea during chemotherapy

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-week history of voluminous watery diarrhea during chemotherapy

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old female with Stage III colorectal adenocarcinoma, hypertension, and hyperlipidemia who returns for evaluation of new symptoms during ongoing chemotherapy. She completed her fourth cycle of chemotherapy 3 weeks ago. She reports having six to eight episodes of voluminous, watery, brown stools daily, which continue despite not eating overnight. She denies recent travel, sick contacts, or consumption of unusual foods. Her appetite has decreased, and she has lost 3 kg since her last visit. She denies fever, bloody stools, or abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Stage III colorectal adenocarcinoma
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Laparoscopic right hemicolectomy with primary anastomosis (5 months prior)
- Cholecystectomy (age 42)
- Appendectomy (age 16)

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg oral daily
- Atorvastatin 20 mg oral daily
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
Penicillin (anaphylaxis), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Father with colorectal cancer diagnosed at age 67
- Paternal grandfather with colorectal cancer
- Mother with osteoporosis

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 15 pack-year history. Alcohol: Social, 2-3 glasses of wine per month. Occupation: High school administrative assistant.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for mild diffuse abdominal tenderness without rebound or guarding.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 12.1 g/dL
- White blood cell count: 5,000/uL
- Platelets: 250,000/uL
- Serum sodium: 140 mEq/L
- Serum potassium: 3.5 mEq/L
- Serum creatinine: 1.0 mg/dL
- Stool Clostridioides difficile toxin: Negative
- Fecal occult blood test: Negative
