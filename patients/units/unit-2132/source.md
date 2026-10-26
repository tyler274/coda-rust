# synthetic_hospital patient 2132

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-03 (Thu) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2026-04-07 (Tue) | outpatient | Orthopedics & Sports Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO",
 "occupation": "Remote software developer",
 "alcohol_use": "None",
 "family_history": [
  "Non-contributory"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Vitamin D deficiency", "Achilles tendinitis"]
Comorbidities: ["Long-distance running"]

## Encounter 0: source 2020-01-15, practice 2025-04-03, outpatient, Internal Medicine

Chief complaint: worsening fatigue, diffuse bone pain, and muscle weakness over the past several months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening fatigue, diffuse bone pain, and muscle weakness over the past several months

PAST MEDICAL HISTORY:
He reports no significant past medical history

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Non-contributory

SOCIAL HISTORY:
He works remotely full-time and rarely leaves his apartment. His diet consists primarily of processed foods with minimal dairy or fortified products. He does not smoke or drink alcohol.

VITAL SIGNS:
Temperature is 36.5°C (97.7°F), pulse is 72/min, and blood pressure is 128/76 mmHg. BMI is 22 kg/m².

PHYSICAL EXAMINATION:
Musculoskeletal examination reveals diffuse tenderness over both tibias and proximal muscle weakness in the thighs bilaterally.

LABORATORY DATA:
Laboratory evaluation reveals:
Calcium Low
Phosphate Low
Parathyroid hormone Elevated

## Encounter 1: source 2021-01-15, practice 2026-04-07, outpatient, Orthopedics & Sports Medicine

Chief complaint: 3-month history of pain in the right ankle

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. David Kim, MD
Department: Orthopedics & Sports Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of pain in the right ankle

HISTORY OF PRESENT ILLNESS:
The patient presents for evaluation of right ankle pain, one year after prior internal medicine encounter for fatigue, diffuse bone pain, and muscle weakness. He reports that the pain worsens with walking and is relieved by rest. He is an avid runner and increased running distance significantly 4 months ago in preparation for a marathon. He denies trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin D deficiency (diagnosed 2020-01-15)

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Non-contributory

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Remote software developer.

PHYSICAL EXAMINATION:
On physical examination, there is tenderness to palpation 2 cm proximal to the insertion of the Achilles tendon on the right calcaneus. There is no erythema, warmth, or swelling. Ankle dorsiflexion against resistance elicits pain. The patient's ankle range of motion is otherwise normal.

IMAGING:
Plain radiographs of the ankle are unremarkable.
