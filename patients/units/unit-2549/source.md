# synthetic_hospital patient 2549

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-05 (Sun) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2025-09-05 (Fri) | ed | Emergency Medicine |
| 2 | 2021-05-15 | 2026-05-04 (Mon) | icu | Trauma Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer",
 "alcohol_use": "Heavy episodic drinking, 3-4 times weekly",
 "family_history": [
  "Noncontributory"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current daily smoker, 1 pack per day for 3 years",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Traumatic hemorrhagic shock with acute coagulopathy", "Unstable pelvic fracture", "Traumatic hemothorax with hypovolemic shock"]
Comorbidities: ["Motor vehicle accident", "Acute posthemorrhagic anemia"]

## Encounter 0: source 2020-01-15, practice 2025-01-05, ed, Emergency Medicine

Chief complaint: Stab wound to the left chest sustained during an altercation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Marcus Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Stab wound to the left chest sustained during an altercation

HISTORY OF PRESENT ILLNESS:
He is brought to the emergency department after sustaining a stab wound to the left chest during an altercation.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Noncontributory

SOCIAL HISTORY:
Smoking: Current daily smoker, 1 pack per day for 3 years. Alcohol: Heavy episodic drinking, 3-4 times weekly. Occupation: Construction laborer.

VITAL SIGNS:
His blood pressure is 70/40 mmHg, heart rate is 130 beats per minute, and respiratory rate is 28 breaths per minute.

PHYSICAL EXAMINATION:
He is pale, diaphoretic, and in severe distress. On examination, breath sounds are absent on the left side, and the trachea is deviated to the right.

LABORATORY DATA:
The patient's hemoglobin is 8 g/dL, hematocrit is 24%

IMAGING:
A chest X-ray shows a large opacification of the left hemithorax with a mediastinal shift to the right. Focused Assessment with Sonography for Trauma (FAST) exam is negative for intra-abdominal fluid.

PLAN:
The patient's peripheral intravenous lines are flowing with normal saline, and he has received 2 units of O-negative blood without significant improvement in blood pressure.

## Encounter 1: source 2020-09-15, practice 2025-09-05, ed, Emergency Medicine

Chief complaint: Severe pelvic pain following high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe pelvic pain following high-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
The patient, who sustained a stab wound and hemothorax in January 2020, is brought to the emergency department after being involved in a high-speed motor vehicle collision. He is alert but appears to be in significant distress.

PAST MEDICAL HISTORY:
Active Problem List:
- Traumatic hemothorax with hypovolemic shock (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Noncontributory

SOCIAL HISTORY:
Smoking: Current daily smoker, 1 pack per day for 3 years. Alcohol: Heavy episodic drinking, 3-4 times weekly. Occupation: Construction laborer.

VITAL SIGNS:
On examination, his blood pressure is 90/60 mmHg, heart rate is 120 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
The patient's pelvis is unstable on manual testing, and there is ecchymosis across the lower abdomen and perineum.

LABORATORY DATA:
The patient's hemoglobin level is 9 g/dL, down from a baseline of 14 g/dL measured six months ago during a routine check-up. His hematocrit is 27%, platelet count is 250,000/mm³, and coagulation profile is within normal limits.

IMAGING:
A focused assessment with sonography for trauma (FAST) exam is negative for intra-abdominal fluid.

## Encounter 2: source 2021-05-15, practice 2026-05-04, icu, Trauma Surgery

Chief complaint: Head-on motor vehicle collision with anxious, pale appearance and signs of hemorrhagic shock

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. James O'Brien, MD
Department: Trauma Surgery
Visit Type: icu

CHIEF COMPLAINT:
Head-on motor vehicle collision with anxious, pale appearance and signs of hemorrhagic shock

HISTORY OF PRESENT ILLNESS:
Returning after prior stab wound and pelvic fracture, he was the restrained driver in a head-on collision. On arrival, he is alert but appears anxious and pale.

PAST MEDICAL HISTORY:
Active Problem List:
- Traumatic hemothorax with hypovolemic shock (diagnosed 2020-01-15)
- Unstable pelvic fracture (diagnosed 2020-09-15)

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Noncontributory

SOCIAL HISTORY:
Smoking: Current daily smoker, 1 pack per day for 3 years. Alcohol: Heavy episodic drinking, 3-4 times weekly. Occupation: Construction laborer.

VITAL SIGNS:
His vital signs are as follows: blood pressure 90/60 mmHg, heart rate 120 beats per minute, respiratory rate 22 breaths per minute, and oxygen saturation 97% on room air.
The patient&#x27;s blood pressure drops to 85/55 mmHg despite initial fluid resuscitation.

PHYSICAL EXAMINATION:
Physical examination reveals tenderness in the left upper quadrant of the abdomen with guarding. There is no evidence of external trauma.

LABORATORY DATA:
His hemoglobin level is 9 g/dL, down from a baseline of 14 g/dL measured during a health check-up six months ago.

IMAGING:
A Focused Assessment with Sonography for Trauma (FAST) exam shows intraperitoneal free fluid.
