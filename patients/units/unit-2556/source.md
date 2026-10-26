# synthetic_hospital patient 2556

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex M, South Asian, University student health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-05-04 (Thu) | outpatient | Cardiology |
| 1 | 2020-07-15 | 2023-10-30 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2024-10-23 (Wed) | outpatient | Internal Medicine |
| 3 | 2022-07-15 | 2025-11-02 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "University student health insurance",
 "occupation": "Graduate student",
 "alcohol_use": "None",
 "family_history": [
  "No significant family history of cardiovascular disease, syncope, or sudden cardiac death"
 ],
 "race_ethnicity": "South Asian",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Starvation", "Fasting metabolic state with PKA activation", "Orthostatic hypotension"]
Comorbidities: ["Dehydration"]

## Encounter 0: source 2020-01-15, practice 2023-05-04, outpatient, Cardiology

Chief complaint: Mild lightheadedness with abrupt postural changes during research protocol

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Mild lightheadedness with abrupt postural changes during research protocol

HISTORY OF PRESENT ILLNESS:
Participates in a research study on cardiovascular responses to acute changes in posture. After lying supine for 10 minutes, he is instructed to stand abruptly. He experiences mild lightheadedness for a few seconds, but then stabilizes without syncope.

ALLERGIES:
NKDA

FAMILY HISTORY:
- No significant family history of cardiovascular disease, syncope, or sudden cardiac death

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
Vital signs reveal a transient drop in systolic blood pressure followed by an increase in heart rate. Repeat measurements 30 seconds after standing show restoration of blood pressure close to baseline.

## Encounter 1: source 2020-07-15, practice 2023-10-30, outpatient, Internal Medicine

Chief complaint: Fatigue and lightheadedness during 48-hour religious fast

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and lightheadedness during 48-hour religious fast

HISTORY OF PRESENT ILLNESS:
The patient returns with lightheadedness and fatigue, similar to January 2020 postural episodes, and is participating in a 48-hour religious fast during which he has only consumed water. He last ate a large meal consisting of rice, lentils, and vegetables before beginning the fast.

PAST MEDICAL HISTORY:
Active Problem List:
- Orthostatic hypotension (diagnosed 2020-01-15)

MEDICATIONS:
He takes no medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- No significant family history of cardiovascular disease, syncope, or sudden cardiac death

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
Vital signs are stable.

PHYSICAL EXAMINATION:
His physical exam is unremarkable aside from mild orthostatic hypotension.

LABORATORY DATA:
Basic metabolic panel is normal except for a serum glucose of 68 mg/dL.

ASSESSMENT AND PLAN:
The liver is primarily responsible for maintaining his blood glucose at this stage of fasting. Hormonal signaling has triggered a shift in hepatic enzyme activity through changes in phosphorylation status.

## Encounter 2: source 2021-07-15, practice 2024-10-23, outpatient, Internal Medicine

Chief complaint: Fatigue and 2-3 pound weight loss after 3-day hiking trip with limited food intake

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James O'Brien, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and 2-3 pound weight loss after 3-day hiking trip with limited food intake

HISTORY OF PRESENT ILLNESS:
The patient returns with recurrent fasting-related fatigue, having presented in July 2020 with similar symptoms during a 48-hour religious fast. He is participating in a three-day hiking trip through a remote national park. Due to limited supplies, he has consumed only small amounts of jerky and water over the past two days. He reports feeling tired but mentally alert. He has lost about 2–3 pounds during the trip but denies nausea, vomiting, or abdominal pain. He last urinated several hours ago and notes that it was dark yellow.

PAST MEDICAL HISTORY:
Active Problem List:
- Orthostatic hypotension (diagnosed 2020-01-15)
- Starvation (diagnosed 2020-07-15)

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- No significant family history of cardiovascular disease, syncope, or sudden cardiac death

SOCIAL HISTORY:
He denies alcohol or drug use

VITAL SIGNS:
His vital signs are within normal limits

PHYSICAL EXAMINATION:
Physical exam shows a lean but well-hydrated man with no jaundice or hepatosplenomegaly

ASSESSMENT AND PLAN:
While sitting by the campfire, he and a friend discuss how the body adapts to fasting, especially the metabolic shift in the liver during prolonged periods without food. In the fasting state, glucagon levels rise and insulin levels fall. This hormonal environment favors hepatic glucose production to maintain euglycemia. Glucagon acts by activating protein kinase A (PKA), which phosphorylates key liver enzymes. In contrast to insulin, which promotes dephosphorylation, glucagon-induced phosphorylation favors enzymes that drive gluconeogenesis and glycogenolysis while suppressing glycolysis and glycogenesis. These phosphorylation changes occur within minutes and are a primary regulatory mechanism during fasting.

## Encounter 3: source 2022-07-15, practice 2025-11-02, ed, Emergency Medicine

Chief complaint: Collapse immediately following 10-kilometer race with dizziness and lightheadedness

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Collapse immediately following 10-kilometer race with dizziness and lightheadedness

HISTORY OF PRESENT ILLNESS:
He returns with similar lightheadedness, having experienced postural symptoms in 2020 and fasting-related episodes in 2020 and 2021. He reports feeling dizzy and lightheaded upon finishing the race but denies any loss of consciousness. He is well-hydrated and has been training for several months. His symptoms resolve with lying down and leg elevation.

PAST MEDICAL HISTORY:
Active Problem List:
- Orthostatic hypotension (diagnosed 2020-01-15)
- Starvation (diagnosed 2020-07-15)
- Fasting metabolic state with PKA activation (diagnosed 2021-07-15)

ALLERGIES:
He has no known drug allergies

FAMILY HISTORY:
- No significant family history of cardiovascular disease, syncope, or sudden cardiac death

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
On examination, his blood pressure is 90/60 mmHg, heart rate is 58 beats per minute, and he is afebrile.

PHYSICAL EXAMINATION:
His skin is cool and moist, and his capillary refill time is less than 2 seconds. His cardiovascular examination is normal with no murmurs, rubs, or gallops. His lungs are clear to auscultation bilaterally. The remainder of the physical examination is unremarkable.
