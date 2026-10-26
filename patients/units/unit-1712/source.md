# synthetic_hospital patient 1712

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-21 (Sun) | inpatient | Obstetrics & Gynecology |
| 1 | 2020-10-15 | 2025-04-11 (Fri) | inpatient | Obstetrics & Gynecology |
| 2 | 2021-07-15 | 2026-01-21 (Wed) | inpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None during pregnancy; social use prior to conception",
 "family_history": [
  "Type 1 diabetes mellitus in father",
  "Type 2 diabetes mellitus in maternal grandmother",
  "Macrosomic infant (>4.5 kg) in prior pregnancy"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale with meals",
   "name": "Insulin lispro"
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
  "Type 1 Diabetes Mellitus",
  "History of Gestational Diabetes Mellitus"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Shoulder dystocia", "Brachial plexus birth injury", "Chorioamnionitis (Intra-amniotic infection)"]
Comorbidities: ["Personal History of Cholelithiasis"]

## Encounter 0: source 2020-01-15, practice 2024-07-21, inpatient, Obstetrics & Gynecology

Chief complaint: Regular uterine contractions every 5 minutes for 6 hours with low-grade fever and lower abdominal pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Regular uterine contractions every 5 minutes for 6 hours with low-grade fever and lower abdominal pain

HISTORY OF PRESENT ILLNESS:
She reports a low-grade fever and lower abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 Diabetes Mellitus
- History of Gestational Diabetes Mellitus

Her pregnancy has been uncomplicated until now.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Prenatal vitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 1 diabetes mellitus in father
- Type 2 diabetes mellitus in maternal grandmother
- Macrosomic infant (>4.5 kg) in prior pregnancy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None during pregnancy; social use prior to conception. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her temperature is 38.3°C (100.9°F), blood pressure is 120/75 mmHg, and heart rate is 88/min.

PHYSICAL EXAMINATION:
Vaginal examination reveals a cervix that is 3 cm dilated and 80% effaced. The amniotic fluid is cloudy with a foul odor.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 15,000/µL
- C-reactive protein: 25 mg/L
- Urinalysis: normal

OTHER STUDIES:
Fetal heart monitoring shows a baseline heart rate of 170/min with minimal variability and no decelerations.

## Encounter 1: source 2020-10-15, practice 2025-04-11, inpatient, Obstetrics & Gynecology

Chief complaint: Regular contractions every 3 minutes, progressing to second stage of labor with pushing

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. James Chen, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Regular contractions every 3 minutes, progressing to second stage of labor with pushing

HISTORY OF PRESENT ILLNESS:
The patient with Type 1 Diabetes Mellitus and prior chorioamnionitis returns in active labor. Two hours later, the cervix is fully dilated, and she begins to push.

PAST MEDICAL HISTORY:
Active Problem List:
- Chorioamnionitis (Intra-amniotic infection) (diagnosed 2020-01-15)
- Type 1 Diabetes Mellitus
- History of Gestational Diabetes Mellitus

Her pregnancy has been complicated by gestational diabetes, which has been diet-controlled. Her first child was delivered vaginally and weighed 4.5 kg at birth.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Prenatal vitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 1 diabetes mellitus in father
- Type 2 diabetes mellitus in maternal grandmother
- Macrosomic infant (>4.5 kg) in prior pregnancy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None during pregnancy; social use prior to conception. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On examination, her cervix is dilated to 8 cm, and the fetal heart tracing is reassuring.

ASSESSMENT AND PLAN:
After the fetal head delivers, the anterior shoulder cannot be delivered with gentle downward traction. The obstetrician notes that the fetal shoulder is impacted behind the pubic symphysis.

The patient's legs are immediately flexed back against her abdomen, and suprapubic pressure is applied without success.

## Encounter 2: source 2021-07-15, practice 2026-01-21, inpatient, Obstetrics & Gynecology

Chief complaint: Regular contractions every 5 minutes at 38 weeks gestation with prolonged second stage and fetal distress

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Patel, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Regular contractions every 5 minutes at 38 weeks gestation with prolonged second stage and fetal distress

HISTORY OF PRESENT ILLNESS:
The patient with Type 1 Diabetes Mellitus and history of chorioamnionitis and shoulder dystocia at 38 weeks of gestation presents to the labor and delivery unit. During labor, the patient has a prolonged second stage, and the fetal heart tracing shows signs of distress.

PAST MEDICAL HISTORY:
Active Problem List:
- Chorioamnionitis (Intra-amniotic infection) (diagnosed 2020-01-15)
- Shoulder dystocia (diagnosed 2020-10-15)
- Type 1 Diabetes Mellitus
- History of Gestational Diabetes Mellitus

She has a history of type 1 diabetes mellitus, which has been poorly controlled during pregnancy.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Prenatal vitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 1 diabetes mellitus in father
- Type 2 diabetes mellitus in maternal grandmother
- Macrosomic infant (>4.5 kg) in prior pregnancy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None during pregnancy; social use prior to conception. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On examination, the fetus is in a cephalic presentation,

LABORATORY DATA:
Her prenatal records indicate that her hemoglobin A1c levels have been consistently above 9%.

IMAGING:
Ultrasound estimates the fetal weight to be 4500 grams

ASSESSMENT AND PLAN:
The obstetrician suspects shoulder dystocia.
