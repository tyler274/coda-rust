# synthetic_hospital patient 2558

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex M, Non-Hispanic White, Private insurance through ACA marketplace (Silver tier).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-12 (Sun) | inpatient | Trauma Surgery |
| 1 | 2020-10-15 | 2025-10-12 (Sun) | icu | Critical Care Medicine |
| 2 | 2021-07-15 | 2026-07-06 (Mon) | icu | Trauma Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance through ACA marketplace (Silver tier)",
 "occupation": "Independent delivery contractor (gig economy driver/cyclist)",
 "alcohol_use": "Occasional social use (1-2 drinks per month)",
 "family_history": [
  "Father: Hypertension (diagnosed age 52)",
  "Mother: Hypothyroidism",
  "No known family history of bleeding disorders, clotting abnormalities, or connective tissue disorders"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Tension pneumothorax", "Traumatic hemorrhagic shock with acute coagulopathy", "Traumatic injury of right subclavian vessels"]
Comorbidities: ["Multiple traumatic injuries"]

## Encounter 0: source 2020-01-15, practice 2025-01-12, inpatient, Trauma Surgery

Chief complaint: Pain in right shoulder and difficulty breathing after bicycle accident

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Trauma Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Pain in right shoulder and difficulty breathing after bicycle accident

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department after being involved in a bicycle accident. He was wearing a helmet and did not lose consciousness, but he reports falling onto his right side and striking his shoulder against the curb.

PAST MEDICAL HISTORY:
His past medical history is unremarkable.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 52)
- Mother: Hypothyroidism
- No known family history of bleeding disorders, clotting abnormalities, or connective tissue disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Independent delivery contractor (gig economy driver/cyclist).

VITAL SIGNS:
On examination, his vital signs are stable

PHYSICAL EXAMINATION:
He has tenderness over the right clavicle and decreased breath sounds on the right side of his chest.

IMAGING:
A chest x-ray reveals a right clavicular fracture and a small right-sided pneumothorax. There is no evidence of rib fractures or hemothorax.

ASSESSMENT AND PLAN:
Despite initial stabilization, he develops swelling and ecchymosis in the right supraclavicular area.

The patient is placed on supplemental oxygen and his pain is managed with analgesia.

## Encounter 1: source 2020-10-15, practice 2025-10-12, icu, Critical Care Medicine

Chief complaint: Severe respiratory distress and hemodynamic instability following motor vehicle collision

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Marcus Johnson, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Severe respiratory distress and hemodynamic instability following motor vehicle collision

HISTORY OF PRESENT ILLNESS:
This 28-year-old male with a history of traumatic injury to the right subclavian vessels following a bicycle accident in January 2020 presents after a motor vehicle collision. On arrival, he is hypotensive, tachycardic, and has severe respiratory distress.

PAST MEDICAL HISTORY:
Active Problem List:
- Traumatic injury of right subclavian vessels (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 52)
- Mother: Hypothyroidism
- No known family history of bleeding disorders, clotting abnormalities, or connective tissue disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Independent delivery contractor (gig economy driver/cyclist).

PHYSICAL EXAMINATION:
Physical examination reveals absent breath sounds and hyperresonance on the left, jugular venous distention, and tracheal deviation to the right.

PLAN:
Needle decompression is immediately performed. Once stabilized, the trauma team plans to place a chest tube. At

## Encounter 2: source 2021-07-15, practice 2026-07-06, icu, Trauma Surgery

Chief complaint: Unconscious with multiple traumatic injuries following severe motor vehicle collision

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Sharma, MD
Department: Trauma Surgery
Visit Type: icu

CHIEF COMPLAINT:
Unconscious with multiple traumatic injuries following severe motor vehicle collision

HISTORY OF PRESENT ILLNESS:
Returning after prior trauma, he is unconscious and has multiple traumatic injuries.

PAST MEDICAL HISTORY:
Active Problem List:
- Traumatic injury of right subclavian vessels (diagnosed 2020-01-15)
- Tension pneumothorax (diagnosed 2020-10-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 52)
- Mother: Hypothyroidism
- No known family history of bleeding disorders, clotting abnormalities, or connective tissue disorders

SOCIAL HISTORY:
With a known history of being a Jehovah's Witness
The patient's friend, who arrived with him, states that the patient has always refused blood transfusions in the past due to his religious beliefs. However, no advance directive is present, and the patient's decision-making capacity is currently impaired.

VITAL SIGNS:
His blood pressure is 80/50 mmHg, heart rate is 130 beats per minute, and respiratory rate is 24 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals a pale, diaphoretic male with a distended abdomen and a deformed femur.

LABORATORY DATA:
The patient's hemoglobin level is 6.0 g/dL, hematocrit is 18%, and platelet count is 250,000/µL.
