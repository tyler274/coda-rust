# synthetic_hospital patient 1991

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, African American, Blue Cross Blue Shield (Employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-13 (Sat) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2026-03-12 (Thu) | ed | Emergency Medicine |
| 2 | 2020-08-15 | 2026-04-15 (Wed) | icu | Neurosurgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield (Employer-sponsored)",
 "occupation": "Warehouse Supervisor",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father: Intracerebral hemorrhage at age 64",
  "Mother: Hypertension",
  "Brother: Substance use disorder"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Current smoker, 1 pack per day for 38 years",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "Appendectomy (age 22)"
 ],
 "chronic_conditions": [
  "Hypertension (poorly controlled)",
  "Cocaine use disorder",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Cocaine-induced cerebral vasculopathy", "Brain herniation"]
Comorbidities: ["Essential hypertension", "Hypertensive emergency"]

## Encounter 0: source 2020-01-15, practice 2025-09-13, ed, Emergency Medicine

Chief complaint: sudden onset of left-sided weakness and slurred speech. He also complains of a severe headache

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Marcus Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of left-sided weakness and slurred speech. He also complains of a severe headache

HISTORY OF PRESENT ILLNESS:
A 52-year-old male with a history of hypertension and smoking presents to the emergency department with a sudden onset of left-sided weakness and slurred speech that began 2 hours ago. He also complains of a severe headache that started around the same time as the weakness. His wife mentions he has been noncompliant with his antihypertensive medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension (poorly controlled)
- Cocaine use disorder
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Appendectomy (age 22)

MEDICATIONS:
He takes antihypertensive medications.

Home Medications:
- Amlodipine 10 mg PO daily
- Lisinopril 40 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Intracerebral hemorrhage at age 64
- Mother: Hypertension
- Brother: Substance use disorder

SOCIAL HISTORY:
Smoking

VITAL SIGNS:
On examination, his blood pressure is 220/120 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 beats per minute.

PHYSICAL EXAMINATION:
Neurological examination reveals left-sided hemiparesis and facial droop.

IMAGING:
A non-contrast head CT scan is performed immediately.

## Encounter 1: source 2020-07-15, practice 2026-03-12, ed, Emergency Medicine

Chief complaint: collapsing at work

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Collapsing at work

HISTORY OF PRESENT ILLNESS:
Patient with history of left-sided weakness 6 months ago, uncontrolled hypertension and cocaine use. Coworkers report that he was speaking normally earlier in the morning but suddenly developed weakness on the right side, became less responsive, and then slumped to the floor.

PAST MEDICAL HISTORY:
Active Problem List:
- Cocaine-induced cerebral vasculopathy (diagnosed 2020-01-15)
- Hypertension (poorly controlled)
- Cocaine use disorder
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Appendectomy (age 22)

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg PO daily
- Lisinopril 40 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Intracerebral hemorrhage at age 64
- Mother: Hypertension
- Brother: Substance use disorder

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 38 years. Alcohol: Occasional (1-2 drinks per month). Occupation: Warehouse Supervisor.

VITAL SIGNS:
On arrival, his blood pressure is 238/126 mm Hg and heart rate is 48/min.

PHYSICAL EXAMINATION:
Neurologic examination shows unresponsiveness to verbal commands, spontaneous flexion of the upper limbs with extension of the lower limbs, and pupils that are constricted to 1 mm bilaterally and minimally reactive to light. Corneal reflexes are intact, and breathing is irregular.

## Encounter 2: source 2020-08-15, practice 2026-04-15, icu, Neurosurgery

Chief complaint: sudden onset of severe headache and vomiting

ENCOUNTER NOTE
Date: 2020-08-15
Provider: Dr. James Okonkwo, MD
Department: Neurosurgery
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of severe headache and vomiting

HISTORY OF PRESENT ILLNESS:
The patient returns after prior cerebral vasculopathy. His wife reports he was found confused and not responding appropriately to questions.

PAST MEDICAL HISTORY:
Active Problem List:
- Cocaine-induced cerebral vasculopathy (diagnosed 2020-01-15)
- Hypertension (poorly controlled)
- Cocaine use disorder
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Appendectomy (age 22)

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg PO daily
- Lisinopril 40 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Intracerebral hemorrhage at age 64
- Mother: Hypertension
- Brother: Substance use disorder

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 38 years. Alcohol: Occasional (1-2 drinks per month). Occupation: Warehouse Supervisor.

VITAL SIGNS:
His blood pressure is 220/120 mmHg, heart rate is 50 bpm, and respiratory rate is irregular.

PHYSICAL EXAMINATION:
On examination, the patient is drowsy and opens his eyes to painful stimuli. His pupils are mid-sized and fixed.

IMAGING:
A CT scan of the head reveals an intracerebral hemorrhage with signs of transtentorial herniation.
