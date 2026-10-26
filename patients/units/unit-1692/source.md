# synthetic_hospital patient 1692

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic Black, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-02 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-03-15 | 2025-02-05 (Wed) | ed | Emergency Medicine |
| 2 | 2021-03-15 | 2026-01-29 (Thu) | inpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother with preeclampsia during pregnancy",
  "Maternal grandmother with essential hypertension",
  "Paternal aunt with systemic lupus erythematosus"
 ],
 "race_ethnicity": "Non-Hispanic Black",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Cesarean section (age 29)",
  "Tonsillectomy (childhood)"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Severe preeclampsia requiring immediate delivery", "Nonreactive nonstress test"]
Comorbidities: ["Severe preeclampsia without HELLP features", "Decreased fetal movements", "Multiparity"]

## Encounter 0: source 2020-01-15, practice 2024-12-02, outpatient, Obstetrics & Gynecology

Chief complaint: presents to the obstetrics clinic for a routine prenatal visit

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to the obstetrics clinic for a routine prenatal visit

HISTORY OF PRESENT ILLNESS:
She reports that her pregnancy has been uncomplicated, but she has noticed decreased fetal movements since the previous day. The patient remains concerned about the decreased fetal movements.

PAST MEDICAL HISTORY:
Her past medical history is unremarkable, and she has had adequate prenatal care.

PAST SURGICAL HISTORY:
- Cesarean section (age 29)
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with preeclampsia during pregnancy
- Maternal grandmother with essential hypertension
- Paternal aunt with systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
Blood pressure is 120/78 mmHg, heart rate is 72 beats per minute, and fetal heart rate is 140 beats per minute.

OTHER STUDIES:
A nonstress test (NST) is performed, which shows minimal variability with no accelerations and no decelerations over 40 minutes.

## Encounter 1: source 2020-03-15, practice 2025-02-05, ed, Emergency Medicine

Chief complaint: presents to the emergency department due to 2 days of persistent nausea, general malaise, and dull right upper quadrant pain that sometimes radiates to her back

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
She presents to the emergency department due to 2 days of persistent nausea, general malaise, and dull right upper quadrant pain that sometimes radiates to her back

HISTORY OF PRESENT ILLNESS:
The 32-year-old patient presents to the emergency department following her January 15th prenatal visit notable for a nonreactive nonstress test. She reports poor oral intake and unusual fatigue. Over the past day, she also noted decreased fetal movement. She denies contractions, vaginal bleeding, or fluid leakage.

PAST MEDICAL HISTORY:
Active Problem List:
- Nonreactive nonstress test (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Cesarean section (age 29)
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with preeclampsia during pregnancy
- Maternal grandmother with essential hypertension
- Paternal aunt with systemic lupus erythematosus

SOCIAL HISTORY:
She does not use tobacco, alcohol, or illicit drugs.

VITAL SIGNS:
On physical exam, her blood pressure is 150/95 mm Hg, temperature is 98.6°F (37°C), and heart rate is 92/min.

PHYSICAL EXAMINATION:
The fundal height corresponds to gestational age. Cardiopulmonary exam is normal. Abdominal exam reveals mild right upper quadrant and epigastric tenderness without rebound or guarding. Neurologic exam is nonfocal, and she has no hyperreflexia. Fetal heart rate tracing shows a baseline of 140/min with reduced variability but no decelerations.

LABORATORY DATA:
A urinalysis obtained in triage reveals 2+ protein. Routine laboratory workup is sent, and a peripheral smear is ordered due to concerns raised by the initial hemogram.

## Encounter 2: source 2021-03-15, practice 2026-01-29, inpatient, Obstetrics & Gynecology

Chief complaint: 24-hour history of severe right upper quadrant pain and malaise

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Sarah Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
24-hour history of severe right upper quadrant pain and malaise

HISTORY OF PRESENT ILLNESS:
She has a history of severe preeclampsia in March 2020. She reports constant pain radiating to her right shoulder.

PAST MEDICAL HISTORY:
Active Problem List:
- Nonreactive nonstress test (diagnosed 2020-01-15)
- Severe preeclampsia requiring immediate delivery (diagnosed 2020-03-15)

She has a history of preeclampsia during her first pregnancy.

PAST SURGICAL HISTORY:
- Cesarean section (age 29)
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with preeclampsia during pregnancy
- Maternal grandmother with essential hypertension
- Paternal aunt with systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure is 160/100 mmHg, and her pulse is 98 beats per minute.

PHYSICAL EXAMINATION:
Physical examination reveals right upper quadrant tenderness without rebound or guarding.

LABORATORY DATA:
Laboratory studies are as follows:

- Hemoglobin: 9.8 g/dL (Normal: 12-16 g/dL)
- Platelet count: 55,000/mm³ (Normal: 150,000-400,000/mm³)
- AST: 120 U/L (Normal: 10-40 U/L)
- ALT: 150 U/L (Normal: 7-56 U/L)
- LDH: 600 U/L (Normal: 140-280 U/L)
- Bilirubin, total: 2.2 mg/dL (Normal: 0.1-1.2 mg/dL)
- Peripheral smear: schistocytes present

Urinalysis shows 2+ proteinuria.

OTHER STUDIES:
The fetal heart rate tracing is reassuring.
