# synthetic_hospital patient 2760

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 38 at first encounter, sex F, Non-Hispanic White, Private, Blue Cross Blue Shield.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-07-04 (Mon) | outpatient | Psychiatry |
| 1 | 2020-07-15 | 2023-01-16 (Mon) | outpatient | Psychiatry |
| 2 | 2021-09-15 | 2024-03-15 (Fri) | outpatient | Internal Medicine |
| 3 | 2023-01-15 | 2025-07-15 (Tue) | ed | Emergency Medicine |
| 4 | 2024-01-15 | 2026-07-05 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private, Blue Cross Blue Shield",
 "occupation": "Retail sales associate",
 "alcohol_use": "None (denies use)",
 "family_history": [
  "Maternal aunt with bipolar disorder",
  "Father with alcohol use disorder",
  "Paternal cousin with neurofibromatosis type 1",
  "Mother with generalized anxiety disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "100 mg PO daily",
   "name": "Sertraline"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Major depressive disorder",
  "Generalized anxiety disorder",
  "Panic disorder",
  "Kleptomania",
  "Pheochromocytoma",
  "Opioid use disorder",
  "History of benzodiazepine use disorder"
 ],
 "age_at_first_encounter": 38
}
```

Primary diagnoses: ["Kleptomania", "Pheochromocytoma", "Intravenous Drug Use (Substance Use Disorder)", "Benzodiazepine withdrawal syndrome", "Major depressive disorder"]
Comorbidities: ["Generalized Anxiety Disorder"]

## Encounter 0: source 2020-01-15, practice 2022-07-04, outpatient, Psychiatry

Chief complaint: presents to the clinic with a 2-month history of persistent sadness, loss of interest in activities, and fatigue.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to the clinic with a 2-month history of persistent sadness, loss of interest in activities, and fatigue.

HISTORY OF PRESENT ILLNESS:
Despite initial improvement, her symptoms have worsened over the past 3 months. She denies any suicidal ideation, recent stressors, or changes in her life situation.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder
- Generalized anxiety disorder
- Panic disorder
- Kleptomania
- Pheochromocytoma
- Opioid use disorder
- History of benzodiazepine use disorder

She has a history of major depressive disorder.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She has been treated with sertraline 100 mg daily for the past year. and is not currently taking any other medications

ALLERGIES:
She has no known drug allergies

FAMILY HISTORY:
- Maternal aunt with bipolar disorder
- Father with alcohol use disorder
- Paternal cousin with neurofibromatosis type 1
- Mother with generalized anxiety disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (denies use). Occupation: Retail sales associate.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Her laboratory values are within normal limits.

PLAN:
After discussing treatment options, she expresses interest in trying a different class of medication and inquires about phenelzine due to a friend's positive experience.

## Encounter 1: source 2020-07-15, practice 2023-01-16, outpatient, Psychiatry

Chief complaint: presents to the clinic with feelings of guilt and shame

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Robert Kim, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to the clinic with feelings of guilt and shame

HISTORY OF PRESENT ILLNESS:
The patient returns to the outpatient psychiatry clinic for follow-up of her depression, currently managed with sertraline, presenting with feelings of guilt and shame. She reports a recurrent failure to resist impulses to steal items that she admits are neither needed for personal use nor of significant value. She describes intense urges prior to the act of stealing, followed by a sense of relief afterwards. The patient is concerned because her behavior is inconsistent with her personal values, and she fears legal consequences.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Generalized anxiety disorder
- Panic disorder
- Kleptomania
- Pheochromocytoma
- Opioid use disorder
- History of benzodiazepine use disorder

She has a history of depression and anxiety.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
For which she takes sertraline

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is non-contributory

SOCIAL HISTORY:
She denies any substance abuse

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Her recent complete blood count, thyroid function tests, and basic metabolic panel are within normal limits

## Encounter 2: source 2021-09-15, practice 2024-03-15, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine follow-up

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to the clinic for a routine follow-up

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up with history of persistent sadness and compulsive behaviors. She reports that it has been effective in managing her pain. However, she mentions feeling anxious when her medication runs low and admits to occasionally taking an extra dose.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Kleptomania (diagnosed 2020-07-15)
- Generalized anxiety disorder
- Panic disorder
- Pheochromocytoma
- Opioid use disorder
- History of benzodiazepine use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She has been on prescription oxycodone for the past 6 months

Home Medications:
- Sertraline 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
A maternal history of alcohol use disorder

SOCIAL HISTORY:
She denies any legal issues.

VITAL SIGNS:
On examination, her vital signs are stable

PHYSICAL EXAMINATION:
There is tenderness to palpation over the lumbar paraspinal muscles without neurological deficits. She has no signs of intoxication or withdrawal.

LABORATORY DATA:
Urine drug screen shows oxycodone and an unexpected low level of noroxycodone, a metabolite of oxycodone.

OTHER STUDIES:
Review of the state’s prescription drug-monitoring program reveals that she has filled her oxycodone prescription at multiple pharmacies.

## Encounter 3: source 2023-01-15, practice 2025-07-15, ed, Emergency Medicine

Chief complaint: presents to the emergency department with a 6-month history of episodic headaches, palpitations, and profuse sweating.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
She presents to the emergency department with a 6-month history of episodic headaches, palpitations, and profuse sweating.

HISTORY OF PRESENT ILLNESS:
Patient with history of depression, kleptomania, and opioid use presents. She reports that these episodes occur suddenly, last for about an hour, and are sometimes accompanied by a sense of doom.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Kleptomania (diagnosed 2020-07-15)
- Intravenous Drug Use (Substance Use Disorder) (diagnosed 2021-09-15)
- Generalized anxiety disorder
- Panic disorder
- Pheochromocytoma
- Opioid use disorder
- History of benzodiazepine use disorder

She has a past medical history of anxiety and panic attacks.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Sertraline

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with bipolar disorder
- Father with alcohol use disorder
- Paternal cousin with neurofibromatosis type 1
- Mother with generalized anxiety disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (denies use). Occupation: Retail sales associate.

VITAL SIGNS:
Her blood pressure in the clinic two weeks ago was 142/90 mmHg. On examination today, her blood pressure is 168/102 mmHg, heart rate is 118 bpm

PHYSICAL EXAMINATION:
She appears anxious and diaphoretic.

LABORATORY DATA:
Laboratory results are as follows:

- Serum glucose: 110 mg/dL
- Serum potassium: 3.8 mEq/L
- Serum creatinine: 0.9 mg/dL
- Plasma metanephrines: Elevated
- Plasma normetanephrines: Elevated

IMAGING:
An abdominal CT scan shows a 3 cm mass in the right adrenal gland.

## Encounter 4: source 2024-01-15, practice 2026-07-05, ed, Emergency Medicine

Chief complaint: increasing anxiety, hand tremors, insomnia, and two episodes of what she describes as 'panic attacks.'

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing anxiety, hand tremors, insomnia, and two episodes of what she describes as 'panic attacks.'

HISTORY OF PRESENT ILLNESS:
The patient is a 42-year-old female with benzodiazepine use disorder, pheochromocytoma, and psychiatric comorbidities who returns with a 2-day history of increasing anxiety, hand tremors, insomnia, and two episodes of what she describes as "panic attacks." She reports that she ran out of her medication 3 days ago and has not been able to get a refill.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Kleptomania (diagnosed 2020-07-15)
- Intravenous Drug Use (Substance Use Disorder) (diagnosed 2021-09-15)
- Pheochromocytoma (diagnosed 2023-01-15)
- Generalized anxiety disorder
- Panic disorder
- Opioid use disorder
- History of benzodiazepine use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes alprazolam 2 mg daily for the past 6 months.

Home Medications:
- Sertraline 100 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with bipolar disorder
- Father with alcohol use disorder
- Paternal cousin with neurofibromatosis type 1
- Mother with generalized anxiety disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (denies use). Occupation: Retail sales associate.

VITAL SIGNS:
Blood pressure 150/95 mmHg, heart rate 110 beats per minute, respiratory rate 22 breaths per minute, and temperature 37.2°C (99.0°F)

PHYSICAL EXAMINATION:
Anxious-appearing woman with diaphoresis and bilateral hand tremors. Neurological examination is nonfocal.
