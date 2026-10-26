# synthetic_hospital patient 1849

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-08 (Mon) | outpatient | Internal Medicine |
| 1 | 2022-01-15 | 2026-07-05 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO",
 "occupation": "Administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Mother with preeclampsia",
  "Maternal history of precipitous labor"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Gestational hypertension"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Precipitous labor", "Carpal tunnel syndrome"]
Comorbidities: ["Pregnancy"]

## Encounter 0: source 2020-01-15, practice 2024-07-08, outpatient, Internal Medicine

Chief complaint: 6-week history of numbness and tingling in right hand affecting the thumb, index, and middle fingers, worse at night and waking her from sleep.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
6-week history of numbness and tingling in right hand affecting the thumb, index, and middle fingers, worse at night and waking her from sleep.

HISTORY OF PRESENT ILLNESS:
Affecting the thumb, index, and middle fingers. She reports that the symptoms are worse at night and occasionally wake her from sleep. She denies any trauma or previous similar episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Gestational hypertension

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She is taking prenatal vitamins

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with preeclampsia
- Maternal history of precipitous labor

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, Tinel's sign is positive over the right carpal tunnel, and Phalen's test reproduces her symptoms after 30 seconds. There is no apparent swelling, erythema, or warmth over the wrist. The remainder of the neurological examination, including strength and reflexes, is normal.

## Encounter 1: source 2022-01-15, practice 2026-07-05, ed, Emergency Medicine

Chief complaint: 4-hour history of regular uterine contractions and lower back pain at term gestation, with history of gestational hypertension.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
4-hour history of regular uterine contractions and lower back pain at term gestation, with history of gestational hypertension.

HISTORY OF PRESENT ILLNESS:
Patient with history of carpal tunnel syndrome presents to the ED. She denies any vaginal bleeding or leakage of fluid. Her prenatal course has been significant for mild gestational hypertension without proteinuria, controlled with diet modification.

PAST MEDICAL HISTORY:
Active Problem List:
- Carpal tunnel syndrome (diagnosed 2020-01-15)
- Gestational hypertension

Her previous pregnancy was uncomplicated and resulted in a full-term vaginal delivery.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with preeclampsia
- Maternal history of precipitous labor

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
On examination, her blood pressure is 138/88 mmHg, heart rate is 88 bpm, and temperature is 37.1°C (98.8°F)

PHYSICAL EXAMINATION:
Abdominal examination reveals a fundal height consistent with gestational age and a uterus that is irritable with palpable contractions every 5 minutes. Fetal heart tones are 140 and 142 bpm. A speculum examination shows a closed cervical os without evidence of bleeding or amniotic fluid leakage.

LABORATORY DATA:
Fetal fibronectin test returns positive.

IMAGING:
Transvaginal ultrasound shows cervical length of 25 mm with no funnelling.
