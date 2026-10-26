# synthetic_hospital patient 2522

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex M, African American, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-07-17 (Mon) | outpatient | Nephrology |
| 1 | 2020-07-15 | 2024-01-08 (Mon) | outpatient | Nephrology |
| 2 | 2021-01-15 | 2024-07-16 (Tue) | outpatient | Internal Medicine |
| 3 | 2021-07-15 | 2025-01-06 (Mon) | outpatient | Pulmonology |
| 4 | 2022-01-15 | 2025-07-08 (Tue) | outpatient | Gastroenterology |
| 5 | 2022-07-15 | 2026-01-19 (Mon) | ed | Emergency Medicine |
| 6 | 2022-10-15 | 2026-04-06 (Mon) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-07-21 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Unemployed; formerly worked as security guard",
 "alcohol_use": "Occasional social use, 1-2 drinks monthly",
 "family_history": [
  "Type 2 diabetes mellitus (mother, diagnosed age 45)",
  "Hypertension (father)",
  "End-stage renal disease (maternal uncle, unknown cause)",
  "HIV-negative (partner)"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Current smoker, 10 cigarettes per day for 15 years",
 "home_medications": [
  {
   "dose": "50/200/25 mg tablet daily (prescribed, adherence poor)",
   "name": "Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine)"
  },
  {
   "dose": "1000 mg oral tablet twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg oral tablet daily",
   "name": "Lisinopril"
  },
  {
   "dose": "800/160 mg tablet daily (for PCP prophylaxis, prescribed)",
   "name": "Trimethoprim-sulfamethoxazole"
  }
 ],
 "surgical_history": [
  "Appendectomy (age 22)"
 ],
 "chronic_conditions": [
  "HIV infection (AIDS, CD4 count <200 cells/mm\u00b3)",
  "IgA nephropathy",
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Chronic kidney disease stage 3"
 ],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["HIV-related opportunistic skin infection", "Cryptococcal meningitis", "Candidal esophagitis", "Tuberculosis", "Cryptosporidiosis", "Cerebral toxoplasmosis", "Focal Segmental Glomerulosclerosis", "IgA nephropathy"]
Comorbidities: ["Medication noncompliance", "HIV-related opportunistic skin infection", "Nephrotic syndrome (edema management)", "Acute kidney injury", "Immunosuppressed state", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2023-07-17, outpatient, Nephrology

Chief complaint: 2-day history of dark urine and mild flank pain following a recent upper respiratory infection

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
2-day history of dark urine and mild flank pain following a recent upper respiratory infection

HISTORY OF PRESENT ILLNESS:
He reports a recent upper respiratory infection that resolved a week ago.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV infection (AIDS, CD4 count <200 cells/mm³)
- IgA nephropathy
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 3

His medical history is significant for similar episodes over the past 3 years, usually following respiratory infections.

PAST SURGICAL HISTORY:
- Appendectomy (age 22)

MEDICATIONS:
He denies any use of medications, including NSAIDs,

Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 50/200/25 mg tablet daily (prescribed, adherence poor)
- Metformin 1000 mg oral tablet twice daily
- Lisinopril 20 mg oral tablet daily
- Trimethoprim-sulfamethoxazole 800/160 mg tablet daily (for PCP prophylaxis, prescribed)

ALLERGIES:
NKDA

FAMILY HISTORY:
He has no family history of kidney disease.

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day for 15 years. Alcohol: Occasional social use, 1-2 drinks monthly. Occupation: Unemployed; formerly worked as security guard.

VITAL SIGNS:
His blood pressure is 152/96 mmHg

PHYSICAL EXAMINATION:
On physical examination, there are no rashes, joint swelling, or other abnormalities noted.

LABORATORY DATA:
Urinalysis reveals red blood cell casts and proteinuria. Laboratory results are as follows:

- Serum creatinine: 1.8 mg/dL (normal: 0.6-1.2 mg/dL)
- Serum albumin: 3.9 g/dL (normal: 3.5-5.0 g/dL)
- Urine protein: 2.1 g/day (normal: <150 mg/day)

## Encounter 1: source 2020-07-15, practice 2024-01-08, outpatient, Nephrology

Chief complaint: Progressive swelling in legs and occasional frothy urine over 2 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive swelling in legs and occasional frothy urine over 2 months

HISTORY OF PRESENT ILLNESS:
The patient with HIV (AIDS) and IgA nephropathy returns to nephrology clinic 6 months after diagnosis presenting with a 2-month history of progressive swelling in his legs and occasional frothy urine. He denies any recent infections, drug use, or changes in medications.

PAST MEDICAL HISTORY:
Active Problem List:
- IgA nephropathy (diagnosed 2020-01-15)
- HIV infection (AIDS, CD4 count <200 cells/mm³)
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 3

He has a past medical history of type 2 diabetes mellitus and hypertension, which are controlled with metformin and lisinopril, respectively.

PAST SURGICAL HISTORY:
- Appendectomy (age 22)

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 50/200/25 mg tablet daily (prescribed, adherence poor)
- Trimethoprim-sulfamethoxazole 800/160 mg tablet daily (for PCP prophylaxis, prescribed)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother, diagnosed age 45)
- Hypertension (father)
- End-stage renal disease (maternal uncle, unknown cause)
- HIV-negative (partner)

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day for 15 years. Alcohol: Occasional social use, 1-2 drinks monthly. Occupation: Unemployed; formerly worked as security guard.

VITAL SIGNS:
His blood pressure is 145/90 mmHg.

PHYSICAL EXAMINATION:
On physical examination, he has 3+ pitting edema in both lower extremities up to the knees and mild periorbital puffiness.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum albumin: 2.1 g/dL (Normal: 3.5-5.0 g/dL)
- Urine protein: 4.2 g/24 hr
- Serum creatinine: 1.2 mg/dL (Normal: 0.6-1.2 mg/dL)
- Fasting blood glucose: 130 mg/dL
- HbA1c: 7.2%
- HIV test: Negative
- Hepatitis panel: Negative

PATHOLOGY:
A renal biopsy is performed, and the pathology report indicates segmental sclerosis and hyalinosis in some glomeruli, without significant immune complex deposition.

## Encounter 2: source 2021-01-15, practice 2024-07-16, outpatient, Internal Medicine

Chief complaint: 2-week history of swelling in legs and occasional frothy urine

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of swelling in legs and occasional frothy urine

HISTORY OF PRESENT ILLNESS:
The patient with HIV and CKD reports 2-week history of swelling in his legs and occasional frothy urine.

PAST MEDICAL HISTORY:
Active Problem List:
- IgA nephropathy (diagnosed 2020-01-15)
- Focal Segmental Glomerulosclerosis (diagnosed 2020-07-15)
- HIV infection (AIDS, CD4 count <200 cells/mm³)
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 3

He has a known history of HIV infection but has been non-compliant with his antiretroviral therapy for the past 6 months. The patient denies any recent use of nephrotoxic drugs or history of diabetes mellitus.

PAST SURGICAL HISTORY:
- Appendectomy (age 22)

MEDICATIONS:
Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 50/200/25 mg tablet daily (prescribed, adherence poor)
- Metformin 1000 mg oral tablet twice daily
- Lisinopril 20 mg oral tablet daily
- Trimethoprim-sulfamethoxazole 800/160 mg tablet daily (for PCP prophylaxis, prescribed)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother, diagnosed age 45)
- Hypertension (father)
- End-stage renal disease (maternal uncle, unknown cause)
- HIV-negative (partner)

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day for 15 years. Alcohol: Occasional social use, 1-2 drinks monthly. Occupation: Unemployed; formerly worked as security guard.

VITAL SIGNS:
His blood pressure is 150/95 mmHg.

PHYSICAL EXAMINATION:
Physical examination reveals pitting edema in both lower extremities up to the level of the knees.

LABORATORY DATA:
Urinalysis shows 4+ proteinuria and oval fat bodies. Serum creatinine is elevated at 3.2 mg/dL (normal: 0.6-1.3 mg/dL), and his CD4 count is 180 cells/mm³ (normal: 500-1500 cells/mm³).

## Encounter 3: source 2021-07-15, practice 2025-01-06, outpatient, Pulmonology

Chief complaint: 3-week history of persistent cough, low-grade fever, night sweats, and 10-pound weight loss

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Johnson, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
3-week history of persistent cough, low-grade fever, night sweats, and 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
This 36-year-old male with HIV/AIDS and CKD stage 3, previously evaluated for IgA nephropathy and a recent opportunistic infection, presents to Pulmonology. He reports intermittent night sweats and increasing fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- IgA nephropathy (diagnosed 2020-01-15)
- Focal Segmental Glomerulosclerosis (diagnosed 2020-07-15)
- HIV-related opportunistic skin infection (diagnosed 2021-01-15)
- HIV infection (AIDS, CD4 count <200 cells/mm³)
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 3

PAST SURGICAL HISTORY:
- Appendectomy (age 22)

MEDICATIONS:
His antiretroviral therapy has been irregular due to social circumstances.

Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 50/200/25 mg tablet daily (prescribed, adherence poor)
- Metformin 1000 mg oral tablet twice daily
- Lisinopril 20 mg oral tablet daily
- Trimethoprim-sulfamethoxazole 800/160 mg tablet daily (for PCP prophylaxis, prescribed)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother, diagnosed age 45)
- Hypertension (father)
- End-stage renal disease (maternal uncle, unknown cause)
- HIV-negative (partner)

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day for 15 years. Alcohol: Occasional social use, 1-2 drinks monthly. Occupation: Unemployed; formerly worked as security guard.

VITAL SIGNS:
His temperature is 38.2°C, heart rate is 92 beats per minute, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
On examination, he is cachectic and has diminished breath sounds in the right upper lobe.

LABORATORY DATA:
His CD4 count is 180 cells/mm³, and his viral load is elevated.
Sputum acid-fast bacilli (AFB) stain is positive.

IMAGING:
A chest x-ray reveals a right upper lobe cavitary lesion.

## Encounter 4: source 2022-01-15, practice 2025-07-08, outpatient, Gastroenterology

Chief complaint: Painful swallowing for two weeks with 10-pound weight loss and low-grade fever

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful swallowing for two weeks with 10-pound weight loss and low-grade fever

HISTORY OF PRESENT ILLNESS:
This 37-year-old male with AIDS, IgA nephropathy, and history of opportunistic infections including tuberculosis presents with painful swallowing. He reports a weight loss of 10 pounds during this period and has a low-grade fever. His current CD4 count is 50 cells/mm³, and his antiretroviral therapy has been irregular. Despite one week of oral nystatin therapy, his symptoms have persisted.

PAST MEDICAL HISTORY:
Active Problem List:
- IgA nephropathy (diagnosed 2020-01-15)
- Focal Segmental Glomerulosclerosis (diagnosed 2020-07-15)
- HIV-related opportunistic skin infection (diagnosed 2021-01-15)
- Tuberculosis (diagnosed 2021-07-15)
- HIV infection (AIDS, CD4 count <200 cells/mm³)
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 3

PAST SURGICAL HISTORY:
- Appendectomy (age 22)

MEDICATIONS:
Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 50/200/25 mg tablet daily (prescribed, adherence poor)
- Metformin 1000 mg oral tablet twice daily
- Lisinopril 20 mg oral tablet daily
- Trimethoprim-sulfamethoxazole 800/160 mg tablet daily (for PCP prophylaxis, prescribed)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother, diagnosed age 45)
- Hypertension (father)
- End-stage renal disease (maternal uncle, unknown cause)
- HIV-negative (partner)

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day for 15 years. Alcohol: Occasional social use, 1-2 drinks monthly. Occupation: Unemployed; formerly worked as security guard.

PHYSICAL EXAMINATION:
On examination, oral thrush is noted.

LABORATORY DATA:
His laboratory values are as follows:

- Hemoglobin: 12.5 g/dL
- White blood cell count: 3,800 cells/mm³
- Platelets: 150,000 cells/mm³
- CD4 count: 50 cells/mm³
- HIV viral load: 120,000 copies/mL

## Encounter 5: source 2022-07-15, practice 2026-01-19, ed, Emergency Medicine

Chief complaint: 3-week history of watery diarrhea and 10-pound weight loss

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Emily Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-week history of watery diarrhea and 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
The patient is a 37-year-old male with AIDS (complicated by poor antiretroviral adherence), IgA nephropathy with superimposed FSGS, and CKD stage 3 who presents with a 3-week history of watery diarrhea and 10-pound weight loss, occurring six months after an episode of candidal esophagitis and one year after tuberculosis. He reports no fever, blood in the stool, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- IgA nephropathy (diagnosed 2020-01-15)
- Focal Segmental Glomerulosclerosis (diagnosed 2020-07-15)
- HIV-related opportunistic skin infection (diagnosed 2021-01-15)
- Tuberculosis (diagnosed 2021-07-15)
- Candidal esophagitis (diagnosed 2022-01-15)
- HIV infection (AIDS, CD4 count <200 cells/mm³)
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 3

PAST SURGICAL HISTORY:
- Appendectomy (age 22)

MEDICATIONS:
His antiretroviral therapy has been sporadic due to social circumstances.

Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 50/200/25 mg tablet daily (prescribed, adherence poor)
- Metformin 1000 mg oral tablet twice daily
- Lisinopril 20 mg oral tablet daily
- Trimethoprim-sulfamethoxazole 800/160 mg tablet daily (for PCP prophylaxis, prescribed)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother, diagnosed age 45)
- Hypertension (father)
- End-stage renal disease (maternal uncle, unknown cause)
- HIV-negative (partner)

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day for 15 years. Alcohol: Occasional social use, 1-2 drinks monthly. Occupation: Unemployed; formerly worked as security guard.

PHYSICAL EXAMINATION:
His physical examination is notable for mild abdominal tenderness without rebound or guarding. His mucous membranes appear dry.

LABORATORY DATA:
CD4 count: 50 cells/mm³
HIV viral load: 120,000 copies/mL
Stool culture: No Salmonella, Shigella, or Campylobacter isolated
Stool ova and parasites: Negative
Stool acid-fast stain: Positive for acid-fast bacilli
Clostridium difficile antigen: Negative

## Encounter 6: source 2022-10-15, practice 2026-04-06, ed, Emergency Medicine

Chief complaint: 2-week history of headache, fever, and neck stiffness progressively worsening despite acetaminophen

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. James O'Connor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-week history of headache, fever, and neck stiffness progressively worsening despite acetaminophen

HISTORY OF PRESENT ILLNESS:
This 37-year-old male with AIDS, IgA nephropathy, and recent cryptosporidiosis returns to the emergency department with a 2-week history of headache, fever, and neck stiffness. He reports that his symptoms have progressively worsened despite taking acetaminophen.

PAST MEDICAL HISTORY:
Active Problem List:
- IgA nephropathy (diagnosed 2020-01-15)
- Focal Segmental Glomerulosclerosis (diagnosed 2020-07-15)
- HIV-related opportunistic skin infection (diagnosed 2021-01-15)
- Tuberculosis (diagnosed 2021-07-15)
- Candidal esophagitis (diagnosed 2022-01-15)
- Cryptosporidiosis (diagnosed 2022-07-15)
- HIV infection (AIDS, CD4 count <200 cells/mm³)
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 3

His medical history is significant for HIV diagnosed 5 years ago, but he admits to poor adherence to antiretroviral therapy.

PAST SURGICAL HISTORY:
- Appendectomy (age 22)

MEDICATIONS:
Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 50/200/25 mg tablet daily (prescribed, adherence poor)
- Metformin 1000 mg oral tablet twice daily
- Lisinopril 20 mg oral tablet daily
- Trimethoprim-sulfamethoxazole 800/160 mg tablet daily (for PCP prophylaxis, prescribed)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother, diagnosed age 45)
- Hypertension (father)
- End-stage renal disease (maternal uncle, unknown cause)
- HIV-negative (partner)

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day for 15 years. Alcohol: Occasional social use, 1-2 drinks monthly. Occupation: Unemployed; formerly worked as security guard.

VITAL SIGNS:
His temperature is 38.5°C, blood pressure is 110/70 mmHg, pulse is 100/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
On examination, he is lethargic but oriented to time, place, and person. Neck stiffness and a positive Brudzinski's sign are noted. No focal neurological deficits are observed.

LABORATORY DATA:
Laboratory tests reveal:

- WBC: 4,500/µL
- Hemoglobin: 12.2 g/dL
- Platelets: 210,000/µL
- CD4 cell count: 50/µL
- HIV viral load: 120,000 copies/mL
- CSF opening pressure: 280 mm H2O
- CSF analysis:
- WBC: 22/µL (90% lymphocytes)
- Protein: 100 mg/dL
- Glucose: 20 mg/dL
- India ink stain: Positive
- Cryptococcal antigen: Positive

## Encounter 7: source 2023-01-15, practice 2026-07-21, ed, Emergency Medicine

Chief complaint: 2-week history of progressive headache, fever, confusion, and difficulty walking

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-week history of progressive headache, fever, confusion, and difficulty walking

HISTORY OF PRESENT ILLNESS:
Patient with HIV/AIDS and prior cryptococcal meningitis presents. His partner reports he has been less responsive and has difficulty walking.

PAST MEDICAL HISTORY:
Active Problem List:
- IgA nephropathy (diagnosed 2020-01-15)
- Focal Segmental Glomerulosclerosis (diagnosed 2020-07-15)
- HIV-related opportunistic skin infection (diagnosed 2021-01-15)
- Tuberculosis (diagnosed 2021-07-15)
- Candidal esophagitis (diagnosed 2022-01-15)
- Cryptosporidiosis (diagnosed 2022-07-15)
- Cryptococcal meningitis (diagnosed 2022-10-15)
- HIV infection (AIDS, CD4 count <200 cells/mm³)
- Type 2 diabetes mellitus
- Hypertension
- Chronic kidney disease stage 3

PAST SURGICAL HISTORY:
- Appendectomy (age 22)

MEDICATIONS:
Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 50/200/25 mg tablet daily (prescribed, adherence poor)
- Metformin 1000 mg oral tablet twice daily
- Lisinopril 20 mg oral tablet daily
- Trimethoprim-sulfamethoxazole 800/160 mg tablet daily (for PCP prophylaxis, prescribed)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother, diagnosed age 45)
- Hypertension (father)
- End-stage renal disease (maternal uncle, unknown cause)
- HIV-negative (partner)

SOCIAL HISTORY:
Smoking: Current smoker, 10 cigarettes per day for 15 years. Alcohol: Occasional social use, 1-2 drinks monthly. Occupation: Unemployed; formerly worked as security guard.

PHYSICAL EXAMINATION:
On examination, the patient is febrile with a temperature of 38.7°C, has a stiff neck, and exhibits right-sided weakness.

LABORATORY DATA:
His CD4 count is 50/mm3.

IMAGING:
An MRI of the brain reveals multiple ring-enhancing lesions in the cerebral cortex.
