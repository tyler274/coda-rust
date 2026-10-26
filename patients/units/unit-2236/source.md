# synthetic_hospital patient 2236

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-07-16 (Fri) | outpatient | Gastroenterology |
| 1 | 2020-05-15 | 2021-11-08 (Mon) | outpatient | Urology |
| 2 | 2020-09-15 | 2022-03-07 (Mon) | outpatient | Internal Medicine |
| 3 | 2021-07-15 | 2023-01-09 (Mon) | outpatient | Vascular Surgery |
| 4 | 2023-05-15 | 2024-11-18 (Mon) | ed | Emergency Medicine |
| 5 | 2023-11-15 | 2025-05-04 (Sun) | ed | Emergency Medicine |
| 6 | 2024-05-15 | 2025-11-19 (Wed) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2026-07-13 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired landscape architect",
 "alcohol_use": "Occasional social use, 1-2 glasses of wine weekly",
 "family_history": [
  "Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 68",
  "Mother: Colorectal cancer diagnosed age 72",
  "Maternal uncle: Deep vein thrombosis",
  "One sibling with coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 15 years ago, 25 pack-year history",
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
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "12.5 mg daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "Screening colonoscopy with polypectomy (recent)",
  "Incision and drainage of lower extremity abscess (10 years prior)"
 ],
 "chronic_conditions": [
  "Hypertension (longstanding)",
  "Type 2 diabetes mellitus",
  "Chronic kidney disease",
  "Obesity",
  "History of deep vein thrombosis (5 years prior)",
  "Post-thrombotic syndrome with chronic venous insufficiency",
  "Benign prostatic hyperplasia",
  "History of advanced colorectal adenoma (KRAS-mutated)"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Age-related sleep pattern changes (Advanced sleep-wake phase disorder)", "KRAS-mutated advanced colorectal adenoma", "Third-degree atrioventricular block", "Venous stasis ulcer", "Acute hypertensive heart failure", "Hemodynamically unstable wide-complex tachycardia", "Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall", "Recurrent benign prostatic hyperplasia"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-07-16, outpatient, Gastroenterology

Chief complaint: Follow-up visit after recent screening colonoscopy with polypectomy to discuss pathology results

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit after recent screening colonoscopy with polypectomy to discuss pathology results

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension (longstanding)
- Type 2 diabetes mellitus
- Chronic kidney disease
- Obesity
- History of deep vein thrombosis (5 years prior)
- Post-thrombotic syndrome with chronic venous insufficiency
- Benign prostatic hyperplasia
- History of advanced colorectal adenoma (KRAS-mutated)

The patient has a history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (recent)
- Incision and drainage of lower extremity abscess (10 years prior)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily
- Hydrochlorothiazide 12.5 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His family history is significant for colorectal cancer in his brother, who was diagnosed at the age of 55.

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 25 pack-year history. Alcohol: Occasional social use, 1-2 glasses of wine weekly. Occupation: Retired landscape architect.

REVIEW OF SYSTEMS:
He reports no abdominal pain, rectal bleeding, or changes in bowel habits.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
His laboratory values are as follows:

- Hemoglobin: 13.8 g/dL
- White blood cell count: 7.2 x 10^9/L
- Platelets: 250 x 10^9/L
- Fasting blood glucose: 146 mg/dL
- Hemoglobin A1c: 7.2%

PATHOLOGY:
Histopathological examination of the polyp revealed high-grade dysplasia.

OTHER STUDIES:
The colonoscopy report indicates the presence of a single 1.5 cm tubulovillous adenoma in the ascending colon, which was removed during the procedure.

## Encounter 1: source 2020-05-15, practice 2021-11-08, outpatient, Urology

Chief complaint: Difficulty initiating urination, weak urinary stream, and nocturia for the past 6 months without dysuria or hematuria

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Rodriguez, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty initiating urination, weak urinary stream, and nocturia for the past 6 months without dysuria or hematuria

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with hypertension, diabetes, CKD, and BPH presents to the urology clinic following gastroenterology follow-up for colorectal adenoma, complaining of difficulty initiating urination, a weak urinary stream, and nocturia for the past 6 months, denying any dysuria, hematuria, or history of urinary tract infections.

PAST MEDICAL HISTORY:
Active Problem List:
- KRAS-mutated advanced colorectal adenoma (diagnosed 2020-01-15)
- Hypertension (longstanding)
- Type 2 diabetes mellitus
- Chronic kidney disease
- Obesity
- History of deep vein thrombosis (5 years prior)
- Post-thrombotic syndrome with chronic venous insufficiency
- Benign prostatic hyperplasia

His past medical history is significant for well-controlled hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (recent)
- Incision and drainage of lower extremity abscess (10 years prior)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily
- Hydrochlorothiazide 12.5 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 68
- Mother: Colorectal cancer diagnosed age 72
- Maternal uncle: Deep vein thrombosis
- One sibling with coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 25 pack-year history. Alcohol: Occasional social use, 1-2 glasses of wine weekly. Occupation: Retired landscape architect.

VITAL SIGNS:
His blood pressure is 135/85 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
The abdominal exam reveals a non-tender, distended bladder. Digital rectal examination shows an enlarged, firm, non-nodular prostate.

LABORATORY DATA:
His serum creatinine is 1.1 mg/dL, blood urea nitrogen (BUN) is 18 mg/dL, and his prostate-specific antigen (PSA) level is 2.5 ng/mL. Urinalysis is unremarkable.

## Encounter 2: source 2020-09-15, practice 2022-03-07, outpatient, Internal Medicine

Chief complaint: Difficulty sleeping for 6 months with early evening somnolence around 8 PM and early morning awakening at 3 AM, plus daily afternoon naps

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty sleeping for 6 months with early evening somnolence around 8 PM and early morning awakening at 3 AM, plus daily afternoon naps

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with hypertension, diabetes mellitus, chronic kidney disease, and history of KRAS-mutated advanced colorectal adenoma presents with sleep disturbances. He reports that he often feels sleepy by 8 PM and wakes up at around 3 AM, unable to fall back asleep. He also mentions that he takes afternoon naps almost daily.

PAST MEDICAL HISTORY:
Active Problem List:
- KRAS-mutated advanced colorectal adenoma (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-05-15)
- Hypertension (longstanding)
- Type 2 diabetes mellitus
- Chronic kidney disease
- Obesity
- History of deep vein thrombosis (5 years prior)
- Post-thrombotic syndrome with chronic venous insufficiency
- Benign prostatic hyperplasia

His past medical history includes hypertension, for which he takes hydrochlorothiazide, and type 2 diabetes managed with metformin.

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (recent)
- Incision and drainage of lower extremity abscess (10 years prior)

MEDICATIONS:
He takes hydrochlorothiazide, metformin.

Home Medications:
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 68
- Mother: Colorectal cancer diagnosed age 72
- Maternal uncle: Deep vein thrombosis
- One sibling with coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 25 pack-year history. Alcohol: Occasional social use, 1-2 glasses of wine weekly. Occupation: Retired landscape architect.

REVIEW OF SYSTEMS:
He denies any symptoms of depression or anxiety.

VITAL SIGNS:
His vital signs are within normal limits

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin A1c: 6.8%
- Fasting blood glucose: 130 mg/dL
- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.0 mg/dL
- Thyroid-stimulating hormone (TSH): 2.5 μIU/mL

## Encounter 3: source 2021-07-15, practice 2023-01-09, outpatient, Vascular Surgery

Chief complaint: Non-healing wound on left lower leg present for 2 months with daily lower extremity swelling worse in evenings, no trauma or fever

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. David Kim, MD
Department: Vascular Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Non-healing wound on left lower leg present for 2 months with daily lower extremity swelling worse in evenings, no trauma or fever

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with longstanding post-thrombotic syndrome and chronic venous insufficiency who presents with a lower extremity wound. He first noticed the wound 2 months ago and says it started as an area of dry, itchy skin that later ulcerated. He reports mild swelling in his lower legs, worse at the end of the day. He denies trauma, fever, or significant pain.

PAST MEDICAL HISTORY:
Active Problem List:
- KRAS-mutated advanced colorectal adenoma (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-05-15)
- Age-related sleep pattern changes (Advanced sleep-wake phase disorder) (diagnosed 2020-09-15)
- Hypertension (longstanding)
- Type 2 diabetes mellitus
- Chronic kidney disease
- Obesity
- History of deep vein thrombosis (5 years prior)
- Post-thrombotic syndrome with chronic venous insufficiency
- Benign prostatic hyperplasia

His past medical history includes obesity, type 2 diabetes mellitus, hypertension, and a deep vein thrombosis 5 years ago.

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (recent)
- Incision and drainage of lower extremity abscess (10 years prior)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily
- Hydrochlorothiazide 12.5 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 68
- Mother: Colorectal cancer diagnosed age 72
- Maternal uncle: Deep vein thrombosis
- One sibling with coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 25 pack-year history. Alcohol: Occasional social use, 1-2 glasses of wine weekly. Occupation: Retired landscape architect.

VITAL SIGNS:
Vital signs are normal.

PHYSICAL EXAMINATION:
On physical exam, there is a 3-cm shallow ulcer over the medial aspect of the left ankle with an irregular border and a moist, granulating base. The surrounding skin is hyperpigmented and indurated, with mild pitting edema. Peripheral pulses are palpable bilaterally. There is no erythema, warmth, or purulent drainage.

## Encounter 4: source 2023-05-15, practice 2024-11-18, ed, Emergency Medicine

Chief complaint: Progressive shortness of breath and fatigue over 6 months with worsening leg swelling and frequent nighttime awakenings due to difficulty breathing

ENCOUNTER NOTE
Date: 2023-05-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath and fatigue over 6 months with worsening leg swelling and frequent nighttime awakenings due to difficulty breathing

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with longstanding hypertension, type 2 diabetes mellitus, chronic kidney disease, and post-thrombotic syndrome with chronic venous insufficiency previously complicated by venous stasis ulcer, who presents with progressive shortness of breath and fatigue. He reports a 6-month history of progressive shortness of breath and fatigue with inability to perform his usual daily activities, worsening swelling in his legs over the past month, and frequent nighttime awakenings due to difficulty breathing that improves when he sits up.

PAST MEDICAL HISTORY:
Active Problem List:
- KRAS-mutated advanced colorectal adenoma (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-05-15)
- Age-related sleep pattern changes (Advanced sleep-wake phase disorder) (diagnosed 2020-09-15)
- Venous stasis ulcer (diagnosed 2021-07-15)
- Hypertension (longstanding)
- Type 2 diabetes mellitus
- Chronic kidney disease
- Obesity
- History of deep vein thrombosis (5 years prior)
- Post-thrombotic syndrome with chronic venous insufficiency
- Benign prostatic hyperplasia

His past medical history is significant for longstanding hypertension, chronic kidney disease, and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (recent)
- Incision and drainage of lower extremity abscess (10 years prior)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily
- Hydrochlorothiazide 12.5 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 68
- Mother: Colorectal cancer diagnosed age 72
- Maternal uncle: Deep vein thrombosis
- One sibling with coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 25 pack-year history. Alcohol: Occasional social use, 1-2 glasses of wine weekly. Occupation: Retired landscape architect.

VITAL SIGNS:
BP is 155/90 mmHg, HR 90

PHYSICAL EXAMINATION:
Bibasilar crackles on lung auscultation. He has peripheral edema and JVD.

IMAGING:
A chest X-ray shows cardiomegaly and pulmonary vascular congestion. Transthoracic echo shows an LV ejection fraction of 32%.

## Encounter 5: source 2023-11-15, practice 2025-05-04, ed, Emergency Medicine

Chief complaint: Dizziness and episodes of syncope occurring over the past week without chest pain or palpitations

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. James Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Dizziness and episodes of syncope occurring over the past week without chest pain or palpitations

HISTORY OF PRESENT ILLNESS:
This patient with hypertension, diabetes, chronic kidney disease, and prior heart failure presents to the emergency department with complaints of dizziness and episodes of syncope over the past week. He denies chest pain, palpitations, or dyspnea.

PAST MEDICAL HISTORY:
Active Problem List:
- KRAS-mutated advanced colorectal adenoma (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-05-15)
- Age-related sleep pattern changes (Advanced sleep-wake phase disorder) (diagnosed 2020-09-15)
- Venous stasis ulcer (diagnosed 2021-07-15)
- Acute hypertensive heart failure (diagnosed 2023-05-15)
- Hypertension (longstanding)
- Type 2 diabetes mellitus
- Chronic kidney disease
- Obesity
- History of deep vein thrombosis (5 years prior)
- Post-thrombotic syndrome with chronic venous insufficiency
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (recent)
- Incision and drainage of lower extremity abscess (10 years prior)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Hydrochlorothiazide 12.5 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 68
- Mother: Colorectal cancer diagnosed age 72
- Maternal uncle: Deep vein thrombosis
- One sibling with coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 25 pack-year history. Alcohol: Occasional social use, 1-2 glasses of wine weekly. Occupation: Retired landscape architect.

VITAL SIGNS:
On examination, his blood pressure is 130/80 mmHg, heart rate is 38 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
The physical examination is notable for irregularly irregular heart sounds and no murmurs, rubs, or gallops.

LABORATORY DATA:
Laboratory values are as follows:

- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.1 mg/dL
- Glucose: 148 mg/dL
- Hemoglobin A1c: 7.2%
- Troponin I: <0.03 ng/mL

OTHER STUDIES:
An electrocardiogram (ECG) shows no P wave correlation with QRS complexes, which are wide and have a rate of 38 bpm.

## Encounter 6: source 2024-05-15, practice 2025-11-19, ed, Emergency Medicine

Chief complaint: Sudden-onset chest pain and confusion beginning 2 hours ago, witnessed by wife

ENCOUNTER NOTE
Date: 2024-05-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset chest pain and confusion beginning 2 hours ago, witnessed by wife

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with third-degree AV block (November 2023) and hypertensive heart failure is brought to the emergency department by his wife due to sudden-onset chest pain and confusion that started 2 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- KRAS-mutated advanced colorectal adenoma (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-05-15)
- Age-related sleep pattern changes (Advanced sleep-wake phase disorder) (diagnosed 2020-09-15)
- Venous stasis ulcer (diagnosed 2021-07-15)
- Acute hypertensive heart failure (diagnosed 2023-05-15)
- Third-degree atrioventricular block (diagnosed 2023-11-15)
- Hypertension (longstanding)
- Type 2 diabetes mellitus
- Chronic kidney disease
- Obesity
- History of deep vein thrombosis (5 years prior)
- Post-thrombotic syndrome with chronic venous insufficiency
- Benign prostatic hyperplasia

He has a history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (recent)
- Incision and drainage of lower extremity abscess (10 years prior)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily
- Hydrochlorothiazide 12.5 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 68
- Mother: Colorectal cancer diagnosed age 72
- Maternal uncle: Deep vein thrombosis
- One sibling with coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 25 pack-year history. Alcohol: Occasional social use, 1-2 glasses of wine weekly. Occupation: Retired landscape architect.

VITAL SIGNS:
His blood pressure is 82/58 mm Hg, pulse is 116/min, respiratory rate is 24/min, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
On arrival, he appears pale, diaphoretic, and disoriented. His skin is cool and clammy to the touch. Jugular venous pressure is elevated. Auscultation reveals distant heart sounds and bibasilar crackles.

OTHER STUDIES:
Electrocardiogram shows ST-segment elevations in the anterior leads.

## Encounter 7: source 2025-01-15, practice 2026-07-13, ed, Emergency Medicine

Chief complaint: Acute onset of palpitations and lightheadedness while gardening 30 minutes ago without chest pain or shortness of breath

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Singh, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of palpitations and lightheadedness while gardening 30 minutes ago without chest pain or shortness of breath

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with history of prior inferior STEMI, third-degree AV block, and hypertensive heart failure who presents with acute palpitations and lightheadedness that started while he was gardening 30 minutes ago. He denies chest pain, shortness of breath, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- KRAS-mutated advanced colorectal adenoma (diagnosed 2020-01-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-05-15)
- Age-related sleep pattern changes (Advanced sleep-wake phase disorder) (diagnosed 2020-09-15)
- Venous stasis ulcer (diagnosed 2021-07-15)
- Acute hypertensive heart failure (diagnosed 2023-05-15)
- Third-degree atrioventricular block (diagnosed 2023-11-15)
- Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall (diagnosed 2024-05-15)
- Hypertension (longstanding)
- Type 2 diabetes mellitus
- Chronic kidney disease
- Obesity
- History of deep vein thrombosis (5 years prior)
- Post-thrombotic syndrome with chronic venous insufficiency
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Screening colonoscopy with polypectomy (recent)
- Incision and drainage of lower extremity abscess (10 years prior)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Hydrochlorothiazide 12.5 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension, myocardial infarction at age 68
- Mother: Colorectal cancer diagnosed age 72
- Maternal uncle: Deep vein thrombosis
- One sibling with coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago, 25 pack-year history. Alcohol: Occasional social use, 1-2 glasses of wine weekly. Occupation: Retired landscape architect.

VITAL SIGNS:
On examination, his blood pressure is 90/60 mmHg, heart rate is 170 beats per minute, respiratory rate is 18 breaths per minute, and oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
His skin is cool and clammy. Cardiac auscultation reveals a rapid, regular rhythm without murmurs, rubs, or gallops. Lung fields are clear to auscultation.

LABORATORY DATA:
Intravenous access is established, and labs are drawn. The patient's bedside glucose is 180 mg/dL.

OTHER STUDIES:
An electrocardiogram (ECG) shows a wide QRS complex tachycardia without P waves.
