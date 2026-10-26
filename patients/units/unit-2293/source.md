# synthetic_hospital patient 2293

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-05 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2025-08-05 (Tue) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2025-11-06 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare with supplemental insurance",
 "occupation": "Retired postal worker",
 "alcohol_use": "Occasional social use (1-2 drinks per month)",
 "family_history": [
  "Father: Lung cancer, deceased age 72",
  "Mother: Hypertension and myocardial infarction at age 76",
  "Sibling: Chronic obstructive pulmonary disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "40 pack-year history; currently smokes 0.5 packs (10 cigarettes) per day",
 "home_medications": [
  {
   "dose": "18 mcg inhaled once daily",
   "name": "Tiotropium"
  },
  {
   "dose": "250/50 mcg inhaled twice daily",
   "name": "Fluticasone propionate/Salmeterol"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for dyspnea",
   "name": "Albuterol (salbutamol)"
  }
 ],
 "surgical_history": [
  "Inguinal hernia repair (age 55)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD), moderate to severe",
  "Tobacco use disorder, current"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Hyperchloremic metabolic acidosis due to excess normal saline infusion", "COPD exacerbation", "Atrioventricular Nodal Re-entrant Tachycardia"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Hypoxemia"]

## Encounter 0: source 2020-01-15, practice 2025-05-05, outpatient, Internal Medicine

Chief complaint: Routine follow-up for chronic obstructive pulmonary disease with increased shortness of breath over the past few months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic obstructive pulmonary disease with increased shortness of breath over the past few months

HISTORY OF PRESENT ILLNESS:
He reports increased shortness of breath over the past few months, particularly when climbing stairs.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD), moderate to severe
- Tobacco use disorder, current

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 55)

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Fluticasone propionate/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol (salbutamol) 90 mcg inhaled every 4-6 hours as needed for dyspnea

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Lung cancer, deceased age 72
- Mother: Hypertension and myocardial infarction at age 76
- Sibling: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
He has a 40-pack-year smoking history and continues to smoke half a pack of cigarettes per day.

PHYSICAL EXAMINATION:
On physical examination, you note a barrel-chested appearance, prolonged expiratory phase with wheezing, and use of accessory muscles for respiration.

LABORATORY DATA:
Arterial blood gas (ABG) analysis reveals the following:

- pH: 7.36
- PaCO2: 58 mmHg
- PaO2: 60 mmHg
- HCO3-: 32 mEq/L

The patient's renal function is within normal limits.

ASSESSMENT AND PLAN:
Based on the patient's chronic condition and ABG results,

## Encounter 1: source 2020-04-15, practice 2025-08-05, ed, Emergency Medicine

Chief complaint: 3-day history of increased shortness of breath, productive cough with greenish sputum, and low-grade fever

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-day history of increased shortness of breath, productive cough with greenish sputum, and low-grade fever

HISTORY OF PRESENT ILLNESS:
Returning three months after evaluation for dyspnea, this 67-year-old male with moderate to severe COPD reports a worsening of his usual dyspnea on exertion and has been unable to perform his daily activities.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperchloremic metabolic acidosis due to excess normal saline infusion (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), moderate to severe
- Tobacco use disorder, current

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 55)

MEDICATIONS:
He takes uses inhaled tiotropium daily.

Home Medications:
- Fluticasone propionate/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol (salbutamol) 90 mcg inhaled every 4-6 hours as needed for dyspnea

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Lung cancer, deceased age 72
- Mother: Hypertension and myocardial infarction at age 76
- Sibling: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
He has a 40-pack-year smoking history

VITAL SIGNS:
On examination, his temperature is 37.8°C (100°F), respiratory rate is 22 breaths/min, heart rate is 102 beats/min, and oxygen saturation is 89% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals diffuse wheezes and prolonged expiration.

LABORATORY DATA:
Arterial blood gas analysis on room air shows:
- pH: 7.36
- PaCO2: 50 mmHg
- PaO2: 60 mmHg
- HCO3-: 29 mEq/L

IMAGING:
Chest radiography shows no signs of consolidation, but hyperinflated lung fields are noted.

## Encounter 2: source 2020-07-15, practice 2025-11-06, ed, Emergency Medicine

Chief complaint: 2-day history of progressive shortness of breath and productive cough with yellow sputum

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of progressive shortness of breath and productive cough with yellow sputum

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with moderate to severe COPD and current tobacco use disorder presents to the emergency department with a 2-day history of progressive shortness of breath and productive cough with yellow sputum, representing recurrent respiratory symptoms following a COPD exacerbation in April 2020 and prior evaluation for progressive dyspnea in January 2020. He denies chest pain, fever, or leg swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperchloremic metabolic acidosis due to excess normal saline infusion (diagnosed 2020-01-15)
- COPD exacerbation (diagnosed 2020-04-15)
- Chronic obstructive pulmonary disease (COPD), moderate to severe
- Tobacco use disorder, current

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 55)

MEDICATIONS:
His medications include tiotropium, salbutamol as needed, and a fluticasone/salmeterol inhaler.

Home Medications:
- Fluticasone propionate/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol (salbutamol) 90 mcg inhaled every 4-6 hours as needed for dyspnea

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Lung cancer, deceased age 72
- Mother: Hypertension and myocardial infarction at age 76
- Sibling: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
Smoking: 40 pack-year history; currently smokes 0.5 packs (10 cigarettes) per day. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Retired postal worker.

VITAL SIGNS:
Respiratory rate of 28 breaths per minute, oxygen saturation is 89% on room air, heart rate is 130 beats per minute, and blood pressure is 145/85 mmHg

PHYSICAL EXAMINATION:
On examination, he is tachypneic and he is using accessory muscles to breathe. Lung auscultation reveals diffuse wheezes and crackles at the bases. His heart rate is irregularly irregular.

LABORATORY DATA:
Arterial blood gas on room air shows:
- pH: 7.32
- PaCO2: 55 mmHg
- PaO2: 60 mmHg
- HCO3-: 26 mEq/L
- SaO2: 88%

OTHER STUDIES:
An electrocardiogram (ECG) shows an irregular rhythm with at least four different P wave morphologies and no evidence of acute ischemia.
