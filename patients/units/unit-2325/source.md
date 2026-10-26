# synthetic_hospital patient 2325

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 70 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2018-03-19 (Mon) | outpatient | Otolaryngology |
| 1 | 2021-07-15 | 2019-09-17 (Tue) | ed | Emergency Medicine |
| 2 | 2024-01-15 | 2022-03-17 (Thu) | ed | Emergency Medicine |
| 3 | 2026-01-15 | 2024-03-07 (Thu) | ed | Emergency Medicine |
| 4 | 2027-01-15 | 2025-03-13 (Thu) | inpatient | Neurology |
| 5 | 2028-01-15 | 2026-03-18 (Wed) | icu | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Social (1-2 glasses of wine per week)",
 "family_history": [
  "Father: Ischemic stroke at age 74",
  "Mother: Hypertension and Type 2 diabetes mellitus, deceased at age 81",
  "Older brother: Myocardial infarction at age 68"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 12 years ago (25 pack-year history)",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "40 mg PO at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Cataract extraction with intraocular lens placement, right eye (2019)",
  "Cataract extraction with intraocular lens placement, left eye (2020)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Hyperlipidemia",
  "Presbycusis",
  "Degenerative conduction system disease"
 ],
 "age_at_first_encounter": 70
}
```

Primary diagnoses: ["Weber syndrome (Midbrain infarction)", "Mobitz type I atrioventricular block (Wenckebach phenomenon)", "Presbycusis", "Lacunar stroke"]
Comorbidities: ["Advanced age"]

## Encounter 0: source 2020-01-15, practice 2018-03-19, outpatient, Otolaryngology

Chief complaint: Difficulty hearing, especially in noisy environments, gradually worsening over the past few years.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty hearing, especially in noisy environments, gradually worsening over the past few years.

HISTORY OF PRESENT ILLNESS:
He reports that this issue has been gradually worsening over the past few years. He has no history of ear infections, ototoxic medication use, or noise exposure in his occupation.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Presbycusis
- Degenerative conduction system disease

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, right eye (2019)
- Cataract extraction with intraocular lens placement, left eye (2020)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO at bedtime
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is non-contributory.

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (25 pack-year history). Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, the patient's tympanic membranes are intact and mobile, and there is no cerumen impaction. Weber test lateralizes to the right, and Rinne test shows air conduction is better than bone conduction bilaterally.

OTHER STUDIES:
Audiometry reveals bilateral, symmetric high-frequency hearing loss. His speech recognition is relatively preserved.

## Encounter 1: source 2021-07-15, practice 2019-09-17, ed, Emergency Medicine

Chief complaint: Several episodes of dizziness and near-syncope over the past week, occasionally feeling like he is 'about to faint' when standing up quickly or walking.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Several episodes of dizziness and near-syncope over the past week, occasionally feeling like he is 'about to faint' when standing up quickly or walking.

HISTORY OF PRESENT ILLNESS:
This patient with presbycusis and degenerative conduction disease presents for emergency evaluation. He denies chest pain but reports occasionally feeling like he is "about to faint" when standing up quickly or walking.

PAST MEDICAL HISTORY:
Active Problem List:
- Presbycusis (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Degenerative conduction system disease

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, right eye (2019)
- Cataract extraction with intraocular lens placement, left eye (2020)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO at bedtime
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Ischemic stroke at age 74
- Mother: Hypertension and Type 2 diabetes mellitus, deceased at age 81
- Older brother: Myocardial infarction at age 68

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (25 pack-year history). Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His blood pressure is 130/70mm Hg, and his pulse is 50

OTHER STUDIES:
An ECG is shown

## Encounter 2: source 2024-01-15, practice 2022-03-17, ed, Emergency Medicine

Chief complaint: Sudden onset of numbness on the right side of his face, arm, and leg without weakness, slurred speech, or visual changes.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of numbness on the right side of his face, arm, and leg without weakness, slurred speech, or visual changes.

HISTORY OF PRESENT ILLNESS:
74-year-old male with hypertension, diabetes, hyperlipidemia, presbycusis, and degenerative conduction system disease presents with sudden onset of numbness on the right side of his face, arm, and leg. He reports no weakness, slurred speech, or visual changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Presbycusis (diagnosed 2020-01-15)
- Mobitz type I atrioventricular block (Wenckebach phenomenon) (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Degenerative conduction system disease

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, right eye (2019)
- Cataract extraction with intraocular lens placement, left eye (2020)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO at bedtime
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Ischemic stroke at age 74
- Mother: Hypertension and Type 2 diabetes mellitus, deceased at age 81
- Older brother: Myocardial infarction at age 68

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (25 pack-year history). Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On exam, he has decreased pinprick, vibration, and proprioception over the entire right side of his body and face. Strength, coordination, gait, and cranial nerve function are otherwise normal.

IMAGING:
MRI reveals a small lacunar infarct.

## Encounter 3: source 2026-01-15, practice 2024-03-07, ed, Emergency Medicine

Chief complaint: Sudden onset of weakness.

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. James Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of weakness.

PAST MEDICAL HISTORY:
Active Problem List:
- Presbycusis (diagnosed 2020-01-15)
- Mobitz type I atrioventricular block (Wenckebach phenomenon) (diagnosed 2021-07-15)
- Lacunar stroke (diagnosed 2024-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Degenerative conduction system disease

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, right eye (2019)
- Cataract extraction with intraocular lens placement, left eye (2020)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO at bedtime
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Ischemic stroke at age 74
- Mother: Hypertension and Type 2 diabetes mellitus, deceased at age 81
- Older brother: Myocardial infarction at age 68

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (25 pack-year history). Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On exam, he has weakness of his right arm and leg, decreased vibration and position sense on the right side, and his tongue deviates to the left when protruded.

## Encounter 4: source 2027-01-15, practice 2025-03-13, inpatient, Neurology

Chief complaint: Sudden weakness that started while having breakfast.

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. Aisha Patel, MD
Department: Neurology
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden weakness that started while having breakfast.

HISTORY OF PRESENT ILLNESS:
Patient with prior weakness and numbness presents because of sudden weakness starting while having breakfast.

PAST MEDICAL HISTORY:
Active Problem List:
- Presbycusis (diagnosed 2020-01-15)
- Mobitz type I atrioventricular block (Wenckebach phenomenon) (diagnosed 2021-07-15)
- Lacunar stroke (diagnosed 2024-01-15)
- Weber syndrome (Midbrain infarction) (diagnosed 2026-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Degenerative conduction system disease

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, right eye (2019)
- Cataract extraction with intraocular lens placement, left eye (2020)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO at bedtime
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Ischemic stroke at age 74
- Mother: Hypertension and Type 2 diabetes mellitus, deceased at age 81
- Older brother: Myocardial infarction at age 68

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (25 pack-year history). Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
He has weakness of his right arm and right leg, decreased vibration and proprioception on the right side of his body, and his tongue deviates to the left when protruded. He has no facial weakness, hoarseness, or swallowing difficulty.

IMAGING:
MRI shows a small infarct in the medulla.

## Encounter 5: source 2028-01-15, practice 2026-03-18, icu, Neurology

Chief complaint: Sudden onset of double vision and weakness.

ENCOUNTER NOTE
Date: 2028-01-15
Provider: Dr. Robert Thompson, MD
Department: Neurology
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of double vision and weakness.

PAST MEDICAL HISTORY:
Active Problem List:
- Presbycusis (diagnosed 2020-01-15)
- Mobitz type I atrioventricular block (Wenckebach phenomenon) (diagnosed 2021-07-15)
- Lacunar stroke (diagnosed 2024-01-15)
- Weber syndrome (Midbrain infarction) (diagnosed 2026-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Degenerative conduction system disease

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, right eye (2019)
- Cataract extraction with intraocular lens placement, left eye (2020)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO at bedtime
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Ischemic stroke at age 74
- Mother: Hypertension and Type 2 diabetes mellitus, deceased at age 81
- Older brother: Myocardial infarction at age 68

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (25 pack-year history). Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On examination, his right eye is "down and out" with a dilated pupil. He has weakness of his left arm and leg. His facial sensation is intact, and he has no swallowing or speech difficulties.

PATHOLOGY:
Brain on autopsy appears as follows.
