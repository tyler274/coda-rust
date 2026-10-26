# synthetic_hospital patient 2121

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-07-21 (Fri) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2024-07-15 (Mon) | ed | Emergency Medicine |
| 2 | 2022-07-15 | 2026-01-22 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "High school biology teacher",
 "alcohol_use": "Social (2-3 drinks per week)",
 "family_history": [
  "Father: Hypertension, Type 2 diabetes mellitus",
  "Mother: Breast cancer (diagnosed age 62)",
  "Maternal uncle: Non-Hodgkin lymphoma (diagnosed age 58)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Tonsillectomy, age 7"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Babesiosis", "Ehrlichiosis", "Lymphoma (Hodgkin or non-Hodgkin)"]
Comorbidities: ["Thrombocytopenia", "Splenomegaly"]

## Encounter 0: source 2020-01-15, practice 2023-07-21, outpatient, Internal Medicine

Chief complaint: 2-week history of progressive fatigue, night sweats, and a painless swelling in his left neck

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of progressive fatigue, night sweats, and a painless swelling in his left neck

HISTORY OF PRESENT ILLNESS:
2-week history of progressive fatigue, night sweats, and a painless swelling in his left neck

PAST MEDICAL HISTORY:
No significant past medical history

PAST SURGICAL HISTORY:
- Tonsillectomy, age 7

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Type 2 diabetes mellitus
- Mother: Breast cancer (diagnosed age 62)
- Maternal uncle: Non-Hodgkin lymphoma (diagnosed age 58)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per week). Occupation: High school biology teacher.

REVIEW OF SYSTEMS:
He denies fever, weight loss, or recent travel

PHYSICAL EXAMINATION:
A firm, non-tender, 3 cm lymph node in the left supraclavicular area. His spleen is palpable 2 cm below the left costal margin.

LABORATORY DATA:
- Hemoglobin: 13.5 g/dL (Normal: 13.8-17.2 g/dL)
- White blood cell count: 11,000/uL (Normal: 4,500-11,000/uL)
- Platelets: 250,000/uL (Normal: 150,000-450,000/uL)
- Lactate dehydrogenase (LDH): Elevated
- HIV test: Negative

IMAGING:
Chest X-ray: No mediastinal widening or lung lesions

PATHOLOGY:
Given the patient's demographic and clinical presentation, you decide to perform a biopsy of the lymph node. The pathology report indicates the presence of Reed-Sternberg cells.

ASSESSMENT AND PLAN:
Suspect Hodgkin lymphoma

Order a confirmatory test, which has a sensitivity of 95% and a specificity of 99% in a population with a disease prevalence of 0.2%. However, in this patient's demographic, the disease prevalence is 2%.

## Encounter 1: source 2021-01-15, practice 2024-07-15, ed, Emergency Medicine

Chief complaint: 1-week history of fever, chills, and malaise

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
1-week history of fever, chills, and malaise

HISTORY OF PRESENT ILLNESS:
One year following evaluation for fatigue, night sweats, and neck swelling, the patient presents with a 1-week history of fever, chills, and malaise. He reports a recent camping trip in upstate New York. He denies any known tick bites but recalls hiking through heavily wooded areas.

PAST MEDICAL HISTORY:
Active Problem List:
- Lymphoma (Hodgkin or non-Hodgkin) (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Tonsillectomy, age 7

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Type 2 diabetes mellitus
- Mother: Breast cancer (diagnosed age 62)
- Maternal uncle: Non-Hodgkin lymphoma (diagnosed age 58)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per week). Occupation: High school biology teacher.

VITAL SIGNS:
His temperature is 38.9°C (102°F), blood pressure is 130/85 mmHg, pulse is 100/min, and respiratory rate is 18/min

PHYSICAL EXAMINATION:
He appears fatigued but is otherwise in no acute distress. His spleen is palpable 2 cm below the left costal margin.

LABORATORY DATA:
- Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- Hematocrit: 30.4% (Normal: 40.7-50.3%)
- Platelets: 85,000/uL (Normal: 150,000-450,000/uL)
- Lactate dehydrogenase (LDH): 450 U/L (Normal: 140-280 U/L)
- Haptoglobin: <10 mg/dL (Normal: 30-200 mg/dL)
- Bilirubin, total: 2.1 mg/dL (Normal: 0.3-1.2 mg/dL)
- Bilirubin, direct: 0.3 mg/dL (Normal: 0.0-0.3 mg/dL)
- Peripheral blood smear: Presence of intraerythrocytic parasites with a 'Maltese cross' appearance

## Encounter 2: source 2022-07-15, practice 2026-01-22, ed, Emergency Medicine

Chief complaint: 5-day history of fever, headache, and myalgias

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
5-day history of fever, headache, and myalgias

HISTORY OF PRESENT ILLNESS:
The 47-year-old male with lymphoma (2020) and prior Babesiosis (2021) returns to the emergency department in July with a 5-day history of fever, headache, and myalgias. He reports a recent camping trip in Arkansas.

PAST MEDICAL HISTORY:
Active Problem List:
- Lymphoma (Hodgkin or non-Hodgkin) (diagnosed 2020-01-15)
- Babesiosis (diagnosed 2021-01-15)

PAST SURGICAL HISTORY:
- Tonsillectomy, age 7

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Type 2 diabetes mellitus
- Mother: Breast cancer (diagnosed age 62)
- Maternal uncle: Non-Hodgkin lymphoma (diagnosed age 58)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per week). Occupation: High school biology teacher.

VITAL SIGNS:
Temperature of 38.9°C (102°F), blood pressure of 120/75 mmHg, heart rate of 90 beats per minute, and respiratory rate of 18 breaths per minute

PHYSICAL EXAMINATION:
On examination, he is febrile with a He appears fatigued but is oriented and alert. There is no rash noted on skin examination.

LABORATORY DATA:
Laboratory findings are as follows: - White blood cell count: 3,000/mm³ (Normal: 4,500-11,000/mm³) - Hemoglobin: 14 g/dL (Normal: 13.8-17.2 g/dL) - Platelet count: 85,000/mm³ (Normal: 150,000-450,000/mm³) - Aspartate aminotransferase (AST): 120 U/L (Normal: 8-48 U/L) - Alanine aminotransferase (ALT): 100 U/L (Normal: 7-55 U/L)

ASSESSMENT AND PLAN:
Based on the patient's presentation and laboratory findings,
