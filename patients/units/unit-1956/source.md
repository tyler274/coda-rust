# synthetic_hospital patient 1956

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, White, non-Hispanic, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-10 (Thu) | ed | Emergency Medicine |
| 1 | 2021-01-15 | 2025-10-13 (Mon) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Environmental Field Researcher",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother: Hypothyroidism",
  "Father: Hypertension controlled with medication",
  "Sister: Healthy"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet PO daily",
   "name": "Ethinyl estradiol/norgestimate"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Early disseminated Lyme disease with facial nerve palsy", "Acute Hepatitis A"]
Comorbidities: ["Exposure to Lyme disease (tick exposure in endemic area)"]

## Encounter 0: source 2020-01-15, practice 2024-10-10, ed, Emergency Medicine

Chief complaint: 5-day history of fever, malaise, and anorexia with dark urine and pale stools

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
5-day history of fever, malaise, and anorexia with dark urine and pale stools

HISTORY OF PRESENT ILLNESS:
She reports the onset of dark urine and pale stools two days ago.

PAST MEDICAL HISTORY:
She has no significant past medical history

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Hypertension controlled with medication
- Sister: Healthy

SOCIAL HISTORY:
Recently returned from a trip to a rural area with poor sanitation

VITAL SIGNS:
On examination, she is afebrile, with a blood pressure of 120/75 mmHg, heart rate of 88 beats per minute

PHYSICAL EXAMINATION:
Mild right upper quadrant tenderness on palpation. There is no scleral icterus or rash.

LABORATORY DATA:
- AST (SGOT): 1,200 U/L
- ALT (SGPT): 1,150 U/L
- Total bilirubin: 2.1 mg/dL
- Direct bilirubin: 1.5 mg/dL
- Alkaline phosphatase: 110 U/L
- White blood cell count: 6,000/mm³
- Hemoglobin: 13.2 g/dL
- Platelets: 250,000/mm³

## Encounter 1: source 2021-01-15, practice 2025-10-13, outpatient, Internal Medicine

Chief complaint: 3-day history of right-sided facial droop following camping in upstate New York

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
3-day history of right-sided facial droop following camping in upstate New York

HISTORY OF PRESENT ILLNESS:
The patient is a 31-year-old female with a history of acute Hepatitis A infection diagnosed in January 2020 who presents with a 3-day history of right-sided facial droop. She reports that 1 week ago, she experienced a flu-like illness with fever, chills, and myalgias, which resolved spontaneously. She denies any recent trauma or vesicular rash. She has a history of camping in the woods of upstate New York 2 weeks prior.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute Hepatitis A (diagnosed 2020-01-15)

MEDICATIONS:
Home Medications:
- Ethinyl estradiol/norgestimate 1 tablet PO daily
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Hypertension controlled with medication
- Sister: Healthy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Environmental Field Researcher.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, you note that she is unable to raise her right eyebrow, frown on the right side, or close her right eye completely. The rest of the cranial nerve examination is normal. There is no lymphadenopathy or rash.
