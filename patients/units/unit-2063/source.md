# synthetic_hospital patient 2063

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex M, Non-Hispanic White, Medicare (due to disability) and supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-10-23 (Fri) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2021-04-26 (Mon) | outpatient | Urology |
| 2 | 2021-03-15 | 2021-12-27 (Mon) | outpatient | Internal Medicine |
| 3 | 2022-05-15 | 2023-02-24 (Fri) | outpatient | Endocrinology |
| 4 | 2024-01-15 | 2024-11-05 (Tue) | outpatient | Internal Medicine |
| 5 | 2025-01-15 | 2025-10-28 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare (due to disability) and supplemental private insurance",
 "occupation": "Retired law enforcement officer",
 "alcohol_use": "Lifelong abstainer",
 "family_history": [
  "Father: Type 2 diabetes mellitus, myocardial infarction at age 62",
  "Mother: Hypertension, diabetic nephropathy",
  "One sibling: Gestational diabetes"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 20 years ago (10 pack-year history)",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "24 units subcutaneously at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "Sliding scale subcutaneously with meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "300 mg PO daily",
   "name": "Phenytoin"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (duration 15 years, historically poorly controlled with recent A1c >9%)",
  "Epilepsy, well-controlled",
  "Diabetic peripheral neuropathy (painful, symmetric, lower extremities)",
  "Diabetic autonomic neuropathy (gastrointestinal and cardiovascular)",
  "Neurogenic bladder",
  "Recurrent urinary tract infections"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["Diabetic autonomic neuropathy", "Acute phenytoin toxicity", "Neurogenic bladder", "Diabetic motor neuropathy"]
Comorbidities: ["Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-10-23, outpatient, Endocrinology

Chief complaint: 6-month history of burning and tingling sensations in his feet that worsen at night

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of burning and tingling sensations in his feet that worsen at night

HISTORY OF PRESENT ILLNESS:
He describes the pain as a "sharp" and "electric" sensation that occasionally radiates upward to his lower legs. His diabetes has been managed with metformin and dietary modifications. The patient denies any recent injuries or changes in his medications. He has tried acetaminophen and ibuprofen with minimal relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (duration 15 years, historically poorly controlled with recent A1c >9%)
- Epilepsy, well-controlled
- Diabetic peripheral neuropathy (painful, symmetric, lower extremities)
- Diabetic autonomic neuropathy (gastrointestinal and cardiovascular)
- Neurogenic bladder
- Recurrent urinary tract infections

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes metformin and dietary modifications.

Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin lispro Sliding scale subcutaneously with meals
- Phenytoin 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 62
- Mother: Hypertension, diabetic nephropathy
- One sibling: Gestational diabetes

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (10 pack-year history). Alcohol: Lifelong abstainer. Occupation: Retired law enforcement officer.

PHYSICAL EXAMINATION:
Decreased sensation to light touch and pinprick in a stocking-glove distribution. Ankle reflexes are diminished bilaterally.

LABORATORY DATA:
HbA1c was 7.8%

## Encounter 1: source 2020-07-15, practice 2021-04-26, outpatient, Urology

Chief complaint: Difficulty initiating urination and weak urinary stream for the past 6 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty initiating urination and weak urinary stream for the past 6 months

HISTORY OF PRESENT ILLNESS:
The patient with known neurogenic bladder returns to the Urology clinic six months following his Endocrinology evaluation for diabetic neuropathy. He presents complaining of difficulty initiating urination and a weak urinary stream for the past 6 months. He also reports a sensation of incomplete bladder emptying and occasional dribbling of urine after he thinks he has finished urinating.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic motor neuropathy (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (duration 15 years, historically poorly controlled with recent A1c >9%)
- Epilepsy, well-controlled
- Diabetic peripheral neuropathy (painful, symmetric, lower extremities)
- Diabetic autonomic neuropathy (gastrointestinal and cardiovascular)
- Neurogenic bladder
- Recurrent urinary tract infections

His diabetes has been poorly controlled, with recent hemoglobin A1c levels consistently above 9%.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin lispro Sliding scale subcutaneously with meals
- Phenytoin 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 62
- Mother: Hypertension, diabetic nephropathy
- One sibling: Gestational diabetes

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (10 pack-year history). Alcohol: Lifelong abstainer. Occupation: Retired law enforcement officer.

PHYSICAL EXAMINATION:
His bladder is palpable and percussible above the pubic symphysis

OTHER STUDIES:
A post-void residual urine volume measurement shows 300 mL

## Encounter 2: source 2021-03-15, practice 2021-12-27, outpatient, Internal Medicine

Chief complaint: Severe diarrhea for the past 3 months with postprandial bloating and lightheadedness

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Aisha Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Severe diarrhea for the past 3 months with postprandial bloating and lightheadedness

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old male with Type 2 Diabetes Mellitus returning for management of diabetic peripheral neuropathy and neurogenic bladder, who presents with complaints of severe diarrhea for the past 3 months. He also reports bloating and intermittent episodes of lightheadedness after meals. He has no recent travel history or antibiotic use and denies fever, blood in the stool, or significant weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic motor neuropathy (diagnosed 2020-01-15)
- Neurogenic bladder (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus (duration 15 years, historically poorly controlled with recent A1c >9%)
- Epilepsy, well-controlled
- Diabetic peripheral neuropathy (painful, symmetric, lower extremities)
- Diabetic autonomic neuropathy (gastrointestinal and cardiovascular)
- Recurrent urinary tract infections

His medical history includes type 2 diabetes mellitus, diagnosed 12 years ago.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes for which he takes insulin and metformin.

Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin lispro Sliding scale subcutaneously with meals
- Phenytoin 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 62
- Mother: Hypertension, diabetic nephropathy
- One sibling: Gestational diabetes

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (10 pack-year history). Alcohol: Lifelong abstainer. Occupation: Retired law enforcement officer.

PHYSICAL EXAMINATION:
Physical examination reveals decreased pinprick sensation in both feet and mildly decreased bowel sounds. Cardiovascular and abdominal exams are otherwise normal.

LABORATORY DATA:
Labs are notable for a hemoglobin A1c of 8.9% and a negative stool culture.

OTHER STUDIES:
Colonoscopy performed one month ago showed no structural abnormalities or inflammation.

## Encounter 3: source 2022-05-15, practice 2023-02-24, outpatient, Endocrinology

Chief complaint: Enrollment visit for randomized controlled trial investigating new oral hypoglycemic agent

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Enrollment visit for randomized controlled trial investigating new oral hypoglycemic agent

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic motor neuropathy (diagnosed 2020-01-15)
- Neurogenic bladder (diagnosed 2020-07-15)
- Diabetic autonomic neuropathy (diagnosed 2021-03-15)
- Type 2 Diabetes Mellitus (duration 15 years, historically poorly controlled with recent A1c >9%)
- Epilepsy, well-controlled
- Diabetic peripheral neuropathy (painful, symmetric, lower extremities)
- Diabetic autonomic neuropathy (gastrointestinal and cardiovascular)
- Recurrent urinary tract infections

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin lispro Sliding scale subcutaneously with meals
- Phenytoin 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 62
- Mother: Hypertension, diabetic nephropathy
- One sibling: Gestational diabetes

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (10 pack-year history). Alcohol: Lifelong abstainer. Occupation: Retired law enforcement officer.

OTHER STUDIES:
The study enrolls 60 participants in each arm and is designed to detect a mean HbA1c difference of 0.7% with a standard deviation of 1.2%. The pre-set significance level (alpha) is 0.05. After data analysis, the study finds a p-value of 0.09 for the primary endpoint. The study team calculates a power of 70% for detecting the pre-specified effect size. The investigators are concerned about the possibility of both type I and type II errors.

## Encounter 4: source 2024-01-15, practice 2024-11-05, outpatient, Internal Medicine

Chief complaint: Routine diabetes follow-up with sustained lifestyle modification maintenance

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James O'Brien, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine diabetes follow-up with sustained lifestyle modification maintenance

HISTORY OF PRESENT ILLNESS:
The patient is a 61-year-old male with 15-year Type 2 Diabetes Mellitus complicated by peripheral and autonomic neuropathy and neurogenic bladder who returns for routine follow-up following prior evaluations in 2020 and 2022. He reports he began a comprehensive lifestyle change program with daily 6 am 45-minute walks, cooking his own meals with a whole foods focus, and regular blood sugar tracking. He states, “This has just become my routine now. If I skip a day, I make sure I get right back on track.” He notes his A1c has improved significantly, expresses pride in his progress, and asks about joining a long-term diabetes support group.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic motor neuropathy (diagnosed 2020-01-15)
- Neurogenic bladder (diagnosed 2020-07-15)
- Diabetic autonomic neuropathy (diagnosed 2021-03-15)
- Type 2 Diabetes Mellitus (duration 15 years, historically poorly controlled with recent A1c >9%)
- Epilepsy, well-controlled
- Diabetic peripheral neuropathy (painful, symmetric, lower extremities)
- Diabetic autonomic neuropathy (gastrointestinal and cardiovascular)
- Recurrent urinary tract infections

Six months ago, he was diagnosed with type 2 diabetes.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin lispro Sliding scale subcutaneously with meals
- Phenytoin 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 62
- Mother: Hypertension, diabetic nephropathy
- One sibling: Gestational diabetes

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (10 pack-year history). Alcohol: Lifelong abstainer. Occupation: Retired law enforcement officer.

## Encounter 5: source 2025-01-15, practice 2025-10-28, ed, Emergency Medicine

Chief complaint: 2-day history of unsteadiness while walking, double vision, and nausea

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Elena Volkov, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of unsteadiness while walking, double vision, and nausea

HISTORY OF PRESENT ILLNESS:
This 62-year-old male with Type 2 Diabetes Mellitus, diabetic neuropathy, and epilepsy on phenytoin presents to the emergency department with a 2-day history of unsteadiness while walking, double vision, and nausea. He denies any recent head trauma, alcohol use, or changes in his diet.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic motor neuropathy (diagnosed 2020-01-15)
- Neurogenic bladder (diagnosed 2020-07-15)
- Diabetic autonomic neuropathy (diagnosed 2021-03-15)
- Type 2 Diabetes Mellitus (duration 15 years, historically poorly controlled with recent A1c >9%)
- Epilepsy, well-controlled
- Diabetic peripheral neuropathy (painful, symmetric, lower extremities)
- Diabetic autonomic neuropathy (gastrointestinal and cardiovascular)
- Recurrent urinary tract infections

His other medical conditions include type 2 diabetes mellitus and a recent urinary tract infection for which he started taking trimethoprim-sulfamethoxazole one week ago.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 24 units subcutaneously at bedtime
- Insulin lispro Sliding scale subcutaneously with meals
- Phenytoin 300 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 62
- Mother: Hypertension, diabetic nephropathy
- One sibling: Gestational diabetes

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (10 pack-year history). Alcohol: Lifelong abstainer. Occupation: Retired law enforcement officer.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the patient has horizontal nystagmus and a wide-based gait.

LABORATORY DATA:
Serum phenytoin level: 32 mcg/mL (therapeutic range: 10-20 mcg/mL)
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL
- Blood urea nitrogen (BUN): 14 mg/dL
- Liver function tests: within normal limits
