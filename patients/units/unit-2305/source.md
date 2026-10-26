# synthetic_hospital patient 2305

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-03 (Sun) | inpatient | Internal Medicine |
| 1 | 2020-09-15 | 2025-07-04 (Fri) | inpatient | Internal Medicine |
| 2 | 2021-05-15 | 2026-02-19 (Thu) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired firefighter",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Chronic obstructive pulmonary disease, deceased age 72",
  "Mother: Lung adenocarcinoma, deceased age 68",
  "Brother: Coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-year history, quit 3 years ago",
 "home_medications": [
  {
   "dose": "90 mcg, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "18 mcg inhaled via HandiHaler once daily",
   "name": "Tiotropium bromide"
  },
  {
   "dose": "250/50 mcg, 1 inhalation twice daily",
   "name": "Fluticasone propionate/Salmeterol"
  }
 ],
 "surgical_history": [
  "Cholecystectomy (2008)",
  "Left knee arthroscopy (2015)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (GOLD Group B)",
  "Emphysema",
  "Tobacco use disorder, in sustained remission"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Smoke inhalation injury", "Community-acquired pneumonia"]
Comorbidities: ["Chronic obstructive pulmonary disease"]

## Encounter 0: source 2020-01-15, practice 2024-11-03, inpatient, Internal Medicine

Chief complaint: Increased cough with yellowish sputum production and fever for 3 days

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Increased cough with yellowish sputum production and fever for 3 days

HISTORY OF PRESENT ILLNESS:
3-day history of increased cough, yellowish sputum production, and fever. He denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (GOLD Group B)
- Emphysema
- Tobacco use disorder, in sustained remission

PAST SURGICAL HISTORY:
- Cholecystectomy (2008)
- Left knee arthroscopy (2015)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath
- Tiotropium bromide 18 mcg inhaled via HandiHaler once daily
- Fluticasone propionate/Salmeterol 250/50 mcg, 1 inhalation twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease, deceased age 72
- Mother: Lung adenocarcinoma, deceased age 68
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 3 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired firefighter.

VITAL SIGNS:
His temperature is 38.5°C, blood pressure is 135/85 mmHg, pulse is 92 beats per minute, and respiratory rate is 22 breaths per minute. The patient's oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
On physical examination, you note increased tactile fremitus and dullness to percussion at the right lower lung zone. Auscultation reveals bronchial breath sounds and late inspiratory crackles in the same area.

LABORATORY DATA:
His other laboratory values are within normal limits.

IMAGING:
A chest x-ray is performed and shows a dense lobar infiltrate in the right lower lobe. No pleural effusion is noted.

## Encounter 1: source 2020-09-15, practice 2025-07-04, inpatient, Internal Medicine

Chief complaint: Increased shortness of breath with productive cough and fever

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Michael Okonkwo, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Increased shortness of breath with productive cough and fever

HISTORY OF PRESENT ILLNESS:
The patient with known COPD and emphysema returns eight months after his January admission. He presents to the emergency department with a 3-day history of increased shortness of breath, productive cough with yellowish sputum, and fever. He denies chest pain, palpitations, or leg swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Community-acquired pneumonia (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (GOLD Group B)
- Emphysema
- Tobacco use disorder, in sustained remission

PAST SURGICAL HISTORY:
- Cholecystectomy (2008)
- Left knee arthroscopy (2015)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath
- Tiotropium bromide 18 mcg inhaled via HandiHaler once daily
- Fluticasone propionate/Salmeterol 250/50 mcg, 1 inhalation twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease, deceased age 72
- Mother: Lung adenocarcinoma, deceased age 68
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 3 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired firefighter.

VITAL SIGNS:
His temperature is 38.7°C, blood pressure is 135/85 mmHg, pulse is 102 beats/min, and respiratory rate is 22 breaths/min.

PHYSICAL EXAMINATION:
On physical examination, you note dullness to percussion and increased tactile fremitus over the right lower lobe. Auscultation of the same area reveals bronchial breath sounds and late inspiratory crackles. The remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 15,000 /µL
- Hemoglobin: 14.2 g/dL
- Platelet count: 250,000 /µL
- Blood urea nitrogen: 18 mg/dL
- Serum creatinine: 1.0 mg/dL

IMAGING:
Chest X-ray shows an area of increased density in the right lower lobe.

## Encounter 2: source 2021-05-15, practice 2026-02-19, icu, Pulmonary & Critical Care

Chief complaint: Acute hypoxemic respiratory failure requiring mechanical ventilation following smoke exposure

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Jessica Rodriguez, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Acute hypoxemic respiratory failure requiring mechanical ventilation following smoke exposure

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old male with COPD and prior community-acquired pneumonia admissions presenting following smoke exposure. He is intubated and mechanically ventilated due to decreased level of consciousness and respiratory distress. Over the next 24 hours, his oxygenation improves, and he remains hemodynamically stable.

PAST MEDICAL HISTORY:
Active Problem List:
- Community-acquired pneumonia (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (GOLD Group B)
- Emphysema
- Tobacco use disorder, in sustained remission

PAST SURGICAL HISTORY:
- Cholecystectomy (2008)
- Left knee arthroscopy (2015)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath
- Tiotropium bromide 18 mcg inhaled via HandiHaler once daily
- Fluticasone propionate/Salmeterol 250/50 mcg, 1 inhalation twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease, deceased age 72
- Mother: Lung adenocarcinoma, deceased age 68
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 3 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired firefighter.

VITAL SIGNS:
Blood pressure of 125/75 mmHg, heart rate of 88 beats per minute, and SpO2 of 98%

PHYSICAL EXAMINATION:
Physical examination reveals bilateral wheezes, no use of accessory muscles, and improved alertness.

LABORATORY DATA:
Arterial blood gas (ABG) analysis on these settings shows pH 7.35, PaCO2 45 mmHg, PaO2 110 mmHg, and HCO3- 24 mEq/L.
repeat ABG on the same ventilator settings shows pH 7.40, PaCO2 40 mmHg, PaO2 200 mmHg, and HCO3- 24 mEq/L.

OTHER STUDIES:
Initial ventilator settings are a tidal volume of 450 mL, respiratory rate of 16 breaths per minute, FiO2 of 100%, and PEEP of 5 cm H2O.
