# synthetic_hospital patient 1750

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 23 at first encounter, sex F, White, non-Hispanic, Private insurance (parent's employer plan).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-08-06 (Wed) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-02-15 | 2025-09-14 (Sun) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2026-02-12 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance (parent's employer plan)",
 "occupation": "Graduate student",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother: Hypothyroidism, diagnosed age 45",
  "Father: Hypertension, diagnosed age 50",
  "No family history of autoimmune disorders or recurrent serious infections"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 23
}
```

Primary diagnoses: ["Gonococcal pharyngitis and pelvic inflammatory disease", "Disseminated gonococcal infection", "Fitz-Hugh-Curtis syndrome"]
Comorbidities: ["Gonococcal pharyngitis and pelvic inflammatory disease", "High risk sexual behavior"]

## Encounter 0: source 2020-01-15, practice 2025-08-06, outpatient, Obstetrics & Gynecology

Chief complaint: Fever, chills, and lower abdominal pain with vaginal discharge and dyspareunia

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Fever, chills, and lower abdominal pain with vaginal discharge and dyspareunia

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with fever, chills, and lower abdominal pain for the past 3 days. She also reports vaginal discharge and pain with intercourse. Her last menstrual period was 2 weeks ago.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism, diagnosed age 45
- Father: Hypertension, diagnosed age 50
- No family history of autoimmune disorders or recurrent serious infections

SOCIAL HISTORY:
She has had multiple sexual partners in the past year and does not consistently use barrier protection.

PHYSICAL EXAMINATION:
On physical exam, she is febrile to 101.8°F (38.8°C), and there is cervical motion tenderness, adnexal tenderness, and purulent cervical discharge.
A pustule is noted on her left finger.

LABORATORY DATA:
A urine pregnancy test is negative.

PLAN:
She is treated with intramuscular ceftriaxone and oral doxycycline.

## Encounter 1: source 2020-02-15, practice 2025-09-14, ed, Emergency Medicine

Chief complaint: Sharp right upper quadrant pain worsening with inspiration, low-grade fever, and persistent lower abdominal cramping

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sharp right upper quadrant pain worsening with inspiration, low-grade fever, and persistent lower abdominal cramping

HISTORY OF PRESENT ILLNESS:
The patient presents to the emergency department one month after her prior obstetrics and gynecology evaluation for lower abdominal pain and vaginal discharge. She describes the pain as sharp and constant, and it worsens with deep breathing. She also reports a low-grade fever and a dull, crampy lower abdominal pain for the past week. Her last menstrual period was 3 weeks ago and was normal.

PAST MEDICAL HISTORY:
Active Problem List:
- Gonococcal pharyngitis and pelvic inflammatory disease (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism, diagnosed age 45
- Father: Hypertension, diagnosed age 50
- No family history of autoimmune disorders or recurrent serious infections

SOCIAL HISTORY:
She is sexually active with multiple partners and does not consistently use barrier contraception.

VITAL SIGNS:
On examination, her temperature is 38.2°C (100.8°F), heart rate is 110 beats per minute, and blood pressure is 125/75 mmHg.

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness in the RUQ without rebound or guarding, and there is cervical motion tenderness on the pelvic exam.

LABORATORY DATA:
- White blood cell count: 12,000/mm³
- Hemoglobin: 13.2 g/dL
- Platelets: 250,000/mm³
- AST: 70 U/L
- ALT: 85 U/L
- Total bilirubin: 1.2 mg/dL
- Alkaline phosphatase: 120 U/L

IMAGING:
Transvaginal ultrasound shows no evidence of ectopic pregnancy or ovarian torsion.

OTHER STUDIES:
A urine pregnancy test is negative.

## Encounter 2: source 2020-07-15, practice 2026-02-12, ed, Emergency Medicine

Chief complaint: High fever, migratory joint pain involving wrists and knees, and new pustular skin lesions on trunk and arms

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
High fever, migratory joint pain involving wrists and knees, and new pustular skin lesions on trunk and arms

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department following recent encounters for lower abdominal pain and right upper quadrant pain worsening with inspiration, now presenting with new concerns. She reports pain and swelling in her wrists, knees, and ankles, which has made it difficult for her to walk. She also mentions the recent appearance of small, red, pustular lesions on her trunk and arms.

PAST MEDICAL HISTORY:
Active Problem List:
- Gonococcal pharyngitis and pelvic inflammatory disease (diagnosed 2020-01-15)
- Fitz-Hugh-Curtis syndrome (diagnosed 2020-02-15)

MEDICATIONS:
She takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism, diagnosed age 45
- Father: Hypertension, diagnosed age 50
- No family history of autoimmune disorders or recurrent serious infections

SOCIAL HISTORY:
She is sexually active with multiple partners and inconsistently uses condoms.

VITAL SIGNS:
On examination, her temperature is 39.2°C (102.5°F), heart rate is 110/min, and blood pressure is 120/75 mmHg.

PHYSICAL EXAMINATION:
Examination of the skin reveals scattered pustules with an erythematous base on the trunk and extremities. Musculoskeletal examination shows tenderness and swelling of the involved joints with pain on passive movement.

LABORATORY DATA:
Synovial fluid analysis from her knee shows a white blood cell count of 20,000/µL with 90% neutrophils. Gram stain is negative, and cultures are pending.
