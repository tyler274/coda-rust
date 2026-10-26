# synthetic_hospital patient 2569

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex M, Non-Hispanic White, Employer-sponsored commercial PPO plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-10-12 (Sun) | inpatient | Hematology & Oncology |
| 1 | 2020-07-15 | 2026-04-11 (Sat) | ed | Emergency Medicine |
| 2 | 2020-09-15 | 2026-06-23 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfamethoxazole-trimethoprim (anaphylaxis - per patient report; currently desensitized per oncology protocol)",
  "Iodinated contrast (urticaria)"
 ],
 "insurance": "Employer-sponsored commercial PPO plan",
 "occupation": "Environmental Field Technician",
 "alcohol_use": "Social (2-3 drinks per week)",
 "family_history": [
  "Paternal grandfather with acute myeloid leukemia (diagnosed age 65)",
  "Mother with hypothyroidism",
  "Father with hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "100 mg PO nightly (maintenance chemotherapy)",
   "name": "Mercaptopurine (6-MP)"
  },
  {
   "dose": "15 mg PO weekly (maintenance chemotherapy)",
   "name": "Methotrexate"
  },
  {
   "dose": "500 mg PO daily (viral prophylaxis)",
   "name": "Valacyclovir"
  },
  {
   "dose": "500 mg PO BID (bacterial prophylaxis during neutropenic periods per oncology protocol)",
   "name": "Ciprofloxacin"
  },
  {
   "dose": "300 mcg subcutaneous injection daily during nadir periods per prior cycle instructions",
   "name": "Filgrastim (G-CSF)"
  }
 ],
 "surgical_history": [
  "Port-a-cath insertion (right subclavian, 8 months ago)",
  "Tonsillectomy and adenoidectomy (age 7)"
 ],
 "chronic_conditions": [
  "Acute lymphoblastic leukemia (diagnosed 8 months ago, currently in maintenance phase)",
  "Seasonal allergic rhinitis"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Ecthyma gangrenosum due to Pseudomonas aeruginosa bacteremia", "Chemotherapy-induced neutropenia", "Rocky Mountain spotted fever"]
Comorbidities: ["Thrombocytopenia"]

## Encounter 0: source 2020-01-15, practice 2025-10-12, inpatient, Hematology & Oncology

Chief complaint: Shaking chills and fever of 40.1°C (104.2°F) on day 10 of induction chemotherapy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Hematology & Oncology
Visit Type: inpatient

CHIEF COMPLAINT:
Shaking chills and fever of 40.1°C (104.2°F) on day 10 of induction chemotherapy

HISTORY OF PRESENT ILLNESS:
He is admitted for induction chemotherapy. On day 10 of his hospital stay, he develops

PAST MEDICAL HISTORY:
Active Problem List:
- Acute lymphoblastic leukemia (diagnosed 8 months ago, currently in maintenance phase)
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Port-a-cath insertion (right subclavian, 8 months ago)
- Tonsillectomy and adenoidectomy (age 7)

MEDICATIONS:
Home Medications:
- Mercaptopurine (6-MP) 100 mg PO nightly (maintenance chemotherapy)
- Methotrexate 15 mg PO weekly (maintenance chemotherapy)
- Valacyclovir 500 mg PO daily (viral prophylaxis)
- Ciprofloxacin 500 mg PO BID (bacterial prophylaxis during neutropenic periods per oncology protocol)
- Filgrastim (G-CSF) 300 mcg subcutaneous injection daily during nadir periods per prior cycle instructions

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis - per patient report; currently desensitized per oncology protocol), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Paternal grandfather with acute myeloid leukemia (diagnosed age 65)
- Mother with hypothyroidism
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per week). Occupation: Environmental Field Technician.

PHYSICAL EXAMINATION:
He appears acutely ill but has no localizing symptoms on physical exam.

LABORATORY DATA:
Laboratory tests reveal hemoglobin 8.4 g/dL, leukocyte count 380/mm³, mean corpuscular volume 89 fL, and platelet count 58,000/mm³.

OTHER STUDIES:
Blood cultures are obtained.

ASSESSMENT AND PLAN:
Despite supportive care, his absolute neutrophil count remains critically low.

Broad-spectrum intravenous antibiotics are started after
The oncology team debates whether to add a recombinant cytokine to his regimen.

## Encounter 1: source 2020-07-15, practice 2026-04-11, ed, Emergency Medicine

Chief complaint: Fever and painful rash three weeks after completing a chemotherapy cycle

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever and painful rash three weeks after completing a chemotherapy cycle

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old with acute lymphoblastic leukemia in maintenance phase and a history of hospitalization for chemotherapy-induced neutropenia who presents to the emergency department with a 2-day history of fever and a painful rash. He completed his last cycle of chemotherapy 3 weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Chemotherapy-induced neutropenia (diagnosed 2020-01-15)
- Acute lymphoblastic leukemia (diagnosed 8 months ago, currently in maintenance phase)
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Port-a-cath insertion (right subclavian, 8 months ago)
- Tonsillectomy and adenoidectomy (age 7)

MEDICATIONS:
Home Medications:
- Mercaptopurine (6-MP) 100 mg PO nightly (maintenance chemotherapy)
- Methotrexate 15 mg PO weekly (maintenance chemotherapy)
- Valacyclovir 500 mg PO daily (viral prophylaxis)
- Ciprofloxacin 500 mg PO BID (bacterial prophylaxis during neutropenic periods per oncology protocol)
- Filgrastim (G-CSF) 300 mcg subcutaneous injection daily during nadir periods per prior cycle instructions

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis - per patient report; currently desensitized per oncology protocol), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Paternal grandfather with acute myeloid leukemia (diagnosed age 65)
- Mother with hypothyroidism
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per week). Occupation: Environmental Field Technician.

VITAL SIGNS:
His temperature is 38.9°C (102°F), blood pressure is 90/60 mmHg, pulse is 120/min, and respirations are 22/min.

PHYSICAL EXAMINATION:
Physical examination reveals multiple, round, erythematous macules on his lower extremities that have progressed to form central bullae with surrounding erythema. One of the lesions has a black, necrotic center with a surrounding erythematous halo.

LABORATORY DATA:
Laboratory studies are as follows:

- White blood cell count: 1,200/mm³ (Normal: 4,500-11,000/mm³)
- Hemoglobin: 9.2 g/dL (Normal: 13.8-17.2 g/dL)
- Platelets: 50,000/mm³ (Normal: 150,000-450,000/mm³)
- Blood cultures: Pending

## Encounter 2: source 2020-09-15, practice 2026-06-23, ed, Emergency Medicine

Chief complaint: Fever, headache, and myalgia for six days after hiking trip with tick exposure

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Priya Sharma, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, headache, and myalgia for six days after hiking trip with tick exposure

HISTORY OF PRESENT ILLNESS:
This 28-year-old male with acute lymphoblastic leukemia in maintenance phase, previously hospitalized for febrile neutropenia during induction chemotherapy and Pseudomonas bacteremia with ecthyma gangrenosum, presents to the emergency department with a 6-day history of fever, headache, and myalgia after returning from a hiking trip in rural North Carolina. He recalls finding multiple ticks on his legs but did not develop a rash. He also notes photophobia and mild nausea but denies vomiting, cough, or sore throat.

PAST MEDICAL HISTORY:
Active Problem List:
- Chemotherapy-induced neutropenia (diagnosed 2020-01-15)
- Ecthyma gangrenosum due to Pseudomonas aeruginosa bacteremia (diagnosed 2020-07-15)
- Acute lymphoblastic leukemia (diagnosed 8 months ago, currently in maintenance phase)
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Port-a-cath insertion (right subclavian, 8 months ago)
- Tonsillectomy and adenoidectomy (age 7)

MEDICATIONS:
Home Medications:
- Mercaptopurine (6-MP) 100 mg PO nightly (maintenance chemotherapy)
- Methotrexate 15 mg PO weekly (maintenance chemotherapy)
- Valacyclovir 500 mg PO daily (viral prophylaxis)
- Ciprofloxacin 500 mg PO BID (bacterial prophylaxis during neutropenic periods per oncology protocol)
- Filgrastim (G-CSF) 300 mcg subcutaneous injection daily during nadir periods per prior cycle instructions

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis - per patient report; currently desensitized per oncology protocol), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Paternal grandfather with acute myeloid leukemia (diagnosed age 65)
- Mother with hypothyroidism
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per week). Occupation: Environmental Field Technician.

PHYSICAL EXAMINATION:
On exam, he is febrile and has mild scleral injection, tender cervical lymph nodes, and some petechiae on the ankles and wrists.

LABORATORY DATA:
His labs show a mild thrombocytopenia and elevated liver enzymes.

PLAN:
He is empirically started on doxycycline.
