# synthetic_hospital patient 1853

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-29 (Fri) | inpatient | Obstetrics & Gynecology |
| 1 | 2020-09-15 | 2025-07-21 (Mon) | ed | Emergency Medicine |
| 2 | 2021-09-15 | 2026-07-30 (Thu) | inpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Latex (rash)",
  "Penicillin (unknown reaction)"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Environmental Consultant",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother: Preterm delivery, seasonal allergies",
  "Father: Hypertension, Type 2 diabetes",
  "Maternal aunt: High-altitude pulmonary edema susceptibility"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "5 mg daily during allergy season",
   "name": "Levocetirizine"
  },
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin with DHA"
  }
 ],
 "surgical_history": [
  "Cesarean section (low transverse)",
  "Knee arthroscopy (age 24)"
 ],
 "chronic_conditions": [
  "Seasonal allergic rhinitis",
  "History of preterm labor"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Respiratory distress syndrome", "High-altitude pulmonary edema"]
Comorbidities: ["Hypoxemia", "Respiratory arrest", "Preterm labor", "Preterm birth at 27 weeks gestation"]

## Encounter 0: source 2020-01-15, practice 2024-11-29, inpatient, Obstetrics & Gynecology

Chief complaint: Preterm labor at 32 weeks gestation despite tocolytic therapy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Preterm labor at 32 weeks gestation despite tocolytic therapy

HISTORY OF PRESENT ILLNESS:
The mother received prenatal care and had an uncomplicated pregnancy until she presented with preterm labor. Despite tocolytics and a course of antenatal corticosteroids, she delivered prematurely.

PAST MEDICAL HISTORY:
Active Problem List:
- Seasonal allergic rhinitis
- History of preterm labor

PAST SURGICAL HISTORY:
- Cesarean section (low transverse)
- Knee arthroscopy (age 24)

MEDICATIONS:
Home Medications:
- Levocetirizine 5 mg daily during allergy season
- Prenatal vitamin with DHA 1 tablet daily

ALLERGIES:
Latex (rash), Penicillin (unknown reaction)

FAMILY HISTORY:
- Mother: Preterm delivery, seasonal allergies
- Father: Hypertension, Type 2 diabetes
- Maternal aunt: High-altitude pulmonary edema susceptibility

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Environmental Consultant.

VITAL SIGNS:
On physical examination, the infant has a respiratory rate of 70 breaths per minute, heart rate of 160 beats per minute, and oxygen saturation of 85% on room air.

PHYSICAL EXAMINATION:
Auscultation of the chest reveals decreased breath sounds bilaterally.

IMAGING:
A chest x-ray is performed and shows a reticulogranular pattern with air bronchograms.

## Encounter 1: source 2020-09-15, practice 2025-07-21, ed, Emergency Medicine

Chief complaint: Progressive shortness of breath and dry cough after hiking at high altitude

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath and dry cough after hiking at high altitude

HISTORY OF PRESENT ILLNESS:
The patient with January 2020 preterm labor and seasonal allergic rhinitis presents following high-altitude exposure. She recently returned from a hiking trip in the Rocky Mountains where she ascended to 12,000 feet over a period of two days. She denies sick contacts or recent illnesses.

PAST MEDICAL HISTORY:
Active Problem List:
- Respiratory distress syndrome (diagnosed 2020-01-15)
- Seasonal allergic rhinitis
- History of preterm labor

PAST SURGICAL HISTORY:
- Cesarean section (low transverse)
- Knee arthroscopy (age 24)

MEDICATIONS:
Home Medications:
- Levocetirizine 5 mg daily during allergy season
- Prenatal vitamin with DHA 1 tablet daily

ALLERGIES:
Latex (rash), Penicillin (unknown reaction)

FAMILY HISTORY:
- Mother: Preterm delivery, seasonal allergies
- Father: Hypertension, Type 2 diabetes
- Maternal aunt: High-altitude pulmonary edema susceptibility

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Environmental Consultant.

VITAL SIGNS:
Her temperature is 37.2°C (99°F), blood pressure is 130/80 mmHg, pulse is 95/min, and respiratory rate is 26/min. Oxygen saturation is 86% on room air. After receiving supplemental oxygen, her oxygen saturation improves to 94%.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral crackles at the lung bases.

LABORATORY DATA:
Laboratory studies are as follows:

- White blood cell count: 11,000/mm³
- Hemoglobin: 14.5 g/dL
- Platelets: 250,000/mm³
- Procalcitonin: 0.1 ng/mL (normal <0.5 ng/mL)
- Arterial blood gas on room air:
- pH: 7.45
- PaCO2: 30 mmHg
- PaO2: 55 mmHg
- HCO3-: 24 mEq/L

IMAGING:
Chest X-ray shows bilateral alveolar infiltrates.

## Encounter 2: source 2021-09-15, practice 2026-07-30, inpatient, Obstetrics & Gynecology

Chief complaint: Acute onset of preterm labor at 34 weeks gestation

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Priya Patel, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Acute onset of preterm labor at 34 weeks gestation

HISTORY OF PRESENT ILLNESS:
Patient with history of preterm labor returns with premature labor. The pregnancy was uncomplicated until premature labor began. He is placed under warming lights and receives tactile stimulation, but continues to exhibit respiratory distress.

PAST MEDICAL HISTORY:
Active Problem List:
- Respiratory distress syndrome (diagnosed 2020-01-15)
- High-altitude pulmonary edema (diagnosed 2020-09-15)
- Seasonal allergic rhinitis
- History of preterm labor

PAST SURGICAL HISTORY:
- Cesarean section (low transverse)
- Knee arthroscopy (age 24)

MEDICATIONS:
Home Medications:
- Levocetirizine 5 mg daily during allergy season
- Prenatal vitamin with DHA 1 tablet daily

ALLERGIES:
Latex (rash), Penicillin (unknown reaction)

FAMILY HISTORY:
- Mother: Preterm delivery, seasonal allergies
- Father: Hypertension, Type 2 diabetes
- Maternal aunt: High-altitude pulmonary edema susceptibility

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Environmental Consultant.

PHYSICAL EXAMINATION:
On auscultation, bilateral breath sounds are diminished and fine rales are present.

LABORATORY DATA:
Arterial blood gas reveals hypoxemia and respiratory acidosis.

IMAGING:
A chest X-ray shows a diffuse reticulogranular (ground-glass) appearance with air bronchograms.

PATHOLOGY:
Despite all the effort, the infant dies and histologic section of the lung is shown.

PLAN:
The neonate is intubated and admitted to the NICU for supplemental oxygen and further care.
