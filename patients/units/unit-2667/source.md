# synthetic_hospital patient 2667

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield PPO (Union-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-13 (Thu) | outpatient | Dermatology |
| 1 | 2020-07-15 | 2024-12-12 (Thu) | outpatient | Dermatology |
| 2 | 2021-07-15 | 2025-12-16 (Tue) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO (Union-sponsored)",
 "occupation": "Carpenter",
 "alcohol_use": "Social (1-2 beers on weekends)",
 "family_history": [
  "Mother: Plaque psoriasis",
  "Father: Hypertension",
  "Paternal grandfather: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "90 mcg per actuation, 2 inhalations every 4-6 hours as needed for wheezing",
   "name": "Albuterol HFA inhaler"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Plaque psoriasis",
  "Mild intermittent asthma"
 ],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Plaque psoriasis", "Streptococcal pharyngitis"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-06-13, outpatient, Dermatology

Chief complaint: Itchy, scaly skin lesions

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Itchy, scaly skin lesions

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of itchy, scaly skin lesions. He reports that the lesions began on his elbows and have since spread to his knees and lower back. He denies any joint pain or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Plaque psoriasis
- Mild intermittent asthma

MEDICATIONS:
He is.

Home Medications:
- Albuterol HFA inhaler 90 mcg per actuation, 2 inhalations every 4-6 hours as needed for wheezing

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Plaque psoriasis
- Father: Hypertension
- Paternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 beers on weekends). Occupation: Carpenter.

PHYSICAL EXAMINATION:
On physical examination, you note well-demarcated, erythematous plaques with a white-silver scale on the extensor surfaces of his elbows and knees. There is no nail pitting or joint swelling.

PATHOLOGY:
A skin biopsy of one of the plaques shows parakeratosis and elongation of the rete ridges.

## Encounter 1: source 2020-07-15, practice 2024-12-12, outpatient, Dermatology

Chief complaint: Persistent skin lesions with new hand involvement

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent skin lesions with new hand involvement

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up evaluation of his plaque psoriasis, last assessed in January 2020. He presents with persistent skin lesions and new hand involvement. He reports the appearance of well-demarcated, erythematous plaques with silvery scales on his elbows and knees. Recently, he has noticed similar plaques on the dorsal surfaces of his hands, which he attributes to his manual job as a carpenter. He denies any joint pain or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Plaque psoriasis (diagnosed 2020-01-15)
- Mild intermittent asthma

MEDICATIONS:
He is.

Home Medications:
- Albuterol HFA inhaler 90 mcg per actuation, 2 inhalations every 4-6 hours as needed for wheezing

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Plaque psoriasis
- Father: Hypertension
- Paternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 beers on weekends). Occupation: Carpenter.

VITAL SIGNS:
His vital signs are within normal limits

PHYSICAL EXAMINATION:
On physical examination, you observe several erythematous plaques with overlying silvery scales on the extensor surfaces of his elbows and knees, as well as on the dorsal aspects of his hands. There is no nail pitting or joint swelling.

LABORATORY DATA:
Laboratory values:
- Complete blood count: Normal
- Erythrocyte sedimentation rate: Normal
- Rheumatoid factor: Negative
- Anti-cyclic citrullinated peptide antibodies: Negative
- HIV test: Negative

## Encounter 2: source 2021-07-15, practice 2025-12-16, outpatient, Internal Medicine

Chief complaint: Worsening red, scaly plaques on elbows, knees, and scalp

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Jennifer Park, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening red, scaly plaques on elbows, knees, and scalp

HISTORY OF PRESENT ILLNESS:
The patient is a 36-year-old male with established plaque psoriasis previously evaluated in Dermatology in January and July 2020, who presents with worsening skin lesions. He reports that the rash initially appeared a few days after he cut his arm on a piece of glass. He denies any recent travel or sick contacts but mentions that he had a sore throat a month ago that resolved without treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Plaque psoriasis (diagnosed 2020-01-15)
- Mild intermittent asthma

MEDICATIONS:
He takes albuterol inhaler as needed.

Home Medications:
- Albuterol HFA inhaler 90 mcg per actuation, 2 inhalations every 4-6 hours as needed for wheezing

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Plaque psoriasis
- Father: Hypertension
- Paternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 beers on weekends). Occupation: Carpenter.

PHYSICAL EXAMINATION:
On physical examination, you note well-demarcated erythematous plaques with silvery scales on the extensor surfaces and scalp. Auscultation of the lungs reveals no abnormalities.

LABORATORY DATA:
His recent laboratory tests show:

- Complete Blood Count (CBC):
- WBC: 7.2 x 10^3/uL (normal: 4.5-11.0 x 10^3/uL)
- Hemoglobin: 14.2 g/dL (normal: 13.8-17.2 g/dL)
- Platelets: 250 x 10^3/uL (normal: 150-450 x 10^3/uL)
- Antistreptolysin O (ASO) titer: Elevated
- HIV test: Negative
