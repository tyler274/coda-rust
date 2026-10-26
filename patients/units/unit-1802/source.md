# synthetic_hospital patient 1802

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex F, Non-Hispanic White, University student health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-01 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-04-15 | 2025-11-28 (Fri) | ed | Emergency Medicine |
| 2 | 2020-08-15 | 2026-03-25 (Wed) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin"
 ],
 "insurance": "University student health insurance",
 "occupation": "Graduate student",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother: Hypertension",
  "Father: Type 2 diabetes mellitus",
  "Sister: Asthma"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Genital herpes simplex virus infection",
  "History of gonococcal cervical infection (treated)"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Chlamydial pharyngitis and pelvic inflammatory disease", "Viral cervicitis", "Severe Clostridioides difficile infection without toxic megacolon"]
Comorbidities: ["Neisseria gonorrhoeae infection (Gonococcal cervicitis)", "History of recent pneumonia"]

## Encounter 0: source 2020-01-15, practice 2025-09-01, outpatient, Obstetrics & Gynecology

Chief complaint: Intermittent postcoital bleeding and yellowish vaginal discharge for two weeks

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent postcoital bleeding and yellowish vaginal discharge for two weeks

HISTORY OF PRESENT ILLNESS:
A two-week history of intermittent postcoital bleeding and a yellowish vaginal discharge. She reports no pain during intercourse but mentions that the bleeding is causing her significant distress. She has been in a monogamous relationship for the past year and denies any history of sexually transmitted infections. Her last menstrual period was three weeks ago and was normal.

PAST MEDICAL HISTORY:
Active Problem List:
- Genital herpes simplex virus infection
- History of gonococcal cervical infection (treated)

PAST SURGICAL HISTORY:
- None

ALLERGIES:
Penicillin

FAMILY HISTORY:
- Mother: Hypertension
- Father: Type 2 diabetes mellitus
- Sister: Asthma

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per week). Occupation: Graduate student.

VITAL SIGNS:
Vital signs are within normal limits

PHYSICAL EXAMINATION:
Pelvic examination reveals a friable cervix with mucopurulent discharge. No cervical motion tenderness is noted.

LABORATORY DATA:
A urine pregnancy test is negative. Nucleic acid amplification testing (NAAT) of the cervical discharge is pending.

## Encounter 1: source 2020-04-15, practice 2025-11-28, ed, Emergency Medicine

Chief complaint: Sudden-onset severe lower abdominal pain and vaginal bleeding; last menstrual period 7 weeks ago

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Williams, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset severe lower abdominal pain and vaginal bleeding; last menstrual period 7 weeks ago

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department with a history of genital herpes simplex virus infection and treated gonococcal cervical infection, having previously presented in January with postcoital bleeding and yellowish vaginal discharge. She reports that her last menstrual period was 7 weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Viral cervicitis (diagnosed 2020-01-15)
- Genital herpes simplex virus infection
- History of gonococcal cervical infection (treated)

Her past medical history is notable for a gonococcal cervical infection treated 6 months ago with intramuscular ceftriaxone.

PAST SURGICAL HISTORY:
- None

ALLERGIES:
Penicillin

FAMILY HISTORY:
- Mother: Hypertension
- Father: Type 2 diabetes mellitus
- Sister: Asthma

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per week). Occupation: Graduate student.

VITAL SIGNS:
Hypotensive

PHYSICAL EXAMINATION:
On examination, she appears pale
Her abdomen is tender, especially in the right lower quadrant, with rebound and guarding.

LABORATORY DATA:
Urine β-hCG is positive.

IMAGING:
Transvaginal ultrasound shows no intrauterine pregnancy but reveals free fluid in the pelvis and a complex adnexal mass on the right.

## Encounter 2: source 2020-08-15, practice 2026-03-25, inpatient, Internal Medicine

Chief complaint: Two days of severe lower abdominal cramps and frequent bloody diarrhea following recent antibiotic course

ENCOUNTER NOTE
Date: 2020-08-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Two days of severe lower abdominal cramps and frequent bloody diarrhea following recent antibiotic course

HISTORY OF PRESENT ILLNESS:
This 24-year-old female with history of genital herpes simplex virus infection and treated gonococcal infection and pelvic inflammatory disease returns with gastrointestinal complaints. She reports experiencing approximately 6–7 episodes of watery diarrhea per day, which recently became streaked with blood. She also notes mild nausea but denies vomiting or fever. Her symptoms began about a week after completing a 10-day course of oral clindamycin for a dental abscess.

PAST MEDICAL HISTORY:
Active Problem List:
- Viral cervicitis (diagnosed 2020-01-15)
- Chlamydial pharyngitis and pelvic inflammatory disease (diagnosed 2020-04-15)
- Genital herpes simplex virus infection
- History of gonococcal cervical infection (treated)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She has not taken any other medications.

ALLERGIES:
Penicillin

FAMILY HISTORY:
- Mother: Hypertension
- Father: Type 2 diabetes mellitus
- Sister: Asthma

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per week). Occupation: Graduate student.

PHYSICAL EXAMINATION:
On exam, she appears uncomfortable but is afebrile. Her abdomen is soft but tender in the left lower quadrant with no rebound or guarding.

LABORATORY DATA:
Laboratory studies show a normal white blood cell count and normal renal function.

PLAN:
She is started on supportive care.
