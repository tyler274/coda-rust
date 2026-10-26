# synthetic_hospital patient 2537

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 29 at first encounter, sex M, Non-Hispanic White, Employer-sponsored commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-31 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2025-05-13 (Tue) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2025-11-15 (Sat) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Employer-sponsored commercial insurance",
 "occupation": "Construction worker",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "Father: Gout, major depressive disorder",
  "Mother: Hypertension, type 2 diabetes mellitus",
  "One older brother: Healthy"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 6 months ago (1 pack per day history)",
 "home_medications": [
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "300 mg PO daily",
   "name": "Allopurinol"
  }
 ],
 "surgical_history": [
  "Appendectomy (age 14)"
 ],
 "chronic_conditions": [
  "Major depressive disorder",
  "Hyperuricemia"
 ],
 "age_at_first_encounter": 29
}
```

Primary diagnoses: ["Septic shock", "Compartment syndrome of hand", "Acute pyelonephritis"]
Comorbidities: ["Acute kidney injury"]

## Encounter 0: source 2020-01-15, practice 2024-10-31, outpatient, Internal Medicine

Chief complaint: New-onset ankle swelling and fatigue in a patient with hyperuricemia recently completing allopurinol therapy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
New-onset ankle swelling and fatigue in a patient with hyperuricemia recently completing allopurinol therapy

HISTORY OF PRESENT ILLNESS:
He recently completed a 2-week course of allopurinol for management of hyperuricemia.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder
- Hyperuricemia

PAST SURGICAL HISTORY:
- Appendectomy (age 14)

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Allopurinol 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Gout, major depressive disorder
- Mother: Hypertension, type 2 diabetes mellitus
- One older brother: Healthy

SOCIAL HISTORY:
Smoking: Former smoker, quit 6 months ago (1 pack per day history). Alcohol: Social, 2-3 drinks per week. Occupation: Construction worker.

REVIEW OF SYSTEMS:
He denies rash, fever, dysuria, urgency, or flank pain.

VITAL SIGNS:
Vitals are within normal limits.

PHYSICAL EXAMINATION:
On physical exam, he has bilateral pitting edema of the lower extremities.

LABORATORY DATA:
Labs show a creatinine of 2.3 mg/dL and BUN of 34 mg/dL. Urinalysis shows mild proteinuria and hematuria, but no RBC casts, no bacteria, and no nitrites or leukocyte esterase. Urine culture is negative.

## Encounter 1: source 2020-07-15, practice 2025-05-13, ed, Emergency Medicine

Chief complaint: Severe pain in left arm after high-voltage electrical injury at construction site

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe pain in left arm after high-voltage electrical injury at construction site

HISTORY OF PRESENT ILLNESS:
This 32-year-old construction worker with major depressive disorder and hyperuricemia was seen in January 2020 for ankle swelling. He is brought to the emergency department after sustaining a high-voltage electrical injury from a power line, is alert and oriented but complains of severe pain in his left arm, which appears swollen and erythematous.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute pyelonephritis (diagnosed 2020-01-15)
- Major depressive disorder
- Hyperuricemia

PAST SURGICAL HISTORY:
- Appendectomy (age 14)

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Allopurinol 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Gout, major depressive disorder
- Mother: Hypertension, type 2 diabetes mellitus
- One older brother: Healthy

SOCIAL HISTORY:
Smoking: Former smoker, quit 6 months ago (1 pack per day history). Alcohol: Social, 2-3 drinks per week. Occupation: Construction worker.

VITAL SIGNS:
Blood pressure 135/85 mmHg, heart rate 110 beats per minute, respiratory rate 18 breaths per minute, and temperature 37.2°C (99.0°F)

PHYSICAL EXAMINATION:
On physical examination, the left forearm is tense and tender to palpation. Passive stretching of the fingers elicits severe pain.

LABORATORY DATA:
Serum creatinine: 1.8 mg/dL (baseline unknown)
- Serum potassium: 5.5 mEq/L
- Serum calcium: 7.9 mg/dL
- Serum phosphorus: 6.2 mg/dL
- Serum creatine kinase (CK): 20,000 U/L
- Urine dipstick: 3+ blood with no RBCs on microscopy

## Encounter 2: source 2021-01-15, practice 2025-11-15, icu, Critical Care Medicine

Chief complaint: Fever, tachycardia, and hypotension developing on day 4 of hospitalization for 40% TBSA burns sustained in house fire

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Patel, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Fever, tachycardia, and hypotension developing on day 4 of hospitalization for 40% TBSA burns sustained in house fire

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old male with major depressive disorder and hyperuricemia, previously treated for acute pyelonephritis and compartment syndrome, who develops fever, tachycardia, and hypotension on the fourth day of hospitalization for burns sustained in a house fire 3 days ago. He is complaining of abdominal discomfort. The patient has been receiving enteral feeding since admission, but today he has vomited twice and has a nasogastric tube output of 600 mL over the past 6 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute pyelonephritis (diagnosed 2020-01-15)
- Compartment syndrome of hand (diagnosed 2020-07-15)
- Major depressive disorder
- Hyperuricemia

PAST SURGICAL HISTORY:
His initial treatment included wound care and escharotomy.

MEDICATIONS:
He is currently on intravenous fluids, pain management with morphine, and prophylactic antibiotics.

Home Medications:
- Sertraline 50 mg PO daily
- Allopurinol 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Gout, major depressive disorder
- Mother: Hypertension, type 2 diabetes mellitus
- One older brother: Healthy

SOCIAL HISTORY:
Smoking: Former smoker, quit 6 months ago (1 pack per day history). Alcohol: Social, 2-3 drinks per week. Occupation: Construction worker.

VITAL SIGNS:
Fever of 38.9°C (102°F), tachycardia with a heart rate of 120 bpm, and hypotension with a blood pressure of 90/60 mmHg

PHYSICAL EXAMINATION:
He appears lethargic. His abdomen is distended and tender on palpation. Bowel sounds are hypoactive.

LABORATORY DATA:
White blood cell count: 16,000/mm³
- Hemoglobin: 10.2 g/dL
- Platelets: 250,000/mm³
- Sodium: 135 mEq/L
- Potassium: 4.5 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 22 mEq/L
- Blood urea nitrogen: 28 mg/dL
- Creatinine: 1.4 mg/dL
- Glucose: 110 mg/dL
- Lactic acid: 4.2 mmol/L
