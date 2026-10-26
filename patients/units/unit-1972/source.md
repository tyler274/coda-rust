# synthetic_hospital patient 1972

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex M, Non-Hispanic White, Medicare (disability).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2026-07-06 (Mon) | ed | Emergency Medicine |
| 1 | 2020-01-15 | 2026-07-06 (Mon) | icu | Pulmonary & Critical Care |
| 2 | 2020-01-15 | 2026-07-06 (Mon) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (disability)",
 "occupation": "Construction worker",
 "alcohol_use": "Occasional",
 "family_history": [
  "Father with chronic obstructive pulmonary disease",
  "Mother with hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 35 pack-year history",
 "home_medications": [
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  },
  {
   "dose": "90 mcg inhaled PRN",
   "name": "Albuterol"
  },
  {
   "dose": "5 mg daily",
   "name": "Amlodipine"
  }
 ],
 "surgical_history": [
  "Appendectomy"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease",
  "Hypertension"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Refractory hypoxemia in ARDS", "Complicated parapneumonic effusion"]
Comorbidities: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Chronic obstructive pulmonary disease", "Sepsis"]

## Encounter 0: source 2020-01-15, practice 2026-07-06, ed, Emergency Medicine

Chief complaint: Increasing shortness of breath, productive cough with yellowish sputum, and pleuritic chest pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing shortness of breath, productive cough with yellowish sputum, and pleuritic chest pain

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department with a 5-day history of increasing shortness of breath, productive cough with yellowish sputum, and pleuritic chest pain. He reports a low-grade fever at home.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease
- Hypertension

PAST SURGICAL HISTORY:
- Appendectomy

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled PRN
- Amlodipine 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with chronic obstructive pulmonary disease
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Current smoker, 35 pack-year history. Alcohol: Occasional. Occupation: Construction worker.

VITAL SIGNS:
His temperature is 38.3°C, blood pressure is 135/85 mmHg, heart rate is 102 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals decreased breath sounds and dullness to percussion at the right lung base.

LABORATORY DATA:
Thoracentesis is performed, and pleural fluid analysis reveals the following:

- pH: 7.1
- Glucose: 40 mg/dL
- Leukocyte count: 60,000/mm3 (90% neutrophils)
- Gram stain: Gram-positive cocci in clusters

IMAGING:
Chest X-ray shows a right-sided pleural effusion.

## Encounter 1: source 2020-01-15, practice 2026-07-06, icu, Pulmonary & Critical Care

Chief complaint: Severe pneumonia with progression to hypoxemic respiratory failure requiring mechanical ventilation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Marcus Johnson, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Severe pneumonia with progression to hypoxemic respiratory failure requiring mechanical ventilation

HISTORY OF PRESENT ILLNESS:
This 52-year-old male with chronic obstructive pulmonary disease and hypertension, initially evaluated earlier today in the Emergency Department for complicated parapneumonic effusion, is intubated and mechanical ventilation is initiated due to hypoxemic respiratory failure. His oxygenation remains poor despite high levels of inspired oxygen (FiO2) and positive end-expiratory pressure (PEEP). His arterial blood gas (ABG) shows a PaO2/FiO2 ratio of 150, indicating moderate ARDS.

PAST MEDICAL HISTORY:
Active Problem List:
- Complicated parapneumonic effusion (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease
- Hypertension

PAST SURGICAL HISTORY:
- Appendectomy

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled PRN
- Amlodipine 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with chronic obstructive pulmonary disease
- Mother with hypertension

SOCIAL HISTORY:
Smoking

VITAL SIGNS:
The patient's current ventilator settings are a tidal volume of 10 mL/kg of predicted body weight, a respiratory rate of 16 breaths per minute, and a PEEP of 8 cm H2O. His current vital signs are: BP 110/70 mmHg, HR 102 bpm, RR (set on ventilator) 16/min, and SpO2 88%.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral crackles on lung auscultation

LABORATORY DATA:
His most recent laboratory values are:

- pH: 7.32
- PaCO2: 50 mmHg
- PaO2: 68 mmHg
- HCO3-: 24 mEq/L
- Lactate: 2.2 mmol/L

IMAGING:
Diffuse bilateral infiltrates on chest X-ray

ASSESSMENT AND PLAN:
Given the patient's ARDS diagnosis and current condition,

## Encounter 2: source 2020-01-15, practice 2026-07-06, icu, Pulmonary & Critical Care

Chief complaint: Refractory hypoxemia despite mechanical ventilation with high FiO2 and PEEP

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Marcus Johnson, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Refractory hypoxemia despite mechanical ventilation with high FiO2 and PEEP

HISTORY OF PRESENT ILLNESS:
This 52-year-old male with chronic obstructive pulmonary disease and hypertension presents with continued compromise following earlier presentation with complicated parapneumonic effusion and progression to respiratory failure requiring mechanical ventilation. He is intubated and mechanically ventilated. His current ventilator settings are FiO2 of 80%, positive end-expiratory pressure (PEEP) of 12 cm H2O, and a tidal volume of 6 mL/kg of ideal body weight. Despite these measures, his oxygenation remains poor, with a PaO2/FiO2 ratio of 150. His urine output has decreased to 20 mL/hr over the past 6 hours. The patient's weight has increased by 3 kg since admission 2 days ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Complicated parapneumonic effusion (diagnosed 2020-01-15)
- Refractory hypoxemia in ARDS (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease
- Hypertension

PAST SURGICAL HISTORY:
- Appendectomy

MEDICATIONS:
He takes intravenous ceftriaxone, azithromycin, and norepinephrine for blood pressure support.

Home Medications:
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled PRN
- Amlodipine 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with chronic obstructive pulmonary disease
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Current smoker, 35 pack-year history. Alcohol: Occasional. Occupation: Construction worker.

VITAL SIGNS:
BP 95/60 mmHg, HR 110 bpm, RR 28 breaths/min (ventilator-controlled), and temperature 38.3°C

PHYSICAL EXAMINATION:
Bilateral crackles on lung auscultation and 2+ pitting edema in the lower extremities

LABORATORY DATA:
- Serum creatinine: 1.8 mg/dL (baseline 1.0 mg/dL)
- Blood urea nitrogen (BUN): 36 mg/dL
- Serum sodium: 145 mEq/L
- Serum potassium: 4.5 mEq/L
- Hemoglobin: 9.8 g/dL
- White blood cell count: 18,000/mm^3
- Platelets: 250,000/mm^3
- Arterial blood gas (ABG): pH 7.32, PaCO2 45 mmHg, PaO2 68 mmHg, HCO3- 22 mEq/L

IMAGING:
Chest X-ray shows diffuse bilateral infiltrates
