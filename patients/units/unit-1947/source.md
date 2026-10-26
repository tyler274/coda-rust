# synthetic_hospital patient 1947

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-25 (Fri) | ed | Emergency Medicine |
| 1 | 2021-01-15 | 2026-07-30 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Graphic designer",
 "alcohol_use": "Social (2-3 drinks per month)",
 "family_history": [
  "Father: Hypertension (diagnosed age 62)",
  "Mother: Hypothyroidism",
  "No family history of sudden cardiac death, cardiomyopathy, or inherited arrhythmia syndromes"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Wisdom teeth extraction (age 20)",
  "Tonsillectomy (age 8)"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Atrioventricular Nodal Re-entrant Tachycardia"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-07-25, ed, Emergency Medicine

Chief complaint: 1-hour history of palpitations and lightheadedness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
1-hour history of palpitations and lightheadedness

HISTORY OF PRESENT ILLNESS:
She describes the palpitations as sudden in onset, with a rapid, regular heart rate that makes her feel like her heart is "racing out of her chest." She denies chest pain, dyspnea, or syncope.

PAST MEDICAL HISTORY:
Her past medical history is significant for occasional similar episodes over the past year, which usually self-resolve within minutes. She has no known cardiac disease

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 20)
- Tonsillectomy (age 8)

MEDICATIONS:
She takes no medications.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 62)
- Mother: Hypothyroidism
- No family history of sudden cardiac death, cardiomyopathy, or inherited arrhythmia syndromes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per month). Occupation: Graphic designer.

VITAL SIGNS:
On examination, her blood pressure is 120/78 mmHg, heart rate is 180 beats per minute, respiratory rate is 18 breaths per minute, and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
Her jugular venous pressure is not elevated, and lung fields are clear. Cardiac examination reveals a regular tachycardia without murmurs, rubs, or gallops.

OTHER STUDIES:
An electrocardiogram (ECG) is performed and shows a narrow QRS complex tachycardia with absent P waves. There are no signs of ischemia or infarction.

## Encounter 1: source 2021-01-15, practice 2026-07-30, ed, Emergency Medicine

Chief complaint: Recurrent 1-hour history of palpitations and lightheadedness

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent 1-hour history of palpitations and lightheadedness

HISTORY OF PRESENT ILLNESS:
Patient returns with recurrent palpitations similar to one year prior. She describes the palpitations as sudden in onset, with a "racing" heart sensation. She has no chest pain, dyspnea, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrioventricular Nodal Re-entrant Tachycardia (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 20)
- Tonsillectomy (age 8)

MEDICATIONS:
She takes no medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 62)
- Mother: Hypothyroidism
- No family history of sudden cardiac death, cardiomyopathy, or inherited arrhythmia syndromes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per month). Occupation: Graphic designer.

VITAL SIGNS:
On examination, her blood pressure is 120/78 mmHg, heart rate is 180 beats per minute, respiratory rate is 18 breaths per minute, and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
The cardiac examination reveals a regular, rapid rhythm without murmurs, rubs, or gallops. The remainder of the physical examination is unremarkable.

OTHER STUDIES:
An electrocardiogram (ECG) is performed and shows a narrow complex tachycardia with no visible P waves before each QRS complex.

PLAN:
Vagal maneuvers are performed, and the tachycardia terminates with restoration of a normal sinus rhythm.
