# synthetic_hospital patient 2158

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield (private employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-04-26 (Fri) | outpatient | Pulmonology |
| 1 | 2021-07-15 | 2025-10-28 (Tue) | outpatient | Orthopedics |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield (private employer-sponsored)",
 "occupation": "Dairy farmer and farm equipment mechanic",
 "alcohol_use": "Social (1-2 beers on weekends)",
 "family_history": [
  "Father: Hypertension, chronic interstitial lung disease of unknown etiology",
  "Mother: Type 2 diabetes mellitus",
  "Sibling: None"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "10 mg PO PRN for seasonal allergies",
   "name": "Cetirizine"
  }
 ],
 "surgical_history": [
  "Tonsillectomy, age 7",
  "No other surgeries"
 ],
 "chronic_conditions": [
  "Essential hypertension",
  "Seasonal allergic rhinitis"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Hypersensitivity Pneumonitis (Chronic/Fibrotic)", "Noninfectious olecranon bursitis"]
Comorbidities: ["Construction worker occupation"]

## Encounter 0: source 2020-01-15, practice 2024-04-26, outpatient, Pulmonology

Chief complaint: presents to the clinic with a 2-month history of progressive dyspnea, dry cough, and intermittent fevers.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic with a 2-month history of progressive dyspnea, dry cough, and intermittent fevers.

HISTORY OF PRESENT ILLNESS:
He reports that his symptoms worsen a few hours after working in the barn and improve when he spends time away from the farm.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Tonsillectomy, age 7
- No other surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Cetirizine 10 mg PO PRN for seasonal allergies

ALLERGIES:
NKDA

FAMILY HISTORY:
He denies any family history of lung disease

SOCIAL HISTORY:
He denies smoking

VITAL SIGNS:
On examination, he is afebrile, with a respiratory rate of 22 breaths per minute, and oxygen saturation of 94% on room air.

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals bilateral inspiratory crackles. His fingers show mild clubbing.

LABORATORY DATA:
Laboratory Values:
- White blood cell count: 7,500/µL (normal range: 4,500-11,000/µL)
- Erythrocyte sedimentation rate: 40 mm/hr (normal range: 0-20 mm/hr)
- Serum precipitins: Positive for avian proteins

IMAGING:
A high-resolution CT scan of the chest demonstrates patchy ground-glass opacities and evidence of honeycombing.

OTHER STUDIES:
Pulmonary function tests show a restrictive pattern with reduced diffusing capacity for carbon monoxide (DLCO).

## Encounter 1: source 2021-07-15, practice 2025-10-28, outpatient, Orthopedics

Chief complaint: presents to the clinic with a 2-week history of swelling over the posterior aspect of his right elbow.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic with a 2-week history of swelling over the posterior aspect of his right elbow.

HISTORY OF PRESENT ILLNESS:
He has hypertension, rhinitis, and prior pneumonitis and presents with elbow swelling. He denies trauma but notes his job requires leaning on his elbows frequently.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypersensitivity Pneumonitis (Chronic/Fibrotic) (diagnosed 2020-01-15)
- Essential hypertension
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Tonsillectomy, age 7
- No other surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Cetirizine 10 mg PO PRN for seasonal allergies

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, chronic interstitial lung disease of unknown etiology
- Mother: Type 2 diabetes mellitus
- Sibling: None

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 beers on weekends). Occupation: Dairy farmer and farm equipment mechanic.

VITAL SIGNS:
He is afebrile and vital signs are stable.

PHYSICAL EXAMINATION:
On examination, there is a swollen, fluctuant area over the olecranon without overlying erythema or warmth. The range of motion in the elbow is preserved without pain.

OTHER STUDIES:
Aspiration of the bursal fluid is performed, revealing a clear, straw-colored fluid with a white blood cell count of 150 cells/µL (normal: <200 cells/µL), no crystals, and a negative Gram stain.
