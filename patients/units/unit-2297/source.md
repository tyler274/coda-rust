# synthetic_hospital patient 2297

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-11 (Wed) | ed | Emergency Medicine |
| 1 | 2020-05-15 | 2025-04-08 (Tue) | icu | Pulmonary & Critical Care |
| 2 | 2020-11-15 | 2025-10-02 (Thu) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "Social, 1-2 beers on weekends",
 "family_history": [
  "Father: Myocardial infarction at age 62, deceased",
  "Mother: Chronic obstructive pulmonary disease, alive",
  "One older brother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 35 pack-year history, quit 3 months ago following myocardial infarction",
 "home_medications": [
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "50 mg oral daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "10 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium bromide"
  },
  {
   "dose": "250/50 mcg inhaled twice daily",
   "name": "Fluticasone propionate/salmeterol"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath",
   "name": "Albuterol sulfate"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention with drug-eluting stent placement to left anterior descending artery (3 months prior)",
  "Diagnostic cardiac catheterization (3 months prior)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage II",
  "Coronary artery disease with recent myocardial infarction (3 months prior)",
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Accidental arterial cannulation during central venous catheter placement", "Acute hypoxemic respiratory failure without hypercapnic encephalopathy", "Bacterial meningitis due to Streptococcus pneumoniae"]
Comorbidities: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Chronic obstructive pulmonary disease"]

## Encounter 0: source 2020-01-15, practice 2024-12-11, ed, Emergency Medicine

Chief complaint: Severe headache, high-grade fever, and confusion

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe headache, high-grade fever, and confusion

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department with a 2-day history of severe headache, high-grade fever, and confusion. His wife reports that he has been less responsive and has been complaining of neck stiffness.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage II
- Coronary artery disease with recent myocardial infarction (3 months prior)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement to left anterior descending artery (3 months prior)
- Diagnostic cardiac catheterization (3 months prior)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Lisinopril 10 mg oral daily
- Tiotropium bromide 18 mcg inhaled daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, deceased
- Mother: Chronic obstructive pulmonary disease, alive
- One older brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 35 pack-year history, quit 3 months ago following myocardial infarction. Alcohol: Social, 1-2 beers on weekends. Occupation: Retired postal worker.

VITAL SIGNS:
Febrile with a temperature of 39.2°C (102.5°F), heart rate is 110 bpm, and blood pressure is 130/85 mmHg

PHYSICAL EXAMINATION:
He is disoriented to time and place. Neck flexion elicits resistance and pain. Lung auscultation reveals decreased breath sounds in the right lower lobe.

LABORATORY DATA:
White blood cell count: 15,000 /µL (normal: 4,500-11,000 /µL)
Blood glucose: 150 mg/dL (normal: 70-99 mg/dL)
Cerebrospinal fluid (CSF) analysis:
Appearance: Cloudy
WBC: 2,000 /µL (normal: 0-5 /µL)
Protein: 180 mg/dL (normal: 15-45 mg/dL)
Glucose: 30 mg/dL (normal: 45-80 mg/dL)
Gram stain: Gram-positive cocci in pairs

## Encounter 1: source 2020-05-15, practice 2025-04-08, icu, Pulmonary & Critical Care

Chief complaint: Evaluation for extubation following mechanical ventilation for acute hypoxemic respiratory failure

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Marcus Rodriguez, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Evaluation for extubation following mechanical ventilation for acute hypoxemic respiratory failure

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with COPD GOLD Stage II, recent myocardial infarction, and prior Streptococcus pneumoniae meningitis presents following ICU admission. He was intubated 5 days ago due to acute hypoxemic respiratory failure and has since been on mechanical ventilation. His current ventilator settings are pressure support ventilation with 5 cm H2O pressure support and 5 cm H2O positive end-expiratory pressure (PEEP).

PAST MEDICAL HISTORY:
Active Problem List:
- Bacterial meningitis due to Streptococcus pneumoniae (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage II
- Coronary artery disease with recent myocardial infarction (3 months prior)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement to left anterior descending artery (3 months prior)
- Diagnostic cardiac catheterization (3 months prior)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Lisinopril 10 mg oral daily
- Tiotropium bromide 18 mcg inhaled daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, deceased
- Mother: Chronic obstructive pulmonary disease, alive
- One older brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 35 pack-year history, quit 3 months ago following myocardial infarction. Alcohol: Social, 1-2 beers on weekends. Occupation: Retired postal worker.

VITAL SIGNS:
Temperature 37.2°C, blood pressure 135/85 mmHg, heart rate 88 beats per minute, and oxygen saturation 94% on the current ventilator settings

PHYSICAL EXAMINATION:
The patient is alert, follows commands, and is able to lift his head off the pillow for 5 seconds. His respiratory rate is 16 breaths per minute, and he has a strong cough.

LABORATORY DATA:
His arterial blood gas (ABG) on these settings 30 minutes ago showed a pH of 7.32, PaCO2 of 45 mmHg, and PaO2 of 78 mmHg on an FiO2 of 40%.

## Encounter 2: source 2020-11-15, practice 2025-10-02, icu, Pulmonary & Critical Care

Chief complaint: Septic shock secondary to pneumonia requiring vasopressor support and central venous catheter placement

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. Priya Patel, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Septic shock secondary to pneumonia requiring vasopressor support and central venous catheter placement

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with COPD GOLD Stage II and recent myocardial infarction, previously admitted for respiratory failure in May 2020, requires vasopressor support with norepinephrine and is scheduled for central venous catheter (CVC) placement for administration of multiple medications. During the procedure, the patient is under ultrasound guidance, but the clinician encounters difficulty advancing the catheter and notes bright red, brisk-flowing blood upon aspiration.

PAST MEDICAL HISTORY:
Active Problem List:
- Bacterial meningitis due to Streptococcus pneumoniae (diagnosed 2020-01-15)
- Acute hypoxemic respiratory failure without hypercapnic encephalopathy (diagnosed 2020-05-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage II
- Coronary artery disease with recent myocardial infarction (3 months prior)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement to left anterior descending artery (3 months prior)
- Diagnostic cardiac catheterization (3 months prior)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Lisinopril 10 mg oral daily
- Tiotropium bromide 18 mcg inhaled daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, deceased
- Mother: Chronic obstructive pulmonary disease, alive
- One older brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 35 pack-year history, quit 3 months ago following myocardial infarction. Alcohol: Social, 1-2 beers on weekends. Occupation: Retired postal worker.

VITAL SIGNS:
The patient's blood pressure is 90/60 mmHg, heart rate is 110 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on 2 liters of oxygen via nasal cannula

LABORATORY DATA:
An arterial blood gas (ABG) is immediately obtained, showing a pH of 7.45, PaCO2 of 35 mmHg, PaO2 of 95 mmHg, and HCO3- of 24 mEq/L.
