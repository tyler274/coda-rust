# synthetic_hospital patient 1808

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-05-19 (Tue) | outpatient | Psychiatry |
| 1 | 2022-01-15 | 2022-05-09 (Mon) | outpatient | Psychiatry |
| 2 | 2023-01-15 | 2023-05-13 (Sat) | ed | Emergency Medicine |
| 3 | 2026-01-15 | 2026-05-17 (Sun) | inpatient | Psychiatry |
| 4 | 2026-02-15 | 2026-06-21 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Marketing Project Manager",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Major depressive disorder",
  "Paternal uncle: Bipolar disorder",
  "Father: Hypothyroidism"
 ],
 "race_ethnicity": "White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "300 mg PO BID",
   "name": "Lithium carbonate"
  },
  {
   "dose": "100 mcg PO daily",
   "name": "Levothyroxine"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 12)"
 ],
 "chronic_conditions": [
  "Bipolar I disorder",
  "Hypothyroidism"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Bipolar I disorder, rapid cycling, in remission", "Bipolar II disorder, current episode hypomanic", "Major Depressive Disorder with psychotic features", "Major depressive disorder"]
Comorbidities: ["Risk of Alzheimer's disease", "Hypothyroidism"]

## Encounter 0: source 2020-01-15, practice 2020-05-19, outpatient, Psychiatry

Chief complaint: Follow-up appointment for mood disorder management; reports multiple mood episodes despite medication adherence

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up appointment for mood disorder management; reports multiple mood episodes despite medication adherence

HISTORY OF PRESENT ILLNESS:
She reports experiencing four episodes of major depression and three episodes of mania in the past year, despite adherence to her current medication regimen of lithium monotherapy. She describes her manic episodes as periods of increased energy, reduced need for sleep, and impulsive behaviors such as excessive spending. Her depressive episodes are characterized by significant anhedonia, fatigue, and recurrent thoughts of death.

PAST MEDICAL HISTORY:
Active Problem List:
- Bipolar I disorder
- Hypothyroidism

Her medical history is significant for hypothyroidism, well-controlled on levothyroxine.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
She takes lithium monotherapy, levothyroxine.

Home Medications:
- Lithium carbonate 300 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is notable for bipolar disorder in her mother.

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Marketing Project Manager.

PHYSICAL EXAMINATION:
On examination, her mood is euthymic, and she is not in acute distress.

LABORATORY DATA:
Her current lithium level is 0.8 mEq/L (therapeutic range: 0.6-1.2 mEq/L). Thyroid-stimulating hormone (TSH) and free T4 are within normal limits.

Laboratory values:
- Lithium level: 0.8 mEq/L
- TSH: 2.1 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 1.2 ng/dL (normal range: 0.8-1.8 ng/dL)

## Encounter 1: source 2022-01-15, practice 2022-05-09, outpatient, Psychiatry

Chief complaint: Persistent sadness, anhedonia, and difficulty concentrating for six months with minimal response to antidepressant therapy

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent sadness, anhedonia, and difficulty concentrating for six months with minimal response to antidepressant therapy

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old female with a history of Bipolar I disorder, rapid cycling, and Hypothyroidism, previously evaluated in Psychiatry on January 15, 2020, for mood disorder management, who presents with a 6-month history of persistent sadness, anhedonia, and difficulty concentrating. She reports significant fatigue and a 10-pound weight gain. She has been treated with sertraline 100 mg daily for the past 3 months with only minimal improvement in her mood symptoms. She denies any history of manic or hypomanic episodes. She is distressed by her weight gain and reports a decrease in libido since starting sertraline, which is affecting her relationship with her partner.

PAST MEDICAL HISTORY:
Active Problem List:
- Bipolar I disorder, rapid cycling, in remission (diagnosed 2020-01-15)
- Bipolar I disorder
- Hypothyroidism

Her medical history is notable for hypothyroidism, well-controlled on levothyroxine.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
She takes sertraline 100 mg daily for the past 3 months; levothyroxine.

Home Medications:
- Lithium carbonate 300 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Major depressive disorder
- Paternal uncle: Bipolar disorder
- Father: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Marketing Project Manager.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Thyroid-stimulating hormone (TSH) level is within normal limits.

## Encounter 2: source 2023-01-15, practice 2023-05-13, ed, Emergency Medicine

Chief complaint: Decreased need for sleep, increased talkativeness, and inflated self-esteem with erratic behavior concerning coworkers

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Decreased need for sleep, increased talkativeness, and inflated self-esteem with erratic behavior concerning coworkers

HISTORY OF PRESENT ILLNESS:
The patient is a 35-year-old female with Bipolar I disorder, previously seen in 2022 for depressive symptoms, who presents with a 2-week history of decreased need for sleep, increased talkativeness, and inflated self-esteem. She reports being more productive at work and having started several new projects. However, her coworkers are concerned about her erratic behavior.

PAST MEDICAL HISTORY:
Active Problem List:
- Bipolar I disorder, rapid cycling, in remission (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2022-01-15)
- Bipolar I disorder
- Hypothyroidism

She has a history of two major depressive episodes, the most recent one being six months ago.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg PO BID
- Levothyroxine 100 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is significant for bipolar disorder in her maternal uncle.

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Marketing Project Manager.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Her thyroid function tests are within normal limits.

## Encounter 3: source 2026-01-15, practice 2026-05-17, inpatient, Psychiatry

Chief complaint: Persistent sadness, anhedonia, and feelings of worthlessness with psychotic features requiring admission

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. David Park, MD
Department: Psychiatry
Visit Type: inpatient

CHIEF COMPLAINT:
Persistent sadness, anhedonia, and feelings of worthlessness with psychotic features requiring admission

HISTORY OF PRESENT ILLNESS:
This 38-year-old female with Bipolar I disorder and prior depression in 2022 and hypomania in 2023 presents with a 6-week history of persistent sadness, anhedonia, and feelings of worthlessness. She reports significant weight loss, insomnia, and decreased energy.

PAST MEDICAL HISTORY:
Active Problem List:
- Bipolar I disorder, rapid cycling, in remission (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2022-01-15)
- Bipolar II disorder, current episode hypomanic (diagnosed 2023-01-15)
- Bipolar I disorder
- Hypothyroidism

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg PO BID
- Levothyroxine 100 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is notable for depression in her mother

SOCIAL HISTORY:
She denies substance abuse

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
On mental status examination, she appears disheveled and tearful. She expresses guilt over causing a recent natural disaster by her negative thoughts and believes she is being punished by supernatural forces. She also reports hearing voices telling her she is worthless and should not be alive.

LABORATORY DATA:
Laboratory studies show:

- CBC: within normal limits
- Electrolytes: within normal limits
- Thyroid function tests: within normal limits
- Urine toxicology screen: negative

## Encounter 4: source 2026-02-15, practice 2026-06-21, ed, Emergency Medicine

Chief complaint: Persistent depression with command auditory hallucinations instructing self-harm

ENCOUNTER NOTE
Date: 2026-02-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Persistent depression with command auditory hallucinations instructing self-harm

HISTORY OF PRESENT ILLNESS:
This 38-year-old female with established Bipolar I disorder returns to the emergency department one month following inpatient psychiatric admission for similar symptoms. She presents with a 6-week history of persistent sadness, anhedonia, and feelings of worthlessness. She reports significant weight loss, insomnia, and decreased concentration. She also describes hearing voices telling her that she is a terrible person and that she should harm herself.

PAST MEDICAL HISTORY:
Active Problem List:
- Bipolar I disorder, rapid cycling, in remission (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2022-01-15)
- Bipolar II disorder, current episode hypomanic (diagnosed 2023-01-15)
- Major Depressive Disorder with psychotic features (diagnosed 2026-01-15)
- Bipolar I disorder
- Hypothyroidism

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg PO BID
- Levothyroxine 100 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is significant for depression in her mother.

SOCIAL HISTORY:
She denies any substance use.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On mental status examination, she appears disheveled, is tearful, and has a blunted affect. She endorses a plan for suicide but refuses hospitalization.

LABORATORY DATA:
Laboratory studies show:

- CBC: within normal limits
- Electrolytes: within normal limits
- Thyroid function tests: within normal limits
- Urine toxicology screen: negative
