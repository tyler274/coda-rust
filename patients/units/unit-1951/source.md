# synthetic_hospital patient 1951

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, Non-Hispanic White, Employer-sponsored private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-10 (Mon) | outpatient | Dermatology |
| 1 | 2021-01-15 | 2026-02-16 (Mon) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored private PPO",
 "occupation": "Marketing coordinator",
 "alcohol_use": "Social (2-3 drinks per week)",
 "family_history": [
  "Mother: breast cancer, age 55",
  "Father: type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet PO daily",
   "name": "Ethinyl estradiol/norgestimate"
  }
 ],
 "surgical_history": [
  "Tonsillectomy, age 8"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Condylomata acuminata"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-02-10, outpatient, Dermatology

Chief complaint: Multiple, painless, flesh-colored growths in the perianal area

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Multiple, painless, flesh-colored growths in the perianal area

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 3-month history of multiple, painless, flesh-colored growths in the perianal area. She reports no bleeding but feels self-conscious about their appearance.

PAST MEDICAL HISTORY:
Her past medical history is unremarkable

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily

ALLERGIES:
She has no known drug allergies

FAMILY HISTORY:
- Mother: breast cancer, age 55
- Father: type 2 diabetes mellitus

SOCIAL HISTORY:
She has been in a monogamous relationship for the past year and has no prior history of sexually transmitted infections.

PHYSICAL EXAMINATION:
On physical examination, you note several 1-2 cm verrucous papules around the anus. There is no lymphadenopathy, and the remainder of the examination is normal.

PATHOLOGY:
A biopsy of one of the lesions is performed, and the pathology report indicates koilocytosis.

## Encounter 1: source 2021-01-15, practice 2026-02-16, outpatient, Obstetrics & Gynecology

Chief complaint: Evaluation of a growth noticed near her external genitalia

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of a growth noticed near her external genitalia

HISTORY OF PRESENT ILLNESS:
She returns with new external genitalia growth after prior perianal lesions in January 2020. She states that she first saw it a few weeks ago, and it seems to have slowly increased in size. She denies any associated pain, discharge, or itching.

PAST MEDICAL HISTORY:
Active Problem List:
- Condylomata acuminata (diagnosed 2020-01-15)

Her last Pap smear was within normal limits. She is otherwise healthy.

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: breast cancer, age 55
- Father: type 2 diabetes mellitus

SOCIAL HISTORY:
She is sexually active with one male partner and reports inconsistent condom use. She does not smoke

PHYSICAL EXAMINATION:
On physical examination, there is a soft, exophytic lesion near the perineum composed of multiple projections forming an irregular surface. There is no inguinal lymphadenopathy, and no additional lesions are seen on speculum examination.
