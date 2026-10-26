# synthetic_hospital patient 1765

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Blue Cross PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-07-28 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2023-02-05 (Sun) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2023-08-07 (Mon) | outpatient | Psychiatry |
| 3 | 2022-01-15 | 2024-07-29 (Mon) | outpatient | Psychiatry |
| 4 | 2022-02-15 | 2024-09-02 (Mon) | outpatient | Psychiatry |
| 5 | 2023-01-15 | 2025-07-26 (Sat) | ed | Emergency Medicine |
| 6 | 2023-07-15 | 2026-01-20 (Tue) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2026-07-24 (Fri) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [],
 "insurance": "Blue Cross PPO",
 "occupation": "Graphic designer",
 "alcohol_use": "Former occasional use, currently abstinent",
 "family_history": [
  "Mother: Hypothyroidism",
  "Father: Major depressive disorder",
  "Sister: Bipolar disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "75 mcg daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "300 mg twice daily",
   "name": "Lithium carbonate"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Hypothyroidism",
  "Bipolar I Disorder",
  "Chronic lower back pain",
  "History of Major Depressive Disorder"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Glucocorticoid-induced neuropsychiatric symptoms", "Bipolar Disorder", "Lithium nephrotoxicity", "Hypothyroidism", "Serotonin syndrome", "Lithium toxicity", "Bipolar disorder, current episode depressed", "Major depressive disorder"]
Comorbidities: ["Major depressive disorder", "Hypothyroidism", "Bipolar Disorder"]

## Encounter 0: source 2020-01-15, practice 2022-07-28, outpatient, Internal Medicine

Chief complaint: Fatigue, weight gain, and constipation over 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, weight gain, and constipation over 6 months

HISTORY OF PRESENT ILLNESS:
A 35-year-old female presents to the clinic with a 6-month history of fatigue, weight gain, and constipation. She reports her menstrual cycles have become irregular and she often feels cold, even in warm environments.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism
- Bipolar I Disorder
- Chronic lower back pain
- History of Major Depressive Disorder

MEDICATIONS:
Home Medications:
- Levothyroxine 75 mcg daily
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
Her family history is notable for hypothyroidism in her mother.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former occasional use, currently abstinent. Occupation: Graphic designer.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, her skin is dry, and she has a delayed relaxation phase of her deep tendon reflexes.

LABORATORY DATA:
Laboratory tests reveal:

- Serum TSH: 10 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 0.7 ng/dL (normal range: 0.8-2.0 ng/dL)
- Total cholesterol: 245 mg/dL (normal range: 40 mg/dL)

## Encounter 1: source 2020-07-15, practice 2023-02-05, ed, Emergency Medicine

Chief complaint: Severe depression with suicidal ideation and formulated plan

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Torres, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe depression with suicidal ideation and formulated plan

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with Bipolar I Disorder and hypothyroidism initially diagnosed six months prior during an outpatient Internal Medicine evaluation for fatigue, weight gain, and constipation presents to the emergency department with a 6-month history of depressed mood, anhedonia, and fatigue. She reports significant weight loss, insomnia, and feelings of worthlessness. She admits to having recurrent thoughts of death and has formulated a plan to overdose on her thyroid medication, to which she has easy access at home.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Bipolar I Disorder
- Chronic lower back pain
- History of Major Depressive Disorder

She has a past medical history of hypothyroidism. She denies any past psychiatric history.

MEDICATIONS:
She takes levothyroxine.

Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Major depressive disorder
- Sister: Bipolar disorder

SOCIAL HISTORY:
She denies substance abuse.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On examination, she appears disheveled and tearful. Physical examination is unremarkable.

LABORATORY DATA:
Thyroid function tests from one week ago are as follows:

- TSH: 2.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 1.2 ng/dL (normal range: 0.8-1.8 ng/dL)

## Encounter 2: source 2021-01-15, practice 2023-08-07, outpatient, Psychiatry

Chief complaint: Episodic mood swings over 6 months with alternating elevated and depressed phases

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Episodic mood swings over 6 months with alternating elevated and depressed phases

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old female with a history of hypothyroidism and a prior psychiatric emergency six months ago who presents for follow-up of recurrent mood symptoms. She describes periods of elevated mood, increased energy, and decreased need for sleep that last for about a week and alternate with episodes of profound sadness, fatigue, and social withdrawal lasting up to two weeks. During her elevated mood phases, she reports engaging in impulsive shopping sprees and having increased sexual activity.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Bipolar I Disorder
- Chronic lower back pain

Her medical history is significant for hypothyroidism.

MEDICATIONS:
She takes she takes levothyroxine.

Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
Her family history is notable for bipolar disorder in her mother.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former occasional use, currently abstinent. Occupation: Graphic designer.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Thyroid function tests are within normal limits.

Laboratory Values:
- Sodium: 140 mEq/L (normal 135-145 mEq/L)
- Potassium: 4.1 mEq/L (normal 3.5-5.1 mEq/L)
- Calcium: 9.2 mg/dL (normal 8.5-10.2 mg/dL)
- Creatinine: 0.9 mg/dL (normal 0.6-1.1 mg/dL)
- Glucose: 98 mg/dL (normal 70-99 mg/dL)
- TSH: 2.1 µIU/mL (normal 0.5-5.0 µIU/mL)
- Free T4: 1.2 ng/dL (normal 0.9-1.7 ng/dL)

## Encounter 3: source 2022-01-15, practice 2024-07-29, outpatient, Psychiatry

Chief complaint: Three-week history of persistent sadness, anhedonia, and fatigue with 5kg weight loss

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Three-week history of persistent sadness, anhedonia, and fatigue with 5kg weight loss

HISTORY OF PRESENT ILLNESS:
The patient with known Bipolar I Disorder presents for follow-up care. She reports a significant decrease in her appetite and has lost 5 kg over the last month. She denies any recent stressors or changes in her life situation.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Bipolar Disorder (diagnosed 2021-01-15)
- Bipolar I Disorder
- Chronic lower back pain

Her medical history is significant for bipolar disorder, for which she has been hospitalized twice in the past for manic episodes.

MEDICATIONS:
She has not been on any medication for the last year as she felt well and did not think it was necessary.

Home Medications:
- Levothyroxine 75 mcg daily
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
Her family history is notable for depression in her mother.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former occasional use, currently abstinent. Occupation: Graphic designer.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, she appears disheveled and has psychomotor retardation.

LABORATORY DATA:
Laboratory tests reveal:
- CBC: within normal limits
- Thyroid function tests: within normal limits
- Urine toxicology screen: negative

ASSESSMENT AND PLAN:
Given the patient's history and presentation,

## Encounter 4: source 2022-02-15, practice 2024-09-02, outpatient, Psychiatry

Chief complaint: One-week history of decreased need for sleep, increased goal-directed activity, and excessive involvement in risky activities

ENCOUNTER NOTE
Date: 2022-02-15
Provider: Dr. James O'Brien, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
One-week history of decreased need for sleep, increased goal-directed activity, and excessive involvement in risky activities

HISTORY OF PRESENT ILLNESS:
Patient with known Bipolar I disorder and hypothyroidism returns for evaluation one month after treatment for a depressive episode. She reports feeling "on top of the world" and more productive than ever. She was started on sertraline 50 mg daily 3 weeks ago after reporting a depressive episode characterized by anhedonia, weight loss, and feelings of worthlessness.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Bipolar Disorder (diagnosed 2021-01-15)
- Bipolar disorder, current episode depressed (diagnosed 2022-01-15)
- Bipolar I Disorder
- Chronic lower back pain

MEDICATIONS:
She takes for which she takes levothyroxine.

Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Major depressive disorder
- Sister: Bipolar disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former occasional use, currently abstinent. Occupation: Graphic designer.

VITAL SIGNS:
Vital signs are stable.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable,

LABORATORY DATA:
Thyroid function tests are within normal limits.

## Encounter 5: source 2023-01-15, practice 2025-07-26, ed, Emergency Medicine

Chief complaint: Acute onset confusion, agitation, and tremors over 2 hours

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset confusion, agitation, and tremors over 2 hours

HISTORY OF PRESENT ILLNESS:
This 35-year-old female with Bipolar I Disorder and Hypothyroidism presents to the emergency department with a 2-hour history of confusion, agitation, and tremors.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Bipolar Disorder (diagnosed 2021-01-15)
- Bipolar disorder, current episode depressed (diagnosed 2022-01-15)
- Glucocorticoid-induced neuropsychiatric symptoms (diagnosed 2022-02-15)
- Bipolar I Disorder
- Chronic lower back pain

She has a past medical history of major depressive disorder and chronic back pain.

MEDICATIONS:
Her current medications include sertraline, which was increased in dose 1 week ago, and tramadol, which she took for the first time earlier today.

Home Medications:
- Levothyroxine 75 mcg daily
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Major depressive disorder
- Sister: Bipolar disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former occasional use, currently abstinent. Occupation: Graphic designer.

VITAL SIGNS:
Tachycardic with a heart rate of 110 bpm, and hypertensive with a blood pressure of 160/95 mmHg. She is febrile with a temperature of 38.7°C (101.7°F).

PHYSICAL EXAMINATION:
On examination, she is diaphoretic. Neurological examination reveals hyperreflexia, inducible clonus, and bilateral mydriasis.

LABORATORY DATA:
Laboratory studies show:
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 0.9 mg/dL
- Blood urea nitrogen (BUN): 14 mg/dL
- Complete blood count (CBC): within normal limits
- Creatine kinase (CK): 550 U/L (elevated)

## Encounter 6: source 2023-07-15, practice 2026-01-20, ed, Emergency Medicine

Chief complaint: Erratic behavior and confusion after alcohol binge with potential lithium overdose

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Erratic behavior and confusion after alcohol binge with potential lithium overdose

HISTORY OF PRESENT ILLNESS:
This 35-year-old female with Bipolar I Disorder and prior serotonin syndrome presents with erratic behavior and confusion. She is brought to the emergency department by her sister and was found at home with several empty bottles of vodka and an empty blister pack of her prescribed lithium.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Bipolar Disorder (diagnosed 2021-01-15)
- Bipolar disorder, current episode depressed (diagnosed 2022-01-15)
- Glucocorticoid-induced neuropsychiatric symptoms (diagnosed 2022-02-15)
- Serotonin syndrome (diagnosed 2023-01-15)
- Bipolar I Disorder
- Chronic lower back pain

The patient has a history of bipolar disorder.

MEDICATIONS:
She takes prescribed lithium.

Home Medications:
- Levothyroxine 75 mcg daily
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Major depressive disorder
- Sister: Bipolar disorder

SOCIAL HISTORY:
She is known to use alcohol occasionally

VITAL SIGNS:
Temperature 37.1°C (98.8°F), blood pressure 130/85 mmHg, pulse 88/min, and respiratory rate 16/min

PHYSICAL EXAMINATION:
On examination, the patient is disoriented, has slurred speech, and is unable to follow commands. Physical examination reveals coarse tremors in her hands.

LABORATORY DATA:
- Serum lithium level: 3.5 mEq/L (therapeutic range: 0.6-1.2 mEq/L)
- Blood alcohol concentration: 0.2% (legal intoxication: 0.08%)
- Serum creatinine: 1.1 mg/dL
- Blood urea nitrogen (BUN): 14 mg/dL
- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum calcium: 9.2 mg/dL
- Liver function tests: within normal limits

PLAN:
The patient's sister provides consent for her medical care. The emergency physician recommends hemodialysis for the patient's condition.

## Encounter 7: source 2024-01-15, practice 2026-07-24, outpatient, Internal Medicine

Chief complaint: Routine follow-up for chronic lithium therapy monitoring

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic lithium therapy monitoring

HISTORY OF PRESENT ILLNESS:
She returns for lithium monitoring with history of toxicity. She reports adherence to her medication regimen and denies any recent mood swings, manic episodes, or depressive symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-07-15)
- Bipolar Disorder (diagnosed 2021-01-15)
- Bipolar disorder, current episode depressed (diagnosed 2022-01-15)
- Glucocorticoid-induced neuropsychiatric symptoms (diagnosed 2022-02-15)
- Serotonin syndrome (diagnosed 2023-01-15)
- Lithium toxicity (diagnosed 2023-07-15)
- Bipolar I Disorder
- Chronic lower back pain

She has a history of Bipolar I disorder.

MEDICATIONS:
She has been stable on lithium therapy for the past two years. for which she takes levothyroxine.

Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Major depressive disorder
- Sister: Bipolar disorder

SOCIAL HISTORY:
She is currently employed as a graphic designer and is in a stable relationship. She does not smoke, use illicit drugs, or consume alcohol.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
- Serum lithium level: 0.8 mEq/L (therapeutic range 0.6-1.2 mEq/L)
- Thyroid-stimulating hormone (TSH): 3.2 μIU/mL (normal range 0.4-4.0 μIU/mL)
- Creatinine: 1.4 mg/dL (increased from a baseline of 0.9 mg/dL six months ago)
- Estimated Glomerular Filtration Rate (eGFR): 50 mL/min/1.73m^2 (decreased from 90 mL/min/1.73m^2 six months ago)
- Calcium: 10.2 mg/dL (normal range 8.5-10.2 mg/dL)
