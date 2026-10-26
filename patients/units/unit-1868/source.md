# synthetic_hospital patient 1868

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex F, White, University student health plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-08-01 (Mon) | ed | Emergency Medicine |
| 1 | 2020-01-15 | 2022-08-01 (Mon) | outpatient | Psychiatry |
| 2 | 2024-01-15 | 2026-07-24 (Fri) | follow_up | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "University student health plan",
 "occupation": "Graduate student",
 "alcohol_use": "Social",
 "family_history": [
  "Mother: Generalized anxiety disorder",
  "Father: Hypertension"
 ],
 "race_ethnicity": "White",
 "smoking_status": "Never",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Primary genital herpes simplex virus infection", "Sexual assault", "Acute stress reaction"]
Comorbidities: ["Contusion of thigh", "Acute stress reaction"]

## Encounter 0: source 2020-01-15, practice 2022-08-01, ed, Emergency Medicine

Chief complaint: Patient reports suspected sexual assault after being forcibly given alcohol and possibly drugged at a college party last night, with limited recollection and waking up in a dorm room with torn clothing.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Maria Santos, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Patient reports suspected sexual assault after being forcibly given alcohol and possibly drugged at a college party last night, with limited recollection and waking up in a dorm room with torn clothing.

HISTORY OF PRESENT ILLNESS:
She has limited recollection of the event but woke up in a dorm room with torn clothing and physical soreness.

PAST MEDICAL HISTORY:
She denies any significant past medical history

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Generalized anxiety disorder
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social. Occupation: Graduate student.

PHYSICAL EXAMINATION:
She is visibly distressed, with a flat affect and poor eye contact. Her physical examination reveals bruising on her upper arms and thighs, and a pelvic examination confirms vaginal trauma.

LABORATORY DATA:
Urine toxicology is positive for benzodiazepines, and a pregnancy test is negative.

ASSESSMENT AND PLAN:
She is concerned about sexually transmitted infections and is unsure about what steps to take next.

## Encounter 1: source 2020-01-15, practice 2022-08-01, outpatient, Psychiatry

Chief complaint: Patient reports flashbacks, intense fear when hearing loud noises, sleep disturbance, feelings of detachment, and persistent guilt following sexual assault 2 nights ago.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Aaliyah Johnson, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Patient reports flashbacks, intense fear when hearing loud noises, sleep disturbance, feelings of detachment, and persistent guilt following sexual assault 2 nights ago.

HISTORY OF PRESENT ILLNESS:
The patient presents for psychiatric evaluation following her earlier Emergency Department visit today for suspected sexual assault. She has not slept well since the incident and has been experiencing flashbacks and intense fear when hearing loud noises or being in a crowded space. She also mentions feeling detached from her surroundings and has a persistent sense of guilt.

PAST MEDICAL HISTORY:
Active Problem List:
- Sexual assault (diagnosed 2020-01-15)

ALLERGIES:
She has no known drug allergies

FAMILY HISTORY:
- Mother: Generalized anxiety disorder
- Father: Hypertension

SOCIAL HISTORY:
She denies any substance abuse and is currently a graduate student.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, she appears anxious and has a flat affect. She has several bruises on her upper extremities and thighs. A pelvic examination is deferred at the patient's request.

LABORATORY DATA:
Laboratory tests reveal the following:

- Complete blood count: within normal limits
- Urine toxicology screen: negative
- Pregnancy test: negative

## Encounter 2: source 2024-01-15, practice 2026-07-24, follow_up, Obstetrics & Gynecology

Chief complaint: Follow-up visit for painful genital ulcers with fever and dysuria initially evaluated in the emergency department 3 days ago.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Emily Chen, MD
Department: Obstetrics & Gynecology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up visit for painful genital ulcers with fever and dysuria initially evaluated in the emergency department 3 days ago.

HISTORY OF PRESENT ILLNESS:
The patient is a 26-year-old woman with a history of sexual assault in 2020 complicated by acute stress reaction who returns to the outpatient clinic for follow-up after being evaluated in the emergency department 3 days ago for painful genital ulcers. At that time, she had fever, dysuria, tender inguinal lymphadenopathy, and several coalescing vesicles on an erythematous base. She now reports modest improvement in pain but is extremely anxious and embarrassed about the diagnosis. She is worried about transmission, recurrence, and long-term consequences. She asks, "Do I have to take this medicine forever? Can I still have children? What about my partner?"

PAST MEDICAL HISTORY:
Active Problem List:
- Sexual assault (diagnosed 2020-01-15)
- Acute stress reaction (diagnosed 2020-01-15)

MEDICATIONS:
Empirical acyclovir was started. is not taking any other medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Generalized anxiety disorder
- Father: Hypertension

SOCIAL HISTORY:
Currently in a new monogamous relationship

PHYSICAL EXAMINATION:
Partial healing of lesions

OTHER STUDIES:
A Tzanck smear revealed multinucleated giant cells
