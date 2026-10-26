# synthetic_hospital patient 2552

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex M, Caucasian, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-04-21 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2023-10-15 (Sun) | ed | Emergency Medicine |
| 2 | 2022-01-15 | 2025-04-08 (Tue) | inpatient | Emergency Medicine |
| 3 | 2023-01-15 | 2026-04-11 (Sat) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "International aid worker",
 "alcohol_use": "None",
 "family_history": [
  "Father: Hypertension",
  "Mother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Caucasian",
 "smoking_status": "Former smoker, quit 2019",
 "home_medications": [
  {
   "dose": "50/25/200 mg PO daily",
   "name": "Bictegravir/Tenofovir alafenamide/Emtricitabine"
  },
  {
   "dose": "300 mg PO daily",
   "name": "Isoniazid"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Pyridoxine"
  },
  {
   "dose": "160/800 mg PO daily",
   "name": "Trimethoprim-sulfamethoxazole"
  },
  {
   "dose": "400 mg PO daily",
   "name": "Fluconazole"
  }
 ],
 "surgical_history": [
  "Appendectomy"
 ],
 "chronic_conditions": [
  "HIV infection",
  "Latent tuberculosis infection",
  "History of cryptococcal meningitis",
  "History of Plasmodium vivax malaria"
 ],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Cryptococcal meningitis", "Latent tuberculosis infection", "Chloroquine-sensitive malaria prophylaxis"]
Comorbidities: ["Medication noncompliance", "Immunosuppressed state"]

## Encounter 0: source 2020-01-15, practice 2023-04-21, outpatient, Internal Medicine

Chief complaint: presents to the clinic for evaluation after testing positive for HIV

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for evaluation after testing positive for HIV

PAST MEDICAL HISTORY:
Active Problem List:
- HIV infection
- Latent tuberculosis infection
- History of cryptococcal meningitis
- History of Plasmodium vivax malaria

PAST SURGICAL HISTORY:
- Appendectomy

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/25/200 mg PO daily
- Isoniazid 300 mg PO daily
- Pyridoxine 50 mg PO daily
- Trimethoprim-sulfamethoxazole 160/800 mg PO daily
- Fluconazole 400 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus

SOCIAL HISTORY:
History of intravenous drug use

REVIEW OF SYSTEMS:
He reports no symptoms of cough, fever, night sweats, or weight loss

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
His CD4 count is 350 cells/mm³ and viral load is 55,000 copies/mL

IMAGING:
A chest x-ray shows no evidence of active tuberculosis

OTHER STUDIES:
A tuberculin skin test (TST) is performed, and 48 hours later, the induration measures 7 mm

## Encounter 1: source 2020-07-15, practice 2023-10-15, ed, Emergency Medicine

Chief complaint: 3-week history of progressively worsening headache, intermittent low-grade fevers, and difficulty concentrating

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-week history of progressively worsening headache, intermittent low-grade fevers, and difficulty concentrating

HISTORY OF PRESENT ILLNESS:
This 33-year-old male with HIV and latent tuberculosis since January 2020 and history of cryptococcal meningitis presents with progressive symptoms. He reports occasional blurred vision, noting his symptoms began shortly after returning from a backpacking trip in the Pacific Northwest, during which he stayed in multiple hostels and hiked in wooded areas. He has lost 5 kg over the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Latent tuberculosis infection (diagnosed 2020-01-15)
- HIV infection
- History of cryptococcal meningitis
- History of Plasmodium vivax malaria

He has no prior medical conditions.

PAST SURGICAL HISTORY:
- Appendectomy

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/25/200 mg PO daily
- Isoniazid 300 mg PO daily
- Pyridoxine 50 mg PO daily
- Trimethoprim-sulfamethoxazole 160/800 mg PO daily
- Fluconazole 400 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus

SOCIAL HISTORY:
He admits to not seeing a physician in several years

PHYSICAL EXAMINATION:
On exam, he is afebrile, oriented but slow to respond, with mild nuchal rigidity and bilateral papilledema.

LABORATORY DATA:
Lumbar puncture reveals an elevated opening pressure, lymphocytic predominance, elevated protein, and decreased glucose. CSF staining is shown.

IMAGING:
A CT scan of the head shows no mass lesions or hydrocephalus.

## Encounter 2: source 2022-01-15, practice 2025-04-08, inpatient, Emergency Medicine

Chief complaint: 4-day history of fevers, chills, fatigue, and vague abdominal discomfort

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Sharma, MD
Department: Emergency Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
4-day history of fevers, chills, fatigue, and vague abdominal discomfort

HISTORY OF PRESENT ILLNESS:
This 35-year-old male with HIV infection, latent tuberculosis infection, and history of cryptococcal meningitis presents with recurrent fevers and musculoskeletal complaints. He returned to the United States 3 weeks ago after spending 18 months working in rural western Uganda. He did not take any malaria prophylaxis. He reports having episodes of fever and sweating every other day, accompanied by headache and muscle aches.

PAST MEDICAL HISTORY:
Active Problem List:
- Latent tuberculosis infection (diagnosed 2020-01-15)
- Cryptococcal meningitis (diagnosed 2020-07-15)
- HIV infection
- History of Plasmodium vivax malaria

PAST SURGICAL HISTORY:
- Appendectomy

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/25/200 mg PO daily
- Isoniazid 300 mg PO daily
- Pyridoxine 50 mg PO daily
- Trimethoprim-sulfamethoxazole 160/800 mg PO daily
- Fluconazole 400 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 2019. Alcohol: None. Occupation: International aid worker.

VITAL SIGNS:
His temperature is 39.5°C (103.1°F)

PHYSICAL EXAMINATION:
On physical examination, he appears tired and mildly diaphoretic. Scleral icterus is present. The spleen is palpable 2 cm below the left costal margin.

LABORATORY DATA:
Lab results show hemoglobin of 9.2 g/dL, platelets 90,000/mm³, reticulocyte count 4.8%, and lactate dehydrogenase (LDH) of 340 U/L.

## Encounter 3: source 2023-01-15, practice 2026-04-11, icu, Emergency Medicine

Chief complaint: 2-week history of low-grade fever, headache, and malaise

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
2-week history of low-grade fever, headache, and malaise

HISTORY OF PRESENT ILLNESS:
This 36-year-old male with HIV infection and a history of cryptococcal meningitis diagnosed in 2020 returns to the emergency department with a 2-week history of low-grade fever, headache, and malaise. He reports that his symptoms have gradually worsened over this period. He has not been compliant with his antiretroviral therapy for the past 6 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Latent tuberculosis infection (diagnosed 2020-01-15)
- Cryptococcal meningitis (diagnosed 2020-07-15)
- Chloroquine-sensitive malaria prophylaxis (diagnosed 2022-01-15)
- HIV infection
- History of Plasmodium vivax malaria

PAST SURGICAL HISTORY:
- Appendectomy

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/25/200 mg PO daily
- Isoniazid 300 mg PO daily
- Pyridoxine 50 mg PO daily
- Trimethoprim-sulfamethoxazole 160/800 mg PO daily
- Fluconazole 400 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 2019. Alcohol: None. Occupation: International aid worker.

VITAL SIGNS:
Temperature is 38.2°C (100.8°F), blood pressure is 120/75 mmHg, pulse is 88/min, and respiratory rate is 16/min

PHYSICAL EXAMINATION:
Neurological examination reveals nuchal rigidity and a positive Brudzinski's sign. Fundoscopic examination is normal. No focal neurological deficits are noted.

LABORATORY DATA:
CD4 count of 45 cells/mm³

OTHER STUDIES:
A lumbar puncture is performed, and the cerebrospinal fluid (CSF) analysis reveals:

- Opening pressure: 280 mmH2O
- WBC: 20 cells/µL (80% lymphocytes)
- Protein: 50 mg/dL
- Glucose: 25 mg/dL (serum glucose 90 mg/dL)
- India ink stain: Negative
- Cryptococcal antigen: Pending
