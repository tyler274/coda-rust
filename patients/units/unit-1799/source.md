# synthetic_hospital patient 1799

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Employer-sponsored commercial insurance (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-10-13 (Mon) | outpatient | Psychiatry |
| 1 | 2020-04-15 | 2026-01-02 (Fri) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2026-03-31 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored commercial insurance (Blue Cross Blue Shield)",
 "occupation": "Registered Nurse, Medical-Surgical Unit",
 "alcohol_use": "Social (1-2 drinks on weekends)",
 "family_history": [
  "Mother: Generalized anxiety disorder",
  "Father: Hypertension",
  "Sister: Healthy"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "0.5 mg tablet, take 1/2 tablet by mouth every 8 hours as needed for anxiety (max 1 tablet per day)",
   "name": "Alprazolam"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction, age 22"
 ],
 "chronic_conditions": [
  "Generalized anxiety disorder"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Benzodiazepine overdose", "Occupational exposure to HIV via needlestick injury", "Acute stress reaction"]
Comorbidities: ["Acute stress reaction"]

## Encounter 0: source 2020-01-15, practice 2025-10-13, outpatient, Psychiatry

Chief complaint: Insomnia, irritability, and flashbacks following motor vehicle accident

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Insomnia, irritability, and flashbacks following motor vehicle accident

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 3-week history of insomnia, irritability, and flashbacks following a severe car accident in which she was a passenger. She reports feeling constantly on edge and has been avoiding driving or being a passenger in a car since the accident.

PAST MEDICAL HISTORY:
Active Problem List:
- Generalized anxiety disorder

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 22

MEDICATIONS:
Home Medications:
- Alprazolam 0.5 mg tablet, take 1/2 tablet by mouth every 8 hours as needed for anxiety (max 1 tablet per day)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Generalized anxiety disorder
- Father: Hypertension
- Sister: Healthy

SOCIAL HISTORY:
She denies any substance abuse

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, she appears anxious and has an increased startle response.

LABORATORY DATA:
Her laboratory tests including a complete blood count, comprehensive metabolic panel, and thyroid function tests are normal.

## Encounter 1: source 2020-04-15, practice 2026-01-02, ed, Emergency Medicine

Chief complaint: Accidental needlestick injury from HIV-positive patient

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Accidental needlestick injury from HIV-positive patient

HISTORY OF PRESENT ILLNESS:
This 28-year-old female with generalized anxiety disorder and a psychiatric evaluation three months prior for acute stress reaction following a motor vehicle accident presents to the emergency department 30 minutes after accidentally sticking herself with a needle she had used on a patient known to be HIV positive. The patient was receiving treatment for HIV with a detectable viral load. The nurse is visibly anxious. She is concerned about her risk of contracting HIV and inquires about the necessary steps she should take following this exposure.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute stress reaction (diagnosed 2020-01-15)
- Generalized anxiety disorder

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 22

MEDICATIONS:
Is currently.

Home Medications:
- Alprazolam 0.5 mg tablet, take 1/2 tablet by mouth every 8 hours as needed for anxiety (max 1 tablet per day)

ALLERGIES:
She denies any allergies

FAMILY HISTORY:
- Mother: Generalized anxiety disorder
- Father: Hypertension
- Sister: Healthy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks on weekends). Occupation: Registered Nurse, Medical-Surgical Unit.

REVIEW OF SYSTEMS:
She has no other symptoms

VITAL SIGNS:
On examination, her vital signs are within normal limits

PHYSICAL EXAMINATION:
The physical examination is unremarkable except for a small puncture wound on her left index finger

LABORATORY DATA:
- HIV antibody/antigen test (nurse): Negative
- Hepatitis B surface antigen (nurse): Negative
- Hepatitis C antibody (nurse): Negative

## Encounter 2: source 2020-07-15, practice 2026-03-31, ed, Emergency Medicine

Chief complaint: Excessive sedation and slurred speech at social gathering

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Excessive sedation and slurred speech at social gathering

HISTORY OF PRESENT ILLNESS:
This 28-year-old female with generalized anxiety disorder, previously evaluated for acute stress reaction following a motor vehicle accident and occupational HIV needlestick exposure, presents with excessive sedation and slurred speech. They report that she has been under significant stress at work and

PAST MEDICAL HISTORY:
Active Problem List:
- Acute stress reaction (diagnosed 2020-01-15)
- Occupational exposure to HIV via needlestick injury (diagnosed 2020-04-15)
- Generalized anxiety disorder

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 22

MEDICATIONS:
For which she occasionally takes medication, but they are unsure of the name. A friend hands you a pill bottle of alprazolam that they found in her purse, which appears to be less than half full.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Generalized anxiety disorder
- Father: Hypertension
- Sister: Healthy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks on weekends). Occupation: Registered Nurse, Medical-Surgical Unit.

VITAL SIGNS:
Her vital signs are: blood pressure 90/60 mmHg, heart rate 55 bpm, respiratory rate 8/min, and oxygen saturation 92% on room air.

PHYSICAL EXAMINATION:
On examination, the patient is somnolent but arousable, with slurred speech and ataxia. Her pupils are normal in size and reactive to light. No needle marks are noted on her extremities.
