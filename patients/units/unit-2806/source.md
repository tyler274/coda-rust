# synthetic_hospital patient 2806

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex F, Non-Hispanic White, Commercial PPO (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-12-23 (Tue) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2026-06-22 (Mon) | outpatient | Endocrinology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO (employer-sponsored)",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "Occasional social use, 1-2 glasses of wine per month",
 "family_history": [
  "Mother: Hashimoto thyroiditis",
  "Sister: Celiac disease",
  "Father: Osteoporotic hip fracture at age 72"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Tonsillectomy (age 8)",
  "Cesarean section (age 30)"
 ],
 "chronic_conditions": [
  "Celiac disease"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Osteomalacia"]
Comorbidities: ["Celiac disease", "Secondary hyperparathyroidism", "Bile Acid Diarrhea", "Vitamin D deficiency"]

## Encounter 0: source 2020-01-15, practice 2025-12-23, outpatient, Internal Medicine

Chief complaint: Six-month history of diffuse bone pain and muscle weakness with difficulty climbing stairs.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of diffuse bone pain and muscle weakness with difficulty climbing stairs.

HISTORY OF PRESENT ILLNESS:
A 55-year-old female presents to the clinic with a 6-month history of diffuse bone pain and muscle weakness. She reports feeling fatigued and has noticed difficulty climbing stairs and getting up from a seated position.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease

She has a history of celiac disease and follows a strict gluten-free diet.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)
- Cesarean section (age 30)

MEDICATIONS:
She does not take any supplements.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis
- Sister: Celiac disease
- Father: Osteoporotic hip fracture at age 72

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use, 1-2 glasses of wine per month. Occupation: High school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, there is tenderness to palpation over her ribs and lumbar spine.

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 8.2 mg/dL (Normal: 8.6-10.2 mg/dL)
- Serum phosphorus: 2.1 mg/dL (Normal: 2.5-4.5 mg/dL)
- Alkaline phosphatase: 140 U/L (Normal: 30-120 U/L)
- 25-hydroxyvitamin D: 12 ng/mL (Normal: 30-100 ng/mL)
- Parathyroid hormone (PTH): Elevated

## Encounter 1: source 2020-07-15, practice 2026-06-22, outpatient, Endocrinology

Chief complaint: Persistent bone pain localized to hips and lower back with progressive muscle weakness.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Okafor, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent bone pain localized to hips and lower back with progressive muscle weakness.

HISTORY OF PRESENT ILLNESS:
This patient with Celiac disease returns for follow-up of symptoms initially evaluated in January 2020. She reports difficulty climbing stairs and getting up from a seated position.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteomalacia (diagnosed 2020-01-15)
- Celiac disease

She has a history of celiac disease and follows a strict gluten-free diet. However, she admits to occasional dietary lapses over the past year.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)
- Cesarean section (age 30)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis
- Sister: Celiac disease
- Father: Osteoporotic hip fracture at age 72

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use, 1-2 glasses of wine per month. Occupation: High school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, she has tenderness over her lumbar spine and pelvis. Her muscle strength is 3/5 in the proximal muscle groups of the lower extremities.

LABORATORY DATA:
Laboratory findings are as follows:

- Serum calcium: 8.0 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum phosphate: 2.1 mg/dL (Normal: 2.5-4.5 mg/dL)
- Serum alkaline phosphatase: 140 U/L (Normal: 30-120 U/L)
- Parathyroid hormone (PTH): Elevated
- 25-hydroxyvitamin D: Low

IMAGING:
An X-ray of the pelvis reveals diffuse cortical thinning and a "Looser's zone" or pseudofracture on the femoral neck.
