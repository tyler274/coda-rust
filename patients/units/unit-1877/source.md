# synthetic_hospital patient 1877

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-03-08 (Fri) | outpatient | Obstetrics & Gynecology |
| 1 | 2021-09-15 | 2025-10-23 (Thu) | inpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [],
 "insurance": "Private PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None (abstinent during pregnancy)",
 "family_history": [
  "Maternal aunt with acute fatty liver of pregnancy",
  "Paternal cousin with LCHAD deficiency",
  "Mother with atopic dermatitis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Acute fatty liver of pregnancy", "Polymorphic eruption of pregnancy (Pruritic urticarial papules and plaques of pregnancy)"]
Comorbidities: ["Pregnancy at 35 weeks gestation"]

## Encounter 0: source 2020-01-15, practice 2024-03-08, outpatient, Obstetrics & Gynecology

Chief complaint: 2-day history of an itchy rash beginning on the abdomen and progressively worsening

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
2-day history of an itchy rash beginning on the abdomen and progressively worsening

HISTORY OF PRESENT ILLNESS:
A 32-year-old woman at 35 weeks gestation presents to the outpatient clinic with a 2-day history of an itchy rash. She reports that the rash began on her abdomen and has been progressively worsening. She denies any new soaps, lotions, or dietary changes.

PAST MEDICAL HISTORY:
Her pregnancy has been uncomplicated, and she has no significant past medical history.

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Maternal aunt with acute fatty liver of pregnancy
- Paternal cousin with LCHAD deficiency
- Mother with atopic dermatitis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstinent during pregnancy). Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
On examination, you note erythematous, urticarial papules and plaques over her abdomen, particularly within the striae, but the umbilicus is spared. There are no lesions on her face, palms, or soles. fetal heart tones are reassuring.

## Encounter 1: source 2021-09-15, practice 2025-10-23, inpatient, Obstetrics & Gynecology

Chief complaint: 2-week history of severe itching particularly on the palms and soles without associated rash

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Marcus Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
2-week history of severe itching particularly on the palms and soles without associated rash

HISTORY OF PRESENT ILLNESS:
The patient returns following a prior pregnancy with rash in 2020. She reports that the itching is worse at night and is not accompanied by a rash. Her pregnancy has been uncomplicated until this point.

PAST MEDICAL HISTORY:
Active Problem List:
- Polymorphic eruption of pregnancy (Pruritic urticarial papules and plaques of pregnancy) (diagnosed 2020-01-15)

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily

ALLERGIES:
No known drug allergies (NKDA)

FAMILY HISTORY:
- Maternal aunt with acute fatty liver of pregnancy
- Paternal cousin with LCHAD deficiency
- Mother with atopic dermatitis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstinent during pregnancy). Occupation: Elementary school teacher.

REVIEW OF SYSTEMS:
She denies any nausea, vomiting, or change in bowel habits.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
On examination, there is no jaundice or hepatosplenomegaly

LABORATORY DATA:
Laboratory tests reveal the following:

- Total bile acids: 15 µmol/L
- Alanine aminotransferase (ALT): 45 U/L (normal 7-56 U/L)
- Aspartate aminotransferase (AST): 50 U/L (normal 10-40 U/L)
- Bilirubin: 0.8 mg/dL (normal 0.1-1.2 mg/dL)
- Alkaline phosphatase: 120 U/L (normal 30-120 U/L)
