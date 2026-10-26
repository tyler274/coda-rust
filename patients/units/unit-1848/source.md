# synthetic_hospital patient 1848

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, Non-Hispanic White, Aetna PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2026-01-24 (Sat) | inpatient | Obstetrics & Gynecology |
| 1 | 2020-01-15 | 2026-01-24 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Aetna PPO",
 "occupation": "Elementary School Teacher",
 "alcohol_use": "None",
 "family_history": [
  "Noncontributory"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Prolonged second stage of labor", "Pubic symphysis diastasis"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2026-01-24, inpatient, Obstetrics & Gynecology

Chief complaint: Admitted to the labor and delivery unit in active labor with prolonged second stage

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Admitted to the labor and delivery unit in active labor with prolonged second stage

HISTORY OF PRESENT ILLNESS:
She received an epidural for pain relief 5 hours ago. Her contractions are strong and occur every 2-3 minutes. The fetal heart tracing has been reassuring throughout labor. Despite adequate contractions and pushing for 4 hours, there has been no further descent of the fetal head, which remains at 0 station. The patient is becoming increasingly exhausted.

PAST MEDICAL HISTORY:
With an uncomplicated pregnancy

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Noncontributory

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary School Teacher.

REVIEW OF SYSTEMS:
She denies any headache, visual changes, or abdominal pain

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
The cervix is fully dilated. On examination, the fetal head is palpable at the level of the ischial spines.

## Encounter 1: source 2020-01-15, practice 2026-01-24, ed, Emergency Medicine

Chief complaint: Severe suprapubic pain radiating to groin

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Marcus Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe suprapubic pain radiating to groin

HISTORY OF PRESENT ILLNESS:
This 30-year-old female returns to the emergency department 2 days postpartum following a recent vaginal delivery with forceps assistance. She reports that the pain radiates to her groin and is exacerbated by walking or any movement that involves separating her legs. She had a prolonged second stage of labor and ultimately delivered a 4.5 kg baby vaginally with the assistance of forceps.

PAST MEDICAL HISTORY:
Active Problem List:
- Prolonged second stage of labor (diagnosed 2020-01-15)

She had no complications during pregnancy.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Noncontributory

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary School Teacher.

PHYSICAL EXAMINATION:
On physical examination, she appears uncomfortable while ambulating. There is tenderness over the pubic symphysis, and a waddling gait is noted. No abdominal guarding or rebound tenderness is present.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 12.5 g/dL
- White blood cell count: 9,000 /µL
- Platelets: 250,000 /µL
- C-reactive protein: 8 mg/L (normal <5 mg/L)

IMAGING:
A pelvic X-ray is ordered, which shows an increased gap between the pubic bones.
