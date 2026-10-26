# synthetic_hospital patient 2302

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-14 (Thu) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-05-06 (Tue) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2025-11-12 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfonamides (maculopapular rash)",
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Deceased from complications of COPD and lung cancer at age 72",
  "Mother: Chronic bronchitis",
  "Older brother: Alpha-1 antitrypsin deficiency"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 48 pack-year history, quit 2 years ago",
 "home_medications": [
  {
   "dose": "100/62.5/25 mcg, 1 inhalation once daily",
   "name": "Fluticasone furoate/Umeclidinium/Vilanterol (Trelegy Ellipta)"
  },
  {
   "dose": "10 mg by mouth daily",
   "name": "Prednisone"
  },
  {
   "dose": "2 L/min via nasal cannula, continuous use",
   "name": "Oxygen concentrator"
  },
  {
   "dose": "90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath",
   "name": "Albuterol sulfate (ProAir HFA)"
  },
  {
   "dose": "500 mcg tablet, one by mouth daily",
   "name": "Roflumilast (Daliresp)"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (2018)",
  "Right inguinal herniorrhaphy (2010)"
 ],
 "chronic_conditions": [
  "Severe chronic obstructive pulmonary disease (COPD), GOLD stage D",
  "Emphysema predominant phenotype",
  "Chronic hypoxemic respiratory failure",
  "Pulmonary hypertension",
  "Cor pulmonale",
  "Chronic corticosteroid-dependent airway inflammation"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Complicated parapneumonic effusion"]
Comorbidities: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Chronic obstructive pulmonary disease", "Acute hypoxemic respiratory failure without hypercapnic encephalopathy"]

## Encounter 0: source 2020-01-15, practice 2024-11-14, ed, Emergency Medicine

Chief complaint: Increased shortness of breath, productive cough with greenish sputum, and fever for 3 days

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increased shortness of breath, productive cough with greenish sputum, and fever for 3 days

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department with a 3-day history of increased shortness of breath, productive cough with greenish sputum, and fever. He reports a decreased appetite and general malaise.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe chronic obstructive pulmonary disease (COPD), GOLD stage D
- Emphysema predominant phenotype
- Chronic hypoxemic respiratory failure
- Pulmonary hypertension
- Cor pulmonale
- Chronic corticosteroid-dependent airway inflammation

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2018)
- Right inguinal herniorrhaphy (2010)

MEDICATIONS:
Home Medications:
- Fluticasone furoate/Umeclidinium/Vilanterol (Trelegy Ellipta) 100/62.5/25 mcg, 1 inhalation once daily
- Prednisone 10 mg by mouth daily
- Oxygen concentrator 2 L/min via nasal cannula, continuous use
- Albuterol sulfate (ProAir HFA) 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Roflumilast (Daliresp) 500 mcg tablet, one by mouth daily

ALLERGIES:
Sulfonamides (maculopapular rash), NKDA

FAMILY HISTORY:
- Father: Deceased from complications of COPD and lung cancer at age 72
- Mother: Chronic bronchitis
- Older brother: Alpha-1 antitrypsin deficiency

SOCIAL HISTORY:
Smoking: Former smoker, 48 pack-year history, quit 2 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
His temperature is 38.6°C, blood pressure is 135/85 mmHg, heart rate is 102 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals decreased breath sounds and crackles at the right lung base.

LABORATORY DATA:
Arterial blood gas on room air shows:

- pH: 7.45
- PaCO2: 32 mmHg
- PaO2: 55 mmHg
- HCO3-: 24 mEq/L

IMAGING:
Chest X-ray shows a right lower lobe consolidation.

PLAN:
The patient is started on appropriate antibiotics. Despite receiving supplemental oxygen via a nasal cannula at 5 L/min, his oxygen saturation improves only to 90%.

## Encounter 1: source 2020-07-15, practice 2025-05-06, ed, Emergency Medicine

Chief complaint: Increased shortness of breath, productive cough with yellowish sputum, and fever for 3 days with worsening when lying on right side

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increased shortness of breath, productive cough with yellowish sputum, and fever for 3 days with worsening when lying on right side

HISTORY OF PRESENT ILLNESS:
The patient with severe COPD (GOLD Stage D), emphysema predominant phenotype, chronic hypoxemic respiratory failure on continuous oxygen, and corticosteroid-dependent airway inflammation returns to the emergency department six months after his prior presentation with similar symptoms. He reports that his symptoms worsen when he lies on his right side.

PAST MEDICAL HISTORY:
Active Problem List:
- Opportunistic pulmonary infection requiring diagnostic sampling (diagnosed 2020-01-15)
- Severe chronic obstructive pulmonary disease (COPD), GOLD stage D
- Emphysema predominant phenotype
- Chronic hypoxemic respiratory failure
- Pulmonary hypertension
- Cor pulmonale
- Chronic corticosteroid-dependent airway inflammation

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2018)
- Right inguinal herniorrhaphy (2010)

MEDICATIONS:
Home Medications:
- Fluticasone furoate/Umeclidinium/Vilanterol (Trelegy Ellipta) 100/62.5/25 mcg, 1 inhalation once daily
- Prednisone 10 mg by mouth daily
- Oxygen concentrator 2 L/min via nasal cannula, continuous use
- Albuterol sulfate (ProAir HFA) 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Roflumilast (Daliresp) 500 mcg tablet, one by mouth daily

ALLERGIES:
Sulfonamides (maculopapular rash), NKDA

FAMILY HISTORY:
- Father: Deceased from complications of COPD and lung cancer at age 72
- Mother: Chronic bronchitis
- Older brother: Alpha-1 antitrypsin deficiency

SOCIAL HISTORY:
Smoking: Former smoker, 48 pack-year history, quit 2 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
His temperature is 38.5°C, blood pressure is 135/85 mmHg, heart rate is 102 beats per minute, and respiratory rate is 26 breaths per minute.

PHYSICAL EXAMINATION:
On physical examination, you note decreased breath sounds and dullness to percussion at the right lung base. Oxygen saturation is 88% on room air, which improves to 92% when he sits upright.

LABORATORY DATA:
Arterial blood gas on room air shows:

- pH: 7.44
- PaCO2: 45 mmHg
- PaO2: 58 mmHg
- HCO3-: 24 mEq/L

IMAGING:
Chest X-ray shows right lower lobe consolidation.

## Encounter 2: source 2021-01-15, practice 2025-11-12, ed, Emergency Medicine

Chief complaint: Increasing shortness of breath, productive cough with yellowish sputum, and pleuritic chest pain for 4 days

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing shortness of breath, productive cough with yellowish sputum, and pleuritic chest pain for 4 days

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old male with severe COPD (GOLD stage D), emphysema predominant phenotype, chronic hypoxemic respiratory failure on continuous home oxygen, pulmonary hypertension, cor pulmonale, and chronic corticosteroid dependence who returns to the emergency department following similar presentations in January and July 2020 for respiratory infections. He reports increasing shortness of breath, productive cough with yellowish sputum, and pleuritic chest pain for 4 days. He also reports a low-grade fever and malaise.

PAST MEDICAL HISTORY:
Active Problem List:
- Opportunistic pulmonary infection requiring diagnostic sampling (diagnosed 2020-01-15)
- Severe chronic obstructive pulmonary disease (COPD), GOLD stage D
- Emphysema predominant phenotype
- Chronic hypoxemic respiratory failure
- Pulmonary hypertension
- Cor pulmonale
- Chronic corticosteroid-dependent airway inflammation

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2018)
- Right inguinal herniorrhaphy (2010)

MEDICATIONS:
Home Medications:
- Fluticasone furoate/Umeclidinium/Vilanterol (Trelegy Ellipta) 100/62.5/25 mcg, 1 inhalation once daily
- Prednisone 10 mg by mouth daily
- Oxygen concentrator 2 L/min via nasal cannula, continuous use
- Albuterol sulfate (ProAir HFA) 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Roflumilast (Daliresp) 500 mcg tablet, one by mouth daily

ALLERGIES:
Sulfonamides (maculopapular rash), NKDA

FAMILY HISTORY:
- Father: Deceased from complications of COPD and lung cancer at age 72
- Mother: Chronic bronchitis
- Older brother: Alpha-1 antitrypsin deficiency

SOCIAL HISTORY:
Smoking: Former smoker, 48 pack-year history, quit 2 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
His temperature is 38.2°C, blood pressure is 135/85 mmHg, heart rate is 102 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
On physical examination, dullness to percussion and decreased breath sounds are noted at the right lung base.

LABORATORY DATA:
Thoracentesis is performed, and the pleural fluid analysis shows:

- Appearance: Turbid
- pH: 7.10
- Glucose: 30 mg/dL
- Protein: 4.5 g/dL
- LDH: 600 U/L
- WBC count: 20,000/mm³ with 90% neutrophils

IMAGING:
Chest radiography reveals a right-sided pleural effusion.
