# synthetic_hospital patient 2611

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex M, Filipino, Uninsured.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-05-22 (Wed) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2024-11-14 (Thu) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2025-11-25 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Uninsured",
 "occupation": "Residential case manager at homeless shelter",
 "alcohol_use": "None",
 "family_history": [
  "Father with hypertension",
  "Mother with type 2 diabetes"
 ],
 "race_ethnicity": "Filipino",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Tuberculosis", "Primary pulmonary tuberculosis"]
Comorbidities: ["Exposure to tuberculosis", "Unvaccinated Status"]

## Encounter 0: source 2020-01-15, practice 2024-05-22, ed, Emergency Medicine

Chief complaint: Productive cough, night sweats, and 5-pound weight loss for 2 weeks

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Productive cough, night sweats, and 5-pound weight loss for 2 weeks

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department with a two-week history of productive cough, night sweats, and a 5-pound weight loss. He recently immigrated from a country with a high prevalence of tuberculosis and has not received any vaccinations or medical care since his arrival.

PAST MEDICAL HISTORY:
He denies any past medical history or use of medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension
- Mother with type 2 diabetes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Residential case manager at homeless shelter.

VITAL SIGNS:
On examination, he is afebrile, with a heart rate of 88 beats per minute, respiratory rate of 20 breaths per minute, and oxygen saturation of 96% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals crackles at the right apex. During the examination, the patient expectorates a small amount of blood-tinged sputum.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 9,000 cells/µL
- Platelets: 250,000 cells/µL
- HIV test: Non-reactive

IMAGING:
Chest radiography shows a right apical infiltrate.

## Encounter 1: source 2020-07-15, practice 2024-11-14, ed, Emergency Medicine

Chief complaint: Productive cough, night sweats, and 5kg weight loss for 3 weeks

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Raj Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Productive cough, night sweats, and 5kg weight loss for 3 weeks

HISTORY OF PRESENT ILLNESS:
The patient returns six months after a prior similar presentation. He presents with a 3-week history of productive cough, night sweats, and a 5 kg weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Tuberculosis (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension
- Mother with type 2 diabetes

SOCIAL HISTORY:
He recently immigrated from a country with a high prevalence of tuberculosis and has not received any vaccinations or medical care since his arrival.

PHYSICAL EXAMINATION:
On examination, he is febrile with a temperature of 38.6°C, tachycardic with a heart rate of 102 beats per minute, and has decreased breath sounds in the right upper lobe.

LABORATORY DATA:
Laboratory tests show a leukocytosis of 12,000 cells/µL with a left shift. Three sputum samples are collected for acid-fast bacilli (AFB) stain and culture. The first sputum AFB stain is negative.

IMAGING:
Chest X-ray reveals a right apical infiltrate with cavitation.

## Encounter 2: source 2021-07-15, practice 2025-11-25, ed, Emergency Medicine

Chief complaint: Persistent fever, non-productive cough, and sharp pleuritic chest pain for 4 weeks

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Persistent fever, non-productive cough, and sharp pleuritic chest pain for 4 weeks

HISTORY OF PRESENT ILLNESS:
Patient returns after similar 2020 ED visits. He reports 4-week history of persistent fever, non-productive cough, and sharp chest pain worsening with deep breaths.

PAST MEDICAL HISTORY:
Active Problem List:
- Tuberculosis (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension
- Mother with type 2 diabetes

SOCIAL HISTORY:
He works in a homeless shelter

VITAL SIGNS:
Temperature of 38.5°C

PHYSICAL EXAMINATION:
Decreased breath sounds in the right upper lobe, and dullness to percussion

LABORATORY DATA:
Laboratory tests reveal:
- White blood cell count: 11,000 /µL
- Hemoglobin: 13.5 g/dL
- Erythrocyte sedimentation rate: 55 mm/hr
- HIV test: Negative

IMAGING:
A chest X-ray reveals a right upper lobe infiltrate and hilar lymphadenopathy

OTHER STUDIES:
A tuberculin skin test shows an induration of 15 mm
