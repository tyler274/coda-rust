# synthetic_hospital patient 2607

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex M, Non-Hispanic White, University student health insurance plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-05-18 (Mon) | outpatient | Dermatology |
| 1 | 2020-07-15 | 2020-11-02 (Mon) | outpatient | Orthopedics |
| 2 | 2023-01-15 | 2023-05-15 (Mon) | ed | Emergency Medicine |
| 3 | 2025-01-15 | 2025-05-02 (Fri) | ed | Emergency Medicine |
| 4 | 2026-01-15 | 2026-05-20 (Wed) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "University student health insurance plan",
 "occupation": "Nursing student / Part-time auto mechanic and swimming instructor",
 "alcohol_use": "Social drinker, 1-2 drinks per week",
 "family_history": [
  "Father: Hypertension (diagnosed age 55)",
  "Mother: Hypothyroidism (diagnosed age 50)",
  "No known family history of autoimmune disorders or complement deficiencies"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Occupational exposure to hepatitis A virus", "Infraspinatus dysfunction due to suprascapular nerve injury", "Tinea pedis", "Meningococcal meningitis", "Tetanus"]
Comorbidities: ["Occupational mechanical strain", "Underimmunization status for pertussis vaccine"]

## Encounter 0: source 2020-01-15, practice 2020-05-18, outpatient, Dermatology

Chief complaint: Pruritic, burning sensations on the soles of his feet and between his toes for 1 week

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Emily Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Pruritic, burning sensations on the soles of his feet and between his toes for 1 week

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 1-week history of pruritic, burning sensations on the soles of his feet and between his toes. He works as a swimming instructor and wears occlusive footwear for prolonged periods. He denies fever or systemic symptoms.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 55)
- Mother: Hypothyroidism (diagnosed age 50)
- No known family history of autoimmune disorders or complement deficiencies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social drinker, 1-2 drinks per week. Occupation: Nursing student / Part-time auto mechanic and swimming instructor.

PHYSICAL EXAMINATION:
On physical examination, there is erythema and maceration between the fourth and fifth toes of the right foot with scaling and fissuring of the interdigital spaces.

IMAGING:
The photograph of the affected area is shown.

PLAN:
Pharmacologic therapy is initiated. Inhibition of

## Encounter 1: source 2020-07-15, practice 2020-11-02, outpatient, Orthopedics

Chief complaint: Persistent right shoulder weakness and discomfort with difficulty reaching behind head

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent right shoulder weakness and discomfort with difficulty reaching behind head

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old male with a history of tinea pedis who presents with persistent shoulder weakness and discomfort. He works as a mechanic and first noticed difficulty reaching behind his head and rotating his arm outward when retrieving tools above shoulder level. He has no history of trauma, dislocation, or prior surgery. He says the symptoms began gradually over the past month and have worsened, particularly after prolonged use of pneumatic tools with overhead reaching. He has no neck pain, sensory changes, or systemic symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Tinea pedis (diagnosed 2020-01-15)

MEDICATIONS:
He takes no medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 55)
- Mother: Hypothyroidism (diagnosed age 50)
- No known family history of autoimmune disorders or complement deficiencies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social drinker, 1-2 drinks per week. Occupation: Nursing student / Part-time auto mechanic and swimming instructor.

PHYSICAL EXAMINATION:
On physical examination, there is no visible deformity or swelling. Range of motion is preserved except for weakness and mild discomfort during resisted external rotation of the right shoulder with the arm at the side. Abduction and forward flexion are symmetric and painless. There is no scapular winging, and neurologic examination of the upper extremities reveals normal strength, reflexes, and sensation. The full-can and empty-can tests are negative.

IMAGING:
Radiographs of the shoulder are unremarkable.

## Encounter 2: source 2023-01-15, practice 2023-05-15, ed, Emergency Medicine

Chief complaint: Needlestick injury while administering IV medication to patient with chronic viral hepatitis

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Needlestick injury while administering IV medication to patient with chronic viral hepatitis

HISTORY OF PRESENT ILLNESS:
This 31-year-old male with prior complaints of foot pruritus and shoulder weakness presents to the occupational health clinic after experiencing a needlestick injury while administering an intravenous medication to a patient known to have chronic hepatitis B. The nurse cannot recall his hepatitis B vaccination status.

PAST MEDICAL HISTORY:
Active Problem List:
- Tinea pedis (diagnosed 2020-01-15)
- Infraspinatus dysfunction due to suprascapular nerve injury (diagnosed 2020-07-15)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 55)
- Mother: Hypothyroidism (diagnosed age 50)
- No known family history of autoimmune disorders or complement deficiencies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social drinker, 1-2 drinks per week. Occupation: Nursing student / Part-time auto mechanic and swimming instructor.

REVIEW OF SYSTEMS:
He denies any symptoms

VITAL SIGNS:
On examination, he is afebrile, with normal vital signs

PHYSICAL EXAMINATION:
No jaundice or hepatomegaly. His skin shows a small puncture wound on his right index finger, with no signs of infection

LABORATORY DATA:
Laboratory tests are ordered, and the results are as follows:

- Hepatitis B surface antigen (HBsAg): Negative
- Hepatitis B e antigen (HBeAg): Not done
- Hepatitis B surface antibody (anti-HBs): Negative
- Hepatitis B core antibody (total anti-HBc): Not done

ASSESSMENT AND PLAN:
Given the nurse's uncertain vaccination status and exposure to a patient with chronic hepatitis B,

## Encounter 3: source 2025-01-15, practice 2025-05-02, ed, Emergency Medicine

Chief complaint: Stepped on a rusty nail while working in backyard with concern for clostridial infection

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Ahmed Hassan, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Stepped on a rusty nail while working in backyard with concern for clostridial infection

HISTORY OF PRESENT ILLNESS:
Patient has a history of tinea pedis. He presents to the emergency department after stepping on a rusty nail while working in his backyard.

PAST MEDICAL HISTORY:
Active Problem List:
- Tinea pedis (diagnosed 2020-01-15)
- Infraspinatus dysfunction due to suprascapular nerve injury (diagnosed 2020-07-15)
- Occupational exposure to hepatitis A virus (diagnosed 2023-01-15)

He is unsure when he last received a tetanus booster.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 55)
- Mother: Hypothyroidism (diagnosed age 50)
- No known family history of autoimmune disorders or complement deficiencies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social drinker, 1-2 drinks per week. Occupation: Nursing student / Part-time auto mechanic and swimming instructor.

PHYSICAL EXAMINATION:
He has a 3-cm puncture wound on the plantar surface of his right foot.

ASSESSMENT AND PLAN:
This vaccine protects against disease caused by a neurotoxin that blocks inhibitory neurotransmitter release in motor neurons. The immunologic component used to generate protective immunity in this vaccine is best described as

The physician administers a tetanus vaccine as part of his wound management.

## Encounter 4: source 2026-01-15, practice 2026-05-20, icu, Emergency Medicine

Chief complaint: Sudden-onset confusion, fever, and severe headache with neck stiffness

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Sarah O'Connor, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden-onset confusion, fever, and severe headache with neck stiffness

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old male with a history of prior emergency department presentations for tetanus prophylaxis in 2025 and occupational hepatitis A exposure in 2023 who is brought to the emergency department by his coworker due to sudden-onset confusion, fever, and headache. Earlier that day, he had complained of a pounding headache and neck stiffness, but went to work anyway. Over the course of a few hours, he became increasingly disoriented and began speaking incoherently. He also reported photophobia and had vomited twice.

PAST MEDICAL HISTORY:
Active Problem List:
- Tinea pedis (diagnosed 2020-01-15)
- Infraspinatus dysfunction due to suprascapular nerve injury (diagnosed 2020-07-15)
- Occupational exposure to hepatitis A virus (diagnosed 2023-01-15)
- Tetanus (diagnosed 2025-01-15)

Has not traveled recently. He is up to date on most vaccinations but never received a meningococcal booster after high school.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 55)
- Mother: Hypothyroidism (diagnosed age 50)
- No known family history of autoimmune disorders or complement deficiencies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social drinker, 1-2 drinks per week. Occupation: Nursing student / Part-time auto mechanic and swimming instructor.

VITAL SIGNS:
On arrival, he is febrile to 39.7°C (103.5°F)

PHYSICAL EXAMINATION:
Appears acutely ill. He is somnolent but arousable and follows only simple commands. Neurologic exam reveals nuchal rigidity and positive Brudzinski and Kernig signs. Pupils are reactive, and no focal motor deficits are noted. Fundoscopic exam is normal. There is no rash on his skin, and cardiopulmonary findings are unremarkable.

LABORATORY DATA:
Within 6 hours, blood cultures return positive for gram-negative diplococci.

IMAGING:
A non-contrast head CT shows no evidence of mass effect or midline shift.

OTHER STUDIES:
A lumbar puncture is performed after imaging.
