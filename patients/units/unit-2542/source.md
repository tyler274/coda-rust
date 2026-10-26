# synthetic_hospital patient 2542

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex M, African American, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-03-06 (Mon) | outpatient | Infectious Disease |
| 1 | 2020-07-15 | 2023-08-23 (Wed) | outpatient | Hematology |
| 2 | 2022-01-15 | 2025-02-24 (Mon) | outpatient | Internal Medicine |
| 3 | 2022-07-15 | 2025-09-01 (Mon) | outpatient | Infectious Disease |
| 4 | 2023-01-15 | 2026-02-23 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Logistics coordinator",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother: HIV positive (source of vertical transmission)",
  "Father: Unknown medical history",
  "Maternal grandmother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50mg/25mg/200mg tablet once daily",
   "name": "Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy)"
  },
  {
   "dose": "500mg tablet once daily",
   "name": "Valacyclovir"
  }
 ],
 "surgical_history": [
  "Neonatal circumcision"
 ],
 "chronic_conditions": [
  "HIV infection (vertically acquired)",
  "Genital herpes simplex virus type 2"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Tetanus prophylaxis", "Epididymitis (antibiotic therapy)", "Primary syphilis", "Maternal HIV infection", "HIV-related opportunistic skin infection"]
Comorbidities: ["High risk sexual behavior"]

## Encounter 0: source 2020-01-15, practice 2023-03-06, outpatient, Infectious Disease

Chief complaint: Routine health check-up

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Infectious Disease
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health check-up

HISTORY OF PRESENT ILLNESS:
He has no complaints and is sexually active with multiple partners. He reports consistent condom use but had unprotected intercourse with a new partner two months ago.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV infection (vertically acquired)
- Genital herpes simplex virus type 2

He has a past medical history of genital herpes.

PAST SURGICAL HISTORY:
- Neonatal circumcision

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50mg/25mg/200mg tablet once daily
- Valacyclovir 500mg tablet once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His family history is non-contributory.

SOCIAL HISTORY:
He denies any history of intravenous drug use

VITAL SIGNS:
Vital signs are within normal limits

PHYSICAL EXAMINATION:
On physical examination, he appears well. His physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal the following:

- HIV-1 antibody: Positive
- HIV-1 RNA (viral load): 55,000 copies/mL
- CD4 count: 600 cells/mm³
- Complete blood count: Normal
- Hepatitis panel: Negative

## Encounter 1: source 2020-07-15, practice 2023-08-23, outpatient, Hematology

Chief complaint: 2-week history of easy bruising and petechial rash on lower extremities

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of easy bruising and petechial rash on lower extremities

HISTORY OF PRESENT ILLNESS:
This 32-year-old male with vertically acquired HIV and genital herpes simplex virus type 2, maintained on Biktarvy and valacyclovir and last evaluated in Infectious Disease clinic six months prior, presents to Hematology with a two-week history of easy bruising and petechial rash on the lower extremities. He denies any recent infections, medications, or significant medical history.

PAST MEDICAL HISTORY:
Active Problem List:
- Maternal HIV infection (diagnosed 2020-01-15)
- HIV infection (vertically acquired)
- Genital herpes simplex virus type 2

PAST SURGICAL HISTORY:
- Neonatal circumcision

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50mg/25mg/200mg tablet once daily
- Valacyclovir 500mg tablet once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: HIV positive (source of vertical transmission)
- Father: Unknown medical history
- Maternal grandmother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Logistics coordinator.

PHYSICAL EXAMINATION:
On examination, you note no hepatosplenomegaly or lymphadenopathy.

LABORATORY DATA:
His complete blood count reveals a platelet count of 25 x 10^9/L, hemoglobin of 14 g/dL, and white blood cell count of 6.5 x 10^9/L. Peripheral blood smear shows normal cell morphology with isolated thrombocytopenia. Coagulation studies are within normal limits. The hepatitis C antibody test returns negative, but the HIV 1/2 antigen/antibody combination immunoassay is reactive, which is confirmed by a positive HIV-1 RNA PCR test.

PLAN:
Serologic tests for hepatitis C and HIV are ordered given the patient's thrombocytopenia.

## Encounter 2: source 2022-01-15, practice 2025-02-24, outpatient, Internal Medicine

Chief complaint: Routine follow-up visit in October

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit in October

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old male with vertically acquired HIV and genital herpes simplex virus type 2 who returns for routine follow-up, having previously presented with easy bruising. He reports adherence to his medication regimen and has no complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Maternal HIV infection (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-07-15)
- HIV infection (vertically acquired)
- Genital herpes simplex virus type 2

He received his last influenza vaccine 1 year ago. He cannot recall if he has ever had chickenpox or shingles, and he does not know his hepatitis A or B status.

PAST SURGICAL HISTORY:
- Neonatal circumcision

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50mg/25mg/200mg tablet once daily
- Valacyclovir 500mg tablet once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: HIV positive (source of vertical transmission)
- Father: Unknown medical history
- Maternal grandmother: Type 2 diabetes mellitus

SOCIAL HISTORY:
He is sexually active with multiple partners and uses condoms inconsistently.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

## Encounter 3: source 2022-07-15, practice 2025-09-01, outpatient, Infectious Disease

Chief complaint: Painless ulcer on penis for 3 weeks

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Sarah Chen, MD
Department: Infectious Disease
Visit Type: outpatient

CHIEF COMPLAINT:
Painless ulcer on penis for 3 weeks

HISTORY OF PRESENT ILLNESS:
The patient with vertically acquired HIV and HSV-2 returns with a new complaint. He reports that the ulcer appeared 3 weeks ago and has not changed in size. He denies any discharge from the lesion but notes mild inguinal lymphadenopathy.

PAST MEDICAL HISTORY:
Active Problem List:
- Maternal HIV infection (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-07-15)
- Tetanus prophylaxis (diagnosed 2022-01-15)
- HIV infection (vertically acquired)
- Genital herpes simplex virus type 2

PAST SURGICAL HISTORY:
- Neonatal circumcision

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50mg/25mg/200mg tablet once daily
- Valacyclovir 500mg tablet once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: HIV positive (source of vertical transmission)
- Father: Unknown medical history
- Maternal grandmother: Type 2 diabetes mellitus

SOCIAL HISTORY:
The patient is sexually active with multiple partners and inconsistently uses condoms.

PHYSICAL EXAMINATION:
On physical examination, there is a single, indurated, non-tender ulcer on the shaft of the penis, and bilateral non-tender inguinal lymphadenopathy is palpable.

LABORATORY DATA:
Rapid plasma reagin (RPR) test returns negative, but the treponemal pallidum particle agglutination assay (TP-PA) is positive.

Laboratory Values:
- RPR: Non-reactive
- TP-PA: Reactive

## Encounter 4: source 2023-01-15, practice 2026-02-23, ed, Emergency Medicine

Chief complaint: Severe pain and swelling in left testicle for 2 days with dysuria and penile discharge

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James O'Connor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe pain and swelling in left testicle for 2 days with dysuria and penile discharge

HISTORY OF PRESENT ILLNESS:
This patient with vertically acquired HIV, genital HSV-2, and prior primary syphilis presents for evaluation. He reports that the pain improves slightly when he lies down and elevates his scrotum. He also notes a recent burning sensation during urination and a milky discharge from his penis.

PAST MEDICAL HISTORY:
Active Problem List:
- Maternal HIV infection (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-07-15)
- Tetanus prophylaxis (diagnosed 2022-01-15)
- Primary syphilis (diagnosed 2022-07-15)
- HIV infection (vertically acquired)
- Genital herpes simplex virus type 2

PAST SURGICAL HISTORY:
- Neonatal circumcision

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50mg/25mg/200mg tablet once daily
- Valacyclovir 500mg tablet once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: HIV positive (source of vertical transmission)
- Father: Unknown medical history
- Maternal grandmother: Type 2 diabetes mellitus

SOCIAL HISTORY:
He is sexually active with multiple partners and does not consistently use condoms.

VITAL SIGNS:
His temperature is 38.3°C (100.9°F), blood pressure is 130/80 mmHg, and heart rate is 88 beats per minute.

PHYSICAL EXAMINATION:
Physical examination reveals a swollen, tender left testicle that is retracted upwards. The pain is relieved with testicular elevation (Prehn&#x27;s sign). The cremasteric reflex is intact.

LABORATORY DATA:
Urinalysis shows pyuria and a positive leukocyte esterase. Urine nucleic acid amplification testing (NAAT) is positive for Chlamydia trachomatis.
