# synthetic_hospital patient 2218

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 70 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-04-20 (Wed) | outpatient | Urology |
| 1 | 2020-07-15 | 2022-10-10 (Mon) | outpatient | Urology |
| 2 | 2021-07-15 | 2023-10-11 (Wed) | ed | Emergency Medicine |
| 3 | 2021-09-15 | 2023-12-13 (Wed) | outpatient | Internal Medicine |
| 4 | 2022-01-15 | 2024-04-22 (Mon) | ed | Emergency Medicine |
| 5 | 2022-07-15 | 2024-10-14 (Mon) | ed | Emergency Medicine |
| 6 | 2023-01-15 | 2025-04-21 (Mon) | outpatient | Internal Medicine |
| 7 | 2023-07-15 | 2025-10-16 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Type 2 Diabetes Mellitus",
  "Mother: Hypertension",
  "Older brother: Benign Prostatic Hyperplasia"
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
   "dose": "0.4 mg PO daily",
   "name": "Tamsulosin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Right total knee arthroplasty"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Benign Prostatic Hyperplasia"
 ],
 "age_at_first_encounter": 70
}
```

Primary diagnoses: ["Recurrent benign prostatic hyperplasia", "Urinary retention", "Hyperkalemia", "Orthostatic hypotension", "Delirium due to medical condition"]
Comorbidities: ["Recurrent benign prostatic hyperplasia", "Urinary tract infection", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-04-20, outpatient, Urology

Chief complaint: 6-month history of urinary frequency, nocturia, and hesitancy with weak urinary stream

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of urinary frequency, nocturia, and hesitancy with weak urinary stream

HISTORY OF PRESENT ILLNESS:
He reports no pain during urination but has noticed a weak urinary stream.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Benign Prostatic Hyperplasia

His past medical history includes well-controlled hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Right total knee arthroplasty

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Tamsulosin 0.4 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His father died of prostate cancer at the age of 75.

SOCIAL HISTORY:
He has no history of smoking and drinks alcohol socially.

VITAL SIGNS:
His blood pressure is 135/85 mmHg, heart rate is 78 beats per minute, and BMI is 29 kg/m^2

PHYSICAL EXAMINATION:
Digital rectal examination reveals a symmetrically enlarged, firm, and non-tender prostate.

LABORATORY DATA:
His urinalysis shows no evidence of infection or hematuria. Serum creatinine is 1.0 mg/dL, and fasting glucose is 140 mg/dL.

## Encounter 1: source 2020-07-15, practice 2022-10-10, outpatient, Urology

Chief complaint: Urinary frequency, urgency, nocturia, and sensation of incomplete bladder emptying with two UTIs in past year

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Urinary frequency, urgency, nocturia, and sensation of incomplete bladder emptying with two UTIs in past year

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with recurrent benign prostatic hyperplasia who returns to the urology clinic six months following his initial evaluation. He presents with a 6-month history of urinary frequency, urgency, and nocturia. He reports a sensation of incomplete bladder emptying and has had two episodes of urinary tract infections in the past year, which were treated with antibiotics.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Benign Prostatic Hyperplasia

His medical history is significant for well-controlled hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Right total knee arthroplasty

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Tamsulosin 0.4 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus
- Mother: Hypertension
- Older brother: Benign Prostatic Hyperplasia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, his abdomen is soft and non-tender, and a digital rectal exam reveals a symmetrically enlarged, firm, and non-tender prostate.

LABORATORY DATA:
Urinalysis shows no signs of current infection. Serum creatinine is 1.2 mg/dL, and blood urea nitrogen (BUN) is 18 mg/dL.

OTHER STUDIES:
Post-void residual volume measured by bladder ultrasound is 120 mL.

## Encounter 2: source 2021-07-15, practice 2023-10-11, ed, Emergency Medicine

Chief complaint: Severe lower abdominal pain and inability to pass urine for 8 hours

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe lower abdominal pain and inability to pass urine for 8 hours

HISTORY OF PRESENT ILLNESS:
The patient with BPH presents following Urology visits in January and July 2020. He reports a history of increasing difficulty with urination over the past year, including a weak urinary stream, nocturia, and a feeling of incomplete bladder emptying.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Benign Prostatic Hyperplasia

PAST SURGICAL HISTORY:
He underwent an elective inguinal hernia repair 2 days ago.

MEDICATIONS:
He takes hydrochlorothiazide, metformin.

Home Medications:
- Lisinopril 10 mg PO daily
- Tamsulosin 0.4 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus
- Mother: Hypertension
- Older brother: Benign Prostatic Hyperplasia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Blood pressure is 145/90 mmHg, heart rate is 88 beats per minute

PHYSICAL EXAMINATION:
He appears uncomfortable. His abdomen is distended and tender in the suprapubic region. Digital rectal examination reveals an enlarged, firm, and non-tender prostate.

IMAGING:
A bladder ultrasound shows a significantly distended bladder with an estimated volume of 600 mL.

## Encounter 3: source 2021-09-15, practice 2023-12-13, outpatient, Internal Medicine

Chief complaint: Recurrent episodes of lightheadedness when standing from sitting position over past two months

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent episodes of lightheadedness when standing from sitting position over past two months

HISTORY OF PRESENT ILLNESS:
The patient, with benign prostatic hyperplasia and recent urinary retention, presents with episodes of lightheadedness occurring particularly when he stands up from a sitting position. He reports these episodes have been occurring for the past two months and are sometimes accompanied by dizziness and blurred vision.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Urinary retention (diagnosed 2021-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Benign Prostatic Hyperplasia

His past medical history is significant for type 2 diabetes mellitus, hypertension, and benign prostatic hyperplasia.

PAST SURGICAL HISTORY:
- Right total knee arthroplasty

MEDICATIONS:
His current medications include metformin, lisinopril, and tamsulosin.

Home Medications:
- Atorvastatin 20 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus
- Mother: Hypertension
- Older brother: Benign Prostatic Hyperplasia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On physical examination, his blood pressure is 145/85 mmHg while seated and 120/70 mmHg after standing for 3 minutes. His heart rate increases from 78 bpm seated to 88 bpm standing.

PHYSICAL EXAMINATION:
The remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin A1c: 7.2%
- Serum creatinine: 1.1 mg/dL
- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L

## Encounter 4: source 2022-01-15, practice 2024-04-22, ed, Emergency Medicine

Chief complaint: Lower abdominal discomfort and inability to void urine 18 hours after right total knee arthroplasty

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Jennifer Walsh, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Lower abdominal discomfort and inability to void urine 18 hours after right total knee arthroplasty

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with recurrent benign prostatic hyperplasia and prior episode of acute urinary retention in July 2021 who presents with lower abdominal discomfort and inability to void urine since undergoing a right total knee arthroplasty 18 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Urinary retention (diagnosed 2021-07-15)
- Orthostatic hypotension (diagnosed 2021-09-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Benign Prostatic Hyperplasia

His past medical history includes benign prostatic hyperplasia, hypertension, and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Right total knee arthroplasty

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Tamsulosin 0.4 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus
- Mother: Hypertension
- Older brother: Benign Prostatic Hyperplasia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On examination, his blood pressure is 165/90 mmHg, heart rate is 102 beats per minute

PHYSICAL EXAMINATION:
He appears to be in distress. His abdomen is distended and tender to palpation in the suprapubic region. A Foley catheter was attempted but unsuccessful.

IMAGING:
A portable bladder ultrasound is performed and shows a bladder volume of 750 mL.

## Encounter 5: source 2022-07-15, practice 2024-10-14, ed, Emergency Medicine

Chief complaint: 24-hour history of severe lower abdominal pain and inability to pass urine

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
24-hour history of severe lower abdominal pain and inability to pass urine

HISTORY OF PRESENT ILLNESS:
The patient returns with recurrent inability to void and reports a 6-month history of difficulty initiating urination, a weak urinary stream, and nocturia.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Urinary retention (diagnosed 2021-07-15)
- Orthostatic hypotension (diagnosed 2021-09-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Benign Prostatic Hyperplasia

PAST SURGICAL HISTORY:
- Right total knee arthroplasty

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Tamsulosin 0.4 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus
- Mother: Hypertension
- Older brother: Benign Prostatic Hyperplasia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
Abdomen is distended and tender in the suprapubic region. Digital rectal examination reveals an enlarged, firm, and non-tender prostate

LABORATORY DATA:
Serum creatinine is 1.2 mg/dL (normal: 0.6-1.2 mg/dL), blood urea nitrogen is 18 mg/dL (normal: 7-20 mg/dL)

IMAGING:
Bladder scan shows a postvoid residual volume of 300 mL

## Encounter 6: source 2023-01-15, practice 2025-04-21, outpatient, Internal Medicine

Chief complaint: Routine follow-up with increased fatigue and muscle weakness over past two weeks

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with increased fatigue and muscle weakness over past two weeks

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of diabetes, hypertension, and BPH with prior retention. He reports feeling generally well but has noticed increased fatigue and muscle weakness over the past two weeks. He denies any chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Urinary retention (diagnosed 2021-07-15)
- Orthostatic hypotension (diagnosed 2021-09-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Benign Prostatic Hyperplasia

PAST SURGICAL HISTORY:
- Right total knee arthroplasty

MEDICATIONS:
His current medications include metformin, lisinopril, and trimethoprim-sulfamethoxazole for a recent urinary tract infection.

Home Medications:
- Tamsulosin 0.4 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus
- Mother: Hypertension
- Older brother: Benign Prostatic Hyperplasia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
The physical examination is unremarkable

LABORATORY DATA:
Laboratory results are as follows:

- Serum potassium: 5.8 mEq/L (Normal: 3.5-5.0 mEq/L)
- Serum creatinine: 1.4 mg/dL (Baseline: 1.0 mg/dL)
- Blood urea nitrogen (BUN): 18 mg/dL (Normal: 7-20 mg/dL)
- Serum sodium: 140 mEq/L (Normal: 135-145 mEq/L)
- Serum glucose: 160 mg/dL (Normal: 70-99 mg/dL)

## Encounter 7: source 2023-07-15, practice 2025-10-16, ed, Emergency Medicine

Chief complaint: Acute confusion, agitation, and visual hallucinations over past two days per wife

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute confusion, agitation, and visual hallucinations over past two days per wife

HISTORY OF PRESENT ILLNESS:
The patient with history of BPH and urinary retention presents with confusion. Per wife, over the past two days, he has been increasingly confused, agitated, and has been having visual hallucinations, particularly at night.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Urinary retention (diagnosed 2021-07-15)
- Orthostatic hypotension (diagnosed 2021-09-15)
- Hyperkalemia (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Benign Prostatic Hyperplasia

PAST SURGICAL HISTORY:
- Right total knee arthroplasty

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Tamsulosin 0.4 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus
- Mother: Hypertension
- Older brother: Benign Prostatic Hyperplasia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Temperature 37.8°C, blood pressure 160/90 mmHg, heart rate 102 beats per minute, and respiratory rate 18 breaths per minute

PHYSICAL EXAMINATION:
On examination, the patient is disoriented to time and place, with a fluctuating level of consciousness. Physical examination is unremarkable except for mild suprapubic tenderness.

LABORATORY DATA:
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 1.2 mg/dL
- Glucose: 180 mg/dL
- White blood cell count: 9,000/uL
- Hemoglobin: 13.5 g/dL
- Urinalysis: Positive for leukocyte esterase and nitrites
