# synthetic_hospital patient 2816

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 44 at first encounter, sex F, Non-Hispanic Black, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-07-15 | 2025-09-14 (Sun) | ed | Emergency Medicine |
| 1 | 2021-03-15 | 2026-05-12 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Part-time customer service representative",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Mother: Type 2 diabetes mellitus",
  "Father: Hypertension, stroke (age 67)",
  "Older brother: Opioid use disorder"
 ],
 "race_ethnicity": "Non-Hispanic Black",
 "smoking_status": "Former smoker, quit 18 months ago (20 pack-year history)",
 "home_medications": [
  {
   "dose": "8 mg/2 mg SL daily",
   "name": "Buprenorphine-naloxone"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Bictegravir-tenofovir alafenamide-emtricitabine"
  },
  {
   "dose": "160 mg/800 mg (double strength) PO three times weekly (Mon/Wed/Fri)",
   "name": "Trimethoprim-sulfamethoxazole"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Human immunodeficiency virus infection with AIDS (CD4 < 200)",
  "Opioid use disorder, in sustained remission on maintenance pharmacotherapy"
 ],
 "age_at_first_encounter": 44
}
```

Primary diagnoses: ["Cytomegalovirus esophagitis", "Primary central nervous system lymphoma"]
Comorbidities: ["HIV-related opportunistic skin infection"]

## Encounter 0: source 2020-07-15, practice 2025-09-14, ed, Emergency Medicine

Chief complaint: Progressive blurred vision and left ocular discomfort for the past 6 days with mild photophobia and floaters

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive blurred vision and left ocular discomfort for the past 6 days with mild photophobia and floaters

HISTORY OF PRESENT ILLNESS:
She presents with progressive blurred vision and left ocular discomfort for the past 6 days. She reports mild photophobia and floaters, but denies trauma or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Human immunodeficiency virus infection with AIDS (CD4 < 200)
- Opioid use disorder, in sustained remission on maintenance pharmacotherapy

Her medical history is significant for opioid use disorder, currently managed with a long-acting opioid agonist.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Buprenorphine-naloxone 8 mg/2 mg SL daily
- Bictegravir-tenofovir alafenamide-emtricitabine 1 tablet PO daily
- Trimethoprim-sulfamethoxazole 160 mg/800 mg (double strength) PO three times weekly (Mon/Wed/Fri)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension, stroke (age 67)
- Older brother: Opioid use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 18 months ago (20 pack-year history). Alcohol: Social (1-2 drinks per month). Occupation: Part-time customer service representative.

VITAL SIGNS:
On examination, her temperature is 36.9°C, pulse is 74/min, and blood pressure is 122/68 mm Hg.

PHYSICAL EXAMINATION:
Fundoscopic exam reveals areas of retinal whitening and hemorrhage in the left eye.
She has no skin lesions or oral thrush.

LABORATORY DATA:
Laboratory studies show CD4+ lymphocyte count of 86 cells/μL.

ASSESSMENT AND PLAN:
The physician suspects a viral etiology associated with advanced immunosuppression.

## Encounter 1: source 2021-03-15, practice 2026-05-12, ed, Emergency Medicine

Chief complaint: Progressive confusion and headache over the past week, now nonverbal and lethargic

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Michael Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive confusion and headache over the past week, now nonverbal and lethargic

HISTORY OF PRESENT ILLNESS:
This patient with HIV/AIDS and opioid use disorder in remission, seen July 2020, is brought by her partner to the emergency department due to progressive confusion and headache over the past week and has been nonverbal today appearing lethargic.

PAST MEDICAL HISTORY:
Active Problem List:
- Cytomegalovirus esophagitis (diagnosed 2020-07-15)
- Human immunodeficiency virus infection with AIDS (CD4 < 200)
- Opioid use disorder, in sustained remission on maintenance pharmacotherapy

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She is on antiretroviral therapy and takes trimethoprim-sulfamethoxazole (TMP-SMX) for Pneumocystis jirovecii prophylaxis.

Home Medications:
- Buprenorphine-naloxone 8 mg/2 mg SL daily
- Bictegravir-tenofovir alafenamide-emtricitabine 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus
- Father: Hypertension, stroke (age 67)
- Older brother: Opioid use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 18 months ago (20 pack-year history). Alcohol: Social (1-2 drinks per month). Occupation: Part-time customer service representative.

PHYSICAL EXAMINATION:
Neurologic examination shows disorientation and mild right-sided weakness.

LABORATORY DATA:
Her CD4 count at the last clinic visit was 70 cells/mm³

IMAGING:
CT findings are shown below.
