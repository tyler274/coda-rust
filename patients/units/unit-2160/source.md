# synthetic_hospital patient 2160

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Commercial PPO (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 1 | 2020-10-15 | 2025-01-16 (Thu) | ed | Emergency Medicine |
| 2 | 2021-10-15 | 2026-01-23 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO (Blue Cross Blue Shield)",
 "occupation": "Construction laborer",
 "alcohol_use": "Social (2-3 drinks per week)",
 "family_history": [
  "Mother: Degenerative disc disease, osteoporosis",
  "Father: Hypertension, type 2 diabetes mellitus",
  "One sibling: Healthy"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (quit 4 years ago, 12 pack-year history)",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "400 mg PO PRN pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "500 mg PO daily",
   "name": "Metformin"
  }
 ],
 "surgical_history": [
  "Appendectomy (age 14)"
 ],
 "chronic_conditions": [
  "Essential hypertension",
  "Obesity (BMI 33)",
  "Prediabetes"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Cauda equina syndrome"]
Comorbidities: []

## Encounter 1: source 2020-10-15, practice 2025-01-16, ed, Emergency Medicine

Chief complaint: Severe lower back pain with difficulty urinating and perineal numbness

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe lower back pain with difficulty urinating and perineal numbness

HISTORY OF PRESENT ILLNESS:
This 45-year-old male with a history of essential hypertension, obesity (BMI 33), and prediabetes on lisinopril, metformin, and ibuprofen as needed presents to the emergency department with severe lower back pain that began two days ago and has worsened overnight. He now reports difficulty urinating, numbness in his inner thighs and perineal region, unsteadiness when walking, and weakness in his right leg.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension
- Obesity (BMI 33)
- Prediabetes

PAST SURGICAL HISTORY:
- Appendectomy (age 14)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Ibuprofen 400 mg PO PRN pain
- Metformin 500 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Degenerative disc disease, osteoporosis
- Father: Hypertension, type 2 diabetes mellitus
- One sibling: Healthy

SOCIAL HISTORY:
Smoking: Former smoker (quit 4 years ago, 12 pack-year history). Alcohol: Social (2-3 drinks per week). Occupation: Construction laborer.

VITAL SIGNS:
Temperature is 36.8°C (98.2°F), blood pressure is 124/78 mm Hg, and pulse is 88/min.

PHYSICAL EXAMINATION:
Neurologic examination shows 3/5 strength in right ankle dorsiflexion and absent right ankle reflex. Sensation is decreased over both posterior thighs and the perianal region. Anal wink is absent.

## Encounter 2: source 2021-10-15, practice 2026-01-23, ed, Emergency Medicine

Chief complaint: Worsening lower back pain with bilateral leg weakness and urinary retention

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening lower back pain with bilateral leg weakness and urinary retention

HISTORY OF PRESENT ILLNESS:
This 46-year-old male with essential hypertension, obesity, and prediabetes returns to the emergency department one year after a previous visit with similar complaints, presenting with worsening lower back pain for the past week, which acutely progressed to bilateral leg weakness and numbness over the past 12 hours. He also reports new-onset urinary retention and difficulty starting his stream.

PAST MEDICAL HISTORY:
Active Problem List:
- Cauda equina syndrome (diagnosed 2020-10-15)
- Essential hypertension
- Obesity (BMI 33)
- Prediabetes

PAST SURGICAL HISTORY:
- Appendectomy (age 14)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Ibuprofen 400 mg PO PRN pain
- Metformin 500 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Degenerative disc disease, osteoporosis
- Father: Hypertension, type 2 diabetes mellitus
- One sibling: Healthy

SOCIAL HISTORY:
Smoking: Former smoker (quit 4 years ago, 12 pack-year history). Alcohol: Social (2-3 drinks per week). Occupation: Construction laborer.

PHYSICAL EXAMINATION:
On examination, there is decreased anal sphincter tone and loss of perianal sensation. Sensation is diminished over the posterior thighs and medial buttocks bilaterally. Lower limb strength is reduced bilaterally, and knee reflexes are diminished. Upper extremity exam is normal.
