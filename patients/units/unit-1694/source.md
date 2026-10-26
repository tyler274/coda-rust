# synthetic_hospital patient 1694

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Employer-based private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-11-25 (Tue) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-06-15 | 2026-04-30 (Thu) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2026-06-04 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Employer-based private PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None during pregnancy; social use prior to conception",
 "family_history": [
  "Mother with preeclampsia during pregnancy",
  "Maternal cousin with systemic lupus erythematosus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "60 mg PO TID",
   "name": "Pyridostigmine"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Myasthenia gravis",
  "History of gestational hypertension"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Severe preeclampsia without HELLP features", "Tetanus prophylaxis"]
Comorbidities: ["Pregnancy (34 weeks gestation)", "Multiparity"]

## Encounter 0: source 2020-01-15, practice 2025-11-25, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit and vaccination counseling

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit and vaccination counseling

HISTORY OF PRESENT ILLNESS:
Her pregnancy has been uncomplicated thus far. She reports adherence to her prenatal vitamins and has no complaints. She is concerned about the upcoming flu season and inquires about ways to protect herself and her unborn child.

PAST MEDICAL HISTORY:
Active Problem List:
- Myasthenia gravis
- History of gestational hypertension

Her obstetric history is significant for one previous full-term delivery with no complications.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Pyridostigmine 60 mg PO TID

ALLERGIES:
She has no known drug allergies

FAMILY HISTORY:
- Mother with preeclampsia during pregnancy
- Maternal cousin with systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None during pregnancy; social use prior to conception. Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
Her physical examination is unremarkable

## Encounter 1: source 2020-06-15, practice 2026-04-30, ed, Emergency Medicine

Chief complaint: Severe headache and visual disturbances described as flashing lights

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe headache and visual disturbances described as flashing lights

HISTORY OF PRESENT ILLNESS:
This patient with myasthenia gravis and gestational hypertension seen January 15 presents to ED with a 1-day history of severe headache and visual disturbances described as "flashing lights."

PAST MEDICAL HISTORY:
Active Problem List:
- Tetanus prophylaxis (diagnosed 2020-01-15)
- Myasthenia gravis
- History of gestational hypertension

Her past medical history is significant for gestational hypertension during her first pregnancy.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Pyridostigmine 60 mg PO TID
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with preeclampsia during pregnancy
- Maternal cousin with systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None during pregnancy; social use prior to conception. Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure is 158/102 mm Hg.

PHYSICAL EXAMINATION:
Physical examination reveals 2+ lower extremity edema. Deep tendon reflexes are brisk, and there is a notable clonus.

LABORATORY DATA:
Laboratory values are as follows:

- Urinalysis: Proteinuria 2+
- Serum creatinine: 1.2 mg/dL (elevated)
- Liver function tests:
- AST: 70 U/L (elevated)
- ALT: 80 U/L (elevated)
- Platelet count: 180,000/µL
- Hemoglobin: 12 g/dL

## Encounter 2: source 2020-07-15, practice 2026-06-04, ed, Emergency Medicine

Chief complaint: Headache, visual disturbances, and right upper quadrant abdominal pain

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Headache, visual disturbances, and right upper quadrant abdominal pain

PAST MEDICAL HISTORY:
Active Problem List:
- Tetanus prophylaxis (diagnosed 2020-01-15)
- Severe preeclampsia without HELLP features (diagnosed 2020-06-15)
- Myasthenia gravis
- History of gestational hypertension

She has a history of myasthenia gravis, which is well-controlled with pyridostigmine.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Pyridostigmine 60 mg PO TID
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with preeclampsia during pregnancy
- Maternal cousin with systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None during pregnancy; social use prior to conception. Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure is 160/110 mmHg.

PHYSICAL EXAMINATION:
Her physical examination reveals 2+ lower extremity edema and deep tendon reflexes are brisk.

LABORATORY DATA:
Laboratory findings are as follows:

- Urinalysis: Proteinuria 3+
- Serum creatinine: 0.9 mg/dL
- Platelet count: 110,000/µL
- AST: 70 U/L
- ALT: 65 U/L

PLAN:
The obstetrician is considering the administration of magnesium sulfate for seizure prophylaxis.
