# synthetic_hospital patient 2214

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 62 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-08-28 (Thu) | icu | Pulmonary & Critical Care |
| 1 | 2020-05-15 | 2026-01-08 (Thu) | icu | Pulmonary & Critical Care |
| 2 | 2020-10-15 | 2026-05-27 (Wed) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield (PPO)",
 "occupation": "Retired truck driver",
 "alcohol_use": "None",
 "family_history": [
  "Father: Myocardial infarction at age 64, hypertension",
  "Mother: COPD, hypertension",
  "Brother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 42 pack-year history, quit 2 months ago following myocardial infarction",
 "home_medications": [
  {
   "dose": "90 mcg inhaler, 2 puffs every 6 hours as needed for shortness of breath",
   "name": "Albuterol"
  },
  {
   "dose": "18 mcg inhaled capsule daily",
   "name": "Tiotropium"
  },
  {
   "dose": "250 mcg inhaled twice daily",
   "name": "Fluticasone propionate"
  },
  {
   "dose": "50 mg oral daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "10 mg oral daily",
   "name": "Lisinopril"
  }
 ],
 "surgical_history": [
  "Cardiac catheterization with drug-eluting stent placement to LAD (2 months ago)",
  "Tonsillectomy (childhood)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD), severe (GOLD Group D)",
  "Hypertension",
  "Coronary artery disease - status post recent myocardial infarction (2 months prior)",
  "Status post right lower lobe pneumonia (3 weeks prior, treated as outpatient)"
 ],
 "age_at_first_encounter": 62
}
```

Primary diagnoses: ["Euthyroid sick syndrome", "Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia", "Tension pneumothorax"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Acute hypoxemic respiratory failure without hypercapnic encephalopathy", "Dependence on mechanical ventilation"]

## Encounter 0: source 2020-01-15, practice 2025-08-28, icu, Pulmonary & Critical Care

Chief complaint: Acute hypoxic respiratory failure requiring intubation and mechanical ventilation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Acute hypoxic respiratory failure requiring intubation and mechanical ventilation

HISTORY OF PRESENT ILLNESS:
He was intubated and mechanically ventilated due to worsening hypoxemia and respiratory distress. His initial ventilator settings were a tidal volume of 450 mL and a respiratory rate of 20 breaths per minute.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Group D)
- Hypertension
- Coronary artery disease - status post recent myocardial infarction (2 months prior)
- Status post right lower lobe pneumonia (3 weeks prior, treated as outpatient)

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement to LAD (2 months ago)
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Albuterol 90 mcg inhaler, 2 puffs every 6 hours as needed for shortness of breath
- Tiotropium 18 mcg inhaled capsule daily
- Fluticasone propionate 250 mcg inhaled twice daily
- Metoprolol succinate 50 mg oral daily
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Lisinopril 10 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64, hypertension
- Mother: COPD, hypertension
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 42 pack-year history, quit 2 months ago following myocardial infarction. Alcohol: None. Occupation: Retired truck driver.

PHYSICAL EXAMINATION:
The patient is sedated but follows commands and is hemodynamically stable. His current ventilator settings are a tidal volume of 450 mL and a respiratory rate of 24 breaths per minute.

LABORATORY DATA:
On day 2 of mechanical ventilation, his arterial blood gas (ABG) results are as follows:

- pH: 7.52
- PaCO2: 28 mmHg
- PaO2: 85 mmHg
- HCO3-: 24 mEq/L
- SaO2: 96%

## Encounter 1: source 2020-05-15, practice 2026-01-08, icu, Pulmonary & Critical Care

Chief complaint: Acute respiratory failure with sudden cardiovascular collapse while on mechanical ventilation

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Acute respiratory failure with sudden cardiovascular collapse while on mechanical ventilation

HISTORY OF PRESENT ILLNESS:
The 62-year-old male with severe COPD (GOLD D), recent myocardial infarction, and pneumonia three weeks ago presents with acute respiratory failure following prior ICU admission in January 2020 for hypoxic respiratory failure requiring intubation. He is intubated and placed on mechanical ventilation with high positive end-expiratory pressure (PEEP) due to severe hypoxemia. On the second day of mechanical ventilation, the patient suddenly develops severe shortness of breath and becomes hypotensive at 85/50 mmHg. His heart rate increases to 118 bpm. His oxygen saturation drops to 85% despite receiving 100% oxygen.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Group D)
- Hypertension
- Coronary artery disease - status post recent myocardial infarction (2 months prior)
- Status post right lower lobe pneumonia (3 weeks prior, treated as outpatient)

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement to LAD (2 months ago)
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Albuterol 90 mcg inhaler, 2 puffs every 6 hours as needed for shortness of breath
- Tiotropium 18 mcg inhaled capsule daily
- Fluticasone propionate 250 mcg inhaled twice daily
- Metoprolol succinate 50 mg oral daily
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Lisinopril 10 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64, hypertension
- Mother: COPD, hypertension
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 42 pack-year history, quit 2 months ago following myocardial infarction. Alcohol: None. Occupation: Retired truck driver.

VITAL SIGNS:
Blood pressure of 85/50 mmHg. His heart rate increases to 118 bpm. The patient's oxygen saturation drops to 85% despite receiving 100% oxygen.

PHYSICAL EXAMINATION:
Physical examination reveals tracheal deviation to the left, decreased breath sounds on the right side, and hyperresonance to percussion over the right hemithorax.

PLAN:
A stat chest X-ray is ordered.

## Encounter 2: source 2020-10-15, practice 2026-05-27, icu, Pulmonary & Critical Care

Chief complaint: Acute hypoxic respiratory failure with persistent low energy levels

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Priya Patel, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Acute hypoxic respiratory failure with persistent low energy levels

HISTORY OF PRESENT ILLNESS:
This 62-year-old male with severe COPD (GOLD Group D), recent myocardial infarction, and pneumonia three weeks prior returns to the intensive care unit following prior admissions. He is admitted for acute hypoxic respiratory failure and is intubated and mechanically ventilated. On hospital day 3, he remains hemodynamically stable on low-dose norepinephrine and is afebrile. The patient's wife inquires about his low energy level and need for thyroid medication.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia (diagnosed 2020-01-15)
- Tension pneumothorax (diagnosed 2020-05-15)
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Group D)
- Hypertension
- Coronary artery disease - status post recent myocardial infarction (2 months prior)
- Status post right lower lobe pneumonia (3 weeks prior, treated as outpatient)

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement to LAD (2 months ago)
- Tonsillectomy (childhood)

MEDICATIONS:
He takes inhaled bronchodilators, intravenous corticosteroids, and metoprolol for blood pressure control.

Home Medications:
- Albuterol 90 mcg inhaler, 2 puffs every 6 hours as needed for shortness of breath
- Tiotropium 18 mcg inhaled capsule daily
- Fluticasone propionate 250 mcg inhaled twice daily
- Metoprolol succinate 50 mg oral daily
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Lisinopril 10 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64, hypertension
- Mother: COPD, hypertension
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 42 pack-year history, quit 2 months ago following myocardial infarction. Alcohol: None. Occupation: Retired truck driver.

LABORATORY DATA:
Serum TSH: 2.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 1.2 ng/dL (normal range: 0.9-1.7 ng/dL)
- Free T3: 1.9 pg/mL (normal range: 2.3-4.2 pg/mL)
