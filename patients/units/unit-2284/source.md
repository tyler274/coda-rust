# synthetic_hospital patient 2284

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-23 (Wed) | outpatient | Neurology |
| 1 | 2020-05-15 | 2025-08-10 (Sun) | ed | Emergency Medicine |
| 2 | 2020-10-15 | 2026-01-11 (Sun) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction worker",
 "alcohol_use": "Heavy daily consumption (6-8 standard drinks); ongoing alcohol use disorder with physiological dependence",
 "family_history": [
  "Father: Type 2 diabetes mellitus, hypertension, stroke",
  "Mother: Chronic obstructive pulmonary disease, osteoporosis",
  "Brother: Alcohol use disorder",
  "Sister: Hyperlipidemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-year history, quit 2022",
 "home_medications": [
  {
   "dose": "30 units subcutaneously at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale with meals per carbohydrate ratio",
   "name": "Insulin lispro"
  },
  {
   "dose": "18 mcg inhaled once daily",
   "name": "Tiotropium"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for wheezing",
   "name": "Albuterol HFA"
  },
  {
   "dose": "10 mg orally once daily",
   "name": "Amlodipine"
  },
  {
   "dose": "40 mg orally once daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg orally once daily",
   "name": "Aspirin"
  },
  {
   "dose": "100 mg orally once daily",
   "name": "Thiamine"
  },
  {
   "dose": "1 mg orally once daily",
   "name": "Folic acid"
  }
 ],
 "surgical_history": [
  "Inguinal hernia repair (2015)",
  "Multiple dental extractions (3 weeks prior to initial presentation)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus, long-standing and poorly controlled",
  "Chronic kidney disease stage III",
  "Chronic obstructive pulmonary disease (COPD)",
  "Alcohol use disorder",
  "Hypertension",
  "Tobacco use disorder, in remission"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Metronidazole-induced polyneuropathy", "Complicated parapneumonic effusion", "Aminoglycoside ototoxicity"]
Comorbidities: ["Alcohol use disorder", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2025-04-23, outpatient, Neurology

Chief complaint: Progressive numbness and tingling in hands and feet with gait unsteadiness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive numbness and tingling in hands and feet with gait unsteadiness

HISTORY OF PRESENT ILLNESS:
He describes the sensation as "wearing gloves and socks all the time." He also reports mild unsteadiness while walking, which has worsened over the past few days. He denies any recent infections, trauma, or new medications except for a course of metronidazole for a dental infection he completed 3 days ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus, long-standing and poorly controlled
- Chronic kidney disease stage III
- Chronic obstructive pulmonary disease (COPD)
- Alcohol use disorder
- Hypertension
- Tobacco use disorder, in remission

PAST SURGICAL HISTORY:
- Inguinal hernia repair (2015)
- Multiple dental extractions (3 weeks prior to initial presentation)

MEDICATIONS:
Home Medications:
- Insulin glargine 30 units subcutaneously at bedtime
- Insulin lispro sliding scale with meals per carbohydrate ratio
- Tiotropium 18 mcg inhaled once daily
- Albuterol HFA 90 mcg inhaled every 4-6 hours as needed for wheezing
- Amlodipine 10 mg orally once daily
- Atorvastatin 40 mg orally once daily
- Aspirin 81 mg orally once daily
- Thiamine 100 mg orally once daily
- Folic acid 1 mg orally once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, stroke
- Mother: Chronic obstructive pulmonary disease, osteoporosis
- Brother: Alcohol use disorder
- Sister: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 2022. Alcohol: Heavy daily consumption (6-8 standard drinks); ongoing alcohol use disorder with physiological dependence. Occupation: Retired construction worker.

PHYSICAL EXAMINATION:
On physical examination, the patient has decreased sensation to light touch and pinprick in a stocking-glove distribution. Ankle reflexes are absent, and proprioception is diminished in the toes.

LABORATORY DATA:
Routine blood work is unremarkable. Fasting blood glucose is 154 mg/dL, and HbA1c is 7.8%.

## Encounter 1: source 2020-05-15, practice 2025-08-10, ed, Emergency Medicine

Chief complaint: Progressive dyspnea, nonproductive cough, pleuritic chest pain, and 10-pound weight loss

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Marcus Williams, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive dyspnea, nonproductive cough, pleuritic chest pain, and 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
This 66-year-old male with Type 2 diabetes mellitus, chronic kidney disease stage III, COPD, and metronidazole-induced polyneuropathy presents to the emergency department with a 3-week history of progressive dyspnea, nonproductive cough, and pleuritic chest pain. He also reports a 10-pound weight loss during this period. He denies fever or chills.

PAST MEDICAL HISTORY:
Active Problem List:
- Metronidazole-induced polyneuropathy (diagnosed 2020-01-15)
- Type 2 diabetes mellitus, long-standing and poorly controlled
- Chronic kidney disease stage III
- Chronic obstructive pulmonary disease (COPD)
- Alcohol use disorder
- Hypertension
- Tobacco use disorder, in remission

PAST SURGICAL HISTORY:
- Inguinal hernia repair (2015)
- Multiple dental extractions (3 weeks prior to initial presentation)

MEDICATIONS:
Home Medications:
- Insulin glargine 30 units subcutaneously at bedtime
- Insulin lispro sliding scale with meals per carbohydrate ratio
- Tiotropium 18 mcg inhaled once daily
- Albuterol HFA 90 mcg inhaled every 4-6 hours as needed for wheezing
- Amlodipine 10 mg orally once daily
- Atorvastatin 40 mg orally once daily
- Aspirin 81 mg orally once daily
- Thiamine 100 mg orally once daily
- Folic acid 1 mg orally once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, stroke
- Mother: Chronic obstructive pulmonary disease, osteoporosis
- Brother: Alcohol use disorder
- Sister: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 2022. Alcohol: Heavy daily consumption (6-8 standard drinks); ongoing alcohol use disorder with physiological dependence. Occupation: Retired construction worker.

VITAL SIGNS:
His temperature is 37.2°C (99°F), blood pressure is 130/85 mmHg, pulse is 102/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
Physical examination reveals decreased breath sounds and dullness to percussion at the right lung base.

LABORATORY DATA:
Thoracentesis yields cloudy pleural fluid with the following characteristics:

- pH: 7.20
- Glucose: 30 mg/dL
- LDH: 600 U/L (serum LDH 300 U/L)
- Total protein: 4.5 g/dL (serum total protein 6.5 g/dL)
- WBC: 20,000/mm³ with 80% neutrophils
- Gram stain: No organisms seen
- Culture: Pending

The pleural fluid has a putrid odor.

IMAGING:
Chest radiography shows a right-sided pleural effusion.

## Encounter 2: source 2020-10-15, practice 2026-01-11, inpatient, Internal Medicine

Chief complaint: Diabetic foot infection with fever, leukocytosis, and purulent drainage from right foot ulcer

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Diabetic foot infection with fever, leukocytosis, and purulent drainage from right foot ulcer

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old male with long-standing poorly controlled Type 2 diabetes mellitus, chronic kidney disease stage III, and prior metronidazole-induced polyneuropathy, who presents with fever, leukocytosis, and purulent drainage from a 2-cm ulcer overlying the first metatarsal head. On hospital day 2, he reports mild tinnitus and a sensation of unsteadiness when walking to the bathroom.

PAST MEDICAL HISTORY:
Active Problem List:
- Metronidazole-induced polyneuropathy (diagnosed 2020-01-15)
- Complicated parapneumonic effusion (diagnosed 2020-05-15)
- Type 2 diabetes mellitus, long-standing and poorly controlled
- Chronic kidney disease stage III
- Chronic obstructive pulmonary disease (COPD)
- Alcohol use disorder
- Hypertension
- Tobacco use disorder, in remission

He has a long-standing history of poorly controlled type 2 diabetes mellitus and chronic kidney disease stage III.

PAST SURGICAL HISTORY:
- Inguinal hernia repair (2015)
- Multiple dental extractions (3 weeks prior to initial presentation)

MEDICATIONS:
The inpatient team initiates combination therapy with piperacillin-tazobactam and gentamicin while awaiting antibiotic susceptibility results.

Home Medications:
- Insulin glargine 30 units subcutaneously at bedtime
- Insulin lispro sliding scale with meals per carbohydrate ratio
- Tiotropium 18 mcg inhaled once daily
- Albuterol HFA 90 mcg inhaled every 4-6 hours as needed for wheezing
- Amlodipine 10 mg orally once daily
- Atorvastatin 40 mg orally once daily
- Aspirin 81 mg orally once daily
- Thiamine 100 mg orally once daily
- Folic acid 1 mg orally once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, stroke
- Mother: Chronic obstructive pulmonary disease, osteoporosis
- Brother: Alcohol use disorder
- Sister: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 2022. Alcohol: Heavy daily consumption (6-8 standard drinks); ongoing alcohol use disorder with physiological dependence. Occupation: Retired construction worker.

LABORATORY DATA:
Wound cultures reveal a multidrug-resistant Pseudomonas aeruginosa .

ASSESSMENT AND PLAN:
The team suspects a drug-related adverse effect based on his recent medication changes.

The gentamicin is discontinued, and supportive care is provided.
