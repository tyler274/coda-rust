# synthetic_hospital patient 2836

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex F, Non-Hispanic White, Medicare Part A and B.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-10-02 (Wed) | outpatient | Endocrinology |
| 1 | 2020-03-15 | 2019-12-06 (Fri) | outpatient | Rheumatology |
| 2 | 2021-03-15 | 2020-12-10 (Thu) | outpatient | Hematology & Oncology |
| 3 | 2022-03-15 | 2021-12-13 (Mon) | outpatient | Internal Medicine |
| 4 | 2023-03-15 | 2022-11-28 (Mon) | ed | Emergency Medicine |
| 5 | 2024-07-15 | 2024-04-12 (Fri) | ed | Emergency Medicine |
| 6 | 2024-08-15 | 2024-05-11 (Sat) | icu | Emergency Medicine |
| 7 | 2026-01-15 | 2025-10-13 (Mon) | outpatient | Palliative Care |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin (anaphylaxis)",
  "Iodinated contrast (urticaria)"
 ],
 "insurance": "Medicare Part A and B",
 "occupation": "Retired high school librarian",
 "alcohol_use": "Occasional social use (1-2 glasses of wine per week)",
 "family_history": [
  "Mother: Invasive breast cancer (diagnosed age 62, deceased)",
  "Father: Hypertension, chronic gout, recurrent kidney stones",
  "Sister: Severe osteoporosis with compression fractures"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 20 years ago (10 pack-years)",
 "home_medications": [
  {
   "dose": "2.5 mg PO daily",
   "name": "Letrozole"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "650 mg PO q6h PRN pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "30 mg PO every 12 hours",
   "name": "Morphine sulfate sustained-release"
  },
  {
   "dose": "300 mg PO TID",
   "name": "Gabapentin"
  },
  {
   "dose": "4 mg PO daily",
   "name": "Dexamethasone"
  }
 ],
 "surgical_history": [
  "Left modified radical mastectomy with sentinel lymph node biopsy (5 years prior)",
  "Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 50)",
  "Laparoscopic cholecystectomy (age 45)"
 ],
 "chronic_conditions": [
  "Invasive ductal carcinoma of the breast, Stage IV with bone metastases",
  "Primary hyperparathyroidism",
  "Hypertension",
  "Hyperlipidemia",
  "Osteoarthritis, degenerative",
  "Osteoporosis",
  "Nephrolithiasis (calcium oxalate)",
  "Calcium pyrophosphate deposition disease (CPPD)"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Malignancy-associated hypercalcemia", "Metastatic carcinoma to bone", "Suicidal ideation", "Giant cell arteritis or autoimmune inflammatory condition", "Pseudogout (Acute Calcium Pyrophosphate Dihydrate Crystal Deposition Disease)", "Acute gouty arthritis (drug-induced)"]
Comorbidities: ["History of breast cancer", "Primary hyperparathyroidism", "Essential hypertension", "Malignancy-associated hypercalcemia", "Long term (current) use of immunosuppressants", "Metastatic carcinoma to bone"]

## Encounter 0: source 2020-01-15, practice 2019-10-02, outpatient, Endocrinology

Chief complaint: Excessive thirst and frequent urination for several months with generalized fatigue

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Excessive thirst and frequent urination for several months with generalized fatigue

HISTORY OF PRESENT ILLNESS:
She drinks large amounts of water throughout the day and wakes multiple times at night to urinate. She also reports generalized fatigue and mild muscle weakness.

PAST MEDICAL HISTORY:
Active Problem List:
- Invasive ductal carcinoma of the breast, Stage IV with bone metastases
- Primary hyperparathyroidism
- Hypertension
- Hyperlipidemia
- Osteoarthritis, degenerative
- Osteoporosis
- Nephrolithiasis (calcium oxalate)
- Calcium pyrophosphate deposition disease (CPPD)

Her medical history includes hypertension, osteoarthritis, and she was recently diagnosed with primary hyperparathyroidism.

PAST SURGICAL HISTORY:
- Left modified radical mastectomy with sentinel lymph node biopsy (5 years prior)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 50)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Medications include hydrochlorothiazide and acetaminophen.

Home Medications:
- Letrozole 2.5 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 20 mg PO daily
- Morphine sulfate sustained-release 30 mg PO every 12 hours
- Gabapentin 300 mg PO TID
- Dexamethasone 4 mg PO daily

ALLERGIES:
Penicillin (anaphylaxis), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Mother: Invasive breast cancer (diagnosed age 62, deceased)
- Father: Hypertension, chronic gout, recurrent kidney stones
- Sister: Severe osteoporosis with compression fractures

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (10 pack-years). Alcohol: Occasional social use (1-2 glasses of wine per week). Occupation: Retired high school librarian.

VITAL SIGNS:
On examination, her blood pressure is 138/82 mmHg and pulse is 80/min.

PHYSICAL EXAMINATION:
She appears thin but in no acute distress. Neurologic examination is normal.

LABORATORY DATA:
Laboratory studies reveal:
Serum sodium 148 mEq/L
Serum calcium 11.4 mg/dL
Serum osmolality 305 mOsm/kg
Urine osmolality 120 mOsm/kg

OTHER STUDIES:
A water deprivation test shows no significant change in urine osmolality, and subsequent desmopressin administration fails to concentrate the urine.

## Encounter 1: source 2020-03-15, practice 2019-12-06, outpatient, Rheumatology

Chief complaint: Severe pain and swelling in left knee for 2 days with chronic constipation and polyuria

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Jamal Washington, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Severe pain and swelling in left knee for 2 days with chronic constipation and polyuria

HISTORY OF PRESENT ILLNESS:
Patient with metastatic breast cancer returns following Endocrinology care. She denies any recent trauma or injury. She has been experiencing polyuria and constipation for the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignancy-associated hypercalcemia (diagnosed 2020-01-15)
- Invasive ductal carcinoma of the breast, Stage IV with bone metastases
- Primary hyperparathyroidism
- Hypertension
- Hyperlipidemia
- Osteoarthritis, degenerative
- Osteoporosis
- Nephrolithiasis (calcium oxalate)
- Calcium pyrophosphate deposition disease (CPPD)

Her medical history is significant for osteoporosis and kidney stones.

PAST SURGICAL HISTORY:
- Left modified radical mastectomy with sentinel lymph node biopsy (5 years prior)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 50)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Letrozole 2.5 mg PO daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 20 mg PO daily
- Acetaminophen 650 mg PO q6h PRN pain
- Morphine sulfate sustained-release 30 mg PO every 12 hours
- Gabapentin 300 mg PO TID
- Dexamethasone 4 mg PO daily

ALLERGIES:
Penicillin (anaphylaxis), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Mother: Invasive breast cancer (diagnosed age 62, deceased)
- Father: Hypertension, chronic gout, recurrent kidney stones
- Sister: Severe osteoporosis with compression fractures

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (10 pack-years). Alcohol: Occasional social use (1-2 glasses of wine per week). Occupation: Retired high school librarian.

PHYSICAL EXAMINATION:
On physical examination, her left knee is warm, swollen, and tender to palpation. There is a decreased range of motion due to pain.

LABORATORY DATA:
Laboratory findings are as follows:

- Serum calcium: 11.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum phosphorus: 2.9 mg/dL (normal: 2.5-4.5 mg/dL)
- Serum parathyroid hormone (PTH): Elevated
- Serum creatinine: 1.2 mg/dL (normal: 0.6-1.1 mg/dL)
- Erythrocyte sedimentation rate (ESR): 30 mm/hr (normal: 0-20 mm/hr)

IMAGING:
A knee x-ray shows chondrocalcinosis of the articular cartilage.

OTHER STUDIES:
Arthrocentesis is performed, and the synovial fluid analysis reveals rhomboid-shaped, positively birefringent crystals.

## Encounter 2: source 2021-03-15, practice 2020-12-10, outpatient, Hematology & Oncology

Chief complaint: Persistent lower back pain worse at night for 2 months

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Elena Vasquez, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent lower back pain worse at night for 2 months

HISTORY OF PRESENT ILLNESS:
The patient with Stage IV breast cancer, hypercalcemia, and pseudogout presents with a 2-month history of persistent lower back pain that is worse at night and does not improve with over-the-counter analgesics. She denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignancy-associated hypercalcemia (diagnosed 2020-01-15)
- Pseudogout (Acute Calcium Pyrophosphate Dihydrate Crystal Deposition Disease) (diagnosed 2020-03-15)
- Invasive ductal carcinoma of the breast, Stage IV with bone metastases
- Primary hyperparathyroidism
- Hypertension
- Hyperlipidemia
- Osteoarthritis, degenerative
- Osteoporosis
- Nephrolithiasis (calcium oxalate)
- Calcium pyrophosphate deposition disease (CPPD)

She has a history of breast cancer treated with mastectomy and adjuvant chemotherapy 5 years ago.

PAST SURGICAL HISTORY:
- Left modified radical mastectomy with sentinel lymph node biopsy (5 years prior)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 50)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Letrozole 2.5 mg PO daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 20 mg PO daily
- Acetaminophen 650 mg PO q6h PRN pain
- Morphine sulfate sustained-release 30 mg PO every 12 hours
- Gabapentin 300 mg PO TID
- Dexamethasone 4 mg PO daily

ALLERGIES:
Penicillin (anaphylaxis), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Mother: Invasive breast cancer (diagnosed age 62, deceased)
- Father: Hypertension, chronic gout, recurrent kidney stones
- Sister: Severe osteoporosis with compression fractures

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (10 pack-years). Alcohol: Occasional social use (1-2 glasses of wine per week). Occupation: Retired high school librarian.

PHYSICAL EXAMINATION:
On physical examination, there is tenderness over the lumbar vertebrae. Neurological examination is normal.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum calcium: 11.2 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum albumin: 4.0 g/dL (Normal: 3.5-5.0 g/dL)
- Alkaline phosphatase: 120 U/L (Normal: 40-130 U/L)
- Serum creatinine: 0.9 mg/dL (Normal: 0.6-1.2 mg/dL)

IMAGING:
A bone scan shows increased uptake in multiple lumbar vertebrae.

## Encounter 3: source 2022-03-15, practice 2021-12-13, outpatient, Internal Medicine

Chief complaint: Sudden severe pain, swelling, and redness in right great toe

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Priya Sharma, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Sudden severe pain, swelling, and redness in right great toe

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old female with Stage IV breast cancer with bone metastases and calcium pyrophosphate deposition disease who presents to her primary care physician with a 2-day history of severe pain, swelling, and redness in her right big toe. The pain began suddenly during the night and has worsened since. She denies fever, chills, trauma, or prior episodes of joint pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignancy-associated hypercalcemia (diagnosed 2020-01-15)
- Pseudogout (Acute Calcium Pyrophosphate Dihydrate Crystal Deposition Disease) (diagnosed 2020-03-15)
- Metastatic carcinoma to bone (diagnosed 2021-03-15)
- Invasive ductal carcinoma of the breast, Stage IV with bone metastases
- Primary hyperparathyroidism
- Hypertension
- Hyperlipidemia
- Osteoarthritis, degenerative
- Osteoporosis
- Nephrolithiasis (calcium oxalate)
- Calcium pyrophosphate deposition disease (CPPD)

She has a history of hypertension.

PAST SURGICAL HISTORY:
- Left modified radical mastectomy with sentinel lymph node biopsy (5 years prior)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 50)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
She was recently started on a diuretic to help manage her blood pressure.

Home Medications:
- Letrozole 2.5 mg PO daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 20 mg PO daily
- Acetaminophen 650 mg PO q6h PRN pain
- Morphine sulfate sustained-release 30 mg PO every 12 hours
- Gabapentin 300 mg PO TID
- Dexamethasone 4 mg PO daily

ALLERGIES:
Penicillin (anaphylaxis), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Mother: Invasive breast cancer (diagnosed age 62, deceased)
- Father: Hypertension, chronic gout, recurrent kidney stones
- Sister: Severe osteoporosis with compression fractures

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (10 pack-years). Alcohol: Occasional social use (1-2 glasses of wine per week). Occupation: Retired high school librarian.

VITAL SIGNS:
Her temperature is 36.8°C (98.2°F), blood pressure is 130/ 80 mm Hg, pulse is 76/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Physical examination reveals a swollen, erythematous, and extremely tender first metatarsophalangeal joint on the right foot.

LABORATORY DATA:
Laboratory studies show a serum uric acid level of 10.2 mg /dL.

## Encounter 4: source 2023-03-15, practice 2022-11-28, ed, Emergency Medicine

Chief complaint: Severe temporal headache for 2 weeks with jaw claudication and new blurred vision

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe temporal headache for 2 weeks with jaw claudication and new blurred vision

HISTORY OF PRESENT ILLNESS:
This 69-year-old female with Stage IV breast cancer and history of malignancy-associated hypercalcemia and pseudogout presents with pain. She describes the pain as throbbing and states it intensifies with combing her hair. She also reports intermittent jaw claudication while chewing. She has been feeling generally unwell with low-grade fevers and fatigue. She reports a new onset of blurred vision in that eye since this morning.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignancy-associated hypercalcemia (diagnosed 2020-01-15)
- Pseudogout (Acute Calcium Pyrophosphate Dihydrate Crystal Deposition Disease) (diagnosed 2020-03-15)
- Metastatic carcinoma to bone (diagnosed 2021-03-15)
- Acute gouty arthritis (drug-induced) (diagnosed 2022-03-15)
- Invasive ductal carcinoma of the breast, Stage IV with bone metastases
- Primary hyperparathyroidism
- Hypertension
- Hyperlipidemia
- Osteoarthritis, degenerative
- Osteoporosis
- Nephrolithiasis (calcium oxalate)
- Calcium pyrophosphate deposition disease (CPPD)

PAST SURGICAL HISTORY:
- Left modified radical mastectomy with sentinel lymph node biopsy (5 years prior)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 50)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Letrozole 2.5 mg PO daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 20 mg PO daily
- Acetaminophen 650 mg PO q6h PRN pain
- Morphine sulfate sustained-release 30 mg PO every 12 hours
- Gabapentin 300 mg PO TID
- Dexamethasone 4 mg PO daily

ALLERGIES:
Penicillin (anaphylaxis), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Mother: Invasive breast cancer (diagnosed age 62, deceased)
- Father: Hypertension, chronic gout, recurrent kidney stones
- Sister: Severe osteoporosis with compression fractures

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (10 pack-years). Alcohol: Occasional social use (1-2 glasses of wine per week). Occupation: Retired high school librarian.

PHYSICAL EXAMINATION:
Her temporal arteries appear prominent and are tender to palpation. Visual acuity is decreased in the right eye. Fundoscopic examination reveals a pale optic disc with blurred margins on the right.

LABORATORY DATA:
- Erythrocyte sedimentation rate (ESR): 85 mm/hr (Normal: 0-20 mm/hr)
- C-reactive protein (CRP): 4.5 mg/dL (Normal: <0.9 mg/dL)
- Hemoglobin: 11.2 g/dL (Normal: 12-16 g/dL)
- Platelet count: 450,000/uL (Normal: 150,000-400,000/uL)

## Encounter 5: source 2024-07-15, practice 2024-04-12, ed, Emergency Medicine

Chief complaint: Worsening confusion, constipation, and generalized weakness over 1 month with nausea

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening confusion, constipation, and generalized weakness over 1 month with nausea

HISTORY OF PRESENT ILLNESS:
The patient with Stage IV invasive ductal carcinoma with bone metastases and prior hypercalcemia in 2020 returns with progressively worsening fatigue and anorexia over the past month, and she recently began experiencing nausea and vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignancy-associated hypercalcemia (diagnosed 2020-01-15)
- Pseudogout (Acute Calcium Pyrophosphate Dihydrate Crystal Deposition Disease) (diagnosed 2020-03-15)
- Metastatic carcinoma to bone (diagnosed 2021-03-15)
- Acute gouty arthritis (drug-induced) (diagnosed 2022-03-15)
- Giant cell arteritis or autoimmune inflammatory condition (diagnosed 2023-03-15)
- Invasive ductal carcinoma of the breast, Stage IV with bone metastases
- Primary hyperparathyroidism
- Hypertension
- Hyperlipidemia
- Osteoarthritis, degenerative
- Osteoporosis
- Nephrolithiasis (calcium oxalate)
- Calcium pyrophosphate deposition disease (CPPD)

PAST SURGICAL HISTORY:
- Left modified radical mastectomy with sentinel lymph node biopsy (5 years prior)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 50)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
She takes letrozole and lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 20 mg PO daily
- Acetaminophen 650 mg PO q6h PRN pain
- Morphine sulfate sustained-release 30 mg PO every 12 hours
- Gabapentin 300 mg PO TID
- Dexamethasone 4 mg PO daily

ALLERGIES:
Penicillin (anaphylaxis), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Mother: Invasive breast cancer (diagnosed age 62, deceased)
- Father: Hypertension, chronic gout, recurrent kidney stones
- Sister: Severe osteoporosis with compression fractures

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (10 pack-years). Alcohol: Occasional social use (1-2 glasses of wine per week). Occupation: Retired high school librarian.

PHYSICAL EXAMINATION:
Physical examination shows dry mucous membranes, diminished deep tendon reflexes, and mild abdominal tenderness. There is no lymphadenopathy, hepatosplenomegaly, or focal neurologic deficit. Neurological examination is non-focal.

IMAGING:
A skeletal survey reveals multiple lytic lesions involving the vertebrae, pelvis, and ribs. Noncontrast CT of the brain shows no acute abnormalities.

OTHER STUDIES:
ECG shows a shortened QT interval

## Encounter 6: source 2024-08-15, practice 2024-05-11, icu, Emergency Medicine

Chief complaint: Progressive confusion, severe weakness, and constipation for 1 week

ENCOUNTER NOTE
Date: 2024-08-15
Provider: Dr. Kwame Asante, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Progressive confusion, severe weakness, and constipation for 1 week

HISTORY OF PRESENT ILLNESS:
The patient with metastatic breast cancer returns to the emergency department with a 1-week history of progressive weakness, constipation, and confusion.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignancy-associated hypercalcemia (diagnosed 2020-01-15)
- Pseudogout (Acute Calcium Pyrophosphate Dihydrate Crystal Deposition Disease) (diagnosed 2020-03-15)
- Metastatic carcinoma to bone (diagnosed 2021-03-15)
- Acute gouty arthritis (drug-induced) (diagnosed 2022-03-15)
- Giant cell arteritis or autoimmune inflammatory condition (diagnosed 2023-03-15)
- Invasive ductal carcinoma of the breast, Stage IV with bone metastases
- Primary hyperparathyroidism
- Hypertension
- Hyperlipidemia
- Osteoarthritis, degenerative
- Osteoporosis
- Nephrolithiasis (calcium oxalate)
- Calcium pyrophosphate deposition disease (CPPD)

She has a past medical history of hypertension, for which she takes hydrochlorothiazide, and breast cancer treated with mastectomy 2 years ago.

PAST SURGICAL HISTORY:
- Left modified radical mastectomy with sentinel lymph node biopsy (5 years prior)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 50)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Letrozole 2.5 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 20 mg PO daily
- Acetaminophen 650 mg PO q6h PRN pain
- Morphine sulfate sustained-release 30 mg PO every 12 hours
- Gabapentin 300 mg PO TID
- Dexamethasone 4 mg PO daily

ALLERGIES:
Penicillin (anaphylaxis), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Mother: Invasive breast cancer (diagnosed age 62, deceased)
- Father: Hypertension, chronic gout, recurrent kidney stones
- Sister: Severe osteoporosis with compression fractures

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (10 pack-years). Alcohol: Occasional social use (1-2 glasses of wine per week). Occupation: Retired high school librarian.

PHYSICAL EXAMINATION:
On examination, she is lethargic but oriented to person and place, not to time. Her abdomen is non-distended, with hypoactive bowel sounds. She has decreased muscle strength in all extremities.

LABORATORY DATA:
Her initial laboratory results are as follows:

- Serum calcium: 14.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum creatinine: 1.8 mg/dL (normal: 0.6-1.1 mg/dL)
- Serum sodium: 142 mEq/L (normal: 135-145 mEq/L)
- Serum potassium: 4.0 mEq/L (normal: 3.5-5.0 mEq/L)
- Serum phosphorus: 2.9 mg/dL (normal: 2.5-4.5 mg/dL)
- Serum albumin: 4.0 g/dL (normal: 3.5-5.0 g/dL)
- Parathyroid hormone (PTH): 10 pg/mL (normal: 15-65 pg/mL)

## Encounter 7: source 2026-01-15, practice 2025-10-13, outpatient, Palliative Care

Chief complaint: Persistent pain and insomnia with expressed desire to end life; inquiries about physician-assisted suicide

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Rebecca Goldman, MD
Department: Palliative Care
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent pain and insomnia with expressed desire to end life; inquiries about physician-assisted suicide

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old female with Stage IV invasive ductal carcinoma with bone metastases, recurrent malignancy-associated hypercalcemia, and calcium pyrophosphate deposition disease, who returns to the palliative care clinic with her family. She has been on multiple lines of chemotherapy and recently started a new regimen. Despite analgesics, she reports persistent pain and difficulty sleeping. She expresses a desire to "just end it all" and inquires about physician-assisted suicide. She appears fatigued and has lost weight since her last visit.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignancy-associated hypercalcemia (diagnosed 2020-01-15)
- Pseudogout (Acute Calcium Pyrophosphate Dihydrate Crystal Deposition Disease) (diagnosed 2020-03-15)
- Metastatic carcinoma to bone (diagnosed 2021-03-15)
- Acute gouty arthritis (drug-induced) (diagnosed 2022-03-15)
- Giant cell arteritis or autoimmune inflammatory condition (diagnosed 2023-03-15)
- Invasive ductal carcinoma of the breast, Stage IV with bone metastases
- Primary hyperparathyroidism
- Hypertension
- Hyperlipidemia
- Osteoarthritis, degenerative
- Osteoporosis
- Nephrolithiasis (calcium oxalate)
- Calcium pyrophosphate deposition disease (CPPD)

PAST SURGICAL HISTORY:
- Left modified radical mastectomy with sentinel lymph node biopsy (5 years prior)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 50)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Her current medications include morphine sustained-release, gabapentin, and dexamethasone.

Home Medications:
- Letrozole 2.5 mg PO daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 20 mg PO daily
- Acetaminophen 650 mg PO q6h PRN pain
- Morphine sulfate sustained-release 30 mg PO every 12 hours

ALLERGIES:
Penicillin (anaphylaxis), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Mother: Invasive breast cancer (diagnosed age 62, deceased)
- Father: Hypertension, chronic gout, recurrent kidney stones
- Sister: Severe osteoporosis with compression fractures

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (10 pack-years). Alcohol: Occasional social use (1-2 glasses of wine per week). Occupation: Retired high school librarian.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On examination, she is alert but appears distressed. Physical examination is notable for diffuse bony tenderness.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 10.2 g/dL
- Calcium: 10.5 mg/dL
- Alkaline phosphatase: 240 U/L
- Creatinine: 1.1 mg/dL
