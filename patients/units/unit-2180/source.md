# synthetic_hospital patient 2180

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 40 at first encounter, sex M, Hispanic, Aetna PPO (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-07 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2025-09-30 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Aetna PPO (employer-sponsored)",
 "occupation": "Financial analyst",
 "alcohol_use": "Occasional (1-2 drinks per weekend)",
 "family_history": [
  "Father: Hypertension, hyperlipidemia",
  "Mother: Type 2 diabetes, obesity",
  "Maternal uncle: Lumbar disc herniation requiring discectomy"
 ],
 "race_ethnicity": "Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "500 mg PO PRN for occasional headaches",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 8)",
  "Knee arthroscopy for meniscal tear (2018, right knee)"
 ],
 "chronic_conditions": [
  "Obesity (BMI 32)",
  "Essential hypertension",
  "Chronic mechanical low back pain"
 ],
 "age_at_first_encounter": 40
}
```

Primary diagnoses: ["L5-S1 intervertebral disc herniation", "S1 radiculopathy"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-04-07, outpatient, Internal Medicine

Chief complaint: 3 weeks of progressive lower back pain radiating down the back of his left leg to the foot

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
3 weeks of progressive lower back pain radiating down the back of his left leg to the foot

HISTORY OF PRESENT ILLNESS:
He describes the pain as sharp and shooting, worsened by sitting and improved by lying flat. He denies bowel or bladder symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Obesity (BMI 32)
- Essential hypertension
- Chronic mechanical low back pain

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)
- Knee arthroscopy for meniscal tear (2018, right knee)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily
- Acetaminophen 500 mg PO PRN for occasional headaches

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, hyperlipidemia
- Mother: Type 2 diabetes, obesity
- Maternal uncle: Lumbar disc herniation requiring discectomy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per weekend). Occupation: Financial analyst.

PHYSICAL EXAMINATION:
Neurologic exam shows decreased sensation over the lateral aspect of the foot, reduced strength in plantarflexion, and an absent Achilles reflex on the left. Dorsiflexion and knee extension are intact. Straight leg raise is positive on the left side.

## Encounter 1: source 2020-07-15, practice 2025-09-30, ed, Emergency Medicine

Chief complaint: sudden-onset low back pain while lifting heavy boxes during a move

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Okonkwo, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset low back pain while lifting heavy boxes during a move

HISTORY OF PRESENT ILLNESS:
The patient is a 40-year-old male with obesity, essential hypertension, and chronic mechanical low back pain evaluated in January 2020 for lower back pain radiating to the left leg, who presents to the emergency department after developing sudden-onset low back pain while lifting heavy boxes during a move. The pain began abruptly when he twisted to his left while holding a box, and he describes it as sharp, shooting, and radiating down the posterior aspect of his left leg to the lateral foot. He reports tingling in his left foot and difficulty walking due to pain. He denies fever, recent trauma, weight loss, or prior similar episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- S1 radiculopathy (diagnosed 2020-01-15)
- Obesity (BMI 32)
- Essential hypertension
- Chronic mechanical low back pain

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)
- Knee arthroscopy for meniscal tear (2018, right knee)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily
- Acetaminophen 500 mg PO PRN for occasional headaches

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, hyperlipidemia
- Mother: Type 2 diabetes, obesity
- Maternal uncle: Lumbar disc herniation requiring discectomy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per weekend). Occupation: Financial analyst.

VITAL SIGNS:
On physical examination, he is afebrile with stable vital signs.

PHYSICAL EXAMINATION:
He leans forward slightly and favors the right leg while walking. Straight leg raise is positive on the left at 30 degrees. Sensory testing shows decreased sensation over the lateral aspect of the left foot. Motor testing reveals mild weakness with plantar flexion on the left. The left Achilles reflex is diminished.
