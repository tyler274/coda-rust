# synthetic_hospital patient 1983

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-13 (Sun) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-04-02 (Wed) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2026-04-09 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Construction supervisor",
 "alcohol_use": "Occasional social use, 1-2 drinks per week",
 "family_history": [
  "Father: Myocardial infarction at age 61",
  "Mother: Ischemic stroke at age 72",
  "Sibling: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 30 pack-year history",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Atenolol"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Lumbar laminectomy L4-L5 (2018)",
  "Diagnostic cardiac catheterization (2023)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Chronic lower back pain",
  "Major depressive disorder"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["Serotonin syndrome", "Hyperkalemia", "Acute ischemic stroke"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2024-10-13, ed, Emergency Medicine

Chief complaint: Weakness and palpitations that started earlier in the day

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Weakness and palpitations that started earlier in the day

HISTORY OF PRESENT ILLNESS:
That started earlier in the day.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Chronic lower back pain
- Major depressive disorder

PAST SURGICAL HISTORY:
- Lumbar laminectomy L4-L5 (2018)
- Diagnostic cardiac catheterization (2023)

MEDICATIONS:
He reports compliance with his medication regimen, which includes metformin, lisinopril, and atenolol.

Home Medications:
- Sertraline 50 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Ischemic stroke at age 72
- Sibling: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 30 pack-year history. Alcohol: Occasional social use, 1-2 drinks per week. Occupation: Construction supervisor.

VITAL SIGNS:
On examination, his blood pressure is 142/88 mmHg, heart rate is 55 bpm

PHYSICAL EXAMINATION:
, and he has mild bilateral pedal edema.

LABORATORY DATA:
Laboratory values are as follows:

- Serum potassium: 6.5 mEq/L
- Serum creatinine: 1.2 mg/dL
- Blood urea nitrogen (BUN): 29 mg/dL
- Serum sodium: 140 mEq/L
- Serum glucose: 158 mg/dL

OTHER STUDIES:
An ECG shows peaked T waves.

## Encounter 1: source 2020-07-15, practice 2025-04-02, ed, Emergency Medicine

Chief complaint: Sudden-onset agitation, confusion, tremors, and unsteady gait

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset agitation, confusion, tremors, and unsteady gait

HISTORY OF PRESENT ILLNESS:
The 57-year-old male with T2DM, HTN, chronic back pain, and MDD, previously evaluated for weakness and palpitations, returns following discharge one week ago after treatment for chronic back pain, having been prescribed several long-term medications. Earlier today he became restless, diaphoretic, and confused, with new tremors and unsteady gait.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperkalemia (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Chronic lower back pain
- Major depressive disorder

PAST SURGICAL HISTORY:
- Lumbar laminectomy L4-L5 (2018)
- Diagnostic cardiac catheterization (2023)

MEDICATIONS:
According to his family, his daily regimen includes an antidepressant, an antihypertensive, an antidiabetic agent, and some additional drugs that he cannot recall.

Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atenolol 50 mg PO daily
- Sertraline 50 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Ischemic stroke at age 72
- Sibling: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 30 pack-year history. Alcohol: Occasional social use, 1-2 drinks per week. Occupation: Construction supervisor.

VITAL SIGNS:
On arrival, he is febrile and hypertensive with a rapid heart rate.

PHYSICAL EXAMINATION:
Physical examination shows dilated pupils, flushed skin, inducible clonus at the ankles, and increased muscle tone.

LABORATORY DATA:
There is no evidence of recent infection, intoxication, or withdrawal, and laboratory results reveal no metabolic disturbances.

ASSESSMENT AND PLAN:
The team suspects a drug-related reaction causing excess serotonergic activity at both central and peripheral receptors, explaining the altered mental status, autonomic instability, and neuromuscular hyperactivity.

## Encounter 2: source 2021-07-15, practice 2026-04-09, ed, Emergency Medicine

Chief complaint: Sudden-onset right-sided weakness and slurred speech beginning 30 minutes ago

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset right-sided weakness and slurred speech beginning 30 minutes ago

HISTORY OF PRESENT ILLNESS:
58-year-old male with type 2 diabetes mellitus, hypertension, major depressive disorder, and prior emergency department presentations for hyperkalemia and serotonin syndrome presents to the emergency department with sudden-onset right-sided weakness and slurred speech that began approximately 30 minutes ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperkalemia (diagnosed 2020-01-15)
- Serotonin syndrome (diagnosed 2020-07-15)
- Type 2 diabetes mellitus
- Hypertension
- Chronic lower back pain
- Major depressive disorder

PAST SURGICAL HISTORY:
- Lumbar laminectomy L4-L5 (2018)
- Diagnostic cardiac catheterization (2023)

MEDICATIONS:
He does not take any medications.

Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atenolol 50 mg PO daily
- Sertraline 50 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Ischemic stroke at age 72
- Sibling: Type 2 diabetes mellitus

SOCIAL HISTORY:
A 30-pack-year smoking history and has not seen a physician in over three years

VITAL SIGNS:
Blood pressure of 192/100 mm Hg

PHYSICAL EXAMINATION:
Neurologic exam shows right facial droop, expressive aphasia, and 2/5 strength in the right upper and lower limbs

LABORATORY DATA:
LDL cholesterol level is 158 mg/dL

IMAGING:
A non-contrast CT of the head reveals no hemorrhage

PLAN:
He is admitted to the stroke unit for further evaluation. he is started on a statin
