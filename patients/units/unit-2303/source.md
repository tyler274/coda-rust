# synthetic_hospital patient 2303

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare with Blue Cross supplemental.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-28 (Sun) | ed | Emergency Medicine |
| 1 | 2020-03-15 | 2025-12-06 (Sat) | inpatient | Internal Medicine |
| 2 | 2020-07-15 | 2026-04-06 (Mon) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin (hives)",
  "Iodinated contrast (urticaria)"
 ],
 "insurance": "Medicare with Blue Cross supplemental",
 "occupation": "Retired diesel mechanic",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father: Lung cancer, chronic bronchitis",
  "Mother: Type 2 diabetes mellitus",
  "Older brother: COPD"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 50 pack-year history, quit 3 years ago",
 "home_medications": [
  {
   "dose": "90 mcg, 2 inhalations every 4 hours as needed",
   "name": "Albuterol sulfate HFA"
  },
  {
   "dose": "18 mcg inhalation capsule daily",
   "name": "Tiotropium bromide"
  },
  {
   "dose": "250/50 mcg, 1 inhalation twice daily",
   "name": "Fluticasone propionate/Salmeterol"
  },
  {
   "dose": "2 liters per minute via nasal cannula, continuous",
   "name": "Home oxygen therapy"
  },
  {
   "dose": "10 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg oral twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg oral daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "Left knee arthroscopy (2005)",
  "Cataract extraction, bilateral (2020)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (GOLD Stage IV)",
  "Pulmonary hypertension",
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Gastroesophageal reflux disease"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Euthyroid sick syndrome", "Hyperchloremic metabolic acidosis due to excess normal saline infusion"]
Comorbidities: ["Chronic obstructive pulmonary disease"]

## Encounter 0: source 2020-01-15, practice 2025-09-28, ed, Emergency Medicine

Chief complaint: Increasing shortness of breath, productive cough with yellowish sputum, and fever

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing shortness of breath, productive cough with yellowish sputum, and fever

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department with a 4-day history of increasing shortness of breath, productive cough with yellowish sputum, and fever. He denies chest pain or leg swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (GOLD Stage IV)
- Pulmonary hypertension
- Type 2 diabetes mellitus
- Hypertension
- Gastroesophageal reflux disease

PAST SURGICAL HISTORY:
- Left knee arthroscopy (2005)
- Cataract extraction, bilateral (2020)

MEDICATIONS:
Home Medications:
- Albuterol sulfate HFA 90 mcg, 2 inhalations every 4 hours as needed
- Tiotropium bromide 18 mcg inhalation capsule daily
- Fluticasone propionate/Salmeterol 250/50 mcg, 1 inhalation twice daily
- Home oxygen therapy 2 liters per minute via nasal cannula, continuous
- Lisinopril 10 mg oral daily
- Metformin 1000 mg oral twice daily
- Omeprazole 20 mg oral daily

ALLERGIES:
Penicillin (hives), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Father: Lung cancer, chronic bronchitis
- Mother: Type 2 diabetes mellitus
- Older brother: COPD

SOCIAL HISTORY:
Smoking: Former smoker, 50 pack-year history, quit 3 years ago. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired diesel mechanic.

VITAL SIGNS:
His temperature is 38.6°C, blood pressure is 135/85 mmHg, pulse is 102 beats/min, and respiratory rate is 22 breaths/min

PHYSICAL EXAMINATION:
Physical examination reveals decreased breath sounds and dullness to percussion at the right lung base

LABORATORY DATA:
Thoracentesis is performed, and the pleural fluid analysis reveals the following:

- Appearance: Cloudy
- pH: 7.45
- Glucose: 90 mg/dL
- Leukocyte count: 500 cells/µL
- Lactate dehydrogenase (LDH): 200 U/L
- Protein: 4.0 g/dL

Serum LDH is 190 U/L and serum protein is 6.5 g/dL.

IMAGING:
Chest X-ray shows a right lower lobe consolidation with a small pleural effusion

ASSESSMENT AND PLAN:
Based on the patient's presentation and pleural fluid analysis,

## Encounter 1: source 2020-03-15, practice 2025-12-06, inpatient, Internal Medicine

Chief complaint: Increasing somnolence and difficulty breathing

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Increasing somnolence and difficulty breathing

HISTORY OF PRESENT ILLNESS:
Patient is a 65-year-old with GOLD Stage IV COPD who presented 2 months prior with similar symptoms and now presents with worsening respiratory status. Over the past 3 days, he has been coughing more than usual and producing greenish sputum. His wife states that he has become progressively drowsier and "harder to wake up," especially today. He normally uses home oxygen and inhalers but has been too fatigued to take his medications properly.

PAST MEDICAL HISTORY:
Active Problem List:
- Opportunistic pulmonary infection requiring diagnostic sampling (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (GOLD Stage IV)
- Pulmonary hypertension
- Type 2 diabetes mellitus
- Hypertension
- Gastroesophageal reflux disease

PAST SURGICAL HISTORY:
- Left knee arthroscopy (2005)
- Cataract extraction, bilateral (2020)

MEDICATIONS:
Home Medications:
- Albuterol sulfate HFA 90 mcg, 2 inhalations every 4 hours as needed
- Tiotropium bromide 18 mcg inhalation capsule daily
- Fluticasone propionate/Salmeterol 250/50 mcg, 1 inhalation twice daily
- Home oxygen therapy 2 liters per minute via nasal cannula, continuous
- Lisinopril 10 mg oral daily
- Metformin 1000 mg oral twice daily
- Omeprazole 20 mg oral daily

ALLERGIES:
Penicillin (hives), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Father: Lung cancer, chronic bronchitis
- Mother: Type 2 diabetes mellitus
- Older brother: COPD

SOCIAL HISTORY:
Smoking: Former smoker, 50 pack-year history, quit 3 years ago. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired diesel mechanic.

VITAL SIGNS:
Respiratory rate of 8/min, pulse oximetry of 88% on 4 L/min nasal cannula, and blood pressure of 138/84 mm Hg

PHYSICAL EXAMINATION:
On examination, he is somnolent but arousable. He has flaring of the nares, shallow breathing, and poor inspiratory effort. Breath sounds are decreased throughout with faint wheezes and no crackles.

LABORATORY DATA:
Arterial blood gas reveals pH 7.19, PCO₂ 76 mm Hg, and HCO₃⁻ 34 mEq/L.

PLAN:
The findings are reviewed with the wife, and the medical team initiates bilevel positive airway pressure.

## Encounter 2: source 2020-07-15, practice 2026-04-06, icu, Pulmonary & Critical Care

Chief complaint: Severe community-acquired pneumonia

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Severe community-acquired pneumonia

HISTORY OF PRESENT ILLNESS:
The patient is a 65-year-old man with a history of COPD GOLD Stage IV, pulmonary hypertension, and prior admissions in January and March 2020 for opportunistic pulmonary infection and hyperchloremic metabolic acidosis who returns to the intensive care unit with severe community-acquired pneumonia requiring mechanical ventilation. On hospital day 3, laboratory tests are performed to assess thyroid function due to persistent hypotension and hypothermia.

PAST MEDICAL HISTORY:
Active Problem List:
- Opportunistic pulmonary infection requiring diagnostic sampling (diagnosed 2020-01-15)
- Hyperchloremic metabolic acidosis due to excess normal saline infusion (diagnosed 2020-03-15)
- Chronic obstructive pulmonary disease (GOLD Stage IV)
- Pulmonary hypertension
- Type 2 diabetes mellitus
- Hypertension
- Gastroesophageal reflux disease

PAST SURGICAL HISTORY:
- Left knee arthroscopy (2005)
- Cataract extraction, bilateral (2020)

MEDICATIONS:
Home Medications:
- Albuterol sulfate HFA 90 mcg, 2 inhalations every 4 hours as needed
- Tiotropium bromide 18 mcg inhalation capsule daily
- Fluticasone propionate/Salmeterol 250/50 mcg, 1 inhalation twice daily
- Home oxygen therapy 2 liters per minute via nasal cannula, continuous
- Lisinopril 10 mg oral daily
- Metformin 1000 mg oral twice daily
- Omeprazole 20 mg oral daily

ALLERGIES:
Penicillin (hives), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Father: Lung cancer, chronic bronchitis
- Mother: Type 2 diabetes mellitus
- Older brother: COPD

SOCIAL HISTORY:
Smoking: Former smoker, 50 pack-year history, quit 3 years ago. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired diesel mechanic.

LABORATORY DATA:
Laboratory studies show:
Thyroid-stimulating hormone (TSH) Slightly low
Total T4 Low-normal
Total T3: Low
Reverse T3 Elevated
