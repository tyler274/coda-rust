# synthetic_hospital patient 2791

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 38 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-09-23 (Mon) | outpatient | Hematology |
| 1 | 2021-04-15 | 2025-12-24 (Wed) | inpatient | General Surgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother with autoimmune hypothyroidism",
  "Maternal cousin with systemic lupus erythematosus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Tonsillectomy (age 12)"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 38
}
```

Primary diagnoses: ["Massive splenomegaly requiring elective splenectomy", "Idiopathic thrombocytopenic purpura"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-09-23, outpatient, Hematology

Chief complaint: Increased bruising over the past two weeks with occasional nosebleeds

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Increased bruising over the past two weeks with occasional nosebleeds

HISTORY OF PRESENT ILLNESS:
She reports noticing easy bruising on her arms and legs without significant trauma. She also has occasional nosebleeds that are difficult to stop.

PAST MEDICAL HISTORY:
She has no history of bleeding disorders

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
She takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with autoimmune hypothyroidism
- Maternal cousin with systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination reveals multiple petechiae and ecchymoses on the extremities. There is no hepatosplenomegaly or lymphadenopathy. Neurologic examination is normal.

LABORATORY DATA:
Hemoglobin                             13.0 g/dL
Hematocrit                             39%
MCV                                    86 fL
Platelet count                         18,000/µL
White blood cell count                 6,800/µL
Bleeding time                          Prolonged
PT                                     12 sec
PTT                                    30 sec

## Encounter 1: source 2021-04-15, practice 2025-12-24, inpatient, General Surgery

Chief complaint: Recurrent thrombocytopenia refractory to medical therapy

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. James Okafor, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Recurrent thrombocytopenia refractory to medical therapy

HISTORY OF PRESENT ILLNESS:
The patient is a 39-year-old female returning for inpatient management following prior Hematology evaluation in January 2020 for increased bruising and occasional nosebleeds. She was diagnosed with immune thrombocytopenic purpura (ITP) one year ago after presenting with gingival bleeding and petechiae. Her platelet count at that time was 7,000/mm³. She was initially treated with corticosteroids and intravenous immunoglobulin, but did not achieve sustained remission. Six months ago, she underwent a laparoscopic splenectomy and her platelet count rose to 160,000/mm³ within two weeks. However, over the past two months, she has noticed easy bruising, increased bleeding during menstruation, and occasional gum bleeding.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic thrombocytopenic purpura (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
She is not taking any medications, including NSAIDs or heparin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with autoimmune hypothyroidism
- Maternal cousin with systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Repeat labs show a platelet count of 17,000/mm³. Hemoglobin, white blood cell count, and coagulation studies are normal. Peripheral smear reveals thrombocytopenia with large platelets and no other abnormalities.
