# synthetic_hospital patient 1690

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Hispanic, Commercial HMO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-01 (Thu) | inpatient | Obstetrics & Gynecology |
| 1 | 2020-01-15 | 2025-05-12 (Mon) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-05-03 (Sun) | inpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin (anaphylaxis)",
  "Latex (contact dermatitis)"
 ],
 "insurance": "Commercial HMO",
 "occupation": "Marketing coordinator",
 "alcohol_use": "None (abstinent during reproductive years)",
 "family_history": [
  "Maternal history of preterm premature rupture of membranes",
  "Paternal type 2 diabetes",
  "Sister with chorioamnionitis in prior pregnancy"
 ],
 "race_ethnicity": "Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "88 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin with iron"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "100 mg PO PRN",
   "name": "Docusate sodium"
  }
 ],
 "surgical_history": [
  "Laparoscopic appendectomy (age 16)",
  "Wisdom teeth extraction"
 ],
 "chronic_conditions": [
  "Generalized anxiety disorder",
  "Hypothyroidism",
  "Iron deficiency anemia"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Preterm premature rupture of membranes (PPROM)", "Term infant", "Intra-amniotic infection (Chorioamnionitis)"]
Comorbidities: ["Term pregnancy", "Multiparity"]

## Encounter 0: source 2020-01-15, practice 2025-05-01, inpatient, Obstetrics & Gynecology

Chief complaint: 6-hour history of intermittent clear vaginal fluid leakage

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
6-hour history of intermittent clear vaginal fluid leakage

HISTORY OF PRESENT ILLNESS:
She denies any contractions, vaginal bleeding, or decrease in fetal movements. Her pregnancy has been uncomplicated, and prenatal care has been adequate with normal growth scans.

PAST MEDICAL HISTORY:
Active Problem List:
- Generalized anxiety disorder
- Hypothyroidism
- Iron deficiency anemia

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 16)
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Levothyroxine 88 mcg PO daily
- Prenatal vitamin with iron 1 tablet PO daily
- Sertraline 50 mg PO daily
- Docusate sodium 100 mg PO PRN

ALLERGIES:
Penicillin (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Maternal history of preterm premature rupture of membranes
- Paternal type 2 diabetes
- Sister with chorioamnionitis in prior pregnancy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstinent during reproductive years). Occupation: Marketing coordinator.

PHYSICAL EXAMINATION:
On examination, her vital signs are stable, and the uterus is non-tender with a fundal height appropriate for gestational age. Speculum examination confirms pooling of clear fluid in the vaginal vault.

IMAGING:
An ultrasound is performed, which reveals an amniotic fluid index of 3 cm. The fetus is in a cephalic presentation with normal biophysical profile.

OTHER STUDIES:
The nitrazine test is positive, and microscopic examination of the fluid shows ferning.

## Encounter 1: source 2020-01-15, practice 2025-05-12, ed, Emergency Medicine

Chief complaint: Fever and lower abdominal pain for 6 hours

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever and lower abdominal pain for 6 hours

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old female with a history of generalized anxiety disorder, hypothyroidism, and iron deficiency anemia who was evaluated earlier today for preterm premature rupture of membranes. She returns to the emergency department with a 6-hour history of fever and lower abdominal pain. She reports that her membranes ruptured 18 hours ago. Her pregnancy has been uncomplicated until now, and she has received appropriate prenatal care.

PAST MEDICAL HISTORY:
Active Problem List:
- Preterm premature rupture of membranes (PPROM) (diagnosed 2020-01-15)
- Generalized anxiety disorder
- Hypothyroidism
- Iron deficiency anemia

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 16)
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Levothyroxine 88 mcg PO daily
- Prenatal vitamin with iron 1 tablet PO daily
- Sertraline 50 mg PO daily
- Docusate sodium 100 mg PO PRN

ALLERGIES:
Penicillin (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Maternal history of preterm premature rupture of membranes
- Paternal type 2 diabetes
- Sister with chorioamnionitis in prior pregnancy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstinent during reproductive years). Occupation: Marketing coordinator.

VITAL SIGNS:
On examination, her temperature is 38.7°C (101.7°F), heart rate is 110 bpm, and blood pressure is 120/75 mmHg.

PHYSICAL EXAMINATION:
Fetal heart rate monitoring shows a baseline of 170 bpm. The uterus is tender to palpation, especially at the fundus.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 18,000/mm³
- Hemoglobin: 12.2 g/dL
- Platelet count: 250,000/mm³
- C-reactive protein: 24 mg/L (normal <10 mg/L)
- Urinalysis: normal

## Encounter 2: source 2021-01-15, practice 2026-05-03, inpatient, Obstetrics & Gynecology

Chief complaint: Labor and delivery at term gestation

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Labor and delivery at term gestation

HISTORY OF PRESENT ILLNESS:
This 29-year-old patient with generalized anxiety disorder, hypothyroidism, iron deficiency anemia, and prior PPROM and chorioamnionitis presents at term for delivery. She delivers a full-term male infant via spontaneous vaginal delivery. The pregnancy was uncomplicated, with prenatal care initiated early. At delivery, the amniotic fluid is clear, and the infant is born with good tone and a strong cry. The umbilical cord is clamped and cut, and the infant is placed on the mother's abdomen. The infant initially appears pink but within the first minute begins to exhibit mild central cyanosis.

PAST MEDICAL HISTORY:
Active Problem List:
- Preterm premature rupture of membranes (PPROM) (diagnosed 2020-01-15)
- Intra-amniotic infection (Chorioamnionitis) (diagnosed 2020-01-15)
- Generalized anxiety disorder
- Hypothyroidism
- Iron deficiency anemia

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 16)
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Levothyroxine 88 mcg PO daily
- Prenatal vitamin with iron 1 tablet PO daily
- Sertraline 50 mg PO daily
- Docusate sodium 100 mg PO PRN

ALLERGIES:
Penicillin (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Maternal history of preterm premature rupture of membranes
- Paternal type 2 diabetes
- Sister with chorioamnionitis in prior pregnancy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstinent during reproductive years). Occupation: Marketing coordinator.

VITAL SIGNS:
The infant's heart rate is 140 beats per minute, respiratory rate is 50 breaths per minute with good effort,

PHYSICAL EXAMINATION:
Muscle tone is active. The infant is crying vigorously.

PLAN:
The delivery room is warm, and the infant is dried and stimulated by rubbing the back.
