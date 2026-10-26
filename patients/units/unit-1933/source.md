# synthetic_hospital patient 1933

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-11-01 (Wed) | outpatient | Obstetrics & Gynecology |
| 1 | 2022-01-15 | 2025-11-05 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Environmental consultant",
 "alcohol_use": "None during pregnancy; social use prior to conception",
 "family_history": [
  "Type 2 diabetes mellitus (mother)",
  "Essential hypertension (father)",
  "Hypothyroidism (maternal grandmother)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "200 mg orally twice daily",
   "name": "Labetalol"
  },
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale with meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "1 tablet orally daily",
   "name": "Prenatal vitamin with folic acid"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Essential hypertension",
  "Pregnancy (second trimester)"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Lyme disease", "Pre-existing essential hypertension complicating pregnancy, second trimester"]
Comorbidities: ["Routine prenatal care"]

## Encounter 0: source 2020-01-15, practice 2023-11-01, outpatient, Obstetrics & Gynecology

Chief complaint: presents to her primary care physician for a routine prenatal visit.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to her primary care physician for a routine prenatal visit.

HISTORY OF PRESENT ILLNESS:
She reports compliance with her antihypertensive medication regimen. Her blood pressure readings have been consistently around 140/90 mmHg at home.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Essential hypertension
- Pregnancy (second trimester)

She has a past medical history of type 2 diabetes mellitus and was diagnosed with essential hypertension two years ago.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Her current medications include insulin and prenatal vitamins.

Home Medications:
- Labetalol 200 mg orally twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Prenatal vitamin with folic acid 1 tablet orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother)
- Essential hypertension (father)
- Hypothyroidism (maternal grandmother)

SOCIAL HISTORY:
Smoking: Never. Alcohol: None during pregnancy; social use prior to conception. Occupation: Environmental consultant.

REVIEW OF SYSTEMS:
She denies any headaches, visual changes, or epigastric pain.

VITAL SIGNS:
On examination, her blood pressure is 148/92 mmHg, heart rate is 78 beats per minute

PHYSICAL EXAMINATION:
The remainder of the physical exam is unremarkable.

LABORATORY DATA:
Urinalysis is negative for protein, and her blood glucose levels have been well-controlled.

## Encounter 1: source 2022-01-15, practice 2025-11-05, ed, Emergency Medicine

Chief complaint: 5-day history of fatigue, headache, and a spreading rash

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Kwame Asante, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
5-day history of fatigue, headache, and a spreading rash

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old female with type 2 diabetes mellitus and essential hypertension currently in her second trimester of pregnancy, with prior obstetric care for hypertensive disorders, who presents to the emergency department. She reports a recent camping trip in the Northeastern United States.

PAST MEDICAL HISTORY:
Active Problem List:
- Pre-existing essential hypertension complicating pregnancy, second trimester (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Essential hypertension
- Pregnancy (second trimester)

She has had an uncomplicated pregnancy thus far.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Labetalol 200 mg orally twice daily
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Prenatal vitamin with folic acid 1 tablet orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother)
- Essential hypertension (father)
- Hypothyroidism (maternal grandmother)

SOCIAL HISTORY:
Smoking: Never. Alcohol: None during pregnancy; social use prior to conception. Occupation: Environmental consultant.

REVIEW OF SYSTEMS:
She denies fever, arthralgia, or any other systemic symptoms.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, you note a 10 cm erythematous rash with central clearing on her left thigh.

LABORATORY DATA:
Laboratory tests reveal normal white blood cell count and no evidence of anemia or thrombocytopenia. Serologic testing for Borrelia burgdorferi is positive.
