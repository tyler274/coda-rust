# synthetic_hospital patient 2312

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 74 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-03-27 (Mon) | outpatient | Cardiology |
| 1 | 2021-07-15 | 2024-09-29 (Sun) | ed | Emergency Medicine |
| 2 | 2023-01-15 | 2026-03-29 (Sun) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Occasional, 1-2 drinks per month",
 "family_history": [
  "Father: Lung cancer, deceased age 68",
  "Mother: Chronic obstructive pulmonary disease",
  "Sister: Rheumatoid arthritis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 50 pack-years, quit 15 years ago",
 "home_medications": [
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  },
  {
   "dose": "250/50 mcg inhaled twice daily",
   "name": "Fluticasone propionate/Salmeterol"
  },
  {
   "dose": "90 mcg inhaled every 4 hours as needed for wheezing",
   "name": "Albuterol HFA"
  },
  {
   "dose": "15 mg orally once weekly",
   "name": "Methotrexate"
  },
  {
   "dose": "1 mg orally daily",
   "name": "Folic acid"
  },
  {
   "dose": "5 mg orally daily",
   "name": "Prednisone"
  }
 ],
 "surgical_history": [
  "Total knee arthroplasty, left (2018)",
  "Appendectomy (1965)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease",
  "Rheumatoid arthritis",
  "History of pulmonary tuberculosis (resolved childhood infection)"
 ],
 "age_at_first_encounter": 74
}
```

Primary diagnoses: ["Non-small cell lung cancer, right upper lobe", "Bronchiectasis", "Pulmonary Hypertension"]
Comorbidities: ["History of recent pneumonia"]

## Encounter 0: source 2020-01-15, practice 2023-03-27, outpatient, Cardiology

Chief complaint: Increasing shortness of breath and fatigue over the past several weeks

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vargas, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing shortness of breath and fatigue over the past several weeks

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease
- Rheumatoid arthritis
- History of pulmonary tuberculosis (resolved childhood infection)

He has a long-standing history of chronic obstructive pulmonary disease and was hospitalized twice in the past year for exacerbations requiring steroids and bronchodilators.

PAST SURGICAL HISTORY:
- Total knee arthroplasty, left (2018)
- Appendectomy (1965)

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone propionate/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol HFA 90 mcg inhaled every 4 hours as needed for wheezing
- Methotrexate 15 mg orally once weekly
- Folic acid 1 mg orally daily
- Prednisone 5 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 68
- Mother: Chronic obstructive pulmonary disease
- Sister: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, 50 pack-years, quit 15 years ago. Alcohol: Occasional, 1-2 drinks per month. Occupation: Retired construction foreman.

VITAL SIGNS:
His temperature is 36.8°C (98.2°F), blood pressure is 118/74 mm Hg, pulse is 96/min, and respirations are 60/min.

PHYSICAL EXAMINATION:
He is using accessory muscles to breathe. Cardiac examination is normal. Pulmonary examination reveals diffuse wheezing and prolonged expiratory phase.

OTHER STUDIES:
ECG findings are shown below.

## Encounter 1: source 2021-07-15, practice 2024-09-29, ed, Emergency Medicine

Chief complaint: 2-week history of progressively worsening cough with large amounts of green sputum and exhaustion after coughing fits

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-week history of progressively worsening cough with large amounts of green sputum and exhaustion after coughing fits

HISTORY OF PRESENT ILLNESS:
This 75-year-old male with COPD and prior tuberculosis presents with worsening symptoms following his 2020 Cardiology evaluation. He reports producing large amounts of green sputum every day and feeling exhausted after coughing fits. He denies fever, chills, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Pulmonary Hypertension (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease
- Rheumatoid arthritis
- History of pulmonary tuberculosis (resolved childhood infection)

His past medical history is significant for rheumatoid arthritis and remote tuberculosis in childhood.

PAST SURGICAL HISTORY:
- Total knee arthroplasty, left (2018)
- Appendectomy (1965)

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone propionate/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol HFA 90 mcg inhaled every 4 hours as needed for wheezing
- Methotrexate 15 mg orally once weekly
- Folic acid 1 mg orally daily
- Prednisone 5 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 68
- Mother: Chronic obstructive pulmonary disease
- Sister: Rheumatoid arthritis

SOCIAL HISTORY:
He quit smoking 15 years ago after a 40 pack-year history.

VITAL SIGNS:
His temperature is 36.8°C (98.2°F), blood pressure is 128/76 mm Hg, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
On examination, he has coarse crackles and wheezing in both lung bases. There is no clubbing or peripheral edema.

LABORATORY DATA:
Sputum culture reveals Haemophilus influenzae .

IMAGING:
A chest x-ray shows peribronchial thickening and increased interstitial markings at the lower lobes bilaterally.

## Encounter 2: source 2023-01-15, practice 2026-03-29, inpatient, Internal Medicine

Chief complaint: Persistent cough, unintentional weight loss, fatigue, and episodes of confusion with difficulty concentrating over the past few weeks

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Patel, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Persistent cough, unintentional weight loss, fatigue, and episodes of confusion with difficulty concentrating over the past few weeks

HISTORY OF PRESENT ILLNESS:
The 77-year-old male with chronic obstructive pulmonary disease, rheumatoid arthritis, and resolved childhood pulmonary tuberculosis returns for evaluation following prior presentations for dyspnea in 2020 and productive cough in 2021. He also reports episodes of confusion and difficulty concentrating over the past several weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Pulmonary Hypertension (diagnosed 2020-01-15)
- Bronchiectasis (diagnosed 2021-07-15)
- Chronic obstructive pulmonary disease
- Rheumatoid arthritis
- History of pulmonary tuberculosis (resolved childhood infection)

PAST SURGICAL HISTORY:
- Total knee arthroplasty, left (2018)
- Appendectomy (1965)

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone propionate/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol HFA 90 mcg inhaled every 4 hours as needed for wheezing
- Methotrexate 15 mg orally once weekly
- Folic acid 1 mg orally daily
- Prednisone 5 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 68
- Mother: Chronic obstructive pulmonary disease
- Sister: Rheumatoid arthritis

SOCIAL HISTORY:
50-pack-year smoking history

PHYSICAL EXAMINATION:
Physical examination reveals no focal neurologic deficits.

LABORATORY DATA:
Serum sodium is 130 mEq/L.

IMAGING:
A chest x-ray shows a central mass in the right upper lobe. CT of the chest confirms a 4.5-cm hilar mass with mediastinal lymphadenopathy.
