# synthetic_hospital patient 2716

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex F, Non-Hispanic White, Medicare (disability) and Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-02-07 (Tue) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2023-08-07 (Mon) | outpatient | Orthopedics |
| 2 | 2021-01-15 | 2024-01-29 (Mon) | outpatient | Gastroenterology |
| 3 | 2021-07-15 | 2024-08-06 (Tue) | outpatient | Rheumatology |
| 4 | 2022-01-15 | 2025-01-30 (Thu) | outpatient | Neurology |
| 5 | 2022-07-15 | 2025-08-07 (Thu) | outpatient | Internal Medicine |
| 6 | 2023-01-15 | 2026-01-28 (Wed) | ed | Emergency Medicine |
| 7 | 2023-07-15 | 2026-07-25 (Sat) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (disability) and Medicaid",
 "occupation": "Part-time administrative assistant (currently on medical leave)",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Mother: osteoporosis with hip fracture at age 78",
  "Father: hypertension, stroke at age 82",
  "Sister: schizophrenia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 15 years ago (12 pack-year history)",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Haloperidol"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Benztropine"
  },
  {
   "dose": "500 mg PO PRN for joint pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Schizophrenia (stable, 20-year history)",
  "Osteoporosis",
  "Severe osteoarthritis, right hip",
  "Dermatomyositis",
  "Esophageal dysmotility",
  "Benign paroxysmal positional vertigo"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Ruptured posterior communicating artery aneurysm with subarachnoid hemorrhage", "Severe hip osteoarthritis", "Benign paroxysmal positional vertigo", "Oculogyric crisis due to antipsychotic medication", "Osteoporosis", "Esophageal dysmotility", "Dermatomyositis"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2023-02-07, outpatient, Internal Medicine

Chief complaint: Routine health maintenance visit with concerns about osteoporosis risk

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance visit with concerns about osteoporosis risk

HISTORY OF PRESENT ILLNESS:
She is concerned about her risk of developing osteoporosis, as a recent DEXA scan revealed a T-score of -2.5 at the lumbar spine. She denies any history of fractures. She inquires about starting a medication to improve her bone density and reduce her risk of fractures. She also expresses concern about her family history of breast cancer and wishes to minimize her risk.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Schizophrenia (stable, 20-year history)
- Osteoporosis
- Severe osteoarthritis, right hip
- Dermatomyositis
- Esophageal dysmotility
- Benign paroxysmal positional vertigo

She has a past medical history significant for hypertension, which is well-controlled on hydrochlorothiazide.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She reports.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Haloperidol 5 mg PO daily
- Benztropine 1 mg PO daily
- Acetaminophen 500 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
A family history of breast cancer in her mother and sister

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (12 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Part-time administrative assistant (currently on medical leave).

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Her laboratory values are as follows:

- Serum calcium: 9.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum phosphate: 3.5 mg/dL (normal: 2.5-4.5 mg/dL)
- Alkaline phosphatase: 70 U/L (normal: 35-100 U/L)
- Serum creatinine: 0.9 mg/dL (normal: 0.6-1.1 mg/dL)
- Thyroid-stimulating hormone (TSH): 2.1 mIU/L (normal: 0.4-4.0 mIU/L)

## Encounter 1: source 2020-07-15, practice 2023-08-07, outpatient, Orthopedics

Chief complaint: Six-month history of progressive right hip pain worsened by activity

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive right hip pain worsened by activity

HISTORY OF PRESENT ILLNESS:
The patient returns six months after osteoporosis evaluation, presenting with progressive right hip pain. She reports that the pain is most severe when she climbs stairs or after prolonged walking, but it improves with rest. She denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis (diagnosed 2020-01-15)
- Hypertension
- Schizophrenia (stable, 20-year history)
- Severe osteoarthritis, right hip
- Dermatomyositis
- Esophageal dysmotility
- Benign paroxysmal positional vertigo

Her medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes she takes hydrochlorothiazide.

Home Medications:
- Haloperidol 5 mg PO daily
- Benztropine 1 mg PO daily
- Acetaminophen 500 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: osteoporosis with hip fracture at age 78
- Father: hypertension, stroke at age 82
- Sister: schizophrenia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (12 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Part-time administrative assistant (currently on medical leave).

VITAL SIGNS:
She has a body mass index of 32 kg/m^2

PHYSICAL EXAMINATION:
On physical examination, there is no erythema or warmth over the hip, but there is pain with passive internal rotation of the right hip. There is no tenderness over the greater trochanter.

IMAGING:
Radiographs of the right hip show joint space narrowing, subchondral sclerosis, and osteophyte formation.

## Encounter 2: source 2021-01-15, practice 2024-01-29, outpatient, Gastroenterology

Chief complaint: Six-month history of intermittent dysphagia and non-cardiac chest pain

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of intermittent dysphagia and non-cardiac chest pain

HISTORY OF PRESENT ILLNESS:
This 57-year-old female with dermatomyositis presents following evaluations for osteoporosis and hip osteoarthritis. She describes the chest pain as non-radiating, cramp-like, and occasionally associated with meals. She denies any relief with antacids.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis (diagnosed 2020-01-15)
- Severe hip osteoarthritis (diagnosed 2020-07-15)
- Hypertension
- Schizophrenia (stable, 20-year history)
- Severe osteoarthritis, right hip
- Dermatomyositis
- Esophageal dysmotility
- Benign paroxysmal positional vertigo

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Haloperidol 5 mg PO daily
- Benztropine 1 mg PO daily
- Acetaminophen 500 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is unremarkable

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (12 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Part-time administrative assistant (currently on medical leave).

PHYSICAL EXAMINATION:
Physical examination is normal.

IMAGING:
She undergoes an esophagram, which reveals intermittent segments of simultaneous contractions giving a 'corkscrew' appearance to the esophagus.

OTHER STUDIES:
An electrocardiogram shows no abnormalities.
Esophageal manometry confirms the presence of multiple simultaneous contractions in the middle and lower esophagus.

## Encounter 3: source 2021-07-15, practice 2024-08-06, outpatient, Rheumatology

Chief complaint: Three-month history of proximal muscle weakness and new facial/hand rash

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Johnson, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of proximal muscle weakness and new facial/hand rash

HISTORY OF PRESENT ILLNESS:
Amid management of osteoporosis, hip osteoarthritis, and recent esophageal dysmotility evaluation, she presents with three months of proximal muscle weakness. She also reports a rash that appeared over her knuckles and eyelids 2 weeks ago. She denies recent travel, sick contacts, or new medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis (diagnosed 2020-01-15)
- Severe hip osteoarthritis (diagnosed 2020-07-15)
- Esophageal dysmotility (diagnosed 2021-01-15)
- Hypertension
- Schizophrenia (stable, 20-year history)
- Severe osteoarthritis, right hip
- Dermatomyositis
- Benign paroxysmal positional vertigo

She has a past medical history of hypertension.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Haloperidol 5 mg PO daily
- Benztropine 1 mg PO daily
- Acetaminophen 500 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: osteoporosis with hip fracture at age 78
- Father: hypertension, stroke at age 82
- Sister: schizophrenia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (12 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Part-time administrative assistant (currently on medical leave).

PHYSICAL EXAMINATION:
On physical examination, she has an erythematous rash over the dorsum of her fingers and a violaceous rash on her upper eyelids. Muscle strength testing reveals symmetrical proximal muscle weakness.

LABORATORY DATA:
Laboratory findings are as follows:

- Creatine kinase (CK): 980 U/L (Normal: 22-198 U/L)
- Aldolase: 23.5 U/L (Normal: 1.0-7.5 U/L)
- Antinuclear antibody (ANA): Positive
- Anti-Jo-1 antibody: Negative
- Erythrocyte sedimentation rate (ESR): 40 mm/hr (Normal: 0-20 mm/hr)

## Encounter 4: source 2022-01-15, practice 2025-01-30, outpatient, Neurology

Chief complaint: Recurrent brief episodes of spinning vertigo triggered by position changes

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Wilson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent brief episodes of spinning vertigo triggered by position changes

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old female with dermatomyositis and osteoporosis who presents with positional vertigo. She reports that these episodes are triggered by movements such as looking up or turning over in bed. She denies any hearing loss, tinnitus, or fullness in the ears.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis (diagnosed 2020-01-15)
- Severe hip osteoarthritis (diagnosed 2020-07-15)
- Esophageal dysmotility (diagnosed 2021-01-15)
- Dermatomyositis (diagnosed 2021-07-15)
- Hypertension
- Schizophrenia (stable, 20-year history)
- Severe osteoarthritis, right hip
- Benign paroxysmal positional vertigo

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes she takes hydrochlorothiazide.

Home Medications:
- Haloperidol 5 mg PO daily
- Benztropine 1 mg PO daily
- Acetaminophen 500 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: osteoporosis with hip fracture at age 78
- Father: hypertension, stroke at age 82
- Sister: schizophrenia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (12 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Part-time administrative assistant (currently on medical leave).

VITAL SIGNS:
On physical examination, her blood pressure is 135/85 mmHg, and her heart rate is 78 beats per minute and regular.

PHYSICAL EXAMINATION:
Neurological examination is unremarkable except for a positive Dix-Hallpike maneuver, which reproduces her symptoms and is associated with nystagmus that resolves within 60 seconds. There is no evidence of carotid bruits, jugular venous distention, or cardiac murmurs.

## Encounter 5: source 2022-07-15, practice 2025-08-07, outpatient, Internal Medicine

Chief complaint: Enrollment visit for randomized controlled trial of new antihypertensive medication

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Enrollment visit for randomized controlled trial of new antihypertensive medication

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis (diagnosed 2020-01-15)
- Severe hip osteoarthritis (diagnosed 2020-07-15)
- Esophageal dysmotility (diagnosed 2021-01-15)
- Dermatomyositis (diagnosed 2021-07-15)
- Benign paroxysmal positional vertigo (diagnosed 2022-01-15)
- Hypertension
- Schizophrenia (stable, 20-year history)
- Severe osteoarthritis, right hip

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Haloperidol 5 mg PO daily
- Benztropine 1 mg PO daily
- Acetaminophen 500 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: osteoporosis with hip fracture at age 78
- Father: hypertension, stroke at age 82
- Sister: schizophrenia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (12 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Part-time administrative assistant (currently on medical leave).

OTHER STUDIES:
The study aims to detect a reduction in mean systolic blood pressure of at least 10 mmHg compared to placebo. Investigators pre-specify an alpha (α) level of 0.05 before the study begins. After enrolling 40 patients in each group, the study finds no statistically significant difference between the new drug and placebo, with a p-value of 0.13. The researchers note that the calculated power of the study is 65% to detect the pre-specified difference. They discuss the implications of their findings and consider increasing the sample size in a future trial.

## Encounter 6: source 2023-01-15, practice 2026-01-28, ed, Emergency Medicine

Chief complaint: Three-week history of involuntary head turning and oculogyric crisis

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three-week history of involuntary head turning and oculogyric crisis

HISTORY OF PRESENT ILLNESS:
This 59-year-old female with stable schizophrenia presents with involuntary head turning. She denies any pain but reports that the condition is causing her significant distress and interfering with her daily activities.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis (diagnosed 2020-01-15)
- Severe hip osteoarthritis (diagnosed 2020-07-15)
- Esophageal dysmotility (diagnosed 2021-01-15)
- Dermatomyositis (diagnosed 2021-07-15)
- Benign paroxysmal positional vertigo (diagnosed 2022-01-15)
- Hypertension
- Schizophrenia (stable, 20-year history)
- Severe osteoarthritis, right hip

She has a past medical history of schizophrenia, for which she has been on medication for the past 20 years. She also has hypertension, controlled with medication.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Her current medications include haloperidol, benztropine, and amlodipine.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Acetaminophen 500 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: osteoporosis with hip fracture at age 78
- Father: hypertension, stroke at age 82
- Sister: schizophrenia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (12 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Part-time administrative assistant (currently on medical leave).

PHYSICAL EXAMINATION:
On physical examination, you note sustained muscle contractions that are causing her head to tilt to one side with her chin pointing towards the opposite shoulder. There is no evidence of infection or trauma.

LABORATORY DATA:
Laboratory values are as follows:

- Complete blood count: within normal limits
- Comprehensive metabolic panel: within normal limits
- Thyroid-stimulating hormone: within normal limits

## Encounter 7: source 2023-07-15, practice 2026-07-25, icu, Emergency Medicine

Chief complaint: Sudden collapse at home preceded by thunderclap headache

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden collapse at home preceded by thunderclap headache

HISTORY OF PRESENT ILLNESS:
The patient with schizophrenia and prior oculogyric crisis presents acutely. Her partner reports that she complained of an excruciating headache shortly before losing consciousness. On arrival, she is somnolent but arousable.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis (diagnosed 2020-01-15)
- Severe hip osteoarthritis (diagnosed 2020-07-15)
- Esophageal dysmotility (diagnosed 2021-01-15)
- Dermatomyositis (diagnosed 2021-07-15)
- Benign paroxysmal positional vertigo (diagnosed 2022-01-15)
- Oculogyric crisis due to antipsychotic medication (diagnosed 2023-01-15)
- Hypertension
- Schizophrenia (stable, 20-year history)
- Severe osteoarthritis, right hip

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Haloperidol 5 mg PO daily
- Benztropine 1 mg PO daily
- Acetaminophen 500 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: osteoporosis with hip fracture at age 78
- Father: hypertension, stroke at age 82
- Sister: schizophrenia

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (12 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Part-time administrative assistant (currently on medical leave).

VITAL SIGNS:
BP is 160/95. HR is 55.

PHYSICAL EXAMINATION:
Neurologic examination shows a dilated and non-reactive right pupil with ptosis of the right eyelid.

IMAGING:
Head CT without contrast shows hyperdensity in the basal cisterns.
