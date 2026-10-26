# synthetic_hospital patient 2224

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, African American, Medicare, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-11-25 (Mon) | outpatient | Nephrology |
| 1 | 2020-05-15 | 2020-03-09 (Mon) | outpatient | Nephrology |
| 2 | 2021-01-15 | 2020-11-09 (Mon) | outpatient | Infectious Disease |
| 3 | 2022-07-15 | 2022-05-07 (Sat) | ed | Emergency Medicine |
| 4 | 2023-01-15 | 2022-11-09 (Wed) | ed | Emergency Medicine |
| 5 | 2024-01-15 | 2023-11-23 (Thu) | ed | Emergency Medicine |
| 6 | 2025-07-15 | 2025-05-13 (Tue) | outpatient | Rheumatology |
| 7 | 2026-07-15 | 2026-05-18 (Mon) | outpatient | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare, Medicaid",
 "occupation": "Retired postal worker (disability)",
 "alcohol_use": "None",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Type 2 diabetes mellitus",
  "Sister: Hypertension"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Former smoker, 40 pack-years, quit 10 years ago",
 "home_medications": [
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "Sliding scale subcutaneous with meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "800 mg PO with meals",
   "name": "Sevelamer carbonate"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Left upper extremity arteriovenous fistula creation",
  "Right internal jugular dialysis catheter placement (historical)",
  "Left carpal tunnel release (historical)"
 ],
 "chronic_conditions": [
  "End-stage renal disease on hemodialysis",
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Anemia of chronic kidney disease"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Dialysis-related amyloidosis", "Sepsis secondary to vascular access-related bacteremia", "Anemia of chronic kidney disease", "Acute pulmonary edema", "Tuberculosis", "Severe hypoglycemia", "Normal pressure hydrocephalus"]
Comorbidities: ["End-stage renal disease on hemodialysis", "Anemia of chronic kidney disease", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2019-11-25, outpatient, Nephrology

Chief complaint: Persistent fatigue and decreased exercise tolerance

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue and decreased exercise tolerance

PAST MEDICAL HISTORY:
Active Problem List:
- End-stage renal disease on hemodialysis
- Type 2 diabetes mellitus
- Hypertension
- Anemia of chronic kidney disease

He has been on hemodialysis for the past year due to end-stage renal disease.

PAST SURGICAL HISTORY:
- Left upper extremity arteriovenous fistula creation
- Right internal jugular dialysis catheter placement (historical)
- Left carpal tunnel release (historical)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals
- Lisinopril 5 mg PO daily
- Sevelamer carbonate 800 mg PO with meals
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired postal worker (disability).

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory testing shows: hemoglobin 9.0 g/dL, mean corpuscular volume 88 fL, leukocyte count 7,500/μL, and platelet count 250,000/μL. Peripheral smear shows normocytic, normochromic red blood cells. Reticulocyte count is low.

## Encounter 1: source 2020-05-15, practice 2020-03-09, outpatient, Nephrology

Chief complaint: Increasing fatigue over the past few months despite treatment

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Marcus Johnson, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue over the past few months despite treatment

HISTORY OF PRESENT ILLNESS:
The patient with end-stage renal disease on hemodialysis returns for follow-up of fatigue initially assessed in January 2020. He reports feeling tired even after adequate sleep with reduced exercise tolerance, denying any bleeding, melena, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Anemia of chronic kidney disease (diagnosed 2020-01-15)
- End-stage renal disease on hemodialysis
- Type 2 diabetes mellitus
- Hypertension

He has a history of hypertension and type 2 diabetes mellitus with poor glycemic control.

PAST SURGICAL HISTORY:
- Left upper extremity arteriovenous fistula creation
- Right internal jugular dialysis catheter placement (historical)
- Left carpal tunnel release (historical)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals
- Lisinopril 5 mg PO daily
- Sevelamer carbonate 800 mg PO with meals
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired postal worker (disability).

VITAL SIGNS:
His blood pressure today is 146/88 mmHg.

PHYSICAL EXAMINATION:
Physical examination shows pale conjunctivae but is otherwise unremarkable.
Neurologic exam is normal.

LABORATORY DATA:
Recent creatinine was 3.1 mg/dL (baseline 2.8 mg/dL). Laboratory results:
Hemoglobin 10.2 g/dL
Hematocrit 31%
MCV													 86 fL
Serum ferritin											 220 ng/mL
Serum iron												 35 µg/dL

## Encounter 2: source 2021-01-15, practice 2020-11-09, outpatient, Infectious Disease

Chief complaint: Progressive fatigue, unintentional weight loss of 10 pounds, and anorexia

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Elena Vasquez, MD
Department: Infectious Disease
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, unintentional weight loss of 10 pounds, and anorexia

HISTORY OF PRESENT ILLNESS:
The patient with ESRD on hemodialysis and prior anemia workup presents with a 2-month history of progressive fatigue, unintentional 10-pound weight loss, and anorexia. He denies cough, fever, or night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Anemia of chronic kidney disease (diagnosed 2020-01-15)
- End-stage renal disease on hemodialysis
- Type 2 diabetes mellitus
- Hypertension

PAST SURGICAL HISTORY:
- Left upper extremity arteriovenous fistula creation
- Right internal jugular dialysis catheter placement (historical)
- Left carpal tunnel release (historical)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals
- Lisinopril 5 mg PO daily
- Sevelamer carbonate 800 mg PO with meals
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
40-pack-year smoking history, although he quit smoking 10 years ago

VITAL SIGNS:
Temperature is 37.2°C (99°F), blood pressure is 150/90 mmHg, pulse is 88/min, and respiratory rate is 16/min

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds at the right apex, but no crackles or wheezes are heard. The remainder of the examination is unremarkable.

LABORATORY DATA:
Normal complete blood count and serum creatinine consistent with his baseline renal impairment

IMAGING:
Chest x-ray demonstrates a right apical infiltrate without cavitation

## Encounter 3: source 2022-07-15, practice 2022-05-07, ed, Emergency Medicine

Chief complaint: Dizziness and palpitations during morning walks

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Dizziness and palpitations during morning walks

HISTORY OF PRESENT ILLNESS:
The 70-year-old male with ESRD on hemodialysis, T2DM, and history of anemia and TB presents to the clinic with complaints of dizziness and palpitations during morning walks. He reports these symptoms have been occurring more frequently over the past two weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Anemia of chronic kidney disease (diagnosed 2020-01-15)
- Tuberculosis (diagnosed 2021-01-15)
- End-stage renal disease on hemodialysis
- Type 2 diabetes mellitus
- Hypertension

PAST SURGICAL HISTORY:
- Left upper extremity arteriovenous fistula creation
- Right internal jugular dialysis catheter placement (historical)
- Left carpal tunnel release (historical)

MEDICATIONS:
He is currently on a regimen of basal-bolus insulin therapy. His other medications include metformin and lisinopril.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals
- Sevelamer carbonate 800 mg PO with meals
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired postal worker (disability).

VITAL SIGNS:
His blood pressure is 130/80 mmHg, heart rate is 98 bpm

PHYSICAL EXAMINATION:
On examination, he appears diaphoretic

LABORATORY DATA:
His capillary blood glucose level at the time of the symptoms is 60 mg/dL. Laboratory values are as follows:

- Serum creatinine: 2.1 mg/dL (elevated)
- Estimated GFR: 35 mL/min/1.73m^2 (decreased)
- Hemoglobin A1c: 7.2%
- Urinalysis: No proteinuria or hematuria

## Encounter 4: source 2023-01-15, practice 2022-11-09, ed, Emergency Medicine

Chief complaint: Worsening shortness of breath and orthopnea requiring sleeping upright

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Sharma, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening shortness of breath and orthopnea requiring sleeping upright

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with end-stage renal disease on hemodialysis, type 2 diabetes mellitus, hypertension, anemia of chronic kidney disease, and history of tuberculosis and severe hypoglycemia presents to the emergency department with worsening shortness of breath that began two days ago and has progressively worsened. He now struggles to walk across the room without becoming winded and has been sleeping upright in a recliner due to severe breathlessness when lying flat. He also reports a 5-pound weight gain and reduced urine output over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Anemia of chronic kidney disease (diagnosed 2020-01-15)
- Tuberculosis (diagnosed 2021-01-15)
- Severe hypoglycemia (diagnosed 2022-07-15)
- End-stage renal disease on hemodialysis
- Type 2 diabetes mellitus
- Hypertension

His medical history includes end-stage renal disease on hemodialysis, hypertension, and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Left upper extremity arteriovenous fistula creation
- Right internal jugular dialysis catheter placement (historical)
- Left carpal tunnel release (historical)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals
- Lisinopril 5 mg PO daily
- Sevelamer carbonate 800 mg PO with meals
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
He missed his last two scheduled dialysis sessions due to transportation issues.

VITAL SIGNS:
Temperature is 36.8°C (98.2°F), blood pressure is 172/94 mm Hg, pulse is 106/min, and respiratory rate is 28/min.

PHYSICAL EXAMINATION:
On exam, he appears visibly uncomfortable and dyspneic.
Jugular venous distension is noted 10 cm above the sternal angle. Lung auscultation reveals diffuse bilateral crackles. Cardiac exam reveals a third heart sound. There is moderate bilateral pitting edema in the lower extremities.

LABORATORY DATA:
Arterial blood gas shows hypoxemia with a widened A–a gradient.

IMAGING:
Chest x-ray shows an enlarged cardiac silhouette and bilateral perihilar alveolar infiltrates.

OTHER STUDIES:
ECG shows left ventricular hypertrophy and sinus tachycardia.

PLAN:
He is placed on oxygen and started on intravenous furosemide.

## Encounter 5: source 2024-01-15, practice 2023-11-23, ed, Emergency Medicine

Chief complaint: Fever and chills for two days following hemodialysis session

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever and chills for two days following hemodialysis session

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with end-stage renal disease on hemodialysis, prior tuberculosis, and pulmonary edema reports symptoms started a few hours after his last dialysis session, denying cough, dysuria, or any other localized symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Anemia of chronic kidney disease (diagnosed 2020-01-15)
- Tuberculosis (diagnosed 2021-01-15)
- Severe hypoglycemia (diagnosed 2022-07-15)
- Acute pulmonary edema (diagnosed 2023-01-15)
- End-stage renal disease on hemodialysis
- Type 2 diabetes mellitus
- Hypertension

His past medical history is significant for diabetes mellitus type 2 and hypertension.

PAST SURGICAL HISTORY:
- Left upper extremity arteriovenous fistula creation
- Right internal jugular dialysis catheter placement (historical)
- Left carpal tunnel release (historical)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals
- Lisinopril 5 mg PO daily
- Sevelamer carbonate 800 mg PO with meals
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired postal worker (disability).

VITAL SIGNS:
On examination, his temperature is 38.7°C (101.7°F), blood pressure is 95/60 mmHg, heart rate is 102 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
His physical examination is notable for a well-healed arteriovenous fistula in his left arm without signs of erythema, warmth, or tenderness.

LABORATORY DATA:
Laboratory studies are as follows:

- White blood cell count: 12,000/mm³ (normal 4,500-11,000/mm³)
- Hemoglobin: 10.2 g/dL (normal 13.8-17.2 g/dL)
- Platelet count: 220,000/mm³ (normal 150,000-450,000/mm³)
- Blood urea nitrogen: 45 mg/dL (normal 7-20 mg/dL)
- Creatinine: 5.6 mg/dL (baseline 5.2 mg/dL)
- Blood cultures: Pending

## Encounter 6: source 2025-07-15, practice 2025-05-13, outpatient, Rheumatology

Chief complaint: Six-month history of worsening bilateral shoulder pain, stiffness, and hand numbness

ENCOUNTER NOTE
Date: 2025-07-15
Provider: Dr. Angela Washington, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of worsening bilateral shoulder pain, stiffness, and hand numbness

HISTORY OF PRESENT ILLNESS:
The dialysis-dependent patient with prior access bacteremia and anemia presents with upper extremity symptoms. He reports difficulty raising his arms above his head and has noticed swelling around his shoulders. He also complains of numbness and tingling in his hands, particularly at night.

PAST MEDICAL HISTORY:
Active Problem List:
- Anemia of chronic kidney disease (diagnosed 2020-01-15)
- Tuberculosis (diagnosed 2021-01-15)
- Severe hypoglycemia (diagnosed 2022-07-15)
- Acute pulmonary edema (diagnosed 2023-01-15)
- Sepsis secondary to vascular access-related bacteremia (diagnosed 2024-01-15)
- End-stage renal disease on hemodialysis
- Type 2 diabetes mellitus
- Hypertension

His past medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Left upper extremity arteriovenous fistula creation
- Right internal jugular dialysis catheter placement (historical)
- Left carpal tunnel release (historical)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals
- Lisinopril 5 mg PO daily
- Sevelamer carbonate 800 mg PO with meals
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired postal worker (disability).

PHYSICAL EXAMINATION:
On physical examination, there is bilateral shoulder joint hypertrophy with tenderness to palpation. Range of motion is limited due to pain. A neurologic examination reveals decreased sensation in the distribution of the median nerve bilaterally.

LABORATORY DATA:
Serum creatinine: 6.5 mg/dL
Blood urea nitrogen (BUN): 55 mg/dL
Hemoglobin: 10.2 g/dL
Calcium: 9.2 mg/dL
Phosphorus: 5.0 mg/dL
β2-microglobulin: Elevated

IMAGING:
Radiographs of the shoulders show subchondral bone cysts and erosions.

## Encounter 7: source 2026-07-15, practice 2026-05-18, outpatient, Neurology

Chief complaint: Progressive difficulty walking with magnetic gait and short shuffling steps

ENCOUNTER NOTE
Date: 2026-07-15
Provider: Dr. David Cohen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty walking with magnetic gait and short shuffling steps

HISTORY OF PRESENT ILLNESS:
The patient is a 74-year-old male with end-stage renal disease on maintenance hemodialysis, type 2 diabetes mellitus, and hypertension, last evaluated for dialysis-related amyloidosis one year ago, who presents to the neurology clinic for assessment of gait changes. He is accompanied by his daughter, who reports progressive difficulty walking over the past 6 months. She describes his gait as "magnetic," with short, shuffling steps and difficulty initiating movement. Additionally, she notes that he has become increasingly forgetful and has had episodes of urinary incontinence.

PAST MEDICAL HISTORY:
Active Problem List:
- Anemia of chronic kidney disease (diagnosed 2020-01-15)
- Tuberculosis (diagnosed 2021-01-15)
- Severe hypoglycemia (diagnosed 2022-07-15)
- Acute pulmonary edema (diagnosed 2023-01-15)
- Sepsis secondary to vascular access-related bacteremia (diagnosed 2024-01-15)
- Dialysis-related amyloidosis (diagnosed 2025-07-15)
- End-stage renal disease on hemodialysis
- Type 2 diabetes mellitus
- Hypertension

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Left upper extremity arteriovenous fistula creation
- Right internal jugular dialysis catheter placement (historical)
- Left carpal tunnel release (historical)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals
- Lisinopril 5 mg PO daily
- Sevelamer carbonate 800 mg PO with meals
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired postal worker (disability).

PHYSICAL EXAMINATION:
On examination, the patient is alert but oriented only to person and place. Cranial nerves are intact, and strength is 5/5 in all extremities. Reflexes are 2+ and symmetric.

IMAGING:
A CT scan of the head shows enlarged ventricles with no evidence of cortical atrophy.

OTHER STUDIES:
Lumbar puncture reveals an opening pressure within normal limits, and cerebrospinal fluid analysis is unremarkable.
