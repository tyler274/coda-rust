# synthetic_hospital patient 1895

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex F, Non-Hispanic White, University student health plan (primary), parental private PPO (secondary).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-04-15 | 2025-02-25 (Tue) | outpatient | Family Medicine |
| 1 | 2020-12-15 | 2025-10-22 (Wed) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "University student health plan (primary), parental private PPO (secondary)",
 "occupation": "Bakery associate at local artisan bakery",
 "alcohol_use": "Social (1-2 drinks on weekends)",
 "family_history": [
  "Mother with history of recurrent urinary tract infections",
  "Father with hypertension",
  "No family history of kidney disease or stones"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablespoon in water as needed for constipation",
   "name": "Psyllium husk fiber supplement"
  },
  {
   "dose": "500 mg tablet daily (started recently for immune support)",
   "name": "Vitamin C"
  },
  {
   "dose": "1 tablet daily",
   "name": "Multivitamin with zinc"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Functional constipation"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Uncomplicated cystitis"]
Comorbidities: []

## Encounter 0: source 2020-04-15, practice 2025-02-25, outpatient, Family Medicine

Chief complaint: 2-day history of dysuria, increased urinary frequency, and suprapubic pain

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Sarah Chen, MD
Department: Family Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
2-day history of dysuria, increased urinary frequency, and suprapubic pain

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 2-day history of dysuria, increased urinary frequency, and suprapubic pain. She denies vaginal discharge or bleeding, back pain, fever, or flank pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Functional constipation

MEDICATIONS:
She is.

Home Medications:
- Psyllium husk fiber supplement 1 tablespoon in water as needed for constipation
- Vitamin C 500 mg tablet daily (started recently for immune support)
- Multivitamin with zinc 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with history of recurrent urinary tract infections
- Father with hypertension
- No family history of kidney disease or stones

SOCIAL HISTORY:
She is sexually active with one partner and uses condoms inconsistently.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
On physical examination, her abdominal exam reveals mild suprapubic tenderness without rebound or guarding. A pelvic exam is deferred as the patient is not comfortable with the procedure.

LABORATORY DATA:
Urinalysis via a clean-catch midstream urine sample is performed, showing:

- Leukocyte esterase: Positive
- Nitrite: Positive
- WBC: >10 per high power field
- RBC: 0-2 per high power field
- Bacteria: Many Gram-negative rods

## Encounter 1: source 2020-12-15, practice 2025-10-22, outpatient, Internal Medicine

Chief complaint: 3 days of painful urination, urinary urgency, and suprapubic discomfort

ENCOUNTER NOTE
Date: 2020-12-15
Provider: Dr. Raj Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
3 days of painful urination, urinary urgency, and suprapubic discomfort

HISTORY OF PRESENT ILLNESS:
The patient with functional constipation returns with urinary symptoms similar to her April 2020 encounter. She denies fever, chills, flank pain, or vaginal discharge.

PAST MEDICAL HISTORY:
Active Problem List:
- Uncomplicated cystitis (diagnosed 2020-04-15)
- Functional constipation

She completed a flu vaccination campaign through her university health program last month and has not had any recent illnesses.

MEDICATIONS:
She takes she does not take any prescription medications. She recently started taking over-the-counter vitamin C and multivitamins with zinc after reading about general immune support online.

Home Medications:
- Psyllium husk fiber supplement 1 tablespoon in water as needed for constipation
- Multivitamin with zinc 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with history of recurrent urinary tract infections
- Father with hypertension
- No family history of kidney disease or stones

SOCIAL HISTORY:
She works full-time at a local bakery and says she occasionally struggles with mild constipation due to irregular work breaks but has not had any bowel changes recently. She is otherwise healthy, does not smoke, and drinks alcohol socially. She has had one sexual partner for the past four months.

REVIEW OF SYSTEMS:
On review of systems, she denies nausea, vomiting, back pain, vaginal irritation, or abnormal bleeding.

VITAL SIGNS:
Her temperature is 37.1°C (98.8°F), blood pressure is 112/72 mmHg, and pulse is 82/min.

PHYSICAL EXAMINATION:
Physical exam is unremarkable except for mild suprapubic tenderness.

LABORATORY DATA:
Urinalysis shows positive leukocyte esterase and nitrites.
