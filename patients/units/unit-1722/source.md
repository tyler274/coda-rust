# synthetic_hospital patient 1722

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Black/African American, Employer-sponsored private PPO insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-11-11 (Tue) | ed | Emergency Medicine |
| 1 | 2020-02-15 | 2025-12-08 (Mon) | outpatient | Neurology |
| 2 | 2020-04-15 | 2026-02-13 (Fri) | outpatient | Neurology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Employer-sponsored private PPO insurance",
 "occupation": "Administrative assistant (sedentary desk work)",
 "alcohol_use": "Occasional social use (1-2 drinks per month)",
 "family_history": [
  "Mother: Class III obesity, type 2 diabetes mellitus",
  "Sister (age 26): Chronic migraine headaches",
  "Maternal aunt: Idiopathic intracranial hypertension (diagnosed age 32)",
  "Paternal grandfather: Essential hypertension"
 ],
 "race_ethnicity": "Black/African American",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Class II Obesity (BMI 37 kg/m\u00b2)",
  "Rapid weight gain (10 lbs over preceding 12 months)"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Intracranial hypertension"]
Comorbidities: ["Abducens nerve palsy", "Obesity"]

## Encounter 0: source 2020-01-15, practice 2025-11-11, ed, Emergency Medicine

Chief complaint: Progressively worsening headaches, nausea, and transient blurred vision for 1 week

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Marcus Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressively worsening headaches, nausea, and transient blurred vision for 1 week

HISTORY OF PRESENT ILLNESS:
She reports recent weight gain and has a BMI of 37.

PAST MEDICAL HISTORY:
Active Problem List:
- Class II Obesity (BMI 37 kg/m²)
- Rapid weight gain (10 lbs over preceding 12 months)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She is not currently taking any medications, including oral contraceptives or vitamin supplements.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Class III obesity, type 2 diabetes mellitus
- Sister (age 26): Chronic migraine headaches
- Maternal aunt: Idiopathic intracranial hypertension (diagnosed age 32)
- Paternal grandfather: Essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Administrative assistant (sedentary desk work).

VITAL SIGNS:
Her blood pressure is 146/92 mmHg, and pulse is 57/min.

PHYSICAL EXAMINATION:
On physical examination, she has bilateral sixth nerve palsy and visual obscurations with bending over. Fundoscopic examination reveals papilledema and loss of spontaneous venous pulsations.

IMAGING:
Non-contrast CT scan of the head reveals no masses, hemorrhage, or hydrocephalus.

OTHER STUDIES:
A lumbar puncture is performed, showing an opening pressure of 315 mmH2O, with normal CSF composition.

## Encounter 1: source 2020-02-15, practice 2025-12-08, outpatient, Neurology

Chief complaint: 3-week history of persistent headaches and transient visual obscurations

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Aisha Johnson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
3-week history of persistent headaches and transient visual obscurations

HISTORY OF PRESENT ILLNESS:
The patient with Class II obesity returns for follow-up one month after her ED presentation. She reports the headaches are worse in the morning and occasionally wake her from sleep. She also notes a "whooshing" sound in her ears that is synchronous with her heartbeat.

PAST MEDICAL HISTORY:
Active Problem List:
- Intracranial hypertension (diagnosed 2020-01-15)
- Class II Obesity (BMI 37 kg/m²)
- Rapid weight gain (10 lbs over preceding 12 months)

PAST SURGICAL HISTORY:
- None

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Class III obesity, type 2 diabetes mellitus
- Sister (age 26): Chronic migraine headaches
- Maternal aunt: Idiopathic intracranial hypertension (diagnosed age 32)
- Paternal grandfather: Essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Administrative assistant (sedentary desk work).

PHYSICAL EXAMINATION:
On physical examination, her BMI is 35 kg/m^2, and fundoscopic examination reveals bilateral papilledema. Extraocular movements are intact except for a horizontal diplopia on extreme lateral gaze. Visual acuity is 20/20 in both eyes, and visual fields are full to confrontation.

LABORATORY DATA:
The cerebrospinal fluid analysis shows:
- WBC: 2 cells/µL (normal 0-5 cells/µL)
- RBC: 0 cells/µL
- Protein: 45 mg/dL (normal 15-45 mg/dL)
- Glucose: 60 mg/dL (normal 45-80 mg/dL)

OTHER STUDIES:
A lumbar puncture is performed, and the opening pressure is measured at 320 mm H_2O.

## Encounter 2: source 2020-04-15, practice 2026-02-13, outpatient, Neurology

Chief complaint: 3-month history of headaches

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Kwame Asante, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of headaches

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of headaches previously evaluated on January 15, 2020, and February 15, 2020. She describes a constant, dull ache that worsens with eye movement, transient visual obscurations, and pulsatile tinnitus. Two months after her last visit, she presents with persistent headaches and worsening vision.

PAST MEDICAL HISTORY:
Active Problem List:
- Intracranial hypertension (diagnosed 2020-01-15)
- Class II Obesity (BMI 37 kg/m²)
- Rapid weight gain (10 lbs over preceding 12 months)

PAST SURGICAL HISTORY:
- None

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Class III obesity, type 2 diabetes mellitus
- Sister (age 26): Chronic migraine headaches
- Maternal aunt: Idiopathic intracranial hypertension (diagnosed age 32)
- Paternal grandfather: Essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Administrative assistant (sedentary desk work).

PHYSICAL EXAMINATION:
BMI is 35 kg/m^2, and fundoscopic examination reveals bilateral papilledema. Visual acuity is 20/20 in both eyes, but visual field testing shows an enlarged blind spot.

LABORATORY DATA:
Basic metabolic panel and complete blood count are within normal limits.

IMAGING:
MRI of the brain and MR venography are normal, ruling out space-occupying lesions and venous sinus thrombosis.

OTHER STUDIES:
Opening pressure measured during lumbar puncture is elevated at 280 mm H2O. Repeat visual field testing demonstrates progressive loss of peripheral vision.

PLAN:
The patient is started on acetazolamide with mild improvement in symptoms.
