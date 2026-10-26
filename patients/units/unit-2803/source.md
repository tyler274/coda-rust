# synthetic_hospital patient 2803

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex F, Non-Hispanic White, Employer-sponsored PPO (Aetna).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-14 (Mon) | outpatient | Pulmonology |
| 1 | 2020-09-15 | 2025-12-10 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA",
  "Environmental: House dust mites, grass pollen"
 ],
 "insurance": "Employer-sponsored PPO (Aetna)",
 "occupation": "Administrative coordinator at a logistics firm",
 "alcohol_use": "Social (1-2 glasses of wine on weekends)",
 "family_history": [
  "Mother: Rheumatoid arthritis with bronchiectasis",
  "Father: Chronic obstructive pulmonary disease (former smoker)",
  "Maternal aunt: Alpha-1 antitrypsin deficiency carrier"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (27 pack-years), quit 2 months ago",
 "home_medications": [
  {
   "dose": "250 mg oral three times weekly (chronic suppressive therapy)",
   "name": "Azithromycin"
  },
  {
   "dose": "100/25 mcg inhaled once daily",
   "name": "Fluticasone furoate/vilanterol"
  },
  {
   "dose": "90 mcg inhaler, 2 puffs every 4-6 hours as needed for dyspnea",
   "name": "Albuterol sulfate"
  },
  {
   "dose": "600 mg nebulized twice daily",
   "name": "Acetylcysteine"
  },
  {
   "dose": "20 mg oral daily",
   "name": "Omeprazole"
  },
  {
   "dose": "0.9% NaCl, twice daily",
   "name": "Saline nasal rinse"
  }
 ],
 "surgical_history": [
  "Tonsillectomy and adenoidectomy (age 7)",
  "Video-assisted thoracoscopic surgery (VATS) lung biopsy (age 44)"
 ],
 "chronic_conditions": [
  "Non-CF bronchiectasis (post-infectious)",
  "Chronic rhinosinusitis",
  "Gastroesophageal reflux disease",
  "Mild persistent asthma",
  "History of severe childhood pneumonia requiring hospitalization"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Bronchiectasis"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-04-14, outpatient, Pulmonology

Chief complaint: Chronic productive cough with recurrent episodes of copious foul-smelling sputum production over the past several years.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic productive cough with recurrent episodes of copious foul-smelling sputum production over the past several years.

HISTORY OF PRESENT ILLNESS:
She presents with a chronic productive cough and recurrent episodes of copious foul-smelling sputum production over the past several years

PAST MEDICAL HISTORY:
Active Problem List:
- Non-CF bronchiectasis (post-infectious)
- Chronic rhinosinusitis
- Gastroesophageal reflux disease
- Mild persistent asthma
- History of severe childhood pneumonia requiring hospitalization

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy (age 7)
- Video-assisted thoracoscopic surgery (VATS) lung biopsy (age 44)

MEDICATIONS:
Home Medications:
- Azithromycin 250 mg oral three times weekly (chronic suppressive therapy)
- Fluticasone furoate/vilanterol 100/25 mcg inhaled once daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4-6 hours as needed for dyspnea
- Acetylcysteine 600 mg nebulized twice daily
- Omeprazole 20 mg oral daily
- Saline nasal rinse 0.9% NaCl, twice daily

ALLERGIES:
NKDA, Environmental: House dust mites, grass pollen

FAMILY HISTORY:
- Mother: Rheumatoid arthritis with bronchiectasis
- Father: Chronic obstructive pulmonary disease (former smoker)
- Maternal aunt: Alpha-1 antitrypsin deficiency carrier

SOCIAL HISTORY:
She has been smoking one pack of cigarettes daily for 27 years.

PHYSICAL EXAMINATION:
On examination, there are crackles at the lung bases and digital clubbing.

LABORATORY DATA:
Sputum cultures have grown Pseudomonas on multiple occasions.

IMAGING:
A high-resolution CT scan of the chest shows dilated airways with thickened bronchial walls and lack of normal tapering, described as “tram-track” and “signet ring” signs.

## Encounter 1: source 2020-09-15, practice 2025-12-10, ed, Emergency Medicine

Chief complaint: Three-year history of chronic cough with intermittent episodes of hemoptysis and three recurrent pneumonia episodes in the past year.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three-year history of chronic cough with intermittent episodes of hemoptysis and three recurrent pneumonia episodes in the past year.

HISTORY OF PRESENT ILLNESS:
This 45-year-old with non-CF bronchiectasis returns for emergency evaluation following her January 2020 pulmonology visit. She reports producing large amounts of thick, greenish sputum daily. She has a history of recurrent pneumonia, with three episodes in the past year. She has no history of travel or exposure to tuberculosis.

PAST MEDICAL HISTORY:
Active Problem List:
- Bronchiectasis (diagnosed 2020-01-15)
- Non-CF bronchiectasis (post-infectious)
- Chronic rhinosinusitis
- Gastroesophageal reflux disease
- Mild persistent asthma
- History of severe childhood pneumonia requiring hospitalization

She has a history of childhood pneumonia that required hospitalization.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy (age 7)
- Video-assisted thoracoscopic surgery (VATS) lung biopsy (age 44)

MEDICATIONS:
Home Medications:
- Azithromycin 250 mg oral three times weekly (chronic suppressive therapy)
- Fluticasone furoate/vilanterol 100/25 mcg inhaled once daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4-6 hours as needed for dyspnea
- Acetylcysteine 600 mg nebulized twice daily
- Omeprazole 20 mg oral daily
- Saline nasal rinse 0.9% NaCl, twice daily

ALLERGIES:
NKDA, Environmental: House dust mites, grass pollen

FAMILY HISTORY:
- Mother: Rheumatoid arthritis with bronchiectasis
- Father: Chronic obstructive pulmonary disease (former smoker)
- Maternal aunt: Alpha-1 antitrypsin deficiency carrier

SOCIAL HISTORY:
She denies smoking

VITAL SIGNS:
Oxygen saturation on room air is 94%

PHYSICAL EXAMINATION:
On physical examination, you note clubbing of the fingers and coarse crackles at the lung bases bilaterally.

IMAGING:
A high-resolution CT scan of the chest reveals bronchial wall thickening and dilation predominantly in the lower lobes.
