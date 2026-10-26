# synthetic_hospital patient 1902

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex F, White, University student health plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-06 (Tue) | outpatient | Neurology |
| 1 | 2020-07-15 | 2025-10-24 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Lamotrigine (severe cutaneous adverse reaction/DRESS)"
 ],
 "insurance": "University student health plan",
 "occupation": "Graduate student",
 "alcohol_use": "Occasional social use, less than 1 drink per month",
 "family_history": [
  "Maternal cousin with epilepsy",
  "Father with hypertension",
  "Mother with hypothyroidism"
 ],
 "race_ethnicity": "White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg by mouth daily",
   "name": "Lamotrigine"
  }
 ],
 "surgical_history": [
  "Tonsillectomy, age 8"
 ],
 "chronic_conditions": [
  "Epilepsy"
 ],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Breakthrough generalized tonic-clonic seizure", "Allopurinol hypersensitivity syndrome (DRESS)"]
Comorbidities: ["Epilepsy"]

## Encounter 0: source 2020-01-15, practice 2025-05-06, outpatient, Neurology

Chief complaint: presents to the clinic with a two-day history of a pruritic rash

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to the clinic with a two-day history of a pruritic rash

HISTORY OF PRESENT ILLNESS:
She has a history of epilepsy and started taking lamotrigine 3 weeks ago. The rash began on her trunk and has spread to her arms and legs. She denies fever, mucosal involvement, or systemic symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Epilepsy

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
Home Medications:
- Lamotrigine 50 mg by mouth daily

ALLERGIES:
Lamotrigine (severe cutaneous adverse reaction/DRESS)

FAMILY HISTORY:
- Maternal cousin with epilepsy
- Father with hypertension
- Mother with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use, less than 1 drink per month. Occupation: Graduate student.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, she has multiple erythematous macules and papules on her trunk and extremities. There are no vesicles, bullae, or target lesions. No lymphadenopathy is noted.

LABORATORY DATA:
Laboratory tests reveal:
- Complete blood count: within normal limits
- Liver function tests: within normal limits
- Serum creatinine: within normal limits

## Encounter 1: source 2020-07-15, practice 2025-10-24, ed, Emergency Medicine

Chief complaint: is brought to the emergency department by her roommate after experiencing a convulsive episode

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Okonkwo, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
She is brought to the emergency department by her roommate after experiencing a convulsive episode

HISTORY OF PRESENT ILLNESS:
The patient with epilepsy, previously evaluated in January 2020 for a pruritic rash, presents after a convulsive episode. The roommate reports that the patient suddenly fell to the ground with a loss of consciousness, exhibited jerking movements of her limbs for approximately 2 minutes, and then became unresponsive. The patient regained consciousness after 5 minutes but appeared confused and drowsy.

PAST MEDICAL HISTORY:
Active Problem List:
- Allopurinol hypersensitivity syndrome (DRESS) (diagnosed 2020-01-15)
- Epilepsy

The patient has a history of epilepsy.

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
She is currently taking medication for seizure control.

Home Medications:
- Lamotrigine 50 mg by mouth daily

ALLERGIES:
Lamotrigine (severe cutaneous adverse reaction/DRESS)

FAMILY HISTORY:
- Maternal cousin with epilepsy
- Father with hypertension
- Mother with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use, less than 1 drink per month. Occupation: Graduate student.

VITAL SIGNS:
Her temperature is 37.2°C (99.0°F), blood pressure is 125/78 mmHg, pulse is 88/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
On examination, the patient is drowsy but oriented to time, place, and person. She has a bitten tongue and a bruise on her left cheek. Neurological examination is nonfocal.

LABORATORY DATA:
Laboratory studies are as follows:

- Sodium: 138 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 14 mg/dL
- Creatinine: 0.9 mg/dL
- Glucose: 98 mg/dL
- Serum calcium: 9.2 mg/dL
- Complete blood count: within normal limits
- Serum antiepileptic drug levels: pending
