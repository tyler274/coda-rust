# synthetic_hospital patient 2705

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex F, Non-Hispanic White, Medicare (ESRD beneficiary).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-07-19 (Mon) | outpatient | Nephrology |
| 1 | 2021-01-15 | 2022-07-18 (Mon) | outpatient | Nephrology |
| 2 | 2021-02-15 | 2022-08-18 (Thu) | ed | Emergency Medicine |
| 3 | 2022-01-15 | 2023-07-17 (Mon) | outpatient | General Surgery |
| 4 | 2023-01-15 | 2024-07-22 (Mon) | outpatient | Rheumatology |
| 5 | 2024-01-15 | 2025-07-14 (Mon) | outpatient | Gastroenterology |
| 6 | 2024-07-15 | 2026-01-19 (Mon) | outpatient | Internal Medicine |
| 7 | 2025-01-15 | 2026-07-15 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfa drugs (anaphylaxis)",
  "Contrast dye (hives)"
 ],
 "insurance": "Medicare (ESRD beneficiary)",
 "occupation": "Former retail manager, currently on disability",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Mother: Invasive breast cancer at age 61, hypertension",
  "Father: End-stage renal disease secondary to diabetes, stroke at age 68",
  "Maternal aunt: Pancreatic cancer at age 55",
  "Brother: Hepatitis C infection"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "800 mg PO three times daily with meals",
   "name": "Sevelamer carbonate"
  },
  {
   "dose": "667 mg PO with meals",
   "name": "Calcium acetate"
  },
  {
   "dose": "0.5 mcg PO daily",
   "name": "Calcitriol"
  },
  {
   "dose": "75 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "650 mg PO every 6 hours PRN pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "100 mg PO twice daily",
   "name": "Docusate sodium"
  }
 ],
 "surgical_history": [
  "Total parathyroidectomy with autotransplantation (complicated by hypoparathyroidism)",
  "Left modified radical mastectomy with sentinel lymph node biopsy",
  "Arteriovenous fistula creation (left upper extremity, 9 years prior)",
  "Exploratory laparotomy with pancreatic tumor resection"
 ],
 "chronic_conditions": [
  "End-stage renal disease on maintenance hemodialysis (9 years duration)",
  "Secondary hyperparathyroidism, status post parathyroidectomy",
  "Post-surgical hypoparathyroidism",
  "Hypertension (long-standing, poorly controlled)",
  "Pancreatic neuroendocrine tumor (VIPoma)",
  "Invasive ductal carcinoma of the left breast",
  "Cryoglobulinemia secondary to chronic hepatitis C infection",
  "Mobitz type I atrioventricular block",
  "Osteoarthritis",
  "History of intravenous drug use (in remission 5 years)",
  "Vitamin D deficiency"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Pancreatic neuroendocrine tumor localization", "Mobitz type I atrioventricular block (Wenckebach phenomenon)", "Invasive ductal carcinoma of left breast", "Hypokalemia managed with oral replacement (tolerating PO intake)", "Secondary hyperparathyroidism", "Postprocedural hypoparathyroidism", "Secondary hyperparathyroidism due to vitamin D deficiency", "Cryoglobulinemia"]
Comorbidities: ["Essential hypertension", "Hyperphosphatemia"]

## Encounter 0: source 2020-01-15, practice 2021-07-19, outpatient, Nephrology

Chief complaint: Routine follow-up for chronic kidney disease with fatigue and generalized bone discomfort

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic kidney disease with fatigue and generalized bone discomfort

HISTORY OF PRESENT ILLNESS:
She reports fatigue and generalized bone discomfort but denies recent falls or trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- End-stage renal disease on maintenance hemodialysis (9 years duration)
- Secondary hyperparathyroidism, status post parathyroidectomy
- Post-surgical hypoparathyroidism
- Hypertension (long-standing, poorly controlled)
- Pancreatic neuroendocrine tumor (VIPoma)
- Invasive ductal carcinoma of the left breast
- Cryoglobulinemia secondary to chronic hepatitis C infection
- Mobitz type I atrioventricular block
- Osteoarthritis
- History of intravenous drug use (in remission 5 years)
- Vitamin D deficiency

PAST SURGICAL HISTORY:
- Total parathyroidectomy with autotransplantation (complicated by hypoparathyroidism)
- Left modified radical mastectomy with sentinel lymph node biopsy
- Arteriovenous fistula creation (left upper extremity, 9 years prior)
- Exploratory laparotomy with pancreatic tumor resection

MEDICATIONS:
She has been compliant with her medications, which include lisinopril, calcium acetate, and vitamin D.

Home Medications:
- Amlodipine 10 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.5 mcg PO daily
- Levothyroxine 75 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain
- Docusate sodium 100 mg PO twice daily

ALLERGIES:
Sulfa drugs (anaphylaxis), Contrast dye (hives)

FAMILY HISTORY:
- Mother: Invasive breast cancer at age 61, hypertension
- Father: End-stage renal disease secondary to diabetes, stroke at age 68
- Maternal aunt: Pancreatic cancer at age 55
- Brother: Hepatitis C infection

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Former retail manager, currently on disability.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Her most recent laboratory studies reveal a serum calcium of 8.2 mg/dL and phosphate of 5.4 mg/dL, and an elevated parathyroid hormone (PTH) level.

## Encounter 1: source 2021-01-15, practice 2022-07-18, outpatient, Nephrology

Chief complaint: Worsening persistent aching pain in thighs and shoulders over the past 2 months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Chen, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening persistent aching pain in thighs and shoulders over the past 2 months

HISTORY OF PRESENT ILLNESS:
The patient with end-stage renal disease returns one year after evaluation for bone discomfort. She reports persistent aching in thighs and shoulders for the past 2 months. Appetite is normal and she denies trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism due to vitamin D deficiency (diagnosed 2020-01-15)
- End-stage renal disease on maintenance hemodialysis (9 years duration)
- Secondary hyperparathyroidism, status post parathyroidectomy
- Post-surgical hypoparathyroidism
- Hypertension (long-standing, poorly controlled)
- Pancreatic neuroendocrine tumor (VIPoma)
- Invasive ductal carcinoma of the left breast
- Cryoglobulinemia secondary to chronic hepatitis C infection
- Mobitz type I atrioventricular block
- Osteoarthritis
- History of intravenous drug use (in remission 5 years)
- Vitamin D deficiency

She has end-stage renal disease secondary to poorly controlled hypertension and has been undergoing hemodialysis for 9 years.

PAST SURGICAL HISTORY:
- Total parathyroidectomy with autotransplantation (complicated by hypoparathyroidism)
- Left modified radical mastectomy with sentinel lymph node biopsy
- Arteriovenous fistula creation (left upper extremity, 9 years prior)
- Exploratory laparotomy with pancreatic tumor resection

MEDICATIONS:
She takes amlodipine and sevelamer.

Home Medications:
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcium acetate 667 mg PO with meals
- Calcitriol 0.5 mcg PO daily
- Levothyroxine 75 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain
- Docusate sodium 100 mg PO twice daily

ALLERGIES:
Sulfa drugs (anaphylaxis), Contrast dye (hives)

FAMILY HISTORY:
- Mother: Invasive breast cancer at age 61, hypertension
- Father: End-stage renal disease secondary to diabetes, stroke at age 68
- Maternal aunt: Pancreatic cancer at age 55
- Brother: Hepatitis C infection

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Former retail manager, currently on disability.

PHYSICAL EXAMINATION:
Physical exam reveals bilateral tenderness over the proximal femurs and clavicles, but no focal neurological deficits.

LABORATORY DATA:
Serum calcium 7.6 mg/dL (normal: 8.5–10.5)
Serum phosphate 6.4 mg/dL (normal: 2.5–4.5)
PTH 820 pg/mL (normal: 15–65)
25-hydroxyvitamin D Normal
1,25-dihydroxyvitamin D Decreased

IMAGING:
X-ray of the femurs shows subperiosteal bone resorption near the diaphysis.

## Encounter 2: source 2021-02-15, practice 2022-08-18, ed, Emergency Medicine

Chief complaint: Progressive muscle cramping, stiffness, perioral numbness, and difficulty holding utensils after recent neck surgery

ENCOUNTER NOTE
Date: 2021-02-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive muscle cramping, stiffness, perioral numbness, and difficulty holding utensils after recent neck surgery

HISTORY OF PRESENT ILLNESS:
This 53-year-old female with end-stage renal disease on maintenance hemodialysis for 9 years and recent parathyroidectomy following January 2021 nephrology evaluation for worsening secondary hyperparathyroidism presents to the emergency department due to progressive muscle cramping, stiffness, and numbness around her mouth that began earlier today. She also reports intermittent tingling in her fingertips and difficulty holding utensils while eating. Two days ago, she underwent a surgical procedure under general anesthesia for treatment of a thyroid mass, after which she was discharged home in stable condition. She has since been taking her prescribed pain medications and consuming oral fluids without difficulty.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism due to vitamin D deficiency (diagnosed 2020-01-15)
- Secondary hyperparathyroidism (diagnosed 2021-01-15)
- End-stage renal disease on maintenance hemodialysis (9 years duration)
- Secondary hyperparathyroidism, status post parathyroidectomy
- Post-surgical hypoparathyroidism
- Hypertension (long-standing, poorly controlled)
- Pancreatic neuroendocrine tumor (VIPoma)
- Invasive ductal carcinoma of the left breast
- Cryoglobulinemia secondary to chronic hepatitis C infection
- Mobitz type I atrioventricular block
- Osteoarthritis
- History of intravenous drug use (in remission 5 years)
- Vitamin D deficiency

Her past medical history includes hypertension and osteoarthritis.

PAST SURGICAL HISTORY:
- Total parathyroidectomy with autotransplantation (complicated by hypoparathyroidism)
- Left modified radical mastectomy with sentinel lymph node biopsy
- Arteriovenous fistula creation (left upper extremity, 9 years prior)
- Exploratory laparotomy with pancreatic tumor resection

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcium acetate 667 mg PO with meals
- Calcitriol 0.5 mcg PO daily
- Levothyroxine 75 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain
- Docusate sodium 100 mg PO twice daily

ALLERGIES:
Sulfa drugs (anaphylaxis), Contrast dye (hives)

FAMILY HISTORY:
- Mother: Invasive breast cancer at age 61, hypertension
- Father: End-stage renal disease secondary to diabetes, stroke at age 68
- Maternal aunt: Pancreatic cancer at age 55
- Brother: Hepatitis C infection

SOCIAL HISTORY:
She does not smoke and drinks alcohol occasionally.

REVIEW OF SYSTEMS:
On review of systems, she endorses mild headaches but denies fever, chest pain, palpitations, shortness of breath, or swelling in her legs.

VITAL SIGNS:
On examination, temperature is 36.8°C (98.2°F), blood pressure is 132/78 mmHg, and pulse is 80/min. BMI is 23 kg/m².

PHYSICAL EXAMINATION:
Neurologic examination reveals brisk deep tendon reflexes and a positive Chvostek sign when tapping over the angle of the mandible anterior to the ear. Inflation of a blood pressure cuff above systolic pressure induces carpopedal spasm within three minutes. Cardiopulmonary and abdominal examinations are normal. The surgical incision over her anterior neck is clean and dry without erythema or fluctuance.

## Encounter 3: source 2022-01-15, practice 2023-07-17, outpatient, General Surgery

Chief complaint: Painless palpable lump in left breast discovered one month ago without nipple discharge or skin changes

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Johnson, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Painless palpable lump in left breast discovered one month ago without nipple discharge or skin changes

HISTORY OF PRESENT ILLNESS:
The patient is a 54-year-old female with end-stage renal disease on hemodialysis and prior parathyroidectomy who presents with a palpable lump in her left breast discovered one month ago. She reports no associated pain, nipple discharge, or changes in the overlying skin.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism due to vitamin D deficiency (diagnosed 2020-01-15)
- Secondary hyperparathyroidism (diagnosed 2021-01-15)
- Postprocedural hypoparathyroidism (diagnosed 2021-02-15)
- End-stage renal disease on maintenance hemodialysis (9 years duration)
- Secondary hyperparathyroidism, status post parathyroidectomy
- Post-surgical hypoparathyroidism
- Hypertension (long-standing, poorly controlled)
- Pancreatic neuroendocrine tumor (VIPoma)
- Invasive ductal carcinoma of the left breast
- Cryoglobulinemia secondary to chronic hepatitis C infection
- Mobitz type I atrioventricular block
- Osteoarthritis
- History of intravenous drug use (in remission 5 years)
- Vitamin D deficiency

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Total parathyroidectomy with autotransplantation (complicated by hypoparathyroidism)
- Left modified radical mastectomy with sentinel lymph node biopsy
- Arteriovenous fistula creation (left upper extremity, 9 years prior)
- Exploratory laparotomy with pancreatic tumor resection

MEDICATIONS:
She takes controlled with hydrochlorothiazide.

Home Medications:
- Amlodipine 10 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcium acetate 667 mg PO with meals
- Calcitriol 0.5 mcg PO daily
- Levothyroxine 75 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain
- Docusate sodium 100 mg PO twice daily

ALLERGIES:
Sulfa drugs (anaphylaxis), Contrast dye (hives)

FAMILY HISTORY:
She has no family history of breast or ovarian cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Former retail manager, currently on disability.

PHYSICAL EXAMINATION:
On physical examination, a 2 cm firm, non-tender mass is palpated in the upper outer quadrant of the left breast. There is no axillary lymphadenopathy.

IMAGING:
Mammography shows a 2.1 cm irregular mass with spiculated margins in the left breast.

PATHOLOGY:
Ultrasound-guided core needle biopsy of the mass is performed, and histopathology reveals invasive ductal carcinoma, estrogen receptor-positive, progesterone receptor-positive, and HER2/neu negative.

PLAN:
The patient elects to undergo breast-conserving therapy.

## Encounter 4: source 2023-01-15, practice 2024-07-22, outpatient, Rheumatology

Chief complaint: Fatigue, joint pain, and new rash on lower extremities with foamy urine noted over the past week

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Park, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, joint pain, and new rash on lower extremities with foamy urine noted over the past week

HISTORY OF PRESENT ILLNESS:
The patient is a 55-year-old female with end-stage renal disease on hemodialysis, cryoglobulinemia, prior parathyroidectomy, and breast cancer who has a history of intravenous drug use and presents to the rheumatology clinic complaining of fatigue, joint pain, and a rash on her lower extremities that appeared over the past week. She denies any recent infections or new medications. She has no history of renal disease but mentions occasional "foamy" urine.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism due to vitamin D deficiency (diagnosed 2020-01-15)
- Secondary hyperparathyroidism (diagnosed 2021-01-15)
- Postprocedural hypoparathyroidism (diagnosed 2021-02-15)
- Invasive ductal carcinoma of left breast (diagnosed 2022-01-15)
- End-stage renal disease on maintenance hemodialysis (9 years duration)
- Secondary hyperparathyroidism, status post parathyroidectomy
- Post-surgical hypoparathyroidism
- Hypertension (long-standing, poorly controlled)
- Pancreatic neuroendocrine tumor (VIPoma)
- Invasive ductal carcinoma of the left breast
- Cryoglobulinemia secondary to chronic hepatitis C infection
- Mobitz type I atrioventricular block
- Osteoarthritis
- History of intravenous drug use (in remission 5 years)
- Vitamin D deficiency

PAST SURGICAL HISTORY:
- Total parathyroidectomy with autotransplantation (complicated by hypoparathyroidism)
- Left modified radical mastectomy with sentinel lymph node biopsy
- Arteriovenous fistula creation (left upper extremity, 9 years prior)
- Exploratory laparotomy with pancreatic tumor resection

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcium acetate 667 mg PO with meals
- Calcitriol 0.5 mcg PO daily
- Levothyroxine 75 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain
- Docusate sodium 100 mg PO twice daily

ALLERGIES:
Sulfa drugs (anaphylaxis), Contrast dye (hives)

FAMILY HISTORY:
- Mother: Invasive breast cancer at age 61, hypertension
- Father: End-stage renal disease secondary to diabetes, stroke at age 68
- Maternal aunt: Pancreatic cancer at age 55
- Brother: Hepatitis C infection

SOCIAL HISTORY:
Intravenous drug use

VITAL SIGNS:
Her temperature is 37.2°C (99°F), blood pressure is 140/90 mmHg, and pulse is 78/min.

PHYSICAL EXAMINATION:
On examination, you note palpable purpura on her legs, mild synovitis in her knees, and no other abnormalities.

LABORATORY DATA:
Complete blood count: Normal
Serum creatinine: 1.8 mg/dL (baseline 0.9 mg/dL)
Urinalysis: Proteinuria, hematuria
Liver function tests: AST 70 U/L, ALT 65 U/L
Serum cryoglobulins: Positive
Complement levels: C4 decreased
Rheumatoid factor: Positive
Hepatitis C antibody: Positive
Hepatitis C RNA: Detected

## Encounter 5: source 2024-01-15, practice 2025-07-14, outpatient, Gastroenterology

Chief complaint: Six-month history of painful pruritic skin lesions spreading from groin to face and extremities with 15-pound weight loss and intermittent diarrhea

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of painful pruritic skin lesions spreading from groin to face and extremities with 15-pound weight loss and intermittent diarrhea

HISTORY OF PRESENT ILLNESS:
The patient with end-stage renal disease on maintenance hemodialysis, prior parathyroidectomy with hypoparathyroidism, and cryoglobulinemia presents with worsening complaints. She reports that the lesions began as small papules on her groin but have since coalesced into large, erythematous plaques that have spread to her face and extremities. She also mentions a 15-pound weight loss, intermittent diarrhea, and new-onset blurry vision.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism due to vitamin D deficiency (diagnosed 2020-01-15)
- Secondary hyperparathyroidism (diagnosed 2021-01-15)
- Postprocedural hypoparathyroidism (diagnosed 2021-02-15)
- Invasive ductal carcinoma of left breast (diagnosed 2022-01-15)
- Cryoglobulinemia (diagnosed 2023-01-15)
- End-stage renal disease on maintenance hemodialysis (9 years duration)
- Secondary hyperparathyroidism, status post parathyroidectomy
- Post-surgical hypoparathyroidism
- Hypertension (long-standing, poorly controlled)
- Pancreatic neuroendocrine tumor (VIPoma)
- Invasive ductal carcinoma of the left breast
- Cryoglobulinemia secondary to chronic hepatitis C infection
- Mobitz type I atrioventricular block
- Osteoarthritis
- History of intravenous drug use (in remission 5 years)
- Vitamin D deficiency

Her past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Total parathyroidectomy with autotransplantation (complicated by hypoparathyroidism)
- Left modified radical mastectomy with sentinel lymph node biopsy
- Arteriovenous fistula creation (left upper extremity, 9 years prior)
- Exploratory laparotomy with pancreatic tumor resection

MEDICATIONS:
Hydrochlorothiazide

Home Medications:
- Amlodipine 10 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcium acetate 667 mg PO with meals
- Calcitriol 0.5 mcg PO daily
- Levothyroxine 75 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain
- Docusate sodium 100 mg PO twice daily

ALLERGIES:
Sulfa drugs (anaphylaxis), Contrast dye (hives)

FAMILY HISTORY:
- Mother: Invasive breast cancer at age 61, hypertension
- Father: End-stage renal disease secondary to diabetes, stroke at age 68
- Maternal aunt: Pancreatic cancer at age 55
- Brother: Hepatitis C infection

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Former retail manager, currently on disability.

PHYSICAL EXAMINATION:
On physical examination, you note several large, annular plaques with central clearing and raised erythematous borders. She also has mild mucosal ulceration.

LABORATORY DATA:
Fasting glucose: 180 mg/dL
- Hemoglobin: 10.2 g/dL (normocytic anemia)
- Serum glucagon: 600 pg/mL

## Encounter 6: source 2024-07-15, practice 2026-01-19, outpatient, Internal Medicine

Chief complaint: Worsening fatigue, leg cramps, and intermittent palpitations over the past two weeks after antihypertensive medication adjustment

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. David Kim, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening fatigue, leg cramps, and intermittent palpitations over the past two weeks after antihypertensive medication adjustment

HISTORY OF PRESENT ILLNESS:
This 56-year-old female with end-stage renal disease on maintenance hemodialysis, post-surgical hypoparathyroidism, and pancreatic neuroendocrine tumor returns following recent antihypertensive adjustment. She was diagnosed with hypertension one month ago and started on a new antihypertensive medication at that time. She reports that her blood pressure readings at home have improved, but now she often feels weak and sometimes experiences palpitations. She denies chest pain, dizziness, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism due to vitamin D deficiency (diagnosed 2020-01-15)
- Secondary hyperparathyroidism (diagnosed 2021-01-15)
- Postprocedural hypoparathyroidism (diagnosed 2021-02-15)
- Invasive ductal carcinoma of left breast (diagnosed 2022-01-15)
- Cryoglobulinemia (diagnosed 2023-01-15)
- Pancreatic neuroendocrine tumor localization (diagnosed 2024-01-15)
- End-stage renal disease on maintenance hemodialysis (9 years duration)
- Secondary hyperparathyroidism, status post parathyroidectomy
- Post-surgical hypoparathyroidism
- Hypertension (long-standing, poorly controlled)
- Pancreatic neuroendocrine tumor (VIPoma)
- Invasive ductal carcinoma of the left breast
- Cryoglobulinemia secondary to chronic hepatitis C infection
- Mobitz type I atrioventricular block
- Osteoarthritis
- History of intravenous drug use (in remission 5 years)
- Vitamin D deficiency

PAST SURGICAL HISTORY:
- Total parathyroidectomy with autotransplantation (complicated by hypoparathyroidism)
- Left modified radical mastectomy with sentinel lymph node biopsy
- Arteriovenous fistula creation (left upper extremity, 9 years prior)
- Exploratory laparotomy with pancreatic tumor resection

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcium acetate 667 mg PO with meals
- Calcitriol 0.5 mcg PO daily
- Levothyroxine 75 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain
- Docusate sodium 100 mg PO twice daily

ALLERGIES:
Sulfa drugs (anaphylaxis), Contrast dye (hives)

FAMILY HISTORY:
- Mother: Invasive breast cancer at age 61, hypertension
- Father: End-stage renal disease secondary to diabetes, stroke at age 68
- Maternal aunt: Pancreatic cancer at age 55
- Brother: Hepatitis C infection

SOCIAL HISTORY:
Her diet is low in sodium but otherwise unremarkable.

VITAL SIGNS:
Her blood pressure is 126/78 mm Hg.

PHYSICAL EXAMINATION:
Physical examination is normal except for mild muscle tenderness in the lower extremities.

LABORATORY DATA:
Basic metabolic panel reveals a decreased serum potassium level.

OTHER STUDIES:
ECG shows flattened T waves and prominent U waves.

## Encounter 7: source 2025-01-15, practice 2026-07-15, ed, Emergency Medicine

Chief complaint: Syncopal episode while standing at grocery store lasting less than one minute with complete recovery but persistent lightheadedness

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Lisa Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Syncopal episode while standing at grocery store lasting less than one minute with complete recovery but persistent lightheadedness

HISTORY OF PRESENT ILLNESS:
The patient with end-stage renal disease on maintenance hemodialysis and known Mobitz type I atrioventricular block presents to the emergency department after experiencing a syncopal episode while standing in line at the grocery store. She reports the episode lasted less than a minute with complete recovery, though she felt lightheaded for several minutes afterward.

PAST MEDICAL HISTORY:
Active Problem List:
- Secondary hyperparathyroidism due to vitamin D deficiency (diagnosed 2020-01-15)
- Secondary hyperparathyroidism (diagnosed 2021-01-15)
- Postprocedural hypoparathyroidism (diagnosed 2021-02-15)
- Invasive ductal carcinoma of left breast (diagnosed 2022-01-15)
- Cryoglobulinemia (diagnosed 2023-01-15)
- Pancreatic neuroendocrine tumor localization (diagnosed 2024-01-15)
- Hypokalemia managed with oral replacement (tolerating PO intake) (diagnosed 2024-07-15)
- End-stage renal disease on maintenance hemodialysis (9 years duration)
- Secondary hyperparathyroidism, status post parathyroidectomy
- Post-surgical hypoparathyroidism
- Hypertension (long-standing, poorly controlled)
- Pancreatic neuroendocrine tumor (VIPoma)
- Invasive ductal carcinoma of the left breast
- Cryoglobulinemia secondary to chronic hepatitis C infection
- Mobitz type I atrioventricular block
- Osteoarthritis
- History of intravenous drug use (in remission 5 years)
- Vitamin D deficiency

She has a history of hypertension.

PAST SURGICAL HISTORY:
- Total parathyroidectomy with autotransplantation (complicated by hypoparathyroidism)
- Left modified radical mastectomy with sentinel lymph node biopsy
- Arteriovenous fistula creation (left upper extremity, 9 years prior)
- Exploratory laparotomy with pancreatic tumor resection

MEDICATIONS:
She takes managed with hydrochlorothiazide.

Home Medications:
- Amlodipine 10 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcium acetate 667 mg PO with meals
- Calcitriol 0.5 mcg PO daily
- Levothyroxine 75 mcg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain
- Docusate sodium 100 mg PO twice daily

ALLERGIES:
Sulfa drugs (anaphylaxis), Contrast dye (hives)

FAMILY HISTORY:
- Mother: Invasive breast cancer at age 61, hypertension
- Father: End-stage renal disease secondary to diabetes, stroke at age 68
- Maternal aunt: Pancreatic cancer at age 55
- Brother: Hepatitis C infection

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Former retail manager, currently on disability.

REVIEW OF SYSTEMS:
She denies any chest pain, palpitations, or shortness of breath

VITAL SIGNS:
On examination, her pulse is 54/min and regular, blood pressure is 110/72 mm Hg, and respiratory rate is 14/min

PHYSICAL EXAMINATION:
Cardiopulmonary examination is unremarkable

OTHER STUDIES:
ECG findings are shown below
