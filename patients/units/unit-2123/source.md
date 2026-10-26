# synthetic_hospital patient 2123

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic White, Medicare (disability) and Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-17 (Thu) | outpatient | Psychiatry |
| 1 | 2020-04-15 | 2025-10-13 (Mon) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-07-05 (Sun) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfa drugs (rash)"
 ],
 "insurance": "Medicare (disability) and Medicaid",
 "occupation": "Receiving Social Security Disability Insurance; former automotive assembly worker",
 "alcohol_use": "None (former heavy use, quit 10 years ago)",
 "family_history": [
  "Father: schizophrenia, died age 62",
  "Mother: Alzheimer's disease, age 82",
  "Brother: bipolar disorder, type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day for 36 years",
 "home_medications": [
  {
   "dose": "5 mg PO twice daily",
   "name": "Haloperidol"
  },
  {
   "dose": "1 mg PO twice daily",
   "name": "Benztropine"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  }
 ],
 "surgical_history": [
  "Appendectomy (age 14)",
  "Inguinal hernia repair (age 35)"
 ],
 "chronic_conditions": [
  "Schizophrenia (20-year history)",
  "Major depressive disorder",
  "Tardive dyskinesia",
  "Medication-induced extrapyramidal symptoms",
  "Hypertension"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Oculogyric crisis due to antipsychotic medication", "Neuroleptic malignant syndrome", "Neuroleptic-induced akathisia"]
Comorbidities: ["Schizophrenia"]

## Encounter 0: source 2020-01-15, practice 2025-07-17, outpatient, Psychiatry

Chief complaint: increasing difficulty in initiating movements, a shuffling gait, and tremors in his hands

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing difficulty in initiating movements, a shuffling gait, and tremors in his hands

HISTORY OF PRESENT ILLNESS:
2-month history of increasing difficulty in initiating movements, a shuffling gait, and tremors in his hands. His family notes that he appears to have a "mask-like" facial expression and has become more stoic.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (20-year history)
- Major depressive disorder
- Tardive dyskinesia
- Medication-induced extrapyramidal symptoms
- Hypertension

PAST SURGICAL HISTORY:
- Appendectomy (age 14)
- Inguinal hernia repair (age 35)

MEDICATIONS:
He was started on haloperidol 6 months ago and has been compliant with his medication regimen.

Home Medications:
- Benztropine 1 mg PO twice daily
- Sertraline 100 mg PO daily
- Lisinopril 10 mg PO daily

ALLERGIES:
Sulfa drugs (rash)

FAMILY HISTORY:
- Father: schizophrenia, died age 62
- Mother: Alzheimer's disease, age 82
- Brother: bipolar disorder, type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 36 years. Alcohol: None (former heavy use, quit 10 years ago). Occupation: Receiving Social Security Disability Insurance; former automotive assembly worker.

PHYSICAL EXAMINATION:
On examination, the patient has a resting tremor, bradykinesia, rigidity, and decreased arm swing while walking.

LABORATORY DATA:
His most recent laboratory tests are as follows:

- Complete blood count: within normal limits
- Comprehensive metabolic panel: within normal limits
- Thyroid function tests: within normal limits
- Serum prolactin: elevated

## Encounter 1: source 2020-04-15, practice 2025-10-13, ed, Emergency Medicine

Chief complaint: repetitive chewing motions and intermittent grimacing

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Repetitive chewing motions and intermittent grimacing

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old male with a 20-year history of schizophrenia and prior presentation three months ago for neuroleptic-induced akathisia, who returns to the clinic with his sister. She reports that over the past 6 months, he has developed repetitive chewing motions and intermittent grimacing. He has been stable on his current antipsychotic regimen for the past 10 years. His sister denies any recent changes in his medications or new stressors.

PAST MEDICAL HISTORY:
Active Problem List:
- Neuroleptic-induced akathisia (diagnosed 2020-01-15)
- Schizophrenia (20-year history)
- Major depressive disorder
- Tardive dyskinesia
- Medication-induced extrapyramidal symptoms
- Hypertension

PAST SURGICAL HISTORY:
- Appendectomy (age 14)
- Inguinal hernia repair (age 35)

MEDICATIONS:
His current medications include haloperidol, benztropine, and sertraline.

Home Medications:
- Lisinopril 10 mg PO daily

ALLERGIES:
Sulfa drugs (rash)

FAMILY HISTORY:
- Father: schizophrenia, died age 62
- Mother: Alzheimer's disease, age 82
- Brother: bipolar disorder, type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 36 years. Alcohol: None (former heavy use, quit 10 years ago). Occupation: Receiving Social Security Disability Insurance; former automotive assembly worker.

VITAL SIGNS:
His vital signs are within normal limits

PHYSICAL EXAMINATION:
On examination, the patient exhibits involuntary movements of his tongue, which protrude in a writhing motion, and he has difficulty sitting still due to trunk and limb movements. The rest of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory studies, including complete blood count, comprehensive metabolic panel, and thyroid function tests, are within normal limits.

## Encounter 2: source 2021-01-15, practice 2026-07-05, icu, Critical Care Medicine

Chief complaint: altered mental status, sweating, and muscle stiffness

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Patel, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Altered mental status, sweating, and muscle stiffness

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old male with a 20-year history of schizophrenia and recent neuroleptic-induced akathisia and oculogyric crisis who presents with symptoms that began earlier today. According to his son, the patient had been more withdrawn over the past 48 hours and last night began pacing the house, mumbling incoherently, and reporting that "people are watching him through the walls." He became increasingly agitated, developed a high fever, and had difficulty moving his arms and legs due to stiffness.

PAST MEDICAL HISTORY:
Active Problem List:
- Neuroleptic-induced akathisia (diagnosed 2020-01-15)
- Oculogyric crisis due to antipsychotic medication (diagnosed 2020-04-15)
- Schizophrenia (20-year history)
- Major depressive disorder
- Tardive dyskinesia
- Medication-induced extrapyramidal symptoms
- Hypertension

He was diagnosed with schizophrenia two weeks ago.

PAST SURGICAL HISTORY:
- Appendectomy (age 14)
- Inguinal hernia repair (age 35)

MEDICATIONS:
He was recently started on risperidone.

Home Medications:
- Haloperidol 5 mg PO twice daily
- Benztropine 1 mg PO twice daily
- Sertraline 100 mg PO daily
- Lisinopril 10 mg PO daily

ALLERGIES:
Sulfa drugs (rash)

FAMILY HISTORY:
- Father: schizophrenia, died age 62
- Mother: Alzheimer's disease, age 82
- Brother: bipolar disorder, type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 36 years. Alcohol: None (former heavy use, quit 10 years ago). Occupation: Receiving Social Security Disability Insurance; former automotive assembly worker.

VITAL SIGNS:
Temperature is 39.4°C (103°F), blood pressure is 148/92 mm Hg, pulse is 122/min, and respirations are 22/min.

PHYSICAL EXAMINATION:
On physical examination, the patient is diaphoretic, disoriented, and responding to internal stimuli. He appears rigid in all extremities, and passive movement is met with resistance. Reflexes are 2+ throughout. There is no nuchal rigidity, and cranial nerve examination is unremarkable. He is not oriented to person, place, or time. The remainder of the exam is normal.

PLAN:
In addition to intravenous fluids and supportive care,
