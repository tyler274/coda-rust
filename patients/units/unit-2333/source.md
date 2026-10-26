# synthetic_hospital patient 2333

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-17 (Mon) | outpatient | Hematology & Oncology |
| 1 | 2020-09-15 | 2025-10-07 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Father: Chronic lymphocytic leukemia (diagnosed age 72)",
  "Mother: Hypertension",
  "Older brother: Prostate cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (quit 15 years ago, 12 pack-year history)",
 "home_medications": [
  {
   "dose": "300 mg PO daily",
   "name": "Allopurinol"
  },
  {
   "dose": "500 mg PO daily",
   "name": "Valacyclovir"
  },
  {
   "dose": "8 mg PO every 8 hours PRN nausea",
   "name": "Ondansetron"
  }
 ],
 "surgical_history": [
  "Appendectomy (age 14)",
  "Right knee arthroscopy (2018)"
 ],
 "chronic_conditions": [
  "Chronic lymphocytic leukemia",
  "Chemotherapy-induced anemia"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Transfusion-related acute lung injury", "Chronic lymphocytic leukemia"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-02-17, outpatient, Hematology & Oncology

Chief complaint: Elevated white blood cell count on routine blood work

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Elevated white blood cell count on routine blood work

HISTORY OF PRESENT ILLNESS:
He reports mild fatigue but no fevers, night sweats, or weight loss. There is no history of recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic lymphocytic leukemia
- Chemotherapy-induced anemia

PAST SURGICAL HISTORY:
- Appendectomy (age 14)
- Right knee arthroscopy (2018)

MEDICATIONS:
Home Medications:
- Allopurinol 300 mg PO daily
- Valacyclovir 500 mg PO daily
- Ondansetron 8 mg PO every 8 hours PRN nausea

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic lymphocytic leukemia (diagnosed age 72)
- Mother: Hypertension
- Older brother: Prostate cancer

SOCIAL HISTORY:
Smoking: Former smoker (quit 15 years ago, 12 pack-year history). Alcohol: Social (1-2 drinks per month). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
His physical examination reveals non-tender cervical lymphadenopathy and mild splenomegaly. He has no palpable purpura or petechiae.

LABORATORY DATA:
Laboratory results show:
Hemoglobin 12.4 g/dL
Platelets 190,000/µL
Leukocytes 80,000/µL with 90% lymphocytes

OTHER STUDIES:
A peripheral blood smear is shown below.

## Encounter 1: source 2020-09-15, practice 2025-10-07, ed, Emergency Medicine

Chief complaint: Fever and chills two hours after packed red blood cell transfusion

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever and chills two hours after packed red blood cell transfusion

HISTORY OF PRESENT ILLNESS:
This 66-year-old male with chronic lymphocytic leukemia diagnosed in January 2020 returns to the emergency department with fever and chills that started two hours after receiving a packed red blood cell transfusion for chemotherapy-induced anemia.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic lymphocytic leukemia (diagnosed 2020-01-15)
- Chemotherapy-induced anemia

PAST SURGICAL HISTORY:
- Appendectomy (age 14)
- Right knee arthroscopy (2018)

MEDICATIONS:
Home Medications:
- Allopurinol 300 mg PO daily
- Valacyclovir 500 mg PO daily
- Ondansetron 8 mg PO every 8 hours PRN nausea

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic lymphocytic leukemia (diagnosed age 72)
- Mother: Hypertension
- Older brother: Prostate cancer

SOCIAL HISTORY:
Smoking: Former smoker (quit 15 years ago, 12 pack-year history). Alcohol: Social (1-2 drinks per month). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His temperature is 38.7°C (101.7°F), blood pressure is 130/85 mmHg, pulse is 88/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
Physical examination reveals no rash or signs of volume overload. His lungs are clear to auscultation, and his heart rhythm is regular without murmurs. The remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory studies show no evidence of hemolysis, and his white blood cell count is within normal limits. Blood cultures have been drawn and are pending.
