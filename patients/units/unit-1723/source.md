# synthetic_hospital patient 1723

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 23 at first encounter, sex F, Black or African American, Uninsured (pending Medicaid enrollment).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-23 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-04-15 | 2025-03-24 (Mon) | outpatient | Obstetrics & Gynecology |
| 2 | 2021-01-15 | 2025-12-21 (Sun) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Uninsured (pending Medicaid enrollment)",
 "occupation": "Part-time restaurant server",
 "alcohol_use": "Occasional (1-2 drinks on weekends)",
 "family_history": [
  "Mother with asthma",
  "Maternal uncle with atopic dermatitis"
 ],
 "race_ethnicity": "Black or African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "90 mcg, 2 puffs every 4-6 hours as needed for wheezing",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "220 mcg, 1 puff twice daily (adherence inconsistent)",
   "name": "Fluticasone propionate inhaler"
  },
  {
   "dose": "5 mg daily as needed for seasonal allergies",
   "name": "Levocetirizine"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Severe persistent asthma",
  "Allergic rhinitis"
 ],
 "age_at_first_encounter": 23
}
```

Primary diagnoses: ["Secondary syphilis", "Status asthmaticus with impending respiratory failure", "Neisseria gonorrhoeae infection (Gonococcal cervicitis)"]
Comorbidities: ["High risk sexual behavior", "History of recent pneumonia"]

## Encounter 0: source 2020-01-15, practice 2024-12-23, outpatient, Obstetrics & Gynecology

Chief complaint: 5-day history of dysuria, increased urinary frequency, and yellow vaginal discharge

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Aisha Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
5-day history of dysuria, increased urinary frequency, and yellow vaginal discharge

HISTORY OF PRESENT ILLNESS:
5-day history of dysuria, increased urinary frequency, and a yellow vaginal discharge

PAST MEDICAL HISTORY:
Active Problem List:
- Severe persistent asthma
- Allergic rhinitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing
- Fluticasone propionate inhaler 220 mcg, 1 puff twice daily (adherence inconsistent)
- Levocetirizine 5 mg daily as needed for seasonal allergies

ALLERGIES:
She has no known drug allergies

FAMILY HISTORY:
- Mother with asthma
- Maternal uncle with atopic dermatitis

SOCIAL HISTORY:
She reports having unprotected sexual intercourse with multiple partners in the past month.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
Pelvic examination reveals a purulent cervical discharge and cervical motion tenderness.

LABORATORY DATA:
A urine pregnancy test is negative. Nucleic acid amplification testing (NAAT) of the cervical discharge is positive for Neisseria gonorrhoeae. Additional laboratory results are as follows:

- HIV antigen/antibody combo test: Negative
- Rapid plasma reagin (RPR): Non-reactive
- Urinalysis: No evidence of urinary tract infection

## Encounter 1: source 2020-04-15, practice 2025-03-24, outpatient, Obstetrics & Gynecology

Chief complaint: Painless rash on chest, back, arms, legs, palms, and soles first noticed 3 weeks ago

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Aisha Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Painless rash on chest, back, arms, legs, palms, and soles first noticed 3 weeks ago

HISTORY OF PRESENT ILLNESS:
The patient with severe persistent asthma and allergic rhinitis returns to the Obstetrics & Gynecology clinic three months after treatment for Neisseria gonorrhoeae infection and presents with a new rash. She reports that it began as a few faint spots on her chest and back but has since spread to include her arms, legs, palms, and soles. The rash is not itchy or painful and has not changed in appearance since it began. She denies fever, sore throat, fatigue, joint pain, or recent illnesses. She also denies any new medications, dietary changes, or use of herbal supplements. She denies any genital sores, vaginal discharge, or urinary symptoms currently.

PAST MEDICAL HISTORY:
Active Problem List:
- Neisseria gonorrhoeae infection (Gonococcal cervicitis) (diagnosed 2020-01-15)
- Severe persistent asthma
- Allergic rhinitis

She has not had prenatal care up to this point and is unsure of her blood type or past medical history. She has not received any vaccinations during this pregnancy. She recalls being treated for “a vaginal infection” over a year ago but does not know the specific diagnosis.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing
- Fluticasone propionate inhaler 220 mcg, 1 puff twice daily (adherence inconsistent)
- Levocetirizine 5 mg daily as needed for seasonal allergies

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with asthma
- Maternal uncle with atopic dermatitis

SOCIAL HISTORY:
The patient is sexually active with multiple male partners and reports inconsistent condom use.

REVIEW OF SYSTEMS:
On review of systems, she has not experienced vision changes, headaches, or neurologic symptoms.

VITAL SIGNS:
On physical examination, her temperature is 36.9°C (98.4°F), blood pressure is 112/72 mm Hg, and heart rate is 82/min.

PHYSICAL EXAMINATION:
She appears well and in no distress. Cutaneous exam reveals symmetric, faintly erythematous macules and papules across her chest, back, abdomen, arms, legs, palms, and soles. The lesions are non-scaly, non-vesicular, and non-blanching. There is no lymphadenopathy, hepatosplenomegaly, or mucosal involvement. Fetal heart tones are present and normal.

## Encounter 2: source 2021-01-15, practice 2025-12-21, icu, Pulmonary & Critical Care

Chief complaint: Progressive difficulty breathing over 1 week despite increased albuterol inhaler use

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Chen, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Progressive difficulty breathing over 1 week despite increased albuterol inhaler use

HISTORY OF PRESENT ILLNESS:
Patient with severe persistent asthma, allergic rhinitis, and history of Neisseria gonorrhoeae and secondary syphilis reports progressive symptom worsening over the past week despite increased albuterol inhaler use.

PAST MEDICAL HISTORY:
Active Problem List:
- Neisseria gonorrhoeae infection (Gonococcal cervicitis) (diagnosed 2020-01-15)
- Secondary syphilis (diagnosed 2020-04-15)
- Severe persistent asthma
- Allergic rhinitis

She has a history of intubation for asthma exacerbation two years ago.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing
- Fluticasone propionate inhaler 220 mcg, 1 puff twice daily (adherence inconsistent)
- Levocetirizine 5 mg daily as needed for seasonal allergies

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with asthma
- Maternal uncle with atopic dermatitis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks on weekends). Occupation: Part-time restaurant server.

VITAL SIGNS:
Her respiratory rate is 28 breaths per minute. Oxygen saturation on room air is 89%.

PHYSICAL EXAMINATION:
On examination, she is using accessory muscles to breathe, and she is speaking in short sentences.

LABORATORY DATA:
Arterial blood gas shows pH 7.32, PaCO2 50 mmHg, PaO2 60 mmHg, and HCO3- 26 mEq/L.

OTHER STUDIES:
Her peak expiratory flow rate is 40% of her personal best.
