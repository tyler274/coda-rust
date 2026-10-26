# synthetic_hospital patient 2664

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 26 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-12-26 (Fri) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2026-06-26 (Fri) | outpatient | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Warehouse associate",
 "alcohol_use": "Occasional social use",
 "family_history": [
  "No family history of Parkinson disease or movement disorders",
  "No family history of early-onset neurological disease",
  "No known family history of substance use disorders"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current tobacco user, 1/2 pack per day",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 26
}
```

Primary diagnoses: ["MPTP-induced parkinsonism", "Methadone overdose"]
Comorbidities: ["Mechanical repetitive strain injury"]

## Encounter 0: source 2020-01-15, practice 2025-12-26, ed, Emergency Medicine

Chief complaint: Brought to the emergency department by police after being found agitated and shouting incoherently at a local park

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department by police after being found agitated and shouting incoherently at a local park

HISTORY OF PRESENT ILLNESS:
His friends report that he has been experimenting with recreational drugs but are unsure of what substances he may have taken.

PAST MEDICAL HISTORY:
His medical history is unremarkable,

ALLERGIES:
He has no known drug allergies.

FAMILY HISTORY:
- No family history of Parkinson disease or movement disorders
- No family history of early-onset neurological disease
- No known family history of substance use disorders

SOCIAL HISTORY:
Smoking: Current tobacco user, 1/2 pack per day. Alcohol: Occasional social use. Occupation: Warehouse associate.

VITAL SIGNS:
He has a temperature of 39.2°C (102.5°F), blood pressure of 180/110 mmHg, and a heart rate of 120 beats per minute

PHYSICAL EXAMINATION:
On examination, the patient is diaphoretic. He is physically aggressive towards staff and exhibits horizontal nystagmus. He appears to be responding to internal stimuli and is disoriented to time, place, and person.

LABORATORY DATA:
Urine toxicology screen is pending.

Laboratory values:
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 98 mg/dL
- Complete blood count: within normal limits

## Encounter 1: source 2020-07-15, practice 2026-06-26, outpatient, Neurology

Chief complaint: Progressively slower movements and resting tremor in right hand noticed by roommate over past several weeks

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressively slower movements and resting tremor in right hand noticed by roommate over past several weeks

HISTORY OF PRESENT ILLNESS:
The patient presents six months after an emergency department visit for agitation and incoherent shouting. He is brought to the clinic by his roommate, who reports the patient has become progressively slower in his movements and developed a resting tremor in his right hand over the past several weeks. He also noticed the patient walks with a stiff gait and has difficulty initiating movement.

PAST MEDICAL HISTORY:
Active Problem List:
- Methadone overdose (diagnosed 2020-01-15)

MEDICATIONS:
He is not taking any prescribed medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history of neurologic conditions.

SOCIAL HISTORY:
He admits to using "various street drugs" over the last few months at local parties but cannot recall the exact names.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, there is cogwheel rigidity of the right upper limb, bradykinesia, and a resting tremor.

LABORATORY DATA:
Serum and urine studies are normal.

IMAGING:
MRI of the brain shows decreased signal intensity in the substantia nigra.

OTHER STUDIES:
Slit-lamp exam shows no abnormalities.

PLAN:
The patient is started on levodopa-carbidopa therapy with mild improvement.
