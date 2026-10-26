# synthetic_hospital patient 2572

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex M, Non-Hispanic White, Private insurance (parental plan).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-04-09 (Tue) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2024-12-18 (Wed) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2025-10-24 (Fri) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance (parental plan)",
 "occupation": "Construction laborer",
 "alcohol_use": "Social alcohol use with occasional binge drinking (4-5 drinks on weekends)",
 "family_history": [
  "No family history of bleeding disorders or coagulopathy",
  "Father with hypertension",
  "Mother with hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 0.5 pack per day",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Traumatic coagulopathy", "Traumatic hemorrhagic shock with acute coagulopathy", "Penetrating abdominal trauma with hemodynamic instability and peritonitis"]
Comorbidities: ["Traumatic hemorrhagic shock with acute coagulopathy"]

## Encounter 0: source 2020-01-15, practice 2024-04-09, ed, Emergency Medicine

Chief complaint: Stab wound to the left lower quadrant of the abdomen sustained during an altercation, with hemodynamic instability and peritonitis.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Stab wound to the left lower quadrant of the abdomen sustained during an altercation, with hemodynamic instability and peritonitis.

HISTORY OF PRESENT ILLNESS:
He is brought to the emergency department after sustaining a stab wound to the left lower quadrant of the abdomen during an altercation. He is alert but appears anxious and diaphoretic.

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of bleeding disorders or coagulopathy
- Father with hypertension
- Mother with hypothyroidism

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 pack per day. Alcohol: Social alcohol use with occasional binge drinking (4-5 drinks on weekends). Occupation: Construction laborer.

VITAL SIGNS:
His vital signs are as follows: blood pressure 90/60 mmHg, heart rate 120 beats per minute, respiratory rate 22 breaths per minute, and temperature 37.2°C.

PHYSICAL EXAMINATION:
Physical examination reveals a 3 cm laceration with protruding omentum in the left lower quadrant. There is marked tenderness on palpation of the abdomen with involuntary guarding. Bowel sounds are diminished.

LABORATORY DATA:
The patient's hemoglobin is 9.8 g/dL (normal: 13.8-17.2 g/dL), hematocrit 29.4% (normal: 40.7-50.3%), and white blood cell count 15,000/mm³ (normal: 4,500-11,000/mm³).

IMAGING:
A focused assessment with sonography for trauma (FAST) exam shows free fluid in the abdomen.

ASSESSMENT AND PLAN:
Based on the clinical presentation and findings,

## Encounter 1: source 2020-09-15, practice 2024-12-18, ed, Emergency Medicine

Chief complaint: Motorcycle accident without helmet, found unconscious at the scene with acute traumatic coagulopathy.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Motorcycle accident without helmet, found unconscious at the scene with acute traumatic coagulopathy.

HISTORY OF PRESENT ILLNESS:
He has history of penetrating trauma. He was not wearing a helmet and was found unconscious at the scene.

PAST MEDICAL HISTORY:
Active Problem List:
- Penetrating abdominal trauma with hemodynamic instability and peritonitis (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of bleeding disorders or coagulopathy
- Father with hypertension
- Mother with hypothyroidism

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 pack per day. Alcohol: Social alcohol use with occasional binge drinking (4-5 drinks on weekends). Occupation: Construction laborer.

VITAL SIGNS:
His vital signs are: blood pressure 95/60 mmHg, heart rate 110 beats per minute, respiratory rate 22 breaths per minute, and oxygen saturation 96% on room air.

PHYSICAL EXAMINATION:
On arrival, his Glasgow Coma Scale score is 12. He is agitated, confused, and vomiting.
Physical examination reveals a contusion over his left temporal area, but no other obvious injuries.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 13.2 g/dL
- Platelet count: 230 x 10^9/L
- Prothrombin time (PT): 14 seconds (normal: 11-13.5 seconds)
- Activated partial thromboplastin time (aPTT): 35 seconds (normal: 30-40 seconds)
- Fibrinogen level: 180 mg/dL (normal: 200-400 mg/dL)
- D-dimer: Elevated

IMAGING:
A non-contrast head CT scan shows a moderate subdural hematoma without significant midline shift.

## Encounter 2: source 2021-07-15, practice 2025-10-24, icu, Emergency Medicine

Chief complaint: High-speed motor vehicle collision with persistent hypotension and hemorrhagic shock despite aggressive fluid resuscitation.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Elena Vasquez, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
High-speed motor vehicle collision with persistent hypotension and hemorrhagic shock despite aggressive fluid resuscitation.

HISTORY OF PRESENT ILLNESS:
With prior traumatic coagulopathy and abdominal trauma, he presents after high-speed collision. He is alert but appears anxious and pale. Blood pressure drops to 85/55 mmHg despite aggressive fluid resuscitation.

PAST MEDICAL HISTORY:
Active Problem List:
- Penetrating abdominal trauma with hemodynamic instability and peritonitis (diagnosed 2020-01-15)
- Traumatic coagulopathy (diagnosed 2020-09-15)

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of bleeding disorders or coagulopathy
- Father with hypertension
- Mother with hypothyroidism

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 pack per day. Alcohol: Social alcohol use with occasional binge drinking (4-5 drinks on weekends). Occupation: Construction laborer.

VITAL SIGNS:
His vital signs are as follows: blood pressure 90/60 mmHg, heart rate 130 beats per minute, respiratory rate 22 breaths per minute, and oxygen saturation 97% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals abdominal distension with guarding and tenderness to palpation in the left upper quadrant. There is no external bleeding noted.

LABORATORY DATA:
Hemoglobin is 9 g/dL (decreased from a baseline of 14 g/dL), hematocrit is 27% (decreased from a baseline of 42%), and platelets are 250,000/mm³.

IMAGING:
Focused Assessment with Sonography for Trauma (FAST) exam shows free fluid in the abdomen.
