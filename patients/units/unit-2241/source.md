# synthetic_hospital patient 2241

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-09-12 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2023-03-15 (Wed) | outpatient | Gastroenterology |
| 2 | 2021-01-15 | 2023-09-14 (Thu) | outpatient | Urology |
| 3 | 2021-09-15 | 2024-05-15 (Wed) | outpatient | Infectious Disease |
| 4 | 2022-03-15 | 2024-11-18 (Mon) | ed | Emergency Medicine |
| 5 | 2022-09-15 | 2025-05-13 (Tue) | inpatient | General Surgery |
| 6 | 2023-03-15 | 2025-11-15 (Sat) | inpatient | Vascular Surgery |
| 7 | 2023-09-15 | 2026-05-20 (Wed) | icu | Cardiothoracic Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "Occasional social use, 1-2 drinks per month",
 "family_history": [
  "Father: Myocardial infarction at age 68, hypertension",
  "Father: Prostate cancer diagnosed age 70",
  "Mother: Type 2 diabetes mellitus, stroke at age 72",
  "Brother: Hypertension, hyperlipidemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 10 years ago (30 pack-year history)",
 "home_medications": [
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "25 mg daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "0.4 mg daily at bedtime",
   "name": "Tamsulosin"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Cataract extraction, bilateral",
  "Laparoscopic inguinal herniorrhaphy"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus, poorly controlled",
  "Hypertension",
  "Hyperlipidemia",
  "Peripheral arterial disease",
  "Chronic kidney disease stage 3",
  "Benign prostatic hyperplasia"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Infected gangrene of lower extremity with nonrevascularizable critical limb ischemia", "Angiodysplasia of cecum", "Central retinal vein occlusion", "Osteomyelitis of metatarsal bone", "Acute cholecystitis", "Prostate Cancer (Suspected)", "Type A Aortic Dissection", "Essential hypertension"]
Comorbidities: ["Poorly controlled diabetes mellitus", "Anemia", "Chronic kidney disease stage 3b", "Peripheral arterial disease", "Essential hypertension", "Diabetic sensory polyneuropathy"]

## Encounter 0: source 2020-01-15, practice 2022-09-12, outpatient, Internal Medicine

Chief complaint: routine check-up for chronic hypertension management

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up for chronic hypertension management

HISTORY OF PRESENT ILLNESS:
He denies any symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus, poorly controlled
- Hypertension
- Hyperlipidemia
- Peripheral arterial disease
- Chronic kidney disease stage 3
- Benign prostatic hyperplasia

He has a history of type 2 diabetes mellitus and hypercholesterolemia.

PAST SURGICAL HISTORY:
- Cataract extraction, bilateral
- Laparoscopic inguinal herniorrhaphy

MEDICATIONS:
His medications include metformin and atorvastatin.

Home Medications:
- Hydrochlorothiazide 25 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Father: Prostate cancer diagnosed age 70
- Mother: Type 2 diabetes mellitus, stroke at age 72
- Brother: Hypertension, hyperlipidemia

SOCIAL HISTORY:
He quit smoking 10 years ago and has a sedentary lifestyle.

VITAL SIGNS:
His blood pressure is 162/98 mmHg on two separate occasions. His BMI is 32 kg/m^2.

PHYSICAL EXAMINATION:
His physical examination is unremarkable except for decreased vibratory sense in his feet.

LABORATORY DATA:
- Fasting glucose: 146 mg/dL
- HbA1c: 7.2%
- Total cholesterol: 210 mg/dL
- LDL cholesterol: 130 mg/dL
- HDL cholesterol: 40 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 1: source 2020-07-15, practice 2023-03-15, outpatient, Gastroenterology

Chief complaint: intermittent maroon-colored stools for the past 6 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent maroon-colored stools for the past 6 months

HISTORY OF PRESENT ILLNESS:
This 66-year-old male with poorly controlled Type 2 diabetes, hypertension, and chronic kidney disease returns six months after Internal Medicine visit. He presents with intermittent maroon-colored stools for the past 6 months and denies abdominal pain, weight loss, or changes in appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Type 2 diabetes mellitus, poorly controlled
- Hypertension
- Hyperlipidemia
- Peripheral arterial disease
- Chronic kidney disease stage 3
- Benign prostatic hyperplasia

His past medical history is significant for hypertension, for which he takes hydrochlorothiazide, and chronic kidney disease stage 3.

PAST SURGICAL HISTORY:
- Cataract extraction, bilateral
- Laparoscopic inguinal herniorrhaphy

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Metformin 1000 mg twice daily
- Atorvastatin 40 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Father: Prostate cancer diagnosed age 70
- Mother: Type 2 diabetes mellitus, stroke at age 72
- Brother: Hypertension, hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (30 pack-year history). Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired postal worker.

VITAL SIGNS:
Blood pressure is 145/90 mmHg, heart rate is 78 beats per minute

PHYSICAL EXAMINATION:
Abdominal examination is unremarkable. Rectal examination reveals maroon-colored stool in the rectal vault.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 92 fL (Normal: 80-96 fL)
- Blood urea nitrogen (BUN): 29 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 2.1 mg/dL (Normal: 0.6-1.3 mg/dL)

OTHER STUDIES:
Colonoscopy shows multiple small, flat, red lesions in the cecum.

## Encounter 2: source 2021-01-15, practice 2023-09-14, outpatient, Urology

Chief complaint: 6-month history of slowly progressive lower back pain worse at night

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Patel, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of slowly progressive lower back pain worse at night

HISTORY OF PRESENT ILLNESS:
This 67-year-old with diabetes, hypertension, BPH, and GI bleeding evaluation presents with lower back pain. He describes the pain as a constant, dull ache that is worse at night and does not improve with over-the-counter pain medications. He denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Angiodysplasia of cecum (diagnosed 2020-07-15)
- Type 2 diabetes mellitus, poorly controlled
- Hypertension
- Hyperlipidemia
- Peripheral arterial disease
- Chronic kidney disease stage 3
- Benign prostatic hyperplasia

His past medical history is significant for hypertension, controlled with hydrochlorothiazide, and benign prostatic hyperplasia.

PAST SURGICAL HISTORY:
- Cataract extraction, bilateral
- Laparoscopic inguinal herniorrhaphy

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Metformin 1000 mg twice daily
- Atorvastatin 40 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Father: Prostate cancer diagnosed age 70
- Mother: Type 2 diabetes mellitus, stroke at age 72
- Brother: Hypertension, hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (30 pack-year history). Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired postal worker.

PHYSICAL EXAMINATION:
On physical examination, there is tenderness to palpation over the lumbar spine, but no overlying skin changes or masses are noted. Neurological examination reveals no motor weakness or sensory deficits. Digital rectal examination shows an enlarged, firm, and nodular prostate.

LABORATORY DATA:
Serum calcium: 9.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum alkaline phosphatase: 140 U/L (normal: 40-130 U/L)
- Prostate-specific antigen (PSA): 22 ng/mL (normal: 0-4 ng/mL)
- Hemoglobin: 13.2 g/dL (normal: 13.8-17.2 g/dL)

IMAGING:
A lumbar spine X-ray demonstrates several focal sclerotic lesions.

## Encounter 3: source 2021-09-15, practice 2024-05-15, outpatient, Infectious Disease

Chief complaint: 3-week history of a non-healing ulcer on the plantar surface of the right foot

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. James O'Brien, MD
Department: Infectious Disease
Visit Type: outpatient

CHIEF COMPLAINT:
3-week history of a non-healing ulcer on the plantar surface of the right foot

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with poorly controlled type 2 diabetes mellitus and peripheral arterial disease, evaluated 8 months prior for suspected prostate cancer, presents for follow-up. He reports minimal pain, which he attributes to his peripheral neuropathy.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Angiodysplasia of cecum (diagnosed 2020-07-15)
- Prostate Cancer (Suspected) (diagnosed 2021-01-15)
- Type 2 diabetes mellitus, poorly controlled
- Hypertension
- Hyperlipidemia
- Peripheral arterial disease
- Chronic kidney disease stage 3
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Cataract extraction, bilateral
- Laparoscopic inguinal herniorrhaphy

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Father: Prostate cancer diagnosed age 70
- Mother: Type 2 diabetes mellitus, stroke at age 72
- Brother: Hypertension, hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (30 pack-year history). Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired postal worker.

VITAL SIGNS:
The patient's temperature is 37.2°C

PHYSICAL EXAMINATION:
On physical examination, the ulcer measures 2 cm in diameter, with a depth that allows for bone palpation with a sterile probe. There is minimal surrounding erythema and no purulent discharge. his peripheral pulses are palpable.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 7,500/µL
- Hemoglobin A1c: 9.2%
- Erythrocyte sedimentation rate: 45 mm/hr
- C-reactive protein: 3.2 mg/dL

IMAGING:
Plain radiographs of the foot show cortical irregularities of the metatarsal bones adjacent to the ulcer.

## Encounter 4: source 2022-03-15, practice 2024-11-18, ed, Emergency Medicine

Chief complaint: sudden onset of painless vision loss in left eye described as a curtain coming down

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of painless vision loss in left eye described as a curtain coming down

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with poorly controlled Type 2 diabetes mellitus, hypertension, chronic kidney disease stage 3, and peripheral arterial disease, with prior suspected prostate cancer and right foot osteomyelitis, presents with sudden painless vision loss. He describes it as a "curtain coming down" over his vision.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Angiodysplasia of cecum (diagnosed 2020-07-15)
- Prostate Cancer (Suspected) (diagnosed 2021-01-15)
- Osteomyelitis of metatarsal bone (diagnosed 2021-09-15)
- Type 2 diabetes mellitus, poorly controlled
- Hypertension
- Hyperlipidemia
- Peripheral arterial disease
- Chronic kidney disease stage 3
- Benign prostatic hyperplasia

He has a history of hypertension and type 2 diabetes mellitus, both of which are poorly controlled.

PAST SURGICAL HISTORY:
- Cataract extraction, bilateral
- Laparoscopic inguinal herniorrhaphy

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Father: Prostate cancer diagnosed age 70
- Mother: Type 2 diabetes mellitus, stroke at age 72
- Brother: Hypertension, hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (30 pack-year history). Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired postal worker.

VITAL SIGNS:
On physical examination, his blood pressure is 170/95 mmHg, and his heart rate is 78 beats per minute.

PHYSICAL EXAMINATION:
Funduscopic examination of the left eye reveals optic disk swelling, scattered retinal hemorrhages, dilated tortuous veins, and a few cotton wool spots. The right eye examination is unremarkable.

LABORATORY DATA:
His fasting blood glucose is 220 mg/dL, and his HbA1c is 9.2%.

## Encounter 5: source 2022-09-15, practice 2025-05-13, inpatient, General Surgery

Chief complaint: severe constant right upper quadrant pain radiating to right shoulder with fever and chills

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. David Kim, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Severe constant right upper quadrant pain radiating to right shoulder with fever and chills

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with poorly controlled diabetes and recent osteomyelitis and central retinal vein occlusion. He presents with a 2-day history of severe constant sharp right upper quadrant pain radiating to his right shoulder, accompanied by fever and chills.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Angiodysplasia of cecum (diagnosed 2020-07-15)
- Prostate Cancer (Suspected) (diagnosed 2021-01-15)
- Osteomyelitis of metatarsal bone (diagnosed 2021-09-15)
- Central retinal vein occlusion (diagnosed 2022-03-15)
- Type 2 diabetes mellitus, poorly controlled
- Hypertension
- Hyperlipidemia
- Peripheral arterial disease
- Chronic kidney disease stage 3
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Cataract extraction, bilateral
- Laparoscopic inguinal herniorrhaphy

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Father: Prostate cancer diagnosed age 70
- Mother: Type 2 diabetes mellitus, stroke at age 72
- Brother: Hypertension, hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (30 pack-year history). Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired postal worker.

VITAL SIGNS:
His temperature is 38.9°C (102°F), heart rate is 110 beats per minute, and blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
Abdominal examination reveals marked tenderness in the right upper quadrant with a positive Murphy's sign.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 18,000/mm³
- Hemoglobin: 13.2 g/dL
- Platelets: 250,000/mm³
- Serum bilirubin: 1.2 mg/dL
- Alkaline phosphatase: 120 U/L
- Aspartate aminotransferase (AST): 50 U/L
- Alanine aminotransferase (ALT): 45 U/L
- Blood glucose: 300 mg/dL

IMAGING:
An abdominal ultrasound shows gallbladder wall thickening and pericholecystic fluid. There is no evidence of gallstones. A plain abdominal radiograph reveals curvilinear areas of gas within the gallbladder wall.

## Encounter 6: source 2023-03-15, practice 2025-11-15, inpatient, Vascular Surgery

Chief complaint: 2-week history of progressive pain, swelling, and erythema of left lower extremity with foul-smelling heel ulcer discharge

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Maria Santos, MD
Department: Vascular Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
2-week history of progressive pain, swelling, and erythema of left lower extremity with foul-smelling heel ulcer discharge

HISTORY OF PRESENT ILLNESS:
This patient has poorly controlled diabetes, peripheral arterial disease, and prior right foot osteomyelitis. He reports foul-smelling discharge from his heel ulcer present for several months.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Angiodysplasia of cecum (diagnosed 2020-07-15)
- Prostate Cancer (Suspected) (diagnosed 2021-01-15)
- Osteomyelitis of metatarsal bone (diagnosed 2021-09-15)
- Central retinal vein occlusion (diagnosed 2022-03-15)
- Acute cholecystitis (diagnosed 2022-09-15)
- Type 2 diabetes mellitus, poorly controlled
- Hypertension
- Hyperlipidemia
- Peripheral arterial disease
- Chronic kidney disease stage 3
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Cataract extraction, bilateral
- Laparoscopic inguinal herniorrhaphy

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Father: Prostate cancer diagnosed age 70
- Mother: Type 2 diabetes mellitus, stroke at age 72
- Brother: Hypertension, hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (30 pack-year history). Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired postal worker.

VITAL SIGNS:
His temperature is 38.6°C (101.5°F), blood pressure is 150/90 mmHg, pulse is 88/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
Physical examination reveals a 3 cm × 4 cm ulcer on the left heel with black eschar and surrounding cellulitis. There is no palpable pulse in the left dorsalis pedis or posterior tibial arteries. Sensation is diminished in both lower extremities.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 18,000/mm³
- Hemoglobin: 12 g/dL
- Platelet count: 250,000/mm³
- Serum creatinine: 1.8 mg/dL
- C-reactive protein: 24 mg/L (normal <5 mg/L)

IMAGING:
Doppler ultrasound of the left lower extremity shows no flow in the anterior tibial, posterior tibial, or peroneal arteries.

## Encounter 7: source 2023-09-15, practice 2026-05-20, icu, Cardiothoracic Surgery

Chief complaint: sudden onset of severe tearing chest pain radiating to the back

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. Robert Thompson, MD
Department: Cardiothoracic Surgery
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of severe tearing chest pain radiating to the back

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with hypertension, diabetes, and peripheral arterial disease, hospitalized for limb ischemia, presents to the emergency department with sudden onset of severe, tearing chest pain radiating to the back. He describes the pain as the worst he has ever experienced.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Angiodysplasia of cecum (diagnosed 2020-07-15)
- Prostate Cancer (Suspected) (diagnosed 2021-01-15)
- Osteomyelitis of metatarsal bone (diagnosed 2021-09-15)
- Central retinal vein occlusion (diagnosed 2022-03-15)
- Acute cholecystitis (diagnosed 2022-09-15)
- Infected gangrene of lower extremity with nonrevascularizable critical limb ischemia (diagnosed 2023-03-15)
- Type 2 diabetes mellitus, poorly controlled
- Hypertension
- Hyperlipidemia
- Peripheral arterial disease
- Chronic kidney disease stage 3
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Cataract extraction, bilateral
- Laparoscopic inguinal herniorrhaphy

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily
- Tamsulosin 0.4 mg daily at bedtime
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Father: Prostate cancer diagnosed age 70
- Mother: Type 2 diabetes mellitus, stroke at age 72
- Brother: Hypertension, hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (30 pack-year history). Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired postal worker.

VITAL SIGNS:
His blood pressure is 150/90 mmHg, heart rate is 88 beats per minute, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals a diastolic murmur best heard along the right sternal border. His extremities are cool to the touch with diminished pulses in the lower extremities.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 14 g/dL
- White blood cell count: 9,000/mm^3
- Platelets: 250,000/mm^3
- Serum creatinine: 1.2 mg/dL
- Troponin I: <0.04 ng/mL (normal)

IMAGING:
A chest x-ray shows a widened mediastinum and an enlarged aortic knob.

OTHER STUDIES:
Electrocardiogram (ECG) shows left ventricular hypertrophy without ST or T wave changes.
