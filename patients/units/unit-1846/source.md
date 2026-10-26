# synthetic_hospital patient 1846

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Commercial PPO (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-29 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2025-05-28 (Wed) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2025-12-08 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO (Blue Cross Blue Shield)",
 "occupation": "Retail store supervisor",
 "alcohol_use": "Former heavy use; in sustained remission for 2 years following inpatient rehabilitation",
 "family_history": [
  "Mother: Generalized anxiety disorder, hypertension",
  "Father: Alcohol use disorder, type 2 diabetes mellitus",
  "Sister: Major depressive disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Wisdom teeth extraction, age 18"
 ],
 "chronic_conditions": [
  "Panic disorder",
  "Alcohol use disorder, in sustained remission"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Hypocalcemia", "Panic disorder"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-11-29, outpatient, Internal Medicine

Chief complaint: Recurrent episodes of intense fear accompanied by palpitations, chest pain, shortness of breath, and fear of losing control over the past 6 months.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Lisa Park, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent episodes of intense fear accompanied by palpitations, chest pain, shortness of breath, and fear of losing control over the past 6 months.

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of recurrent episodes of intense fear accompanied by palpitations, chest pain, shortness of breath, and a fear of losing control. These episodes occur unpredictably and are not associated with any specific triggers. She reports that these episodes last for about 20 minutes and are followed by at least 1 hour of worry about having another attack.

PAST MEDICAL HISTORY:
Active Problem List:
- Panic disorder
- Alcohol use disorder, in sustained remission

Her medical history is significant for alcohol abuse, for which she attended a rehabilitation program 2 years ago and has since been sober.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 18

MEDICATIONS:
She denies using any medications or recreational drugs.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Generalized anxiety disorder, hypertension
- Father: Alcohol use disorder, type 2 diabetes mellitus
- Sister: Major depressive disorder

SOCIAL HISTORY:
She denies using any medications or recreational drugs.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable,

OTHER STUDIES:
An electrocardiogram shows normal sinus rhythm with no abnormalities.

## Encounter 1: source 2020-07-15, practice 2025-05-28, ed, Emergency Medicine

Chief complaint: Sudden onset palpitations, chest tightness, and feeling of impending doom that began 30 minutes ago while shopping.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset palpitations, chest tightness, and feeling of impending doom that began 30 minutes ago while shopping.

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department six months following her initial outpatient evaluation for recurrent fear episodes. She reports that similar episodes have occurred sporadically over the past year, usually lasting for about 10 to 20 minutes. She has visited multiple healthcare providers for various unexplained symptoms including dizziness, tremors, and abdominal discomfort, but extensive workups have not revealed any significant abnormalities.

PAST MEDICAL HISTORY:
Active Problem List:
- Panic disorder (diagnosed 2020-01-15)
- Alcohol use disorder, in sustained remission

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 18

MEDICATIONS:
She takes no medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Generalized anxiety disorder, hypertension
- Father: Alcohol use disorder, type 2 diabetes mellitus
- Sister: Major depressive disorder

SOCIAL HISTORY:
She denies the use of tobacco, alcohol, or illicit drugs.

VITAL SIGNS:
On examination, her vital signs are as follows: blood pressure 140/85 mmHg, heart rate 110 bpm, respiratory rate 18 breaths/min, and temperature 98.6°F (37°C).

PHYSICAL EXAMINATION:
Her physical examination is normal.

OTHER STUDIES:
An electrocardiogram shows sinus tachycardia with no other abnormalities.

## Encounter 2: source 2021-01-15, practice 2025-12-08, ed, Emergency Medicine

Chief complaint: 30-minute history of palpitations, dizziness, and tingling in her hands and around her mouth.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Anika Desai, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
30-minute history of palpitations, dizziness, and tingling in her hands and around her mouth.

HISTORY OF PRESENT ILLNESS:
The patient returns to the Emergency Department with symptoms similar to her prior visit in July 2020; she carries a history of panic disorder initially diagnosed in January 2020. She reports that the episode began suddenly while she was shopping. She has a history of similar episodes over the past year, which she describes as her heart "racing," accompanied by a fear of dying and a sensation of being detached from herself. She denies any chest pain, shortness of breath, or loss of consciousness.

PAST MEDICAL HISTORY:
Active Problem List:
- Panic disorder (diagnosed 2020-01-15)
- Alcohol use disorder, in sustained remission

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 18

MEDICATIONS:
She takes no medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Generalized anxiety disorder, hypertension
- Father: Alcohol use disorder, type 2 diabetes mellitus
- Sister: Major depressive disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former heavy use; in sustained remission for 2 years following inpatient rehabilitation. Occupation: Retail store supervisor.

VITAL SIGNS:
On examination, her heart rate is 110 beats per minute, blood pressure is 135/85 mmHg, respiratory rate is 28 breaths per minute, and oxygen saturation is 99% on room air.

PHYSICAL EXAMINATION:
Her cardiovascular and respiratory examinations are normal. Neurological examination reveals no focal deficits, but she exhibits carpopedal spasm when her blood pressure is measured.

LABORATORY DATA:
An arterial blood gas analysis shows:
- pH: 7.52
- pCO2: 28 mmHg
- HCO3-: 24 mEq/L
- Serum calcium: 8.0 mg/dL (Normal: 8.5-10.2 mg/dL)
