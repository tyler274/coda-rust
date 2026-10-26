# synthetic_hospital patient 2125

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Medicare (disability due to malignancy).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-12-10 (Wed) | ed | Emergency Medicine |
| 1 | 2020-04-15 | 2026-02-25 (Wed) | icu | Critical Care Medicine |
| 2 | 2020-09-15 | 2026-07-25 (Sat) | icu | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfamethoxazole-trimethoprim (anaphylaxis)",
  "Iodinated contrast (hives)"
 ],
 "insurance": "Medicare (disability due to malignancy)",
 "occupation": "Construction site supervisor",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: lung cancer (deceased age 68)",
  "Mother: type 2 diabetes mellitus, hypertension",
  "Paternal uncle: acute leukemia (unspecified type, deceased age 55)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 2019",
 "home_medications": [
  {
   "dose": "800 mg PO twice daily (antiviral prophylaxis)",
   "name": "Acyclovir"
  },
  {
   "dose": "300 mg PO daily (fungal prophylaxis)",
   "name": "Posaconazole delayed-release"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "17 g PO daily as needed for constipation",
   "name": "Polyethylene glycol 3350"
  },
  {
   "dose": "5 mg PO every 6 hours as needed for bone pain",
   "name": "Oxycodone"
  }
 ],
 "surgical_history": [
  "Right chest wall port-a-cath insertion (2023)",
  "Bone marrow biopsy (multiple, most recent 3 months ago)",
  "Tonsillectomy (childhood)"
 ],
 "chronic_conditions": [
  "Acute myeloid leukemia (intermediate risk cytogenetics)",
  "Hypertension",
  "Chemotherapy-induced peripheral neuropathy"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Chemotherapy-induced neutropenia", "Central pontine myelinolysis"]
Comorbidities: ["Hyponatremia", "Severe sepsis"]

## Encounter 0: source 2020-01-15, practice 2025-12-10, ed, Emergency Medicine

Chief complaint: Fever and chills for 2 days

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever and chills for 2 days

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department with. He completed his last cycle of chemotherapy 3 weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute myeloid leukemia (intermediate risk cytogenetics)
- Hypertension
- Chemotherapy-induced peripheral neuropathy

PAST SURGICAL HISTORY:
- Right chest wall port-a-cath insertion (2023)
- Bone marrow biopsy (multiple, most recent 3 months ago)
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Acyclovir 800 mg PO twice daily (antiviral prophylaxis)
- Posaconazole delayed-release 300 mg PO daily (fungal prophylaxis)
- Amlodipine 5 mg PO daily
- Polyethylene glycol 3350 17 g PO daily as needed for constipation
- Oxycodone 5 mg PO every 6 hours as needed for bone pain

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis), Iodinated contrast (hives)

FAMILY HISTORY:
- Father: lung cancer (deceased age 68)
- Mother: type 2 diabetes mellitus, hypertension
- Paternal uncle: acute leukemia (unspecified type, deceased age 55)

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 2019. Alcohol: Social (1-2 drinks per week). Occupation: Construction site supervisor.

VITAL SIGNS:
His temperature is 38.9°C (102°F), blood pressure is 100/60 mmHg, pulse is 110/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
Physical examination reveals pallor and diffuse mucosal ulcerations. No other abnormalities are noted.

LABORATORY DATA:
Laboratory studies are as follows:

- White blood cell count: 1,000/uL (4,500-11,000/uL)
- Absolute neutrophil count: 100/uL (1,500-8,000/uL)
- Hemoglobin: 8.2 g/dL (13.8-17.2 g/dL)
- Platelets: 50,000/uL (150,000-450,000/uL)
- Blood cultures: Pending

## Encounter 1: source 2020-04-15, practice 2026-02-25, icu, Critical Care Medicine

Chief complaint: Severe sepsis with hemodynamic instability requiring intensive care

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Severe sepsis with hemodynamic instability requiring intensive care

HISTORY OF PRESENT ILLNESS:
The patient is a 55-year-old male with a history of acute myeloid leukemia with intermediate risk cytogenetics who presented to the emergency department on January 15, 2020 with fever and chemotherapy-induced neutropenia, and now returns for management of severe sepsis with hemodynamic instability requiring intensive care. He is closely monitored for electrolyte disturbances. Over the course of his first 4 days in the unit, his serum sodium measurements (in mEq/L) are recorded each morning as follows: 136, 142, 131, and 139. The medical team wishes to use the mean serum sodium over these days to guide further management and monitor for trends of hypo- or hypernatremia. The nurse notes that the patient received varying amounts of intravenous fluids each day, but the sodium values were not affected by acute dilution. No hemolysis or laboratory error is reported.

PAST MEDICAL HISTORY:
Active Problem List:
- Chemotherapy-induced neutropenia (diagnosed 2020-01-15)
- Acute myeloid leukemia (intermediate risk cytogenetics)
- Hypertension
- Chemotherapy-induced peripheral neuropathy

PAST SURGICAL HISTORY:
- Right chest wall port-a-cath insertion (2023)
- Bone marrow biopsy (multiple, most recent 3 months ago)
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Acyclovir 800 mg PO twice daily (antiviral prophylaxis)
- Posaconazole delayed-release 300 mg PO daily (fungal prophylaxis)
- Amlodipine 5 mg PO daily
- Polyethylene glycol 3350 17 g PO daily as needed for constipation
- Oxycodone 5 mg PO every 6 hours as needed for bone pain

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis), Iodinated contrast (hives)

FAMILY HISTORY:
- Father: lung cancer (deceased age 68)
- Mother: type 2 diabetes mellitus, hypertension
- Paternal uncle: acute leukemia (unspecified type, deceased age 55)

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 2019. Alcohol: Social (1-2 drinks per week). Occupation: Construction site supervisor.

PLAN:
The nephrology consultant requests the mean serum sodium to two decimal places to assess for chronic hyponatremia risk.

## Encounter 2: source 2020-09-15, practice 2026-07-25, icu, Neurology

Chief complaint: Altered mental status and severe hyponatremia

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Priya Patel, MD
Department: Neurology
Visit Type: icu

CHIEF COMPLAINT:
Altered mental status and severe hyponatremia

HISTORY OF PRESENT ILLNESS:
The patient is a 55-year-old man with acute myeloid leukemia who was previously hospitalized in January for chemotherapy-induced neutropenia and in April for severe sepsis requiring intensive care. He is admitted to the ICU for altered mental status and severe hyponatremia (serum Na⁺ = 112 mEq/L). He is treated with IV hypertonic saline. Over the next 48 hours, his sodium level rises to 137 mEq/L. On day 3, he becomes acutely unresponsive.

PAST MEDICAL HISTORY:
Active Problem List:
- Chemotherapy-induced neutropenia (diagnosed 2020-01-15)
- Acute myeloid leukemia (intermediate risk cytogenetics)
- Hypertension
- Chemotherapy-induced peripheral neuropathy

PAST SURGICAL HISTORY:
- Right chest wall port-a-cath insertion (2023)
- Bone marrow biopsy (multiple, most recent 3 months ago)
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Acyclovir 800 mg PO twice daily (antiviral prophylaxis)
- Posaconazole delayed-release 300 mg PO daily (fungal prophylaxis)
- Amlodipine 5 mg PO daily
- Polyethylene glycol 3350 17 g PO daily as needed for constipation
- Oxycodone 5 mg PO every 6 hours as needed for bone pain

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis), Iodinated contrast (hives)

FAMILY HISTORY:
- Father: lung cancer (deceased age 68)
- Mother: type 2 diabetes mellitus, hypertension
- Paternal uncle: acute leukemia (unspecified type, deceased age 55)

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 2019. Alcohol: Social (1-2 drinks per week). Occupation: Construction site supervisor.

PHYSICAL EXAMINATION:
Neurologic exam reveals complete quadriplegia, but he can open his eyes and blink to command. He cannot speak, move his face, or breathe spontaneously.

LABORATORY DATA:
Serum Na⁺ = 112 mEq/L; sodium level rises to 137 mEq/L

IMAGING:
MRI shows symmetric hyperintensity in the central pons, sparing the tegmentum.
