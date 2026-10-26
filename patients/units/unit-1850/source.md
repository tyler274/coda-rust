# synthetic_hospital patient 1850

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-19 (Wed) | outpatient | Obstetrics & Gynecology |
| 1 | 2021-07-15 | 2025-12-11 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin (rash)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Marketing coordinator",
 "alcohol_use": "Rare (1-2 drinks/month, none during pregnancy)",
 "family_history": [
  "Maternal grandmother: Hypothyroidism",
  "Mother: Rheumatoid arthritis",
  "Father: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "100 mcg PO daily",
   "name": "Levothyroxine"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction (age 22)"
 ],
 "chronic_conditions": [
  "Hashimoto's thyroiditis",
  "Primary hypothyroidism"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Hypothyroidism in pregnancy (first trimester, inadequately controlled)", "Incomplete spontaneous abortion"]
Comorbidities: ["Pregnancy at 8 weeks gestation"]

## Encounter 0: source 2020-01-15, practice 2024-06-19, outpatient, Obstetrics & Gynecology

Chief complaint: presents to her primary care physician for a routine prenatal visit

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to her primary care physician for a routine prenatal visit

HISTORY OF PRESENT ILLNESS:
She reports feeling more fatigued over the past few weeks, which she attributes to her pregnancy. She denies any other symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto's thyroiditis
- Primary hypothyroidism

She has a history of Hashimoto's thyroiditis.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 22)

MEDICATIONS:
She has been stable on levothyroxine 100 mcg daily

ALLERGIES:
Penicillin (rash)

FAMILY HISTORY:
- Maternal grandmother: Hypothyroidism
- Mother: Rheumatoid arthritis
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks/month, none during pregnancy). Occupation: Marketing coordinator.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests are performed, and the results are as follows:

- TSH: 6.5 mIU/L (normal range for first trimester: 0.1-2.5 mIU/L)
- Total T4: 6.0 mcg/dL (normal range for first trimester: 7.5-14.5 mcg/dL)

## Encounter 1: source 2021-07-15, practice 2025-12-11, ed, Emergency Medicine

Chief complaint: 6-hour history of vaginal bleeding and lower abdominal cramping

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
6-hour history of vaginal bleeding and lower abdominal cramping

HISTORY OF PRESENT ILLNESS:
The patient with Hashimoto's thyroiditis and prior inadequately controlled hypothyroidism in pregnancy reports moderate bleeding and has passed a few small clots. Her last menstrual period was 8 weeks ago, and she had a positive home pregnancy test 4 weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism in pregnancy (first trimester, inadequately controlled) (diagnosed 2020-01-15)
- Hashimoto's thyroiditis
- Primary hypothyroidism

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Levothyroxine 100 mcg PO daily

ALLERGIES:
Penicillin (rash)

FAMILY HISTORY:
- Maternal grandmother: Hypothyroidism
- Mother: Rheumatoid arthritis
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks/month, none during pregnancy). Occupation: Marketing coordinator.

VITAL SIGNS:
On examination, her blood pressure is 110/70 mmHg, pulse is 88 beats per minute, and temperature is 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
Abdominal examination reveals mild suprapubic tenderness. Speculum examination shows blood in the vaginal vault and a dilated cervix. Bimanual examination confirms a soft, enlarged uterus consistent with an 8-week gestation. No tissue is noted at the cervical os.

LABORATORY DATA:
The patient's laboratory results are as follows:

- Hemoglobin: 11.2 g/dL
- White blood cell count: 9,000/mm³
- Platelets: 250,000/mm³
- Blood type: A positive
- Rh factor: Positive
- Quantitative beta-hCG: 12,000 mIU/mL

IMAGING:
A bedside ultrasound is performed, revealing a gestational sac in the lower uterine segment with no fetal heart activity.
