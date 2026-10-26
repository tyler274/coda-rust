# synthetic_hospital patient 2135

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 40 at first encounter, sex M, Non-Hispanic White, Commercial PPO (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-08-02 (Sat) | ed | Emergency Medicine |
| 1 | 2021-01-15 | 2026-07-29 (Wed) | outpatient | Dermatology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO (Blue Cross Blue Shield)",
 "occupation": "Landscaper",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Malignant melanoma (diagnosed age 55, deceased)",
  "Mother: Type 2 diabetes mellitus",
  "Paternal uncle: Basal cell carcinoma"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Emergency fasciotomy with escharotomies, bilateral lower extremities (age 40)",
  "Wide local excision of left forearm malignant melanoma with sentinel lymph node biopsy (age 41)"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 40
}
```

Primary diagnoses: ["Acute compartment syndrome of lower extremity due to circumferential full-thickness burns", "Malignant melanoma"]
Comorbidities: ["Smoke inhalation exposure"]

## Encounter 0: source 2020-01-15, practice 2025-08-02, ed, Emergency Medicine

Chief complaint: Severe pain in his legs after being rescued from a house fire with circumferential burns to both lower legs

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe pain in his legs after being rescued from a house fire with circumferential burns to both lower legs

HISTORY OF PRESENT ILLNESS:
He is brought to the emergency department after being rescued from a house fire. He has sustained circumferential burns to both lower legs. He complains of severe pain in his legs, which is not relieved by intravenous morphine.

PAST MEDICAL HISTORY:
His past medical history is unremarkable.

PAST SURGICAL HISTORY:
- Emergency fasciotomy with escharotomies, bilateral lower extremities (age 40)
- Wide local excision of left forearm malignant melanoma with sentinel lymph node biopsy (age 41)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Malignant melanoma (diagnosed age 55, deceased)
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Basal cell carcinoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Landscaper.

VITAL SIGNS:
Vital signs are as follows: blood pressure 130/85 mmHg, heart rate 110 bpm, respiratory rate 22 breaths/min, and temperature 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
On examination, the burned areas appear white and leathery, and he is unable to move his toes.
Physical examination reveals tense compartments in the lower legs with diminished pulses.

LABORATORY DATA:
Laboratory values are:
- Hemoglobin: 14.5 g/dL
- White blood cell count: 16,000/mm³
- Platelets: 250,000/mm³
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL
- Blood urea nitrogen: 18 mg/dL

## Encounter 1: source 2021-01-15, practice 2026-07-29, outpatient, Dermatology

Chief complaint: Evaluation of a pigmented lesion on his left forearm that has increased in size and changed color over several months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of a pigmented lesion on his left forearm that has increased in size and changed color over several months

HISTORY OF PRESENT ILLNESS:
The patient returns one year after treatment for fire-related compartment syndrome and burns. He first noticed the lesion several months ago and reports that it has recently increased in size and changed in color.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute compartment syndrome of lower extremity due to circumferential full-thickness burns (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Emergency fasciotomy with escharotomies, bilateral lower extremities (age 40)
- Wide local excision of left forearm malignant melanoma with sentinel lymph node biopsy (age 41)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Malignant melanoma (diagnosed age 55, deceased)
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Basal cell carcinoma

SOCIAL HISTORY:
Works as a landscaper and spends long hours in the sun

PHYSICAL EXAMINATION:
The lesion is shown in the image below. Dermoscopy reveals irregular pigment network and atypical dots and globules.

OTHER STUDIES:
An excisional biopsy is performed, and histopathology is pending.
