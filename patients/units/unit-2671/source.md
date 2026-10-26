# synthetic_hospital patient 2671

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-18 (Fri) | outpatient | Cardiology |
| 1 | 2021-01-15 | 2026-07-22 (Wed) | outpatient | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Electrical contractor",
 "alcohol_use": "Social (1-2 beers on weekends)",
 "family_history": [
  "Father: Hypertension, age 62",
  "Mother: Hypothyroidism, age 60",
  "No known family history of sudden cardiac death, cardiomyopathy, or premature coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Tonsillectomy age 8",
  "Knee arthroscopy age 28"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Hypertrophic obstructive cardiomyopathy"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-07-18, outpatient, Cardiology

Chief complaint: Exertional dyspnea and chest pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Exertional dyspnea and chest pain

HISTORY OF PRESENT ILLNESS:
He reports that his symptoms have progressively worsened over the past six months. He denies any syncope, palpitations, or family history of sudden cardiac death.

PAST SURGICAL HISTORY:
- Tonsillectomy age 8
- Knee arthroscopy age 28

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, age 62
- Mother: Hypothyroidism, age 60
- No known family history of sudden cardiac death, cardiomyopathy, or premature coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 beers on weekends). Occupation: Electrical contractor.

VITAL SIGNS:
On physical examination, his blood pressure is 130/85 mmHg, and his heart rate is 78 bpm.

PHYSICAL EXAMINATION:
A harsh crescendo-decrescendo systolic murmur is heard at the left sternal border, which increases in intensity when the patient performs the Valsalva maneuver and decreases when he squats.

IMAGING:
An echocardiogram reveals asymmetric septal hypertrophy and systolic anterior motion of the mitral valve leaflet with a left ventricular outflow tract gradient of 55 mmHg at rest.

## Encounter 1: source 2021-01-15, practice 2026-07-22, outpatient, Cardiology

Chief complaint: Progressive exertional dyspnea and chest pain

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive exertional dyspnea and chest pain

HISTORY OF PRESENT ILLNESS:
The patient returns to the cardiology clinic one year following his initial diagnosis of hypertrophic obstructive cardiomyopathy. He presents with a 6-month history of progressive exertional dyspnea and chest pain. He reports that the symptoms are worse when he is lying flat.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertrophic obstructive cardiomyopathy (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Tonsillectomy age 8
- Knee arthroscopy age 28

ALLERGIES:
NKDA

FAMILY HISTORY:
His medical history is significant for a family member with sudden cardiac death at a young age.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 beers on weekends). Occupation: Electrical contractor.

PHYSICAL EXAMINATION:
On physical examination, you note a harsh systolic murmur that increases in intensity when the patient performs the Valsalva maneuver.

IMAGING:
An echocardiogram is performed, revealing asymmetric septal hypertrophy and systolic anterior motion of the mitral valve leaflet.
