# synthetic_hospital patient 2151

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 54 at first encounter, sex M, Non-Hispanic White, Medicare (disability) and supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-10 (Mon) | outpatient | Rheumatology |
| 1 | 2020-03-15 | 2024-08-09 (Fri) | inpatient | Internal Medicine |
| 2 | 2022-01-15 | 2026-06-03 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (disability) and supplemental private insurance",
 "occupation": "Construction worker (currently on medical disability)",
 "alcohol_use": "Former heavy use; history of chronic alcohol use disorder, in remission since approximately 1 year ago",
 "family_history": [
  "Father: Chronic obstructive pulmonary disease",
  "Mother: Hypothyroidism (autoimmune)",
  "Brother: Lung cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit approximately 1 year ago",
 "home_medications": [
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
  },
  {
   "dose": "600 mg orally twice daily",
   "name": "Calcium carbonate"
  },
  {
   "dose": "2000 IU orally daily",
   "name": "Vitamin D3"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Rheumatoid arthritis (seropositive, erosive)",
  "History of pulmonary tuberculosis (completed treatment)",
  "History of occupational asbestos exposure",
  "Tobacco use disorder (former)",
  "Alcohol use disorder (former, in remission)"
 ],
 "age_at_first_encounter": 54
}
```

Primary diagnoses: ["Pneumococcal pneumonia", "Rheumatoid arthritis", "Tuberculosis"]
Comorbidities: ["Rheumatoid arthritis"]

## Encounter 0: source 2020-01-15, practice 2024-06-10, outpatient, Rheumatology

Chief complaint: Worsening pain and stiffness in hands and wrists, with symptoms most severe in the morning and improving gradually throughout the day.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening pain and stiffness in hands and wrists, with symptoms most severe in the morning and improving gradually throughout the day.

HISTORY OF PRESENT ILLNESS:
He reports that the symptoms are most severe in the morning and improve gradually throughout the day.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (seropositive, erosive)
- History of pulmonary tuberculosis (completed treatment)
- History of occupational asbestos exposure
- Tobacco use disorder (former)
- Alcohol use disorder (former, in remission)

MEDICATIONS:
Home Medications:
- Methotrexate 15 mg orally once weekly
- Folic acid 1 mg orally daily
- Prednisone 5 mg orally daily
- Calcium carbonate 600 mg orally twice daily
- Vitamin D3 2000 IU orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Mother: Hypothyroidism (autoimmune)
- Brother: Lung cancer

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit approximately 1 year ago. Alcohol: Former heavy use; history of chronic alcohol use disorder, in remission since approximately 1 year ago. Occupation: Construction worker (currently on medical disability).

PHYSICAL EXAMINATION:
Physical examination shows tenderness and swelling in multiple metacarpophalangeal and proximal interphalangeal joints bilaterally.

LABORATORY DATA:
Laboratory testing reveals elevated inflammatory markers

IMAGING:
Imaging demonstrates erosive changes consistent with rheumatoid arthritis

PLAN:
His rheumatologist discusses starting etanercept in addition to his current methotrexate therapy.

## Encounter 1: source 2020-03-15, practice 2024-08-09, inpatient, Internal Medicine

Chief complaint: Three-week history of fever, night sweats, weight loss, productive cough, and fatigue.

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Marcus Okafor, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Three-week history of fever, night sweats, weight loss, productive cough, and fatigue.

HISTORY OF PRESENT ILLNESS:
The patient is a 54-year-old male with seropositive erosive rheumatoid arthritis and prior pulmonary tuberculosis who presents with a 3-week history of fever, night sweats, weight loss, productive cough, and fatigue. He was recently diagnosed with rheumatoid arthritis 2 months ago and started on a new medication at that time.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (diagnosed 2020-01-15)
- Rheumatoid arthritis (seropositive, erosive)
- History of pulmonary tuberculosis (completed treatment)
- History of occupational asbestos exposure
- Tobacco use disorder (former)
- Alcohol use disorder (former, in remission)

MEDICATIONS:
He takes started on a new medication 2 months ago.

Home Medications:
- Methotrexate 15 mg orally once weekly
- Folic acid 1 mg orally daily
- Prednisone 5 mg orally daily
- Calcium carbonate 600 mg orally twice daily
- Vitamin D3 2000 IU orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history of colon cancer

SOCIAL HISTORY:
History of chronic alcohol use and a 30-pack-year smoking history

VITAL SIGNS:
Temperature is 38.6°C, heart rate is 102/min, respiratory rate is 22/min, and blood pressure is 128/76 mm Hg

PHYSICAL EXAMINATION:
He is obese with a BMI of 34 kg/m². On examination, there are diffuse crackles over both lung fields.

IMAGING:
A chest x-ray is obtained and shown below.

## Encounter 2: source 2022-01-15, practice 2026-06-03, ed, Emergency Medicine

Chief complaint: Four-day history of productive cough, fever, shortness of breath, chills, and pleuritic chest pain.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Four-day history of productive cough, fever, shortness of breath, chills, and pleuritic chest pain.

HISTORY OF PRESENT ILLNESS:
Patient with rheumatoid arthritis and prior tuberculosis presents with 4-day history of productive cough, fever, shortness of breath, chills, and pleuritic chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (diagnosed 2020-01-15)
- Tuberculosis (diagnosed 2020-03-15)
- Rheumatoid arthritis (seropositive, erosive)
- History of pulmonary tuberculosis (completed treatment)
- History of occupational asbestos exposure
- Tobacco use disorder (former)
- Alcohol use disorder (former, in remission)

He was diagnosed with rheumatoid arthritis one year ago.

MEDICATIONS:
He has been receiving rituximab infusions every six months since then.

Home Medications:
- Methotrexate 15 mg orally once weekly
- Folic acid 1 mg orally daily
- Prednisone 5 mg orally daily
- Calcium carbonate 600 mg orally twice daily
- Vitamin D3 2000 IU orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Mother: Hypothyroidism (autoimmune)
- Brother: Lung cancer

SOCIAL HISTORY:
He worked as a construction worker for many years with frequent asbestos exposure but has no history of smoking or alcohol use.

VITAL SIGNS:
Febrile (38.5°C), with increased respiratory rate

PHYSICAL EXAMINATION:
Decreased breath sounds with crackles at the right lung base.

LABORATORY DATA:
Sputum culture later grows Streptococcus pneumoniae.

IMAGING:
Chest x-ray is shown below.
