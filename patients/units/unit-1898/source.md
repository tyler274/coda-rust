# synthetic_hospital patient 1898

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-05-30 (Thu) | ed | Emergency Medicine |
| 1 | 2021-01-15 | 2025-06-09 (Mon) | ed | Emergency Medicine |
| 2 | 2022-01-15 | 2026-06-09 (Tue) | icu | Medical ICU |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Maternal grandmother with hypothyroidism (autoimmune)",
  "Father with asthma",
  "Mother with bipolar disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "115/21 mcg, 2 inhalations BID",
   "name": "Fluticasone/salmeterol (Advair HFA)"
  },
  {
   "dose": "90 mcg, 2 inhalations PRN",
   "name": "Albuterol HFA"
  },
  {
   "dose": "0.25 mcg PO daily",
   "name": "Calcitriol"
  },
  {
   "dose": "1250 mg PO TID with meals",
   "name": "Calcium carbonate"
  },
  {
   "dose": "25 mg PO at bedtime (recently titrated from 12.5 mg)",
   "name": "Quetiapine"
  }
 ],
 "surgical_history": [
  "Total thyroidectomy (age 33) for papillary thyroid carcinoma"
 ],
 "chronic_conditions": [
  "Severe persistent asthma",
  "Postsurgical hypoparathyroidism",
  "Bipolar I disorder"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Torsades de Pointes", "Hypokalemia managed with oral replacement (tolerating PO intake)", "Hypoparathyroidism"]
Comorbidities: ["Acquired long QT syndrome requiring cardiology consultation"]

## Encounter 0: source 2020-01-15, practice 2024-05-30, ed, Emergency Medicine

Chief complaint: Several days of muscle cramps, perioral numbness, and generalized fatigue

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Several days of muscle cramps, perioral numbness, and generalized fatigue

HISTORY OF PRESENT ILLNESS:
She presents to the emergency department after several days of muscle cramps, perioral numbness, and generalized fatigue. She denies vomiting, diarrhea, diuretic use, or recent changes in diet.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe persistent asthma
- Postsurgical hypoparathyroidism
- Bipolar I disorder

PAST SURGICAL HISTORY:
- Total thyroidectomy (age 33) for papillary thyroid carcinoma

MEDICATIONS:
Home Medications:
- Fluticasone/salmeterol (Advair HFA) 115/21 mcg, 2 inhalations BID
- Albuterol HFA 90 mcg, 2 inhalations PRN
- Calcitriol 0.25 mcg PO daily
- Calcium carbonate 1250 mg PO TID with meals
- Quetiapine 25 mg PO at bedtime (recently titrated from 12.5 mg)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal grandmother with hypothyroidism (autoimmune)
- Father with asthma
- Mother with bipolar disorder

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical exam reveals a positive Chvostek sign and delayed deep tendon reflexes.

LABORATORY DATA:
Serum studies show a calcium of 7.2 mg/dL and a phosphate of 5.8 mg/dL.

OTHER STUDIES:
ECG reveals a prolonged QT interval.

## Encounter 1: source 2021-01-15, practice 2025-06-09, ed, Emergency Medicine

Chief complaint: Generalized weakness and palpitations with increased albuterol use for asthma symptoms

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Generalized weakness and palpitations with increased albuterol use for asthma symptoms

HISTORY OF PRESENT ILLNESS:
The patient with severe persistent asthma and postsurgical hypoparathyroidism returns to the emergency department one year after prior presentation for muscle cramps and perioral numbness, with complaints of generalized weakness and palpitations beginning earlier in the day. She reports using her albuterol inhaler more frequently over the past week due to an increase in nighttime wheezing and shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypoparathyroidism (diagnosed 2020-01-15)
- Severe persistent asthma
- Postsurgical hypoparathyroidism
- Bipolar I disorder

PAST SURGICAL HISTORY:
- Total thyroidectomy (age 33) for papillary thyroid carcinoma

MEDICATIONS:
Her medications include inhaled corticosteroids, albuterol as needed, and a long-acting β2-agonist.

Home Medications:
- Fluticasone/salmeterol (Advair HFA) 115/21 mcg, 2 inhalations BID
- Albuterol HFA 90 mcg, 2 inhalations PRN
- Calcitriol 0.25 mcg PO daily
- Calcium carbonate 1250 mg PO TID with meals
- Quetiapine 25 mg PO at bedtime (recently titrated from 12.5 mg)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal grandmother with hypothyroidism (autoimmune)
- Father with asthma
- Mother with bipolar disorder

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

REVIEW OF SYSTEMS:
She denies chest pain, fever, or cough.

VITAL SIGNS:
On examination, her heart rate is 110 bpm, blood pressure is 130/85 mmHg, respiratory rate is 20 breaths/min, and oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
She appears anxious. Cardiac examination reveals a regular tachycardia without murmurs, rubs, or gallops. Lung examination shows mild expiratory wheezing bilaterally. Neurological examination is notable for mild diffuse muscle weakness and decreased deep tendon reflexes.

LABORATORY DATA:
Laboratory studies reveal:

- Serum potassium: 2.9 mEq/L (Normal: 3.5-5.0 mEq/L)
- Serum magnesium: 1.8 mg/dL (Normal: 1.7-2.2 mg/dL)
- Serum calcium: 9.2 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum creatinine: 0.9 mg/dL (Normal: 0.6-1.1 mg/dL)
- Blood glucose: 98 mg/dL (Normal: 70-99 mg/dL)

OTHER STUDIES:
An electrocardiogram shows sinus tachycardia with no other abnormalities.

## Encounter 2: source 2022-01-15, practice 2026-06-09, icu, Medical ICU

Chief complaint: Syncope while walking dog preceded by palpitations and lightheadedness

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Medical ICU
Visit Type: icu

CHIEF COMPLAINT:
Syncope while walking dog preceded by palpitations and lightheadedness

HISTORY OF PRESENT ILLNESS:
This 36-year-old female with severe persistent asthma, postsurgical hypoparathyroidism, bipolar I disorder, and prior hypokalemia in 2021, is brought to the emergency department by ambulance after briefly losing consciousness while walking her dog. Her husband says she had been complaining of palpitations and lightheadedness earlier in the day.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypoparathyroidism (diagnosed 2020-01-15)
- Hypokalemia managed with oral replacement (tolerating PO intake) (diagnosed 2021-01-15)
- Severe persistent asthma
- Postsurgical hypoparathyroidism
- Bipolar I disorder

PAST SURGICAL HISTORY:
- Total thyroidectomy (age 33) for papillary thyroid carcinoma

MEDICATIONS:
Her only medication is a recently prescribed antipsychotic.

Home Medications:
- Fluticasone/salmeterol (Advair HFA) 115/21 mcg, 2 inhalations BID
- Albuterol HFA 90 mcg, 2 inhalations PRN
- Calcitriol 0.25 mcg PO daily
- Calcium carbonate 1250 mg PO TID with meals
- Quetiapine 25 mg PO at bedtime (recently titrated from 12.5 mg)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal grandmother with hypothyroidism (autoimmune)
- Father with asthma
- Mother with bipolar disorder

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Heart rate is 112/min. Her blood pressure is 90/62 mm Hg.

PHYSICAL EXAMINATION:
On arrival, she is alert but dizzy

LABORATORY DATA:
Laboratory testing reveals mild hypokalemia and hypomagnesemia.

OTHER STUDIES:
ECG findings are shown below.

PLAN:
She is immediately given intravenous magnesium sulfate.
