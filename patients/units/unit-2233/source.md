# synthetic_hospital patient 2233

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental Plan F.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-07-17 (Mon) | outpatient | Neurology |
| 1 | 2020-04-15 | 2023-10-18 (Wed) | outpatient | Neurology |
| 2 | 2020-07-15 | 2024-01-08 (Mon) | outpatient | Neurology |
| 3 | 2021-01-15 | 2024-07-16 (Tue) | outpatient | Geriatrics |
| 4 | 2022-01-15 | 2025-07-07 (Mon) | outpatient | Cardiology |
| 5 | 2022-07-15 | 2026-01-15 (Thu) | ed | Emergency Medicine |
| 6 | 2022-10-15 | 2026-04-08 (Wed) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-07-11 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental Plan F",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Occasional social use, 1-2 glasses of wine per week",
 "family_history": [
  "Father: Coronary artery disease, myocardial infarction at age 76, deceased at 78",
  "Mother: Type 2 diabetes mellitus, dementia with Lewy bodies, deceased at 84",
  "Sister: Hypertension, age 70",
  "Son: Hypertension, age 45"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 20 years ago (20 pack-year history)",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily with meals",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO once daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO once daily at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "0.4 mg PO once daily at bedtime",
   "name": "Tamsulosin"
  }
 ],
 "surgical_history": [
  "Cataract extraction with intraocular lens placement, bilateral (2019)",
  "Transurethral resection of the prostate (TURP) (2018)",
  "Inguinal hernia repair, left (2010)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Hyperlipidemia",
  "Chronic Kidney Disease Stage 3",
  "Benign Prostatic Hyperplasia",
  "Parkinson Disease",
  "Normal Pressure Hydrocephalus"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Mobitz type I second-degree AV block", "Cardiac tamponade", "Normal pressure hydrocephalus", "Parkinson disease dementia", "Chronic subdural hematoma"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2023-07-17, outpatient, Neurology

Chief complaint: Progressive difficulty walking, memory problems, and urinary urgency over 6 months with recent falls

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty walking, memory problems, and urinary urgency over 6 months with recent falls

HISTORY OF PRESENT ILLNESS:
A 72-year-old male presents to the clinic with a 6-month history of progressive difficulty walking, memory problems, and urinary urgency. His wife reports that he shuffles when he walks and has fallen twice in the past month. He has become more forgetful, often repeating questions several times during conversations. The patient also complains of needing to rush to the bathroom and has had episodes of incontinence.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3
- Benign Prostatic Hyperplasia
- Parkinson Disease
- Normal Pressure Hydrocephalus

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, bilateral (2019)
- Transurethral resection of the prostate (TURP) (2018)
- Inguinal hernia repair, left (2010)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily with meals
- Lisinopril 20 mg PO once daily
- Atorvastatin 40 mg PO once daily at bedtime
- Tamsulosin 0.4 mg PO once daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 76, deceased at 78
- Mother: Type 2 diabetes mellitus, dementia with Lewy bodies, deceased at 84
- Sister: Hypertension, age 70
- Son: Hypertension, age 45

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (20 pack-year history). Alcohol: Occasional social use, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, the patient has a broad-based gait, difficulty with heel-to-toe walking, and magnetic steps. There is no muscle weakness, but he exhibits increased muscle tone in the lower extremities.

IMAGING:
A CT scan of the head shows enlarged ventricles with minimal cortical atrophy.

## Encounter 1: source 2020-04-15, practice 2023-10-18, outpatient, Neurology

Chief complaint: Progressive difficulty walking with magnetic gait and urinary incontinence

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Okonkwo, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty walking with magnetic gait and urinary incontinence

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of his gait disturbance initially evaluated on January 15, 2020, with known history of Parkinson disease and normal pressure hydrocephalus. He presents with a 6-month history of progressive difficulty walking, which he describes as feeling like his feet are "stuck to the floor." His wife has noticed that he has become more forgetful and occasionally fails to make it to the bathroom in time.

PAST MEDICAL HISTORY:
Active Problem List:
- Normal pressure hydrocephalus (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3
- Benign Prostatic Hyperplasia
- Parkinson Disease

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, bilateral (2019)
- Transurethral resection of the prostate (TURP) (2018)
- Inguinal hernia repair, left (2010)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily with meals
- Lisinopril 20 mg PO once daily
- Atorvastatin 40 mg PO once daily at bedtime
- Tamsulosin 0.4 mg PO once daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 76, deceased at 78
- Mother: Type 2 diabetes mellitus, dementia with Lewy bodies, deceased at 84
- Sister: Hypertension, age 70
- Son: Hypertension, age 45

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (20 pack-year history). Alcohol: Occasional social use, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, the patient has a broad-based, magnetic gait and difficulty initiating movement. There is no papilledema, and his cranial nerves are intact.

LABORATORY DATA:
A urinary analysis is unremarkable.

IMAGING:
A non-contrast head CT scan shows enlarged ventricles with no evidence of cortical atrophy.

## Encounter 2: source 2020-07-15, practice 2024-01-08, outpatient, Neurology

Chief complaint: Progressive memory problems and difficulty with navigation and finances

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Elena Vasquez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive memory problems and difficulty with navigation and finances

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with Parkinson disease and Normal Pressure Hydrocephalus who returns for follow-up following prior neurology encounters in January and April 2020 for gait difficulty and urinary symptoms. He presents to the clinic with his wife, who reports that her husband has been experiencing progressive memory problems over the past year. She is particularly concerned about his difficulty navigating to places he used to know well and managing his finances, tasks he previously performed with ease. Additionally, she mentions that he has had episodes of talking and moving as if acting out his dreams. The patient also describes seeing small animals in the house that his wife cannot see.

PAST MEDICAL HISTORY:
Active Problem List:
- Normal pressure hydrocephalus (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3
- Benign Prostatic Hyperplasia
- Parkinson Disease

His past medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, bilateral (2019)
- Transurethral resection of the prostate (TURP) (2018)
- Inguinal hernia repair, left (2010)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Tamsulosin 0.4 mg PO once daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 76, deceased at 78
- Mother: Type 2 diabetes mellitus, dementia with Lewy bodies, deceased at 84
- Sister: Hypertension, age 70
- Son: Hypertension, age 45

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (20 pack-year history). Alcohol: Occasional social use, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Blood pressure: 142/88 mmHg

PHYSICAL EXAMINATION:
On examination, you note a resting tremor and bradykinesia. His Mini-Mental State Examination score is 24/30, with deficits noted in attention and visuospatial skills.

LABORATORY DATA:
Fasting glucose: 138 mg/dL
HbA1c: 7.2%
Serum creatinine: 1.1 mg/dL
Total cholesterol: 185 mg/dL

## Encounter 3: source 2021-01-15, practice 2024-07-16, outpatient, Geriatrics

Chief complaint: Episodes of confusion and memory loss with fluctuating cognition

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Patterson, MD
Department: Geriatrics
Visit Type: outpatient

CHIEF COMPLAINT:
Episodes of confusion and memory loss with fluctuating cognition

HISTORY OF PRESENT ILLNESS:
The patient is a 73-year-old male with Parkinson disease and Normal Pressure Hydrocephalus returning for follow-up of cognitive concerns. He was previously managed in Neurology for gait difficulties and memory impairment, with progression to Parkinson disease dementia documented six months prior. He presents with his daughter who reports that her father has been experiencing episodes of confusion and memory loss over the past year. She notes these episodes fluctuate throughout the day, with periods of lucidity. Additionally, she mentions he has had visual hallucinations, often describing small animals in the house that are not there. The patient also complains of stiffness and tremors in his hands, which have progressively worsened over the past six months.

PAST MEDICAL HISTORY:
Active Problem List:
- Normal pressure hydrocephalus (diagnosed 2020-01-15)
- Parkinson disease dementia (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3
- Benign Prostatic Hyperplasia
- Parkinson Disease

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, bilateral (2019)
- Transurethral resection of the prostate (TURP) (2018)
- Inguinal hernia repair, left (2010)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Tamsulosin 0.4 mg PO once daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 76, deceased at 78
- Mother: Type 2 diabetes mellitus, dementia with Lewy bodies, deceased at 84
- Sister: Hypertension, age 70
- Son: Hypertension, age 45

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (20 pack-year history). Alcohol: Occasional social use, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On examination, the patient has a shuffling gait, bradykinesia, and a resting tremor. His Mini-Mental State Examination score is 24/30, with deficits noted in attention and visuospatial skills. There is no evidence of stroke or focal neurological deficits.

LABORATORY DATA:
Laboratory values:
- CBC: within normal limits
- Electrolytes: within normal limits
- Fasting glucose: 148 mg/dL
- HbA1c: 7.2%
- TSH: within normal limits
- Vitamin B12: within normal limits

IMAGING:
Brain MRI shows mild generalized atrophy without acute infarct or significant white matter disease.

## Encounter 4: source 2022-01-15, practice 2025-07-07, outpatient, Cardiology

Chief complaint: Occasional lightheadedness for the past two weeks

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Washington, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Occasional lightheadedness for the past two weeks

HISTORY OF PRESENT ILLNESS:
The patient with Parkinson disease, Normal Pressure Hydrocephalus, and dementia presents with lightheadedness. He denies any chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Normal pressure hydrocephalus (diagnosed 2020-01-15)
- Parkinson disease dementia (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3
- Benign Prostatic Hyperplasia
- Parkinson Disease

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, bilateral (2019)
- Transurethral resection of the prostate (TURP) (2018)
- Inguinal hernia repair, left (2010)

MEDICATIONS:
His medications include metformin, lisinopril, and atenolol.

Home Medications:
- Atorvastatin 40 mg PO once daily at bedtime
- Tamsulosin 0.4 mg PO once daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 76, deceased at 78
- Mother: Type 2 diabetes mellitus, dementia with Lewy bodies, deceased at 84
- Sister: Hypertension, age 70
- Son: Hypertension, age 45

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (20 pack-year history). Alcohol: Occasional social use, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On physical examination, his blood pressure is 130/80 mmHg, heart rate is 58 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
His cardiovascular examination is notable for irregularly irregular pulse.

LABORATORY DATA:
Serum electrolytes are within normal limits.

OTHER STUDIES:
An electrocardiogram (ECG) is performed and shows a pattern of progressive prolongation of the PR interval followed by a nonconducted P wave, after which the cycle repeats.

ASSESSMENT AND PLAN:
Based on the patient's presentation and ECG findings,

## Encounter 5: source 2022-07-15, practice 2026-01-15, ed, Emergency Medicine

Chief complaint: Progressive headache, intermittent confusion, and difficulty walking after fall three weeks ago

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive headache, intermittent confusion, and difficulty walking after fall three weeks ago

HISTORY OF PRESENT ILLNESS:
The patient is a 74-year-old male with longstanding Normal Pressure Hydrocephalus and Parkinson disease dementia who has had previous falls and cognitive fluctuations, and who presents to the emergency department with a 2-week history of progressive headache, intermittent confusion, and difficulty walking. His daughter reports that he has been increasingly somnolent over the past few days. The patient admits to a fall without loss of consciousness approximately 3 weeks ago but did not seek medical attention at the time.

PAST MEDICAL HISTORY:
Active Problem List:
- Normal pressure hydrocephalus (diagnosed 2020-01-15)
- Parkinson disease dementia (diagnosed 2020-07-15)
- Mobitz type I second-degree AV block (diagnosed 2022-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3
- Benign Prostatic Hyperplasia
- Parkinson Disease

His past medical history includes hypertension, type 2 diabetes mellitus, and benign prostatic hyperplasia.

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, bilateral (2019)
- Transurethral resection of the prostate (TURP) (2018)
- Inguinal hernia repair, left (2010)

MEDICATIONS:
He takes hydrochlorothiazide, metformin, and tamsulosin.

Home Medications:
- Lisinopril 20 mg PO once daily
- Atorvastatin 40 mg PO once daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 76, deceased at 78
- Mother: Type 2 diabetes mellitus, dementia with Lewy bodies, deceased at 84
- Sister: Hypertension, age 70
- Son: Hypertension, age 45

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (20 pack-year history). Alcohol: Occasional social use, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His blood pressure is 145/85 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
On examination, he is alert but disoriented to time.
Neurological examination reveals mild right-sided weakness and a sluggish pupillary response on the right.

IMAGING:
A non-contrast head CT scan shows a crescent-shaped hyperdensity along the left convexity, with midline shift.

## Encounter 6: source 2022-10-15, practice 2026-04-08, ed, Emergency Medicine

Chief complaint: Intermittent dizziness and palpitations

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. Amara Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Intermittent dizziness and palpitations

HISTORY OF PRESENT ILLNESS:
The patient with prior Mobitz type I second-degree AV block returns to the emergency department with complaints of intermittent dizziness and palpitations for the past few weeks. He denies chest pain, dyspnea, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Normal pressure hydrocephalus (diagnosed 2020-01-15)
- Parkinson disease dementia (diagnosed 2020-07-15)
- Mobitz type I second-degree AV block (diagnosed 2022-01-15)
- Chronic subdural hematoma (diagnosed 2022-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3
- Benign Prostatic Hyperplasia
- Parkinson Disease

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, bilateral (2019)
- Transurethral resection of the prostate (TURP) (2018)
- Inguinal hernia repair, left (2010)

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Metformin 1000 mg PO twice daily with meals
- Tamsulosin 0.4 mg PO once daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 76, deceased at 78
- Mother: Type 2 diabetes mellitus, dementia with Lewy bodies, deceased at 84
- Sister: Hypertension, age 70
- Son: Hypertension, age 45

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (20 pack-year history). Alcohol: Occasional social use, 1-2 glasses of wine per week. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On physical examination, his blood pressure is 130/80 mmHg, heart rate is 56 bpm, and respiratory rate is 16 breaths per minute

PHYSICAL EXAMINATION:
His cardiovascular examination reveals an irregularly irregular pulse.

OTHER STUDIES:
An electrocardiogram (ECG) is performed and demonstrates a repeating pattern where every third atrial beat is not followed by a ventricular beat, suggesting a 2:1 atrioventricular (AV) block. The PR intervals of conducted beats are not consistently lengthening prior to the dropped beats. Vagal maneuvers are performed, and the ECG now shows some of the previously non-conducted P waves followed by a QRS complex with a prolonged PR interval.

## Encounter 7: source 2023-01-15, practice 2026-07-11, ed, Emergency Medicine

Chief complaint: Found unresponsive at home with preceding chest pain

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Singh, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found unresponsive at home with preceding chest pain

HISTORY OF PRESENT ILLNESS:
The patient is a 75-year-old male with Parkinson disease and prior AV block. He is unable to provide history due to his condition. His neighbor reports the patient complained of intermittent chest pain for the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Normal pressure hydrocephalus (diagnosed 2020-01-15)
- Parkinson disease dementia (diagnosed 2020-07-15)
- Mobitz type I second-degree AV block (diagnosed 2022-01-15)
- Chronic subdural hematoma (diagnosed 2022-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic Kidney Disease Stage 3
- Benign Prostatic Hyperplasia
- Parkinson Disease

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, bilateral (2019)
- Transurethral resection of the prostate (TURP) (2018)
- Inguinal hernia repair, left (2010)

MEDICATIONS:
The patient's medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Tamsulosin 0.4 mg PO once daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease, myocardial infarction at age 76, deceased at 78
- Mother: Type 2 diabetes mellitus, dementia with Lewy bodies, deceased at 84
- Sister: Hypertension, age 70
- Son: Hypertension, age 45

SOCIAL HISTORY:
The patient's son arrives shortly after and is unaware of his father's wishes regarding invasive procedures.

VITAL SIGNS:
On examination, the patient is afebrile, blood pressure is 90/60 mmHg, heart rate is 52 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals jugular venous distension, distant heart sounds, and cold extremities.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 13.2 g/dL
- WBC count: 7,800/mm^3
- Platelets: 210,000/mm^3
- Serum creatinine: 2.1 mg/dL (baseline 1.9 mg/dL)
- Blood urea nitrogen: 44 mg/dL
- Serum potassium: 4.5 mEq/L
- Troponin I: Elevated

IMAGING:
Echocardiogram confirms a large pericardial effusion with evidence of cardiac tamponade.

OTHER STUDIES:
An ECG shows low voltage QRS complexes and electrical alternans.
