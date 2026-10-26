# synthetic_hospital patient 2831

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 78 at first encounter, sex F, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-05-04 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2020-07-06 (Mon) | outpatient | Neurology |
| 2 | 2022-01-15 | 2022-05-02 (Mon) | outpatient | Neurology |
| 3 | 2023-01-15 | 2023-05-10 (Wed) | ed | Emergency Medicine |
| 4 | 2023-07-15 | 2023-10-30 (Mon) | ed | Emergency Medicine |
| 5 | 2024-01-15 | 2024-05-11 (Sat) | ed | Emergency Medicine |
| 6 | 2024-07-15 | 2024-11-14 (Thu) | ed | Emergency Medicine |
| 7 | 2026-01-15 | 2026-05-18 (Mon) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "Occasional (1-2 glasses of wine weekly)",
 "family_history": [
  "Mother: Late-onset Alzheimer's disease",
  "Father: Myocardial infarction at age 68",
  "Sister: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "40 mg PO nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "5 mg PO BID",
   "name": "Oxybutynin"
  }
 ],
 "surgical_history": [
  "Cardiac catheterization (12 years prior)",
  "Cholecystectomy (age 45)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Hyperlipidemia",
  "Coronary Artery Disease (status post myocardial infarction 12 years prior)",
  "Mild Cognitive Impairment",
  "Overactive Bladder"
 ],
 "age_at_first_encounter": 78
}
```

Primary diagnoses: ["Major depressive disorder", "Sexual assault", "Malignancy-associated hypercalcemia", "Alzheimer disease", "Delirium due to medical condition", "Anticholinergic toxicity from over-the-counter medication use", "ST-segment elevation myocardial infarction, inferior wall"]
Comorbidities: ["Problems with peer relationships", "Family History of Stroke", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-05-04, outpatient, Internal Medicine

Chief complaint: Patient presents to the clinic with her daughter, who reports that her mother has been increasingly forgetful over the past 6 months. She has lost interest in her hobbies and has been sleeping more than usual, with the patient admitting to feeling hopeless and having difficulty concentrating.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Patient presents to the clinic with her daughter, who reports that her mother has been increasingly forgetful over the past 6 months. She has lost interest in her hobbies and has been sleeping more than usual, with the patient admitting to feeling hopeless and having difficulty concentrating.

HISTORY OF PRESENT ILLNESS:
She has lost interest in her hobbies and has been sleeping more than usual. The patient admits to feeling hopeless and has had difficulty concentrating, which she attributes to "just getting old." She denies any recent medication changes, alcohol use, or head trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Coronary Artery Disease (status post myocardial infarction 12 years prior)
- Mild Cognitive Impairment
- Overactive Bladder

The patient has a history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cardiac catheterization (12 years prior)
- Cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO nightly
- Aspirin 81 mg PO daily
- Oxybutynin 5 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Late-onset Alzheimer's disease
- Father: Myocardial infarction at age 68
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine weekly). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, the patient appears disheveled and has a flat affect. Her Mini-Mental State Examination (MMSE) score is 24/30, with deficits noted in attention and calculation.

LABORATORY DATA:
Her laboratory values are as follows:

- Hemoglobin A1c: 7.2%
- Thyroid-Stimulating Hormone (TSH): 3.5 mIU/L
- Vitamin B12: 450 pg/mL
- Serum creatinine: 1.0 mg/dL
- Electrolytes: Within normal limits

## Encounter 1: source 2020-03-15, practice 2020-07-06, outpatient, Neurology

Chief complaint: Patient presents to the clinic with her daughter, who reports that her mother has been increasingly forgetful over the past 6 months, misplacing items, forgetting appointments, and struggling to recall recent conversations.

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Park, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Patient presents to the clinic with her daughter, who reports that her mother has been increasingly forgetful over the past 6 months, misplacing items, forgetting appointments, and struggling to recall recent conversations.

HISTORY OF PRESENT ILLNESS:
The patient with mild cognitive impairment returns for follow-up of progressive memory symptoms two months after prior assessment. She presents to the clinic with her daughter, who reports that her mother has been increasingly forgetful over the past 6 months. The patient has been misplacing items, forgetting appointments, and struggling to recall recent conversations. The daughter is concerned about possible dementia.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Coronary Artery Disease (status post myocardial infarction 12 years prior)
- Mild Cognitive Impairment
- Overactive Bladder

The patient has a past medical history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cardiac catheterization (12 years prior)
- Cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO nightly
- Aspirin 81 mg PO daily
- Oxybutynin 5 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
As the patient's sister was diagnosed with Alzheimer's disease at the age of 75.

SOCIAL HISTORY:
She lives alone and has become less socially active since her husband passed away 1 year ago.

VITAL SIGNS:
Her blood pressure is 130/80 mmHg,

PHYSICAL EXAMINATION:
On examination, the patient appears disheveled and has a flat affect. She scores 24/30 on the Mini-Mental State Examination (MMSE), losing points in orientation, recall, and attention/calculation.

LABORATORY DATA:
Her fasting blood glucose is 146 mg/dL. A complete blood count, thyroid function tests, and vitamin B12 levels are within normal limits.

## Encounter 2: source 2022-01-15, practice 2022-05-02, outpatient, Neurology

Chief complaint: Patient is brought to the clinic by her daughter due to worsening memory loss and functional decline. Over the past 2 years, the patient has slowly lost the ability to manage her own affairs, forgotten to pay bills, and gotten lost while driving to familiar places.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Michael Park, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Patient is brought to the clinic by her daughter due to worsening memory loss and functional decline. Over the past 2 years, the patient has slowly lost the ability to manage her own affairs, forgotten to pay bills, and gotten lost while driving to familiar places.

HISTORY OF PRESENT ILLNESS:
The patient returns to the neurology clinic for further evaluation of progressive cognitive decline, having been previously assessed in 2020 for memory concerns and managed for major depressive disorder. She presents today accompanied by her daughter, who reports continued functional deterioration over the intervening two years. Over the past 2 years, the patient has slowly lost the ability to manage her own affairs. Initially, she occasionally misplaced household items and repeated the same questions. Over the past 12 months, she has forgotten to pay bills, has gotten lost while driving to familiar places, and has had increasing trouble remembering recent conversations. Her daughter now manages her finances and medications. The daughter also notes personality changes. The patient, once cheerful and outgoing, has become more withdrawn and irritable. She frequently accuses family members of "stealing" objects that she herself misplaced. At family gatherings, she repeats the same story several times in a short period and struggles to recall the names of distant relatives. She sometimes forgets what she had for breakfast, though she can describe in detail events that happened 40 years ago. Her daily functioning has been significantly affected. She can no longer prepare complex meals, balance her checkbook, or follow television shows. Recently, she needed help with dressing, as she put on two blouses and no pants. She sometimes forgets to bathe unless reminded. She is able to feed herself and use the bathroom without assistance, though she occasionally has difficulty remembering where her bedroom is in the house. The patient denies headaches, focal weakness, numbness, or episodes of loss of consciousness. There is no history of strokes or transient ischemic attacks. She has no difficulty walking and denies tremors or stiffness. She has not had visual hallucinations or fluctuations in her alertness.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Coronary Artery Disease (status post myocardial infarction 12 years prior)
- Mild Cognitive Impairment
- Overactive Bladder

Medical history includes well-controlled hypertension and hyperlipidemia. She had a myocardial infarction 12 years ago.

PAST SURGICAL HISTORY:
- Cardiac catheterization (12 years prior)
- Cholecystectomy (age 45)

MEDICATIONS:
She takes aspirin, enalapril, and a statin.

Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO nightly
- Oxybutynin 5 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is significant for her mother, who developed dementia in her late 70s and required nursing home care.

SOCIAL HISTORY:
She does not smoke and drinks wine occasionally.

VITAL SIGNS:
Blood pressure is 142/84 mm Hg, pulse 70/min, respirations 16/min, and temperature 36.8°C.

PHYSICAL EXAMINATION:
On examination, the patient is pleasant but disoriented to date and place. She is neatly dressed but needs prompting to answer many questions. Speech is fluent but circumstantial. Mini-Mental State Examination score is 17/30. She recalls 0 of 3 words after 5 minutes, is unable to copy intersecting pentagons, and cannot draw a clock face. Naming of common objects is intact. Cranial nerves are normal, strength and sensation are symmetric, reflexes are normal, and gait is steady.

LABORATORY DATA:
Laboratory studies, including CBC, electrolytes, thyroid function, and vitamin B12, are normal.

IMAGING:
MRI of the brain shows generalized cortical atrophy most prominent in the hippocampi and temporal lobes, with no evidence of infarcts, masses, or hydrocephalus.

## Encounter 3: source 2023-01-15, practice 2023-05-10, ed, Emergency Medicine

Chief complaint: Patient presents with recent onset of postmenopausal bleeding and uncharacteristic agitation following alleged sexual assault.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Patient presents with recent onset of postmenopausal bleeding and uncharacteristic agitation following alleged sexual assault.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Alzheimer disease (diagnosed 2022-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Coronary Artery Disease (status post myocardial infarction 12 years prior)
- Mild Cognitive Impairment
- Overactive Bladder

The patient has a history of hypertension, well-controlled with hydrochlorothiazide, and type 2 diabetes managed with metformin.

PAST SURGICAL HISTORY:
- Cardiac catheterization (12 years prior)
- Cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO nightly
- Aspirin 81 mg PO daily
- Oxybutynin 5 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Late-onset Alzheimer's disease
- Father: Myocardial infarction at age 68
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
She is a widow and lives alone, although a new caregiver was hired two months ago to assist with daily activities.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the patient appears withdrawn and hesitant to speak in the presence of her caregiver. Pelvic examination reveals a small perineal laceration and bruising on the inner thighs.

LABORATORY DATA:
Laboratory tests show:

- Hemoglobin: 12.1 g/dL
- Platelets: 250 x 10^3/uL
- INR: 1.0
- Urinalysis: Normal

## Encounter 4: source 2023-07-15, practice 2023-10-30, ed, Emergency Medicine

Chief complaint: Patient presents with a 3-month history of fatigue, constipation, and depression, with recent onset of bone pain particularly in her back and hips.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Patient presents with a 3-month history of fatigue, constipation, and depression, with recent onset of bone pain particularly in her back and hips.

HISTORY OF PRESENT ILLNESS:
The patient has Alzheimer disease and depression. She also reports a recent onset of bone pain, particularly in her back and hips.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Alzheimer disease (diagnosed 2022-01-15)
- Sexual assault (diagnosed 2023-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Coronary Artery Disease (status post myocardial infarction 12 years prior)
- Mild Cognitive Impairment
- Overactive Bladder

Her past medical history is significant for hypertension, which is well-controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Cardiac catheterization (12 years prior)
- Cholecystectomy (age 45)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO nightly
- Aspirin 81 mg PO daily
- Oxybutynin 5 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Late-onset Alzheimer's disease
- Father: Myocardial infarction at age 68
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine weekly). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her blood pressure is 142/88 mmHg

PHYSICAL EXAMINATION:
She has mild tenderness over her lumbar spine

LABORATORY DATA:
Serum calcium: 12.1 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum creatinine: 1.0 mg/dL (normal: 0.6-1.2 mg/dL)
- Serum phosphorus: 2.4 mg/dL (normal: 2.5-4.5 mg/dL)
- Alkaline phosphatase: 120 U/L (normal: 40-130 U/L)
- Parathyroid hormone (PTH): 5 pg/mL (normal: 15-65 pg/mL)
- 25-hydroxyvitamin D: 30 ng/mL (normal: 20-50 ng/mL)

## Encounter 5: source 2024-01-15, practice 2024-05-11, ed, Emergency Medicine

Chief complaint: Patient is brought to the emergency department by her daughter because of a 2-day history of increasing confusion, agitation, and difficulty sleeping.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Patient is brought to the emergency department by her daughter because of a 2-day history of increasing confusion, agitation, and difficulty sleeping.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Alzheimer disease (diagnosed 2022-01-15)
- Sexual assault (diagnosed 2023-01-15)
- Malignancy-associated hypercalcemia (diagnosed 2023-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Coronary Artery Disease (status post myocardial infarction 12 years prior)
- Mild Cognitive Impairment
- Overactive Bladder

The patient has a history of hypertension, type 2 diabetes mellitus, and mild cognitive impairment.

PAST SURGICAL HISTORY:
She underwent a total hip replacement 5 days ago.

MEDICATIONS:
Her medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Aspirin 81 mg PO daily
- Oxybutynin 5 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Late-onset Alzheimer's disease
- Father: Myocardial infarction at age 68
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine weekly). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her temperature is 37.8°C (100°F), blood pressure is 150/90 mmHg, pulse is 95/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
On examination, the patient is disoriented to time and place, with fluctuating attention and intermittent hallucinations.

LABORATORY DATA:
Laboratory studies show:

- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 180 mg/dL
- Calcium: 9.2 mg/dL
- Hemoglobin: 12.5 g/dL
- White blood cell count: 9,000/mm^3

A urinalysis is unremarkable.

IMAGING:
A chest X-ray shows no acute disease.

## Encounter 6: source 2024-07-15, practice 2024-11-14, ed, Emergency Medicine

Chief complaint: Patient is agitated, confused, and picking at her bed sheets. Her daughter reports that the symptoms started abruptly a few hours ago with no recent medication changes or illnesses.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Patient is agitated, confused, and picking at her bed sheets. Her daughter reports that the symptoms started abruptly a few hours ago with no recent medication changes or illnesses.

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department six months after a prior delirium episode, presenting with acute behavioral changes in the setting of known Alzheimer disease. Her daughter reports that the symptoms started abruptly a few hours ago and that her mother has been otherwise healthy recently, with no recent illnesses or changes in her chronic medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Alzheimer disease (diagnosed 2022-01-15)
- Sexual assault (diagnosed 2023-01-15)
- Malignancy-associated hypercalcemia (diagnosed 2023-07-15)
- Delirium due to medical condition (diagnosed 2024-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Coronary Artery Disease (status post myocardial infarction 12 years prior)
- Mild Cognitive Impairment
- Overactive Bladder

PAST SURGICAL HISTORY:
- Cardiac catheterization (12 years prior)
- Cholecystectomy (age 45)

MEDICATIONS:
She takes metoprolol, metformin, and oxybutynin.

Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO nightly
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Late-onset Alzheimer's disease
- Father: Myocardial infarction at age 68
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine weekly). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Tachycardic with a heart rate of 110 bpm, blood pressure is 150/90 mmHg, and she is afebrile

PHYSICAL EXAMINATION:
Her skin is dry and flushed, and her pupils are dilated and minimally reactive to light. She is disoriented to time and place.

LABORATORY DATA:
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 160 mg/dL
- Calcium: 9.2 mg/dL

## Encounter 7: source 2026-01-15, practice 2026-05-18, icu, Emergency Medicine

Chief complaint: Patient presents to the emergency department with a 2-hour history of severe fatigue and a feeling of indigestion that has not improved with antacids.

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. David Thompson, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Patient presents to the emergency department with a 2-hour history of severe fatigue and a feeling of indigestion that has not improved with antacids.

HISTORY OF PRESENT ILLNESS:
This 84-year-old female with prior myocardial infarction and Alzheimer disease presents to the emergency department with a 2-hour history of severe fatigue and a feeling of indigestion that has not improved with antacids.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Alzheimer disease (diagnosed 2022-01-15)
- Sexual assault (diagnosed 2023-01-15)
- Malignancy-associated hypercalcemia (diagnosed 2023-07-15)
- Delirium due to medical condition (diagnosed 2024-01-15)
- Anticholinergic toxicity from over-the-counter medication use (diagnosed 2024-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Coronary Artery Disease (status post myocardial infarction 12 years prior)
- Mild Cognitive Impairment
- Overactive Bladder

She has a past medical history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cardiac catheterization (12 years prior)
- Cholecystectomy (age 45)

MEDICATIONS:
Her medications include metformin, lisinopril, and amlodipine.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO nightly
- Aspirin 81 mg PO daily
- Oxybutynin 5 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Late-onset Alzheimer's disease
- Father: Myocardial infarction at age 68
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 glasses of wine weekly). Occupation: Retired elementary school teacher.

REVIEW OF SYSTEMS:
She denies chest pain, shortness of breath, or diaphoresis.

VITAL SIGNS:
On examination, her blood pressure is 160/90 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
She appears uncomfortable but is not in acute distress. Cardiac examination reveals a regular rhythm without murmurs, rubs, or gallops. Lung fields are clear to auscultation bilaterally.

LABORATORY DATA:
Serum cardiac markers are as follows:

- Troponin I: 0.5 ng/mL (normal <0.04 ng/mL)
- Creatine kinase-MB: 25 U/L (normal 0-5 U/L)
- Myoglobin: 90 ng/mL (normal 28-72 ng/mL)

OTHER STUDIES:
An electrocardiogram (ECG) is performed and shows ST-segment elevation in leads II, III, and aVF.
