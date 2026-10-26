# synthetic_hospital patient 2655

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex M, Hispanic/Latino, Commercial health insurance through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-12-02 (Tue) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2026-03-02 (Mon) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial health insurance through employer",
 "occupation": "Childcare provider at local daycare center",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Paternal grandfather with history of pulmonary tuberculosis",
  "Mother with hypertension"
 ],
 "race_ethnicity": "Hispanic/Latino",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Latent tuberculosis infection", "Vitamin B6 deficiency"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-12-02, outpatient, Internal Medicine

Chief complaint: Routine health examination required for his new job at a local daycare

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health examination required for his new job at a local daycare

HISTORY OF PRESENT ILLNESS:
He has no complaints and reports feeling well.

PAST MEDICAL HISTORY:
His past medical history is unremarkable

MEDICATIONS:
He takes no medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His family history is negative for tuberculosis.

SOCIAL HISTORY:
He was born in the United States and has never traveled outside the country. He does not smoke, use illicit drugs, or consume alcohol.

VITAL SIGNS:
His vital signs are within normal limits

PHYSICAL EXAMINATION:
The exam is unremarkable

OTHER STUDIES:
As part of the screening process, a tuberculin skin test (TST) is performed. Forty-eight hours later, the patient returns for the reading, which shows an induration of 16 mm.

## Encounter 1: source 2020-04-15, practice 2026-03-02, outpatient, Internal Medicine

Chief complaint: Follow-up appointment with new onset paresthesias in right arm

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Elena Vasquez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up appointment with new onset paresthesias in right arm

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up three months after initiating treatment for latent tuberculosis infection identified during a routine examination for daycare employment. Three months into the treatment, he develops paresthesias in his right arm.

PAST MEDICAL HISTORY:
Active Problem List:
- Latent tuberculosis infection (diagnosed 2020-01-15)

He had a BCG vaccine when he was in infancy.

MEDICATIONS:
He's started on TB prophylaxis.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Paternal grandfather with history of pulmonary tuberculosis
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Childcare provider at local daycare center.

IMAGING:
Chest x-ray is negative.

OTHER STUDIES:
He had a PPD test performed a few days ago that shows 12mm induration.
