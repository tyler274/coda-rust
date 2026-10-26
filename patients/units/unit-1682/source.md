# synthetic_hospital patient 1682

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Private commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-10 (Wed) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-06-15 | 2026-02-05 (Thu) | ed | Obstetrics & Gynecology |
| 2 | 2020-09-15 | 2026-05-05 (Tue) | ed | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "No alcohol use during pregnancy; occasional social use prior to conception",
 "family_history": [],
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
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Intrauterine fetal demise", "Placenta previa", "Severe Fetal Anemia due to Fetomaternal Hemorrhage"]
Comorbidities: ["Multiparity", "Pregnancy at 36 weeks gestation"]

## Encounter 0: source 2020-01-15, practice 2025-09-10, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit with occasional painless vaginal spotting over the past week

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit with occasional painless vaginal spotting over the past week

HISTORY OF PRESENT ILLNESS:
She reports feeling well but mentions occasional, painless vaginal spotting over the past week.
Her prenatal course has been unremarkable until now, with normal blood pressure and no gestational diabetes.
The patient inquires about her delivery options, expressing a desire for a vaginal birth if possible.

PAST MEDICAL HISTORY:
Her previous pregnancies were uncomplicated, resulting in vaginal deliveries at term.

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily

ALLERGIES:
NKDA

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use during pregnancy; occasional social use prior to conception. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her vital signs are stable

PHYSICAL EXAMINATION:
The fetal heart rate is regular. A pelvic examination is not performed due to her reported spotting.

IMAGING:
An ultrasound is ordered, which reveals a complete placenta previa.

## Encounter 1: source 2020-06-15, practice 2026-02-05, ed, Obstetrics & Gynecology

Chief complaint: Decreased fetal movement noted for the past 6 hours

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. Kwame Asante, MD
Department: Obstetrics & Gynecology
Visit Type: ed

CHIEF COMPLAINT:
Decreased fetal movement noted for the past 6 hours

PAST MEDICAL HISTORY:
Active Problem List:
- Placenta previa (diagnosed 2020-01-15)

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily

ALLERGIES:
NKDA

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use during pregnancy; occasional social use prior to conception. Occupation: Elementary school teacher.

## Encounter 2: source 2020-09-15, practice 2026-05-05, ed, Obstetrics & Gynecology

Chief complaint: Not feeling her baby move since the previous evening

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Priya Patel, MD
Department: Obstetrics & Gynecology
Visit Type: ed

CHIEF COMPLAINT:
Not feeling her baby move since the previous evening

HISTORY OF PRESENT ILLNESS:
The patient, previously evaluated in January for vaginal spotting and in June for decreased fetal movement, returns to the emergency department with a complaint of not feeling her baby move since the previous evening. She denies any vaginal bleeding, leakage of fluid, or abdominal pain. Her pregnancy has been uncomplicated, and she has had regular prenatal care.

PAST MEDICAL HISTORY:
Active Problem List:
- Placenta previa (diagnosed 2020-01-15)
- Severe Fetal Anemia due to Fetomaternal Hemorrhage (diagnosed 2020-06-15)

MEDICATIONS:
Her only medication is a prenatal vitamin

ALLERGIES:
NKDA

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use during pregnancy; occasional social use prior to conception. Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On examination, her abdomen is appropriately sized for gestational age, but on auscultation, no fetal heart tones are detected.

IMAGING:
A bedside ultrasound is performed, which shows no fetal cardiac activity.
