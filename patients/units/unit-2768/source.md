# synthetic_hospital patient 2768

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex F, Non-Hispanic White, Medicare (disability).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-30 (Mon) | outpatient | Hematology & Oncology |
| 1 | 2020-07-15 | 2025-06-23 (Mon) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2026-06-14 (Sun) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfamethoxazole-trimethoprim (anaphylaxis)",
  "Ibuprofen (gastropathy)"
 ],
 "insurance": "Medicare (disability)",
 "occupation": "Part-time library assistant (reduced hours due to disability)",
 "alcohol_use": "None (abstains due to medication regimen)",
 "family_history": [
  "Mother: rheumatoid arthritis",
  "Maternal aunt: systemic lupus erythematosus",
  "Father: type 2 diabetes mellitus, hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg orally once weekly",
   "name": "Methotrexate"
  },
  {
   "dose": "5 mg orally every morning",
   "name": "Prednisone"
  },
  {
   "dose": "1 mg orally daily (except methotrexate day)",
   "name": "Folic acid"
  },
  {
   "dose": "35 mg orally weekly",
   "name": "Risedronate"
  },
  {
   "dose": "600 mg/400 IU orally twice daily",
   "name": "Calcium carbonate with vitamin D3"
  },
  {
   "dose": "300 mcg subcutaneously as needed for ANC <1000",
   "name": "Filgrastim"
  }
 ],
 "surgical_history": [
  "Carpal tunnel release, bilateral (8 years ago)",
  "Bone marrow biopsy (3 years ago, hypocellular marrow with maturation arrest)",
  "Diagnostic splenic aspiration (3 years ago, benign)"
 ],
 "chronic_conditions": [
  "Seropositive rheumatoid arthritis (diagnosed age 46, 10-year duration)",
  "Felty syndrome with chronic neutropenia and splenomegaly",
  "Osteoporosis secondary to chronic corticosteroid use",
  "Chronic fatigue syndrome (secondary to autoimmune process)"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Felty syndrome"]
Comorbidities: ["Rheumatoid arthritis"]

## Encounter 0: source 2020-01-15, practice 2024-12-30, outpatient, Hematology & Oncology

Chief complaint: Persistent fatigue, morning stiffness lasting over an hour, and swelling in the small joints of her hands and wrists

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, morning stiffness lasting over an hour, and swelling in the small joints of her hands and wrists

PAST MEDICAL HISTORY:
Active Problem List:
- Seropositive rheumatoid arthritis (diagnosed age 46, 10-year duration)
- Felty syndrome with chronic neutropenia and splenomegaly
- Osteoporosis secondary to chronic corticosteroid use
- Chronic fatigue syndrome (secondary to autoimmune process)

PAST SURGICAL HISTORY:
- Carpal tunnel release, bilateral (8 years ago)
- Bone marrow biopsy (3 years ago, hypocellular marrow with maturation arrest)
- Diagnostic splenic aspiration (3 years ago, benign)

MEDICATIONS:
Home Medications:
- Methotrexate 20 mg orally once weekly
- Prednisone 5 mg orally every morning
- Folic acid 1 mg orally daily (except methotrexate day)
- Risedronate 35 mg orally weekly
- Calcium carbonate with vitamin D3 600 mg/400 IU orally twice daily
- Filgrastim 300 mcg subcutaneously as needed for ANC <1000

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis), Ibuprofen (gastropathy)

FAMILY HISTORY:
- Mother: rheumatoid arthritis
- Maternal aunt: systemic lupus erythematosus
- Father: type 2 diabetes mellitus, hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstains due to medication regimen). Occupation: Part-time library assistant (reduced hours due to disability).

LABORATORY DATA:
Laboratory studies show elevated ESR and C-reactive protein. Rheumatoid factor and anti-cyclic citrullinated peptide antibodies are positive.
Laboratory studies show a hemoglobin of 10 g/dL, with low serum iron and normal ferritin.

PATHOLOGY:
Synovial biopsy shows pannus formation with infiltration of plasma cells and lymphocytes.

ASSESSMENT AND PLAN:
The cytokine most responsible for this patient's hematologic abnormality also plays a role in

## Encounter 1: source 2020-07-15, practice 2025-06-23, ed, Emergency Medicine

Chief complaint: Persistent fatigue and new-onset bruising

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Persistent fatigue and new-onset bruising

HISTORY OF PRESENT ILLNESS:
This 56-year-old female with seropositive rheumatoid arthritis and Felty syndrome presents to the emergency department six months following her hematology evaluation. She reports generalized joint pain and stiffness in the morning that lasts for an hour, especially in her hands and knees. Her symptoms have been ongoing for years, and she was previously diagnosed with a "joint condition," though she cannot recall the name. She has recently noted increased bruising on her arms and recurrent upper respiratory infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Seropositive rheumatoid arthritis (diagnosed age 46, 10-year duration)
- Felty syndrome with chronic neutropenia and splenomegaly
- Osteoporosis secondary to chronic corticosteroid use
- Chronic fatigue syndrome (secondary to autoimmune process)

PAST SURGICAL HISTORY:
- Carpal tunnel release, bilateral (8 years ago)
- Bone marrow biopsy (3 years ago, hypocellular marrow with maturation arrest)
- Diagnostic splenic aspiration (3 years ago, benign)

MEDICATIONS:
Home Medications:
- Methotrexate 20 mg orally once weekly
- Prednisone 5 mg orally every morning
- Folic acid 1 mg orally daily (except methotrexate day)
- Risedronate 35 mg orally weekly
- Calcium carbonate with vitamin D3 600 mg/400 IU orally twice daily
- Filgrastim 300 mcg subcutaneously as needed for ANC <1000

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis), Ibuprofen (gastropathy)

FAMILY HISTORY:
- Mother: rheumatoid arthritis
- Maternal aunt: systemic lupus erythematosus
- Father: type 2 diabetes mellitus, hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstains due to medication regimen). Occupation: Part-time library assistant (reduced hours due to disability).

VITAL SIGNS:
On exam, her temperature is 37.0°C (98.6°F), blood pressure is 122/76 mmHg, pulse is 74/min, and respiratory rate is 14/min.

PHYSICAL EXAMINATION:
Physical examination reveals a mildly enlarged spleen and synovial thickening in both wrists. There is no evidence of active joint erythema or warmth.

LABORATORY DATA:
Lab findings show WBC count 2,400/mm³, hemoglobin 11.1 g/dL, and platelet count 140,000/mm³. Rheumatoid factor and anti-CCP antibodies are positive.

## Encounter 2: source 2021-07-15, practice 2026-06-14, inpatient, Internal Medicine

Chief complaint: Recurrent infections over the past 6 months, including multiple episodes of cellulitis and one episode of pneumonia

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Recurrent infections over the past 6 months, including multiple episodes of cellulitis and one episode of pneumonia

HISTORY OF PRESENT ILLNESS:
This 57-year-old female with seropositive rheumatoid arthritis complicated by Felty syndrome and chronic neutropenia, previously evaluated for persistent joint swelling and morning stiffness in January 2020 and diagnosed with Felty syndrome in July 2020 following presentation with fatigue and bruising, returns for evaluation of recurrent infections over the past six months. She reports that her RA symptoms are currently well-controlled on methotrexate and low-dose prednisone.

PAST MEDICAL HISTORY:
Active Problem List:
- Felty syndrome (diagnosed 2020-07-15)
- Seropositive rheumatoid arthritis (diagnosed age 46, 10-year duration)
- Felty syndrome with chronic neutropenia and splenomegaly
- Osteoporosis secondary to chronic corticosteroid use
- Chronic fatigue syndrome (secondary to autoimmune process)

PAST SURGICAL HISTORY:
- Carpal tunnel release, bilateral (8 years ago)
- Bone marrow biopsy (3 years ago, hypocellular marrow with maturation arrest)
- Diagnostic splenic aspiration (3 years ago, benign)

MEDICATIONS:
She takes methotrexate and low-dose prednisone.

Home Medications:
- Folic acid 1 mg orally daily (except methotrexate day)
- Risedronate 35 mg orally weekly
- Calcium carbonate with vitamin D3 600 mg/400 IU orally twice daily
- Filgrastim 300 mcg subcutaneously as needed for ANC <1000

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis), Ibuprofen (gastropathy)

FAMILY HISTORY:
- Mother: rheumatoid arthritis
- Maternal aunt: systemic lupus erythematosus
- Father: type 2 diabetes mellitus, hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstains due to medication regimen). Occupation: Part-time library assistant (reduced hours due to disability).

VITAL SIGNS:
Afebrile, with a blood pressure of 130/80 mmHg, heart rate of 78 beats per minute, and respiratory rate of 16 beats per minute

PHYSICAL EXAMINATION:
She has no lymphadenopathy. Her joints show no signs of active inflammation, but there are deformities consistent with long-standing RA. Abdominal examination reveals a palpable spleen tip.

LABORATORY DATA:
Hemoglobin: 12.5 g/dL
- White blood cell count: 2.1 x 10^9/L (Neutrophils 800/mm^3)
- Platelet count: 180 x 10^9/L
- Rheumatoid factor: Positive
- Anti-citrullinated peptide antibodies: Positive
