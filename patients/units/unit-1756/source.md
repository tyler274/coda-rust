# synthetic_hospital patient 1756

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Private (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-12 (Sat) | ed | Emergency Medicine |
| 1 | 2020-03-15 | 2025-06-20 (Fri) | inpatient | Internal Medicine |
| 2 | 2020-09-15 | 2025-12-10 (Wed) | inpatient | Nephrology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private (Blue Cross Blue Shield)",
 "occupation": "Graduate Student (Architecture)",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Major Depressive Disorder",
  "Father: Alcohol Use Disorder",
  "Sister: Anorexia Nervosa"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "100 mg daily",
   "name": "Sertraline"
  },
  {
   "dose": "10 mg nightly",
   "name": "Olanzapine"
  },
  {
   "dose": "500 mg daily",
   "name": "Valacyclovir"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Major Depressive Disorder, recurrent, severe",
  "Anorexia Nervosa, restricting type",
  "Genital Herpes (HSV-2)"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Major Depressive Disorder with Psychotic Features", "Hyperphosphatemia", "Acute kidney injury"]
Comorbidities: ["Cachexia"]

## Encounter 0: source 2020-01-15, practice 2025-04-12, ed, Emergency Medicine

Chief complaint: Brought to the emergency department by her husband due to a 3-week history of severe depression, auditory hallucinations, delusional thinking, and suicidal ideation with a detailed plan.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department by her husband due to a 3-week history of severe depression, auditory hallucinations, delusional thinking, and suicidal ideation with a detailed plan.

HISTORY OF PRESENT ILLNESS:
She believes that she is responsible for a catastrophic event that never occurred. She has lost 10 pounds in the last month because she believes her food is poisoned and refuses to eat. She has also expressed a desire to end her life and has a detailed plan to do so.

PAST MEDICAL HISTORY:
Active Problem List:
- Major Depressive Disorder, recurrent, severe
- Anorexia Nervosa, restricting type
- Genital Herpes (HSV-2)

The patient has a history of major depressive disorder but has never experienced psychotic features before.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Her medications include sertraline and olanzapine, which were started 2 weeks ago without significant improvement.

Home Medications:
- Valacyclovir 500 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Major Depressive Disorder
- Father: Alcohol Use Disorder
- Sister: Anorexia Nervosa

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate Student (Architecture).

VITAL SIGNS:
Her vital signs are as follows: blood pressure 90/60 mmHg, heart rate 110 beats/min, respiratory rate 18 breaths/min, and temperature 36.8°C (98.2°F).

PHYSICAL EXAMINATION:
On examination, she is cachectic, appears dehydrated, and is minimally responsive to questions.

LABORATORY DATA:
Laboratory studies reveal:

- Sodium: 142 mEq/L
- Potassium: 3.9 mEq/L
- Chloride: 104 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen (BUN): 28 mg/dL
- Creatinine: 1.2 mg/dL
- Glucose: 96 mg/dL
- Calcium: 9.2 mg/dL
- Complete blood count, thyroid function tests, and urine toxicology screen are within normal limits.

## Encounter 1: source 2020-03-15, practice 2025-06-20, inpatient, Internal Medicine

Chief complaint: Severe malnutrition with BMI of 15 and muscle weakness with difficulty breathing developing on day 3 of nutritional rehabilitation.

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Severe malnutrition with BMI of 15 and muscle weakness with difficulty breathing developing on day 3 of nutritional rehabilitation.

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old female with Major Depressive Disorder and Anorexia Nervosa, restricting type, who returns for inpatient management following an Emergency Department evaluation two months prior for acute psychiatric decompensation. She has a BMI of 15 and reports a 6-month history of consuming less than 500 calories per day. She is started on a nutritional rehabilitation program with gradual caloric increase. On the third day of refeeding, she complains of muscle weakness and difficulty breathing.

PAST MEDICAL HISTORY:
Active Problem List:
- Major Depressive Disorder with Psychotic Features (diagnosed 2020-01-15)
- Major Depressive Disorder, recurrent, severe
- Anorexia Nervosa, restricting type
- Genital Herpes (HSV-2)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 100 mg daily
- Olanzapine 10 mg nightly
- Valacyclovir 500 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Major Depressive Disorder
- Father: Alcohol Use Disorder
- Sister: Anorexia Nervosa

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate Student (Architecture).

VITAL SIGNS:
On admission, her vital signs are stable

PHYSICAL EXAMINATION:
Physical examination reveals cachexia, dry skin, and lanugo

LABORATORY DATA:
Sodium: 140 mEq/L
Potassium: 3.5 mEq/L
Chloride: 100 mEq/L
Bicarbonate: 24 mEq/L
Blood urea nitrogen (BUN): 15 mg/dL
Creatinine: 0.8 mg/dL
Glucose: 110 mg/dL
Calcium: 8.9 mg/dL
Magnesium: 1.8 mg/dL
Phosphorus: 1.2 mg/dL (Normal: 2.5-4.5 mg/dL)

OTHER STUDIES:
An ECG shows no specific abnormalities.

## Encounter 2: source 2020-09-15, practice 2025-12-10, inpatient, Nephrology

Chief complaint: Altered mental status, fever, generalized vesicular rash, and oliguria developing three days after initiating high-dose intravenous acyclovir for herpes simplex encephalitis.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Priya Patel, MD
Department: Nephrology
Visit Type: inpatient

CHIEF COMPLAINT:
Altered mental status, fever, generalized vesicular rash, and oliguria developing three days after initiating high-dose intravenous acyclovir for herpes simplex encephalitis.

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old female with Major Depressive Disorder, recurrent severe, and Anorexia Nervosa, restricting type, previously hospitalized in January 2020 for severe depression with psychotic features and in March 2020 for complications of nutritional rehabilitation, who presents with decreased urine output. She was diagnosed with herpes simplex encephalitis and started on high-dose intravenous acyclovir three days ago. Her urine output has decreased significantly over the past 24 hours. Her current medications include no other nephrotoxic agents.

PAST MEDICAL HISTORY:
Active Problem List:
- Major Depressive Disorder with Psychotic Features (diagnosed 2020-01-15)
- Hyperphosphatemia (diagnosed 2020-03-15)
- Major Depressive Disorder, recurrent, severe
- Anorexia Nervosa, restricting type
- Genital Herpes (HSV-2)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 100 mg daily
- Olanzapine 10 mg nightly
- Valacyclovir 500 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Major Depressive Disorder
- Father: Alcohol Use Disorder
- Sister: Anorexia Nervosa

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate Student (Architecture).

VITAL SIGNS:
Temperature of 38.7°C, blood pressure of 130/85 mmHg, and heart rate of 102 beats per minute

PHYSICAL EXAMINATION:
She is lethargic but arousable

LABORATORY DATA:
- Serum creatinine: 2.1 mg/dL (baseline 0.9 mg/dL)
- Blood urea nitrogen (BUN): 36 mg/dL
- Serum sodium: 140 mEq/L
- Serum potassium: 4.5 mEq/L
- Urinalysis: No cells or casts, presence of crystals
