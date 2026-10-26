# synthetic_hospital patient 2788

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 37 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-04 (Wed) | outpatient | Pulmonology |
| 1 | 2021-01-15 | 2025-11-24 (Mon) | outpatient | Rheumatology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Graphic designer",
 "alcohol_use": "None",
 "family_history": [
  "Father with early-onset emphysema (diagnosed age 44)",
  "Maternal aunt with rheumatoid arthritis",
  "Mother with eczema"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "90 mcg, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "110 mcg, 2 puffs twice daily",
   "name": "Fluticasone propionate inhaler"
  },
  {
   "dose": "0.1%, apply to affected areas twice daily for eczema flares",
   "name": "Triamcinolone acetonide cream"
  },
  {
   "dose": "200 mg, 1-2 tablets every 6-8 hours as needed for joint pain (max 1200 mg/day)",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Tonsillectomy and adenoidectomy (age 7)"
 ],
 "chronic_conditions": [
  "Asthma, mild persistent",
  "Atopic dermatitis"
 ],
 "age_at_first_encounter": 37
}
```

Primary diagnoses: ["Alpha-1 antitrypsin deficiency", "Rheumatoid arthritis"]
Comorbidities: ["Asthma"]

## Encounter 0: source 2020-01-15, practice 2024-12-04, outpatient, Pulmonology

Chief complaint: Six-month history of progressive shortness of breath and chronic cough productive of clear sputum despite adherence to asthma maintenance therapy.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive shortness of breath and chronic cough productive of clear sputum despite adherence to asthma maintenance therapy.

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of progressive shortness of breath and a chronic cough productive of clear sputum.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma, mild persistent
- Atopic dermatitis

Her past medical history is significant for childhood asthma, which resolved during her teenage years.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy (age 7)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath
- Fluticasone propionate inhaler 110 mcg, 2 puffs twice daily
- Triamcinolone acetonide cream 0.1%, apply to affected areas twice daily for eczema flares
- Ibuprofen 200 mg, 1-2 tablets every 6-8 hours as needed for joint pain (max 1200 mg/day)

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of lung disease.

SOCIAL HISTORY:
She denies any history of smoking or occupational exposures.

PHYSICAL EXAMINATION:
On physical examination, she has decreased breath sounds at the lung bases and a prolonged expiratory phase.

LABORATORY DATA:
Laboratory Values:
- Complete Blood Count (CBC): Within normal limits
- Serum α1 antitrypsin (AAT) level: Pending

IMAGING:
A chest X-ray reveals hyperinflation and basilar-predominant emphysema.

OTHER STUDIES:
Her spirometry test shows an FEV1/FVC ratio of 65% (normal >70%), indicating an obstructive pattern. Post-bronchodilator testing shows minimal improvement.

ASSESSMENT AND PLAN:
Given her age and lack of typical risk factors for chronic obstructive pulmonary disease (COPD), you suspect an underlying genetic cause.

## Encounter 1: source 2021-01-15, practice 2025-11-24, outpatient, Rheumatology

Chief complaint: Six-month history of episodic numbness and color changes in fingers when exposed to cold, accompanied by joint pain and morning stiffness in hands and knees.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Amara Okafor, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of episodic numbness and color changes in fingers when exposed to cold, accompanied by joint pain and morning stiffness in hands and knees.

HISTORY OF PRESENT ILLNESS:
The patient is a 38-year-old female with mild persistent asthma, atopic dermatitis, and alpha-1 antitrypsin deficiency who presents for rheumatology evaluation. She reports that her fingers turn white and then blue when she is exposed to cold temperatures or during periods of stress. The episodes are painful and are sometimes accompanied by tingling. She also mentions experiencing occasional joint pain and morning stiffness in her hands and knees.

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency (diagnosed 2020-01-15)
- Asthma, mild persistent
- Atopic dermatitis

Her past medical history is significant for mild asthma and eczema.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy (age 7)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath
- Fluticasone propionate inhaler 110 mcg, 2 puffs twice daily
- Triamcinolone acetonide cream 0.1%, apply to affected areas twice daily for eczema flares
- Ibuprofen 200 mg, 1-2 tablets every 6-8 hours as needed for joint pain (max 1200 mg/day)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with early-onset emphysema (diagnosed age 44)
- Maternal aunt with rheumatoid arthritis
- Mother with eczema

SOCIAL HISTORY:
She denies smoking, alcohol, or drug use.

VITAL SIGNS:
On physical examination, her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Examination of her hands reveals pallor of the fingertips bilaterally, with no ulcerations or gangrene. Capillary refill time is less than 2 seconds. The remainder of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory studies are ordered, and the results are as follows:

- Complete blood count: Normal
- Erythrocyte sedimentation rate (ESR): 50 mm/hr (Normal: 0-20 mm/hr)
- C-reactive protein (CRP): 24 mg/L (Normal: <10 mg/L)
- Antinuclear antibodies (ANA): Positive
- Anti-centromere antibodies: Negative
- Anti-topoisomerase antibodies (Scl-70): Negative
- Rheumatoid factor (RF): Positive
- Anti-cyclic citrullinated peptide (anti-CCP) antibodies: Positive
