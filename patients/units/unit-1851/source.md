# synthetic_hospital patient 1851

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex F, Asian, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-27 (Sun) | inpatient | Cardiology |
| 1 | 2020-03-15 | 2024-12-24 (Tue) | ed | Emergency Medicine |
| 2 | 2021-03-15 | 2025-12-29 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Propylthiouracil (agranulocytosis)",
  "Methimazole (agranulocytosis)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 glasses of wine per week)",
 "family_history": [
  "Mother: Graves' disease",
  "Sister: Type 1 diabetes mellitus",
  "Maternal grandmother: Hashimoto thyroiditis"
 ],
 "race_ethnicity": "Asian",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "125 mcg PO daily",
   "name": "Levothyroxine"
  }
 ],
 "surgical_history": [
  "Radioactive iodine ablation (age 29)"
 ],
 "chronic_conditions": [
  "Graves' disease",
  "Hypothyroidism (post-radioactive iodine ablation)",
  "Recurrent thyrotoxicosis"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Agranulocytosis", "Acute decompensated heart failure"]
Comorbidities: ["Graves disease"]

## Encounter 0: source 2020-01-15, practice 2024-10-27, inpatient, Cardiology

Chief complaint: Palpitations, heat intolerance, and unintentional weight loss

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Palpitations, heat intolerance, and unintentional weight loss

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 2-month history of palpitations, heat intolerance, and unintentional weight loss despite an increased appetite. She also reports experiencing increased anxiety and difficulty sleeping.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves' disease
- Hypothyroidism (post-radioactive iodine ablation)
- Recurrent thyrotoxicosis

Her past medical history is significant for Graves' disease, for which she was treated with radioactive iodine ablation 5 years ago.

PAST SURGICAL HISTORY:
- Radioactive iodine ablation (age 29)

MEDICATIONS:
Home Medications:
- Levothyroxine 125 mcg PO daily

ALLERGIES:
Propylthiouracil (agranulocytosis), Methimazole (agranulocytosis)

FAMILY HISTORY:
- Mother: Graves' disease
- Sister: Type 1 diabetes mellitus
- Maternal grandmother: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Elementary school teacher.

VITAL SIGNS:
On physical examination, her blood pressure is 145/55 mmHg, heart rate is 112 bpm, and temperature is 37.8°C (100°F).

PHYSICAL EXAMINATION:
She has a prominent point of maximal impulse on cardiac examination and a palpable thyroid gland with no nodules. Her skin is warm and moist, and she has a fine tremor on outstretched hands.

LABORATORY DATA:
Laboratory tests reveal the following:

- Thyroid-stimulating hormone (TSH): <0.01 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free thyroxine (T4): 3.2 ng/dL (normal range: 0.9-1.7 ng/dL)
- Total triiodothyronine (T3): 250 ng/dL (normal range: 80-180 ng/dL)

## Encounter 1: source 2020-03-15, practice 2024-12-24, ed, Emergency Medicine

Chief complaint: 2-day history of fever, sore throat, and fatigue

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of fever, sore throat, and fatigue

PAST MEDICAL HISTORY:
Active Problem List:
- Acute decompensated heart failure (diagnosed 2020-01-15)
- Graves' disease
- Hypothyroidism (post-radioactive iodine ablation)
- Recurrent thyrotoxicosis

She was recently diagnosed with hyperthyroidism and began medication for it two weeks ago. Her past medical history is otherwise unremarkable.

PAST SURGICAL HISTORY:
- Radioactive iodine ablation (age 29)

MEDICATIONS:
She takes she takes no other medications.

Home Medications:
- Levothyroxine 125 mcg PO daily

ALLERGIES:
Propylthiouracil (agranulocytosis), Methimazole (agranulocytosis)

FAMILY HISTORY:
- Mother: Graves' disease
- Sister: Type 1 diabetes mellitus
- Maternal grandmother: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Elementary school teacher.

VITAL SIGNS:
Temperature is 102 F

PHYSICAL EXAMINATION:
Scattered posterior pharyngeal ulcerations and with mild surrounding erythema

## Encounter 2: source 2021-03-15, practice 2025-12-29, ed, Emergency Medicine

Chief complaint: High fever, severe sore throat, and difficulty swallowing

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
High fever, severe sore throat, and difficulty swallowing

HISTORY OF PRESENT ILLNESS:
This 35-year-old with Graves' disease and prior agranulocytosis presents to the clinic with a 2-day history of high fever, severe sore throat, and difficulty swallowing.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute decompensated heart failure (diagnosed 2020-01-15)
- Agranulocytosis (diagnosed 2020-03-15)
- Graves' disease
- Hypothyroidism (post-radioactive iodine ablation)
- Recurrent thyrotoxicosis

She has a past medical history of Graves' disease.

PAST SURGICAL HISTORY:
- Radioactive iodine ablation (age 29)

MEDICATIONS:
She has been taking propylthiouracil for the past 6 months.

Home Medications:
- Levothyroxine 125 mcg PO daily

ALLERGIES:
Propylthiouracil (agranulocytosis), Methimazole (agranulocytosis)

FAMILY HISTORY:
- Mother: Graves' disease
- Sister: Type 1 diabetes mellitus
- Maternal grandmother: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Elementary school teacher.

REVIEW OF SYSTEMS:
She denies cough, rhinorrhea, or any other symptoms.

PHYSICAL EXAMINATION:
On examination, she appears ill and has a temperature of 39.2°C (102.5°F). Her throat is erythematous without exudates, and there are no palpable cervical lymph nodes.

LABORATORY DATA:
Her last complete blood count (CBC) was normal 1 month ago.

ASSESSMENT AND PLAN:
Given her symptoms and medication history,
