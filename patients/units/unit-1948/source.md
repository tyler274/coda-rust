# synthetic_hospital patient 1948

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex F, White, non-Hispanic, Private PPO (parent's employer-sponsored plan).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-15 (Mon) | inpatient | Pulmonology |
| 1 | 2021-05-15 | 2025-11-19 (Wed) | outpatient | Pulmonology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO (parent's employer-sponsored plan)",
 "occupation": "Administrative Assistant (remote hybrid)",
 "alcohol_use": "None",
 "family_history": [
  "Father: Cystic fibrosis carrier (F508del), healthy",
  "Mother: Cystic fibrosis carrier (F508del), healthy",
  "Younger brother: Cystic fibrosis, age 21",
  "Maternal grandmother: Hypertension"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "2.5 mg nebulized once daily",
   "name": "Dornase alfa (Pulmozyme)"
  },
  {
   "dose": "3% nebulized twice daily",
   "name": "Hypertonic saline"
  },
  {
   "dose": "90 mcg/actuation, 2 puffs every 4-6 hours as needed",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "250 mg orally three times weekly (Mon/Wed/Fri)",
   "name": "Azithromycin"
  },
  {
   "dose": "300 mg nebulized twice daily in alternating 28-day on/off cycles",
   "name": "Tobramycin inhalation solution (Tobi)"
  },
  {
   "dose": "36,000 units with meals, 18,000 units with snacks",
   "name": "Pancrelipase (Creon)"
  },
  {
   "dose": "2000 IU orally daily",
   "name": "Vitamin D3 (cholecalciferol)"
  },
  {
   "dose": "1 capsule orally daily",
   "name": "Multivitamin (ADEK)"
  },
  {
   "dose": "16 units subcutaneously at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "20 mg orally daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "Functional endoscopic sinus surgery (2019)",
  "Mediport placement (2020)"
 ],
 "chronic_conditions": [
  "Cystic fibrosis (homozygous F508del)",
  "Cystic fibrosis-related diabetes",
  "Pancreatic insufficiency",
  "Chronic sinusitis with nasal polyps",
  "Gastroesophageal reflux disease"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Cystic fibrosis with pulmonary manifestations, severe", "Pulmonary aspergillosis"]
Comorbidities: ["Cystic fibrosis"]

## Encounter 0: source 2020-01-15, practice 2024-07-15, inpatient, Pulmonology

Chief complaint: Two-week history of increased cough, greenish sputum production, and progressive shortness of breath

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: inpatient

CHIEF COMPLAINT:
Two-week history of increased cough, greenish sputum production, and progressive shortness of breath

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 2-week history of increased cough, sputum production, and shortness of breath. She reports that her sputum has become greenish and more copious. She denies any recent travel or sick contacts but mentions that her symptoms have progressively worsened despite using her usual inhaler therapies.

PAST MEDICAL HISTORY:
Active Problem List:
- Cystic fibrosis (homozygous F508del)
- Cystic fibrosis-related diabetes
- Pancreatic insufficiency
- Chronic sinusitis with nasal polyps
- Gastroesophageal reflux disease

PAST SURGICAL HISTORY:
- Functional endoscopic sinus surgery (2019)
- Mediport placement (2020)

MEDICATIONS:
Home Medications:
- Dornase alfa (Pulmozyme) 2.5 mg nebulized once daily
- Hypertonic saline 3% nebulized twice daily
- Albuterol HFA inhaler 90 mcg/actuation, 2 puffs every 4-6 hours as needed
- Azithromycin 250 mg orally three times weekly (Mon/Wed/Fri)
- Tobramycin inhalation solution (Tobi) 300 mg nebulized twice daily in alternating 28-day on/off cycles
- Pancrelipase (Creon) 36,000 units with meals, 18,000 units with snacks
- Vitamin D3 (cholecalciferol) 2000 IU orally daily
- Multivitamin (ADEK) 1 capsule orally daily
- Insulin glargine 16 units subcutaneously at bedtime
- Omeprazole 20 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Cystic fibrosis carrier (F508del), healthy
- Mother: Cystic fibrosis carrier (F508del), healthy
- Younger brother: Cystic fibrosis, age 21
- Maternal grandmother: Hypertension

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Administrative Assistant (remote hybrid).

VITAL SIGNS:
Her temperature is 38.2°C (100.8°F), respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals diffuse wheezing and crackles in the lower lobes bilaterally.

LABORATORY DATA:
A sputum culture grows a gram-negative organism that is oxidase positive.

Laboratory Values:
- White blood cell count: 15,000/µL (Normal: 4,500-11,000/µL)
- Neutrophils: 78% (Normal: 40-60%)
- Hemoglobin: 12.5 g/dL (Normal: 12-16 g/dL)
- Platelets: 250,000/µL (Normal: 150,000-450,000/µL)

OTHER STUDIES:
Her most recent pulmonary function tests show a 15% decline from her baseline forced expiratory volume in 1 second (FEV1).

## Encounter 1: source 2021-05-15, practice 2025-11-19, outpatient, Pulmonology

Chief complaint: Three-week history of worsening cough, sputum production, and dyspnea unresponsive to usual antibiotic regimen

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Michael Okonkwo, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-week history of worsening cough, sputum production, and dyspnea unresponsive to usual antibiotic regimen

HISTORY OF PRESENT ILLNESS:
The 25-year-old female with cystic fibrosis (homozygous F508del) returns following her January 2020 admission. She reports that her symptoms have not improved despite using her usual antibiotic regimen and increasing the use of her albuterol inhaler. She denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Cystic fibrosis with pulmonary manifestations, severe (diagnosed 2020-01-15)
- Cystic fibrosis (homozygous F508del)
- Cystic fibrosis-related diabetes
- Pancreatic insufficiency
- Chronic sinusitis with nasal polyps
- Gastroesophageal reflux disease

PAST SURGICAL HISTORY:
- Functional endoscopic sinus surgery (2019)
- Mediport placement (2020)

MEDICATIONS:
Home Medications:
- Dornase alfa (Pulmozyme) 2.5 mg nebulized once daily
- Hypertonic saline 3% nebulized twice daily
- Albuterol HFA inhaler 90 mcg/actuation, 2 puffs every 4-6 hours as needed
- Azithromycin 250 mg orally three times weekly (Mon/Wed/Fri)
- Tobramycin inhalation solution (Tobi) 300 mg nebulized twice daily in alternating 28-day on/off cycles
- Pancrelipase (Creon) 36,000 units with meals, 18,000 units with snacks
- Vitamin D3 (cholecalciferol) 2000 IU orally daily
- Multivitamin (ADEK) 1 capsule orally daily
- Insulin glargine 16 units subcutaneously at bedtime
- Omeprazole 20 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Cystic fibrosis carrier (F508del), healthy
- Mother: Cystic fibrosis carrier (F508del), healthy
- Younger brother: Cystic fibrosis, age 21
- Maternal grandmother: Hypertension

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Administrative Assistant (remote hybrid).

VITAL SIGNS:
Her temperature is 37.2°C (99°F), blood pressure is 120/75 mmHg, pulse is 88/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
Physical examination reveals diffuse wheezing and crackles on lung auscultation.

LABORATORY DATA:
- Total serum IgE: 1500 IU/mL (Normal: <100 IU/mL)
- Aspergillus-specific IgE: Positive
- White blood cell count: 12,000/µL with eosinophilia

IMAGING:
- Chest X-ray: Infiltrates in the upper lobes

OTHER STUDIES:
Her most recent pulmonary function tests show a 15% decline from her baseline.
