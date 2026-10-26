# synthetic_hospital patient 2677

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 26 at first encounter, sex M, White, non-Hispanic, Private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-30 (Thu) | outpatient | Internal Medicine |
| 1 | 2021-07-15 | 2026-07-29 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance",
 "occupation": "Graduate student",
 "alcohol_use": "None",
 "family_history": [
  "Bipolar disorder in mother",
  "Hypertension in father"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "600 mg PO BID",
   "name": "Lithium carbonate"
  },
  {
   "dose": "1000 mg PO daily",
   "name": "Divalproex sodium"
  }
 ],
 "surgical_history": [
  "Tonsillectomy, age 8"
 ],
 "chronic_conditions": [
  "Bipolar I disorder"
 ],
 "age_at_first_encounter": 26
}
```

Primary diagnoses: ["Toxic liver disease", "Nephrogenic Diabetes Insipidus"]
Comorbidities: ["Bipolar Disorder"]

## Encounter 0: source 2020-01-15, practice 2025-01-30, outpatient, Internal Medicine

Chief complaint: 3-month history of increasing thirst and urination

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of increasing thirst and urination

PAST MEDICAL HISTORY:
Active Problem List:
- Bipolar I disorder

He has a long history of bipolar disorder for which he takes medication.

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
Home Medications:
- Lithium carbonate 600 mg PO BID
- Divalproex sodium 1000 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His mother has Hashimoto thyroiditis.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
Vitals are normal.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory studies show a serum sodium of 148 mEq/L and urinary specific gravity of 1.001. Fasting glucose is 97 mg/dL.

## Encounter 1: source 2021-07-15, practice 2026-07-29, ed, Emergency Medicine

Chief complaint: Acute confusion, lethargy, and vomiting brought in by roommate

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute confusion, lethargy, and vomiting brought in by roommate

HISTORY OF PRESENT ILLNESS:
The patient is a 27-year-old male with Bipolar I disorder and history of nephrogenic diabetes insipidus who presents with acute confusion, lethargy, and vomiting. Two weeks ago, he was started on a new medication for mood stabilization after being hospitalized for a manic episode. His roommate reports that the patient has appeared more fatigued over the past few days and had one episode of slurred speech this morning.

PAST MEDICAL HISTORY:
Active Problem List:
- Nephrogenic Diabetes Insipidus (diagnosed 2020-01-15)
- Bipolar I disorder

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
Home Medications:
- Lithium carbonate 600 mg PO BID
- Divalproex sodium 1000 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Bipolar disorder in mother
- Hypertension in father

SOCIAL HISTORY:
He denies alcohol or illicit drug use.

VITAL SIGNS:
Vitals are: temperature 37.4°C (99.3°F), blood pressure 112/76 mmHg, pulse 92/min, and respiratory rate 14/min.

PHYSICAL EXAMINATION:
Physical examination reveals mild scleral icterus and asterixis.

LABORATORY DATA:
Laboratory tests show:
AST                              388 U/L
ALT                               421 U/L
Total bilirubin                3.4 mg/dL
Ammonia                       98 µmol/L (normal: 15–45)
INR                              2.1
