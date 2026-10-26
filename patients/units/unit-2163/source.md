# synthetic_hospital patient 2163

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 54 at first encounter, sex M, Non-Hispanic White, Private PPO through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-07-15 | 2025-06-05 (Thu) | inpatient | Hematology/Oncology |
| 1 | 2020-11-15 | 2025-10-16 (Thu) | inpatient | Hematology/Oncology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO through employer",
 "occupation": "Financial analyst",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "Father: Acute myeloid leukemia (diagnosed age 68)",
  "Mother: Hypertension, type 2 diabetes mellitus",
  "One sibling: Hyperlipidemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 8 years ago, 12 pack-year history",
 "home_medications": [
  {
   "dose": "10 mg by mouth daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg by mouth at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "5 mg by mouth daily",
   "name": "Amlodipine"
  }
 ],
 "surgical_history": [
  "Totally implantable venous access port insertion",
  "Bone marrow biopsy",
  "Allogeneic stem cell transplantation"
 ],
 "chronic_conditions": [
  "Acute myeloid leukemia",
  "Hypertension",
  "Dyslipidemia",
  "History of allogeneic hematopoietic stem cell transplantation"
 ],
 "age_at_first_encounter": 54
}
```

Primary diagnoses: ["Acute graft-versus-host disease", "Invasive pulmonary aspergillosis"]
Comorbidities: ["Acute myeloid leukemia"]

## Encounter 0: source 2020-07-15, practice 2025-06-05, inpatient, Hematology/Oncology

Chief complaint: Fever, chills, nonproductive cough, and pleuritic chest pain developing during hospitalization for induction chemotherapy

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Hematology/Oncology
Visit Type: inpatient

CHIEF COMPLAINT:
Fever, chills, nonproductive cough, and pleuritic chest pain developing during hospitalization for induction chemotherapy

HISTORY OF PRESENT ILLNESS:
During the first two weeks of treatment, he develops profound neutropenia. On hospital day 15, he begins experiencing fevers, chills, and progressive fatigue. Over the next 48 hours, he develops a nonproductive cough, pleuritic chest pain, and mild shortness of breath. Broad-spectrum antibacterial therapy fails to improve symptoms after 4 days.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute myeloid leukemia
- Hypertension
- Dyslipidemia
- History of allogeneic hematopoietic stem cell transplantation

PAST SURGICAL HISTORY:
- Totally implantable venous access port insertion
- Bone marrow biopsy
- Allogeneic stem cell transplantation

MEDICATIONS:
He takes cefepime and vancomycin.

Home Medications:
- Lisinopril 10 mg by mouth daily
- Atorvastatin 20 mg by mouth at bedtime
- Amlodipine 5 mg by mouth daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Acute myeloid leukemia (diagnosed age 68)
- Mother: Hypertension, type 2 diabetes mellitus
- One sibling: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 12 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Financial analyst.

VITAL SIGNS:
Temperature is 38.9°C (102°F), pulse is 104/min, blood pressure is 116/72 mm Hg, and oxygen saturation is 95% on room air

PHYSICAL EXAMINATION:
Lungs are clear to auscultation, but there is mild tenderness over the right posterior thoracic area on deep inspiration

LABORATORY DATA:
Blood cultures remain negative, and serum galactomannan antigen testing is positive

IMAGING:
Chest CT reveals several nodular densities in the upper lobes with surrounding ground-glass opacities (“halo sign”)

OTHER STUDIES:
A bronchoalveolar lavage grows Aspergillus fumigatus

PLAN:
The infectious diseases service recommends initiation of caspofungin

## Encounter 1: source 2020-11-15, practice 2025-10-16, inpatient, Hematology/Oncology

Chief complaint: Diffuse maculopapular rash and profuse watery diarrhea

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology/Oncology
Visit Type: inpatient

CHIEF COMPLAINT:
Diffuse maculopapular rash and profuse watery diarrhea

HISTORY OF PRESENT ILLNESS:
The patient is a 54-year-old male with acute myeloid leukemia status post allogeneic hematopoietic stem cell transplantation who returns for evaluation of new symptoms. He was previously hospitalized in July 2020 for invasive pulmonary aspergillosis complicating induction chemotherapy. Two weeks later, he develops diffuse maculopapular rash and profuse watery diarrhea.

PAST MEDICAL HISTORY:
Active Problem List:
- Invasive pulmonary aspergillosis (diagnosed 2020-07-15)
- Acute myeloid leukemia
- Hypertension
- Dyslipidemia
- History of allogeneic hematopoietic stem cell transplantation

PAST SURGICAL HISTORY:
Undergoes an allogeneic bone marrow transplant for treatment of

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg by mouth daily
- Atorvastatin 20 mg by mouth at bedtime
- Amlodipine 5 mg by mouth daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Acute myeloid leukemia (diagnosed age 68)
- Mother: Hypertension, type 2 diabetes mellitus
- One sibling: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 12 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Financial analyst.

LABORATORY DATA:
Elevated liver enzymes

PATHOLOGY:
A biopsy of the skin shows apoptotic keratinocytes with lymphocytic infiltration
