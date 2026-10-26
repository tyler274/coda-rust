# synthetic_hospital patient 2528

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-08-17 (Sun) | ed | Emergency Medicine |
| 1 | 2020-01-15 | 2025-08-24 (Sun) | icu | Intensive Care Unit |
| 2 | 2020-07-15 | 2026-02-11 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Part-time retail stock clerk",
 "alcohol_use": "None reported",
 "family_history": [
  "Maternal uncle: Schizophrenia",
  "Father: Major depressive disorder",
  "Mother: Generalized anxiety disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1/2 pack per day",
 "home_medications": [
  {
   "dose": "2 mg PO daily at bedtime (dose recently adjusted)",
   "name": "Risperidone"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 8)"
 ],
 "chronic_conditions": [
  "Schizophrenia",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Neuroleptic malignant syndrome"]
Comorbidities: ["Acute kidney injury", "Schizophrenia", "Rhabdomyolysis requiring fluid resuscitation"]

## Encounter 0: source 2020-01-15, practice 2025-08-17, ed, Emergency Medicine

Chief complaint: altered mental status, fever, and muscle rigidity

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Altered mental status, fever, and muscle rigidity

HISTORY OF PRESENT ILLNESS:
He was started on risperidone 2 weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Risperidone 2 mg PO daily at bedtime (dose recently adjusted)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal uncle: Schizophrenia
- Father: Major depressive disorder
- Mother: Generalized anxiety disorder

SOCIAL HISTORY:
Smoking: Current smoker, 1/2 pack per day. Alcohol: None reported. Occupation: Part-time retail stock clerk.

VITAL SIGNS:
His temperature is 39.5°C, blood pressure is 160/100 mmHg, and heart rate is 110 bpm.

PHYSICAL EXAMINATION:
Physical examination reveals diaphoresis, generalized severe muscle rigidity, and tremor.

LABORATORY DATA:
Laboratory studies show:

- WBC count: 15,000/mm³
- Creatine kinase (CK): 25,000 U/L
- Serum creatinine: 1.5 mg/dL
- AST: 120 U/L
- ALT: 100 U/L

## Encounter 1: source 2020-01-15, practice 2025-08-24, icu, Intensive Care Unit

Chief complaint: progressively worsening altered mental status, fever, and muscle rigidity despite medication cessation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Intensive Care Unit
Visit Type: icu

CHIEF COMPLAINT:
Progressively worsening altered mental status, fever, and muscle rigidity despite medication cessation

HISTORY OF PRESENT ILLNESS:
The patient is a 24-year-old male with schizophrenia and tobacco use disorder who was brought to the Emergency Department by his family earlier today for altered mental status, fever, and muscle rigidity. He presents to the Intensive Care Unit with progressively worsening altered mental status, fever, and muscle rigidity. His symptoms started two days ago and have progressively worsened. The patient's mental status continues to deteriorate despite cessation of haloperidol and initiation of supportive care.

PAST MEDICAL HISTORY:
Active Problem List:
- Neuroleptic malignant syndrome (diagnosed 2020-01-15)
- Schizophrenia
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

MEDICATIONS:
He takes haloperidol, which was started one week ago.

Home Medications:
- Risperidone 2 mg PO daily at bedtime (dose recently adjusted)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal uncle: Schizophrenia
- Father: Major depressive disorder
- Mother: Generalized anxiety disorder

SOCIAL HISTORY:
Smoking: Current smoker, 1/2 pack per day. Alcohol: None reported. Occupation: Part-time retail stock clerk.

VITAL SIGNS:
Temperature of 39.5°C (103.1°F), heart rate of 110 bpm, blood pressure of 160/90 mmHg

PHYSICAL EXAMINATION:
On examination, the patient is diaphoretic, has a temperature of 39.5°C (103.1°F), heart rate of 110 bpm, blood pressure of 160/90 mmHg, and is noted to have generalized severe muscle rigidity.

LABORATORY DATA:
Laboratory studies reveal:

- WBC count: 15,000/mm³
- Creatine kinase (CK): 25,000 U/L
- Serum electrolytes:
- Sodium: 142 mEq/L
- Potassium: 5.2 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 22 mEq/L
- Serum creatinine: 1.5 mg/dL
- AST: 120 U/L
- ALT: 100 U/L

Urinalysis shows dark urine with a positive myoglobin test.

## Encounter 2: source 2020-07-15, practice 2026-02-11, ed, Emergency Medicine

Chief complaint: high fever, confusion, and muscle stiffness

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
High fever, confusion, and muscle stiffness

HISTORY OF PRESENT ILLNESS:
This patient with schizophrenia returns after prior admissions. He is brought to the emergency department by his family due to a 2-day history of high fever, confusion, and muscle stiffness.

PAST MEDICAL HISTORY:
Active Problem List:
- Neuroleptic malignant syndrome (diagnosed 2020-01-15)
- Schizophrenia
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

MEDICATIONS:
His medications include risperidone, which was increased in dose 1 week ago.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal uncle: Schizophrenia
- Father: Major depressive disorder
- Mother: Generalized anxiety disorder

SOCIAL HISTORY:
Smoking: Current smoker, 1/2 pack per day. Alcohol: None reported. Occupation: Part-time retail stock clerk.

VITAL SIGNS:
Temperature of 39.5°C (103.1°F), heart rate of 110 bpm, blood pressure of 160/90 mmHg

PHYSICAL EXAMINATION:
On examination, the patient is diaphoretic, and is markedly rigid in all extremities. He is obtunded and only responsive to painful stimuli.

LABORATORY DATA:
Laboratory studies show:

- WBC: 12,000/mm³
- Creatine kinase (CK): 25,000 U/L
- Serum creatinine: 1.8 mg/dL
- AST: 120 U/L
- ALT: 100 U/L
