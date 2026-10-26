# synthetic_hospital patient 2717

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex F, Black/African American, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-23 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2025-10-25 (Sat) | ed | Emergency Medicine |
| 2 | 2020-10-15 | 2026-01-19 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Occasional (1-2 glasses of wine per month)",
 "family_history": [
  "Father: Fatal ruptured cerebral aneurysm at age 45",
  "Mother: Essential hypertension, type 2 diabetes",
  "Maternal aunt: Polycystic kidney disease"
 ],
 "race_ethnicity": "Black/African American",
 "smoking_status": "Former smoker (quit 2018, 10 pack-year history)",
 "home_medications": [
  {
   "dose": "per Phase IV clinical trial protocol",
   "name": "Drug X (investigational antihypertensive agent)"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "20 mg daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "Endovascular coiling of ruptured anterior communicating artery aneurysm (2023)",
  "Laparoscopic appendectomy (2010)"
 ],
 "chronic_conditions": [
  "Essential hypertension",
  "History of ruptured cerebral aneurysm"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Ruptured cerebral aneurysm", "Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2025-04-23, outpatient, Internal Medicine

Chief complaint: Blood pressure remained uncontrolled on standard triple therapy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Blood pressure remained uncontrolled on standard triple therapy

HISTORY OF PRESENT ILLNESS:
She is prescribed a newly approved antihypertensive medication, Drug X, after her blood pressure remained uncontrolled on standard therapy

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension
- History of ruptured cerebral aneurysm

PAST SURGICAL HISTORY:
- Endovascular coiling of ruptured anterior communicating artery aneurysm (2023)
- Laparoscopic appendectomy (2010)

MEDICATIONS:
Home Medications:
- Drug X (investigational antihypertensive agent) per Phase IV clinical trial protocol
- Aspirin 81 mg daily
- Omeprazole 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Fatal ruptured cerebral aneurysm at age 45
- Mother: Essential hypertension, type 2 diabetes
- Maternal aunt: Polycystic kidney disease

SOCIAL HISTORY:
Smoking: Former smoker (quit 2018, 10 pack-year history). Alcohol: Occasional (1-2 glasses of wine per month). Occupation: Elementary school teacher.

ASSESSMENT AND PLAN:
Six months after FDA approval, reports begin to emerge about rare but serious hepatic side effects associated with Drug X.

The FDA mandates ongoing data collection to monitor long-term safety as more patients are exposed to the medication. A pharmaceutical company initiates a large-scale, years-long registry to track adverse events, new-onset comorbidities, and mortality related to Drug X in the general population. The registry includes reports from healthcare providers, patients, and electronic health records. Another research group conducts a nested case-control study within the registry to investigate potential drug-induced autoimmune phenomena. The primary aim of these efforts is to determine if continued use of Drug X is justified in light of its emerging risk profile.

## Encounter 1: source 2020-07-15, practice 2025-10-25, ed, Emergency Medicine

Chief complaint: Sudden-onset, severe headache described as 'the worst headache of my life'

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset, severe headache described as 'the worst headache of my life'

HISTORY OF PRESENT ILLNESS:
This 42-year-old female with essential hypertension in a Phase IV trial presents to the Emergency Department. She reports the headache started abruptly about 2 hours ago while she was gardening. She also complains of nausea and has vomited twice. She denies recent head trauma or falls.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension
- History of ruptured cerebral aneurysm

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Endovascular coiling of ruptured anterior communicating artery aneurysm (2023)
- Laparoscopic appendectomy (2010)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Drug X (investigational antihypertensive agent) per Phase IV clinical trial protocol
- Aspirin 81 mg daily
- Omeprazole 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Fatal ruptured cerebral aneurysm at age 45
- Mother: Essential hypertension, type 2 diabetes
- Maternal aunt: Polycystic kidney disease

SOCIAL HISTORY:
Smoking: Former smoker (quit 2018, 10 pack-year history). Alcohol: Occasional (1-2 glasses of wine per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her temperature is 37.8°C (100°F), blood pressure is 165/95 mmHg, pulse is 88 beats per minute, and respiratory rate is 18 breaths per minute

PHYSICAL EXAMINATION:
On examination, she is alert but anxious. Neurological examination reveals nuchal rigidity. Fundoscopic examination is normal.

IMAGING:
A non-contrast CT scan of the brain is performed immediately and shows a hyperdense area in the basal cisterns.

## Encounter 2: source 2020-10-15, practice 2026-01-19, ed, Emergency Medicine

Chief complaint: Weakness, dizziness, and palpitations following two days of severe vomiting

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Elena Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Weakness, dizziness, and palpitations following two days of severe vomiting

HISTORY OF PRESENT ILLNESS:
This 42-year-old female with essential hypertension, history of ruptured cerebral aneurysm, and Phase IV clinical trial enrollment presents to the emergency department with a two-day history of weakness, dizziness, and palpitations. She also reports a recent history of severe vomiting due to gastroenteritis.

PAST MEDICAL HISTORY:
Active Problem List:
- Ruptured cerebral aneurysm (diagnosed 2020-07-15)
- Essential hypertension

She has a past medical history of hypertension.

PAST SURGICAL HISTORY:
- Endovascular coiling of ruptured anterior communicating artery aneurysm (2023)
- Laparoscopic appendectomy (2010)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Drug X (investigational antihypertensive agent) per Phase IV clinical trial protocol
- Aspirin 81 mg daily
- Omeprazole 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Fatal ruptured cerebral aneurysm at age 45
- Mother: Essential hypertension, type 2 diabetes
- Maternal aunt: Polycystic kidney disease

SOCIAL HISTORY:
Smoking: Former smoker (quit 2018, 10 pack-year history). Alcohol: Occasional (1-2 glasses of wine per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Blood pressure is 100/60 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
She appears dehydrated

LABORATORY DATA:
Serum Na+: 140 mEq/L
- Serum K+: 3.0 mEq/L
- Serum Cl-: 90 mEq/L
- Serum HCO3-: 35 mEq/L
- Urine Cl-: 10 mEq/L
- Blood pH: 7.50
