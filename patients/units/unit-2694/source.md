# synthetic_hospital patient 2694

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex F, Non-Hispanic White, Medicare (ESRD entitlement) with secondary PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2015-12-07 (Mon) | outpatient | Endocrinology |
| 1 | 2020-04-15 | 2016-02-29 (Mon) | outpatient | Cardiology |
| 2 | 2021-01-15 | 2016-12-12 (Mon) | outpatient | Neurology |
| 3 | 2022-01-15 | 2017-12-05 (Tue) | outpatient | Otolaryngology |
| 4 | 2023-01-15 | 2018-12-10 (Mon) | outpatient | Endocrinology |
| 5 | 2024-03-15 | 2020-02-10 (Mon) | outpatient | Oncology |
| 6 | 2026-01-15 | 2021-12-12 (Sun) | ed | Emergency Medicine |
| 7 | 2030-01-15 | 2025-12-10 (Wed) | outpatient | Nephrology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare (ESRD entitlement) with secondary PPO",
 "occupation": "Retired elementary school teacher; avid gardener",
 "alcohol_use": "Social, 1-2 glasses of wine per week",
 "family_history": [
  "Mother: Hypertension, subarachnoid hemorrhage at age 68",
  "Father: Type 2 diabetes mellitus, hyperlipidemia",
  "Maternal aunt: Breast cancer (premenopausal)",
  "Paternal grandmother: Rheumatoid arthritis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit approximately 2 years ago (20 pack-year history)",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "40 mg PO nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "112 mcg PO daily (30 minutes before breakfast)",
   "name": "Levothyroxine"
  },
  {
   "dose": "20 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "15 mg PO weekly (held on dialysis days)",
   "name": "Methotrexate"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "800 mg PO three times daily with meals",
   "name": "Sevelamer carbonate"
  },
  {
   "dose": "4000 units IV with each dialysis session",
   "name": "Epoetin alfa"
  },
  {
   "dose": "4 mg PO every 8 hours as needed for nausea",
   "name": "Ondansetron"
  },
  {
   "dose": "200 mg PO twice daily",
   "name": "Carbamazepine"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy (optimal cytoreductive surgery)",
  "Endovascular coiling of ruptured anterior communicating artery aneurysm",
  "Placement of right internal jugular tunneled hemodialysis catheter",
  "Ramsay Hunt syndrome (Herpes zoster oticus) with subsequent facial nerve decompression"
 ],
 "chronic_conditions": [
  "Hypertension (15-year history)",
  "Hyperlipidemia",
  "Rheumatoid arthritis (seropositive, 15-year history)",
  "Type 2 diabetes mellitus (diagnosed within past year)",
  "Primary hypothyroidism",
  "Stage 5 chronic kidney disease on maintenance hemodialysis (three times weekly)",
  "History of high-grade serous ovarian carcinoma (s/p optimal cytoreductive surgery 14 months ago)"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Hypothyroidism", "Ruptured cerebral aneurysm", "Metronidazole-induced polyneuropathy", "Ramsay Hunt syndrome (Herpes zoster oticus)", "Trigeminal neuralgia", "Dialysis-related amyloidosis"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2015-12-07, outpatient, Endocrinology

Chief complaint: Progressive memory difficulties, fatigue, and unintentional weight gain with heavy menstrual periods

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive memory difficulties, fatigue, and unintentional weight gain with heavy menstrual periods

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of progressive memory difficulties, fatigue, and an unintentional weight gain of 10 pounds. She reports feeling cold even in warm environments and has noticed her skin becoming dry. She also complains of constipation and heavy menstrual periods.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension (15-year history)
- Hyperlipidemia
- Rheumatoid arthritis (seropositive, 15-year history)
- Type 2 diabetes mellitus (diagnosed within past year)
- Primary hypothyroidism
- Stage 5 chronic kidney disease on maintenance hemodialysis (three times weekly)
- History of high-grade serous ovarian carcinoma (s/p optimal cytoreductive surgery 14 months ago)

Her past medical history is significant for hypercholesterolemia.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy (optimal cytoreductive surgery)
- Endovascular coiling of ruptured anterior communicating artery aneurysm
- Placement of right internal jugular tunneled hemodialysis catheter
- Ramsay Hunt syndrome (Herpes zoster oticus) with subsequent facial nerve decompression

MEDICATIONS:
She takes for which she takes atorvastatin.

Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Levothyroxine 112 mcg PO daily (30 minutes before breakfast)
- Insulin glargine 20 units subcutaneous at bedtime
- Methotrexate 15 mg PO weekly (held on dialysis days)
- Prednisone 5 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Epoetin alfa 4000 units IV with each dialysis session
- Ondansetron 4 mg PO every 8 hours as needed for nausea
- Carbamazepine 200 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypertension, subarachnoid hemorrhage at age 68
- Father: Type 2 diabetes mellitus, hyperlipidemia
- Maternal aunt: Breast cancer (premenopausal)
- Paternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit approximately 2 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired elementary school teacher; avid gardener.

VITAL SIGNS:
Heart rate is 58 bpm

PHYSICAL EXAMINATION:
On physical examination, her skin is dry and cool to the touch. Her reflexes are sluggish

LABORATORY DATA:
Serum TSH: 14 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 0.5 ng/dL (normal range: 0.9-1.7 ng/dL)
- Total cholesterol: 245 mg/dL (normal range: <200 mg/dL)
- LDL cholesterol: 160 mg/dL (normal range: <100 mg/dL)

## Encounter 1: source 2020-04-15, practice 2016-02-29, outpatient, Cardiology

Chief complaint: Participation in randomized clinical trial for new antihypertensive medication

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Participation in randomized clinical trial for new antihypertensive medication

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Hypertension (15-year history)
- Hyperlipidemia
- Rheumatoid arthritis (seropositive, 15-year history)
- Type 2 diabetes mellitus (diagnosed within past year)
- Primary hypothyroidism
- Stage 5 chronic kidney disease on maintenance hemodialysis (three times weekly)
- History of high-grade serous ovarian carcinoma (s/p optimal cytoreductive surgery 14 months ago)

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy (optimal cytoreductive surgery)
- Endovascular coiling of ruptured anterior communicating artery aneurysm
- Placement of right internal jugular tunneled hemodialysis catheter
- Ramsay Hunt syndrome (Herpes zoster oticus) with subsequent facial nerve decompression

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 40 mg PO nightly
- Levothyroxine 112 mcg PO daily (30 minutes before breakfast)
- Insulin glargine 20 units subcutaneous at bedtime
- Methotrexate 15 mg PO weekly (held on dialysis days)
- Prednisone 5 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Epoetin alfa 4000 units IV with each dialysis session
- Ondansetron 4 mg PO every 8 hours as needed for nausea
- Carbamazepine 200 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypertension, subarachnoid hemorrhage at age 68
- Father: Type 2 diabetes mellitus, hyperlipidemia
- Maternal aunt: Breast cancer (premenopausal)
- Paternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit approximately 2 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired elementary school teacher; avid gardener.

OTHER STUDIES:
The study enrolls 60 patients per group and aims to detect a 10 mm Hg difference in systolic blood pressure between the new drug and placebo, with a preset alpha of 0.05. At the end of the trial, the mean reduction in systolic blood pressure for the drug group is 7 mm Hg, while the placebo group has a mean reduction of 2 mm Hg (p = 0.08). The investigators conclude that the new drug does not result in a statistically significant reduction in blood pressure compared to placebo. However, a larger subsequent trial with 400 patients per group detects a difference of 7 mm Hg (p = 0.01) and concludes the drug is effective.

## Encounter 2: source 2021-01-15, practice 2016-12-12, outpatient, Neurology

Chief complaint: Brief episodes of severe electric facial pain triggered by chewing

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Watson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Brief episodes of severe electric facial pain triggered by chewing

HISTORY OF PRESENT ILLNESS:
This 53-year-old female with CKD on dialysis and ovarian carcinoma history, seen previously in endocrinology and cardiology, presents with facial pain. She describes the pain as sudden, sharp, and electric in quality, lasting only a few seconds at a time, but recurring multiple times throughout the day. The episodes are often triggered by chewing or light touch to certain areas of her cheek. She denies fever, vision changes, limb weakness, or sensory loss between episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Hypertension (15-year history)
- Hyperlipidemia
- Rheumatoid arthritis (seropositive, 15-year history)
- Type 2 diabetes mellitus (diagnosed within past year)
- Primary hypothyroidism
- Stage 5 chronic kidney disease on maintenance hemodialysis (three times weekly)
- History of high-grade serous ovarian carcinoma (s/p optimal cytoreductive surgery 14 months ago)

Her past medical history includes controlled hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy (optimal cytoreductive surgery)
- Endovascular coiling of ruptured anterior communicating artery aneurysm
- Placement of right internal jugular tunneled hemodialysis catheter
- Ramsay Hunt syndrome (Herpes zoster oticus) with subsequent facial nerve decompression

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 40 mg PO nightly
- Levothyroxine 112 mcg PO daily (30 minutes before breakfast)
- Insulin glargine 20 units subcutaneous at bedtime
- Methotrexate 15 mg PO weekly (held on dialysis days)
- Prednisone 5 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Epoetin alfa 4000 units IV with each dialysis session
- Ondansetron 4 mg PO every 8 hours as needed for nausea
- Carbamazepine 200 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypertension, subarachnoid hemorrhage at age 68
- Father: Type 2 diabetes mellitus, hyperlipidemia
- Maternal aunt: Breast cancer (premenopausal)
- Paternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit approximately 2 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired elementary school teacher; avid gardener.

PHYSICAL EXAMINATION:
Neurologic examination between attacks is normal, with symmetric facial movement, intact corneal reflexes, and preserved sensation.

IMAGING:
An MRI of the brain shows no mass lesion or demyelinating plaques.

## Encounter 3: source 2022-01-15, practice 2017-12-05, outpatient, Otolaryngology

Chief complaint: Severe right ear pain with right-sided facial droop and taste disturbance

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Park, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Severe right ear pain with right-sided facial droop and taste disturbance

HISTORY OF PRESENT ILLNESS:
This immunosuppressed patient with prior trigeminal neuralgia presents with facial symptoms. She reports that the pain is constant, sharp, and worsens with movement of her head. She also complains of a decreased ability to taste.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Trigeminal neuralgia (diagnosed 2021-01-15)
- Hypertension (15-year history)
- Hyperlipidemia
- Rheumatoid arthritis (seropositive, 15-year history)
- Type 2 diabetes mellitus (diagnosed within past year)
- Primary hypothyroidism
- Stage 5 chronic kidney disease on maintenance hemodialysis (three times weekly)
- History of high-grade serous ovarian carcinoma (s/p optimal cytoreductive surgery 14 months ago)

Her past medical history is significant for type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy (optimal cytoreductive surgery)
- Endovascular coiling of ruptured anterior communicating artery aneurysm
- Placement of right internal jugular tunneled hemodialysis catheter
- Ramsay Hunt syndrome (Herpes zoster oticus) with subsequent facial nerve decompression

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 40 mg PO nightly
- Levothyroxine 112 mcg PO daily (30 minutes before breakfast)
- Insulin glargine 20 units subcutaneous at bedtime
- Methotrexate 15 mg PO weekly (held on dialysis days)
- Prednisone 5 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Epoetin alfa 4000 units IV with each dialysis session
- Ondansetron 4 mg PO every 8 hours as needed for nausea
- Carbamazepine 200 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypertension, subarachnoid hemorrhage at age 68
- Father: Type 2 diabetes mellitus, hyperlipidemia
- Maternal aunt: Breast cancer (premenopausal)
- Paternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit approximately 2 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired elementary school teacher; avid gardener.

VITAL SIGNS:
Her vital signs are within normal limits.
Blood pressure: 130/85 mmHg

PHYSICAL EXAMINATION:
On physical examination, you note an erythematous vesicular rash in her right external auditory canal and partial paralysis of the muscles on the right side of her face, including her forehead. Otoscopic examination reveals no signs of acute otitis media.

LABORATORY DATA:
Fasting blood glucose: 162 mg/dL
HbA1c: 7.8%
Total cholesterol: 210 mg/dL
HDL: 45 mg/dL
LDL: 145 mg/dL

## Encounter 4: source 2023-01-15, practice 2018-12-10, outpatient, Endocrinology

Chief complaint: Six-month history of fatigue, weight gain, constipation, and cold intolerance

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of fatigue, weight gain, constipation, and cold intolerance

HISTORY OF PRESENT ILLNESS:
The patient returns for hypothyroidism follow-up. She also reports feeling cold all the time, even in warm environments.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Trigeminal neuralgia (diagnosed 2021-01-15)
- Ramsay Hunt syndrome (Herpes zoster oticus) (diagnosed 2022-01-15)
- Hypertension (15-year history)
- Hyperlipidemia
- Rheumatoid arthritis (seropositive, 15-year history)
- Type 2 diabetes mellitus (diagnosed within past year)
- Primary hypothyroidism
- Stage 5 chronic kidney disease on maintenance hemodialysis (three times weekly)
- History of high-grade serous ovarian carcinoma (s/p optimal cytoreductive surgery 14 months ago)

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy (optimal cytoreductive surgery)
- Endovascular coiling of ruptured anterior communicating artery aneurysm
- Placement of right internal jugular tunneled hemodialysis catheter
- Ramsay Hunt syndrome (Herpes zoster oticus) with subsequent facial nerve decompression

MEDICATIONS:
She takes for which she takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO nightly
- Levothyroxine 112 mcg PO daily (30 minutes before breakfast)
- Insulin glargine 20 units subcutaneous at bedtime
- Methotrexate 15 mg PO weekly (held on dialysis days)
- Prednisone 5 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Epoetin alfa 4000 units IV with each dialysis session
- Ondansetron 4 mg PO every 8 hours as needed for nausea
- Carbamazepine 200 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypertension, subarachnoid hemorrhage at age 68
- Father: Type 2 diabetes mellitus, hyperlipidemia
- Maternal aunt: Breast cancer (premenopausal)
- Paternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit approximately 2 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired elementary school teacher; avid gardener.

VITAL SIGNS:
Her heart rate is 58 bpm, and her blood pressure is 142/88 mmHg.

PHYSICAL EXAMINATION:
On physical examination, her skin is dry, and she has non-pitting edema around her eyes.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum TSH: 12.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 0.7 ng/dL (normal range: 0.9-1.7 ng/dL)
- Total cholesterol: 245 mg/dL (normal range: 40 mg/dL)
- Triglycerides: 220 mg/dL (normal range: <150 mg/dL)

## Encounter 5: source 2024-03-15, practice 2020-02-10, outpatient, Oncology

Chief complaint: Progressive paresthesia and neuropathy three days after new chemotherapy infusion

ENCOUNTER NOTE
Date: 2024-03-15
Provider: Dr. Anita Patel, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive paresthesia and neuropathy three days after new chemotherapy infusion

HISTORY OF PRESENT ILLNESS:
The patient is a 56-year-old female with stage 5 chronic kidney disease on maintenance hemodialysis, seropositive rheumatoid arthritis, and prior neurological conditions including trigeminal neuralgia and Ramsay Hunt syndrome, who returns to the oncology clinic for evaluation of progressive paresthesia. She was diagnosed 14 months ago and initially treated with optimal cytoreductive surgery followed by a platinum-based regimen, but her disease progressed after 2 additional lines of chemotherapy. Her oncologist started her on a new agent targeting rapidly dividing cells. She now reports progressive tingling, numbness, and burning pain in her toes and fingertips, which have been gradually ascending over the past 48 hours. She also notes mild weakness when climbing stairs, generalized fatigue, and increased hair shedding.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Trigeminal neuralgia (diagnosed 2021-01-15)
- Ramsay Hunt syndrome (Herpes zoster oticus) (diagnosed 2022-01-15)
- Hypertension (15-year history)
- Hyperlipidemia
- Rheumatoid arthritis (seropositive, 15-year history)
- Type 2 diabetes mellitus (diagnosed within past year)
- Primary hypothyroidism
- Stage 5 chronic kidney disease on maintenance hemodialysis (three times weekly)
- History of high-grade serous ovarian carcinoma (s/p optimal cytoreductive surgery 14 months ago)

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy (optimal cytoreductive surgery)
- Endovascular coiling of ruptured anterior communicating artery aneurysm
- Placement of right internal jugular tunneled hemodialysis catheter
- Ramsay Hunt syndrome (Herpes zoster oticus) with subsequent facial nerve decompression

MEDICATIONS:
Medications include lisinopril, atorvastatin, and ondansetron as needed for nausea.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Levothyroxine 112 mcg PO daily (30 minutes before breakfast)
- Insulin glargine 20 units subcutaneous at bedtime
- Methotrexate 15 mg PO weekly (held on dialysis days)
- Prednisone 5 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Epoetin alfa 4000 units IV with each dialysis session
- Carbamazepine 200 mg PO twice daily

ALLERGIES:
She has no known drug allergies.

FAMILY HISTORY:
- Mother: Hypertension, subarachnoid hemorrhage at age 68
- Father: Type 2 diabetes mellitus, hyperlipidemia
- Maternal aunt: Breast cancer (premenopausal)
- Paternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
She is a former smoker

VITAL SIGNS:
On examination, she is afebrile with stable vital signs.

PHYSICAL EXAMINATION:
Neurologic exam reveals decreased pinprick and vibration sensation in a stocking-glove distribution, mild weakness in bilateral hip flexors, and slightly reduced ankle reflexes. Upper extremity strength is intact. Her surgical scars are well-healed, and there is no abdominal tenderness or organomegaly.

LABORATORY DATA:
Laboratory evaluation shows normal complete blood count, renal function, and electrolytes.

## Encounter 6: source 2026-01-15, practice 2021-12-12, ed, Emergency Medicine

Chief complaint: Sudden onset severe thunderclap headache while gardening with nausea and photophobia

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset severe thunderclap headache while gardening with nausea and photophobia

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old female with a history of trigeminal neuralgia, Ramsay Hunt syndrome, and stage 5 chronic kidney disease on maintenance hemodialysis who presents with sudden onset severe thunderclap headache which started 2 hours ago while she was gardening. She also reports nausea and photophobia.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Trigeminal neuralgia (diagnosed 2021-01-15)
- Ramsay Hunt syndrome (Herpes zoster oticus) (diagnosed 2022-01-15)
- Metronidazole-induced polyneuropathy (diagnosed 2024-03-15)
- Hypertension (15-year history)
- Hyperlipidemia
- Rheumatoid arthritis (seropositive, 15-year history)
- Type 2 diabetes mellitus (diagnosed within past year)
- Primary hypothyroidism
- Stage 5 chronic kidney disease on maintenance hemodialysis (three times weekly)
- History of high-grade serous ovarian carcinoma (s/p optimal cytoreductive surgery 14 months ago)

Her past medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy (optimal cytoreductive surgery)
- Endovascular coiling of ruptured anterior communicating artery aneurysm
- Placement of right internal jugular tunneled hemodialysis catheter
- Ramsay Hunt syndrome (Herpes zoster oticus) with subsequent facial nerve decompression

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 40 mg PO nightly
- Levothyroxine 112 mcg PO daily (30 minutes before breakfast)
- Insulin glargine 20 units subcutaneous at bedtime
- Methotrexate 15 mg PO weekly (held on dialysis days)
- Prednisone 5 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Epoetin alfa 4000 units IV with each dialysis session
- Ondansetron 4 mg PO every 8 hours as needed for nausea
- Carbamazepine 200 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypertension, subarachnoid hemorrhage at age 68
- Father: Type 2 diabetes mellitus, hyperlipidemia
- Maternal aunt: Breast cancer (premenopausal)
- Paternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit approximately 2 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired elementary school teacher; avid gardener.

VITAL SIGNS:
Blood pressure of 165/95 mmHg and a heart rate of 88 beats per minute

PHYSICAL EXAMINATION:
On examination, she is alert but anxious, with a blood pressure of 165/95 mmHg and a heart rate of 88 beats per minute. Neurological examination reveals neck stiffness and a positive Brudzinski's sign.

IMAGING:
A non-contrast head CT scan shows a diffuse subarachnoid hemorrhage. Cerebral angiography does not reveal an aneurysm.

PLAN:
The patient is admitted to the neurointensive care unit for monitoring and management.

## Encounter 7: source 2030-01-15, practice 2025-12-10, outpatient, Nephrology

Chief complaint: Increasing fatigue, leg swelling, and foamy urine over three months

ENCOUNTER NOTE
Date: 2030-01-15
Provider: Dr. Lisa Johnson, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue, leg swelling, and foamy urine over three months

HISTORY OF PRESENT ILLNESS:
The patient with Stage 5 CKD on hemodialysis and history of ovarian carcinoma and cerebral aneurysm presents to the clinic with a 3-month history of increasing fatigue, swelling of her legs, and foamy urine.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Trigeminal neuralgia (diagnosed 2021-01-15)
- Ramsay Hunt syndrome (Herpes zoster oticus) (diagnosed 2022-01-15)
- Metronidazole-induced polyneuropathy (diagnosed 2024-03-15)
- Ruptured cerebral aneurysm (diagnosed 2026-01-15)
- Hypertension (15-year history)
- Hyperlipidemia
- Rheumatoid arthritis (seropositive, 15-year history)
- Type 2 diabetes mellitus (diagnosed within past year)
- Primary hypothyroidism
- Stage 5 chronic kidney disease on maintenance hemodialysis (three times weekly)
- History of high-grade serous ovarian carcinoma (s/p optimal cytoreductive surgery 14 months ago)

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and omentectomy (optimal cytoreductive surgery)
- Endovascular coiling of ruptured anterior communicating artery aneurysm
- Placement of right internal jugular tunneled hemodialysis catheter
- Ramsay Hunt syndrome (Herpes zoster oticus) with subsequent facial nerve decompression

MEDICATIONS:
She has been on methotrexate and low-dose prednisone for the past 10 years.

Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 40 mg PO nightly
- Levothyroxine 112 mcg PO daily (30 minutes before breakfast)
- Insulin glargine 20 units subcutaneous at bedtime
- Sevelamer carbonate 800 mg PO three times daily with meals
- Epoetin alfa 4000 units IV with each dialysis session
- Ondansetron 4 mg PO every 8 hours as needed for nausea
- Carbamazepine 200 mg PO twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypertension, subarachnoid hemorrhage at age 68
- Father: Type 2 diabetes mellitus, hyperlipidemia
- Maternal aunt: Breast cancer (premenopausal)
- Paternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, quit approximately 2 years ago (20 pack-year history). Alcohol: Social, 1-2 glasses of wine per week. Occupation: Retired elementary school teacher; avid gardener.

PHYSICAL EXAMINATION:
On physical examination, her blood pressure is 145/90 mmHg, and she has bilateral pitting edema up to her knees.

LABORATORY DATA:
Laboratory studies reveal:

- Serum albumin: 2.8 g/dL (Normal: 3.5-5.0 g/dL)
- Serum creatinine: 1.2 mg/dL (Normal: 0.6-1.1 mg/dL)
- Urine protein: 4.5 g/24 hours (Normal: <150 mg/24 hours)
- Urine sediment: No cells or casts
- Serum total cholesterol: 260 mg/dL (Normal: <200 mg/dL)

PATHOLOGY:
A renal biopsy is performed, and the pathologist reports the presence of amyloid deposits that stain with Congo red and demonstrate apple-green birefringence under polarized light.
