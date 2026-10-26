# synthetic_hospital patient 1936

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-19 (Mon) | follow_up | Obstetrics & Gynecology |
| 1 | 2021-01-15 | 2026-05-11 (Mon) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "Occasional (1-2 glasses of wine per week)",
 "family_history": [
  "Maternal aunt with cervical dysplasia",
  "Paternal grandmother with breast cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "Cervical conization (loop electrosurgical excision procedure) 6 months ago"
 ],
 "chronic_conditions": [
  "Human papillomavirus infection"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Cervical stenosis", "Surveillance with repeat cervical cytology"]
Comorbidities: ["Human papillomavirus infection"]

## Encounter 0: source 2020-01-15, practice 2025-05-19, follow_up, Obstetrics & Gynecology

Chief complaint: Follow-up visit six months after cervical conization for CIN 3, reporting new onset of scant, irregular, and painful menstrual cycles.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up visit six months after cervical conization for CIN 3, reporting new onset of scant, irregular, and painful menstrual cycles.

HISTORY OF PRESENT ILLNESS:
After undergoing cervical conization six months ago for high-grade cervical intraepithelial neoplasia (CIN 3) diagnosed on colposcopy and biopsy. She reports a new onset of irregular menstrual cycles, which have become scant and sometimes painful since the procedure.

PAST MEDICAL HISTORY:
Active Problem List:
- Human papillomavirus infection

Her past medical history is significant for human papillomavirus (HPV) infection.

PAST SURGICAL HISTORY:
- Cervical conization (loop electrosurgical excision procedure) 6 months ago

MEDICATIONS:
Home Medications:
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with cervical dysplasia
- Paternal grandmother with breast cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine per week). Occupation: High school mathematics teacher.

VITAL SIGNS:
On physical examination, her vital signs are within normal limits.

PHYSICAL EXAMINATION:
A pelvic examination reveals a narrowed external cervical os.

## Encounter 1: source 2021-01-15, practice 2026-05-11, outpatient, Obstetrics & Gynecology

Chief complaint: Routine cervical cancer surveillance visit with no active complaints.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine cervical cancer surveillance visit with no active complaints.

HISTORY OF PRESENT ILLNESS:
She returns for surveillance after CIN 3 conization and has no complaints; her Pap smear was 3 years ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical stenosis (diagnosed 2020-01-15)
- Human papillomavirus infection

PAST SURGICAL HISTORY:
- Cervical conization (loop electrosurgical excision procedure) 6 months ago

MEDICATIONS:
Home Medications:
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal aunt with cervical dysplasia
- Paternal grandmother with breast cancer

SOCIAL HISTORY:
She is sexually active with one partner and uses condoms inconsistently.

PATHOLOGY:
A Pap smear is performed and cytology shows squamous intraepithelial lesion positive for human papillomavirus with low-grade dysplasia.

PLAN:
The patient is counseled on follow-up and HPV vaccination.
