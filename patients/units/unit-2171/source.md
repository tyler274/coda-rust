# synthetic_hospital patient 2171

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-10-09 (Mon) | outpatient | Cardiology |
| 1 | 2021-07-15 | 2025-04-01 (Tue) | ed | Emergency Medicine |
| 2 | 2022-07-15 | 2026-04-14 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin - rash",
  "Shellfish - anaphylaxis"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Logistics coordinator",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Prostate cancer, diagnosed age 72, deceased",
  "Mother: Breast cancer, diagnosed age 68",
  "Maternal grandfather: Coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 8 years ago",
 "home_medications": [
  {
   "dose": "7.5 mg IM every 28 days",
   "name": "Leuprolide"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "600 mg/400 IU PO twice daily",
   "name": "Calcium carbonate/Vitamin D3"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Staging laparotomy with splenectomy (20 years ago)",
  "Radical retropubic prostatectomy with bilateral pelvic lymphadenectomy (3 years ago)",
  "Right hip arthroscopy (10 years ago)"
 ],
 "chronic_conditions": [
  "Hodgkin lymphoma, Stage II, in remission status post mantle field radiation therapy 20 years prior",
  "Prostate adenocarcinoma, Gleason 7 (3+4), status post radical prostatectomy and adjuvant external beam radiation therapy 3 years prior",
  "Hypertension",
  "Hyperlipidemia",
  "Osteopenia secondary to androgen deprivation therapy"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["Spinal metastasis with cord compression", "Constrictive pericarditis", "Cauda equina syndrome"]
Comorbidities: ["Prostate Cancer (Suspected)", "History of pelvic radiation therapy"]

## Encounter 0: source 2020-01-15, practice 2023-10-09, outpatient, Cardiology

Chief complaint: Six-month history of increasing abdominal girth, lower extremity swelling, and dyspnea on exertion

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. James Patterson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of increasing abdominal girth, lower extremity swelling, and dyspnea on exertion

HISTORY OF PRESENT ILLNESS:
6-month history of increasing abdominal girth, lower extremity swelling, and dyspnea on exertion. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Hodgkin lymphoma, Stage II, in remission status post mantle field radiation therapy 20 years prior
- Prostate adenocarcinoma, Gleason 7 (3+4), status post radical prostatectomy and adjuvant external beam radiation therapy 3 years prior
- Hypertension
- Hyperlipidemia
- Osteopenia secondary to androgen deprivation therapy

PAST SURGICAL HISTORY:
- Staging laparotomy with splenectomy (20 years ago)
- Radical retropubic prostatectomy with bilateral pelvic lymphadenectomy (3 years ago)
- Right hip arthroscopy (10 years ago)

MEDICATIONS:
Home Medications:
- Leuprolide 7.5 mg IM every 28 days
- Amlodipine 5 mg PO daily
- Atorvastatin 20 mg PO daily
- Calcium carbonate/Vitamin D3 600 mg/400 IU PO twice daily
- Aspirin 81 mg PO daily

ALLERGIES:
Penicillin - rash, Shellfish - anaphylaxis

FAMILY HISTORY:
- Father: Prostate cancer, diagnosed age 72, deceased
- Mother: Breast cancer, diagnosed age 68
- Maternal grandfather: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Logistics coordinator.

VITAL SIGNS:
Blood pressure is 115/75 mmHg, heart rate is 88 bpm, and respiratory rate is 18 breaths per minute

PHYSICAL EXAMINATION:
He has elevated jugular venous pressure, a pericardial knock is heard on auscultation, and his abdomen is distended with a fluid wave consistent with ascites. His liver is palpable 3 cm below the right costal margin, and there is 2+ pitting edema in both lower extremities up to the knees.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,800 /μL
- Platelet count: 235,000 /μL
- Serum creatinine: 1.1 mg/dL
- Serum albumin: 3.5 g/dL
- Alanine aminotransferase (ALT): 45 U/L
- Aspartate aminotransferase (AST): 50 U/L
- Total bilirubin: 1.2 mg/dL
- Brain natriuretic peptide (BNP): 150 pg/mL

IMAGING:
Chest radiograph reveals pericardial calcifications. An echocardiogram shows thickened pericardium with normal systolic function and impaired diastolic filling.

## Encounter 1: source 2021-07-15, practice 2025-04-01, ed, Emergency Medicine

Chief complaint: Two-week history of progressive lower back pain, difficulty walking, and urinary incontinence

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Sharma, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of progressive lower back pain, difficulty walking, and urinary incontinence

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with Hodgkin lymphoma, prostate cancer, and constrictive pericarditis reports a sensation of "pins and needles" in his legs and has experienced several episodes of urinary incontinence over the past few days.

PAST MEDICAL HISTORY:
Active Problem List:
- Constrictive pericarditis (diagnosed 2020-01-15)
- Hodgkin lymphoma, Stage II, in remission status post mantle field radiation therapy 20 years prior
- Prostate adenocarcinoma, Gleason 7 (3+4), status post radical prostatectomy and adjuvant external beam radiation therapy 3 years prior
- Hypertension
- Hyperlipidemia
- Osteopenia secondary to androgen deprivation therapy

His medical history is significant for prostate cancer treated with radical prostatectomy 3 years ago.

PAST SURGICAL HISTORY:
- Staging laparotomy with splenectomy (20 years ago)
- Radical retropubic prostatectomy with bilateral pelvic lymphadenectomy (3 years ago)
- Right hip arthroscopy (10 years ago)

MEDICATIONS:
Home Medications:
- Leuprolide 7.5 mg IM every 28 days
- Amlodipine 5 mg PO daily
- Atorvastatin 20 mg PO daily
- Calcium carbonate/Vitamin D3 600 mg/400 IU PO twice daily
- Aspirin 81 mg PO daily

ALLERGIES:
Penicillin - rash, Shellfish - anaphylaxis

FAMILY HISTORY:
- Father: Prostate cancer, diagnosed age 72, deceased
- Mother: Breast cancer, diagnosed age 68
- Maternal grandfather: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Logistics coordinator.

PHYSICAL EXAMINATION:
On physical examination, he has increased tone in the lower extremities, 3+ deep tendon reflexes in the patellar and Achilles tendons, and a positive Babinski sign on the left.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 7.8 x 10^3/uL
- Platelets: 250 x 10^3/uL
- Serum prostate-specific antigen (PSA): 9.8 ng/mL (elevated)
- Serum calcium: 9.6 mg/dL
- Alkaline phosphatase: 120 U/L (elevated)

IMAGING:
An urgent MRI of the spine reveals a mass compressing the thoracic spinal cord at the T8 level.

## Encounter 2: source 2022-07-15, practice 2026-04-14, ed, Emergency Medicine

Chief complaint: Sudden onset of severe lower back pain with bilateral lower extremity weakness and acute urinary retention

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe lower back pain with bilateral lower extremity weakness and acute urinary retention

HISTORY OF PRESENT ILLNESS:
Patient with prostate adenocarcinoma and prior cauda equina syndrome presents with similar complaints, reporting severe, constant pain unrelieved by over-the-counter analgesics.

PAST MEDICAL HISTORY:
Active Problem List:
- Constrictive pericarditis (diagnosed 2020-01-15)
- Cauda equina syndrome (diagnosed 2021-07-15)
- Hodgkin lymphoma, Stage II, in remission status post mantle field radiation therapy 20 years prior
- Prostate adenocarcinoma, Gleason 7 (3+4), status post radical prostatectomy and adjuvant external beam radiation therapy 3 years prior
- Hypertension
- Hyperlipidemia
- Osteopenia secondary to androgen deprivation therapy

PAST SURGICAL HISTORY:
- Staging laparotomy with splenectomy (20 years ago)
- Radical retropubic prostatectomy with bilateral pelvic lymphadenectomy (3 years ago)
- Right hip arthroscopy (10 years ago)

MEDICATIONS:
Home Medications:
- Leuprolide 7.5 mg IM every 28 days
- Amlodipine 5 mg PO daily
- Atorvastatin 20 mg PO daily
- Calcium carbonate/Vitamin D3 600 mg/400 IU PO twice daily
- Aspirin 81 mg PO daily

ALLERGIES:
Penicillin - rash, Shellfish - anaphylaxis

FAMILY HISTORY:
- Father: Prostate cancer, diagnosed age 72, deceased
- Mother: Breast cancer, diagnosed age 68
- Maternal grandfather: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Logistics coordinator.

PHYSICAL EXAMINATION:
On physical examination, the patient has decreased sensation to light touch and pinprick below the level of the umbilicus, 3/5 strength in both lower extremities, and diminished rectal tone.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 7.8 x 10^3/uL
- Platelets: 250 x 10^3/uL
- Serum creatinine: 1.0 mg/dL
- Blood urea nitrogen: 14 mg/dL
- Calcium: 9.8 mg/dL

IMAGING:
An urgent MRI of the spine reveals a mass compressing the thoracic spinal cord.
