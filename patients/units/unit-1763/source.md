# synthetic_hospital patient 1763

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex F, White, non-Hispanic, Private insurance through university health plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-10 (Mon) | ed | Emergency Medicine |
| 1 | 2020-04-15 | 2025-05-03 (Sat) | ed | Emergency Medicine |
| 2 | 2021-04-15 | 2026-05-06 (Wed) | icu | Critical Care |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance through university health plan",
 "occupation": "University student",
 "alcohol_use": "Social, 1-2 drinks per month",
 "family_history": [
  "Mother: Hypothyroidism",
  "Father: Hypertension",
  "Sister: Menorrhagia"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Menorrhagia"
 ],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Acute kidney injury", "Toxic shock syndrome"]
Comorbidities: ["Acute kidney injury", "Dehydration"]

## Encounter 0: source 2020-01-15, practice 2025-02-10, ed, Emergency Medicine

Chief complaint: 2-day history of high fever, vomiting, and severe muscle aches

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of high fever, vomiting, and severe muscle aches

HISTORY OF PRESENT ILLNESS:
She presents to the emergency department with a 2-day history of high fever, vomiting, and severe muscle aches. She reports using super-absorbent tampons during her menstrual periods, including the current one.

PAST MEDICAL HISTORY:
Active Problem List:
- Menorrhagia

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Hypertension
- Sister: Menorrhagia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: University student.

VITAL SIGNS:
On examination, her temperature is 39.5°C (103.1°F), blood pressure is 85/50 mmHg, and heart rate is 120 beats per minute.

PHYSICAL EXAMINATION:
She appears ill and disoriented. A diffuse, blanching erythematous rash is noted over her trunk and extremities.

LABORATORY DATA:
Laboratory studies reveal:

- White blood cell count: 18,000/mm³
- Hemoglobin: 12.5 g/dL
- Platelet count: 100,000/mm³
- Creatinine: 1.8 mg/dL (baseline 0.9 mg/dL)
- Blood urea nitrogen: 30 mg/dL
- Aspartate aminotransferase (AST): 70 U/L
- Alanine aminotransferase (ALT): 65 U/L
- Prothrombin time (PT): 14 seconds (control 11 seconds)
- Partial thromboplastin time (PTT): 35 seconds (control 30 seconds)

## Encounter 1: source 2020-04-15, practice 2025-05-03, ed, Emergency Medicine

Chief complaint: Progressive fatigue, dizziness, and decreased appetite

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive fatigue, dizziness, and decreased appetite

HISTORY OF PRESENT ILLNESS:
This 22-year-old female with a history of menorrhagia and prior toxic shock syndrome presentation three months ago returns with progressive fatigue, dizziness, and decreased appetite over the past several days. She is brought to the emergency department by her roommate due to these symptoms. She reports having difficulty concentrating and feeling increasingly lightheaded when getting out of bed. On further questioning, she reluctantly admits to taking over-the-counter "water pills" almost daily for the past year to maintain a lean physique. She reports drinking minimal water each day and avoiding meals with salt. She denies vomiting, diarrhea, or recent illnesses. Her menstrual cycles are irregular, and she frequently experiences dry mouth and muscle cramps.

PAST MEDICAL HISTORY:
Active Problem List:
- Toxic shock syndrome (diagnosed 2020-01-15)
- Menorrhagia

PAST SURGICAL HISTORY:
- None

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Hypertension
- Sister: Menorrhagia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: University student.

VITAL SIGNS:
Blood pressure is 96/60 mm Hg and heart rate is 108/min.

PHYSICAL EXAMINATION:
On physical examination, she appears thin with sunken eyes, dry mucous membranes, and decreased skin turgor. Cardiopulmonary examination is normal. A Foley catheter is placed, and urine output is minimal.

## Encounter 2: source 2021-04-15, practice 2026-05-06, icu, Critical Care

Chief complaint: 2-day history of high fever, vomiting, and diarrhea with diffuse rash

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Aisha Patel, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
2-day history of high fever, vomiting, and diarrhea with diffuse rash

HISTORY OF PRESENT ILLNESS:
Returning following prior presentations in January and April 2020, she reports the onset of a diffuse, red, macular rash that started on her trunk and spread to her extremities. She has been using super-absorbent tampons for her heavy menstrual flow.

PAST MEDICAL HISTORY:
Active Problem List:
- Toxic shock syndrome (diagnosed 2020-01-15)
- Acute kidney injury (diagnosed 2020-04-15)
- Menorrhagia

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Hypertension
- Sister: Menorrhagia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks per month. Occupation: University student.

VITAL SIGNS:
On examination, her temperature is 39.5°C (103.1°F), blood pressure is 85/50 mmHg, pulse is 120/min, and respiratory rate is 22/min

PHYSICAL EXAMINATION:
She appears ill and disoriented. Her skin examination reveals a blanching erythematous rash that involves her palms and soles.

LABORATORY DATA:
Laboratory studies show:

- White blood cell count: 18,000/mm³ (Normal: 4,500-11,000/mm³)
- Hemoglobin: 12.5 g/dL (Normal: 12-16 g/dL)
- Platelets: 150,000/mm³ (Normal: 150,000-450,000/mm³)
- Blood urea nitrogen: 28 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 1.8 mg/dL (Normal: 0.6-1.1 mg/dL)
- Sodium: 130 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 5.5 mEq/L (Normal: 3.5-5.0 mEq/L)
- Aspartate aminotransferase (AST): 70 U/L (Normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 50 U/L (Normal: 7-56 U/L)
