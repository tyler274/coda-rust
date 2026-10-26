# synthetic_hospital patient 1923

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Private Insurance (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-03-25 (Mon) | outpatient | Psychiatry |
| 1 | 2022-01-15 | 2026-03-30 (Mon) | outpatient | Psychiatry |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private Insurance (PPO)",
 "occupation": "Marketing Specialist",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Father: Bipolar I disorder",
  "Mother: Major depressive disorder",
  "Maternal grandmother: Generalized anxiety disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [
  "Bipolar I disorder",
  "History of cyclothymic disorder"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Bipolar disorder, current episode depressed", "Cyclothymic disorder"]
Comorbidities: ["Witness to severe trauma"]

## Encounter 0: source 2020-01-15, practice 2024-03-25, outpatient, Psychiatry

Chief complaint: 3-year history of fluctuating mood states with periods of increased energy and productivity alternating with weeks of low mood and fatigue

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
3-year history of fluctuating mood states with periods of increased energy and productivity alternating with weeks of low mood and fatigue

HISTORY OF PRESENT ILLNESS:
She reports periods of increased energy, talkativeness, and decreased need for sleep that last for several days, during which she feels very productive and creative. These episodes are followed by weeks of feeling down, loss of interest in activities, and fatigue. She denies ever having a period of mood disturbance severe enough to impair her social or occupational functioning or to necessitate hospitalization. She has no history of psychotic symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Bipolar I disorder
- History of cyclothymic disorder

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is significant for bipolar disorder in an uncle.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Marketing Specialist.

PHYSICAL EXAMINATION:
Physical examination and routine laboratory tests are unremarkable.

LABORATORY DATA:
Routine laboratory tests are unremarkable. Thyroid function tests are within normal limits.

## Encounter 1: source 2022-01-15, practice 2026-03-30, outpatient, Psychiatry

Chief complaint: 3-week history of persistent sadness, decreased energy, and loss of interest in activities following a period of decreased sleep and increased goal-directed activity

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
3-week history of persistent sadness, decreased energy, and loss of interest in activities following a period of decreased sleep and increased goal-directed activity

HISTORY OF PRESENT ILLNESS:
The patient returns for outpatient psychiatric follow-up, having been evaluated in January 2020 for a three-year history of fluctuating mood states, and presents with a 3-week history of persistent sadness, decreased energy, and loss of interest in activities. She reports a decreased need for sleep and increased goal-directed activity, including starting multiple new projects at work, which occurred a month prior to her current symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Cyclothymic disorder (diagnosed 2020-01-15)
- Bipolar I disorder

She has a history of bipolar I disorder.

MEDICATIONS:
She is currently not on any medication

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is significant for bipolar disorder in her mother.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Marketing Specialist.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Her thyroid-stimulating hormone (TSH) and complete blood count (CBC) are within normal limits.
