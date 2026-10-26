# synthetic_hospital patient 2627

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-09-19 (Thu) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2025-05-21 (Wed) | icu | Critical Care |
| 2 | 2021-07-15 | 2026-03-13 (Fri) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Self-employed carpenter",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Mother: major depressive disorder",
  "Paternal uncle: completed suicide",
  "Father: hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 1 year ago",
 "home_medications": [
  {
   "dose": "50 mg by mouth daily",
   "name": "Sertraline"
  },
  {
   "dose": "50 mg by mouth at bedtime as needed for sleep",
   "name": "Trazodone"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Major depressive disorder, recurrent, moderate",
  "Generalized anxiety disorder",
  "Insomnia"
 ],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Thermal inhalation injury of upper airway with impending obstruction", "Carbon monoxide poisoning without airway compromise"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-09-19, ed, Emergency Medicine

Chief complaint: Headache, dizziness, and confusion after working in garage with car engine running

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Headache, dizziness, and confusion after working in garage with car engine running

HISTORY OF PRESENT ILLNESS:
He was found in the garage with the car engine running for about an hour while he was working on a woodworking project. His wife mentions that he seemed "out of it" and was speaking incoherently.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder, recurrent, moderate
- Generalized anxiety disorder
- Insomnia

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg by mouth daily
- Trazodone 50 mg by mouth at bedtime as needed for sleep

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: major depressive disorder
- Paternal uncle: completed suicide
- Father: hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago. Alcohol: Social, 1-2 drinks per week. Occupation: Self-employed carpenter.

VITAL SIGNS:
His vital signs are as follows: temperature 37.0°C (98.6°F), blood pressure 135/85 mmHg, pulse 100/min, respiratory rate 20/min, and oxygen saturation 92% on room air

PHYSICAL EXAMINATION:
On physical examination, the patient is somnolent but arousable, with a flushed appearance. The remainder of the physical examination is noncontributory.

LABORATORY DATA:
An arterial blood gas shows:

- pH: 7.45
- PaCO2: 35 mmHg
- PaO2: 95 mmHg
- Bicarbonate: 24 mEq/L
- Carboxyhemoglobin: 25%

## Encounter 1: source 2020-09-15, practice 2025-05-21, icu, Critical Care

Chief complaint: Rescued from house fire with severe cough and difficulty speaking due to airway compromise

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Thompson, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Rescued from house fire with severe cough and difficulty speaking due to airway compromise

HISTORY OF PRESENT ILLNESS:
This 35-year-old male with mood and anxiety disorders and prior carbon monoxide exposure is coughing and has difficulty speaking.

PAST MEDICAL HISTORY:
Active Problem List:
- Carbon monoxide poisoning without airway compromise (diagnosed 2020-01-15)
- Major depressive disorder, recurrent, moderate
- Generalized anxiety disorder
- Insomnia

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg by mouth daily
- Trazodone 50 mg by mouth at bedtime as needed for sleep

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: major depressive disorder
- Paternal uncle: completed suicide
- Father: hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago. Alcohol: Social, 1-2 drinks per week. Occupation: Self-employed carpenter.

PHYSICAL EXAMINATION:
On examination, he has burns on his face, singeing of his eyebrows, and soot around his nostrils. His voice is hoarse, and he is in moderate respiratory distress. Auscultation of the chest reveals wheezing. His oropharynx is erythematous with blisters on the soft palate.

LABORATORY DATA:
Arterial blood gas shows a PaO2 of 60 mmHg, PaCO2 of 35 mmHg, and carboxyhemoglobin level of 15%.

## Encounter 2: source 2021-07-15, practice 2026-03-13, icu, Emergency Medicine

Chief complaint: Found unconscious in garage with car engine running

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Sharma, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Found unconscious in garage with car engine running

PAST MEDICAL HISTORY:
Active Problem List:
- Carbon monoxide poisoning without airway compromise (diagnosed 2020-01-15)
- Thermal inhalation injury of upper airway with impending obstruction (diagnosed 2020-09-15)
- Major depressive disorder, recurrent, moderate
- Generalized anxiety disorder
- Insomnia

The patient has a history of depression and was recently discharged from a psychiatric facility.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg by mouth daily
- Trazodone 50 mg by mouth at bedtime as needed for sleep

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: major depressive disorder
- Paternal uncle: completed suicide
- Father: hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago. Alcohol: Social, 1-2 drinks per week. Occupation: Self-employed carpenter.

VITAL SIGNS:
His vital signs are: blood pressure 130/85 mmHg, heart rate 98 bpm, respiratory rate 22 breaths/min, and oxygen saturation of 92% on room air.

PHYSICAL EXAMINATION:
On examination, the patient is drowsy but arousable, with a headache and confusion. Physical examination reveals bright red skin coloration. Neurological examination is non-focal.

LABORATORY DATA:
The patient's carboxyhemoglobin level is found to be elevated. Arterial blood gas analysis shows the following:

- pH: 7.28
- PaCO2: 38 mmHg
- PaO2: 95 mmHg
- HCO3-: 18 mEq/L
- Lactate: 5 mmol/L
