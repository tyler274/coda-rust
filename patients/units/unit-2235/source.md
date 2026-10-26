# synthetic_hospital patient 2235

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-05-11 (Tue) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2021-11-01 (Mon) | outpatient | Neurology |
| 2 | 2021-01-15 | 2022-05-02 (Mon) | outpatient | Orthopedics |
| 3 | 2021-07-15 | 2022-10-31 (Mon) | outpatient | Neurology |
| 4 | 2022-01-15 | 2023-04-27 (Thu) | outpatient | Hematology & Oncology |
| 5 | 2022-07-15 | 2023-11-02 (Thu) | outpatient | Oncology |
| 6 | 2024-01-15 | 2025-04-25 (Fri) | ed | Emergency Medicine |
| 7 | 2024-07-15 | 2025-11-03 (Mon) | outpatient | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Father: Type 2 diabetes mellitus, prostate cancer",
  "Mother: Hypertension, hyperlipidemia",
  "Maternal uncle: Chronic lymphocytic leukemia"
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
   "dose": "12.5 mg PO daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "Cataract extraction, right eye (2020)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (20-year duration)",
  "Diabetic retinopathy",
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["REM Sleep Behavior Disorder", "Diabetic foot ulcer in proliferative phase of healing", "Thoracic spinal stenosis", "Metastatic carcinoma to bone", "Chronic lymphocytic leukemia", "Uncomplicated parapneumonic effusion", "Creutzfeldt-Jakob disease"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-05-11, outpatient, Internal Medicine

Chief complaint: Foul odor and fluid leakage from right foot during routine diabetes follow-up

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Foul odor and fluid leakage from right foot during routine diabetes follow-up

HISTORY OF PRESENT ILLNESS:
He reports that over the past few weeks, he has noticed a foul smell from his right foot and that it occasionally leaks fluid. He denies any fever or pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (20-year duration)
- Diabetic retinopathy
- Hypertension
- Hyperlipidemia

His medical history is significant for hypertension and diabetic retinopathy.

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (2020)

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, prostate cancer
- Mother: Hypertension, hyperlipidemia
- Maternal uncle: Chronic lymphocytic leukemia

SOCIAL HISTORY:
He wears sandals at home and occasionally walks barefoot indoors.

VITAL SIGNS:
Temperature is 37.1°C, Blood pressure is 142/88 mmHg, Pulse is 84/min, and Respiratory rate 14/min.

PHYSICAL EXAMINATION:
Physical examination reveals a shallow ulcer on the plantar surface of the right foot surrounded by callused skin and minimal erythema. Sensation is diminished in both feet.

LABORATORY DATA:
Labs: HbA1c is 8.9%, WBC is 8,400/mm³.

## Encounter 1: source 2020-07-15, practice 2021-11-01, outpatient, Neurology

Chief complaint: Episodes of acting out dreams while asleep with bed falls for 6 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodes of acting out dreams while asleep with bed falls for 6 months

HISTORY OF PRESENT ILLNESS:
The patient with 20-year Type 2 Diabetes Mellitus, diabetic retinopathy, hypertension, and hyperlipidemia presents six months after last visit for diabetic foot ulcer. She describes instances where he has been seen punching and kicking in bed, which sometimes results in him falling off the bed. The patient is embarrassed and admits to vivid dreams that correlate with the movements. He denies any sleep problems prior to these episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (20-year duration)
- Diabetic retinopathy
- Hypertension
- Hyperlipidemia

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (2020)

MEDICATIONS:
He is currently taking metformin and lisinopril.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, prostate cancer
- Mother: Hypertension, hyperlipidemia
- Maternal uncle: Chronic lymphocytic leukemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, the patient is alert and oriented. Neurological examination is unremarkable, with no evidence of rigidity, tremor, or bradykinesia. His muscle tone is normal while awake.

OTHER STUDIES:
Polysomnography is performed and shows increased muscle activity during REM sleep.

## Encounter 2: source 2021-01-15, practice 2022-05-02, outpatient, Orthopedics

Chief complaint: Lower back pain radiating to thighs exacerbated by walking

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Jennifer Park, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Lower back pain radiating to thighs exacerbated by walking

HISTORY OF PRESENT ILLNESS:
This 66-year-old male with Type 2 Diabetes Mellitus (20-year duration), diabetic retinopathy, hypertension, hyperlipidemia, diabetic foot ulcer, and REM Sleep Behavior Disorder presents to the clinic with a 6-month history of lower back pain that radiates to his thighs. He reports that the pain is exacerbated when he walks for more than a few minutes and improves when he leans forward or sits down. He denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-01-15)
- REM Sleep Behavior Disorder (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus (20-year duration)
- Diabetic retinopathy
- Hypertension
- Hyperlipidemia

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (2020)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, prostate cancer
- Mother: Hypertension, hyperlipidemia
- Maternal uncle: Chronic lymphocytic leukemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, lumbar extension elicits pain, and there is no loss of sensation or motor function in the lower extremities. Straight leg raise test is negative bilaterally. Vascular examination of the lower extremities is normal.

IMAGING:
An MRI of the lumbar spine reveals narrowing of the spinal canal and thickening of the ligamentum flavum.

## Encounter 3: source 2021-07-15, practice 2022-10-31, outpatient, Neurology

Chief complaint: Worsening sleep disturbances with dream enactment behavior and minor injuries

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening sleep disturbances with dream enactment behavior and minor injuries

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of sleep disturbances initially evaluated in July 2020, with longstanding Type 2 Diabetes Mellitus. She describes instances where he appears to be punching or kicking in his sleep, sometimes falling out of bed, which has resulted in minor injuries. The patient is unaware of these episodes but does recall vivid, action-packed dreams.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-01-15)
- REM Sleep Behavior Disorder (diagnosed 2020-07-15)
- Thoracic spinal stenosis (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus (20-year duration)
- Diabetic retinopathy
- Hypertension
- Hyperlipidemia

His past medical history is significant for mild, untreated hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (2020)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, prostate cancer
- Mother: Hypertension, hyperlipidemia
- Maternal uncle: Chronic lymphocytic leukemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, the patient is alert and oriented with normal cognitive function. His neurological examination is unremarkable, with no evidence of rigidity, bradykinesia, or tremor.

OTHER STUDIES:
Polysomnography shows increased muscle activity during REM sleep.

## Encounter 4: source 2022-01-15, practice 2023-04-27, outpatient, Hematology & Oncology

Chief complaint: Fatigue, night sweats, and 10-pound unintentional weight loss

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Aisha Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, night sweats, and 10-pound unintentional weight loss

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with Type 2 Diabetes Mellitus, diabetic retinopathy, and a diabetic foot ulcer presenting to the Hematology and Oncology clinic with a 2-month history of fatigue, night sweats, and a 10-pound unintentional weight loss. He reports a feeling of fullness in his abdomen and has noticed his appetite has decreased.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-01-15)
- REM Sleep Behavior Disorder (diagnosed 2020-07-15)
- Thoracic spinal stenosis (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus (20-year duration)
- Diabetic retinopathy
- Hypertension
- Hyperlipidemia

His past medical history is significant for hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus managed with metformin.

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (2020)

MEDICATIONS:
He takes hydrochlorothiazide, metformin.

Home Medications:
- Lisinopril 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, prostate cancer
- Mother: Hypertension, hyperlipidemia
- Maternal uncle: Chronic lymphocytic leukemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, he appears pale and has palpable bilateral cervical and axillary lymphadenopathy. His spleen is palpable 3 cm below the left costal margin.

LABORATORY DATA:
Laboratory findings are as follows:
- Hemoglobin: 9.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 85 fL (Normal: 80-96 fL)
- White blood cell count (WBC): 14,000 /µL (Normal: 4,500-11,000 /µL)
- Platelet count: 150,000 /µL (Normal: 150,000-450,000 /µL)
- Lactate dehydrogenase (LDH): Elevated
- Peripheral blood smear: Smudge cells present

## Encounter 5: source 2022-07-15, practice 2023-11-02, outpatient, Oncology

Chief complaint: Progressive low back pain worse at night with unintentional weight loss

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Okafor, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive low back pain worse at night with unintentional weight loss

HISTORY OF PRESENT ILLNESS:
The patient with Diabetes Mellitus returns to Oncology six months after prior evaluation for constitutional symptoms, presenting with back pain. He reports no recent trauma or injury. The pain is described as a constant, dull ache that does not improve with rest and is more severe at night. He also mentions unintentional weight loss of 10 pounds over the last two months and occasional night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-01-15)
- REM Sleep Behavior Disorder (diagnosed 2020-07-15)
- Thoracic spinal stenosis (diagnosed 2021-01-15)
- Chronic lymphocytic leukemia (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus (20-year duration)
- Diabetic retinopathy
- Hypertension
- Hyperlipidemia

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (2020)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, prostate cancer
- Mother: Hypertension, hyperlipidemia
- Maternal uncle: Chronic lymphocytic leukemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, there is tenderness to palpation over the lumbar spine, but no obvious deformity. Straight leg raise test is negative bilaterally. Neurological examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 12.5 g/dL (normal: 13.8-17.2 g/dL)
- Erythrocyte sedimentation rate (ESR): 70 mm/hr (normal: 0-22 mm/hr)
- Calcium: 10.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Alkaline phosphatase: 120 U/L (normal: 40-130 U/L)

ASSESSMENT AND PLAN:
Given the patient's clinical presentation and laboratory findings,

## Encounter 6: source 2024-01-15, practice 2025-04-25, ed, Emergency Medicine

Chief complaint: Progressive dyspnea on exertion and dry cough for 3 weeks

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive dyspnea on exertion and dry cough for 3 weeks

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with type 2 diabetes mellitus, chronic lymphocytic leukemia, and metastatic bone disease presents with progressive dyspnea on exertion and dry cough for three weeks. He denies chest pain, fever, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-01-15)
- REM Sleep Behavior Disorder (diagnosed 2020-07-15)
- Thoracic spinal stenosis (diagnosed 2021-01-15)
- Chronic lymphocytic leukemia (diagnosed 2022-01-15)
- Metastatic carcinoma to bone (diagnosed 2022-07-15)
- Type 2 Diabetes Mellitus (20-year duration)
- Diabetic retinopathy
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (2020)

MEDICATIONS:
Metformin, lisinopril, and hydrochlorothiazide

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, prostate cancer
- Mother: Hypertension, hyperlipidemia
- Maternal uncle: Chronic lymphocytic leukemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Blood pressure is 145/90 mmHg, heart rate is 88 beats per minute, respiratory rate is 20 breaths per minute, and oxygen saturation is 94% on room air

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals decreased breath sounds at the right lung base. There is no peripheral edema, jugular venous distention, or cardiac murmurs.

LABORATORY DATA:
Serum creatinine: 1.2 mg/dL
- Blood urea nitrogen: 18 mg/dL
- Serum albumin: 4.0 g/dL
- Total protein: 7.2 g/dL
- Serum glucose: 160 mg/dL

IMAGING:
A chest X-ray shows a moderate-sized right pleural effusion without cardiomegaly or pulmonary edema

## Encounter 7: source 2024-07-15, practice 2025-11-03, outpatient, Neurology

Chief complaint: Rapidly progressive memory loss, confusion, and sudden muscle jerks

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Rapidly progressive memory loss, confusion, and sudden muscle jerks

HISTORY OF PRESENT ILLNESS:
The patient returns with worsening neuropsychiatric symptoms in the setting of longstanding REM Sleep Behavior Disorder and diabetes mellitus. His wife reports that he has had several episodes of sudden muscle jerks and has become increasingly irritable and withdrawn. She also mentions that he seems to be sleeping more than usual and has difficulty staying awake during the day.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2020-01-15)
- REM Sleep Behavior Disorder (diagnosed 2020-07-15)
- Thoracic spinal stenosis (diagnosed 2021-01-15)
- Chronic lymphocytic leukemia (diagnosed 2022-01-15)
- Metastatic carcinoma to bone (diagnosed 2022-07-15)
- Uncomplicated parapneumonic effusion (diagnosed 2024-01-15)
- Type 2 Diabetes Mellitus (20-year duration)
- Diabetic retinopathy
- Hypertension
- Hyperlipidemia

His past medical history is significant for hypertension, controlled with lisinopril, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Cataract extraction, right eye (2020)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, prostate cancer
- Mother: Hypertension, hyperlipidemia
- Maternal uncle: Chronic lymphocytic leukemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, the patient appears disoriented to time and place. Neurological examination reveals generalized myoclonus.

LABORATORY DATA:
Routine blood tests are normal. A lumbar puncture is performed, and the cerebrospinal fluid analysis reveals:

- WBC: 3 /mm^3 (normal: 0-5 /mm^3)
- Protein: 45 mg/dL (normal: 15-45 mg/dL)
- Glucose: 60 mg/dL (normal: 45-80 mg/dL)
- 14-3-3 protein: Positive

OTHER STUDIES:
An EEG shows generalized periodic sharp wave complexes.
