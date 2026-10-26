# synthetic_hospital patient 1887

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic Black, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-08-02 (Fri) | inpatient | Obstetrics & Gynecology |
| 1 | 2021-04-15 | 2025-10-19 (Sun) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None (pregnant)",
 "family_history": [
  "Mother: Preeclampsia requiring delivery at 36 weeks gestation",
  "Maternal aunt: Eclampsia during first pregnancy"
 ],
 "race_ethnicity": "Non-Hispanic Black",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin with DHA"
  },
  {
   "dose": "500 mg twice daily as needed for heartburn",
   "name": "Calcium carbonate"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Eclampsia", "Uncertain fetal presentation in active labor"]
Comorbidities: ["Term pregnancy"]

## Encounter 0: source 2020-01-15, practice 2024-08-02, inpatient, Obstetrics & Gynecology

Chief complaint: presents to the labor and delivery unit with regular contractions every 5 minutes for the past 2 hours.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
She presents to the labor and delivery unit with regular contractions every 5 minutes for the past 2 hours.

HISTORY OF PRESENT ILLNESS:
She reports that the contractions are moderately painful and have been increasing in intensity.

PAST MEDICAL HISTORY:
Her prenatal course was unremarkable, and she has no significant past medical history.

MEDICATIONS:
Home Medications:
- Prenatal vitamin with DHA 1 tablet daily
- Calcium carbonate 500 mg twice daily as needed for heartburn

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Preeclampsia requiring delivery at 36 weeks gestation
- Maternal aunt: Eclampsia during first pregnancy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (pregnant). Occupation: Elementary school teacher.

VITAL SIGNS:
On physical examination, her blood pressure is 120/78 mmHg, heart rate is 88 beats per minute, and temperature is 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
Abdominal examination reveals a gravid uterus consistent with gestational age. Fetal heart tones are heard in the lower quadrants, with a rate of 140 beats per minute. Digital cervical examination reveals the cervix to be 3 cm dilated and 80% effaced, but the presenting part is not clearly palpable.

ASSESSMENT AND PLAN:
The obstetrician is uncertain whether the fetus is in a cephalic or breech presentation.

## Encounter 1: source 2021-04-15, practice 2025-10-19, icu, Emergency Medicine

Chief complaint: is brought to the emergency department by her partner after a witnessed seizure at home.

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Marcus Okafor, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
She is brought to the emergency department by her partner after a witnessed seizure at home.

HISTORY OF PRESENT ILLNESS:
This 29-year-old female with a history of inpatient obstetric hospitalization in January 2020 for active labor complicated by uncertain fetal presentation presents to the emergency department with her partner. According to him, she was sitting on the couch when her body stiffened and began shaking for about a minute, followed by confusion and drowsiness. He reports that she had been complaining of headaches and occasional blurry vision over the past several days, but did not seek medical attention. She has had no recent illnesses, fevers, or trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Uncertain fetal presentation in active labor (diagnosed 2020-01-15)

Her first pregnancy was uncomplicated.

MEDICATIONS:
Home Medications:
- Prenatal vitamin with DHA 1 tablet daily
- Calcium carbonate 500 mg twice daily as needed for heartburn

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Preeclampsia requiring delivery at 36 weeks gestation
- Maternal aunt: Eclampsia during first pregnancy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (pregnant). Occupation: Elementary school teacher.

VITAL SIGNS:
Vital signs show a temperature of 36.7°C, heart rate 96/min, respiratory rate 18/min, and blood pressure of 162/106 mm Hg.

PHYSICAL EXAMINATION:
Upon arrival, the patient is postictal but arousable.
There is 2+ pitting edema in both lower extremities. Fetal heart monitoring reveals a reactive tracing.

LABORATORY DATA:
Urinalysis reveals 3+ protein. Her labs are pending.
