# synthetic_hospital patient 2834

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-02-26 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2020-08-17 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2021-08-17 (Tue) | outpatient | Neurology |
| 3 | 2022-01-15 | 2022-02-18 (Fri) | outpatient | Neurology |
| 4 | 2022-07-15 | 2022-08-29 (Mon) | outpatient | Neurology |
| 5 | 2023-07-15 | 2023-08-16 (Wed) | outpatient | Neurology |
| 6 | 2025-01-15 | 2025-02-25 (Tue) | ed | Emergency Medicine |
| 7 | 2026-01-15 | 2026-02-27 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Alzheimer's disease (mother, onset age 75)",
  "Type 2 diabetes mellitus (father)",
  "Hypertension (both parents)",
  "Myocardial infarction (father, age 68)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
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
   "dose": "20 mg PO nightly",
   "name": "Simvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Cataract extraction with intraocular lens placement (bilateral)",
  "Total abdominal hysterectomy (age 45)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Alzheimer's disease", "Delirium due to medical condition", "Hydrochlorothiazide adverse reaction", "Essential hypertension", "Major Depressive Disorder with Psychotic Features"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-02-26, outpatient, Internal Medicine

Chief complaint: Occasional headaches and dizziness for the past two months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Occasional headaches and dizziness for the past two months

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia

She has a past medical history of type 2 diabetes mellitus and hyperlipidemia.

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement (bilateral)
- Total abdominal hysterectomy (age 45)

MEDICATIONS:
Her medications include metformin, simvastatin, and a daily aspirin.

Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer's disease (mother, onset age 75)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
She does not smoke and consumes alcohol occasionally.

VITAL SIGNS:
On physical examination, her blood pressure is 162/78 mmHg, heart rate is 78 beats per minute, and body mass index is 29 kg/m^2.

PHYSICAL EXAMINATION:
Her cardiovascular examination is unremarkable except for a systolic ejection murmur. Fundoscopic examination shows no signs of hypertensive retinopathy.

LABORATORY DATA:
Fasting blood glucose: 138 mg/dL
Hemoglobin A1c: 7.2%
Total cholesterol: 190 mg/dL
LDL cholesterol: 100 mg/dL
HDL cholesterol: 45 mg/dL
Serum creatinine: 1.0 mg/dL
Estimated glomerular filtration rate: 60 mL/min/1.73m^2

## Encounter 1: source 2020-07-15, practice 2020-08-17, outpatient, Internal Medicine

Chief complaint: Fatigue and increased nocturia noted during routine follow-up visit

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and increased nocturia noted during routine follow-up visit

HISTORY OF PRESENT ILLNESS:
The patient with hypertension, type 2 diabetes mellitus, and hyperlipidemia returns for follow-up six months after her initial presentation for headaches and dizziness. She reports feeling generally fatigued and has noticed increased frequency of nocturia.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement (bilateral)
- Total abdominal hysterectomy (age 45)

MEDICATIONS:
Her current medications include metformin, lisinopril, and hydrochlorothiazide.

Home Medications:
- Simvastatin 20 mg PO nightly
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer's disease (mother, onset age 75)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired elementary school teacher.

REVIEW OF SYSTEMS:
She denies any chest pain, palpitations, or edema.

PHYSICAL EXAMINATION:
On physical examination, her blood pressure is 138/86 mmHg, heart rate is 78 bpm, and she appears well-hydrated.

LABORATORY DATA:
Laboratory results are as follows:

- Sodium: 132 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 3.2 mEq/L (Normal: 3.5-5.1 mEq/L)
- Chloride: 98 mEq/L (Normal: 98-106 mEq/L)
- Bicarbonate: 24 mEq/L (Normal: 22-29 mEq/L)
- Blood urea nitrogen (BUN): 18 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 1.0 mg/dL (Normal: 0.6-1.1 mg/dL)
- Glucose (fasting): 126 mg/dL (Normal: 70-99 mg/dL)
- Calcium: 10.6 mg/dL (Normal: 8.5-10.2 mg/dL)
- Magnesium: 1.6 mg/dL (Normal: 1.7-2.2 mg/dL)
- Uric acid: 8.5 mg/dL (Normal: 2.4-6.0 mg/dL)
- Total cholesterol: 210 mg/dL (Normal: <200 mg/dL)
- LDL cholesterol: 140 mg/dL (Normal: <100 mg/dL)
- Triglycerides: 200 mg/dL (Normal: <150 mg/dL)

## Encounter 2: source 2021-07-15, practice 2021-08-17, outpatient, Neurology

Chief complaint: Daughter concerned about mother's increasing forgetfulness and repetition of questions

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Okafor, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Daughter concerned about mother's increasing forgetfulness and repetition of questions

HISTORY OF PRESENT ILLNESS:
The patient is a 73-year-old with hypertension, type 2 diabetes mellitus, and hyperlipidemia previously seen in Internal Medicine who over the past year has become increasingly forgetful, often misplacing items and forgetting appointments. Her daughter also reports that her mother has been repeating questions and is unable to recall recent conversations.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Hydrochlorothiazide adverse reaction (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia

The patient has a past medical history of hypertension and hyperlipidemia, both controlled with medication.

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement (bilateral)
- Total abdominal hysterectomy (age 45)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Simvastatin 20 mg PO nightly
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer's disease (mother, onset age 75)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
She lives alone and manages her daily activities, although she has stopped driving after getting lost on her way home several months ago.

PHYSICAL EXAMINATION:
On examination, the patient is alert and oriented to person and place but not to the date. She scores 20/30 on the Mini-Mental State Examination, losing points in orientation, recall, and attention/calculation. Her gait is normal, and there are no focal neurological deficits.

LABORATORY DATA:
Laboratory Values:
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 0.9 mg/dL
- Glucose: 98 mg/dL
- Thyroid-stimulating hormone: 2.5 μIU/mL
- Vitamin B12: 400 pg/mL

IMAGING:
A CT scan of the head shows generalized cerebral atrophy without evidence of stroke or mass lesions.

## Encounter 3: source 2022-01-15, practice 2022-02-18, outpatient, Neurology

Chief complaint: Daughter reports mother's worsening memory and difficulty finding words during conversations

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Okafor, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Daughter reports mother's worsening memory and difficulty finding words during conversations

HISTORY OF PRESENT ILLNESS:
The patient, who has hypertension, type 2 diabetes mellitus, and hyperlipidemia, returns for follow-up of cognitive concerns from July 2021. Over the past year, the patient has had increasing difficulty remembering recent events, such as appointments and conversations. The daughter also reports that her mother has been misplacing items more frequently and sometimes struggles to find the right words during conversations.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Hydrochlorothiazide adverse reaction (diagnosed 2020-07-15)
- Alzheimer's disease (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia

The patient has a past medical history of hypertension and hyperlipidemia, both controlled with medication.

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement (bilateral)
- Total abdominal hysterectomy (age 45)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Simvastatin 20 mg PO nightly
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer's disease (mother, onset age 75)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
She lives alone and manages her own finances.

PHYSICAL EXAMINATION:
On examination, the patient is alert and oriented to person and place but not to the date. She scores 24 out of 30 on the Mini-Mental State Examination, losing points in orientation, recall, and attention/calculation. Her speech is fluent, and language comprehension is intact. Neurological examination does not reveal any focal deficits.

IMAGING:
A non-contrast CT scan of the head shows generalized cerebral atrophy with more pronounced volume loss in the medial temporal lobes and hippocampi.

## Encounter 4: source 2022-07-15, practice 2022-08-29, outpatient, Neurology

Chief complaint: Daughter concerned about mother's inability to manage finances and frequent misplacement of items

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Priya Patel, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Daughter concerned about mother's inability to manage finances and frequent misplacement of items

HISTORY OF PRESENT ILLNESS:
The patient is a 74-year-old female with a history of Alzheimer's disease evaluated in this department in July 2021 and January 2022, returning for follow-up assessment. She is brought to the clinic by her daughter due to concerns about her mother's memory. Over the past year, the daughter has noticed that her mother forgets recent conversations, misplaces items frequently, and has difficulty managing her finances. The patient has also gotten lost while driving to familiar places on several occasions.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Hydrochlorothiazide adverse reaction (diagnosed 2020-07-15)
- Alzheimer's disease (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia

The patient has a history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement (bilateral)
- Total abdominal hysterectomy (age 45)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Simvastatin 20 mg PO nightly
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer's disease (mother, onset age 75)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
The patient lives alone and has been managing her own medications.

VITAL SIGNS:
Her blood pressure is 140/85 mmHg

PHYSICAL EXAMINATION:
On examination, the patient is well-groomed but appears mildly disoriented to time. She is unable to recall 3 out of 3 objects after 5 minutes and struggles with serial 7s. Her Mini-Mental State Examination (MMSE) score is 22 out of 30.

LABORATORY DATA:
Her fasting blood glucose is 160 mg/dL. Her complete blood count, electrolytes, renal function, and thyroid function tests are within normal limits.

## Encounter 5: source 2023-07-15, practice 2023-08-16, outpatient, Neurology

Chief complaint: Family concerned about worsening forgetfulness, leaving stove on, getting lost, and personality changes

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Priya Patel, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Family concerned about worsening forgetfulness, leaving stove on, getting lost, and personality changes

HISTORY OF PRESENT ILLNESS:
The patient returns to the neurology clinic for continued evaluation of progressive cognitive decline initially assessed in July 2021. Over the past few years, she has become increasingly inattentive, struggles to manage her finances, and often repeats the same questions. She recently left the stove on several times and got lost while walking to a neighbor's house. Family members report that she has become more withdrawn, occasionally irritable, and less able to follow conversations. There is no history of seizures, hallucinations, or abrupt stepwise deterioration.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Hydrochlorothiazide adverse reaction (diagnosed 2020-07-15)
- Alzheimer's disease (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia

Her medical history includes long-standing hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement (bilateral)
- Total abdominal hysterectomy (age 45)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Simvastatin 20 mg PO nightly
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer's disease (mother, onset age 75)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
She does not smoke or drink alcohol.

PHYSICAL EXAMINATION:
On examination, she is disoriented to time and place, cannot recall three objects after a short delay, and has trouble completing a clock-drawing task. Strength, sensation, and coordination are intact.

LABORATORY DATA:
Routine laboratory studies, including vitamin B12 and thyroid function tests, are within normal limits.

## Encounter 6: source 2025-01-15, practice 2025-02-25, ed, Emergency Medicine

Chief complaint: Six-week history of severe depression with auditory hallucinations and refusal to eat

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Six-week history of severe depression with auditory hallucinations and refusal to eat

HISTORY OF PRESENT ILLNESS:
The patient is a 77-year-old female with a five-year history of progressive cognitive decline documented in serial neurology encounters, previously characterized by worsening forgetfulness, difficulty managing finances, and personality changes, who now presents with acute behavioral symptoms. Her daughter reports that the patient has been expressing feelings of hopelessness and guilt, and she has lost interest in activities she used to enjoy. The patient has also been experiencing auditory hallucinations telling her she is worthless and should not eat. Over the past two weeks, she has refused to eat or drink anything, resulting in a 5 kg weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Hydrochlorothiazide adverse reaction (diagnosed 2020-07-15)
- Alzheimer's disease (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia

The patient has a history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement (bilateral)
- Total abdominal hysterectomy (age 45)

MEDICATIONS:
Her current medications include metformin, lisinopril, and sertraline.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Simvastatin 20 mg PO nightly
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer's disease (mother, onset age 75)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg, pulse is 78/min, and BMI is 22 kg/m^2.

PHYSICAL EXAMINATION:
On examination, the patient is lethargic but oriented to person, place, and time.

LABORATORY DATA:
Laboratory tests reveal:

- Sodium: 142 mEq/L
- Potassium: 4.1 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen (BUN): 21 mg/dL
- Creatinine: 1.0 mg/dL
- Fasting glucose: 160 mg/dL
- Hemoglobin A1c: 7.2%
- Thyroid-stimulating hormone (TSH): 2.5 μIU/mL

## Encounter 7: source 2026-01-15, practice 2026-02-27, ed, Emergency Medicine

Chief complaint: Acute increased confusion, agitation, and combative behavior over the past two days

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Aisha Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute increased confusion, agitation, and combative behavior over the past two days

HISTORY OF PRESENT ILLNESS:
The patient is a 78-year-old female with progressive Alzheimer's disease and prior 2025 emergency presentation for depression with psychosis, who is brought to the emergency department by her family due to increased confusion and agitation over the past two days. She has been more combative, yelling at family members, and has had two episodes of pushing her caregiver. Her family denies any recent falls, head trauma, or new medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Hydrochlorothiazide adverse reaction (diagnosed 2020-07-15)
- Alzheimer's disease (diagnosed 2021-07-15)
- Major Depressive Disorder with Psychotic Features (diagnosed 2025-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement (bilateral)
- Total abdominal hysterectomy (age 45)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Simvastatin 20 mg PO nightly
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer's disease (mother, onset age 75)
- Type 2 diabetes mellitus (father)
- Hypertension (both parents)
- Myocardial infarction (father, age 68)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her temperature is 37.2°C (99.0°F), blood pressure is 150/85 mmHg, pulse is 88/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Physical examination reveals an elderly woman who is alert but disoriented to time and place. She is unable to follow complex commands and is easily distractible. Her speech is coherent but with frequent topic changes. Neurological examination is nonfocal.

LABORATORY DATA:
Laboratory studies show:
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 0.9 mg/dL
- Glucose: 98 mg/dL
- Calcium: 9.2 mg/dL
- Complete blood count, liver function tests, and thyroid function tests are within normal limits.
- Urinalysis is unremarkable.

IMAGING:
A chest X-ray shows no acute disease.
