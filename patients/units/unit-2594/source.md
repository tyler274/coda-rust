# synthetic_hospital patient 2594

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex M, Non-Hispanic White, Private employer-sponsored PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-03-22 (Sat) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2025-11-07 (Fri) | ed | Emergency Medicine |
| 2 | 2021-05-15 | 2026-07-16 (Thu) | icu | Trauma Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private employer-sponsored PPO",
 "occupation": "Field sales representative",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "No family history of bleeding disorders or hematologic conditions",
  "Father with hypertension (controlled)",
  "Mother with hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Intra-abdominal hemorrhage requiring operative management", "Traumatic pancreatic injury"]
Comorbidities: ["Blunt abdominal trauma"]

## Encounter 0: source 2020-01-15, practice 2025-03-22, ed, Emergency Medicine

Chief complaint: Persistent abdominal discomfort and nausea for two days following a bicycle accident five days ago.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Persistent abdominal discomfort and nausea for two days following a bicycle accident five days ago.

HISTORY OF PRESENT ILLNESS:
He reports that he was involved in a bicycle accident 5 days ago, where he collided with a car and was thrown over the handlebars, landing on his upper abdomen against the curb. He was initially evaluated at the scene and did not have any apparent injuries, so he did not seek further medical attention.

PAST MEDICAL HISTORY:
His medical history is unremarkable

MEDICATIONS:
He takes no medications.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- No family history of bleeding disorders or hematologic conditions
- Father with hypertension (controlled)
- Mother with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Field sales representative.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, his abdomen is soft but tender to palpation in the epigastric region. There is no rebound tenderness or guarding.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 13.8 g/dL
- White blood cell count: 9,000/mm^3
- Serum amylase: 450 U/L (Normal: 30-110 U/L)
- Serum lipase: 210 U/L (Normal: 0-160 U/L)
- Liver function tests: Normal

IMAGING:
An abdominal ultrasound shows a small amount of peripancreatic fluid collection.

ASSESSMENT AND PLAN:
Based on the clinical scenario,

## Encounter 1: source 2020-09-15, practice 2025-11-07, ed, Emergency Medicine

Chief complaint: Diffuse abdominal pain following a high-speed motor vehicle collision.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Diffuse abdominal pain following a high-speed motor vehicle collision.

HISTORY OF PRESENT ILLNESS:
This 28-year-old male with a prior traumatic pancreatic injury from a January 2020 bicycle accident presents to the emergency department following a high-speed motor vehicle collision. He was the restrained driver, and airbags were deployed. On arrival, he is alert but appears anxious and complains of

PAST MEDICAL HISTORY:
Active Problem List:
- Traumatic pancreatic injury (diagnosed 2020-01-15)

MEDICATIONS:
He takes no medications.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- No family history of bleeding disorders or hematologic conditions
- Father with hypertension (controlled)
- Mother with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Field sales representative.

VITAL SIGNS:
His vital signs are as follows: blood pressure 110/70 mmHg, heart rate 105 bpm, respiratory rate 22 breaths/min, and oxygen saturation 97% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals abdominal distension, tenderness to palpation in all quadrants, and a seatbelt sign across the lower abdomen. There is no rebound tenderness or guarding.

IMAGING:
A Focused Assessment with Sonography for Trauma (FAST) exam is performed and reveals free fluid in the hepatorenal recess (Morison&#x27;s pouch). No other abnormalities are noted on the FAST exam.

## Encounter 2: source 2021-05-15, practice 2026-07-16, icu, Trauma Surgery

Chief complaint: Evaluation following high-speed motor vehicle collision with signs of hemorrhagic shock.

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Priya Patel, MD
Department: Trauma Surgery
Visit Type: icu

CHIEF COMPLAINT:
Evaluation following high-speed motor vehicle collision with signs of hemorrhagic shock.

HISTORY OF PRESENT ILLNESS:
The patient, with history of traumatic pancreatic injury and prior intra-abdominal hemorrhage requiring operative management in 2020, presents following a high-speed motor vehicle collision. He was the restrained driver, and airbags were deployed. On arrival, he is alert but appears anxious and diaphoretic.

PAST MEDICAL HISTORY:
Active Problem List:
- Traumatic pancreatic injury (diagnosed 2020-01-15)
- Intra-abdominal hemorrhage requiring operative management (diagnosed 2020-09-15)

MEDICATIONS:
He takes no medications.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- No family history of bleeding disorders or hematologic conditions
- Father with hypertension (controlled)
- Mother with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Field sales representative.

VITAL SIGNS:
His vital signs are as follows: blood pressure 90/60 mmHg, heart rate 120 beats per minute, respiratory rate 22 breaths per minute, and oxygen saturation 97% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals abdominal distension, guarding, and diffuse tenderness to palpation. No other injuries are identified on the secondary survey.

IMAGING:
A Focused Assessment with Sonography for Trauma (FAST) examination shows free fluid in the hepatorenal recess (Morison&#x27;s pouch) and splenorenal recess.
