# synthetic_hospital patient 2793

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 37 at first encounter, sex F, White, Non-Hispanic, Blue Cross Blue Shield (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-23 (Wed) | follow_up | Endocrinology |
| 1 | 2020-02-15 | 2025-08-22 (Fri) | outpatient | Endocrinology |
| 2 | 2020-07-15 | 2026-01-24 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield (PPO)",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother: Autoimmune hypothyroidism",
  "Father: Hypertension, type 2 diabetes mellitus",
  "Paternal aunt: Breast cancer (age 62)",
  "No family history of thyroid malignancy or parathyroid disease"
 ],
 "race_ethnicity": "White, Non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "125 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "600 mg PO TID with meals",
   "name": "Calcium carbonate"
  },
  {
   "dose": "0.25 mcg PO daily",
   "name": "Calcitriol"
  },
  {
   "dose": "500 mg PO PRN pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Total thyroidectomy with central neck dissection (age 35) for papillary thyroid carcinoma"
 ],
 "chronic_conditions": [
  "Acquired hypothyroidism (postsurgical)",
  "History of malignant neoplasm of thyroid (papillary carcinoma)"
 ],
 "age_at_first_encounter": 37
}
```

Primary diagnoses: ["Postprocedural hypoparathyroidism"]
Comorbidities: ["Malignant neoplasm of thyroid gland", "Acquired absence of thyroid gland"]

## Encounter 0: source 2020-01-15, practice 2025-07-23, follow_up, Endocrinology

Chief complaint: Tingling around her lips and in her fingertips for the past two days following recent neck surgery

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: follow_up

CHIEF COMPLAINT:
Tingling around her lips and in her fingertips for the past two days following recent neck surgery

HISTORY OF PRESENT ILLNESS:
She reports that one week ago, she underwent a total thyroidectomy for papillary thyroid carcinoma. She denies any other symptoms. Her postoperative course was uneventful, and she was discharged on levothyroxine replacement therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Acquired hypothyroidism (postsurgical)
- History of malignant neoplasm of thyroid (papillary carcinoma)

PAST SURGICAL HISTORY:
- Total thyroidectomy with central neck dissection (age 35) for papillary thyroid carcinoma

MEDICATIONS:
Her current medications include levothyroxine and acetaminophen.

Home Medications:
- Calcium carbonate 600 mg PO TID with meals
- Calcitriol 0.25 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism
- Father: Hypertension, type 2 diabetes mellitus
- Paternal aunt: Breast cancer (age 62)
- No family history of thyroid malignancy or parathyroid disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, you note a positive Chvostek sign.

LABORATORY DATA:
Laboratory values are as follows:

- Serum calcium: 7.2 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum phosphorus: 5.5 mg/dL (Normal: 2.5-4.5 mg/dL)
- Serum magnesium: 1.9 mg/dL (Normal: 1.7-2.2 mg/dL)
- Parathyroid hormone (PTH): 5 pg/mL (Normal: 15-65 pg/mL)
- Creatinine: 0.9 mg/dL (Normal: 0.6-1.1 mg/dL)

## Encounter 1: source 2020-02-15, practice 2025-08-22, outpatient, Endocrinology

Chief complaint: Tingling around her mouth and in her fingertips; occasional muscle cramps and spasms

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Tingling around her mouth and in her fingertips; occasional muscle cramps and spasms

HISTORY OF PRESENT ILLNESS:
The patient with postsurgical hypothyroidism and papillary thyroid carcinoma returns for follow-up one month after her initial presentation. She presents with tingling around her mouth and in her fingertips for the past two weeks. She also reports occasional muscle cramps and spasms.

PAST MEDICAL HISTORY:
Active Problem List:
- Postprocedural hypoparathyroidism (diagnosed 2020-01-15)
- Acquired hypothyroidism (postsurgical)
- History of malignant neoplasm of thyroid (papillary carcinoma)

She has a past medical history of thyroidectomy due to papillary thyroid carcinoma two years ago.

PAST SURGICAL HISTORY:
- Total thyroidectomy with central neck dissection (age 35) for papillary thyroid carcinoma

MEDICATIONS:
Home Medications:
- Levothyroxine 125 mcg PO daily
- Calcium carbonate 600 mg PO TID with meals
- Calcitriol 0.25 mcg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism
- Father: Hypertension, type 2 diabetes mellitus
- Paternal aunt: Breast cancer (age 62)
- No family history of thyroid malignancy or parathyroid disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, Chvostek's sign is positive.

LABORATORY DATA:
Her recent blood work shows:

- Serum calcium: 7.2 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum phosphorus: 5.5 mg/dL (Normal: 2.5-4.5 mg/dL)
- Serum magnesium: 1.9 mg/dL (Normal: 1.7-2.2 mg/dL)
- Serum creatinine: 0.9 mg/dL (Normal: 0.6-1.1 mg/dL)

## Encounter 2: source 2020-07-15, practice 2026-01-24, ed, Emergency Medicine

Chief complaint: Worsening muscle cramps and tingling around her lips and fingertips

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening muscle cramps and tingling around her lips and fingertips

HISTORY OF PRESENT ILLNESS:
The patient returns following January and February encounters and comes to the emergency department with a 3-day history of worsening muscle cramps and tingling around her lips and fingertips.

PAST MEDICAL HISTORY:
Active Problem List:
- Postprocedural hypoparathyroidism (diagnosed 2020-01-15)
- Acquired hypothyroidism (postsurgical)
- History of malignant neoplasm of thyroid (papillary carcinoma)

PAST SURGICAL HISTORY:
She underwent a total thyroidectomy 2 weeks ago for papillary thyroid cancer.

MEDICATIONS:
Home Medications:
- Levothyroxine 125 mcg PO daily
- Calcium carbonate 600 mg PO TID with meals
- Calcitriol 0.25 mcg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism
- Father: Hypertension, type 2 diabetes mellitus
- Paternal aunt: Breast cancer (age 62)
- No family history of thyroid malignancy or parathyroid disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On exam, she has a positive Chvostek sign.

LABORATORY DATA:
Labs show a serum calcium of 6.8 mg/dL and phosphate of 5.3 mg/dL. Magnesium and renal function are normal.

ASSESSMENT AND PLAN:
The electrolyte disturbance in this patient is caused by deficiency of a hormone that carries out

The patient is admitted and started on intravenous calcium gluconate.
