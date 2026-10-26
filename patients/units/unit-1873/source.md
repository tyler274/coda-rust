# synthetic_hospital patient 1873

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 23 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2026-01-08 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2026-06-25 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield",
 "occupation": "Graduate student",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Mother with heavy menstrual bleeding and easy bruising",
  "Paternal grandfather with prolonged bleeding after surgery"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "325 mg PO daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "1300 mg PO three times daily during menstruation",
   "name": "Tranexamic acid"
  }
 ],
 "surgical_history": [
  "Laparoscopic ovarian cyst removal"
 ],
 "chronic_conditions": [
  "Von Willebrand Disease, Type 2A",
  "Iron deficiency anemia",
  "Menorrhagia"
 ],
 "age_at_first_encounter": 23
}
```

Primary diagnoses: ["Von Willebrand disease, Type 2A", "Iron Deficiency Anemia"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2026-01-08, outpatient, Internal Medicine

Chief complaint: Fatigue and decreased exercise tolerance over the past 3 months.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and decreased exercise tolerance over the past 3 months.

HISTORY OF PRESENT ILLNESS:
She presents to her primary care provider with complaints of fatigue and decreased exercise tolerance over the past 3 months. She reports difficulty completing her usual running routine and feels winded after climbing a single flight of stairs. She denies chest pain, palpitations, or recent illness. She reports regular monthly menstrual periods that last 4–5 days, and denies menorrhagia, intermenstrual bleeding, or passage of clots.

PAST MEDICAL HISTORY:
Active Problem List:
- Von Willebrand Disease, Type 2A
- Iron deficiency anemia
- Menorrhagia

PAST SURGICAL HISTORY:
- Laparoscopic ovarian cyst removal

MEDICATIONS:
She is.

Home Medications:
- Ferrous sulfate 325 mg PO daily
- Tranexamic acid 1300 mg PO three times daily during menstruation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with heavy menstrual bleeding and easy bruising
- Paternal grandfather with prolonged bleeding after surgery

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Graduate student.

PHYSICAL EXAMINATION:
A physical examination reveals pale conjunctivae, but no jaundice, lymphadenopathy, or hepatosplenomegaly. Cardiac and pulmonary exams are normal.

LABORATORY DATA:
A complete blood count shows a hemoglobin level of 9.3 g/dL.

## Encounter 1: source 2020-07-15, practice 2026-06-25, ed, Emergency Medicine

Chief complaint: Excessive bleeding following recent laparoscopic ovarian cyst removal.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Okonkwo, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Excessive bleeding following recent laparoscopic ovarian cyst removal.

HISTORY OF PRESENT ILLNESS:
VWD Type 2A patient with prior anemia presents with excessive bleeding following recent surgery. She reports a lifelong history of menorrhagia requiring iron supplements, occasional prolonged nosebleeds, and easy bruising, but she was told as a child that her labs were "normal."

PAST MEDICAL HISTORY:
Active Problem List:
- Iron Deficiency Anemia (diagnosed 2020-01-15)
- Von Willebrand Disease, Type 2A
- Menorrhagia

PAST SURGICAL HISTORY:
- Laparoscopic ovarian cyst removal

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg PO daily
- Tranexamic acid 1300 mg PO three times daily during menstruation

ALLERGIES:
NKDA

FAMILY HISTORY:
Her father has a similar bleeding history but was never diagnosed with a specific disorder.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Graduate student.

PHYSICAL EXAMINATION:
On exam, she appears pale with scattered ecchymoses on the extremities.

LABORATORY DATA:
Labs show hemoglobin 9.6 g/dL, platelets 210,000/μL, PT 12 seconds, and PTT 39 seconds. Ristocetin cofactor assay shows abnormal agglutination; multimer analysis shows decreased high-molecular-weight multimers.
