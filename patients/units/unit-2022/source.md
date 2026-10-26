# synthetic_hospital patient 2022

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Medicare (disability).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-09-30 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2023-01-02 (Mon) | outpatient | Cardiology |
| 2 | 2021-03-15 | 2023-11-24 (Fri) | outpatient | Internal Medicine |
| 3 | 2021-11-15 | 2024-07-20 (Sat) | ed | Emergency Medicine |
| 4 | 2022-05-15 | 2025-01-23 (Thu) | ed | Emergency Medicine |
| 5 | 2022-11-15 | 2025-07-31 (Thu) | ed | Emergency Medicine |
| 6 | 2023-05-15 | 2026-02-03 (Tue) | ed | Emergency Medicine |
| 7 | 2023-11-15 | 2026-07-23 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare (disability)",
 "occupation": "Former truck driver, currently disabled",
 "alcohol_use": "Chronic heavy use, 6-8 standard drinks daily for 25 years",
 "family_history": [
  "Father: Myocardial infarction at age 62, hypertension",
  "Mother: Type 2 diabetes mellitus, hyperlipidemia",
  "One brother with history of kidney stones"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-year history, quit 1 year ago",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Spironolactone"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Furosemide"
  },
  {
   "dose": "40 mg PO nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  }
 ],
 "surgical_history": [
  "Open appendectomy at age 30",
  "Multiple episodes of nephrolithiasis managed conservatively",
  "Left inguinal hernia repair"
 ],
 "chronic_conditions": [
  "Severe hypertension",
  "Heart failure with reduced ejection fraction (HFrEF)",
  "Alcohol-related cirrhosis",
  "Type 2 diabetes mellitus",
  "Hypercholesterolemia",
  "Chronic kidney disease Stage 3a",
  "History of recurrent nephrolithiasis",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Acute kidney injury", "Ureterolithiasis", "Secondary hypertension due to renal artery stenosis", "Hypokalemia managed with oral replacement (tolerating PO intake)", "Adhesive small bowel obstruction", "Fournier gangrene", "Spontaneous bacterial peritonitis", "Pseudogynecomastia"]
Comorbidities: ["Acute kidney injury", "Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia", "Renal artery stenosis", "Alcoholic cirrhosis", "Essential hypertension", "Ascites"]

## Encounter 0: source 2020-01-15, practice 2022-09-30, outpatient, Internal Medicine

Chief complaint: Follow-up visit one month after initiation of lisinopril for severe hypertension, now with acute kidney injury

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit one month after initiation of lisinopril for severe hypertension, now with acute kidney injury

HISTORY OF PRESENT ILLNESS:
One month ago, his blood pressure was 196/114 mm Hg during an annual physical, and he was started on lisinopril. At that time, his serum creatinine was 0.9 mg/dL, and serum electrolytes were normal. Today, he feels well and has no complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe hypertension
- Heart failure with reduced ejection fraction (HFrEF)
- Alcohol-related cirrhosis
- Type 2 diabetes mellitus
- Hypercholesterolemia
- Chronic kidney disease Stage 3a
- History of recurrent nephrolithiasis
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Open appendectomy at age 30
- Multiple episodes of nephrolithiasis managed conservatively
- Left inguinal hernia repair

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Spironolactone 25 mg PO daily
- Furosemide 40 mg PO daily
- Atorvastatin 40 mg PO nightly
- Insulin glargine 24 units subcutaneous at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- One brother with history of kidney stones

SOCIAL HISTORY:
He does not use NSAIDs or herbal supplements.

VITAL SIGNS:
His blood pressure is 172/92 mm Hg.

PHYSICAL EXAMINATION:
Physical examination shows no bruits, edema, or rash.

LABORATORY DATA:
Repeat labs reveal a serum creatinine of 2.6 mg/dL and no proteinuria or hematuria.

## Encounter 1: source 2020-04-15, practice 2023-01-02, outpatient, Cardiology

Chief complaint: Difficult-to-control blood pressure despite triple therapy with rising creatinine

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficult-to-control blood pressure despite triple therapy with rising creatinine

HISTORY OF PRESENT ILLNESS:
This 55-year-old male with severe hypertension and CKD Stage 3a returns three months after his Internal Medicine visit with acute kidney injury after lisinopril initiation. Despite adherence to hydrochlorothiazide, amlodipine, and metoprolol, his blood pressure remains elevated at 162/98 mmHg. He reports no symptoms of headache, vision changes, or chest pain. Laboratory tests show a serum creatinine increase from 1.0 mg/dL to 1.4 mg/dL since that visit when lisinopril was started.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Severe hypertension
- Heart failure with reduced ejection fraction (HFrEF)
- Alcohol-related cirrhosis
- Type 2 diabetes mellitus
- Hypercholesterolemia
- Chronic kidney disease Stage 3a
- History of recurrent nephrolithiasis
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Open appendectomy at age 30
- Multiple episodes of nephrolithiasis managed conservatively
- Left inguinal hernia repair

MEDICATIONS:
His current medications include lisinopril, hydrochlorothiazide, amlodipine, metoprolol, and atorvastatin.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Spironolactone 25 mg PO daily
- Furosemide 40 mg PO daily
- Insulin glargine 24 units subcutaneous at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- One brother with history of kidney stones

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 1 year ago. Alcohol: Chronic heavy use, 6-8 standard drinks daily for 25 years. Occupation: Former truck driver, currently disabled.

PHYSICAL EXAMINATION:
Physical examination reveals a continuous, high-pitched epigastric bruit.

LABORATORY DATA:
Laboratory Values:
- Serum Creatinine: 1.4 mg/dL (baseline: 1.0 mg/dL)
- Serum Potassium: 4.5 mEq/L
- Serum Sodium: 140 mEq/L
- Total Cholesterol: 190 mg/dL
- LDL Cholesterol: 120 mg/dL
- HDL Cholesterol: 45 mg/dL

IMAGING:
His kidney ultrasound shows the right kidney measuring 9 cm in length and the left kidney measuring 11 cm.

## Encounter 2: source 2021-03-15, practice 2023-11-24, outpatient, Internal Medicine

Chief complaint: Bilateral breast tenderness and enlargement two months after starting spironolactone

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Aisha Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Bilateral breast tenderness and enlargement two months after starting spironolactone

HISTORY OF PRESENT ILLNESS:
The patient with severe hypertension complicated by renal artery stenosis and prior medication-induced AKI returns with breast symptoms following recent medication adjustment. Two months ago, his cardiologist started him on an additional medication to improve blood pressure control and reduce symptoms of fluid overload. Since then, he has developed breast tenderness and enlargement that is causing him embarrassment. He denies nipple discharge, skin changes, or pain elsewhere.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Secondary hypertension due to renal artery stenosis (diagnosed 2020-04-15)
- Severe hypertension
- Heart failure with reduced ejection fraction (HFrEF)
- Alcohol-related cirrhosis
- Type 2 diabetes mellitus
- Hypercholesterolemia
- Chronic kidney disease Stage 3a
- History of recurrent nephrolithiasis
- Tobacco use disorder

He has a history of hypertension and heart failure with reduced ejection fraction (EF 35%).

PAST SURGICAL HISTORY:
- Open appendectomy at age 30
- Multiple episodes of nephrolithiasis managed conservatively
- Left inguinal hernia repair

MEDICATIONS:
His current medications include lisinopril and metoprolol.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Spironolactone 25 mg PO daily
- Furosemide 40 mg PO daily
- Atorvastatin 40 mg PO nightly
- Insulin glargine 24 units subcutaneous at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- One brother with history of kidney stones

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 1 year ago. Alcohol: Chronic heavy use, 6-8 standard drinks daily for 25 years. Occupation: Former truck driver, currently disabled.

VITAL SIGNS:
Blood pressure today is 126/74 mmHg, and pulse is 68/min.

PHYSICAL EXAMINATION:
Cardiopulmonary examination is stable, and lower extremity edema has improved. Testicular examination is normal.

## Encounter 3: source 2021-11-15, practice 2024-07-20, ed, Emergency Medicine

Chief complaint: Increasing abdominal girth, shortness of breath, and hypokalemia while on diuretic therapy

ENCOUNTER NOTE
Date: 2021-11-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing abdominal girth, shortness of breath, and hypokalemia while on diuretic therapy

HISTORY OF PRESENT ILLNESS:
This 56-year-old male with heart failure with reduced ejection fraction, previously treated for spironolactone-induced pseudogynecomastia, presents with diuretic complications. He reports compliance with his prescribed spironolactone but admits to occasionally missing doses of his furosemide due to frequent urination.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Secondary hypertension due to renal artery stenosis (diagnosed 2020-04-15)
- Pseudogynecomastia (diagnosed 2021-03-15)
- Severe hypertension
- Heart failure with reduced ejection fraction (HFrEF)
- Alcohol-related cirrhosis
- Type 2 diabetes mellitus
- Hypercholesterolemia
- Chronic kidney disease Stage 3a
- History of recurrent nephrolithiasis
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Open appendectomy at age 30
- Multiple episodes of nephrolithiasis managed conservatively
- Left inguinal hernia repair

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Spironolactone 25 mg PO daily
- Furosemide 40 mg PO daily
- Atorvastatin 40 mg PO nightly
- Insulin glargine 24 units subcutaneous at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- One brother with history of kidney stones

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 1 year ago. Alcohol: Chronic heavy use, 6-8 standard drinks daily for 25 years. Occupation: Former truck driver, currently disabled.

VITAL SIGNS:
His blood pressure is 130/85 mmHg, heart rate is 88 bpm, and respiratory rate is 20 breaths per minute

PHYSICAL EXAMINATION:
On examination, he has a distended abdomen with a fluid wave and bilateral lower extremity pitting edema.

LABORATORY DATA:
Laboratory results are as follows:

- Serum sodium: 136 mEq/L
- Serum potassium: 3.2 mEq/L
- Serum chloride: 95 mEq/L
- Serum bicarbonate: 32 mEq/L
- Blood urea nitrogen (BUN): 28 mg/dL
- Serum creatinine: 1.4 mg/dL
- Urine sodium: 10 mEq/L
- Urine potassium: 25 mEq/L

## Encounter 4: source 2022-05-15, practice 2025-01-23, ed, Emergency Medicine

Chief complaint: Severe left flank pain radiating to groin with nausea and urinary frequency

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Priya Kumar, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe left flank pain radiating to groin with nausea and urinary frequency

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with recurrent nephrolithiasis and CKD Stage 3a, most recently seen in November 2021 for hypokalemia, presents with flank pain. The pain is constant, radiates to the groin, and is associated with nausea. He reports a recent increase in the frequency of urination, often feeling that he cannot completely empty his bladder. He also mentions having had a few episodes of nocturia with large urine volumes.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Secondary hypertension due to renal artery stenosis (diagnosed 2020-04-15)
- Pseudogynecomastia (diagnosed 2021-03-15)
- Hypokalemia managed with oral replacement (tolerating PO intake) (diagnosed 2021-11-15)
- Severe hypertension
- Heart failure with reduced ejection fraction (HFrEF)
- Alcohol-related cirrhosis
- Type 2 diabetes mellitus
- Hypercholesterolemia
- Chronic kidney disease Stage 3a
- History of recurrent nephrolithiasis
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Open appendectomy at age 30
- Multiple episodes of nephrolithiasis managed conservatively
- Left inguinal hernia repair

MEDICATIONS:
Hydrochlorothiazide

Home Medications:
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Amlodipine 5 mg PO daily
- Spironolactone 25 mg PO daily
- Furosemide 40 mg PO daily
- Atorvastatin 40 mg PO nightly
- Insulin glargine 24 units subcutaneous at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- One brother with history of kidney stones

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 1 year ago. Alcohol: Chronic heavy use, 6-8 standard drinks daily for 25 years. Occupation: Former truck driver, currently disabled.

VITAL SIGNS:
Afebrile, blood pressure is 145/90 mmHg

PHYSICAL EXAMINATION:
Costovertebral angle tenderness on the left side

LABORATORY DATA:
Serum creatinine: 2.1 mg/dL (elevated)
Blood urea nitrogen (BUN): 36 mg/dL (elevated)
Urinalysis: Hematuria, no pyuria or bacteriuria

IMAGING:
A non-contrast CT scan of the abdomen and pelvis shows a 7 mm stone at the ureteropelvic junction with moderate hydronephrosis of the left kidney.

## Encounter 5: source 2022-11-15, practice 2025-07-31, ed, Emergency Medicine

Chief complaint: Fever, increasing abdominal girth, and acute confusion in patient with known cirrhosis

ENCOUNTER NOTE
Date: 2022-11-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, increasing abdominal girth, and acute confusion in patient with known cirrhosis

HISTORY OF PRESENT ILLNESS:
The patient with known alcohol-related cirrhosis and recurrent nephrolithiasis returns with recurrent abdominal distension, having presented similarly in November 2021. He presents to the emergency department with a two-day history of increasing abdominal girth and discomfort. He also reports feeling feverish and has experienced a recent onset of confusion. His wife notes that he has been less responsive and has not been eating well.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Secondary hypertension due to renal artery stenosis (diagnosed 2020-04-15)
- Pseudogynecomastia (diagnosed 2021-03-15)
- Hypokalemia managed with oral replacement (tolerating PO intake) (diagnosed 2021-11-15)
- Ureterolithiasis (diagnosed 2022-05-15)
- Severe hypertension
- Heart failure with reduced ejection fraction (HFrEF)
- Alcohol-related cirrhosis
- Type 2 diabetes mellitus
- Hypercholesterolemia
- Chronic kidney disease Stage 3a
- History of recurrent nephrolithiasis
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Open appendectomy at age 30
- Multiple episodes of nephrolithiasis managed conservatively
- Left inguinal hernia repair

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Spironolactone 25 mg PO daily
- Furosemide 40 mg PO daily
- Atorvastatin 40 mg PO nightly
- Insulin glargine 24 units subcutaneous at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- One brother with history of kidney stones

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 1 year ago. Alcohol: Chronic heavy use, 6-8 standard drinks daily for 25 years. Occupation: Former truck driver, currently disabled.

VITAL SIGNS:
On examination, his temperature is 38.2°C (100.8°F), blood pressure is 100/60 mmHg, and heart rate is 102 beats per minute

PHYSICAL EXAMINATION:
Abdominal examination reveals a fluid wave and shifting dullness. His skin shows spider angiomas and palmar erythema. Neurological examination is significant for asterixis.

LABORATORY DATA:
Laboratory studies are as follows:

- White blood cell count: 12,000/mm3
- Hemoglobin: 10.2 g/dL
- Platelet count: 85,000/mm3
- Serum creatinine: 1.5 mg/dL
- Serum albumin: 2.8 g/dL
- INR: 1.6

OTHER STUDIES:
An urgent diagnostic paracentesis is performed, and the ascitic fluid analysis reveals:

- Ascitic fluid protein: 1.0 g/dL
- Ascitic fluid albumin: 0.8 g/dL
- Ascitic fluid neutrophil count: 500/mm3
- Ascitic fluid culture: Pending

## Encounter 6: source 2023-05-15, practice 2026-02-03, ed, Emergency Medicine

Chief complaint: Severe scrotal and perineal pain with swelling, fever, and chills

ENCOUNTER NOTE
Date: 2023-05-15
Provider: Dr. Maria Santos, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe scrotal and perineal pain with swelling, fever, and chills

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with alcohol-related cirrhosis, severe hypertension, and prior spontaneous bacterial peritonitis six months ago presents to the emergency department with a 2-day history of severe pain and swelling in the scrotal and perineal area. He reports a high fever and chills that started suddenly the previous night.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Secondary hypertension due to renal artery stenosis (diagnosed 2020-04-15)
- Pseudogynecomastia (diagnosed 2021-03-15)
- Hypokalemia managed with oral replacement (tolerating PO intake) (diagnosed 2021-11-15)
- Ureterolithiasis (diagnosed 2022-05-15)
- Spontaneous bacterial peritonitis (diagnosed 2022-11-15)
- Severe hypertension
- Heart failure with reduced ejection fraction (HFrEF)
- Alcohol-related cirrhosis
- Type 2 diabetes mellitus
- Hypercholesterolemia
- Chronic kidney disease Stage 3a
- History of recurrent nephrolithiasis
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Open appendectomy at age 30
- Multiple episodes of nephrolithiasis managed conservatively
- Left inguinal hernia repair

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Spironolactone 25 mg PO daily
- Furosemide 40 mg PO daily
- Atorvastatin 40 mg PO nightly
- Insulin glargine 24 units subcutaneous at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- One brother with history of kidney stones

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 1 year ago. Alcohol: Chronic heavy use, 6-8 standard drinks daily for 25 years. Occupation: Former truck driver, currently disabled.

VITAL SIGNS:
On examination, the patient is tachycardic with a heart rate of 110 bpm, hypotensive with a blood pressure of 90/60 mmHg, and febrile with a temperature of 39.5°C (103.1°F).

PHYSICAL EXAMINATION:
The scrotal and perineal skin is erythematous, edematous, and there is a foul-smelling discharge. Crepitus is palpable in the same area.

LABORATORY DATA:
White blood cell count: 22,000 /μL
- Hemoglobin: 12.5 g/dL
- Platelet count: 150,000 /μL
- Blood urea nitrogen: 30 mg/dL
- Creatinine: 2.1 mg/dL
- C-reactive protein: 250 mg/L
- Blood cultures: Pending

## Encounter 7: source 2023-11-15, practice 2026-07-23, ed, Emergency Medicine

Chief complaint: Worsening diffuse abdominal pain, vomiting, and obstipation for 18 hours

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. Robert Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening diffuse abdominal pain, vomiting, and obstipation for 18 hours

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with cirrhosis and prior SBP reports progressive bloating, minimal oral intake, and no passage of gas or stool since last night.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Secondary hypertension due to renal artery stenosis (diagnosed 2020-04-15)
- Pseudogynecomastia (diagnosed 2021-03-15)
- Hypokalemia managed with oral replacement (tolerating PO intake) (diagnosed 2021-11-15)
- Ureterolithiasis (diagnosed 2022-05-15)
- Spontaneous bacterial peritonitis (diagnosed 2022-11-15)
- Fournier gangrene (diagnosed 2023-05-15)
- Severe hypertension
- Heart failure with reduced ejection fraction (HFrEF)
- Alcohol-related cirrhosis
- Type 2 diabetes mellitus
- Hypercholesterolemia
- Chronic kidney disease Stage 3a
- History of recurrent nephrolithiasis
- Tobacco use disorder

PAST SURGICAL HISTORY:
His history includes an open appendectomy 8 years ago

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Spironolactone 25 mg PO daily
- Furosemide 40 mg PO daily
- Atorvastatin 40 mg PO nightly
- Insulin glargine 24 units subcutaneous at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- One brother with history of kidney stones

SOCIAL HISTORY:
Drinks fluids inconsistently

REVIEW OF SYSTEMS:
He denies fever, hematemesis, melena, chest pain, or urinary symptoms.

VITAL SIGNS:
Temperature is 37.2 °C (99.0 °F), blood pressure 102/66 mm Hg, pulse 112/min, and respirations 20/min

PHYSICAL EXAMINATION:
On arrival he appears uncomfortable and mildly dehydrated. The abdomen is distended and tympanic with diffuse mild tenderness; high-pitched rushes are heard intermittently followed by quiet intervals. No groin or incisional bulges are palpable, and rectal exam shows an empty vault.

LABORATORY DATA:
Labs show:
Sodium                            134 mEq/L
Potassium                         3.1 mEq/L
Chloride                          90 mEq/L
Bicarbonate                       33 mEq/L
Blood urea nitrogen           38 mg/dL
Creatinine                        1.6 mg/dL

IMAGING:
A plain upright abdominal film is obtained. Imaging shows multiple stacked air–fluid levels in centrally located dilated loops with little distal gas.

PLAN:
Intravenous fluids are begun
