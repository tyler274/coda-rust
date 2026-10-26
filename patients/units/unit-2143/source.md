# synthetic_hospital patient 2143

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-23 (Thu) | outpatient | Pulmonary & Critical Care |
| 1 | 2021-07-15 | 2026-07-20 (Mon) | outpatient | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance",
 "occupation": "Financial analyst",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father with interstitial lung disease",
  "Mother with rheumatoid arthritis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Lifelong nonsmoker",
 "home_medications": [
  {
   "dose": "2 mg twice daily (goal trough 8-12 ng/mL)",
   "name": "Tacrolimus"
  },
  {
   "dose": "750 mg twice daily",
   "name": "Mycophenolate mofetil"
  },
  {
   "dose": "5 mg daily",
   "name": "Prednisone"
  }
 ],
 "surgical_history": [
  "Bilateral lung transplantation"
 ],
 "chronic_conditions": [
  "Idiopathic pulmonary fibrosis"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Cytomegalovirus pneumonitis", "Idiopathic Pulmonary Fibrosis"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-01-23, outpatient, Pulmonary & Critical Care

Chief complaint: Increasing shortness of breath over the past 6 months and persistent dry cough unresponsive to OTC treatments

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing shortness of breath over the past 6 months and persistent dry cough unresponsive to OTC treatments

HISTORY OF PRESENT ILLNESS:
He says he becomes winded when walking up a single flight of stairs and has also developed a persistent dry cough that does not improve with over-the-counter treatments. He denies wheezing, sputum production, chest pain, or leg swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic pulmonary fibrosis

PAST SURGICAL HISTORY:
- Bilateral lung transplantation

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg twice daily (goal trough 8-12 ng/mL)
- Mycophenolate mofetil 750 mg twice daily
- Prednisone 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with interstitial lung disease
- Mother with rheumatoid arthritis

SOCIAL HISTORY:
He works as a financial analyst and has no known occupational exposures. There is no recent travel or history of tuberculosis contact.

PHYSICAL EXAMINATION:
On examination, he has fine bibasilar inspiratory crackles but no wheezing or signs of fluid overload. His oxygen saturation is 94% on room air at rest but drops to 88% with exertion during a 6-minute walk test. Cardiac examination is normal, and there is no peripheral edema.

IMAGING:
Chest x-ray reveals a subtle interstitial pattern predominantly at the lung bases. High-resolution CT of the chest shows reticulonodular opacities with peripheral and basal predominance, along with subpleural honeycombing.

OTHER STUDIES:
Pulmonary function testing is performed. Total lung capacity is reduced. There is no evidence of airway obstruction.

## Encounter 1: source 2021-07-15, practice 2026-07-20, outpatient, Pulmonary & Critical Care

Chief complaint: Progressive dyspnea, nonproductive cough, and fever over the past 2 weeks

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Okonkwo, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea, nonproductive cough, and fever over the past 2 weeks

HISTORY OF PRESENT ILLNESS:
Patient returns for idiopathic pulmonary fibrosis follow-up. His postoperative course was uneventful. He denies any sick contacts or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
He underwent a lung transplantation 2 months ago

MEDICATIONS:
He was discharged on a regimen of tacrolimus, mycophenolate mofetil, and prednisone

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with interstitial lung disease
- Mother with rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Lifelong nonsmoker. Alcohol: Social (1-2 drinks per week). Occupation: Financial analyst.

VITAL SIGNS:
Temperature is 38.7°C (101.7°F), blood pressure is 130/85 mmHg, heart rate is 102 beats per minute, and respiratory rate is 26 breaths per minute. Oxygen saturation is 89% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral crackles.

LABORATORY DATA:
White blood cell count: 9,000/uL
Hemoglobin: 12.5 g/dL
Platelet count: 250,000/uL
Serum creatinine: 1.2 mg/dL
Blood urea nitrogen: 21 mg/dL
Liver enzymes: Within normal limits
CMV PCR: Positive

IMAGING:
Chest X-ray shows diffuse interstitial infiltrates.
