# synthetic_hospital patient 1814

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex F, Non-Hispanic White, Private insurance through parent.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-22 (Thu) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-07-15 | 2025-11-08 (Sat) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-05-24 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance through parent",
 "occupation": "University student",
 "alcohol_use": "Occasional social use (1-2 drinks per month)",
 "family_history": [
  "Mother with polycystic ovary syndrome",
  "No family history of ectopic pregnancy or thromboembolic disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [
  "Irregular menstrual cycles"
 ],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Ruptured Ectopic Pregnancy", "Ovarian Torsion", "Tetanus prophylaxis"]
Comorbidities: ["Inconsistent condom use"]

## Encounter 0: source 2020-01-15, practice 2025-05-22, outpatient, Obstetrics & Gynecology

Chief complaint: presents to the clinic for a health maintenance examination

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to the clinic for a health maintenance examination

HISTORY OF PRESENT ILLNESS:
She has not received the HPV vaccine and inquires about whether she should receive it now. Her last menstrual period was 2 weeks ago, and she has no complaints of abnormal vaginal bleeding or discharge

PAST MEDICAL HISTORY:
Active Problem List:
- Irregular menstrual cycles

MEDICATIONS:
She takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with polycystic ovary syndrome
- No family history of ectopic pregnancy or thromboembolic disease

SOCIAL HISTORY:
She is sexually active with one partner and uses condoms inconsistently

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
The pelvic examination is unremarkable. She has no cervical lesions or tenderness on bimanual examination

OTHER STUDIES:
Her immunization record confirms that she has not received the HPV vaccine

## Encounter 1: source 2020-07-15, practice 2025-11-08, ed, Emergency Medicine

Chief complaint: presents to the emergency department with sudden onset of severe right lower quadrant pain that started 6 hours ago

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
She presents to the emergency department with sudden onset of severe right lower quadrant pain that started 6 hours ago

HISTORY OF PRESENT ILLNESS:
Returning six months after her last gynecology visit, the patient with irregular menses describes the pain as sharp and constant, denies any vaginal bleeding, discharge, or fever, and notes her last menstrual period was 2 weeks ago and was normal.

PAST MEDICAL HISTORY:
Active Problem List:
- Tetanus prophylaxis (diagnosed 2020-01-15)
- Irregular menstrual cycles

MEDICATIONS:
She takes no medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with polycystic ovary syndrome
- No family history of ectopic pregnancy or thromboembolic disease

SOCIAL HISTORY:
She is sexually active with one partner and uses condoms inconsistently.

VITAL SIGNS:
On physical examination, her vital signs are stable,

PHYSICAL EXAMINATION:
She appears to be in distress due to pain. Abdominal examination reveals tenderness and guarding in the right lower quadrant without rebound tenderness. A pelvic examination shows right adnexal tenderness.

LABORATORY DATA:
A urine pregnancy test is negative.

IMAGING:
Transvaginal ultrasound reveals a 5 cm right adnexal mass with peripheral follicles and absent color Doppler flow to the right ovary.

## Encounter 2: source 2021-01-15, practice 2026-05-24, ed, Emergency Medicine

Chief complaint: acute onset of severe lower abdominal pain

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of severe lower abdominal pain

HISTORY OF PRESENT ILLNESS:
The patient, who has irregular menstrual cycles, returns six months after emergency department treatment for ovarian torsion. She is sexually active with one partner and uses condoms inconsistently. She reports a history of irregular menses and has not had a menstrual period for the past 10 weeks. She denies nausea, vomiting, or vaginal bleeding.

PAST MEDICAL HISTORY:
Active Problem List:
- Tetanus prophylaxis (diagnosed 2020-01-15)
- Ovarian Torsion (diagnosed 2020-07-15)
- Irregular menstrual cycles

ALLERGIES:
She has no known drug allergies

FAMILY HISTORY:
- Mother with polycystic ovary syndrome
- No family history of ectopic pregnancy or thromboembolic disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: University student.

VITAL SIGNS:
On physical examination, her vital signs are stable

PHYSICAL EXAMINATION:
She appears to be in moderate distress. Abdominal examination reveals lower quadrant tenderness with rebound tenderness.

LABORATORY DATA:
A urine pregnancy test is positive.

IMAGING:
Transvaginal ultrasound shows a 3 cm complex adnexal mass on the right side with no intrauterine pregnancy.

ASSESSMENT AND PLAN:
The patient's mother arrives and is demanding to know what is happening with her daughter. The patient has not given consent to share her medical information.
