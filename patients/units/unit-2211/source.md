# synthetic_hospital patient 2211

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental Plan F.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-03-10 (Wed) | outpatient | Neurology |
| 1 | 2020-07-15 | 2021-09-13 (Mon) | outpatient | Neurology |
| 2 | 2021-07-15 | 2022-08-31 (Wed) | outpatient | Neurology |
| 3 | 2023-01-15 | 2024-03-04 (Mon) | outpatient | Geriatrics |
| 4 | 2025-01-15 | 2026-02-27 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental Plan F",
 "occupation": "Retired municipal accountant",
 "alcohol_use": "Rare (1-2 drinks on holidays)",
 "family_history": [
  "Father: Parkinson disease, diagnosed age 70",
  "Mother: Type 2 diabetes mellitus, hypertension",
  "Paternal uncle: Intracerebral hemorrhage (fatal)",
  "Brother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25/100 mg PO TID",
   "name": "Carbidopa/Levodopa"
  },
  {
   "dose": "0.5 mg PO TID",
   "name": "Ropinirole"
  },
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "3 mg PO qHS",
   "name": "Melatonin"
  }
 ],
 "surgical_history": [
  "Cholecystectomy (laparoscopic, age 54)",
  "Bilateral cataract surgery (age 66)"
 ],
 "chronic_conditions": [
  "Parkinson Disease (diagnosed age 63)",
  "Parkinson Disease Dementia",
  "REM Sleep Behavior Disorder",
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Dyslipidemia"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Parkinson disease with psychosis", "Dopamine agonist-induced hallucinations", "Parkinson disease dementia", "Intracerebral hemorrhage with airway compromise"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-03-10, outpatient, Neurology

Chief complaint: 6-month history of a tremor in his right hand and progressively smaller handwriting.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of a tremor in his right hand and progressively smaller handwriting.

HISTORY OF PRESENT ILLNESS:
A 67-year-old male presents to the clinic with a 6-month history of a tremor in his right hand. He reports that the tremor is more noticeable at rest and seems to temporarily improve with voluntary movement. The patient also mentions that his handwriting has become smaller and that he has difficulty with buttoning his shirt. He denies any pain associated with the tremor.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson Disease (diagnosed age 63)
- Parkinson Disease Dementia
- REM Sleep Behavior Disorder
- Hypertension
- Type 2 Diabetes Mellitus
- Dyslipidemia

His past medical history is significant for hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Cholecystectomy (laparoscopic, age 54)
- Bilateral cataract surgery (age 66)

MEDICATIONS:
Home Medications:
- Carbidopa/Levodopa 25/100 mg PO TID
- Ropinirole 0.5 mg PO TID
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily
- Melatonin 3 mg PO qHS

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson disease, diagnosed age 70
- Mother: Type 2 diabetes mellitus, hypertension
- Paternal uncle: Intracerebral hemorrhage (fatal)
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks on holidays). Occupation: Retired municipal accountant.

PHYSICAL EXAMINATION:
On physical examination, you observe a rhythmic, "pill-rolling" tremor of his right hand at rest, which diminishes with intentional movement. Muscle rigidity is noted in the right upper extremity. His gait is characterized by reduced arm swing on the right side. There are no signs of cerebellar dysfunction, and the remainder of the neurological examination is unremarkable.

LABORATORY DATA:
His most recent laboratory values are as follows:

- Fasting glucose: 130 mg/dL
- HbA1c: 7.2%
- Serum creatinine: 1.0 mg/dL
- TSH: 2.5 mIU/L

## Encounter 1: source 2020-07-15, practice 2021-09-13, outpatient, Neurology

Chief complaint: Progressive difficulty in buttoning his shirts and a persistent 'pill-rolling' motion of his right hand.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Raj Patel, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty in buttoning his shirts and a persistent 'pill-rolling' motion of his right hand.

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with Parkinson Disease diagnosed at age 63 returning for follow-up after his January 2020 visit for tremor and micrographia. He presents with a 6-month history of progressive difficulty in buttoning his shirts and a persistent "pill-rolling" motion of his right hand. He also reports his wife has noticed that his facial expression has become less animated. He denies any pain, weakness, or sensory changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Parkinson Disease (diagnosed age 63)
- Parkinson Disease Dementia
- REM Sleep Behavior Disorder
- Hypertension
- Type 2 Diabetes Mellitus
- Dyslipidemia

His past medical history is significant for hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Cholecystectomy (laparoscopic, age 54)
- Bilateral cataract surgery (age 66)

MEDICATIONS:
He takes hydrochlorothiazide and metformin.

Home Medications:
- Carbidopa/Levodopa 25/100 mg PO TID
- Ropinirole 0.5 mg PO TID
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily
- Melatonin 3 mg PO qHS

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson disease, diagnosed age 70
- Mother: Type 2 diabetes mellitus, hypertension
- Paternal uncle: Intracerebral hemorrhage (fatal)
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks on holidays). Occupation: Retired municipal accountant.

PHYSICAL EXAMINATION:
On examination, you observe a resting tremor in his right hand, which subsides with voluntary movement. Muscle tone is increased in the right upper extremity, and there is cogwheel rigidity on passive movement of the right wrist. His gait is slow with reduced arm swing on the right. There are no signs of cerebellar dysfunction, and the rest of the neurological examination is unremarkable.

LABORATORY DATA:
Fasting glucose: 130 mg/dL
- HbA1c: 7.2%
- Serum creatinine: 1.0 mg/dL
- Complete blood count: within normal limits

## Encounter 2: source 2021-07-15, practice 2022-08-31, outpatient, Neurology

Chief complaint: Experiencing visual hallucinations for the past two months, describing small animals running across the floor.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Experiencing visual hallucinations for the past two months, describing small animals running across the floor.

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with Parkinson disease returning with visual disturbances. He was seen for tremor, micrographia, and pill-rolling motions. He describes seeing small animals running across the floor, which are not seen by others. Parkinson symptoms are well-controlled on his current medication regimen. He denies recent head trauma, loss of consciousness, or illicit drug use.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Parkinson Disease (diagnosed age 63)
- Parkinson Disease Dementia
- REM Sleep Behavior Disorder
- Hypertension
- Type 2 Diabetes Mellitus
- Dyslipidemia

His past medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cholecystectomy (laparoscopic, age 54)
- Bilateral cataract surgery (age 66)

MEDICATIONS:
His current medications include carbidopa/levodopa, ropinirole, metformin, and lisinopril.

Home Medications:
- Atorvastatin 20 mg PO daily
- Melatonin 3 mg PO qHS

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson disease, diagnosed age 70
- Mother: Type 2 diabetes mellitus, hypertension
- Paternal uncle: Intracerebral hemorrhage (fatal)
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks on holidays). Occupation: Retired municipal accountant.

VITAL SIGNS:
On examination, his blood pressure is 135/85 mmHg, heart rate is 78 bpm, and other vital signs are within normal limits.

PHYSICAL EXAMINATION:
Neurological examination reveals a resting tremor, bradykinesia, and rigidity. There are no signs of focal neurological deficits.

LABORATORY DATA:
Laboratory tests reveal: Hemoglobin A1c: 7.2%, Serum creatinine: 1.0 mg/dL, Liver function tests: within normal limits.

## Encounter 3: source 2023-01-15, practice 2024-03-04, outpatient, Geriatrics

Chief complaint: Episodes of confusion and memory loss that fluctuate throughout the day.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James O'Brien, MD
Department: Geriatrics
Visit Type: outpatient

CHIEF COMPLAINT:
Episodes of confusion and memory loss that fluctuate throughout the day.

HISTORY OF PRESENT ILLNESS:
The patient returns for Geriatrics follow-up in the context of an 8-year history of Parkinson disease previously evaluated in Neurology for tremor, micrographia, and visual hallucinations. He presents with his daughter who reports that he has been experiencing episodes of confusion and memory loss that seem to fluctuate throughout the day. She also reports that he has had several episodes of talking and moving in his sleep as if acting out his dreams. Additionally, she has noticed that he has been having visual hallucinations, often describing children playing in the room when no one is there.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Dopamine agonist-induced hallucinations (diagnosed 2021-07-15)
- Parkinson Disease (diagnosed age 63)
- Parkinson Disease Dementia
- REM Sleep Behavior Disorder
- Hypertension
- Type 2 Diabetes Mellitus
- Dyslipidemia

The patient has a history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cholecystectomy (laparoscopic, age 54)
- Bilateral cataract surgery (age 66)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Carbidopa/Levodopa 25/100 mg PO TID
- Ropinirole 0.5 mg PO TID
- Melatonin 3 mg PO qHS

ALLERGIES:
Severe reaction to a low dose of haloperidol

FAMILY HISTORY:
- Father: Parkinson disease, diagnosed age 70
- Mother: Type 2 diabetes mellitus, hypertension
- Paternal uncle: Intracerebral hemorrhage (fatal)
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks on holidays). Occupation: Retired municipal accountant.

PHYSICAL EXAMINATION:
On examination, the patient demonstrates a shuffling gait, bradykinesia, and a mild resting tremor. His Mini-Mental State Examination score is 24/30.

## Encounter 4: source 2025-01-15, practice 2026-02-27, ed, Emergency Medicine

Chief complaint: Sudden onset of right-sided weakness and difficulty speaking that began two hours ago.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Michael Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of right-sided weakness and difficulty speaking that began two hours ago.

HISTORY OF PRESENT ILLNESS:
This 73-year-old male with Parkinson Disease diagnosed age 63 and Parkinson Disease Dementia, last seen two years ago for cognitive decline, presents to the emergency department with sudden onset of right-sided weakness and difficulty speaking that began two hours ago. His wife reports that he was in his usual state of health until this morning when he suddenly collapsed at home.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Dopamine agonist-induced hallucinations (diagnosed 2021-07-15)
- Parkinson disease dementia (diagnosed 2023-01-15)
- Parkinson Disease (diagnosed age 63)
- REM Sleep Behavior Disorder
- Hypertension
- Type 2 Diabetes Mellitus
- Dyslipidemia

PAST SURGICAL HISTORY:
- Cholecystectomy (laparoscopic, age 54)
- Bilateral cataract surgery (age 66)

MEDICATIONS:
Home Medications:
- Carbidopa/Levodopa 25/100 mg PO TID
- Ropinirole 0.5 mg PO TID
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily
- Melatonin 3 mg PO qHS

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson disease, diagnosed age 70
- Mother: Type 2 diabetes mellitus, hypertension
- Paternal uncle: Intracerebral hemorrhage (fatal)
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks on holidays). Occupation: Retired municipal accountant.

VITAL SIGNS:
His blood pressure is 220/120 mmHg, heart rate is 88 beats per minute, and respiratory rate is 22 breaths per minute with labored breathing.

PHYSICAL EXAMINATION:
On examination, the patient is drowsy but arousable, with a Glasgow Coma Scale score of 11 (E3V3M5). Neurological examination reveals left gaze preference, right hemiplegia, and global aphasia. The patient&#x27;s respiratory efforts become more labored, and he is unable to protect his airway with a decreased level of consciousness.

IMAGING:
A non-contrast head CT scan shows a left basal ganglia hemorrhage with surrounding edema and a slight midline shift.
