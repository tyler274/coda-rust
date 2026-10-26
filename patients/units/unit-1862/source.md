# synthetic_hospital patient 1862

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex F, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-26 (Wed) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2025-10-17 (Fri) | inpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Customer service representative",
 "alcohol_use": "None",
 "family_history": [
  "Type 1 diabetes mellitus in younger brother",
  "Hashimoto thyroiditis in mother",
  "Preterm delivery in maternal grandmother"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "24 units subcutaneously at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale subcutaneously before meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "88 mcg orally daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "1 tablet orally daily",
   "name": "Prenatal vitamin with iron"
  }
 ],
 "surgical_history": [
  "Tonsillectomy and adenoidectomy"
 ],
 "chronic_conditions": [
  "Type 1 diabetes mellitus",
  "Autoimmune hypothyroidism"
 ],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Hyponatremia", "Respiratory distress syndrome"]
Comorbidities: ["Type 1 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2025-02-26, ed, Emergency Medicine

Chief complaint: Acute confusion and witnessed generalized tonic-clonic seizure at home

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute confusion and witnessed generalized tonic-clonic seizure at home

HISTORY OF PRESENT ILLNESS:
A 32-year-old woman is brought to the emergency department by her husband due to confusion and a witnessed generalized tonic-clonic seizure at home. Her husband reports that she has been drinking large amounts of water over the past few days to "stay hydrated" due to her illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus
- Autoimmune hypothyroidism

Her past medical history is significant for poorly controlled type 1 diabetes mellitus and a recent episode of gastroenteritis with significant vomiting and diarrhea.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin lispro sliding scale subcutaneously before meals
- Levothyroxine 88 mcg orally daily
- Prenatal vitamin with iron 1 tablet orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 1 diabetes mellitus in younger brother
- Hashimoto thyroiditis in mother
- Preterm delivery in maternal grandmother

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Customer service representative.

VITAL SIGNS:
Her temperature is 37.1°C (98.8°F), blood pressure is 130/85 mmHg, pulse is 88/min, and respirations are 16/min.

PHYSICAL EXAMINATION:
On examination, she is lethargic but arousable, oriented to person only, and has diffuse muscle weakness. Neurological examination is nonfocal.

LABORATORY DATA:
Laboratory studies are as follows:

- Serum sodium: 118 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum chloride: 88 mEq/L
- Serum bicarbonate: 24 mEq/L
- Serum glucose: 180 mg/dL
- Serum osmolality: 240 mOsm/kg
- Urine osmolality: 500 mOsm/kg
- Urine sodium: 40 mEq/L

## Encounter 1: source 2020-09-15, practice 2025-10-17, inpatient, Obstetrics & Gynecology

Chief complaint: Active labor at 35 weeks gestation with limited prenatal care

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Robert Kim, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Active labor at 35 weeks gestation with limited prenatal care

HISTORY OF PRESENT ILLNESS:
This 33-year-old female with Type 1 diabetes, autoimmune hypothyroidism, and January seizure history has limited prenatal care. Her pregnancy is complicated by poor glycemic control. She delivers a male infant weighing 2500 grams.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyponatremia (diagnosed 2020-01-15)
- Type 1 diabetes mellitus
- Autoimmune hypothyroidism

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin lispro sliding scale subcutaneously before meals
- Levothyroxine 88 mcg orally daily
- Prenatal vitamin with iron 1 tablet orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 1 diabetes mellitus in younger brother
- Hashimoto thyroiditis in mother
- Preterm delivery in maternal grandmother

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Customer service representative.

VITAL SIGNS:
Her vital signs are stable. His initial oxygen saturation is 85% on room air. His oxygen saturation improves to 92%.

PHYSICAL EXAMINATION:
On examination, she is anxious but in no acute distress. Fetal heart tones are normal. The neonate has immediate respiratory distress with grunting, nasal flaring, and intercostal retractions. Apgar scores are 5 at 1 minute and 7 at 5 minutes.

LABORATORY DATA:
Recent hemoglobin A1c of 9.2%

IMAGING:
A chest radiograph is obtained and shows diffuse bilateral atelectasis with a ground-glass appearance.

PLAN:
The neonate is placed on supplemental oxygen.
