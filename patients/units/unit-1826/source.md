# synthetic_hospital patient 1826

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex F, White, non-Hispanic, University student health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-01-27 (Mon) | outpatient | Psychiatry |
| 1 | 2021-01-15 | 2021-01-15 (Fri) | outpatient | Obstetrics & Gynecology |
| 2 | 2022-07-15 | 2022-07-28 (Thu) | ed | Emergency Medicine |
| 3 | 2024-07-15 | 2024-07-30 (Tue) | ed | Emergency Medicine |
| 4 | 2026-01-15 | 2026-01-19 (Mon) | ed | Emergency Medicine |
| 5 | 2026-03-15 | 2026-03-27 (Fri) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "University student health insurance",
 "occupation": "Graduate student",
 "alcohol_use": "None",
 "family_history": [
  "Major depressive disorder in mother",
  "Migraine headaches in sister",
  "Alcohol use disorder in maternal uncle"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "50 mg PO PRN migraine headache",
   "name": "Sumatriptan"
  },
  {
   "dose": "150 mg IM every 12 weeks",
   "name": "Depot medroxyprogesterone acetate"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction"
 ],
 "chronic_conditions": [
  "Major depressive disorder, recurrent",
  "Generalized anxiety disorder",
  "Psychogenic non-epileptic seizure disorder",
  "History of anorexia nervosa, in sustained remission",
  "Migraine without aura"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Psychogenic non-epileptic seizures", "Depot medroxyprogesterone acetate contraception", "Major depressive disorder", "Tricyclic antidepressant overdose", "Serotonin syndrome"]
Comorbidities: ["Major depressive disorder", "Situational anxiety"]

## Encounter 0: source 2020-01-15, practice 2020-01-27, outpatient, Psychiatry

Chief complaint: Six-month history of depressed mood, anhedonia, and significant weight loss with difficulty concentrating on graduate studies

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of depressed mood, anhedonia, and significant weight loss with difficulty concentrating on graduate studies

HISTORY OF PRESENT ILLNESS:
She reports feeling tired most of the day and has trouble concentrating on her graduate studies.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Psychogenic non-epileptic seizure disorder
- History of anorexia nervosa, in sustained remission
- Migraine without aura

She has a past medical history of anorexia nervosa, which was actively managed during her teenage years, but she has been in remission for the past 5 years.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Sumatriptan 50 mg PO PRN migraine headache
- Depot medroxyprogesterone acetate 150 mg IM every 12 weeks

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her family history is notable for depression in her mother.

SOCIAL HISTORY:
She denies any substance abuse,

VITAL SIGNS:
Her vital signs are within normal limits,

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for a low BMI of 17.5.

LABORATORY DATA:
Basic metabolic panel and thyroid function tests are normal.

PLAN:
After discussing treatment options, she expresses a preference for pharmacotherapy.

## Encounter 1: source 2021-01-15, practice 2021-01-15, outpatient, Obstetrics & Gynecology

Chief complaint: Concern regarding possible unintended pregnancy following unprotected intercourse 96 hours ago

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Concern regarding possible unintended pregnancy following unprotected intercourse 96 hours ago

HISTORY OF PRESENT ILLNESS:
This 25-year-old with major depressive disorder, initially seen in Psychiatry on 2020-01-15 for depressed mood and anhedonia, returns to OB/GYN regarding her depot medroxyprogesterone contraception. She reports having unprotected intercourse 96 hours ago. Her last menstrual period began 14 days ago, and her cycles are typically 28 days long with no history of irregularity. She denies nausea, breast tenderness, or urinary frequency. She is anxious and requests the most effective emergency contraception available.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Psychogenic non-epileptic seizure disorder
- History of anorexia nervosa, in sustained remission
- Migraine without aura

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Sumatriptan 50 mg PO PRN migraine headache
- Depot medroxyprogesterone acetate 150 mg IM every 12 weeks

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Major depressive disorder in mother
- Migraine headaches in sister
- Alcohol use disorder in maternal uncle

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable

LABORATORY DATA:
Her urine pregnancy test is negative

## Encounter 2: source 2022-07-15, practice 2022-07-28, ed, Emergency Medicine

Chief complaint: Witnessed generalized shaking and falling with thrashing movements lasting approximately 5 minutes

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Witnessed generalized shaking and falling with thrashing movements lasting approximately 5 minutes

HISTORY OF PRESENT ILLNESS:
The patient is a 26-year-old female with established psychogenic non-epileptic seizure disorder and prior outpatient psychiatric and gynecologic care who presents with witnessed generalized shaking. She suddenly started shaking and fell to the ground, with the witness noting she was "thrashing around" and seemed to be resisting attempts to hold her still. Her eyes were tightly closed throughout the episode, which lasted for about 5 minutes.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Depot medroxyprogesterone acetate contraception (diagnosed 2021-01-15)
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Psychogenic non-epileptic seizure disorder
- History of anorexia nervosa, in sustained remission
- Migraine without aura

She has a history of depression and anxiety, for which she has been seeing a therapist but is not currently on medication.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Sumatriptan 50 mg PO PRN migraine headache
- Depot medroxyprogesterone acetate 150 mg IM every 12 weeks

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Major depressive disorder in mother
- Migraine headaches in sister
- Alcohol use disorder in maternal uncle

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

PHYSICAL EXAMINATION:
On examination, the patient is alert and oriented with no confusion. She recalls the entire event and reports a feeling of losing control before the shaking began. There is no tongue biting or urinary incontinence. Her neurological examination is normal.

OTHER STUDIES:
An electroencephalogram (EEG) performed during a similar event last week showed no epileptiform activity.

## Encounter 3: source 2024-07-15, practice 2024-07-30, ed, Emergency Medicine

Chief complaint: Sudden unresponsive episode with eye fluttering lasting approximately 2 minutes witnessed by husband

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden unresponsive episode with eye fluttering lasting approximately 2 minutes witnessed by husband

HISTORY OF PRESENT ILLNESS:
This 28-year-old patient with Major Depressive Disorder and Psychogenic Non-Epileptic Seizure Disorder returns two years after her prior PNES episode, presenting with a witnessed event. Her husband reports that she suddenly started shaking and fell to the ground. He also noticed her eyes fluttering and that she was unresponsive for about 2 minutes. She has no history of similar episodes or any. Her husband mentions the limb movements during the episode seemed irregular and not synchronized.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Depot medroxyprogesterone acetate contraception (diagnosed 2021-01-15)
- Psychogenic non-epileptic seizures (diagnosed 2022-07-15)
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Psychogenic non-epileptic seizure disorder
- History of anorexia nervosa, in sustained remission
- Migraine without aura

She has a history of depression and anxiety,.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

MEDICATIONS:
She takes for which she takes sertraline.

Home Medications:
- Sumatriptan 50 mg PO PRN migraine headache
- Depot medroxyprogesterone acetate 150 mg IM every 12 weeks

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Family history of seizures.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

PHYSICAL EXAMINATION:
On examination, she is alert and oriented with no confusion or drowsiness. Her neurological examination is normal.

LABORATORY DATA:
An electrocardiogram (ECG) and complete blood count (CBC) are normal.

OTHER STUDIES:
A continuous video electroencephalogram (EEG) is performed, which does not show any epileptiform activity during a witnessed episode that resembles the home event.

## Encounter 4: source 2026-01-15, practice 2026-01-19, ed, Emergency Medicine

Chief complaint: Acute confusion and agitation suspected to be due to intentional medication overdose

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute confusion and agitation suspected to be due to intentional medication overdose

HISTORY OF PRESENT ILLNESS:
The patient with major depressive disorder and prior seizures returns. They report that she has been depressed recently and suspect she may have taken an overdose of her medication.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Depot medroxyprogesterone acetate contraception (diagnosed 2021-01-15)
- Psychogenic non-epileptic seizures (diagnosed 2022-07-15)
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Psychogenic non-epileptic seizure disorder
- History of anorexia nervosa, in sustained remission
- Migraine without aura

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Sumatriptan 50 mg PO PRN migraine headache
- Depot medroxyprogesterone acetate 150 mg IM every 12 weeks

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Major depressive disorder in mother
- Migraine headaches in sister
- Alcohol use disorder in maternal uncle

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
On examination, the patient is tachycardic with a heart rate of 120 bpm, blood pressure is 140/90 mmHg, and she has a temperature of 39.2°C (102.5°F).

PHYSICAL EXAMINATION:
She is disoriented to time and place. Her pupils are dilated and poorly responsive to light. Her skin is flushed and dry. Bowel sounds are diminished.

LABORATORY DATA:
Serum electrolytes and other relevant laboratory values are as follows:

 - Sodium: 140 mEq/L
 - Potassium: 3.9 mEq/L
 - Chloride: 100 mEq/L
 - Bicarbonate: 24 mEq/L
 - Blood urea nitrogen: 18 mg/dL
 - Creatinine: 0.9 mg/dL
 - Glucose: 98 mg/dL

OTHER STUDIES:
An ECG shows a QRS duration of 120 msec.

## Encounter 5: source 2026-03-15, practice 2026-03-27, icu, Emergency Medicine

Chief complaint: Acute onset confusion, agitation, and incoherent speech with psychomotor hyperactivity

ENCOUNTER NOTE
Date: 2026-03-15
Provider: Dr. Adebayo Okafor, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Acute onset confusion, agitation, and incoherent speech with psychomotor hyperactivity

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department two months following a tricyclic antidepressant overdose. Over the past day, she has become increasingly restless and disoriented, occasionally speaking incoherently and picking at her clothes.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Depot medroxyprogesterone acetate contraception (diagnosed 2021-01-15)
- Psychogenic non-epileptic seizures (diagnosed 2022-07-15)
- Tricyclic antidepressant overdose (diagnosed 2026-01-15)
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Psychogenic non-epileptic seizure disorder
- History of anorexia nervosa, in sustained remission
- Migraine without aura

PAST SURGICAL HISTORY:
- Wisdom teeth extraction

MEDICATIONS:
She recently started taking a new antidepressant prescribed by her psychiatrist, in addition to her ongoing migraine medication.

Home Medications:
- Sertraline 50 mg PO daily
- Sumatriptan 50 mg PO PRN migraine headache
- Depot medroxyprogesterone acetate 150 mg IM every 12 weeks

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Major depressive disorder in mother
- Migraine headaches in sister
- Alcohol use disorder in maternal uncle

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
On examination, her temperature is 101.8°F (38.8°C), heart rate is 112/min, respiratory rate is 20/min, and blood pressure is 148/94 mm Hg.

PHYSICAL EXAMINATION:
She is diaphoretic and tremulous. Her pupils are dilated. When asked to lift her arm against resistance, she exhibits increased tone and tremor. Deep tendon reflexes are brisk throughout, and clonus is elicited at both ankles. She does not have neck stiffness or focal neurologic deficits. The emergency physician notes no evidence of trauma or signs of infection on examination.

OTHER STUDIES:
ECG and head CT performed urgently are unremarkable.
