# synthetic_hospital patient 1766

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 26 at first encounter, sex F, Hispanic, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-18 (Fri) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-06-15 | 2025-12-16 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Administrative assistant",
 "alcohol_use": "None (discontinued upon confirmation of pregnancy)",
 "family_history": [
  "Type 2 Diabetes Mellitus (mother)",
  "Hypertension (father)"
 ],
 "race_ethnicity": "Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "500 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus"
 ],
 "age_at_first_encounter": 26
}
```

Primary diagnoses: ["Asymptomatic bacteriuria in pregnancy"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-07-18, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at 16 weeks gestation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at 16 weeks gestation

HISTORY OF PRESENT ILLNESS:
She has no complaints and her pregnancy has been uneventful thus far.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus

Her medical history is significant for type 2 diabetes mellitus, which is well-controlled on metformin.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes metformin.

Home Medications:
- Prenatal vitamin 1 tablet daily

ALLERGIES:
She has no known drug allergies.

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (mother)
- Hypertension (father)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (discontinued upon confirmation of pregnancy). Occupation: Administrative assistant.

VITAL SIGNS:
Her blood pressure is 120/78 mmHg, heart rate is 72 beats per minute, and temperature is 98.6°F (37°C).

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
A urine dipstick reveals +1 leukocyte esterase and is negative for nitrites, protein, glucose, and ketones. A subsequent urine culture grows >100,000 colony-forming units/mL of Escherichia coli.

## Encounter 1: source 2020-06-15, practice 2025-12-16, ed, Emergency Medicine

Chief complaint: Fever, nausea, right-sided flank pain, and burning with urination

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, nausea, right-sided flank pain, and burning with urination

HISTORY OF PRESENT ILLNESS:
The patient is a 26-year-old pregnant female with Type 2 Diabetes Mellitus and a history of asymptomatic bacteriuria diagnosed during her prenatal care in January 2020. She is brought to the emergency department by her partner due to fever, nausea, right-sided flank pain, and burning with urination. The symptoms began gradually three days ago as urinary frequency and suprapubic pressure but have since worsened, now accompanied by chills and subjective fever. She has no vaginal bleeding or discharge and reports normal fetal movement. She denies any prior urinary tract infections and has not had any recent sick contacts. She attended early prenatal care but missed her last two visits.

PAST MEDICAL HISTORY:
Active Problem List:
- Asymptomatic bacteriuria in pregnancy (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Metformin 500 mg twice daily
- Prenatal vitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 Diabetes Mellitus (mother)
- Hypertension (father)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (discontinued upon confirmation of pregnancy). Occupation: Administrative assistant.

VITAL SIGNS:
Temperature is 38.8°C (101.8°F), pulse 110/min, and BP 108/70 mm Hg.

PHYSICAL EXAMINATION:
On physical examination, she appears flushed and mildly diaphoretic. Costovertebral angle tenderness is present on the right side. Abdominal exam reveals mild suprapubic discomfort, but no rebound or guarding.

LABORATORY DATA:
Urinalysis reveals leukocyte esterase, nitrites, and significant pyuria.

IMAGING:
A renal ultrasound shows right-sided hydronephrosis without calculi or structural abnormalities.

PLAN:
She is started on intravenous fluids and antibiotics.
