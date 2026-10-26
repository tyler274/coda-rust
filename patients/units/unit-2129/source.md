# synthetic_hospital patient 2129

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 37 at first encounter, sex M, African American, Commercial PPO (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-03 (Mon) | outpatient | Nephrology |
| 1 | 2021-07-15 | 2024-04-01 (Mon) | ed | Emergency Medicine |
| 2 | 2023-01-15 | 2025-10-09 (Thu) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO (Blue Cross Blue Shield)",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "Social (1-2 drinks on weekends)",
 "family_history": [
  "Father: Type 2 diabetes mellitus, hypertension",
  "Mother: Seasonal allergies, hypothyroidism",
  "Paternal uncle: End-stage renal disease (etiology unknown)"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily as needed for allergic rhinitis",
   "name": "Cetirizine"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Seasonal allergic rhinitis",
  "Acne vulgaris, mild"
 ],
 "age_at_first_encounter": 37
}
```

Primary diagnoses: ["Focal Segmental Glomerulosclerosis", "Toxic epidermal necrolysis", "Systemic inflammatory response syndrome"]
Comorbidities: ["Upper respiratory tract infection"]

## Encounter 0: source 2020-01-15, practice 2022-10-03, outpatient, Nephrology

Chief complaint: 2-month history of fatigue and bilateral leg swelling

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Aisha Patel, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
2-month history of fatigue and bilateral leg swelling

HISTORY OF PRESENT ILLNESS:
Treated for a respiratory tract infection three weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Seasonal allergic rhinitis
- Acne vulgaris, mild

MEDICATIONS:
Home Medications:
- Cetirizine 10 mg PO daily as needed for allergic rhinitis

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Seasonal allergies, hypothyroidism
- Paternal uncle: End-stage renal disease (etiology unknown)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks on weekends). Occupation: High school mathematics teacher.

VITAL SIGNS:
Vital signs are normal.

PHYSICAL EXAMINATION:
On physical examination, he has periorbital edema and 2+ pitting edema of the lower extremities.

LABORATORY DATA:
Urinalysis reveals 4+ protein and oval fat bodies under polarized light. Serum studies show an albumin of 2.5 g/dL, total serum cholesterol of 300 mg/dL. Creatinine is 1.0 mg/dL.

PATHOLOGY:
A renal biopsy reveals scattered areas of sclerosis and hyalinosis involving some glomeruli. There are no complement protein or immunoglobulin deposits.

## Encounter 1: source 2021-07-15, practice 2024-04-01, ed, Emergency Medicine

Chief complaint: Fever, malaise, and elevated inflammatory markers after recent upper respiratory tract infection

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, malaise, and elevated inflammatory markers after recent upper respiratory tract infection

HISTORY OF PRESENT ILLNESS:
This 38-year-old male with prior focal segmental glomerulosclerosis presents after a recent upper respiratory tract infection. His immune response is being studied as part of a clinical trial evaluating a new anti-inflammatory drug.

PAST MEDICAL HISTORY:
Active Problem List:
- Focal Segmental Glomerulosclerosis (diagnosed 2020-01-15)
- Seasonal allergic rhinitis
- Acne vulgaris, mild

MEDICATIONS:
Home Medications:
- Cetirizine 10 mg PO daily as needed for allergic rhinitis

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Seasonal allergies, hypothyroidism
- Paternal uncle: End-stage renal disease (etiology unknown)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks on weekends). Occupation: High school mathematics teacher.

OTHER STUDIES:
Analysis reveals that treatment with lipopolysaccharide (LPS) increases expression of pro-inflammatory cytokines such as TNF-α and IL-1.

## Encounter 2: source 2023-01-15, practice 2025-10-09, icu, Critical Care Medicine

Chief complaint: Progressive fatigue, difficulty swallowing, and painful skin lesions following urgent care visit for upper respiratory illness

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Progressive fatigue, difficulty swallowing, and painful skin lesions following urgent care visit for upper respiratory illness

HISTORY OF PRESENT ILLNESS:
This 40-year-old male with a history of Focal Segmental Glomerulosclerosis (2020) and prior Systemic Inflammatory Response Syndrome (2021) presents with acute mucocutaneous symptoms. He states that over the past week he has felt generally unwell with low-grade fevers and body aches. A few days after visiting an urgent care clinic for a suspected upper respiratory illness, he began experiencing redness and irritation on his face and chest. This was followed by tender, expanding areas of skin discoloration that have since developed blisters and peeling. He also reports severe discomfort in his mouth and difficulty opening his eyes due to burning and swelling. He reports trouble drinking fluids and painful urination.

PAST MEDICAL HISTORY:
Active Problem List:
- Focal Segmental Glomerulosclerosis (diagnosed 2020-01-15)
- Systemic inflammatory response syndrome (diagnosed 2021-07-15)
- Seasonal allergic rhinitis
- Acne vulgaris, mild

His past medical history is notable for seasonal allergies and mild acne.

MEDICATIONS:
Medications include an over-the-counter antihistamine and a short course of oral treatment prescribed during his urgent care visit.

Home Medications:
- Cetirizine 10 mg PO daily as needed for allergic rhinitis

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Seasonal allergies, hypothyroidism
- Paternal uncle: End-stage renal disease (etiology unknown)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks on weekends). Occupation: High school mathematics teacher.

VITAL SIGNS:
Febrile, tachycardic

PHYSICAL EXAMINATION:
On examination, the patient appears distressed. His upper trunk and face show widespread dusky erythematous patches with flaccid bullae and sloughing skin. Nikolsky sign is present. Examination of the oropharynx reveals erosions and hemorrhagic crusting, and similar lesions are noted around the eyes and genitals.

LABORATORY DATA:
Laboratory evaluation shows mild leukopenia and elevated liver enzymes.

IMAGING:
Chest radiograph is unremarkable.

PATHOLOGY:
Skin biopsy is obtained and shows full-thickness epidermal necrosis with sparse lymphocytic infiltration.
