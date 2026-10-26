# synthetic_hospital patient 2863

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex F, Non-Hispanic White, Medicare Part A and B.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-07 (Mon) | outpatient | Gastroenterology |
| 1 | 2020-09-15 | 2026-02-23 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare Part A and B",
 "occupation": "Retired elementary school principal",
 "alcohol_use": "None",
 "family_history": [
  "Maternal osteoporosis with hip fracture at age 81",
  "Paternal hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "70 mg orally once weekly (initiated 3 weeks prior)",
   "name": "Alendronate"
  },
  {
   "dose": "600 mg-400 IU orally twice daily",
   "name": "Calcium carbonate with Vitamin D3"
  },
  {
   "dose": "1 tablet orally daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy (1998)",
  "Cholecystectomy (2015)"
 ],
 "chronic_conditions": [
  "Osteoporosis",
  "Vitamin D deficiency"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Axillary nerve injury", "Pill-induced esophagitis"]
Comorbidities: ["Osteoporosis"]

## Encounter 0: source 2020-01-15, practice 2025-07-07, outpatient, Gastroenterology

Chief complaint: Persistent chest discomfort and difficulty swallowing over the past week

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent chest discomfort and difficulty swallowing over the past week

HISTORY OF PRESENT ILLNESS:
She describes a burning sensation in the middle of her chest that worsens when eating solid foods. She denies any shortness of breath, palpitations, nausea, or fever. Her symptoms are not relieved by antacids or dietary changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis
- Vitamin D deficiency

She has a history of osteoporosis.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (1998)
- Cholecystectomy (2015)

MEDICATIONS:
She takes for which she began a new medication three weeks ago. She takes no other medications besides a daily multivitamin.

Home Medications:
- Alendronate 70 mg orally once weekly (initiated 3 weeks prior)
- Calcium carbonate with Vitamin D3 600 mg-400 IU orally twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal osteoporosis with hip fracture at age 81
- Paternal hypertension

SOCIAL HISTORY:
She denies alcohol use and has never smoked.

REVIEW OF SYSTEMS:
Review of systems is negative for weight loss, hematemesis, or melena.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory studies, including complete blood count and comprehensive metabolic panel, are normal.

IMAGING:
ECG and chest x-ray performed in the emergency department two days ago were also unremarkable.

PLAN:
Upper endoscopy is scheduled, but the patient is concerned and wants an explanation now.

## Encounter 1: source 2020-09-15, practice 2026-02-23, ed, Emergency Medicine

Chief complaint: Fell directly onto her left shoulder while stepping off a curb with immediate pain and difficulty lifting the arm

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Michael Okonkwo, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fell directly onto her left shoulder while stepping off a curb with immediate pain and difficulty lifting the arm

HISTORY OF PRESENT ILLNESS:
She has osteoporosis and prior esophagitis. She reports immediate pain and difficulty lifting the arm.

PAST MEDICAL HISTORY:
Active Problem List:
- Pill-induced esophagitis (diagnosed 2020-01-15)
- Osteoporosis
- Vitamin D deficiency

She has a history of osteoporosis.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (1998)
- Cholecystectomy (2015)

MEDICATIONS:
Home Medications:
- Alendronate 70 mg orally once weekly (initiated 3 weeks prior)
- Calcium carbonate with Vitamin D3 600 mg-400 IU orally twice daily
- Multivitamin 1 tablet orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal osteoporosis with hip fracture at age 81
- Paternal hypertension

SOCIAL HISTORY:
Lives independently.

PHYSICAL EXAMINATION:
On examination, there is swelling and tenderness over the proximal humerus. Active range of motion is limited due to pain, but she can move her fingers and wrist normally. Radial and ulnar pulses are intact.

IMAGING:
An x-ray confirms the diagnosis.
