# synthetic_hospital patient 2231

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-07-07 (Fri) | outpatient | Otolaryngology |
| 1 | 2020-07-15 | 2024-01-10 (Wed) | outpatient | Vascular Surgery |
| 2 | 2021-01-15 | 2024-07-11 (Thu) | outpatient | Urology |
| 3 | 2021-07-15 | 2025-01-09 (Thu) | outpatient | Pulmonology |
| 4 | 2022-01-15 | 2025-06-25 (Wed) | outpatient | Pulmonology |
| 5 | 2022-07-15 | 2026-01-02 (Fri) | outpatient | Urology |
| 6 | 2022-10-15 | 2026-04-12 (Sun) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-07-10 (Fri) | inpatient | Pulmonology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired shipbuilder",
 "alcohol_use": "Heavy alcohol use for several decades",
 "family_history": [
  "Father with lung cancer",
  "Mother with hypertension",
  "Brother with myocardial infarction at age 65"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 45 pack-year history",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Hypertension",
  "Chronic bronchitis"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Syndrome of inappropriate antidiuretic hormone secretion (SIADH)", "Non-small cell lung cancer, right upper lobe", "Iliofemoral Deep Vein Thrombosis", "Femoral artery aneurysm", "Bladder cancer", "Urothelial carcinoma of bladder (posterior wall)", "Oral squamous cell carcinoma"]
Comorbidities: ["Tobacco use disorder", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2023-07-07, outpatient, Otolaryngology

Chief complaint: painless swelling on the right side of his neck

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Painless swelling on the right side of his neck

HISTORY OF PRESENT ILLNESS:
That he first noticed several weeks ago. He reports no recent fevers, sore throat, or dental procedures. He has not seen a dentist in over a decade and frequently complains of chronic bad breath and dry mouth.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Chronic bronchitis

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with lung cancer
- Mother with hypertension
- Brother with myocardial infarction at age 65

SOCIAL HISTORY:
Many years of alcohol and tobacco use

REVIEW OF SYSTEMS:
He does not endorse weight loss, night sweats, or difficulty swallowing.

PHYSICAL EXAMINATION:
On examination, he has a firm, non-tender, 2.5 cm mass just inferior to the angle of the mandible. The overlying skin appears normal without erythema or warmth. Oral examination shows poor dentition but no visible masses or ulcers.

PATHOLOGY:
Biopsy of the lymph node demonstrates nests and sheets of atypical squamous cells with intercellular bridges and focal keratinization.

OTHER STUDIES:
Flexible nasopharyngoscopy reveals no obvious lesions in the upper pharynx.

## Encounter 1: source 2020-07-15, practice 2024-01-10, outpatient, Vascular Surgery

Chief complaint: intermittent right calf pain with walking that subsides with rest

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Vascular Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent right calf pain with walking that subsides with rest

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with hypertension, chronic bronchitis, and prior oral squamous cell carcinoma presents with a 2-month history of intermittent right calf pain that worsens with walking and subsides with rest. He also reports recent onset of numbness and tingling in the same leg.

PAST MEDICAL HISTORY:
Active Problem List:
- Oral squamous cell carcinoma (diagnosed 2020-01-15)
- Hypertension
- Chronic bronchitis

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for an abdominal aortic aneurysm in his father.

SOCIAL HISTORY:
Smoking

PHYSICAL EXAMINATION:
On physical examination, you note a palpable, pulsatile mass in the right popliteal fossa. The right foot has a diminished pedal pulse compared to the left.

IMAGING:
Duplex ultrasonography of the right lower extremity shows a 3.5 cm aneurysm in the popliteal artery with partial thrombosis.

OTHER STUDIES:
Ankle-brachial index (ABI) on the right is 0.7 and on the left is 1.1.

## Encounter 2: source 2021-01-15, practice 2024-07-11, outpatient, Urology

Chief complaint: intermittent, painless hematuria for the past month

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Park, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent, painless hematuria for the past month

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with hypertension and chronic bronchitis previously evaluated in January 2020 for painless neck swelling and in July 2020 for intermittent right calf pain who presents with intermittent, painless hematuria for the past month. He has no associated dysuria or fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Oral squamous cell carcinoma (diagnosed 2020-01-15)
- Femoral artery aneurysm (diagnosed 2020-07-15)
- Hypertension
- Chronic bronchitis

His medical history includes hypertension and chronic bronchitis.

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with lung cancer
- Mother with hypertension
- Brother with myocardial infarction at age 65

SOCIAL HISTORY:
He has a 45-pack-year smoking history and continues to smoke one pack per day.

LABORATORY DATA:
Urinalysis confirms gross hematuria with no signs of infection.

IMAGING:
CT urogram shows no evidence of hydronephrosis or lymphadenopathy.

PATHOLOGY:
Biopsy shows transitional cell carcinoma (urothelial carcinoma).

OTHER STUDIES:
A cystoscopy reveals a papillary lesion on the posterior bladder wall.

PLAN:
The urologist explains that further evaluation is necessary to determine the stage of the tumor.

## Encounter 3: source 2021-07-15, practice 2025-01-09, outpatient, Pulmonology

Chief complaint: persistent cough and unintentional weight loss over the past 3 months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Emily Thompson, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough and unintentional weight loss over the past 3 months

HISTORY OF PRESENT ILLNESS:
The patient with chronic bronchitis and prior oral and urothelial cancer presents to the clinic complaining of a persistent cough and unintentional weight loss over the past 3 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Oral squamous cell carcinoma (diagnosed 2020-01-15)
- Femoral artery aneurysm (diagnosed 2020-07-15)
- Urothelial carcinoma of bladder (posterior wall) (diagnosed 2021-01-15)
- Hypertension
- Chronic bronchitis

MEDICATIONS:
He takes controlled with lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with lung cancer
- Mother with hypertension
- Brother with myocardial infarction at age 65

SOCIAL HISTORY:
He has a history of working in shipbuilding for 30 years

PHYSICAL EXAMINATION:
On physical examination, you note clubbing of the fingers and dullness to percussion at the right lung base.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 13.8 g/dL
- White blood cell count: 7,500 /μL
- Platelets: 250,000 /μL
- Serum calcium: 10.2 mg/dL
- Carcinoembryonic antigen (CEA): Elevated

IMAGING:
Chest X-ray reveals a right-sided pleural thickening and a mass in the right upper lobe.

## Encounter 4: source 2022-01-15, practice 2025-06-25, outpatient, Pulmonology

Chief complaint: progressive pain in his wrists and ankles with mild shortness of breath on exertion

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Thompson, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive pain in his wrists and ankles with mild shortness of breath on exertion

HISTORY OF PRESENT ILLNESS:
The patient, with histories of oral squamous cell carcinoma and urothelial carcinoma, returns for follow-up with progressive wrist and ankle pain. He describes deep, achy discomfort with occasional swelling. He also reports mild exertional dyspnea and 5 kg unintentional weight loss over the past 2 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Oral squamous cell carcinoma (diagnosed 2020-01-15)
- Femoral artery aneurysm (diagnosed 2020-07-15)
- Urothelial carcinoma of bladder (posterior wall) (diagnosed 2021-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2021-07-15)
- Hypertension
- Chronic bronchitis

His medical history includes hypertension and.

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with lung cancer
- Mother with hypertension
- Brother with myocardial infarction at age 65

SOCIAL HISTORY:
50 pack-years of smoking.

PHYSICAL EXAMINATION:
On physical examination, his joints are tender to palpation with minimal swelling. Fingernail clubbing is present bilaterally.

IMAGING:
A chest x-ray shows a peripheral nodule in the right upper lobe. A CT scan confirms a 3.5 cm lesion with no evidence of central necrosis or hilar lymphadenopathy.

## Encounter 5: source 2022-07-15, practice 2026-01-02, outpatient, Urology

Chief complaint: painless gross hematuria and increased urinary frequency for 2 months

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James Park, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Painless gross hematuria and increased urinary frequency for 2 months

HISTORY OF PRESENT ILLNESS:
Patient, with prior bladder urothelial carcinoma, returns with 2-month history of painless gross hematuria and increased urinary frequency. He denies any dysuria or fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Oral squamous cell carcinoma (diagnosed 2020-01-15)
- Femoral artery aneurysm (diagnosed 2020-07-15)
- Urothelial carcinoma of bladder (posterior wall) (diagnosed 2021-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2021-07-15)
- Hypertension
- Chronic bronchitis

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
No family history of cancer

SOCIAL HISTORY:
Smoking a pack of cigarettes daily for 40 years

PHYSICAL EXAMINATION:
No costovertebral angle tenderness, and the abdominal exam is unremarkable

LABORATORY DATA:
Urinalysis reveals red blood cells but no infection. Urine cytology shows atypical urothelial cells. The patient's renal function is normal, and a complete blood count is within normal limits.

## Encounter 6: source 2022-10-15, practice 2026-04-12, ed, Emergency Medicine

Chief complaint: progressive left calf pain and swelling over 3 days

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive left calf pain and swelling over 3 days

HISTORY OF PRESENT ILLNESS:
The patient, with history of multiple malignancies and femoral artery aneurysm, presents to the emergency department with a 3-day history of progressive left calf pain and swelling. He denies any recent trauma or injury to the leg.

PAST MEDICAL HISTORY:
Active Problem List:
- Oral squamous cell carcinoma (diagnosed 2020-01-15)
- Femoral artery aneurysm (diagnosed 2020-07-15)
- Urothelial carcinoma of bladder (posterior wall) (diagnosed 2021-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2021-07-15)
- Bladder cancer (diagnosed 2022-07-15)
- Hypertension
- Chronic bronchitis

His medical history is significant for hypertension.

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with lung cancer
- Mother with hypertension
- Brother with myocardial infarction at age 65

SOCIAL HISTORY:
A 40-pack-year smoking history. He recently retired and admits to a sedentary lifestyle since retirement.

PHYSICAL EXAMINATION:
On physical examination, his left calf is erythematous, warm, and 3 cm larger in circumference than the right. Homan's sign is positive.

LABORATORY DATA:
His laboratory values are as follows:

- Hemoglobin: 13.2 g/dL
- Platelet count: 255 x 10^9/L
- Prothrombin time (PT): 12 seconds
- Activated partial thromboplastin time (aPTT): 30 seconds
- D-dimer: 750 ng/mL (elevated)

IMAGING:
A duplex ultrasound of the lower extremities reveals a noncompressible segment in the left popliteal vein.

## Encounter 7: source 2023-01-15, practice 2026-07-10, inpatient, Pulmonology

Chief complaint: persistent cough with acute onset confusion and generalized weakness over the past week

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Kim, MD
Department: Pulmonology
Visit Type: inpatient

CHIEF COMPLAINT:
Persistent cough with acute onset confusion and generalized weakness over the past week

HISTORY OF PRESENT ILLNESS:
The patient with lung and bladder cancer presents. He reports generalized weakness and occasional confusion over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Oral squamous cell carcinoma (diagnosed 2020-01-15)
- Femoral artery aneurysm (diagnosed 2020-07-15)
- Urothelial carcinoma of bladder (posterior wall) (diagnosed 2021-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2021-07-15)
- Bladder cancer (diagnosed 2022-07-15)
- Iliofemoral Deep Vein Thrombosis (diagnosed 2022-10-15)
- Hypertension
- Chronic bronchitis

His past medical history is significant for hypertension, controlled with hydrochlorothiazide.

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with lung cancer
- Mother with hypertension
- Brother with myocardial infarction at age 65

SOCIAL HISTORY:
With a history of smoking

REVIEW OF SYSTEMS:
He denies fever, chest pain, or shortness of breath.

VITAL SIGNS:
On physical examination, he is afebrile, blood pressure is 130/85 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds in the right upper lobe. Neurological examination is nonfocal.

LABORATORY DATA:
Laboratory results are as follows:

- Sodium: 125 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 4.2 mEq/L (Normal: 3.5-5.1 mEq/L)
- Chloride: 95 mEq/L (Normal: 96-106 mEq/L)
- Bicarbonate: 24 mEq/L (Normal: 22-29 mEq/L)
- Blood Urea Nitrogen: 18 mg/dL (Normal: 7-20 mg/dL)
- Serum Creatinine: 0.9 mg/dL (Normal: 0.6-1.2 mg/dL)
- Serum Osmolality: 260 mOsm/kg (Normal: 275-295 mOsm/kg)
- Urine Osmolality: 500 mOsm/kg (Normal: 50-1200 mOsm/kg)
- Urine Sodium: 40 mEq/L (Normal: 20-40 mEq/L)

IMAGING:
Chest X-ray shows a mass in the right upper lobe of the lung.

ASSESSMENT AND PLAN:
Based on the patient's presentation and laboratory findings,
