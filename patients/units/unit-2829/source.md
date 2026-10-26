# synthetic_hospital patient 2829

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 69 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-07-06 (Wed) | follow_up | Cardiology |
| 1 | 2020-04-15 | 2022-09-27 (Tue) | outpatient | Internal Medicine |
| 2 | 2020-10-15 | 2023-04-03 (Mon) | outpatient | Ophthalmology |
| 3 | 2021-04-15 | 2023-10-16 (Mon) | outpatient | Cardiology |
| 4 | 2021-10-15 | 2024-04-02 (Tue) | outpatient | Internal Medicine |
| 5 | 2022-05-15 | 2024-11-11 (Mon) | outpatient | Internal Medicine |
| 6 | 2022-11-15 | 2025-04-29 (Tue) | outpatient | Geriatrics |
| 7 | 2023-07-15 | 2026-01-07 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "ACE inhibitor-induced cough",
  "Sulfa drugs"
 ],
 "insurance": "Medicare",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "Rare",
 "family_history": [
  "Father with Type 2 Diabetes Mellitus and chronic kidney disease",
  "Mother with hypertension and hyperlipidemia",
  "Paternal grandfather with ischemic stroke"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "5 mg PO daily (adjusted per INR)",
   "name": "Warfarin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "500 mg PO every 6 hours PRN for knee pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "17 g PO daily PRN for constipation",
   "name": "Polyethylene glycol 3350"
  }
 ],
 "surgical_history": [
  "Mechanical mitral valve replacement",
  "Remote cholecystectomy"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Chronic Kidney Disease Stage 4",
  "Heart Failure with preserved Ejection Fraction",
  "Hyperlipidemia",
  "Cervical spondylosis",
  "Osteoarthritis, bilateral knees",
  "Age-related cataracts, bilateral",
  "Obesity (BMI 32 kg/m\u00b2)",
  "Chronic constipation"
 ],
 "age_at_first_encounter": 69
}
```

Primary diagnoses: ["Anemia of chronic kidney disease", "Osteoarthritis of knee", "Status post mechanical mitral valve replacement", "Lisinopril adverse effect", "Cervical spinal cord or ligamentous injury (indication for MRI)", "Age-related bilateral cataracts", "Heart failure with preserved ejection fraction", "Adverse effect of anticholinergic medication"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-07-06, follow_up, Cardiology

Chief complaint: Postoperative follow-up three weeks after mechanical mitral valve replacement

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: follow_up

CHIEF COMPLAINT:
Postoperative follow-up three weeks after mechanical mitral valve replacement

HISTORY OF PRESENT ILLNESS:
She reports mild fatigue but no chest pain, palpitations, or dyspnea. Postoperatively, she was started on an oral anticoagulant to prevent thromboembolic complications related to her prosthetic valve.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 4
- Heart Failure with preserved Ejection Fraction
- Hyperlipidemia
- Cervical spondylosis
- Osteoarthritis, bilateral knees
- Age-related cataracts, bilateral
- Obesity (BMI 32 kg/m²)
- Chronic constipation

Her past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement
- Remote cholecystectomy

MEDICATIONS:
Her current medications include lisinopril, metformin, and a blood thinner.

Home Medications:
- Insulin glargine 20 units subcutaneous at bedtime
- Amlodipine 5 mg PO daily
- Warfarin 5 mg PO daily (adjusted per INR)
- Atorvastatin 20 mg PO daily
- Acetaminophen 500 mg PO every 6 hours PRN for knee pain
- Polyethylene glycol 3350 17 g PO daily PRN for constipation

ALLERGIES:
ACE inhibitor-induced cough, Sulfa drugs

FAMILY HISTORY:
- Father with Type 2 Diabetes Mellitus and chronic kidney disease
- Mother with hypertension and hyperlipidemia
- Paternal grandfather with ischemic stroke

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
, and her physical exam is unremarkable.

LABORATORY DATA:
Recent labs show a prolonged prothrombin time (PT) with a normal activated partial thromboplastin time (aPTT).

## Encounter 1: source 2020-04-15, practice 2022-09-27, outpatient, Internal Medicine

Chief complaint: Bilateral knee pain worsening over 6 months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Bilateral knee pain worsening over 6 months

HISTORY OF PRESENT ILLNESS:
The patient with diabetes mellitus and CKD returns three months following mechanical mitral valve replacement. She presents with a 6-month history of bilateral knee pain that worsens with activity and improves with rest. She reports morning stiffness lasting less than 30 minutes and occasional swelling in her knees. She has tried acetaminophen and ibuprofen with minimal relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post mechanical mitral valve replacement (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 4
- Heart Failure with preserved Ejection Fraction
- Hyperlipidemia
- Cervical spondylosis
- Osteoarthritis, bilateral knees
- Age-related cataracts, bilateral
- Obesity (BMI 32 kg/m²)
- Chronic constipation

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement
- Remote cholecystectomy

MEDICATIONS:
She takes metformin and lisinopril.

Home Medications:
- Insulin glargine 20 units subcutaneous at bedtime
- Amlodipine 5 mg PO daily
- Warfarin 5 mg PO daily (adjusted per INR)
- Atorvastatin 20 mg PO daily
- Acetaminophen 500 mg PO every 6 hours PRN for knee pain
- Polyethylene glycol 3350 17 g PO daily PRN for constipation

ALLERGIES:
ACE inhibitor-induced cough, Sulfa drugs

FAMILY HISTORY:
- Father with Type 2 Diabetes Mellitus and chronic kidney disease
- Mother with hypertension and hyperlipidemia
- Paternal grandfather with ischemic stroke

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare. Occupation: Retired elementary school teacher.

VITAL SIGNS:
BMI of 32 kg/m^2

PHYSICAL EXAMINATION:
Crepitus with knee movement, no erythema or warmth, and mild bilateral knee effusions. There is no evidence of joint instability, and the ligaments are intact.

IMAGING:
Radiographs of the knees show joint space narrowing, subchondral sclerosis, and osteophyte formation.

## Encounter 2: source 2020-10-15, practice 2023-04-03, outpatient, Ophthalmology

Chief complaint: Progressively worsening vision in both eyes over the past year

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Priya Patel, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressively worsening vision in both eyes over the past year

HISTORY OF PRESENT ILLNESS:
The patient returns to ophthalmology reporting progressively worsening vision in both eyes over the past year, continuing longitudinal care following recent internal medicine evaluation for knee osteoarthritis and cardiology follow-up for mechanical mitral valve replacement. She reports difficulty reading, driving at night due to glare, and recently started avoiding knitting, which she used to enjoy. She denies any eye pain, redness, or history of trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post mechanical mitral valve replacement (diagnosed 2020-01-15)
- Osteoarthritis of knee (diagnosed 2020-04-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 4
- Heart Failure with preserved Ejection Fraction
- Hyperlipidemia
- Cervical spondylosis
- Osteoarthritis, bilateral knees
- Age-related cataracts, bilateral
- Obesity (BMI 32 kg/m²)
- Chronic constipation

Her past medical history is significant for type 2 diabetes mellitus, hypertension, and hypercholesterolemia.

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement
- Remote cholecystectomy

MEDICATIONS:
Home Medications:
- Insulin glargine 20 units subcutaneous at bedtime
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Warfarin 5 mg PO daily (adjusted per INR)
- Atorvastatin 20 mg PO daily
- Acetaminophen 500 mg PO every 6 hours PRN for knee pain
- Polyethylene glycol 3350 17 g PO daily PRN for constipation

ALLERGIES:
ACE inhibitor-induced cough, Sulfa drugs

FAMILY HISTORY:
- Father with Type 2 Diabetes Mellitus and chronic kidney disease
- Mother with hypertension and hyperlipidemia
- Paternal grandfather with ischemic stroke

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, visual acuity is 20/50 in the right eye and 20/60 in the left eye. Slit-lamp examination reveals bilateral opacification of the lenses. The remainder of the eye examination, including intraocular pressure, is within normal limits. Fundoscopic examination shows no evidence of diabetic retinopathy or macular degeneration.

## Encounter 3: source 2021-04-15, practice 2023-10-16, outpatient, Cardiology

Chief complaint: Progressive exertional dyspnea and fatigue over 6 months with 5kg weight gain

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive exertional dyspnea and fatigue over 6 months with 5kg weight gain

HISTORY OF PRESENT ILLNESS:
The patient returns for Cardiology follow-up of her mechanical mitral valve replacement and heart failure with preserved ejection fraction, presenting with progressive exertional dyspnea and fatigue over the past 6 months. She reports weight gain of 5 kg during this period. She has been adherent to her antihypertensive medications and metformin. She denies chest pain, palpitations, or lower extremity edema.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post mechanical mitral valve replacement (diagnosed 2020-01-15)
- Osteoarthritis of knee (diagnosed 2020-04-15)
- Age-related bilateral cataracts (diagnosed 2020-10-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 4
- Heart Failure with preserved Ejection Fraction
- Hyperlipidemia
- Cervical spondylosis
- Osteoarthritis, bilateral knees
- Obesity (BMI 32 kg/m²)
- Chronic constipation

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement
- Remote cholecystectomy

MEDICATIONS:
She takes antihypertensive medications and metformin.

Home Medications:
- Insulin glargine 20 units subcutaneous at bedtime
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Warfarin 5 mg PO daily (adjusted per INR)
- Atorvastatin 20 mg PO daily
- Acetaminophen 500 mg PO every 6 hours PRN for knee pain
- Polyethylene glycol 3350 17 g PO daily PRN for constipation

ALLERGIES:
ACE inhibitor-induced cough, Sulfa drugs

FAMILY HISTORY:
- Father with Type 2 Diabetes Mellitus and chronic kidney disease
- Mother with hypertension and hyperlipidemia
- Paternal grandfather with ischemic stroke

SOCIAL HISTORY:
She has a sedentary lifestyle and a body mass index of 32 kg/m^2.

VITAL SIGNS:
Blood pressure is 145/90 mmHg, heart rate is 78 beats per minute, and respiratory rate is 18 breaths per minute

PHYSICAL EXAMINATION:
Cardiac auscultation reveals an S4 gallop, and lung fields are clear. There is no jugular venous distention or peripheral edema.

LABORATORY DATA:
Serum sodium: 140 mEq/L
Serum potassium: 4.2 mEq/L
Serum creatinine: 1.1 mg/dL
Fasting blood glucose: 158 mg/dL
B-type natriuretic peptide (BNP): 300 pg/mL

IMAGING:
An echocardiogram shows a left ventricular ejection fraction of 60%, left ventricular hypertrophy, and evidence of diastolic dysfunction.

## Encounter 4: source 2021-10-15, practice 2024-04-02, outpatient, Internal Medicine

Chief complaint: Fatigue and decreased activity tolerance for the past several months

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. James O'Brien, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and decreased activity tolerance for the past several months

HISTORY OF PRESENT ILLNESS:
This 70-year-old female with CKD stage 4, HFpEF, and mechanical mitral valve returns with progressive fatigue. She reports she no longer walks because she tires quickly and sometimes naps during the day. She denies chest pain, bleeding, dietary changes, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post mechanical mitral valve replacement (diagnosed 2020-01-15)
- Osteoarthritis of knee (diagnosed 2020-04-15)
- Age-related bilateral cataracts (diagnosed 2020-10-15)
- Heart failure with preserved ejection fraction (diagnosed 2021-04-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 4
- Hyperlipidemia
- Cervical spondylosis
- Osteoarthritis, bilateral knees
- Obesity (BMI 32 kg/m²)
- Chronic constipation

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement
- Remote cholecystectomy

MEDICATIONS:
She takes insulin, lisinopril, and amlodipine.

Home Medications:
- Insulin glargine 20 units subcutaneous at bedtime
- Warfarin 5 mg PO daily (adjusted per INR)
- Atorvastatin 20 mg PO daily
- Acetaminophen 500 mg PO every 6 hours PRN for knee pain
- Polyethylene glycol 3350 17 g PO daily PRN for constipation

ALLERGIES:
ACE inhibitor-induced cough, Sulfa drugs

FAMILY HISTORY:
- Father with Type 2 Diabetes Mellitus and chronic kidney disease
- Mother with hypertension and hyperlipidemia
- Paternal grandfather with ischemic stroke

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
Pale conjunctivae and trace ankle edema

LABORATORY DATA:
Hemoglobin 8.4 g/dL
Hematocrit 26%
MCV 88 fL
Reticulocyte count 1.1%
Serum creatinine 3.4 mg/dL
Ferritin 110 ng/mL

## Encounter 5: source 2022-05-15, practice 2024-11-11, outpatient, Internal Medicine

Chief complaint: Persistent dry cough for the past 3 weeks

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. James O'Brien, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent dry cough for the past 3 weeks

HISTORY OF PRESENT ILLNESS:
The patient is a 71-year-old female with a history of mechanical mitral valve replacement, heart failure with preserved ejection fraction, and chronic kidney disease stage 4 who presents for evaluation of a persistent dry cough. She describes the cough as dry and nonproductive, occurring throughout the day and sometimes at night, interfering with her sleep. She denies fever, chills, shortness of breath, or sputum production. There is no history of recent upper respiratory infection, allergic symptoms, or acid reflux. She tried over-the-counter cough syrup without improvement. The patient notes the cough began approximately one week after starting her new medication, and she expresses concern regarding potential side effects of her blood pressure medicine after reading online information.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post mechanical mitral valve replacement (diagnosed 2020-01-15)
- Osteoarthritis of knee (diagnosed 2020-04-15)
- Age-related bilateral cataracts (diagnosed 2020-10-15)
- Heart failure with preserved ejection fraction (diagnosed 2021-04-15)
- Anemia of chronic kidney disease (diagnosed 2021-10-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 4
- Hyperlipidemia
- Cervical spondylosis
- Osteoarthritis, bilateral knees
- Obesity (BMI 32 kg/m²)
- Chronic constipation

The patient has a history of hypertension and type 2 diabetes mellitus, both well controlled with medications. She also has mild chronic kidney disease.

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement
- Remote cholecystectomy

MEDICATIONS:
Her medications include metformin and a recently started antihypertensive prescribed 1 month ago after her last office visit.

Home Medications:
- Insulin glargine 20 units subcutaneous at bedtime
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Warfarin 5 mg PO daily (adjusted per INR)
- Atorvastatin 20 mg PO daily
- Acetaminophen 500 mg PO every 6 hours PRN for knee pain
- Polyethylene glycol 3350 17 g PO daily PRN for constipation

ALLERGIES:
ACE inhibitor-induced cough, Sulfa drugs

FAMILY HISTORY:
- Father with Type 2 Diabetes Mellitus and chronic kidney disease
- Mother with hypertension and hyperlipidemia
- Paternal grandfather with ischemic stroke

SOCIAL HISTORY:
She has no history of smoking or occupational dust exposure.

REVIEW OF SYSTEMS:
On review of systems, she denies wheezing, hemoptysis, chest pain, or palpitations. She has not noticed any swelling of the face or extremities.

VITAL SIGNS:
Oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
Physical examination is unremarkable, with normal lung sounds and no signs of heart failure.

IMAGING:
Chest x-ray is normal.

ASSESSMENT AND PLAN:
The physician suspects the cough is medication-related and explains the mechanism to the patient.

## Encounter 6: source 2022-11-15, practice 2025-04-29, outpatient, Geriatrics

Chief complaint: Increasing forgetfulness and confusion over two months

ENCOUNTER NOTE
Date: 2022-11-15
Provider: Dr. Aisha Johnson, MD
Department: Geriatrics
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing forgetfulness and confusion over two months

HISTORY OF PRESENT ILLNESS:
Patient with mechanical mitral valve replacement, heart failure, and chronic kidney disease presents to the clinic with her daughter. Her daughter reports that her mother has been experiencing increasing forgetfulness and confusion over the past two months.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post mechanical mitral valve replacement (diagnosed 2020-01-15)
- Osteoarthritis of knee (diagnosed 2020-04-15)
- Age-related bilateral cataracts (diagnosed 2020-10-15)
- Heart failure with preserved ejection fraction (diagnosed 2021-04-15)
- Anemia of chronic kidney disease (diagnosed 2021-10-15)
- Lisinopril adverse effect (diagnosed 2022-05-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 4
- Hyperlipidemia
- Cervical spondylosis
- Osteoarthritis, bilateral knees
- Obesity (BMI 32 kg/m²)
- Chronic constipation

The patient has a history of hypertension, type 2 diabetes mellitus, and chronic constipation.

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement
- Remote cholecystectomy

MEDICATIONS:
Her current medications include metformin, lisinopril, and a newly prescribed medication for constipation that she started three months ago.

Home Medications:
- Insulin glargine 20 units subcutaneous at bedtime
- Amlodipine 5 mg PO daily
- Warfarin 5 mg PO daily (adjusted per INR)
- Atorvastatin 20 mg PO daily
- Acetaminophen 500 mg PO every 6 hours PRN for knee pain
- Polyethylene glycol 3350 17 g PO daily PRN for constipation

ALLERGIES:
ACE inhibitor-induced cough, Sulfa drugs

FAMILY HISTORY:
- Father with Type 2 Diabetes Mellitus and chronic kidney disease
- Mother with hypertension and hyperlipidemia
- Paternal grandfather with ischemic stroke

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her blood pressure is 130/80 mmHg

PHYSICAL EXAMINATION:
On examination, the patient is alert but appears to have difficulty with short-term memory tasks and is disoriented to the date. Her physical examination is otherwise unremarkable.

LABORATORY DATA:
Laboratory tests reveal:

- Serum glucose: 140 mg/dL
- Serum creatinine: 1.0 mg/dL
- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Hemoglobin A1c: 7.2%

## Encounter 7: source 2023-07-15, practice 2026-01-07, ed, Emergency Medicine

Chief complaint: Fall at home with neck hyperextension and arm weakness

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fall at home with neck hyperextension and arm weakness

HISTORY OF PRESENT ILLNESS:
This 72-year-old female with a history of cervical spondylosis and mechanical mitral valve replacement on chronic warfarin anticoagulation presents to the Emergency Department following a fall. She reports that she tripped over a rug and landed on her back with her neck hyperextended. She complains of weakness in her arms, more pronounced than in her legs, and has difficulty handling objects. She denies loss of consciousness or bowel and bladder incontinence.

PAST MEDICAL HISTORY:
Active Problem List:
- Status post mechanical mitral valve replacement (diagnosed 2020-01-15)
- Osteoarthritis of knee (diagnosed 2020-04-15)
- Age-related bilateral cataracts (diagnosed 2020-10-15)
- Heart failure with preserved ejection fraction (diagnosed 2021-04-15)
- Anemia of chronic kidney disease (diagnosed 2021-10-15)
- Lisinopril adverse effect (diagnosed 2022-05-15)
- Adverse effect of anticholinergic medication (diagnosed 2022-11-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 4
- Hyperlipidemia
- Cervical spondylosis
- Osteoarthritis, bilateral knees
- Obesity (BMI 32 kg/m²)
- Chronic constipation

Her past medical history includes hypertension, type 2 diabetes mellitus, and cervical spondylosis.

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement
- Remote cholecystectomy

MEDICATIONS:
Home Medications:
- Insulin glargine 20 units subcutaneous at bedtime
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Warfarin 5 mg PO daily (adjusted per INR)
- Atorvastatin 20 mg PO daily
- Acetaminophen 500 mg PO every 6 hours PRN for knee pain
- Polyethylene glycol 3350 17 g PO daily PRN for constipation

ALLERGIES:
ACE inhibitor-induced cough, Sulfa drugs

FAMILY HISTORY:
- Father with Type 2 Diabetes Mellitus and chronic kidney disease
- Mother with hypertension and hyperlipidemia
- Paternal grandfather with ischemic stroke

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, she has a full range of motion in her neck with tenderness over the cervical spine. Neurological examination reveals decreased pain and temperature sensation in a "cape-like" distribution over her shoulders and arms, with preserved proprioception. Motor examination shows 3/5 strength in the biceps and triceps, and 4/5 strength in the quadriceps and hamstrings. Reflexes are 2+ and symmetric in the lower extremities, but 1+ in the biceps and triceps tendon.

IMAGING:
An urgent cervical spine MRI shows no evidence of acute fracture but reveals cervical spinal stenosis with cord edema consistent with a central cord injury.
