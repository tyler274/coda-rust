# synthetic_hospital patient 1905

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 29 at first encounter, sex F, Hispanic, Private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2026-02-16 (Mon) | inpatient | Obstetrics & Gynecology |
| 1 | 2020-01-15 | 2026-02-25 (Wed) | inpatient | Neonatology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private insurance",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Sister: Preterm delivery at 32 weeks gestation",
  "Mother: Hypothyroidism",
  "Father: Essential hypertension"
 ],
 "race_ethnicity": "Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin with iron"
  }
 ],
 "surgical_history": [
  "Laparoscopic appendectomy at age 22",
  "Wisdom teeth extraction"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 29
}
```

Primary diagnoses: ["Respiratory distress syndrome"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2026-02-16, inpatient, Obstetrics & Gynecology

Chief complaint: Preterm labor at 30 weeks gestation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Preterm labor at 30 weeks gestation

HISTORY OF PRESENT ILLNESS:
Her prenatal course has been uncomplicated, and she has not received any antenatal corticosteroids. She delivers a male infant vaginally after spontaneous rupture of membranes. Apgar scores are 6 and 7 at 1 and 5 minutes, respectively. Within the first hour of life, the neonate develops labored breathing, flaring of the nostrils, and subcostal retractions. Oxygen saturation remains low despite supplemental oxygen.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy at age 22
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Sister: Preterm delivery at 32 weeks gestation
- Mother: Hypothyroidism
- Father: Essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination reveals diminished breath sounds bilaterally, and the infant exhibits poor chest expansion.

IMAGING:
A chest x-ray reveals reticulogranular appearance.

## Encounter 1: source 2020-01-15, practice 2026-02-25, inpatient, Neonatology

Chief complaint: Newborn with nasal flaring, intercostal retractions, and expiratory grunting

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. James Chen, MD
Department: Neonatology
Visit Type: inpatient

CHIEF COMPLAINT:
Newborn with nasal flaring, intercostal retractions, and expiratory grunting

HISTORY OF PRESENT ILLNESS:
The neonate was delivered today at 30 weeks gestation after maternal preterm labor, following premature rupture of membranes. Shortly after birth, the newborn develops nasal flaring, intercostal retractions, and expiratory grunting. He requires supplemental oxygen to maintain adequate saturation.

PAST MEDICAL HISTORY:
Active Problem List:
- Respiratory distress syndrome (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy at age 22
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Sister: Preterm delivery at 32 weeks gestation
- Mother: Hypothyroidism
- Father: Essential hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

LABORATORY DATA:
Analysis of amniotic fluid collected prior to delivery reveals a lecithin/sphingomyelin ratio of 1.2.

IMAGING:
A chest x-ray shows diffuse ground-glass opacities and air bronchograms.
