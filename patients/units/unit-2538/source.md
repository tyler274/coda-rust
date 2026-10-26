# synthetic_hospital patient 2538

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-04-24 (Wed) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2024-12-08 (Sun) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2025-10-09 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer",
 "alcohol_use": "Severe alcohol use disorder with binge drinking pattern; consumes 10-15 standard drinks per day during episodic use",
 "family_history": [
  "Father with alcohol use disorder",
  "Maternal uncle with opioid use disorder",
  "Mother with major depressive disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current daily smoker, 1 pack per day",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [
  "Alcohol use disorder",
  "Cocaine use disorder",
  "Opioid use disorder"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Rhabdomyolysis requiring fluid resuscitation"]
Comorbidities: ["Acute kidney injury", "Alcohol use disorder", "Hypocalcemia", "Hyperkalemia", "Hyperphosphatemia"]

## Encounter 0: source 2020-01-15, practice 2024-04-24, ed, Emergency Medicine

Chief complaint: presents to the emergency department with severe muscle pain, weakness, and dark urine

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
He presents to the emergency department with severe muscle pain, weakness, and dark urine

HISTORY OF PRESENT ILLNESS:
He reports that he attended a bachelor party where he drank approximately 15 beers each day and used cocaine intermittently.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol use disorder
- Cocaine use disorder
- Opioid use disorder

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder
- Maternal uncle with opioid use disorder
- Mother with major depressive disorder

SOCIAL HISTORY:
Smoking: Current daily smoker, 1 pack per day. Alcohol: Severe alcohol use disorder with binge drinking pattern; consumes 10-15 standard drinks per day during episodic use. Occupation: Construction laborer.

VITAL SIGNS:
His temperature is 37.2°C (99°F), blood pressure is 150/90 mmHg, pulse is 110/min, and respirations are 20/min

PHYSICAL EXAMINATION:
On physical examination, he is agitated but oriented. He appears dehydrated. His muscle strength is 3/5 in the proximal muscles of his upper and lower extremities.

LABORATORY DATA:
Laboratory studies are as follows:

- Serum creatinine: 2.1 mg/dL (increased from a baseline of 0.9 mg/dL)
- Serum potassium: 5.5 mEq/L
- Serum calcium: 8.0 mg/dL
- Serum phosphorus: 6.0 mg/dL
- Serum CK (creatine kinase): 20,000 U/L
- Urinalysis: Brown-colored urine with a large amount of blood on dipstick, but no red blood cells on microscopy

## Encounter 1: source 2020-09-15, practice 2024-12-08, ed, Emergency Medicine

Chief complaint: is brought to the emergency department after being found collapsed on the floor of his apartment

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
He is brought to the emergency department after being found collapsed on the floor of his apartment

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old male with alcohol, cocaine, and opioid use disorders who returns to the emergency department eight months after a prior presentation with severe muscle pain, weakness, and dark urine. He was last seen 36 hours ago by a neighbor.

PAST MEDICAL HISTORY:
Active Problem List:
- Rhabdomyolysis requiring fluid resuscitation (diagnosed 2020-01-15)
- Alcohol use disorder
- Cocaine use disorder
- Opioid use disorder

He has a history of alcohol use disorder.

MEDICATIONS:
He takes no medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder
- Maternal uncle with opioid use disorder
- Mother with major depressive disorder

SOCIAL HISTORY:
Smoking: Current daily smoker, 1 pack per day. Alcohol: Severe alcohol use disorder with binge drinking pattern; consumes 10-15 standard drinks per day during episodic use. Occupation: Construction laborer.

VITAL SIGNS:
His temperature is 36.5°C (97.7°F), blood pressure is 105/68 mm Hg, pulse is 88/min, and respirations are 16/min.

PHYSICAL EXAMINATION:
On examination, he is obtunded but arousable. His extremities are cool, and there is diffuse muscle tenderness, particularly in the thighs and upper arms.

LABORATORY DATA:
Urinalysis reveals 2+ blood with 0–1 RBCs/hpf. Additional labs are obtained:
Sodium 137 mEq/L
Potassium 6.5 mEq/L
BUN 28 mg/dL
Creatinine 2.9 mg/dL
Creatine kinase (CK) 35,000 U/L

## Encounter 2: source 2021-07-15, practice 2025-10-09, ed, Emergency Medicine

Chief complaint: is brought to the emergency department by paramedics after being found unresponsive in his apartment

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
He is brought to the emergency department by paramedics after being found unresponsive in his apartment

HISTORY OF PRESENT ILLNESS:
The patient returns with presentation similar to prior substance-related visits. His roommate reports that the patient has a history of heroin use and may have taken an unknown quantity last night.

PAST MEDICAL HISTORY:
Active Problem List:
- Rhabdomyolysis requiring fluid resuscitation (diagnosed 2020-01-15)
- Alcohol use disorder
- Cocaine use disorder
- Opioid use disorder

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder
- Maternal uncle with opioid use disorder
- Mother with major depressive disorder

SOCIAL HISTORY:
Smoking: Current daily smoker, 1 pack per day. Alcohol: Severe alcohol use disorder with binge drinking pattern; consumes 10-15 standard drinks per day during episodic use. Occupation: Construction laborer.

VITAL SIGNS:
Respiratory rate of 8 breaths per minute, blood pressure of 90/60 mmHg, and pulse of 55 beats per minute

PHYSICAL EXAMINATION:
On examination, the patient is drowsy but arousable. He has notable swelling and tenderness in his left lower leg, which is pale and cool to the touch. The patient's urine is dark brown.

LABORATORY DATA:
Laboratory studies are as follows:

- Serum creatinine: 3.2 mg/dL (baseline 0.9 mg/dL)
- Serum potassium: 5.8 mEq/L
- Serum calcium: 7.9 mg/dL
- Serum phosphorus: 6.5 mg/dL
- Serum CK (creatine kinase): 20,000 U/L
- Urinalysis: Positive for blood, with no red blood cells on microscopy
