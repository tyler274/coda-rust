# synthetic_hospital patient 2140

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex M, White, non-Hispanic, Federal Employee Health Benefits (FEHB) - Blue Cross Blue Shield Standard.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-08 (Sun) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2026-02-21 (Sat) | icu | General Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Federal Employee Health Benefits (FEHB) - Blue Cross Blue Shield Standard",
 "occupation": "United States Postal Service mail processing clerk",
 "alcohol_use": "Social, approximately 3-4 alcoholic beverages per week",
 "family_history": [
  "Father: Hypertension, diagnosed age 60",
  "Mother: Hypothyroidism",
  "Paternal uncle: Malignant melanoma"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Former smoker, 8 pack-year history, quit 5 years ago",
 "home_medications": [],
 "surgical_history": [
  "Tonsillectomy and adenoidectomy, age 10",
  "Left ankle open reduction internal fixation, age 24"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Burn wound infection requiring prophylactic antibiotics", "Inhalational anthrax"]
Comorbidities: ["Septic shock"]

## Encounter 0: source 2020-01-15, practice 2025-06-08, ed, Emergency Medicine

Chief complaint: Acute confusion and disorientation with fever and non-productive cough following 2 days of flu-like symptoms.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute confusion and disorientation with fever and non-productive cough following 2 days of flu-like symptoms.

HISTORY OF PRESENT ILLNESS:
He initially developed low-grade fever, fatigue, and myalgias about 2 days ago, which were attributed to a viral illness. His symptoms worsened with nausea, mild abdominal discomfort, and non-productive cough. This morning, he was found diaphoretic and confused in the office break room. His coworkers mention that he had recently returned to work after a week-long vacation and had received a small padded envelope with no return address upon his return. He also has mild retrosternal chest discomfort and has been hyperventilating.

PAST MEDICAL HISTORY:
Past medical history is unremarkable.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 10
- Left ankle open reduction internal fixation, age 24

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, diagnosed age 60
- Mother: Hypothyroidism
- Paternal uncle: Malignant melanoma

SOCIAL HISTORY:
Smoking: Former smoker, 8 pack-year history, quit 5 years ago. Alcohol: Social, approximately 3-4 alcoholic beverages per week. Occupation: United States Postal Service mail processing clerk.

VITAL SIGNS:
Temperature is 38.4°C (101.1°F), blood pressure is 92/58 mm Hg, pulse is 114/min, and respirations are 30/min. Oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
Physical examination shows mild scleral injection and decreased breath sounds bilaterally. Neurologic exam reveals mild confusion but no focal deficits.

LABORATORY DATA:
Blood cultures are pending.

IMAGING:
Chest x-ray shows a widened mediastinum with clear lung fields.

## Encounter 1: source 2020-09-15, practice 2026-02-21, icu, General Surgery

Chief complaint: Fever and suspected burn wound infection on hospital day 7 following house fire with 30% BSA thermal injury.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Okonkwo, MD
Department: General Surgery
Visit Type: icu

CHIEF COMPLAINT:
Fever and suspected burn wound infection on hospital day 7 following house fire with 30% BSA thermal injury.

HISTORY OF PRESENT ILLNESS:
The patient is a 42-year-old male with a history of inhalational anthrax who presents with fever on hospital day 7 following admission for burns sustained in a house fire. He was admitted to the burn unit with second and third-degree burns covering 30% of his body surface area, including his arms, chest, and abdomen. He was initially stabilized, intubated, and fluid resuscitated. On hospital day 7, the patient develops a fever,

PAST MEDICAL HISTORY:
Active Problem List:
- Inhalational anthrax (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 10
- Left ankle open reduction internal fixation, age 24

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, diagnosed age 60
- Mother: Hypothyroidism
- Paternal uncle: Malignant melanoma

SOCIAL HISTORY:
Smoking: Former smoker, 8 pack-year history, quit 5 years ago. Alcohol: Social, approximately 3-4 alcoholic beverages per week. Occupation: United States Postal Service mail processing clerk.

VITAL SIGNS:
38.9°C (102°F), tachycardia, and hypotension.

PHYSICAL EXAMINATION:
The burn wounds on his chest and abdomen, which were previously clean and covered with skin grafts, now appear erythematous with greenish-black discoloration at the margins. There is a new foul odor noted, and the skin grafts appear to be detaching.

LABORATORY DATA:
His white blood cell count is elevated at 16,000/uL.
