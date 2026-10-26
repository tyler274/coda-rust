# synthetic_hospital patient 1967

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 26 at first encounter, sex F, White, non-Hispanic, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 1 | 2020-03-15 | 2025-10-16 (Thu) | ed | Emergency Medicine |
| 2 | 2020-09-15 | 2026-04-14 (Tue) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Pollen",
  "Dust mites"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Veterinary technician",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Mother: Hypothyroidism",
  "Father: Essential hypertension",
  "Maternal aunt: Rheumatoid arthritis"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 spray each nostril daily",
   "name": "Fluticasone nasal spray"
  },
  {
   "dose": "2 puffs every 4-6 hours as needed",
   "name": "Albuterol inhaler"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Ethinyl estradiol/levonorgestrel"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction (age 17)",
  "Tonsillectomy (age 5)"
 ],
 "chronic_conditions": [
  "Seasonal allergic rhinitis",
  "Asthma (mild, well-controlled)"
 ],
 "age_at_first_encounter": 26
}
```

Primary diagnoses: ["Cat-scratch disease"]
Comorbidities: []

## Encounter 1: source 2020-03-15, practice 2025-10-16, ed, Emergency Medicine

Chief complaint: 1-week history of fatigue, night sweats, and low-grade fever with painful lump in right axilla noticed 2 days ago

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
1-week history of fatigue, night sweats, and low-grade fever with painful lump in right axilla noticed 2 days ago

HISTORY OF PRESENT ILLNESS:
This 26-year-old with seasonal allergic rhinitis and asthma reports a painful lump in her right axilla noticed two days ago that has progressively enlarged.

PAST MEDICAL HISTORY:
Active Problem List:
- Seasonal allergic rhinitis
- Asthma (mild, well-controlled)

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 17)
- Tonsillectomy (age 5)

MEDICATIONS:
Home Medications:
- Fluticasone nasal spray 1 spray each nostril daily
- Albuterol inhaler 2 puffs every 4-6 hours as needed
- Ethinyl estradiol/levonorgestrel 1 tablet PO daily

ALLERGIES:
Pollen, Dust mites

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Essential hypertension
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
She recently adopted a pet cat and set up a fish tank with 10 different subtropical fish.

PHYSICAL EXAMINATION:
On exam, there is a warm, erythematous, tender lymph node with overlying induration.

LABORATORY DATA:
Laboratory testing shows mild leukocytosis.

PATHOLOGY:
A biopsy of the axillary lymph node reveals granulomas with central stellate microabscesses surrounded by histiocytes and neutrophils.

## Encounter 2: source 2020-09-15, practice 2026-04-14, outpatient, Internal Medicine

Chief complaint: 1-week history of fever, fatigue, tender swelling in right axilla, and recent headaches

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
1-week history of fever, fatigue, tender swelling in right axilla, and recent headaches

HISTORY OF PRESENT ILLNESS:
The patient returns six months after her March 2020 emergency department visit for similar symptoms. She reports a one-week history of fever, fatigue, tender right axillary swelling, recent headaches, and night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Cat-scratch disease (diagnosed 2020-03-15)
- Seasonal allergic rhinitis
- Asthma (mild, well-controlled)

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 17)
- Tonsillectomy (age 5)

MEDICATIONS:
Home Medications:
- Fluticasone nasal spray 1 spray each nostril daily
- Albuterol inhaler 2 puffs every 4-6 hours as needed
- Ethinyl estradiol/levonorgestrel 1 tablet PO daily

ALLERGIES:
Pollen, Dust mites

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Essential hypertension
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
She works at a veterinary clinic and recently fostered a litter of stray kittens.

VITAL SIGNS:
On exam, her temperature is 38.1°C (100.6°F).

PHYSICAL EXAMINATION:
The right axillary lymph node is firm, tender, and mobile, with overlying erythema and warmth. Multiple healed and fresh scratches are noted on her forearms.

LABORATORY DATA:
Laboratory tests are unremarkable. An indirect immunofluorescence assay is positive for Bartonella henselae.

OTHER STUDIES:
A lymph node biopsy is obtained.
