# synthetic_hospital patient 2268

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 64 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield (Private PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2016-11-07 (Mon) | outpatient | Dermatology |
| 1 | 2021-01-15 | 2017-11-06 (Mon) | outpatient | Pulmonology |
| 2 | 2024-01-15 | 2020-10-30 (Fri) | outpatient | Cardiology |
| 3 | 2028-01-15 | 2024-10-27 (Sun) | ed | Emergency Medicine |
| 4 | 2029-01-15 | 2025-11-01 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield (Private PPO)",
 "occupation": "Retired stone mason and construction worker",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Father: Lung cancer (deceased age 72)",
  "Mother: Hypertension, cardiovascular disease",
  "Brother: Chronic obstructive pulmonary disease",
  "Paternal uncle: Acute leukemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 50 pack-year history, with previous quit attempt 10 years ago",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg daily",
   "name": "Amlodipine"
  },
  {
   "dose": "12.5 mg daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for wheezing",
   "name": "Albuterol"
  }
 ],
 "surgical_history": [
  "Mohs micrographic surgery of left ear for squamous cell carcinoma",
  "Bone marrow biopsy",
  "Mediastinal lymph node biopsy"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Chronic obstructive pulmonary disease with emphysema and chronic bronchitis (30-year history)",
  "Idiopathic pulmonary fibrosis",
  "History of squamous cell carcinoma of the left ear (status post treatment)",
  "History of hematologic malignancy in remission"
 ],
 "age_at_first_encounter": 64
}
```

Primary diagnoses: ["Squamous cell carcinoma of skin", "Idiopathic Pulmonary Fibrosis", "Cardiac tamponade", "Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease)"]
Comorbidities: ["Tobacco use disorder", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2016-11-07, outpatient, Dermatology

Chief complaint: Non-healing ulcerated lesion on his left ear with gradual enlargement over the past four months.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Non-healing ulcerated lesion on his left ear with gradual enlargement over the past four months.

HISTORY OF PRESENT ILLNESS:
He reports gradual enlargement over the past four months with occasional bleeding but no pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Chronic obstructive pulmonary disease with emphysema and chronic bronchitis (30-year history)
- Idiopathic pulmonary fibrosis
- History of squamous cell carcinoma of the left ear (status post treatment)
- History of hematologic malignancy in remission

PAST SURGICAL HISTORY:
- Mohs micrographic surgery of left ear for squamous cell carcinoma
- Bone marrow biopsy
- Mediastinal lymph node biopsy

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
NKDA

FAMILY HISTORY:
His father died of laryngeal cancer.

SOCIAL HISTORY:
His history is significant for a 40-pack-year smoking history and extensive sun exposure as a retired construction worker.

VITAL SIGNS:
Vitals are within normal limits.

PHYSICAL EXAMINATION:
Physical examination reveals a scaly, erythematous plaque with central ulceration.

OTHER STUDIES:
A biopsy is performed

## Encounter 1: source 2021-01-15, practice 2017-11-06, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath over the past 8 months, becoming winded while climbing stairs.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Robert Martinez, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath over the past 8 months, becoming winded while climbing stairs.

HISTORY OF PRESENT ILLNESS:
The patient returns to Pulmonology for follow-up of his interstitial lung disease, one year after Dermatology treatment for left ear squamous cell carcinoma. He states that he becomes winded while climbing stairs and recently had to stop taking his dog on long walks. He denies chest pain, orthopnea, or leg swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Squamous cell carcinoma of skin (diagnosed 2020-01-15)
- Hypertension
- Chronic obstructive pulmonary disease with emphysema and chronic bronchitis (30-year history)
- Idiopathic pulmonary fibrosis
- History of squamous cell carcinoma of the left ear (status post treatment)
- History of hematologic malignancy in remission

PAST SURGICAL HISTORY:
- Mohs micrographic surgery of left ear for squamous cell carcinoma
- Bone marrow biopsy
- Mediastinal lymph node biopsy

MEDICATIONS:
He takes amlodipine and lisinopril.

Home Medications:
- Hydrochlorothiazide 12.5 mg daily
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 72)
- Mother: Hypertension, cardiovascular disease
- Brother: Chronic obstructive pulmonary disease
- Paternal uncle: Acute leukemia

SOCIAL HISTORY:
25 pack-year smoking history, though he quit smoking 10 years ago. He worked as a stone mason for 30 years before retiring.

VITAL SIGNS:
Afebrile, blood pressure is 130/78 mmHg, pulse is 86/min, and respiratory rate is 18/min. Oxygen saturation is 94% on room air at rest, dropping to 88% with minimal exertion.

PHYSICAL EXAMINATION:
Fine bibasilar crackles are heard on lung auscultation. There is no jugular venous distention, no peripheral edema, and heart sounds are normal without gallops or murmurs.

IMAGING:
A high-resolution chest CT shows reticular opacities and subpleural honeycombing predominantly at the lung bases. Echocardiogram shows normal left ventricular function.

PLAN:
Right heart catheterization is planned to assess pulmonary pressures and distinguish between causes of his dyspnea.

## Encounter 2: source 2024-01-15, practice 2020-10-30, outpatient, Cardiology

Chief complaint: Progressive bilateral lower extremity swelling and increasing shortness of breath on exertion over 6 weeks.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Priya Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive bilateral lower extremity swelling and increasing shortness of breath on exertion over 6 weeks.

HISTORY OF PRESENT ILLNESS:
The patient with idiopathic pulmonary fibrosis and chronic obstructive pulmonary disease presents with a 6-week history of progressive bilateral lower extremity swelling and increasing shortness of breath on exertion. He has also noted a sensation of abdominal fullness and occasional right upper quadrant discomfort. He denies chest pain, palpitations, syncope, or orthopnea.

PAST MEDICAL HISTORY:
Active Problem List:
- Squamous cell carcinoma of skin (diagnosed 2020-01-15)
- Idiopathic Pulmonary Fibrosis (diagnosed 2021-01-15)
- Hypertension
- Chronic obstructive pulmonary disease with emphysema and chronic bronchitis (30-year history)
- History of squamous cell carcinoma of the left ear (status post treatment)
- History of hematologic malignancy in remission

His past medical history includes a 30-year history of chronic bronchitis and emphysema. He has had two prior hospitalizations for COPD exacerbations in the last year, both requiring oral steroids and antibiotics. He has no known history of coronary artery disease or prior myocardial infarction.

PAST SURGICAL HISTORY:
- Mohs micrographic surgery of left ear for squamous cell carcinoma
- Bone marrow biopsy
- Mediastinal lymph node biopsy

MEDICATIONS:
He takes inhaled tiotropium and albuterol as needed.

Home Medications:
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 12.5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 72)
- Mother: Hypertension, cardiovascular disease
- Brother: Chronic obstructive pulmonary disease
- Paternal uncle: Acute leukemia

SOCIAL HISTORY:
He continues to smoke one pack of cigarettes per day.

VITAL SIGNS:
On physical examination, he is afebrile with a blood pressure of 118/78 mm Hg, pulse of 92/min, and respiratory rate of 20/min. Oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
He has prominent jugular venous distension at 45 degrees, a parasternal heave, and a grade 2/6 systolic murmur heard best at the lower left sternal border that increases in intensity with inspiration. Lung auscultation reveals scattered expiratory wheezes and faint crackles at the bases. Cardiac examination shows a normal S1 and S2 with no gallops. Abdominal examination reveals a pulsatile liver edge and mild hepatomegaly. There is 2+ pitting edema bilaterally to the knees.

LABORATORY DATA:
His hemoglobin concentration is 18.3 g/dL.

IMAGING:
Chest x-ray reveals hyperinflated lung fields and a prominent right ventricle.

OTHER STUDIES:
ECG shows right axis deviation and signs of right ventricular hypertrophy.

## Encounter 3: source 2028-01-15, practice 2024-10-27, ed, Emergency Medicine

Chief complaint: Progressive fatigue and episodes of lightheadedness over the past few days with abdominal bloating.

ENCOUNTER NOTE
Date: 2028-01-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive fatigue and episodes of lightheadedness over the past few days with abdominal bloating.

HISTORY OF PRESENT ILLNESS:
Patient with COPD, idiopathic pulmonary fibrosis, and Cor pulmonale presents. He reports poor appetite and abdominal bloating that has been worsening, along with intermittent chest pressure that is not clearly related to exertion.

PAST MEDICAL HISTORY:
Active Problem List:
- Squamous cell carcinoma of skin (diagnosed 2020-01-15)
- Idiopathic Pulmonary Fibrosis (diagnosed 2021-01-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2024-01-15)
- Hypertension
- Chronic obstructive pulmonary disease with emphysema and chronic bronchitis (30-year history)
- History of squamous cell carcinoma of the left ear (status post treatment)
- History of hematologic malignancy in remission

His past medical history is notable for a prior hematologic malignancy treated several years ago.

PAST SURGICAL HISTORY:
- Mohs micrographic surgery of left ear for squamous cell carcinoma
- Bone marrow biopsy
- Mediastinal lymph node biopsy

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 72)
- Mother: Hypertension, cardiovascular disease
- Brother: Chronic obstructive pulmonary disease
- Paternal uncle: Acute leukemia

SOCIAL HISTORY:
He continues to smoke

REVIEW OF SYSTEMS:
He denies fever, cough, or recent infection.

VITAL SIGNS:
Blood pressure is 86/58 mmHg, pulse is 118/min, respirations are 22/min, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
On examination, the patient appears ill and anxious, sitting upright and breathing quickly. Neck examination shows distended veins that remain engorged throughout respiration. Heart sounds are faint, and lung fields are clear. The abdomen is distended with shifting dullness, and the extremities show cool skin with weak pulses that are difficult to palpate during inspiration.

## Encounter 4: source 2029-01-15, practice 2025-11-01, ed, Emergency Medicine

Chief complaint: Dyspnea and chest pain that worsens upon lying down.

ENCOUNTER NOTE
Date: 2029-01-15
Provider: Dr. Emily Zhang, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Dyspnea and chest pain that worsens upon lying down.

HISTORY OF PRESENT ILLNESS:
The patient is a 73-year-old male with COPD, idiopathic pulmonary fibrosis, and a history of squamous cell carcinoma and hematologic malignancy who returns with dyspnea and positional chest pain, having presented with similar cardiac symptoms approximately one year ago. He denies any recent illness or trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Squamous cell carcinoma of skin (diagnosed 2020-01-15)
- Idiopathic Pulmonary Fibrosis (diagnosed 2021-01-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2024-01-15)
- Cardiac tamponade (diagnosed 2028-01-15)
- Hypertension
- Chronic obstructive pulmonary disease with emphysema and chronic bronchitis (30-year history)
- History of squamous cell carcinoma of the left ear (status post treatment)
- History of hematologic malignancy in remission

PAST SURGICAL HISTORY:
- Mohs micrographic surgery of left ear for squamous cell carcinoma
- Bone marrow biopsy
- Mediastinal lymph node biopsy

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 12.5 mg daily
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 72)
- Mother: Hypertension, cardiovascular disease
- Brother: Chronic obstructive pulmonary disease
- Paternal uncle: Acute leukemia

SOCIAL HISTORY:
With a history of 50-pack-year smoking

VITAL SIGNS:
On physical examination, his blood pressure is 110/70 mmHg, heart rate is 110 bpm, respiratory rate is 22 breaths per minute, and oxygen saturation is 95% on room air.

PHYSICAL EXAMINATION:
His jugular venous pressure is elevated, and heart sounds are muffled. Lung fields are clear to auscultation.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 14 g/dL
- Leukocyte count: 7,000/mm^3
- Platelet count: 250,000/mm^3
- Serum creatinine: 1.0 mg/dL
- BUN: 14 mg/dL
- Troponin I: <0.04 ng/mL (normal)

IMAGING:
A chest x-ray reveals an enlarged cardiac silhouette with no pulmonary infiltrates.

OTHER STUDIES:
An ECG shows low voltage QRS complexes and electrical alternans.
