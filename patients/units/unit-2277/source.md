# synthetic_hospital patient 2277

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-10-27 (Mon) | outpatient | Hematology & Oncology |
| 1 | 2020-04-15 | 2026-02-09 (Mon) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2026-05-13 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Father with lung cancer",
  "Mother with type 2 diabetes mellitus",
  "Older brother with COPD"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 10 years ago",
 "home_medications": [
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "18 mcg inhaled once daily",
   "name": "Tiotropium"
  },
  {
   "dose": "90 mcg inhaler, 2 puffs every 4-6 hours as needed for wheezing",
   "name": "Albuterol sulfate"
  }
 ],
 "surgical_history": [
  "Bronchoscopy with transbronchial biopsy (lung cancer diagnosis)",
  "Mediastinoscopy (lymph node staging)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD)",
  "Type 2 diabetes mellitus",
  "Stage IIIB non-small cell lung cancer (s/p chemoradiation therapy, 6 months post-treatment)"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Hyperchloremic metabolic acidosis due to excess normal saline infusion"]
Comorbidities: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Chronic obstructive pulmonary disease", "History of recent pneumonia", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2025-10-27, outpatient, Hematology & Oncology

Chief complaint: Follow-up visit for stage IIIB lung cancer surveillance with new onset cough and shortness of breath.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit for stage IIIB lung cancer surveillance with new onset cough and shortness of breath.

HISTORY OF PRESENT ILLNESS:
He reports a new onset of cough and shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD)
- Type 2 diabetes mellitus
- Stage IIIB non-small cell lung cancer (s/p chemoradiation therapy, 6 months post-treatment)

He was diagnosed with stage IIIB non-small cell lung cancer (NSCLC) 6 months ago and completed a course of chemotherapy and radiation therapy.

PAST SURGICAL HISTORY:
- Bronchoscopy with transbronchial biopsy (lung cancer diagnosis)
- Mediastinoscopy (lymph node staging)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Tiotropium 18 mcg inhaled once daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4-6 hours as needed for wheezing

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with lung cancer
- Mother with type 2 diabetes mellitus
- Older brother with COPD

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 10 years ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
Physical examination reveals decreased breath sounds in the right lower lung field

LABORATORY DATA:
Laboratory tests are as follows:

- White blood cell count: 12,000 /µL
- Hemoglobin: 13.2 g/dL
- Platelets: 250,000 /µL
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL

IMAGING:
A chest X-ray shows a new right lower lobe consolidation

PLAN:
The patient is enrolled in a clinical trial comparing two new targeted therapies for NSCLC. The primary endpoint is event-free survival. The Kaplan-Meier curves for both study groups show a separation of survival curves with a p-value of 0.03 at 6 months.

## Encounter 1: source 2020-04-15, practice 2026-02-09, ed, Emergency Medicine

Chief complaint: High fever, shaking chills, and productive cough with yellowish sputum for 2 days.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
High fever, shaking chills, and productive cough with yellowish sputum for 2 days.

HISTORY OF PRESENT ILLNESS:
This 65-year-old male with COPD, type 2 diabetes, and stage IIIB non-small cell lung cancer s/p chemoradiation (6 months post-treatment) was last evaluated in January 2020 for cancer surveillance with cough and shortness of breath. He presents to the emergency department with a 2-day history of high fever, shaking chills, and a productive cough with yellowish sputum.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD)
- Type 2 diabetes mellitus
- Stage IIIB non-small cell lung cancer (s/p chemoradiation therapy, 6 months post-treatment)

He has a past medical history of type 2 diabetes mellitus and chronic obstructive pulmonary disease (COPD).

PAST SURGICAL HISTORY:
- Bronchoscopy with transbronchial biopsy (lung cancer diagnosis)
- Mediastinoscopy (lymph node staging)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Tiotropium 18 mcg inhaled once daily
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4-6 hours as needed for wheezing

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with lung cancer
- Mother with type 2 diabetes mellitus
- Older brother with COPD

SOCIAL HISTORY:
He quit smoking 10 years ago after a 30-year pack history.

VITAL SIGNS:
On examination, his temperature is 39.2°C (102.5°F), blood pressure is 135/85 mmHg, pulse is 102 beats/min, and respiratory rate is 22 breaths/min. Oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Chest auscultation reveals decreased breath sounds and dullness to percussion at the right lower lobe.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 18,000/mm³ (normal: 4,500-11,000/mm³)
- Neutrophils: 80% (normal: 40-60%)
- Lymphocytes: 10% (normal: 20-40%)
- Hemoglobin: 14 g/dL (normal: 13.8-17.2 g/dL)
- Platelets: 250,000/mm³ (normal: 150,000-450,000/mm³)
- Blood glucose: 180 mg/dL (normal: 70-99 mg/dL)

IMAGING:
A chest x-ray shows a dense consolidation in the right lower lobe.

## Encounter 2: source 2020-07-15, practice 2026-05-13, ed, Emergency Medicine

Chief complaint: Fever, productive cough, and increasing shortness of breath for 3 days.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, productive cough, and increasing shortness of breath for 3 days.

HISTORY OF PRESENT ILLNESS:
The patient is a 65-year-old male with stage IIIB non-small cell lung cancer s/p chemoradiation six months prior, COPD, and type 2 diabetes mellitus who returns to the emergency department three months after a prior visit for fever and productive cough. He presents with a 3-day history of fever, productive cough, and increasing shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Opportunistic pulmonary infection requiring diagnostic sampling (diagnosed 2020-04-15)
- Chronic obstructive pulmonary disease (COPD)
- Type 2 diabetes mellitus
- Stage IIIB non-small cell lung cancer (s/p chemoradiation therapy, 6 months post-treatment)

PAST SURGICAL HISTORY:
- Bronchoscopy with transbronchial biopsy (lung cancer diagnosis)
- Mediastinoscopy (lymph node staging)

MEDICATIONS:
He takes metformin and tiotropium.

Home Medications:
- Albuterol sulfate 90 mcg inhaler, 2 puffs every 4-6 hours as needed for wheezing

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with lung cancer
- Mother with type 2 diabetes mellitus
- Older brother with COPD

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 10 years ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
Tachypneic with a respiratory rate of 28 breaths per minute, febrile with a temperature of 38.7°C, and hypotensive with a blood pressure of 90/60 mmHg. His oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals bilateral crackles.

LABORATORY DATA:
Arterial blood gas (ABG):
- pH: 7.25
- PaCO2: 60 mmHg
- PaO2: 55 mmHg
- HCO3-: 22 mEq/L
- Complete blood count (CBC):
- White blood cells (WBC): 18,000/uL
- Serum lactate: 5 mmol/L
- Blood cultures are pending.
