# synthetic_hospital patient 1869

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 27 at first encounter, sex F, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-23 (Fri) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-04-15 | 2025-08-19 (Tue) | ed | Emergency Medicine |
| 2 | 2020-11-15 | 2026-03-19 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Restaurant server",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Mother: Type 2 diabetes mellitus, diagnosed age 45",
  "Father: Essential hypertension",
  "No family history of ectopic pregnancy or infertility"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 5 cigarettes per day",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 27
}
```

Primary diagnoses: ["Ruptured Ectopic Pregnancy", "Trichomoniasis"]
Comorbidities: ["Viral cervicitis"]

## Encounter 0: source 2020-01-15, practice 2025-05-23, outpatient, Obstetrics & Gynecology

Chief complaint: Vaginal discomfort and an unusual discharge for the past week

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Vaginal discomfort and an unusual discharge for the past week

HISTORY OF PRESENT ILLNESS:
She describes the discharge as yellow-green, frothy, and having an unpleasant odor. She denies any recent new sexual partners but mentions that her symptoms started a few days after her last menstrual period.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder

MEDICATIONS:
She is not currently taking any medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, diagnosed age 45
- Father: Essential hypertension
- No family history of ectopic pregnancy or infertility

SOCIAL HISTORY:
Smoking: Current smoker, 5 cigarettes per day. Alcohol: Social, 1-2 drinks per week. Occupation: Restaurant server.

PHYSICAL EXAMINATION:
On physical examination, the vaginal mucosa is erythematous, and the discharge is noted as described. A pelvic exam reveals cervical tenderness. The pH of the vaginal secretions is measured at 5.0. A wet mount microscopy of the vaginal discharge is performed, revealing motile, flagellated ovoid protozoa.

LABORATORY DATA:
Laboratory Values:
- Vaginal pH: 5.0
- Wet mount microscopy: Motile, flagellated ovoid protozoa observed

## Encounter 1: source 2020-04-15, practice 2025-08-19, ed, Emergency Medicine

Chief complaint: Acute-onset, severe left lower quadrant pain

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute-onset, severe left lower quadrant pain

HISTORY OF PRESENT ILLNESS:
27-year-old female with tobacco use disorder, evaluated three months ago for vaginal complaints, presents with a 6-hour history of acute-onset, severe left lower quadrant pain. She reports her last menstrual period was 5 weeks ago with regular 28-day cycle.

PAST MEDICAL HISTORY:
Active Problem List:
- Trichomoniasis (diagnosed 2020-01-15)
- Tobacco use disorder

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, diagnosed age 45
- Father: Essential hypertension
- No family history of ectopic pregnancy or infertility

SOCIAL HISTORY:
She is sexually active with one partner and uses condoms inconsistently.

VITAL SIGNS:
Her blood pressure is 110/70 mmHg, heart rate is 98 bpm, and temperature is 37.2°C (99.0°F)

PHYSICAL EXAMINATION:
Pelvic examination reveals left adnexal tenderness.

LABORATORY DATA:
A urine pregnancy test is positive. Serum β-hCG is measured at 1200 IU/L.

IMAGING:
Transvaginal ultrasound shows no intrauterine pregnancy or adnexal masses. There is no free fluid in the pelvis.

## Encounter 2: source 2020-11-15, practice 2026-03-19, ed, Emergency Medicine

Chief complaint: Lower abdominal pain and vaginal discharge

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. Jennifer Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Lower abdominal pain and vaginal discharge

HISTORY OF PRESENT ILLNESS:
The patient is a 27-year-old female with tobacco use disorder, treated for trichomoniasis in January 2020, and with a history of ruptured ectopic pregnancy in April 2020, who presents with lower abdominal pain and vaginal discharge that started five days ago. The pain is described as bilateral, dull, and worsens with movement or sexual activity. She also reports subjective fevers and chills but has not taken her temperature. Her last menstrual period was two weeks ago and was normal. She denies urinary frequency, urgency, or dysuria. The vaginal discharge is described as yellow and malodorous. She returns 3 months later for a follow-up visit and reports that her symptoms resolved completely with treatment. She asks if there are any long-term complications she should be aware of.

PAST MEDICAL HISTORY:
Active Problem List:
- Trichomoniasis (diagnosed 2020-01-15)
- Ruptured Ectopic Pregnancy (diagnosed 2020-04-15)
- Tobacco use disorder

MEDICATIONS:
She is not on any medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, diagnosed age 45
- Father: Essential hypertension
- No family history of ectopic pregnancy or infertility

SOCIAL HISTORY:
She is sexually active with multiple partners and inconsistently uses condoms. She does not use any form of long-term contraception.

VITAL SIGNS:
On physical exam, her temperature is 38.3°C (100.9°F), blood pressure is 112/70 mmHg, and heart rate is 102/min.

PHYSICAL EXAMINATION:
Abdominal exam reveals lower abdominal tenderness without rebound or guarding. Pelvic exam shows mucopurulent cervical discharge, cervical motion tenderness, and bilateral adnexal tenderness. Her vital signs and pelvic exam are normal at this time.

LABORATORY DATA:
A urine pregnancy test is negative. Wet mount reveals abundant white blood cells. Cervical nucleic acid amplification tests are pending.

ASSESSMENT AND PLAN:
She is diagnosed clinically

Treated empirically with ceftriaxone and doxycycline.
