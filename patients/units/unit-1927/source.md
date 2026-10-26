# synthetic_hospital patient 1927

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex F, Ashkenazi Jewish, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-09 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2021-07-15 | 2026-06-15 (Mon) | outpatient | Dermatology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Software engineer",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Breast cancer diagnosed age 42, ovarian cancer diagnosed age 51",
  "Maternal aunt: Breast cancer diagnosed age 39",
  "Maternal grandmother: Breast cancer diagnosed age 44",
  "Father: Hypertension"
 ],
 "race_ethnicity": "Ashkenazi Jewish",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "35 mcg/0.25 mg oral tablet once daily",
   "name": "Ethinyl estradiol/norgestimate"
  },
  {
   "dose": "2000 IU oral daily",
   "name": "Cholecalciferol"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 8)",
  "Wisdom teeth extraction (age 21)"
 ],
 "chronic_conditions": [
  "Hereditary breast and ovarian cancer syndrome (BRCA1 pathogenic variant)",
  "Fair skin phenotype (Fitzpatrick type II)"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Hereditary breast and ovarian cancer syndrome", "Nodular melanoma"]
Comorbidities: ["Family history of breast cancer"]

## Encounter 0: source 2020-01-15, practice 2024-12-09, outpatient, Obstetrics & Gynecology

Chief complaint: Discovered a firm mass in the upper outer quadrant of the right breast during self-examination

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Discovered a firm mass in the upper outer quadrant of the right breast during self-examination

HISTORY OF PRESENT ILLNESS:
She has no symptoms but recently discovered a firm mass in the upper outer quadrant of her right breast during self-examination.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary breast and ovarian cancer syndrome (BRCA1 pathogenic variant)
- Fair skin phenotype (Fitzpatrick type II)

PAST SURGICAL HISTORY:
She has no prior surgeries

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 35 mcg/0.25 mg oral tablet once daily
- Cholecalciferol 2000 IU oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her mother had ovarian cancer at age 49, and her older sister was diagnosed with bilateral breast cancer in her early 40s.

SOCIAL HISTORY:
The patient does not smoke or drink. Menses began at age 11, and she has not been pregnant.

PHYSICAL EXAMINATION:
Physical exam confirms a 2.5-cm, nonmobile lesion without skin changes or nipple discharge. No lymphadenopathy is noted, and the remainder of the exam is unremarkable.

IMAGING:
A mammogram reveals a suspicious irregular mass

OTHER STUDIES:
Biopsy is pending

PLAN:
Genetic testing is initiated based on her personal and family history.

## Encounter 1: source 2021-07-15, practice 2026-06-15, outpatient, Dermatology

Chief complaint: Concern about a changing mole on her back

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Kwame Asante, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Concern about a changing mole on her back

HISTORY OF PRESENT ILLNESS:
The patient with a BRCA1 variant evaluated for a breast mass in 2020 reports that a mole on her back, present for many years, has recently changed in appearance. She denies any pain or itching in the area.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary breast and ovarian cancer syndrome (diagnosed 2020-01-15)
- Hereditary breast and ovarian cancer syndrome (BRCA1 pathogenic variant)
- Fair skin phenotype (Fitzpatrick type II)

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)
- Wisdom teeth extraction (age 21)

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 35 mcg/0.25 mg oral tablet once daily
- Cholecalciferol 2000 IU oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is significant for a father with melanoma.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Software engineer.

PHYSICAL EXAMINATION:
On physical examination, you note a 1 cm pigmented lesion with an asymmetric shape, irregular borders, and variable colors ranging from brown to black. The lesion is located on her upper back, a region she indicates has been sun-exposed during her vacations. Dermoscopy shows irregular streaks and a blue-white veil. No lymphadenopathy is palpable. The rest of the skin examination is unremarkable.
