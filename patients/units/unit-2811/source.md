# synthetic_hospital patient 2811

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex F, Non-Hispanic White, Commercial preferred provider organization (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-08-26 (Mon) | outpatient | Gastroenterology |
| 1 | 2021-07-15 | 2026-02-19 (Thu) | outpatient | Cardiology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial preferred provider organization (PPO)",
 "occupation": "Administrative assistant",
 "alcohol_use": "Rare (1-2 glasses of wine per month)",
 "family_history": [
  "Mother: Rheumatoid arthritis",
  "Father: Hypertension",
  "Sister: Hashimoto thyroiditis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "40 mg PO daily",
   "name": "Omeprazole"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "1000 mg PO twice daily",
   "name": "Mycophenolate mofetil"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Acetylsalicylic acid"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (age 42)",
  "Carpal tunnel release, right wrist (age 54)"
 ],
 "chronic_conditions": [
  "Limited cutaneous systemic sclerosis (CREST syndrome)",
  "Raynaud phenomenon",
  "Gastroesophageal reflux disease"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Pulmonary arterial hypertension associated with systemic sclerosis", "Esophageal dysmotility"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-08-26, outpatient, Gastroenterology

Chief complaint: Intermittent chest discomfort and difficulty swallowing

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent chest discomfort and difficulty swallowing

HISTORY OF PRESENT ILLNESS:
She reports episodes of sudden, intense retrosternal pain that can last for several minutes, often occurring while eating but sometimes even at rest. The pain is non-radiating and not associated with exertion. She describes it as “squeezing,” and notes that she occasionally feels food getting stuck mid-way down her chest. The pain is sometimes relieved by drinking warm liquids. She denies any history of heartburn, unintentional weight loss, or food regurgitation.

PAST MEDICAL HISTORY:
Active Problem List:
- Limited cutaneous systemic sclerosis (CREST syndrome)
- Raynaud phenomenon
- Gastroesophageal reflux disease

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 42)
- Carpal tunnel release, right wrist (age 54)

MEDICATIONS:
Home Medications:
- Omeprazole 40 mg PO daily
- Amlodipine 5 mg PO daily
- Mycophenolate mofetil 1000 mg PO twice daily
- Acetylsalicylic acid 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Hypertension
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 glasses of wine per month). Occupation: Administrative assistant.

VITAL SIGNS:
Her vital signs and physical exam are unremarkable.

PHYSICAL EXAMINATION:
Her vital signs and physical exam are unremarkable.

LABORATORY DATA:
Cardiac enzymes were within normal limits.

IMAGING:
To further investigate her symptoms, an upper GI barium swallow was obtained and is shown below.

OTHER STUDIES:
A recent EKG and exercise stress test were normal. Esophageal manometry is pending.

## Encounter 1: source 2021-07-15, practice 2026-02-19, outpatient, Cardiology

Chief complaint: Progressive fatigue and exertional dyspnea over the past year

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and exertional dyspnea over the past year

HISTORY OF PRESENT ILLNESS:
The patient with limited cutaneous systemic sclerosis and prior Gastroenterology evaluation in January 2020 for esophageal symptoms returns to clinic due to progressive fatigue and exertional dyspnea over the past year. She also reports difficulty swallowing solid foods, intermittent episodes of fingers turning white and painful in the cold, and tightness of the skin on her hands.

PAST MEDICAL HISTORY:
Active Problem List:
- Esophageal dysmotility (diagnosed 2020-01-15)
- Limited cutaneous systemic sclerosis (CREST syndrome)
- Raynaud phenomenon
- Gastroesophageal reflux disease

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 42)
- Carpal tunnel release, right wrist (age 54)

MEDICATIONS:
Home Medications:
- Omeprazole 40 mg PO daily
- Amlodipine 5 mg PO daily
- Mycophenolate mofetil 1000 mg PO twice daily
- Acetylsalicylic acid 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Father: Hypertension
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 glasses of wine per month). Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On examination, she has taut, shiny skin over the fingers and telangiectasias on her face. Cardiac auscultation reveals a loud P2 component of the second heart sound.

OTHER STUDIES:
Pulmonary function tests show decreased diffusing capacity of carbon monoxide (DLCO).
