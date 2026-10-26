# synthetic_hospital patient 2103

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 48 at first encounter, sex M, Non-Hispanic White, Medicare (SSDI) with Aetna supplemental.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-03 (Mon) | outpatient | Pulmonology |
| 1 | 2021-07-15 | 2026-08-05 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare (SSDI) with Aetna supplemental",
 "occupation": "Former flavoring manufacturing plant operator (currently on disability leave)",
 "alcohol_use": "Rare, 1-2 drinks on holidays",
 "family_history": [
  "Father: Chronic obstructive pulmonary disease (diagnosed age 55)",
  "Mother: Hypertension, Type 2 diabetes mellitus",
  "Older brother: Asthma"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1.5 packs per day for 30 years (45 pack-years)",
 "home_medications": [
  {
   "dose": "18 mcg inhaled once daily",
   "name": "Tiotropium bromide"
  },
  {
   "dose": "250/50 mcg inhaled twice daily (started 4 weeks ago)",
   "name": "Fluticasone propionate/Salmeterol"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for shortness of breath",
   "name": "Albuterol sulfate"
  }
 ],
 "surgical_history": [
  "Arthroscopic knee meniscus repair (2018)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD), GOLD Group C"
 ],
 "age_at_first_encounter": 48
}
```

Primary diagnoses: ["Bronchiolitis obliterans", "Tension pneumothorax"]
Comorbidities: ["Chronic obstructive pulmonary disease"]

## Encounter 0: source 2020-01-15, practice 2025-02-03, outpatient, Pulmonology

Chief complaint: Follow-up visit for chronic respiratory symptoms; reports subjective improvement on new ICS/LABA therapy but persistent cough and occasional wheezing

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit for chronic respiratory symptoms; reports subjective improvement on new ICS/LABA therapy but persistent cough and occasional wheezing

HISTORY OF PRESENT ILLNESS:
He was started on a new inhaled corticosteroid/long-acting beta-agonist (ICS/LABA) combination therapy 4 weeks ago. He reports a subjective improvement in his symptoms, stating that he feels he can breathe easier and has had fewer exacerbations. However, he notes a persistent cough and occasional wheezing, especially at night. His previous physician, who was involved in a clinical trial for the new medication, had assured him of its efficacy.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD), GOLD Group C

PAST SURGICAL HISTORY:
- Arthroscopic knee meniscus repair (2018)

MEDICATIONS:
Home Medications:
- Tiotropium bromide 18 mcg inhaled once daily
- Fluticasone propionate/Salmeterol 250/50 mcg inhaled twice daily (started 4 weeks ago)
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease (diagnosed age 55)
- Mother: Hypertension, Type 2 diabetes mellitus
- Older brother: Asthma

SOCIAL HISTORY:
Smoking: Current smoker, 1.5 packs per day for 30 years (45 pack-years). Alcohol: Rare, 1-2 drinks on holidays. Occupation: Former flavoring manufacturing plant operator (currently on disability leave).

VITAL SIGNS:
On examination, his oxygen saturation is 94% on room air

PHYSICAL EXAMINATION:
Auscultation reveals mild expiratory wheezes bilaterally.

OTHER STUDIES:
Spirometry shows a forced expiratory volume in 1 second (FEV1) of 60% predicted, which is unchanged from his last visit.

ASSESSMENT AND PLAN:
Given the patient's subjective improvement but objective lack of change in FEV1,

## Encounter 1: source 2021-07-15, practice 2026-08-05, ed, Emergency Medicine

Chief complaint: Acute onset of severe shortness of breath and left-sided chest pain following vigorous coughing episode

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of severe shortness of breath and left-sided chest pain following vigorous coughing episode

HISTORY OF PRESENT ILLNESS:
This 49-year-old male with COPD GOLD Group C and history of bronchiolitis obliterans diagnosed in January 2020, recently started on fluticasone propionate/salmeterol, presents to the emergency department with acute onset of shortness of breath and left-sided chest pain that started suddenly after a vigorous coughing episode. He appears anxious, diaphoretic, and has difficulty speaking in full sentences.

PAST MEDICAL HISTORY:
Active Problem List:
- Bronchiolitis obliterans (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), GOLD Group C

PAST SURGICAL HISTORY:
- Arthroscopic knee meniscus repair (2018)

MEDICATIONS:
Home Medications:
- Tiotropium bromide 18 mcg inhaled once daily
- Fluticasone propionate/Salmeterol 250/50 mcg inhaled twice daily (started 4 weeks ago)
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease (diagnosed age 55)
- Mother: Hypertension, Type 2 diabetes mellitus
- Older brother: Asthma

SOCIAL HISTORY:
Smoking: Current smoker, 1.5 packs per day for 30 years (45 pack-years). Alcohol: Rare, 1-2 drinks on holidays. Occupation: Former flavoring manufacturing plant operator (currently on disability leave).

VITAL SIGNS:
Blood pressure 90/60 mmHg, heart rate 122 beats per minute, respiratory rate 30 breaths per minute, oxygen saturation 88% on room air, and temperature 37.2°C

PHYSICAL EXAMINATION:
Physical examination reveals jugular venous distension, tracheal deviation to the right, and hyperresonance with decreased breath sounds on the left side of the chest.

LABORATORY DATA:
The patient's arterial blood gas on room air shows a pH of 7.32, PaCO2 of 50 mmHg, PaO2 of 60 mmHg, and HCO3- of 24 mEq/L.

IMAGING:
A chest X-ray confirms the presence of a large left-sided pneumothorax with mediastinal shift.

ASSESSMENT AND PLAN:
The patient's condition deteriorates rapidly, and he requires airway protection.
