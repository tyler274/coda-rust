# synthetic_hospital patient 2232

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare Part B.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2018-11-23 (Fri) | outpatient | Pain Management |
| 1 | 2020-07-15 | 2019-06-04 (Tue) | outpatient | Neurology |
| 2 | 2021-07-15 | 2020-06-05 (Fri) | outpatient | Hematology-Oncology |
| 3 | 2022-07-15 | 2021-06-04 (Fri) | outpatient | Hematology-Oncology |
| 4 | 2024-01-15 | 2022-11-21 (Mon) | outpatient | Neurology |
| 5 | 2025-03-15 | 2024-01-19 (Fri) | outpatient | Neurology |
| 6 | 2026-05-15 | 2025-03-29 (Sat) | ed | Emergency Medicine |
| 7 | 2027-05-15 | 2026-03-20 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfa drugs (hives)",
  "Iodinated contrast (anaphylaxis)"
 ],
 "insurance": "Medicare Part B",
 "occupation": "Retired postal clerk",
 "alcohol_use": "Occasional, 1-2 beers on weekends",
 "family_history": [
  "Father: Ischemic stroke at age 74",
  "Mother: Type 2 Diabetes Mellitus, Hypertension",
  "Maternal uncle: Waldenstr\u00f6m macroglobulinemia diagnosed age 72",
  "Sister: Osteoarthritis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 8 years ago, 25 pack-year history",
 "home_medications": [
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "0.4 mg daily at bedtime",
   "name": "Tamsulosin"
  },
  {
   "dose": "60 mg twice daily",
   "name": "Morphine sulfate extended-release"
  },
  {
   "dose": "15 mg every 4 hours as needed for breakthrough pain",
   "name": "Morphine sulfate immediate-release"
  },
  {
   "dose": "40 mg daily at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Left cataract extraction with intraocular lens placement (2021)",
  "Diagnostic cystoscopy for hematuria (2019)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Benign Prostatic Hyperplasia",
  "Lumbar Spinal Stenosis",
  "Cervical Spondylosis",
  "History of Transient Ischemic Attack (2 years prior)",
  "Hyperlipidemia",
  "Chronic Kidney Disease Stage 3a"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Waldenstr\u00f6m macroglobulinemia", "Spinal stenosis", "AL amyloidosis", "Normal pressure hydrocephalus", "Vascular dementia", "Cervical spondylotic myelopathy", "Lacunar infarction", "Chorea"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2018-11-23, outpatient, Pain Management

Chief complaint: Routine follow-up for chronic lower back pain with poor control on current opioid regimen.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pain Management
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic lower back pain with poor control on current opioid regimen.

HISTORY OF PRESENT ILLNESS:
He reports that his pain has been poorly controlled with his current regimen of extended-release morphine 60 mg twice daily. He denies any recent trauma or new neurological symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Benign Prostatic Hyperplasia
- Lumbar Spinal Stenosis
- Cervical Spondylosis
- History of Transient Ischemic Attack (2 years prior)
- Hyperlipidemia
- Chronic Kidney Disease Stage 3a

His medical history is significant for hypertension, type 2 diabetes mellitus, and benign prostatic hyperplasia.

PAST SURGICAL HISTORY:
- Left cataract extraction with intraocular lens placement (2021)
- Diagnostic cystoscopy for hematuria (2019)

MEDICATIONS:
He takes metformin, lisinopril, and tamsulosin.

Home Medications:
- Hydrochlorothiazide 12.5 mg daily
- Morphine sulfate extended-release 60 mg twice daily
- Morphine sulfate immediate-release 15 mg every 4 hours as needed for breakthrough pain
- Atorvastatin 40 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
Sulfa drugs (hives), Iodinated contrast (anaphylaxis)

FAMILY HISTORY:
- Father: Ischemic stroke at age 74
- Mother: Type 2 Diabetes Mellitus, Hypertension
- Maternal uncle: Waldenström macroglobulinemia diagnosed age 72
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Retired postal clerk.

VITAL SIGNS:
Blood pressure is 142/88 mmHg, heart rate is 78 bpm, and respiratory rate is 16 breaths per minute

PHYSICAL EXAMINATION:
His back examination reveals chronic lumbar tenderness without overlying erythema or warmth. Neurological examination is unremarkable.

LABORATORY DATA:
- Hemoglobin A1c: 7.2%
- Serum creatinine: 1.1 mg/dL
- Estimated GFR: 58 mL/min/1.73m^2

PLAN:
The provider decides to optimize his pain control and considers converting his opioid medication to a different agent using a CDSS to ensure accurate dosing.

## Encounter 1: source 2020-07-15, practice 2019-06-04, outpatient, Neurology

Chief complaint: Progressive neck pain over 6 months with difficulty walking and electric shock sensations down the back upon neck flexion.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive neck pain over 6 months with difficulty walking and electric shock sensations down the back upon neck flexion.

HISTORY OF PRESENT ILLNESS:
Patient returns six months following lumbar stenosis evaluation, reporting a sensation of electric shocks running down his back when he flexes his neck.

PAST MEDICAL HISTORY:
Active Problem List:
- Spinal stenosis (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Benign Prostatic Hyperplasia
- Lumbar Spinal Stenosis
- Cervical Spondylosis
- History of Transient Ischemic Attack (2 years prior)
- Hyperlipidemia
- Chronic Kidney Disease Stage 3a

His medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Left cataract extraction with intraocular lens placement (2021)
- Diagnostic cystoscopy for hematuria (2019)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Morphine sulfate extended-release 60 mg twice daily
- Morphine sulfate immediate-release 15 mg every 4 hours as needed for breakthrough pain
- Atorvastatin 40 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
Sulfa drugs (hives), Iodinated contrast (anaphylaxis)

FAMILY HISTORY:
- Father: Ischemic stroke at age 74
- Mother: Type 2 Diabetes Mellitus, Hypertension
- Maternal uncle: Waldenström macroglobulinemia diagnosed age 72
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Retired postal clerk.

PHYSICAL EXAMINATION:
On physical examination, you note atrophy of the intrinsic hand muscles and decreased grip strength. Reflex testing reveals diminished biceps and triceps reflexes in the upper extremities, but 3+ patellar reflexes and sustained ankle clonus in the lower extremities. Babinski sign is positive on the right.

LABORATORY DATA:
Laboratory values:
- Hemoglobin A1c: 7.2%
- Fasting blood glucose: 156 mg/dL
- Total cholesterol: 210 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 130 mg/dL
- Triglycerides: 200 mg/dL

IMAGING:
MRI of the cervical spine shows degenerative changes with narrowing of the spinal canal at multiple levels.

## Encounter 2: source 2021-07-15, practice 2020-06-05, outpatient, Hematology-Oncology

Chief complaint: Two-month history of blurred vision, headaches, episodic dizziness, and peripheral tingling sensations.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Emily Watson, MD
Department: Hematology-Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of blurred vision, headaches, episodic dizziness, and peripheral tingling sensations.

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with cervical myelopathy and lumbar spinal stenosis, seen in Neurology and Pain Management, presents with a 2-month history of blurred vision, headaches, and episodic dizziness. He also reports a tingling sensation in the hands and feet.

PAST MEDICAL HISTORY:
Active Problem List:
- Spinal stenosis (diagnosed 2020-01-15)
- Cervical spondylotic myelopathy (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Benign Prostatic Hyperplasia
- Lumbar Spinal Stenosis
- Cervical Spondylosis
- History of Transient Ischemic Attack (2 years prior)
- Hyperlipidemia
- Chronic Kidney Disease Stage 3a

His past medical history is significant for type 2 diabetes mellitus and hypertension, which are well-controlled with metformin and lisinopril, respectively.

PAST SURGICAL HISTORY:
- Left cataract extraction with intraocular lens placement (2021)
- Diagnostic cystoscopy for hematuria (2019)

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Hydrochlorothiazide 12.5 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Morphine sulfate extended-release 60 mg twice daily
- Morphine sulfate immediate-release 15 mg every 4 hours as needed for breakthrough pain
- Atorvastatin 40 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
Sulfa drugs (hives), Iodinated contrast (anaphylaxis)

FAMILY HISTORY:
- Father: Ischemic stroke at age 74
- Mother: Type 2 Diabetes Mellitus, Hypertension
- Maternal uncle: Waldenström macroglobulinemia diagnosed age 72
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Retired postal clerk.

PHYSICAL EXAMINATION:
On physical examination, you note conjunctival pallor and engorged retinal veins. Neurological examination reveals decreased sensation to light touch in a stocking-glove distribution.

LABORATORY DATA:
Laboratory studies are ordered, and the results are as follows:

- Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 89 fL (Normal: 80-100 fL)
- Serum creatinine: 1.4 mg/dL (Normal: 0.6-1.3 mg/dL)
- Serum calcium: 9.2 mg/dL (Normal: 8.5-10.2 mg/dL)
- Total protein: 9.8 g/dL (Normal: 6.4-8.3 g/dL)
- Albumin: 3.2 g/dL (Normal: 3.5-5.0 g/dL)
- Serum protein electrophoresis: Monoclonal spike in the gamma region
- Serum viscosity: Elevated

PATHOLOGY:
A bone marrow biopsy is performed, and the results show infiltration with small lymphocytes that have plasmacytoid features.

ASSESSMENT AND PLAN:
Based on the clinical presentation and laboratory findings,

## Encounter 3: source 2022-07-15, practice 2021-06-04, outpatient, Hematology-Oncology

Chief complaint: Three-month history of progressive fatigue, bilateral leg swelling, unintentional weight loss, orthopnea, and frothy urine.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Emily Watson, MD
Department: Hematology-Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of progressive fatigue, bilateral leg swelling, unintentional weight loss, orthopnea, and frothy urine.

HISTORY OF PRESENT ILLNESS:
The patient returns to Hematology-Oncology for ongoing management. He also reports difficulty breathing when lying flat and has noticed that his urine appears frothy.

PAST MEDICAL HISTORY:
Active Problem List:
- Spinal stenosis (diagnosed 2020-01-15)
- Cervical spondylotic myelopathy (diagnosed 2020-07-15)
- Waldenström macroglobulinemia (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Benign Prostatic Hyperplasia
- Lumbar Spinal Stenosis
- Cervical Spondylosis
- History of Transient Ischemic Attack (2 years prior)
- Hyperlipidemia
- Chronic Kidney Disease Stage 3a

His medical history is significant for hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus managed with metformin.

PAST SURGICAL HISTORY:
- Left cataract extraction with intraocular lens placement (2021)
- Diagnostic cystoscopy for hematuria (2019)

MEDICATIONS:
Hydrochlorothiazide and metformin

Home Medications:
- Lisinopril 20 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Morphine sulfate extended-release 60 mg twice daily
- Morphine sulfate immediate-release 15 mg every 4 hours as needed for breakthrough pain
- Atorvastatin 40 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
Sulfa drugs (hives), Iodinated contrast (anaphylaxis)

FAMILY HISTORY:
- Father: Ischemic stroke at age 74
- Mother: Type 2 Diabetes Mellitus, Hypertension
- Maternal uncle: Waldenström macroglobulinemia diagnosed age 72
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Retired postal clerk.

VITAL SIGNS:
On physical examination, his blood pressure is 150/90 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a low-pitched, rumbling diastolic murmur. There is marked periorbital purpura and bilateral pitting edema up to the knees.

LABORATORY DATA:
Laboratory findings are as follows:

- Serum albumin: 2.8 g/dL (Normal: 3.5-5.0 g/dL)
- Serum creatinine: 1.9 mg/dL (Normal: 0.6-1.2 mg/dL)
- Urine protein: 4.5 g/24 hours (Normal: <150 mg/24 hours)
- Total cholesterol: 260 mg/dL (Normal: <200 mg/dL)
- Liver function tests: Mildly elevated

IMAGING:
An echocardiogram shows increased ventricular wall thickness with normal chamber size, and Doppler imaging suggests restrictive filling patterns. Abdominal ultrasonography reveals hepatomegaly with no evidence of obstruction.

## Encounter 4: source 2024-01-15, practice 2022-11-21, outpatient, Neurology

Chief complaint: Six-month history of progressive memory loss, gait instability, and urinary incontinence reported by daughter.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James O'Brien, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive memory loss, gait instability, and urinary incontinence reported by daughter.

HISTORY OF PRESENT ILLNESS:
This patient with history of cervical spondylotic myelopathy, Waldenström macroglobulinemia, and AL amyloidosis presents with six months of progressive memory loss, gait instability, and urinary incontinence. His daughter notes that the patient, who was previously independent, now requires assistance with daily activities.

PAST MEDICAL HISTORY:
Active Problem List:
- Spinal stenosis (diagnosed 2020-01-15)
- Cervical spondylotic myelopathy (diagnosed 2020-07-15)
- Waldenström macroglobulinemia (diagnosed 2021-07-15)
- AL amyloidosis (diagnosed 2022-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Benign Prostatic Hyperplasia
- Lumbar Spinal Stenosis
- Cervical Spondylosis
- History of Transient Ischemic Attack (2 years prior)
- Hyperlipidemia
- Chronic Kidney Disease Stage 3a

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Left cataract extraction with intraocular lens placement (2021)
- Diagnostic cystoscopy for hematuria (2019)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Morphine sulfate extended-release 60 mg twice daily
- Morphine sulfate immediate-release 15 mg every 4 hours as needed for breakthrough pain
- Atorvastatin 40 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
Sulfa drugs (hives), Iodinated contrast (anaphylaxis)

FAMILY HISTORY:
- Father: Ischemic stroke at age 74
- Mother: Type 2 Diabetes Mellitus, Hypertension
- Maternal uncle: Waldenström macroglobulinemia diagnosed age 72
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Retired postal clerk.

PHYSICAL EXAMINATION:
On examination, the patient has a broad-based, magnetic gait and difficulty with heel-to-toe walking. There is no muscle weakness, and sensation is intact. A mini-mental state examination reveals mild cognitive impairment.

IMAGING:
A non-contrast head CT scan shows enlarged ventricles with normal-sized sulci.

## Encounter 5: source 2025-03-15, practice 2024-01-19, outpatient, Neurology

Chief complaint: Progressive memory loss and executive dysfunction over the past year, with difficulty planning daily activities.

ENCOUNTER NOTE
Date: 2025-03-15
Provider: Dr. James O'Brien, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive memory loss and executive dysfunction over the past year, with difficulty planning daily activities.

HISTORY OF PRESENT ILLNESS:
The patient, a 73-year-old male with a history of transient ischemic attack, chronic kidney disease, and prior neurology evaluation for cervical spondylotic myelopathy and normal pressure hydrocephalus, returns to the clinic with his daughter. She reports that her father has been experiencing progressive memory loss over the past year. She also notes that he has had difficulty planning and organizing daily activities, which is uncharacteristic of his usual behavior. The patient admits to feeling more confused lately and has gotten lost while driving on several occasions.

PAST MEDICAL HISTORY:
Active Problem List:
- Spinal stenosis (diagnosed 2020-01-15)
- Cervical spondylotic myelopathy (diagnosed 2020-07-15)
- Waldenström macroglobulinemia (diagnosed 2021-07-15)
- AL amyloidosis (diagnosed 2022-07-15)
- Normal pressure hydrocephalus (diagnosed 2024-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Benign Prostatic Hyperplasia
- Lumbar Spinal Stenosis
- Cervical Spondylosis
- History of Transient Ischemic Attack (2 years prior)
- Hyperlipidemia
- Chronic Kidney Disease Stage 3a

PAST SURGICAL HISTORY:
- Left cataract extraction with intraocular lens placement (2021)
- Diagnostic cystoscopy for hematuria (2019)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Morphine sulfate extended-release 60 mg twice daily
- Morphine sulfate immediate-release 15 mg every 4 hours as needed for breakthrough pain
- Atorvastatin 40 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
Sulfa drugs (hives), Iodinated contrast (anaphylaxis)

FAMILY HISTORY:
- Father: Ischemic stroke at age 74
- Mother: Type 2 Diabetes Mellitus, Hypertension
- Maternal uncle: Waldenström macroglobulinemia diagnosed age 72
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Retired postal clerk.

VITAL SIGNS:
The patient's blood pressure is 145/90 mmHg

PHYSICAL EXAMINATION:
On physical examination, the patient has a right-sided pronator drift and a brisk deep tendon reflex on the right. His gait is slow with reduced arm swing on the right.

LABORATORY DATA:
His fasting blood glucose is 162 mg/dL

IMAGING:
A non-contrast head CT scan shows multiple small areas of hypodensity in the subcortical white matter and basal ganglia bilaterally.

## Encounter 6: source 2026-05-15, practice 2025-03-29, ed, Emergency Medicine

Chief complaint: Sudden onset of large, flinging involuntary movements of the left arm and leg beginning earlier today, preventing ambulation and self-care.

ENCOUNTER NOTE
Date: 2026-05-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of large, flinging involuntary movements of the left arm and leg beginning earlier today, preventing ambulation and self-care.

HISTORY OF PRESENT ILLNESS:
The patient is a 74-year-old male with cervical spondylotic myelopathy, Waldenström macroglobulinemia, and vascular dementia who presents with new neurologic symptoms. The symptoms began earlier in the day and have progressively worsened. His wife describes them as large, unpredictable flinging movements of the left arm and leg, making it difficult for him to walk or feed himself. He has no history of similar episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Spinal stenosis (diagnosed 2020-01-15)
- Cervical spondylotic myelopathy (diagnosed 2020-07-15)
- Waldenström macroglobulinemia (diagnosed 2021-07-15)
- AL amyloidosis (diagnosed 2022-07-15)
- Normal pressure hydrocephalus (diagnosed 2024-01-15)
- Vascular dementia (diagnosed 2025-03-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Benign Prostatic Hyperplasia
- Lumbar Spinal Stenosis
- Cervical Spondylosis
- History of Transient Ischemic Attack (2 years prior)
- Hyperlipidemia
- Chronic Kidney Disease Stage 3a

His medical history includes hypertension and type 2 diabetes mellitus, both poorly controlled.

PAST SURGICAL HISTORY:
- Left cataract extraction with intraocular lens placement (2021)
- Diagnostic cystoscopy for hematuria (2019)

MEDICATIONS:
He takes lisinopril and metformin but reports nonadherence over the past several months.

Home Medications:
- Hydrochlorothiazide 12.5 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Morphine sulfate extended-release 60 mg twice daily
- Morphine sulfate immediate-release 15 mg every 4 hours as needed for breakthrough pain
- Atorvastatin 40 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
Sulfa drugs (hives), Iodinated contrast (anaphylaxis)

FAMILY HISTORY:
- Father: Ischemic stroke at age 74
- Mother: Type 2 Diabetes Mellitus, Hypertension
- Maternal uncle: Waldenström macroglobulinemia diagnosed age 72
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Retired postal clerk.

PHYSICAL EXAMINATION:
On examination, the patient is alert and oriented. He exhibits continuous, irregular, high-amplitude movements of the left upper and lower extremities. Muscle tone is difficult to assess due to the involuntary motion, but no spasticity or rigidity is appreciated. Reflexes are symmetric and there is no pronator drift or sensory deficit. Cranial nerves are intact.

IMAGING:
A non-contrast head CT reveals a small area of hyperdensity in the deep structures of the right cerebral hemisphere, consistent with an acute lacunar infarct.

## Encounter 7: source 2027-05-15, practice 2026-03-20, ed, Emergency Medicine

Chief complaint: Sudden onset of right-sided weakness.

ENCOUNTER NOTE
Date: 2027-05-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of right-sided weakness.

PAST MEDICAL HISTORY:
Active Problem List:
- Spinal stenosis (diagnosed 2020-01-15)
- Cervical spondylotic myelopathy (diagnosed 2020-07-15)
- Waldenström macroglobulinemia (diagnosed 2021-07-15)
- AL amyloidosis (diagnosed 2022-07-15)
- Normal pressure hydrocephalus (diagnosed 2024-01-15)
- Vascular dementia (diagnosed 2025-03-15)
- Chorea (diagnosed 2026-05-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Benign Prostatic Hyperplasia
- Lumbar Spinal Stenosis
- Cervical Spondylosis
- History of Transient Ischemic Attack (2 years prior)
- Hyperlipidemia
- Chronic Kidney Disease Stage 3a

PAST SURGICAL HISTORY:
- Left cataract extraction with intraocular lens placement (2021)
- Diagnostic cystoscopy for hematuria (2019)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Morphine sulfate extended-release 60 mg twice daily
- Morphine sulfate immediate-release 15 mg every 4 hours as needed for breakthrough pain
- Atorvastatin 40 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
Sulfa drugs (hives), Iodinated contrast (anaphylaxis)

FAMILY HISTORY:
- Father: Ischemic stroke at age 74
- Mother: Type 2 Diabetes Mellitus, Hypertension
- Maternal uncle: Waldenström macroglobulinemia diagnosed age 72
- Sister: Osteoarthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Retired postal clerk.

PHYSICAL EXAMINATION:
On examination, he has weakness of the right face, arm, and leg. Strength is equally reduced in both proximal and distal muscles. Sensation, coordination, language, and cranial nerve examination are normal.

IMAGING:
MRI reveals a small lacunar infarct.
