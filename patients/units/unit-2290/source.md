# synthetic_hospital patient 2290

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-01 (Sun) | inpatient | Internal Medicine |
| 1 | 2020-07-15 | 2025-06-07 (Sat) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2025-12-04 (Thu) | inpatient | Pulmonology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Occasional social use (1-2 beers per week)",
 "family_history": [
  "Father: Myocardial infarction at age 71, hyperlipidemia, hypertension",
  "Mother: Severe emphysema, died at age 80",
  "Brother: Gout, hyperlipidemia",
  "Sister: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 42 pack-year history, quit 2 years ago",
 "home_medications": [
  {
   "dose": "40 mg PO daily (started 3 weeks prior)",
   "name": "Atorvastatin"
  },
  {
   "dose": "0.6 mg PO daily for gout prophylaxis (started 3 weeks prior)",
   "name": "Colchicine"
  },
  {
   "dose": "0.2 mg PO twice daily (adherence inconsistent; missed several doses in past week)",
   "name": "Desmopressin acetate"
  },
  {
   "dose": "18 mcg inhaled once daily",
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
  "Open reduction and internal fixation of right femoral neck fracture (2019)",
  "Bilateral cataract extraction with intraocular lens placement (2021)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) stage 2",
  "Hyperlipidemia",
  "Chronic gout",
  "Central diabetes insipidus (post-traumatic, following motor vehicle accident with mild traumatic brain injury 5 years prior)"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Rhabdomyolysis requiring fluid resuscitation", "Hypernatremia or free water deficit (maintenance fluid indication)", "Bacterial pneumonia"]
Comorbidities: ["Acute kidney injury"]

## Encounter 0: source 2020-01-15, practice 2024-12-01, inpatient, Internal Medicine

Chief complaint: 2-day history of generalized muscle pain, weakness, and dark urine

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
2-day history of generalized muscle pain, weakness, and dark urine

HISTORY OF PRESENT ILLNESS:
He reports no recent trauma or vigorous exercise.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) stage 2
- Hyperlipidemia
- Chronic gout
- Central diabetes insipidus (post-traumatic, following motor vehicle accident with mild traumatic brain injury 5 years prior)

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right femoral neck fracture (2019)
- Bilateral cataract extraction with intraocular lens placement (2021)

MEDICATIONS:
He takes atorvastatin and colchicine, which were both started 3 weeks ago.

Home Medications:
- Desmopressin acetate 0.2 mg PO twice daily (adherence inconsistent; missed several doses in past week)
- Tiotropium bromide 18 mcg inhaled once daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 71, hyperlipidemia, hypertension
- Mother: Severe emphysema, died at age 80
- Brother: Gout, hyperlipidemia
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 42 pack-year history, quit 2 years ago. Alcohol: Occasional social use (1-2 beers per week). Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On examination, he appears dehydrated and his muscle strength is 4/5 in all major muscle groups. There is no muscle tenderness on palpation. His urine is tea-colored.

LABORATORY DATA:
Serum creatinine: 2.8 mg/dL (baseline 1.0 mg/dL)
- Serum potassium: 5.5 mEq/L
- Serum calcium: 8.9 mg/dL
- Serum phosphorus: 6.2 mg/dL
- Serum CK (creatine kinase): 20,000 U/L
- Urinalysis: Dark yellow color, no red blood cells, 2+ blood

## Encounter 1: source 2020-07-15, practice 2025-06-07, ed, Emergency Medicine

Chief complaint: Altered mental status

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Altered mental status

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with central diabetes insipidus and prior rhabdomyolysis who returns with altered mental status. His daughter reports that he has been less responsive and increasingly lethargic over the past two days. She also mentions that he has been taking his desmopressin regularly and has been drinking less water than usual.

PAST MEDICAL HISTORY:
Active Problem List:
- Rhabdomyolysis requiring fluid resuscitation (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) stage 2
- Hyperlipidemia
- Chronic gout
- Central diabetes insipidus (post-traumatic, following motor vehicle accident with mild traumatic brain injury 5 years prior)

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right femoral neck fracture (2019)
- Bilateral cataract extraction with intraocular lens placement (2021)

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily (started 3 weeks prior)
- Colchicine 0.6 mg PO daily for gout prophylaxis (started 3 weeks prior)
- Desmopressin acetate 0.2 mg PO twice daily (adherence inconsistent; missed several doses in past week)
- Tiotropium bromide 18 mcg inhaled once daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 71, hyperlipidemia, hypertension
- Mother: Severe emphysema, died at age 80
- Brother: Gout, hyperlipidemia
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 42 pack-year history, quit 2 years ago. Alcohol: Occasional social use (1-2 beers per week). Occupation: Retired construction foreman.

VITAL SIGNS:
Blood pressure 90/60 mmHg, heart rate 110 bpm, respiratory rate 24 breaths/min, and temperature 37.2°C (99.0°F)

PHYSICAL EXAMINATION:
On examination, the patient is disoriented, tachypneic, and has dry mucous membranes.
The patient's urine output is noted to be low.

LABORATORY DATA:
Laboratory results are:

- Sodium: 165 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 4.2 mEq/L (Normal: 3.5-5.1 mEq/L)
- Chloride: 128 mEq/L (Normal: 98-107 mEq/L)
- Bicarbonate: 24 mEq/L (Normal: 22-29 mEq/L)
- Blood urea nitrogen (BUN): 35 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 1.2 mg/dL (Normal: 0.6-1.2 mg/dL)
- Glucose: 180 mg/dL (Normal: 70-99 mg/dL)

## Encounter 2: source 2021-01-15, practice 2025-12-04, inpatient, Pulmonology

Chief complaint: 4-day history of fever, productive cough, and shortness of breath

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Pulmonology
Visit Type: inpatient

CHIEF COMPLAINT:
4-day history of fever, productive cough, and shortness of breath

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with COPD GOLD stage 2 and prior admissions for rhabdomyolysis and hypernatremia presents with respiratory symptoms. He reports a "flu-like" illness 10 days ago, which initially seemed to improve after a few days of rest and over-the-counter medications. However, over the past 48 hours, he has experienced a worsening of his symptoms, with a new high fever and increased yellowish sputum production. He denies chest pain, hemoptysis, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Rhabdomyolysis requiring fluid resuscitation (diagnosed 2020-01-15)
- Hypernatremia or free water deficit (maintenance fluid indication) (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) stage 2
- Hyperlipidemia
- Chronic gout
- Central diabetes insipidus (post-traumatic, following motor vehicle accident with mild traumatic brain injury 5 years prior)

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right femoral neck fracture (2019)
- Bilateral cataract extraction with intraocular lens placement (2021)

MEDICATIONS:
Home Medications:
- Atorvastatin 40 mg PO daily (started 3 weeks prior)
- Colchicine 0.6 mg PO daily for gout prophylaxis (started 3 weeks prior)
- Desmopressin acetate 0.2 mg PO twice daily (adherence inconsistent; missed several doses in past week)
- Tiotropium bromide 18 mcg inhaled once daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 71, hyperlipidemia, hypertension
- Mother: Severe emphysema, died at age 80
- Brother: Gout, hyperlipidemia
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 42 pack-year history, quit 2 years ago. Alcohol: Occasional social use (1-2 beers per week). Occupation: Retired construction foreman.

VITAL SIGNS:
Temperature is 39.2°C (102.5°F), blood pressure is 135/85 mmHg, pulse is 110 beats/min, and respiratory rate is 28 breaths/min. Oxygen saturation is 89% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds and coarse crackles in the right lower lobe.

LABORATORY DATA:
White blood cell count: 18,000/mm³ with left shift
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/mm³
- Blood urea nitrogen: 28 mg/dL
- Creatinine: 1.1 mg/dL
- Lactate dehydrogenase (LDH): 350 U/L (elevated)
- Blood cultures: Pending

IMAGING:
A chest radiograph shows a dense consolidation in the right lower lobe with evidence of cavitation.
