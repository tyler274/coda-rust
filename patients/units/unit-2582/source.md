# synthetic_hospital patient 2582

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex M, Non-Hispanic White, TRICARE.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-24 (Sat) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-11-14 (Fri) | ed | Emergency Medicine |
| 2 | 2021-03-15 | 2026-07-24 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "TRICARE",
 "occupation": "U.S. Army Infantryman (Active Duty)",
 "alcohol_use": "Occasional social use (1-2 drinks weekly)",
 "family_history": [
  "No significant family history"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Fat embolism syndrome", "Traumatic hemorrhagic shock with acute coagulopathy", "Refractory hypoxemia in ARDS"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-05-24, ed, Emergency Medicine

Chief complaint: Brought to emergency department after being shot in the abdomen with significant blood loss at scene

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to emergency department after being shot in the abdomen with significant blood loss at scene

HISTORY OF PRESENT ILLNESS:
He is drowsy and minimally responsive to voice. Paramedics report significant blood loss at the scene.

ALLERGIES:
NKDA

FAMILY HISTORY:
- No significant family history

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks weekly). Occupation: U.S. Army Infantryman (Active Duty).

PHYSICAL EXAMINATION:
On arrival, he is pale and diaphoretic with cold extremities, thready pulses, and delayed capillary refill. There is no jugular venous distension. Breath sounds are clear bilaterally, and heart sounds are normal.

PLAN:
He is intubated, and large-bore IV access is established for fluid resuscitation. A pulmonary artery catheter is inserted for hemodynamic monitoring.

## Encounter 1: source 2020-07-15, practice 2025-11-14, ed, Emergency Medicine

Chief complaint: Found submerged in lake for unknown period, unconscious with GCS 8

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found submerged in lake for unknown period, unconscious with GCS 8

HISTORY OF PRESENT ILLNESS:
The patient returns six months after being shot in the abdomen. Bystanders report that he was last seen swimming 20 minutes before being found. On arrival, he is unconscious, with a Glasgow Coma Scale score of 8.

PAST MEDICAL HISTORY:
Active Problem List:
- Traumatic hemorrhagic shock with acute coagulopathy (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- No significant family history

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks weekly). Occupation: U.S. Army Infantryman (Active Duty).

VITAL SIGNS:
His temperature is 35.6°C (96.1°F), blood pressure is 110/70 mmHg, pulse is 102/min, and respiratory rate is 8/min with shallow breaths. Oxygen saturation is 85% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral crackles on chest auscultation and frothy pink sputum in the oropharynx.

LABORATORY DATA:
Arterial blood gas on 100% oxygen shows:

- pH: 7.25
- PaCO2: 50 mmHg
- PaO2: 60 mmHg
- HCO3-: 24 mEq/L

## Encounter 2: source 2021-03-15, practice 2026-07-24, ed, Emergency Medicine

Chief complaint: Severe pelvic pain and difficulty breathing following high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe pelvic pain and difficulty breathing following high-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department with history of gunshot wound and submersion, presenting after a high-speed motor vehicle collision with severe pelvic pain and difficulty breathing.

PAST MEDICAL HISTORY:
Active Problem List:
- Traumatic hemorrhagic shock with acute coagulopathy (diagnosed 2020-01-15)
- Refractory hypoxemia in ARDS (diagnosed 2020-07-15)

MEDICATIONS:
He takes no medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- No significant family history

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks weekly). Occupation: U.S. Army Infantryman (Active Duty).

VITAL SIGNS:
On examination, his blood pressure is 100/60 mmHg, heart rate is 110 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 89% on room air.

PHYSICAL EXAMINATION:
He appears anxious and confused. Physical examination reveals an unstable pelvis. Auscultation of the lungs demonstrates bilateral crackles. A petechial rash is noted on his chest and axillae.

LABORATORY DATA:
Arterial blood gas shows hypoxemia with a PaO2 of 60 mmHg.

IMAGING:
Chest X-ray reveals bilateral diffuse infiltrates.
