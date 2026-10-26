# synthetic_hospital patient 1686

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-09-15 (Sun) | ed | Emergency Medicine |
| 1 | 2021-01-15 | 2025-09-07 (Sun) | inpatient | Obstetrics & Gynecology |
| 2 | 2021-10-15 | 2026-05-31 (Sun) | inpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin (rash)",
  "Latex (contact dermatitis)"
 ],
 "insurance": "Private PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 glasses of wine per month)",
 "family_history": [
  "Mother: Postpartum hemorrhage",
  "Mother: Type 2 diabetes mellitus",
  "Father: Essential hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "75 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "325 mg PO daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 7)",
  "Episiotomy repair (age 24)"
 ],
 "chronic_conditions": [
  "Iron deficiency anemia",
  "Hypothyroidism"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Postpartum hemorrhage", "Vaginal laceration causing postpartum hemorrhage"]
Comorbidities: ["Acute posthemorrhagic anemia", "Hemorrhagic shock"]

## Encounter 0: source 2020-01-15, practice 2024-09-15, ed, Emergency Medicine

Chief complaint: She is experiencing persistent vaginal bleeding.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
She is experiencing persistent vaginal bleeding.

HISTORY OF PRESENT ILLNESS:
She is brought to the emergency department 30 minutes after a spontaneous vaginal delivery at home. Her prenatal course was uncomplicated, and she received routine prenatal care.

PAST MEDICAL HISTORY:
Active Problem List:
- Iron deficiency anemia
- Hypothyroidism

Her first pregnancy was uneventful, and she had a normal spontaneous vaginal delivery.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)
- Episiotomy repair (age 24)

MEDICATIONS:
Home Medications:
- Levothyroxine 75 mcg PO daily
- Ferrous sulfate 325 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
Penicillin (rash), Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Postpartum hemorrhage
- Mother: Type 2 diabetes mellitus
- Father: Essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure is 90/60 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
She appears pale and diaphoretic. The uterus is boggy on palpation, and there is a steady flow of blood from the vagina.

LABORATORY DATA:
Her hemoglobin level is 8 g/dL, and her platelet count is 250,000/µL. Coagulation profile is within normal limits.

PLAN:
Bimanual uterine massage is initiated, and she receives 10 units of intramuscular oxytocin. Despite these measures, the bleeding continues.

## Encounter 1: source 2021-01-15, practice 2025-09-07, inpatient, Obstetrics & Gynecology

Chief complaint: in active labor

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Liu, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
In active labor

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old female with iron deficiency anemia, hypothyroidism, and a history of postpartum hemorrhage in January 2020 who presents in active labor and opted for a trial of labor after cesarean (TOLAC). After several hours, she successfully delivers a healthy male infant vaginally with the assistance of forceps. The placenta is delivered intact, and the estimated blood loss is 300 mL. However, 30 minutes post-delivery, the patient complains of persistent vaginal bleeding and a feeling of faintness.

PAST MEDICAL HISTORY:
Active Problem List:
- Postpartum hemorrhage (diagnosed 2020-01-15)
- Iron deficiency anemia
- Hypothyroidism

PAST SURGICAL HISTORY:
She has a history of one previous cesarean section

MEDICATIONS:
Home Medications:
- Levothyroxine 75 mcg PO daily
- Ferrous sulfate 325 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
Penicillin (rash), Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Postpartum hemorrhage
- Mother: Type 2 diabetes mellitus
- Father: Essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Blood pressure is 90/60 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
The uterus is firm on palpation. On examination, there is continuous bleeding from the vagina. The cervix is visualized and appears intact. A speculum examination reveals a deep laceration in the upper vaginal wall.

LABORATORY DATA:
Hemoglobin: 9 g/dL (decreased from 12 g/dL at admission)
Hematocrit: 27% (decreased from 36% at admission)
Platelets: 250,000/mm³
Prothrombin time (PT): 12 seconds (normal)
Partial thromboplastin time (PTT): 30 seconds (normal)

## Encounter 2: source 2021-10-15, practice 2026-05-31, inpatient, Obstetrics & Gynecology

Chief complaint: feeling lightheaded and progressively weak

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. Maria Gonzalez, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Feeling lightheaded and progressively weak

HISTORY OF PRESENT ILLNESS:
Patient with anemia and prior PPH returns. She had an uncomplicated vaginal delivery but required manual removal of a retained placenta.

PAST MEDICAL HISTORY:
Active Problem List:
- Postpartum hemorrhage (diagnosed 2020-01-15)
- Vaginal laceration causing postpartum hemorrhage (diagnosed 2021-01-15)
- Iron deficiency anemia
- Hypothyroidism

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)
- Episiotomy repair (age 24)

MEDICATIONS:
Home Medications:
- Levothyroxine 75 mcg PO daily
- Ferrous sulfate 325 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
Penicillin (rash), Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Postpartum hemorrhage
- Mother: Type 2 diabetes mellitus
- Father: Essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs on admission were blood pressure 90/60 mmHg, heart rate 120 bpm, respiratory rate 22 breaths/min, and temperature 37.1°C.
Despite aggressive fluid resuscitation, her blood pressure remains at 85/55 mmHg and her heart rate is now 128 bpm.

PHYSICAL EXAMINATION:
On examination, her abdomen is soft with minimal tenderness. There is no vaginal bleeding noted beyond normal lochia.

LABORATORY DATA:
Her hemoglobin has dropped from 12 g/dL pre-delivery to 7 g/dL. Her coagulation profile is within normal limits.

IMAGING:
A focused assessment with sonography for trauma (FAST) exam is negative for free fluid in the abdomen.
