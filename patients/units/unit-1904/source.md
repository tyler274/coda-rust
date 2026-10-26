# synthetic_hospital patient 1904

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 26 at first encounter, sex F, Non-Hispanic White, Private insurance (parent's plan).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-08-07 (Wed) | ed | Emergency Medicine |
| 1 | 2022-01-15 | 2026-08-03 (Mon) | outpatient | Psychiatry |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance (parent's plan)",
 "occupation": "Graduate student (PhD candidate in Philosophy)",
 "alcohol_use": "None",
 "family_history": [
  "Bipolar disorder in mother",
  "Major depression in maternal grandmother",
  "Suicide in maternal uncle"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "History of major depressive disorder (prior episode, untreated)"
 ],
 "age_at_first_encounter": 26
}
```

Primary diagnoses: ["Bipolar disorder, current episode manic"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-08-07, ed, Emergency Medicine

Chief complaint: Brought to the clinic by her roommate due to unusual behavior for the past 10 days including decreased sleep and excessive spending

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Maria Santos, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the clinic by her roommate due to unusual behavior for the past 10 days including decreased sleep and excessive spending

HISTORY OF PRESENT ILLNESS:
The patient has been sleeping less than 3 hours per night but reports feeling extremely energetic. She has gone on a spontaneous shopping spree, buying expensive electronics she cannot afford, and attempted to contact a news station to share her "revolutionary" philosophical theory. Her speech is pressured and tangential, and she becomes irritable when interrupted.

PAST MEDICAL HISTORY:
Active Problem List:
- History of major depressive disorder (prior episode, untreated)

She had one episode of severe depression 2 years ago that resolved without treatment.

PAST SURGICAL HISTORY:
- None

ALLERGIES:
NKDA

FAMILY HISTORY:
- Bipolar disorder in mother
- Major depression in maternal grandmother
- Suicide in maternal uncle

SOCIAL HISTORY:
There is no history of substance use.

PHYSICAL EXAMINATION:
Physical exam is unremarkable.

LABORATORY DATA:
Urine toxicology is unremarkable.

## Encounter 1: source 2022-01-15, practice 2026-08-03, outpatient, Psychiatry

Chief complaint: Decreased need for sleep, increased goal-directed activity, and excessive involvement in activities with high potential for painful consequences

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Decreased need for sleep, increased goal-directed activity, and excessive involvement in activities with high potential for painful consequences

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old woman with a history of a prior untreated major depressive episode who returns for psychiatric evaluation two years after an emergency department presentation for similar symptoms of decreased sleep and excessive spending. She presents with a 4-day history of decreased need for sleep, increased goal-directed activity, and excessive involvement in activities that have a high potential for painful consequences. She reports feeling "on top of the world" and has been spending large amounts of money online, which she cannot afford. She has also been more talkative than usual and feels that her thoughts are racing. She denies any history of hallucinations or delusions.

PAST MEDICAL HISTORY:
Active Problem List:
- Bipolar disorder, current episode manic (diagnosed 2020-01-15)
- History of major depressive disorder (prior episode, untreated)

Had difficulty concentrating for a period of two months.

PAST SURGICAL HISTORY:
- None

ALLERGIES:
NKDA

FAMILY HISTORY:
- Bipolar disorder in mother
- Major depression in maternal grandmother
- Suicide in maternal uncle

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student (PhD candidate in Philosophy).

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Her thyroid-stimulating hormone (TSH) and complete blood count (CBC) are within normal limits.
