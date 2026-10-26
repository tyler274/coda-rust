# synthetic_hospital patient 2579

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 26 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-10-15 (Wed) | outpatient | Psychiatry |
| 1 | 2020-09-15 | 2026-06-01 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Part-time retail stock associate",
 "alcohol_use": "Social, 1-2 drinks per month",
 "family_history": [
  "Father: Schizophrenia",
  "Mother: Major depressive disorder",
  "Older brother: Substance use disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1/2 pack per day",
 "home_medications": [
  {
   "dose": "2 mg tablet, take 1 by mouth at bedtime",
   "name": "Risperidone"
  },
  {
   "dose": "20 mg capsule, take 1 by mouth daily",
   "name": "Fluoxetine"
  },
  {
   "dose": "100 mg softgel, take 1 by mouth daily as needed for constipation",
   "name": "Docusate sodium"
  }
 ],
 "surgical_history": [
  "Tonsillectomy age 8",
  "Appendectomy age 19"
 ],
 "chronic_conditions": [
  "Schizophrenia, paranoid type, diagnosed age 23",
  "Major depressive disorder, recurrent, severe",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 26
}
```

Primary diagnoses: ["Schizophrenia with medication nonadherence", "Serotonin syndrome"]
Comorbidities: ["Medication noncompliance"]

## Encounter 0: source 2020-01-15, practice 2025-10-15, outpatient, Psychiatry

Chief complaint: Follow-up appointment for schizophrenia management with reported medication nonadherence and worsening paranoid delusions

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up appointment for schizophrenia management with reported medication nonadherence and worsening paranoid delusions

HISTORY OF PRESENT ILLNESS:
Despite initial improvements, his adherence to the medication regimen has been inconsistent due to his belief that the medications are "poisonous." He reports auditory hallucinations telling him not to trust his doctors, and his family notes that his social withdrawal and paranoid delusions have worsened over the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia, paranoid type, diagnosed age 23
- Major depressive disorder, recurrent, severe
- Tobacco use disorder

He was diagnosed 3 years ago.

PAST SURGICAL HISTORY:
- Tonsillectomy age 8
- Appendectomy age 19

MEDICATIONS:
He has been prescribed several oral antipsychotic medications, including risperidone and aripiprazole.

Home Medications:
- Fluoxetine 20 mg capsule, take 1 by mouth daily
- Docusate sodium 100 mg softgel, take 1 by mouth daily as needed for constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Schizophrenia
- Mother: Major depressive disorder
- Older brother: Substance use disorder

SOCIAL HISTORY:
Smoking: Current smoker, 1/2 pack per day. Alcohol: Social, 1-2 drinks per month. Occupation: Part-time retail stock associate.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, he appears disheveled, avoids eye contact, and responds to internal stimuli.

LABORATORY DATA:
Laboratory tests reveal no abnormalities.

ASSESSMENT AND PLAN:
Given his history and presentation,

## Encounter 1: source 2020-09-15, practice 2026-06-01, ed, Emergency Medicine

Chief complaint: Confusion, agitation, and muscle stiffness

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Confusion, agitation, and muscle stiffness

HISTORY OF PRESENT ILLNESS:
The 26-year-old male patient with schizophrenia, paranoid type, and major depressive disorder was last seen January 2020 for medication nonadherence. He is brought to the emergency department by his roommate due to confusion, agitation, and muscle stiffness. His psychiatrist recently switched him from fluoxetine to tranylcypromine. On the day of presentation, he reportedly took both medications along with over-the-counter dextromethorphan for a cough.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia with medication nonadherence (diagnosed 2020-01-15)
- Schizophrenia, paranoid type, diagnosed age 23
- Major depressive disorder, recurrent, severe
- Tobacco use disorder

He has a history of major depressive disorder that has been poorly controlled.

PAST SURGICAL HISTORY:
- Tonsillectomy age 8
- Appendectomy age 19

MEDICATIONS:
He takes fluoxetine, tranylcypromine, over-the-counter dextromethorphan.

Home Medications:
- Risperidone 2 mg tablet, take 1 by mouth at bedtime
- Docusate sodium 100 mg softgel, take 1 by mouth daily as needed for constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Schizophrenia
- Mother: Major depressive disorder
- Older brother: Substance use disorder

SOCIAL HISTORY:
Smoking: Current smoker, 1/2 pack per day. Alcohol: Social, 1-2 drinks per month. Occupation: Part-time retail stock associate.

VITAL SIGNS:
His temperature is 39.6°C (103.3°F), pulse is 128/min, blood pressure is 168/102 mm Hg, and respirations are 22/min.

PHYSICAL EXAMINATION:
On examination, the patient is disoriented and diaphoretic with bilateral lower extremity hyperreflexia and inducible ankle clonus. He has tremulous upper limbs and rigid lower limbs. Bowel sounds are hyperactive.

LABORATORY DATA:
Labs are pending.
