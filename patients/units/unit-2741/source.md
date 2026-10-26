# synthetic_hospital patient 2741

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 49 at first encounter, sex F, Non-Hispanic White, Commercial PPO (Blue Cross).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-06-30 (Fri) | outpatient | Psychiatry |
| 1 | 2023-01-15 | 2026-07-06 (Mon) | outpatient | Psychiatry |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO (Blue Cross)",
 "occupation": "Marketing Director",
 "alcohol_use": "None",
 "family_history": [
  "Major depressive disorder (mother)",
  "Hypothyroidism (maternal grandmother)",
  "Alcohol use disorder (father)",
  "Suicide (maternal uncle)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "88 mcg daily",
   "name": "Levothyroxine"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (2015)",
  "Tonsillectomy (childhood)"
 ],
 "chronic_conditions": [
  "Hypothyroidism"
 ],
 "age_at_first_encounter": 49
}
```

Primary diagnoses: ["Major depressive disorder"]
Comorbidities: ["Hypothyroidism"]

## Encounter 0: source 2020-01-15, practice 2023-06-30, outpatient, Psychiatry

Chief complaint: persistent sadness, loss of interest in activities she once enjoyed, and difficulty sleeping for 6 months following the death of her husband

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent sadness, loss of interest in activities she once enjoyed, and difficulty sleeping for 6 months following the death of her husband

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of persistent sadness, loss of interest in activities she once enjoyed, and difficulty sleeping. She reports that these symptoms began shortly after the death of her husband. She feels worthless and has recurrent thoughts of death, although she denies any specific plans for suicide. She has lost 10 pounds over the last two months without trying.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism

Her past medical history is significant for hypothyroidism.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2015)
- Tonsillectomy (childhood)

MEDICATIONS:
Levothyroxine

ALLERGIES:
NKDA

FAMILY HISTORY:
- Major depressive disorder (mother)
- Hypothyroidism (maternal grandmother)
- Alcohol use disorder (father)
- Suicide (maternal uncle)

SOCIAL HISTORY:
She denies alcohol or drug use

PHYSICAL EXAMINATION:
On physical examination, she appears disheveled and tearful

LABORATORY DATA:
Her thyroid-stimulating hormone level is within normal limits

## Encounter 1: source 2023-01-15, practice 2026-07-06, outpatient, Psychiatry

Chief complaint: persistent fatigue and difficulty concentrating at work for the past 6 weeks, with loss of interest in previously enjoyed activities including yoga

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue and difficulty concentrating at work for the past 6 weeks, with loss of interest in previously enjoyed activities including yoga

HISTORY OF PRESENT ILLNESS:
The patient is a 52-year-old female with hypothyroidism on levothyroxine and a history of major depressive disorder diagnosed in 2020, who presents with persistent fatigue and difficulty concentrating at work for the past 6 weeks. She reports that she has been feeling sad and has lost interest in activities that she used to enjoy, including her weekly yoga classes. She has trouble falling asleep and wakes up frequently during the night. She denies any recent stressors or changes in her life situation.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Hypothyroidism

Her past medical history is significant for hypothyroidism.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2015)
- Tonsillectomy (childhood)

MEDICATIONS:
She takes levothyroxine

ALLERGIES:
NKDA

FAMILY HISTORY:
- Major depressive disorder (mother)
- Hypothyroidism (maternal grandmother)
- Alcohol use disorder (father)
- Suicide (maternal uncle)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Marketing Director.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable

LABORATORY DATA:
Her thyroid-stimulating hormone (TSH) level is within normal limits

OTHER STUDIES:
A polysomnographic study shows decreased REM latency, increased REM sleep duration, and increased REM sleep density.
