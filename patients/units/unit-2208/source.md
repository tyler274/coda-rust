# synthetic_hospital patient 2208

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 70 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-02-02 (Tue) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2021-08-02 (Mon) | outpatient | Oncology |
| 2 | 2021-01-15 | 2022-02-14 (Mon) | outpatient | Urology |
| 3 | 2021-07-15 | 2022-08-09 (Tue) | outpatient | Nephrology |
| 4 | 2022-07-15 | 2023-08-07 (Mon) | outpatient | Neurology |
| 5 | 2023-07-15 | 2024-08-12 (Mon) | outpatient | Neurology |
| 6 | 2024-07-15 | 2025-07-29 (Tue) | outpatient | Neurology |
| 7 | 2025-01-15 | 2026-01-28 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired",
 "alcohol_use": "None",
 "family_history": [
  "Father: Prostate cancer",
  "Mother: Cerebrovascular accident",
  "Sister: Type 2 Diabetes Mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
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
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Metoprolol succinate"
  }
 ],
 "surgical_history": [
  "Radical prostatectomy (age 67)",
  "External beam radiation therapy (age 69)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Hyperlipidemia",
  "Prostate cancer (status post radical prostatectomy with biochemical recurrence)",
  "Vascular dementia",
  "History of ischemic stroke"
 ],
 "age_at_first_encounter": 70
}
```

Primary diagnoses: ["Prostate Cancer (Suspected)", "Nephrogenic Diabetes Insipidus", "Ventricular Tachycardia", "Mild cognitive impairment", "Vascular dementia", "Essential hypertension"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-02-02, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a follow-up after being started on a new medication regimen 3 months ago for cardiovascular risk reduction

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a follow-up after being started on a new medication regimen 3 months ago for cardiovascular risk reduction

HISTORY OF PRESENT ILLNESS:
The patient was initiated on a treatment plan to reduce his cardiovascular risk. He reports feeling generally well, with no chest pain, palpitations, or dyspnea. The patient's primary endpoint for his cardiovascular risk reduction was a composite of myocardial infarction, stroke, and cardiovascular death.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Prostate cancer (status post radical prostatectomy with biochemical recurrence)
- Vascular dementia
- History of ischemic stroke

PAST SURGICAL HISTORY:
- Radical prostatectomy (age 67)
- External beam radiation therapy (age 69)

MEDICATIONS:
His current medications include metformin, lisinopril, atorvastatin, and a recently added drug. After reviewing the patient's chart, you note that the new medication added to his regimen was a beta-blocker.

Home Medications:
- Metoprolol succinate 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Prostate cancer
- Mother: Cerebrovascular accident
- Sister: Type 2 Diabetes Mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired.

VITAL SIGNS:
His blood pressure is 132/84 mmHg, and his pulse is 78 bpm.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin A1c: 7.2%
- Total cholesterol: 180 mg/dL
- LDL cholesterol: 90 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 150 mg/dL
- Serum creatinine: 1.1 mg/dL
- Potassium: 4.5 mEq/L

## Encounter 1: source 2020-07-15, practice 2021-08-02, outpatient, Oncology

Chief complaint: increasing fatigue and difficulty initiating urination over the past 6 months with rising PSA levels

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Park, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue and difficulty initiating urination over the past 6 months with rising PSA levels

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with a history of prostate cancer status post radical prostatectomy who presents to the oncology clinic for evaluation of rising prostate-specific antigen levels since his internal medicine appointment six months ago. He underwent radical prostatectomy 3 years ago, and his postoperative course was uneventful. His prostate-specific antigen levels remained undetectable until 6 months ago. Since then, his PSA levels have gradually increased, with the most recent level being 4.5 ng/mL.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Prostate cancer (status post radical prostatectomy with biochemical recurrence)
- Vascular dementia
- History of ischemic stroke

His other medical history includes well-controlled hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Radical prostatectomy (age 67)
- External beam radiation therapy (age 69)

MEDICATIONS:
His current medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Metoprolol succinate 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Prostate cancer
- Mother: Cerebrovascular accident
- Sister: Type 2 Diabetes Mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired.

REVIEW OF SYSTEMS:
He denies any bone pain, weight loss, or other constitutional symptoms.

PHYSICAL EXAMINATION:
Digital rectal examination reveals no abnormalities.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 13.2 g/dL
- PSA: 4.5 ng/mL (increased from <0.1 ng/mL 6 months ago)
- Creatinine: 1.1 mg/dL
- Fasting glucose: 142 mg/dL
- Total cholesterol: 178 mg/dL

IMAGING:
A bone scan is negative for metastatic disease.

## Encounter 2: source 2021-01-15, practice 2022-02-14, outpatient, Urology

Chief complaint: six-month history of nocturia, difficulty initiating urination, and a weak urinary stream without pain

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of nocturia, difficulty initiating urination, and a weak urinary stream without pain

HISTORY OF PRESENT ILLNESS:
The patient returns to urology clinic for urinary symptoms following prior oncology consultation for prostate cancer. He reports 6-month history of nocturia, difficulty initiating urination, and weak urinary stream, and denies pain or burning during urination.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Prostate Cancer (Suspected) (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Prostate cancer (status post radical prostatectomy with biochemical recurrence)
- Vascular dementia
- History of ischemic stroke

His past medical history is significant for hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Radical prostatectomy (age 67)
- External beam radiation therapy (age 69)

MEDICATIONS:
He takes hydrochlorothiazide, metformin.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 25 mg PO daily

ALLERGIES:
He has no known drug allergies

FAMILY HISTORY:
His father was diagnosed with prostate cancer at the age of 75.

SOCIAL HISTORY:
He does not smoke or consume alcohol

VITAL SIGNS:
On physical examination, his blood pressure is 135/85 mmHg, heart rate is 78 bpm, and BMI is 29 kg/m^2.

PHYSICAL EXAMINATION:
Digital rectal examination reveals a symmetrically enlarged, firm, and non-tender prostate without nodules.

LABORATORY DATA:
His serum prostate-specific antigen (PSA) level is 6.5 ng/mL. The rest of the laboratory values are within normal limits.

## Encounter 3: source 2021-07-15, practice 2022-08-09, outpatient, Nephrology

Chief complaint: progressive urinary changes with nocturia and passing large amounts of clear urine despite limiting fluid intake

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Robert Kim, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive urinary changes with nocturia and passing large amounts of clear urine despite limiting fluid intake

HISTORY OF PRESENT ILLNESS:
The patient is a 71-year-old male with a history of prostate cancer status post radical prostatectomy who was treated with external beam radiation approximately one year ago and has since noticed increasing difficulty with urination. Following recent evaluation in Urology for similar symptoms, he now presents with progressive changes over the past 4 months, reporting that he is getting up multiple times at night and passing large amounts of clear urine despite limiting fluid intake before bedtime. He denies dysuria, gross hematuria, or flank pain. He occasionally feels tired during the day due to sleep disruption but has no fever, weight loss, or abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Prostate Cancer (Suspected) (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Prostate cancer (status post radical prostatectomy with biochemical recurrence)
- Vascular dementia
- History of ischemic stroke

PAST SURGICAL HISTORY:
- Radical prostatectomy (age 67)
- External beam radiation therapy (age 69)

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Metformin 1000 mg PO BID
- Metoprolol succinate 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is unremarkable

SOCIAL HISTORY:
He does not smoke or drink alcohol

VITAL SIGNS:
His vital signs are normal

PHYSICAL EXAMINATION:
Abdominal exam shows no palpable masses or suprapubic tenderness

LABORATORY DATA:
Normal serum glucose, calcium, and electrolytes

IMAGING:
Renal ultrasonography demonstrates bilateral hydroureters without evidence of stones or obstructing masses.

## Encounter 4: source 2022-07-15, practice 2023-08-07, outpatient, Neurology

Chief complaint: progressive memory loss over the past year with forgetting recent conversations and difficulty managing finances

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Lisa Thompson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive memory loss over the past year with forgetting recent conversations and difficulty managing finances

HISTORY OF PRESENT ILLNESS:
The patient with vascular dementia and prior stroke returns for neurological evaluation. His caregiver is concerned because he has started to forget recent conversations and has difficulty managing his finances, which was never an issue before.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Prostate Cancer (Suspected) (diagnosed 2020-07-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2021-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Prostate cancer (status post radical prostatectomy with biochemical recurrence)
- Vascular dementia
- History of ischemic stroke

PAST SURGICAL HISTORY:
- Radical prostatectomy (age 67)
- External beam radiation therapy (age 69)

MEDICATIONS:
He takes metformin, lisinopril, and atorvastatin.

Home Medications:
- Metoprolol succinate 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Prostate cancer
- Mother: Cerebrovascular accident
- Sister: Type 2 Diabetes Mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired.

PHYSICAL EXAMINATION:
On physical examination, the patient is alert and oriented to person and place but not to time. He has difficulty recalling the names of his grandchildren and cannot remember the events of the previous day. His Mini-Mental State Examination (MMSE) score is 22 out of 30, indicating mild cognitive impairment. His neurological examination is otherwise unremarkable.

LABORATORY DATA:
- Fasting glucose: 130 mg/dL
- HbA1c: 7.2%
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 100 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 250 mg/dL
- TSH: 2.5 mIU/L
- Vitamin B12: 400 pg/mL

## Encounter 5: source 2023-07-15, practice 2024-08-12, outpatient, Neurology

Chief complaint: gradual decline in ability to plan and organize activities over the past year with two episodes of sudden confusion

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Robert Kim, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradual decline in ability to plan and organize activities over the past year with two episodes of sudden confusion

HISTORY OF PRESENT ILLNESS:
The patient returns to the neurology clinic for ongoing evaluation of cognitive concerns initially assessed in this service approximately one year ago, presenting with his daughter to report progressive neurocognitive symptoms in the context of known cerebrovascular disease. His daughter reports that her father has experienced a gradual decline in his ability to plan and organize activities over the past year. She also notes that he has had two episodes of sudden confusion and weakness in the past six months, each lasting a few hours before resolving. The patient admits to mild forgetfulness but is more concerned about his increasing difficulty in managing his finances.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Prostate Cancer (Suspected) (diagnosed 2020-07-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2021-07-15)
- Mild cognitive impairment (diagnosed 2022-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Prostate cancer (status post radical prostatectomy with biochemical recurrence)
- Vascular dementia
- History of ischemic stroke

PAST SURGICAL HISTORY:
- Radical prostatectomy (age 67)
- External beam radiation therapy (age 69)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Metoprolol succinate 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Prostate cancer
- Mother: Cerebrovascular accident
- Sister: Type 2 Diabetes Mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired.

VITAL SIGNS:
Blood pressure of 145/90 mmHg

PHYSICAL EXAMINATION:
On examination, the patient has a blood pressure of 145/90 mmHg, and a neurological examination reveals mild right-sided weakness and brisk deep tendon reflexes on the right.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin A1c: 7.8%
- Fasting Blood Glucose: 162 mg/dL
- Total Cholesterol: 190 mg/dL
- LDL Cholesterol: 110 mg/dL
- HDL Cholesterol: 45 mg/dL

IMAGING:
An MRI of the brain shows multiple small areas of infarction in the left cerebral hemisphere and extensive periventricular white matter changes.

## Encounter 6: source 2024-07-15, practice 2025-07-29, outpatient, Neurology

Chief complaint: sudden change in ability to perform daily tasks and manage finances since hospital admission for stroke three months ago

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Lisa Thompson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Sudden change in ability to perform daily tasks and manage finances since hospital admission for stroke three months ago

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of progressive cognitive impairment with vascular dementia. She also reports that he has become more irritable and has had difficulty walking since his last hospital admission for a stroke 3 months ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Prostate Cancer (Suspected) (diagnosed 2020-07-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2021-07-15)
- Mild cognitive impairment (diagnosed 2022-07-15)
- Vascular dementia (diagnosed 2023-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Prostate cancer (status post radical prostatectomy with biochemical recurrence)
- History of ischemic stroke

His past medical history includes hypertension, type 2 diabetes mellitus, and hyperlipidemia.

PAST SURGICAL HISTORY:
- Radical prostatectomy (age 67)
- External beam radiation therapy (age 69)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Prostate cancer
- Mother: Cerebrovascular accident
- Sister: Type 2 Diabetes Mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired.

PHYSICAL EXAMINATION:
On examination, the patient has a mild right-sided hemiparesis and a positive Romberg sign. His Mini-Mental State Examination score is 24/30, with deficits noted in attention and calculation. His gait is slow and unsteady.

IMAGING:
A CT scan of the brain shows multiple areas of infarction in both cerebral hemispheres.

## Encounter 7: source 2025-01-15, practice 2026-01-28, ed, Emergency Medicine

Chief complaint: palpitations and lightheadedness that started abruptly while gardening thirty minutes ago

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Palpitations and lightheadedness that started abruptly while gardening thirty minutes ago

HISTORY OF PRESENT ILLNESS:
With vascular dementia and stroke, he reports palpitations and lightheadedness that started abruptly while gardening 30 minutes ago. He denies chest pain, dyspnea, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Prostate Cancer (Suspected) (diagnosed 2020-07-15)
- Nephrogenic Diabetes Insipidus (diagnosed 2021-07-15)
- Mild cognitive impairment (diagnosed 2022-07-15)
- Vascular dementia (diagnosed 2023-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- Prostate cancer (status post radical prostatectomy with biochemical recurrence)
- History of ischemic stroke

PAST SURGICAL HISTORY:
- Radical prostatectomy (age 67)
- External beam radiation therapy (age 69)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Metoprolol succinate 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Prostate cancer
- Mother: Cerebrovascular accident
- Sister: Type 2 Diabetes Mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired.

VITAL SIGNS:
On examination, his blood pressure is 110/70 mmHg, heart rate is 150 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
The patient is alert and oriented. Cardiac auscultation reveals a regular rhythm without murmurs, rubs, or gallops. Lung fields are clear to auscultation bilaterally. Examination of the neck veins shows intermittent, prominent jugular venous pulsations that appear to be independent of the heartbeat.

OTHER STUDIES:
An electrocardiogram (ECG) is performed immediately.
