# synthetic_hospital patient 2644

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 19 at first encounter, sex M, Non-Hispanic White, University student health insurance plan (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-25 (Fri) | outpatient | Student Health Services |
| 1 | 2020-07-15 | 2026-01-12 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Amoxicillin (non-IgE mediated maculopapular rash associated with infectious mononucleosis)"
 ],
 "insurance": "University student health insurance plan (PPO)",
 "occupation": "College freshman, resides in university dormitory",
 "alcohol_use": "Occasional social use (1-2 drinks per weekend)",
 "family_history": [
  "Father: Hypertension, diagnosed age 42",
  "Mother: Hypothyroidism, diagnosed age 40",
  "No family history of primary immunodeficiency or autoimmune disorders"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 19
}
```

Primary diagnoses: ["Infectious mononucleosis", "Enteroviral meningitis"]
Comorbidities: ["Neonatal herpes simplex virus exposure"]

## Encounter 0: source 2020-01-15, practice 2025-07-25, outpatient, Student Health Services

Chief complaint: 1-week history of fever, sore throat, and fatigue with subsequent maculopapular rash after amoxicillin exposure

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Student Health Services
Visit Type: outpatient

CHIEF COMPLAINT:
1-week history of fever, sore throat, and fatigue with subsequent maculopapular rash after amoxicillin exposure

HISTORY OF PRESENT ILLNESS:
He reports that the symptoms have progressively worsened despite taking over-the-counter pain relievers. His roommate recently recovered from a similar illness. Three days later, he develops a widespread, maculopapular rash after self-medicating with his friend's amoxicillin for the persistent sore throat.

PAST MEDICAL HISTORY:
His past medical history is unremarkable

MEDICATIONS:
He takes no regular medications

ALLERGIES:
Amoxicillin (non-IgE mediated maculopapular rash associated with infectious mononucleosis)

FAMILY HISTORY:
- Father: Hypertension, diagnosed age 42
- Mother: Hypothyroidism, diagnosed age 40
- No family history of primary immunodeficiency or autoimmune disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per weekend). Occupation: College freshman, resides in university dormitory.

VITAL SIGNS:
Temperature of 38.5°C (101.3°F)

PHYSICAL EXAMINATION:
Exudative pharyngitis, bilateral tender cervical lymphadenopathy, and mild hepatosplenomegaly

LABORATORY DATA:
Laboratory tests reveal the following:

- White blood cell count: 15,000/µL
- Atypical lymphocytes: 20%
- Alanine aminotransferase (ALT): 70 U/L (normal 7-56 U/L)
- Aspartate aminotransferase (AST): 68 U/L (normal 8-48 U/L)
- Monospot test: Positive

ASSESSMENT AND PLAN:
The patient is diagnosed with infectious mononucleosis

Advised supportive care

## Encounter 1: source 2020-07-15, practice 2026-01-12, ed, Emergency Medicine

Chief complaint: Headache, photophobia, neck stiffness, and low-grade fever for two days

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Headache, photophobia, neck stiffness, and low-grade fever for two days

HISTORY OF PRESENT ILLNESS:
The patient is a 19-year-old male with prior infectious mononucleosis who presents with headache, photophobia, neck stiffness, and fever that began two days ago. He reports increasing fatigue, nausea, poor appetite, and diffuse myalgias. Several of his dormitory roommates had similar flu-like symptoms over the past week, including sore throat and malaise. He has no recent travel, animal exposures, or drug use.

PAST MEDICAL HISTORY:
Active Problem List:
- Infectious mononucleosis (diagnosed 2020-01-15)

He is fully vaccinated.

MEDICATIONS:
He takes no regular medications

ALLERGIES:
Amoxicillin (non-IgE mediated maculopapular rash associated with infectious mononucleosis)

FAMILY HISTORY:
- Father: Hypertension, diagnosed age 42
- Mother: Hypothyroidism, diagnosed age 40
- No family history of primary immunodeficiency or autoimmune disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per weekend). Occupation: College freshman, resides in university dormitory.

VITAL SIGNS:
Temperature is 38.1°C (100.6°F), blood pressure is 112/74 mm Hg, pulse is 92/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
On exam, he is alert but appears uncomfortable. He exhibits photophobia and neck rigidity but no focal neurologic deficits.

LABORATORY DATA:
Lumbar puncture yields clear CSF with the following findings:
Opening pressure 170 mm H₂O
White blood cells 130/mm³ (92% lymphocytes)
Red blood cells 5/mm³
Protein 76 mg/dL
Glucose 64 mg/dL
Serum glucose 98 mg/dL
Gram stain No organisms seen

IMAGING:
Head CT is unremarkable.
