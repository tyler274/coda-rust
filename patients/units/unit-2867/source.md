# synthetic_hospital patient 2867

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-08 (Mon) | ed | Emergency Medicine |
| 1 | 2020-04-15 | 2025-12-06 (Sat) | inpatient | Pulmonary & Critical Care |
| 2 | 2020-07-15 | 2026-03-07 (Sat) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal clerk",
 "alcohol_use": "None",
 "family_history": [
  "Father: Lung cancer",
  "Mother: Emphysema",
  "Brother: Chronic obstructive pulmonary disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 45 pack-years, quit 6 months ago",
 "home_medications": [
  {
   "dose": "18 mcg inhaled once daily",
   "name": "Tiotropium"
  },
  {
   "dose": "90 mcg inhaled every 4 to 6 hours as needed for shortness of breath",
   "name": "Albuterol (salbutamol)"
  },
  {
   "dose": "250 mcg inhaled twice daily",
   "name": "Fluticasone propionate"
  },
  {
   "dose": "8 mg orally every 8 hours as needed for nausea",
   "name": "Ondansetron"
  }
 ],
 "surgical_history": [
  "Bronchoscopy with transbronchial biopsy",
  "Port-a-cath placement"
 ],
 "chronic_conditions": [
  "Severe chronic obstructive pulmonary disease (COPD), GOLD Stage III",
  "Small cell lung cancer, currently undergoing chemotherapy"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Syndrome of inappropriate antidiuretic hormone secretion (SIADH)", "Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia"]
Comorbidities: ["Chronic obstructive pulmonary disease"]

## Encounter 0: source 2020-01-15, practice 2025-09-08, ed, Emergency Medicine

Chief complaint: Increased shortness of breath, productive cough with yellow sputum, and fever

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increased shortness of breath, productive cough with yellow sputum, and fever

HISTORY OF PRESENT ILLNESS:
She presents to the emergency department with a 3-day history of increased shortness of breath, productive cough with yellow sputum, and fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe chronic obstructive pulmonary disease (COPD), GOLD Stage III
- Small cell lung cancer, currently undergoing chemotherapy

PAST SURGICAL HISTORY:
- Bronchoscopy with transbronchial biopsy
- Port-a-cath placement

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol (salbutamol) 90 mcg inhaled every 4 to 6 hours as needed for shortness of breath
- Fluticasone propionate 250 mcg inhaled twice daily
- Ondansetron 8 mg orally every 8 hours as needed for nausea

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Lung cancer
- Mother: Emphysema
- Brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
Smoking: Former smoker, 45 pack-years, quit 6 months ago. Alcohol: None. Occupation: Retired postal clerk.

REVIEW OF SYSTEMS:
She denies chest pain, vomiting, or diarrhea.

VITAL SIGNS:
Her temperature is 38.6°C (101.5°F), blood pressure is 135/85 mmHg, pulse is 102/min, and respiratory rate is 28/min.

PHYSICAL EXAMINATION:
Physical examination reveals crackles in the right lower lung field.

LABORATORY DATA:
Arterial blood gas (ABG) analysis shows:

- pH: 7.48
- PaCO2: 28 mmHg
- PaO2: 60 mmHg
- HCO3: 24 mEq/L

IMAGING:
Chest X-ray demonstrates a right lower lobe infiltrate.

## Encounter 1: source 2020-04-15, practice 2025-12-06, inpatient, Pulmonary & Critical Care

Chief complaint: Progressive shortness of breath, productive cough with yellow sputum, and generalized weakness

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonary & Critical Care
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive shortness of breath, productive cough with yellow sputum, and generalized weakness

HISTORY OF PRESENT ILLNESS:
72-year-old with COPD GOLD III, small cell lung cancer on chemotherapy, prior ED visit for similar symptoms. She presents to ED with 3-day history of progressive shortness of breath, productive cough with yellow sputum, and generalized weakness. She denies chest pain, fever, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia (diagnosed 2020-01-15)
- Severe chronic obstructive pulmonary disease (COPD), GOLD Stage III
- Small cell lung cancer, currently undergoing chemotherapy

PAST SURGICAL HISTORY:
- Bronchoscopy with transbronchial biopsy
- Port-a-cath placement

MEDICATIONS:
Her medications include tiotropium, salbutamol, and fluticasone.

Home Medications:
- Albuterol (salbutamol) 90 mcg inhaled every 4 to 6 hours as needed for shortness of breath
- Fluticasone propionate 250 mcg inhaled twice daily
- Ondansetron 8 mg orally every 8 hours as needed for nausea

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Lung cancer
- Mother: Emphysema
- Brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
Smoking: Former smoker, 45 pack-years, quit 6 months ago. Alcohol: None. Occupation: Retired postal clerk.

VITAL SIGNS:
On examination, she is afebrile, blood pressure is 130/80 mmHg, heart rate is 88 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral wheezes and crackles at the bases.

LABORATORY DATA:
Laboratory findings are as follows:

- Sodium: 125 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 4.2 mEq/L (Normal: 3.5-5.1 mEq/L)
- Chloride: 95 mEq/L (Normal: 98-106 mEq/L)
- Bicarbonate: 24 mEq/L (Normal: 22-29 mEq/L)
- Blood urea nitrogen: 18 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 0.9 mg/dL (Normal: 0.6-1.1 mg/dL)
- Glucose: 98 mg/dL (Normal: 70-99 mg/dL)
- Serum osmolality: 260 mOsm/kg (Normal: 275-295 mOsm/kg)
- Urine osmolality: 500 mOsm/kg (Normal: 50-1200 mOsm/kg)
- Urine sodium: 80 mEq/L (Normal: 20-40 mEq/L)

IMAGING:
Chest X-ray shows increased interstitial markings and a small right-sided pleural effusion.

ASSESSMENT AND PLAN:
The patient is diagnosed with an acute exacerbation of COPD and secondary bacterial pneumonia.

She is started on appropriate antibiotics and bronchodilators. Considering her electrolyte abnormalities,

## Encounter 2: source 2020-07-15, practice 2026-03-07, icu, Critical Care Medicine

Chief complaint: Confusion and lethargy

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Confusion and lethargy

HISTORY OF PRESENT ILLNESS:
This 72-year-old with COPD (GOLD III) and small cell lung cancer on chemotherapy, hospitalized in April 2020, presents to the emergency department with a two-day history of confusion and lethargy. Her daughter reports that the patient has been less responsive and has not been eating or drinking much over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia (diagnosed 2020-01-15)
- Syndrome of inappropriate antidiuretic hormone secretion (SIADH) (diagnosed 2020-04-15)
- Severe chronic obstructive pulmonary disease (COPD), GOLD Stage III
- Small cell lung cancer, currently undergoing chemotherapy

The patient has a history of small cell lung cancer and is currently undergoing chemotherapy.

PAST SURGICAL HISTORY:
- Bronchoscopy with transbronchial biopsy
- Port-a-cath placement

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol (salbutamol) 90 mcg inhaled every 4 to 6 hours as needed for shortness of breath
- Fluticasone propionate 250 mcg inhaled twice daily
- Ondansetron 8 mg orally every 8 hours as needed for nausea

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Lung cancer
- Mother: Emphysema
- Brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
Smoking: Former smoker, 45 pack-years, quit 6 months ago. Alcohol: None. Occupation: Retired postal clerk.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On examination, the patient is oriented to person but not to time or place. There is no evidence of dehydration. Neurological examination is non-focal.

LABORATORY DATA:
- Serum sodium: 120 mEq/L
- Serum osmolality: 260 mOsm/kg
- Urine sodium: 45 mEq/L
- Urine osmolality: 500 mOsm/kg
