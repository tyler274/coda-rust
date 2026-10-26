# synthetic_hospital patient 2239

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 69 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-02-22 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2021-05-19 (Wed) | outpatient | Hematology & Oncology |
| 2 | 2021-09-15 | 2022-10-18 (Tue) | ed | Emergency Medicine |
| 3 | 2022-09-15 | 2023-10-30 (Mon) | inpatient | Urology |
| 4 | 2022-10-15 | 2023-12-03 (Sun) | inpatient | Infectious Diseases |
| 5 | 2023-10-15 | 2024-11-17 (Sun) | ed | Nephrology |
| 6 | 2023-11-15 | 2024-12-19 (Thu) | outpatient | Internal Medicine |
| 7 | 2024-11-15 | 2025-12-16 (Tue) | icu | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Allopurinol (acute interstitial nephritis)",
  "Shellfish"
 ],
 "insurance": "Medicare",
 "occupation": "Retired",
 "alcohol_use": "None",
 "family_history": [
  "Father: Hypertension, Paget disease of bone",
  "Mother: Osteoporosis",
  "Brother: Chronic lymphocytic leukemia",
  "Paternal grandfather: Cardiac disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "5 mg oral daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg oral daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "0.4 mg oral daily",
   "name": "Tamsulosin"
  },
  {
   "dose": "200 mg oral PRN pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Foley catheter placement (urinary)",
  "No major surgical procedures"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Benign prostatic hyperplasia",
  "Osteoarthritis",
  "Stage 3 chronic kidney disease",
  "Heart failure with reduced ejection fraction",
  "Chronic lymphocytic leukemia",
  "Paget disease of bone"
 ],
 "age_at_first_encounter": 69
}
```

Primary diagnoses: ["Obstructive Uropathy", "Cyanide toxicity", "Paget disease of bone", "Infective Endocarditis (Aortic Valve) with Complications", "Hyperkalemia", "Chronic lymphocytic leukemia", "Drug-induced acute interstitial nephritis (Allopurinol-induced)"]
Comorbidities: ["Lactic acidosis", "Recurrent benign prostatic hyperplasia", "Acute kidney injury", "Osteoarthritis", "Essential hypertension", "Hyperkalemia"]

## Encounter 0: source 2020-01-15, practice 2021-02-22, outpatient, Internal Medicine

Chief complaint: Routine health maintenance visit

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance visit

HISTORY OF PRESENT ILLNESS:
He reports no current complaints and is active in his retirement, enjoying golf and gardening.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Benign prostatic hyperplasia
- Osteoarthritis
- Stage 3 chronic kidney disease
- Heart failure with reduced ejection fraction
- Chronic lymphocytic leukemia
- Paget disease of bone

His medical history is significant for hypertension, controlled with hydrochlorothiazide, and benign prostatic hyperplasia.

PAST SURGICAL HISTORY:
- Foley catheter placement (urinary)
- No major surgical procedures

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Tamsulosin 0.4 mg oral daily
- Ibuprofen 200 mg oral PRN pain

ALLERGIES:
Allopurinol (acute interstitial nephritis), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Paget disease of bone
- Mother: Osteoporosis
- Brother: Chronic lymphocytic leukemia
- Paternal grandfather: Cardiac disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired.

VITAL SIGNS:
His vital signs are within normal limits

PHYSICAL EXAMINATION:
The examination is unremarkable

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 9.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum albumin: 4.2 g/dL (normal: 3.5-5.0 g/dL)
- Serum creatinine: 1.0 mg/dL (normal: 0.6-1.2 mg/dL)
- Alkaline phosphatase (ALP): 250 U/L (normal: 40-130 U/L)
- Aspartate aminotransferase (AST): 28 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 30 U/L (normal: 7-56 U/L)

ASSESSMENT AND PLAN:
Given the patient's age and laboratory findings,

## Encounter 1: source 2020-04-15, practice 2021-05-19, outpatient, Hematology & Oncology

Chief complaint: Three-month history of fatigue, unintentional weight loss, and generalized lymphadenopathy

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of fatigue, unintentional weight loss, and generalized lymphadenopathy

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with a history of Paget disease, hypertension, and heart failure who presents to the Hematology clinic three months after his January 2020 routine health maintenance visit. He denies fever, night sweats, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Hypertension
- Benign prostatic hyperplasia
- Osteoarthritis
- Stage 3 chronic kidney disease
- Heart failure with reduced ejection fraction
- Chronic lymphocytic leukemia

His past medical history is significant for hypertension, controlled with hydrochlorothiazide, and benign prostatic hyperplasia.

PAST SURGICAL HISTORY:
- Foley catheter placement (urinary)
- No major surgical procedures

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Tamsulosin 0.4 mg oral daily
- Ibuprofen 200 mg oral PRN pain

ALLERGIES:
Allopurinol (acute interstitial nephritis), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Paget disease of bone
- Mother: Osteoporosis
- Brother: Chronic lymphocytic leukemia
- Paternal grandfather: Cardiac disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired.

PHYSICAL EXAMINATION:
On physical examination, he is afebrile with stable vital signs. Examination reveals multiple non-tender, firm lymph nodes in the cervical and axillary regions, each approximately 2 cm in diameter. The spleen is palpable 3 cm below the left costal margin.

LABORATORY DATA:
A complete blood count with differential and peripheral blood smear is performed with the following results:

- White blood cell count: 22,000/µL (normal: 4,500-11,000/µL)
- Hemoglobin: 12.2 g/dL (normal: 13.8-17.2 g/dL)
- Platelet count: 150,000/µL (normal: 150,000-450,000/µL)
- Lymphocytes: 80% of white cells
- Smudge cells present on smear

## Encounter 2: source 2021-09-15, practice 2022-10-18, ed, Emergency Medicine

Chief complaint: Fatigue, decreased appetite, and progressive difficulty urinating with nocturnal incontinence

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fatigue, decreased appetite, and progressive difficulty urinating with nocturnal incontinence

HISTORY OF PRESENT ILLNESS:
This 70-year-old with chronic lymphocytic leukemia diagnosed April 2020 and benign prostatic hyperplasia presents to the emergency department with fatigue, decreased appetite, and progressive difficulty urinating over the past several days. He reports nocturnal incontinence and lower abdominal discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Chronic lymphocytic leukemia (diagnosed 2020-04-15)
- Hypertension
- Benign prostatic hyperplasia
- Osteoarthritis
- Stage 3 chronic kidney disease
- Heart failure with reduced ejection fraction

His medical history includes hypertension and osteoarthritis.

PAST SURGICAL HISTORY:
- Foley catheter placement (urinary)
- No major surgical procedures

MEDICATIONS:
He takes for which he takes amlodipine and over-the-counter ibuprofen.

Home Medications:
- Hydrochlorothiazide 25 mg oral daily
- Tamsulosin 0.4 mg oral daily

ALLERGIES:
Allopurinol (acute interstitial nephritis), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Paget disease of bone
- Mother: Osteoporosis
- Brother: Chronic lymphocytic leukemia
- Paternal grandfather: Cardiac disease

SOCIAL HISTORY:
He does not use tobacco or alcohol.

VITAL SIGNS:
On physical examination, his blood pressure is 146/86 mm Hg, and pulse is 82/min

PHYSICAL EXAMINATION:
The lower abdomen is distended and tender. Digital rectal examination reveals a symmetrically enlarged, smooth prostate.

LABORATORY DATA:
His serum creatinine is 2.4 mg/dL, blood urea nitrogen is 45 mg/dL, and urinalysis shows mild proteinuria without hematuria, pyuria, or casts.

IMAGING:
Bladder scan reveals >600 mL of retained urine.

## Encounter 3: source 2022-09-15, practice 2023-10-30, inpatient, Urology

Chief complaint: Progressively worsening urinary issues with acute urinary retention and bilateral lower extremity swelling

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Priya Patel, MD
Department: Urology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressively worsening urinary issues with acute urinary retention and bilateral lower extremity swelling

HISTORY OF PRESENT ILLNESS:
The patient is a 71-year-old male with benign prostatic hyperplasia who returns with recurrent urinary symptoms one year after a prior similar episode. For the past year, he has experienced difficulty initiating urination, a weak urinary stream, nocturia, and recent bilateral lower extremity swelling. Over the past 24 hours, he developed increasing suprapubic discomfort and has been unable to urinate despite repeated attempts.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Chronic lymphocytic leukemia (diagnosed 2020-04-15)
- Obstructive Uropathy (diagnosed 2021-09-15)
- Hypertension
- Benign prostatic hyperplasia
- Osteoarthritis
- Stage 3 chronic kidney disease
- Heart failure with reduced ejection fraction

PAST SURGICAL HISTORY:
- Foley catheter placement (urinary)
- No major surgical procedures

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Tamsulosin 0.4 mg oral daily
- Ibuprofen 200 mg oral PRN pain

ALLERGIES:
Allopurinol (acute interstitial nephritis), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Paget disease of bone
- Mother: Osteoporosis
- Brother: Chronic lymphocytic leukemia
- Paternal grandfather: Cardiac disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired.

VITAL SIGNS:
Vital signs reveal temperature 37.4°C (99.3°F), BP 162/92 mm Hg, pulse 82/min, respirations 14/min

PHYSICAL EXAMINATION:
On examination, he has suprapubic tenderness and 2+ bilateral lower extremity pitting edema. Bladder percussion is dull to the level of the umbilicus.

LABORATORY DATA:
Lab studies show: Sodium: 142 mEq/L Potassium: 5.6 mEq/L Bicarbonate: 19 mEq/L BUN: 61 mg/dL Creatinine: 3.7 mg/dL

## Encounter 4: source 2022-10-15, practice 2023-12-03, inpatient, Infectious Diseases

Chief complaint: Fever, chills, and dysuria two weeks following hospitalization

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. David Kim, MD
Department: Infectious Diseases
Visit Type: inpatient

CHIEF COMPLAINT:
Fever, chills, and dysuria two weeks following hospitalization

HISTORY OF PRESENT ILLNESS:
The patient is a 71-year-old male with chronic lymphocytic leukemia, Paget disease, and benign prostatic hyperplasia recently hospitalized for obstructive uropathy who presents to the emergency department with a 3-day history of fever, chills, and dysuria. He reports hospitalization two weeks ago for acute urinary retention with urinary catheter placement and subsequent removal prior to discharge.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Chronic lymphocytic leukemia (diagnosed 2020-04-15)
- Obstructive Uropathy (diagnosed 2021-09-15)
- Hypertension
- Benign prostatic hyperplasia
- Osteoarthritis
- Stage 3 chronic kidney disease
- Heart failure with reduced ejection fraction

PAST SURGICAL HISTORY:
- Foley catheter placement (urinary)
- No major surgical procedures

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Tamsulosin 0.4 mg oral daily
- Ibuprofen 200 mg oral PRN pain

ALLERGIES:
Allopurinol (acute interstitial nephritis), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Paget disease of bone
- Mother: Osteoporosis
- Brother: Chronic lymphocytic leukemia
- Paternal grandfather: Cardiac disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired.

VITAL SIGNS:
His temperature is 38.9°C (102°F), blood pressure is 130/80 mmHg, pulse is 110/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
On physical examination, a grade 2/6 systolic murmur is noted at the right upper sternal border that was not documented in previous medical records. His abdomen is soft, non-tender, and a slightly enlarged, firm prostate is palpated on digital rectal examination.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 14,000/uL
- Hemoglobin: 13.2 g/dL
- Creatinine: 1.4 mg/dL (baseline 1.0 mg/dL)
- Urinalysis: Pyuria, positive leukocyte esterase, and positive nitrite
- Blood cultures: Pending

IMAGING:
An echocardiogram is ordered.

## Encounter 5: source 2023-10-15, practice 2024-11-17, ed, Nephrology

Chief complaint: Generalized weakness and reduced urine output following recent hospitalization

ENCOUNTER NOTE
Date: 2023-10-15
Provider: Dr. Emily Johnson, MD
Department: Nephrology
Visit Type: ed

CHIEF COMPLAINT:
Generalized weakness and reduced urine output following recent hospitalization

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with Stage 3 chronic kidney disease, heart failure with reduced ejection fraction, chronic lymphocytic leukemia, and prior obstructive uropathy, who presents to the emergency department with generalized weakness and reduced urine output over the past 3 days. He was discharged 1 week ago following hospitalization for decompensated heart failure. At discharge, he was prescribed lisinopril, carvedilol, furosemide, and ketorolac for acute musculoskeletal back pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Chronic lymphocytic leukemia (diagnosed 2020-04-15)
- Obstructive Uropathy (diagnosed 2021-09-15)
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2022-10-15)
- Hypertension
- Benign prostatic hyperplasia
- Osteoarthritis
- Stage 3 chronic kidney disease
- Heart failure with reduced ejection fraction

He has a history of hypertension, heart failure with reduced ejection fraction, and osteoarthritis.

PAST SURGICAL HISTORY:
- Foley catheter placement (urinary)
- No major surgical procedures

MEDICATIONS:
He takes lisinopril, carvedilol, furosemide, and ketorolac.

Home Medications:
- Amlodipine 5 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Tamsulosin 0.4 mg oral daily
- Ibuprofen 200 mg oral PRN pain

ALLERGIES:
Allopurinol (acute interstitial nephritis), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Paget disease of bone
- Mother: Osteoporosis
- Brother: Chronic lymphocytic leukemia
- Paternal grandfather: Cardiac disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired.

REVIEW OF SYSTEMS:
He has no fever, abdominal pain, or dysuria.

VITAL SIGNS:
His temperature is 36.7°C (98°F), blood pressure is 102/66 mm Hg, and heart rate is 58/min

PHYSICAL EXAMINATION:
He has jugular venous distension to the angle of the jaw, bibasilar crackles, and trace pedal edema.

LABORATORY DATA:
Potassium                                 5.6 mEq/L
BUN                                          55 mg/dL
Creatinine                               2.4 mg/dL
Urinalysis                                Unremarkable

## Encounter 6: source 2023-11-15, practice 2024-12-19, outpatient, Internal Medicine

Chief complaint: Mild breast tenderness noted two weeks after recent encounter

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Mild breast tenderness noted two weeks after recent encounter

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with chronic lymphocytic leukemia, heart failure with reduced ejection fraction, stage 3 chronic kidney disease, and recent allopurinol-induced acute interstitial nephritis who returns for follow-up two weeks following his emergency department visit. He reports mild breast tenderness but no other complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Chronic lymphocytic leukemia (diagnosed 2020-04-15)
- Obstructive Uropathy (diagnosed 2021-09-15)
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2022-10-15)
- Drug-induced acute interstitial nephritis (Allopurinol-induced) (diagnosed 2023-10-15)
- Hypertension
- Benign prostatic hyperplasia
- Osteoarthritis
- Stage 3 chronic kidney disease
- Heart failure with reduced ejection fraction

PAST SURGICAL HISTORY:
- Foley catheter placement (urinary)
- No major surgical procedures

MEDICATIONS:
He takes started on spironolactone as part of guideline-directed medical therapy.

Home Medications:
- Amlodipine 5 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Tamsulosin 0.4 mg oral daily
- Ibuprofen 200 mg oral PRN pain

ALLERGIES:
Allopurinol (acute interstitial nephritis), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Paget disease of bone
- Mother: Osteoporosis
- Brother: Chronic lymphocytic leukemia
- Paternal grandfather: Cardiac disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired.

VITAL SIGNS:
Vital signs are stable

PHYSICAL EXAMINATION:
Physical exam shows trace bilateral gynecomastia

LABORATORY DATA:
Basic metabolic panel shows potassium of 5.4 mEq/L (previously 4.5 mEq/L), sodium 138 mEq/L, and creatinine 1.2 mg/dL

ASSESSMENT AND PLAN:
His physician anticipates compensatory changes in the renin-angiotensin-aldosterone system (RAAS) due to spironolactone's mechanism of action.

## Encounter 7: source 2024-11-15, practice 2025-12-16, icu, Cardiology

Chief complaint: Agitation and confusion developing during management of hypertensive emergency

ENCOUNTER NOTE
Date: 2024-11-15
Provider: Dr. Robert Smith, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Agitation and confusion developing during management of hypertensive emergency

HISTORY OF PRESENT ILLNESS:
This 73-year-old male with heart failure with reduced ejection fraction, stage 3 CKD, and history of hyperkalemia and nephritis is admitted to the intensive care unit for management of a hypertensive emergency. He is started on an intravenous infusion of nitroprusside. After 72 hours of treatment, the nursing staff reports that the patient has become increasingly agitated and confused.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Chronic lymphocytic leukemia (diagnosed 2020-04-15)
- Obstructive Uropathy (diagnosed 2021-09-15)
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2022-10-15)
- Drug-induced acute interstitial nephritis (Allopurinol-induced) (diagnosed 2023-10-15)
- Hyperkalemia (diagnosed 2023-11-15)
- Hypertension
- Benign prostatic hyperplasia
- Osteoarthritis
- Stage 3 chronic kidney disease
- Heart failure with reduced ejection fraction

PAST SURGICAL HISTORY:
- Foley catheter placement (urinary)
- No major surgical procedures

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg oral daily
- Hydrochlorothiazide 25 mg oral daily
- Tamsulosin 0.4 mg oral daily
- Ibuprofen 200 mg oral PRN pain

ALLERGIES:
Allopurinol (acute interstitial nephritis), Shellfish

FAMILY HISTORY:
- Father: Hypertension, Paget disease of bone
- Mother: Osteoporosis
- Brother: Chronic lymphocytic leukemia
- Paternal grandfather: Cardiac disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired.

VITAL SIGNS:
His blood pressure is now 130/85 mmHg. His respiratory rate is 22 breaths per minute, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
On examination, the patient is disoriented to time and place, and his speech is slurred. He has no focal neurological deficits.

LABORATORY DATA:
Laboratory results are as follows:

- Serum lactate: 5 mmol/L (normal: 0.5-2.2 mmol/L)
- Arterial blood pH: 7.32 (normal: 7.35-7.45)
- Serum bicarbonate: 18 mEq/L (normal: 22-29 mEq/L)
- Blood urea nitrogen: 40 mg/dL (normal: 7-20 mg/dL)
- Serum creatinine: 2.5 mg/dL (baseline: 1.8 mg/dL)
