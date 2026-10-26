# synthetic_hospital patient 2688

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 23 at first encounter, sex M, Non-Hispanic White, Private insurance through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-08 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2026-03-12 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private insurance through employer",
 "occupation": "Construction worker",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "Father: Hypertension",
  "Mother: Migraine disorder",
  "No known inherited metabolic disorders"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg daily",
   "name": "Zinc gluconate supplement"
  },
  {
   "dose": "One tablet daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 23
}
```

Primary diagnoses: ["Hyperzincemia"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-09-08, outpatient, Internal Medicine

Chief complaint: Follow-up appointment to review laboratory report

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up appointment to review laboratory report

PAST MEDICAL HISTORY:
He has no past medical history.

MEDICATIONS:
Home Medications:
- Zinc gluconate supplement 50 mg daily
- Multivitamin One tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
On further questioning, his father had been hospitalized about 5 years ago for pancreatitis.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks per week. Occupation: Construction worker.

PHYSICAL EXAMINATION:
Physical exam is unremarkable, with no xanthomas or hepatosplenomegaly.

LABORATORY DATA:
His fasting lipid panel shows normal total cholesterol, LDL, and HDL, and elevated triglycerides. There is no evidence of chylomicrons on refrigerated plasma.

## Encounter 1: source 2020-07-15, practice 2026-03-12, ed, Emergency Medicine

Chief complaint: Recurrent episodes of severe sharp upper abdominal pain radiating to the back with nausea and vomiting

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent episodes of severe sharp upper abdominal pain radiating to the back with nausea and vomiting

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department six months following his prior diagnosis of hyperzincemia. Over the past three months, he describes the pain as sharp and localized to the upper abdomen, often radiating to his back and accompanied by nausea and vomiting. He reports similar episodes in the past that required hospitalization.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)

MEDICATIONS:
Home Medications:
- Zinc gluconate supplement 50 mg daily
- Multivitamin One tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Migraine disorder
- No known inherited metabolic disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks per week. Occupation: Construction worker.

PHYSICAL EXAMINATION:
Physical examination reveals tenderness in the epigastric region without rebound or guarding.

LABORATORY DATA:
Laboratory tests reveal elevated serum triglyceride levels at 2000 mg/dL, amylase activity of 2000 U/L, and lipase activity of 1800 U/L.

IMAGING:
Imaging confirms no gallstones or biliary tract obstruction.
