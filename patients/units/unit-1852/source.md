# synthetic_hospital patient 1852

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex F, White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-26 (Mon) | outpatient | Dermatology |
| 1 | 2020-07-15 | 2025-11-21 (Fri) | ed | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Server",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Hypertension",
  "Father: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Acute pyelonephritis", "Secondary syphilis"]
Comorbidities: ["Pregnancy"]

## Encounter 0: source 2020-01-15, practice 2025-05-26, outpatient, Dermatology

Chief complaint: evaluation of a rash that she first noticed approximately 3 weeks ago

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of a rash that she first noticed approximately 3 weeks ago

HISTORY OF PRESENT ILLNESS:
She reports that it began as a few faint spots on her chest and back but has since spread to include her arms, legs, palms, and soles. The rash is not itchy or painful and has not changed in appearance since it began. She denies fever, sore throat, fatigue, joint pain, or recent illnesses. She also denies any new medications, dietary changes, or use of herbal supplements. She denies any genital sores, vaginal discharge, or urinary symptoms currently.

PAST MEDICAL HISTORY:
She has not had prenatal care up to this point and is unsure of her blood type or past medical history. She has not received any vaccinations during this pregnancy. She recalls being treated for “a vaginal infection” over a year ago but does not know the specific diagnosis.

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypertension
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
The patient is sexually active with multiple male partners and reports inconsistent condom use.

REVIEW OF SYSTEMS:
On review of systems, she has not experienced vision changes, headaches, or neurologic symptoms.

VITAL SIGNS:
On physical examination, her temperature is 36.9°C (98.4°F), blood pressure is 112/72 mm Hg, and heart rate is 82/min.

PHYSICAL EXAMINATION:
She appears well and in no distress. Cutaneous exam reveals symmetric, faintly erythematous macules and papules across her chest, back, abdomen, arms, legs, palms, and soles. The lesions are non-scaly, non-vesicular, and non-blanching. There is no lymphadenopathy, hepatosplenomegaly, or mucosal involvement. Fetal heart tones are present and normal.

## Encounter 1: source 2020-07-15, practice 2025-11-21, ed, Obstetrics & Gynecology

Chief complaint: burning with urination and pelvic pressure over the past 4 days

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James O'Brien, MD
Department: Obstetrics & Gynecology
Visit Type: ed

CHIEF COMPLAINT:
Burning with urination and pelvic pressure over the past 4 days

HISTORY OF PRESENT ILLNESS:
This 24-year-old with a history of secondary syphilis diagnosed in January presents to her obstetrician for evaluation of burning with urination and pelvic pressure. She denies fever, flank pain, nausea, or vaginal discharge. Her pregnancy has been uncomplicated thus far. She has not had any prenatal urinary screening since her initial visit at 8 weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary syphilis (diagnosed 2020-01-15)

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypertension
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Server.

VITAL SIGNS:
Her temperature is 36.9°C (98.4°F), blood pressure is 108/72 mm Hg, and pulse is 76/min.

PHYSICAL EXAMINATION:
On physical exam, there is mild suprapubic tenderness without costovertebral angle tenderness. Uterine size is appropriate for gestational age, and fetal heart tones are reassuring.

LABORATORY DATA:
A clean-catch urine sample is obtained and shows cloudy urine. Urinalysis reveals positive nitrites, leukocyte esterase, and numerous white blood cells. No proteinuria is seen. A urine culture is sent.
