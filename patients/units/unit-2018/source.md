# synthetic_hospital patient 2018

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Employer-sponsored commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-02-11 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2021-08-23 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2022-02-17 (Thu) | outpatient | Internal Medicine |
| 3 | 2021-07-15 | 2022-08-15 (Mon) | outpatient | Cardiology |
| 4 | 2022-07-15 | 2023-08-11 (Fri) | outpatient | Pulmonology |
| 5 | 2023-07-15 | 2024-08-10 (Sat) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2025-02-20 (Thu) | outpatient | Internal Medicine |
| 7 | 2025-01-15 | 2026-02-23 (Mon) | outpatient | Urology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored commercial insurance",
 "occupation": "Long-haul truck driver (previous 10-year history in construction)",
 "alcohol_use": "Social (2-3 drinks per week)",
 "family_history": [
  "Father: Myocardial infarction at age 64",
  "Mother: Type 2 diabetes mellitus with diabetic nephropathy",
  "Older brother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 35 pack-year history (reports brief quit attempt 5 years ago)",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 12)",
  "Diagnostic cardiac catheterization (age 54)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Type 2 Diabetes Mellitus"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Assessment of decision-making capacity", "Stable Angina Pectoris (Medical Management)", "Hyperlipidemia", "Renal artery stenosis", "Isolated pharmacologic pain management without immobilization", "Wilms tumor of left kidney", "Idiopathic Pulmonary Fibrosis"]
Comorbidities: ["Cardiovascular disease risk factors", "Tobacco use disorder", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-02-11, outpatient, Internal Medicine

Chief complaint: Routine check-up for ongoing management of hyperlipidemia

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up for ongoing management of hyperlipidemia

HISTORY OF PRESENT ILLNESS:
He has no complaints

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Type 2 Diabetes Mellitus

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Diagnostic cardiac catheterization (age 54)

MEDICATIONS:
He takes metformin, lisinopril, and hydrochlorothiazide.

Home Medications:
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His father had a myocardial infarction at age 60

SOCIAL HISTORY:
He quit smoking 5 years ago after a 30-year pack history

VITAL SIGNS:
His body mass index is 29 kg/m^2. His blood pressure today is 132/84 mmHg

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Laboratory results are as follows:

- Total cholesterol: 210 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 160 mg/dL
- Triglycerides: 175 mg/dL
- HbA1c: 7.2%

ASSESSMENT AND PLAN:
Based on the Pooled Cohort Equations cardiovascular risk calculator, his estimated 10-year ASCVD risk is 12%

## Encounter 1: source 2020-07-15, practice 2021-08-23, outpatient, Internal Medicine

Chief complaint: Follow-up visit for prospective cohort study of antihypertensive medication

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit for prospective cohort study of antihypertensive medication

HISTORY OF PRESENT ILLNESS:
This 55-year-old male with hypertension, hyperlipidemia, and type 2 diabetes mellitus returns for follow-up care following his routine check-up in January 2020 for ongoing hyperlipidemia management. He is enrolled in a prospective cohort study assessing the relationship between a new antihypertensive medication and the incidence of myocardial infarction (MI) among high-risk patients. He reports compliance with the medication regimen and has no new complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Diagnostic cardiac catheterization (age 54)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64
- Mother: Type 2 diabetes mellitus with diabetic nephropathy
- Older brother: Hypertension

SOCIAL HISTORY:
Smoking

REVIEW OF SYSTEMS:
He has no chest pain, dyspnea, or edema.

VITAL SIGNS:
His blood pressure today is 132/84 mmHg.

LABORATORY DATA:
His previous cholesterol levels were LDL 160 mg/dL and HDL 40 mg/dL.

OTHER STUDIES:
The study coordinator informs you that 20% of the study participants have been lost to follow-up.

PLAN:
He is scheduled for a follow-up lipid panel next month.

## Encounter 2: source 2021-01-15, practice 2022-02-17, outpatient, Internal Medicine

Chief complaint: Routine health maintenance visit with concerns regarding lung cancer risk

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance visit with concerns regarding lung cancer risk

HISTORY OF PRESENT ILLNESS:
The patient with hypertension, hyperlipidemia, and type 2 diabetes returns for health maintenance six months after his last visit for antihypertensive medication. He has no complaints but mentions that several of his friends have recently been diagnosed with lung cancer, which has made him anxious about his own health. The patient inquires about his own risk for lung cancer and what factors might contribute to it.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus

His blood pressure is controlled.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Diagnostic cardiac catheterization (age 54)

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64
- Mother: Type 2 diabetes mellitus with diabetic nephropathy
- Older brother: Hypertension

SOCIAL HISTORY:
Smoking. He has a 30-pack-year smoking history and continues to smoke half a pack of cigarettes per day

REVIEW OF SYSTEMS:
He denies any respiratory symptoms, weight loss, or fatigue

PHYSICAL EXAMINATION:
His physical examination is unremarkable

IMAGING:
His recent chest X-ray was normal

PLAN:
You decide to discuss a recent case-control study that evaluated multiple risk factors for lung cancer

## Encounter 3: source 2021-07-15, practice 2022-08-15, outpatient, Cardiology

Chief complaint: Intermittent chest pressure during morning walks for the past two months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent chest pressure during morning walks for the past two months

HISTORY OF PRESENT ILLNESS:
The patient is a 56-year-old male with hypertension, hyperlipidemia, and type 2 diabetes mellitus established in longitudinal care who presents with chest discomfort. The pain is described as a pressure-like sensation in the center of his chest, occurring mostly during his morning walks and subsiding with rest. He denies any pain at rest or at night.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Isolated pharmacologic pain management without immobilization (diagnosed 2021-01-15)
- Hypertension
- Type 2 Diabetes Mellitus

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Diagnostic cardiac catheterization (age 54)

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Significant for coronary artery disease in his father

SOCIAL HISTORY:
Smoking: Current smoker, 35 pack-year history (reports brief quit attempt 5 years ago). Alcohol: Social (2-3 drinks per week). Occupation: Long-haul truck driver (previous 10-year history in construction).

VITAL SIGNS:
On physical examination, his blood pressure is 138/86 mmHg, heart rate is 78 bpm, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
His cardiovascular examination is unremarkable, and there are no signs of heart failure.

LABORATORY DATA:
His fasting lipid panel is as follows:

- Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 150 mg/dL

OTHER STUDIES:
An electrocardiogram (ECG) shows no acute changes.

ASSESSMENT AND PLAN:
Given the patient's symptoms and risk factors,

## Encounter 4: source 2022-07-15, practice 2023-08-11, outpatient, Pulmonology

Chief complaint: Progressive exertional dyspnea and nonproductive cough over 6 months

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Emily Johnson, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive exertional dyspnea and nonproductive cough over 6 months

HISTORY OF PRESENT ILLNESS:
The patient with hypertension, hyperlipidemia, and type 2 diabetes mellitus presents following prior cardiology evaluation for exertional chest pressure and previous pulmonary risk assessment. He denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Isolated pharmacologic pain management without immobilization (diagnosed 2021-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Diagnostic cardiac catheterization (age 54)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for coronary artery disease in his father.

SOCIAL HISTORY:
He has a 30-pack-year smoking history and works in construction.

VITAL SIGNS:
On physical examination, his blood pressure is 135/85 mmHg, heart rate is 88 beats per minute, respiratory rate is 20 breaths per minute, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral end-inspiratory crackles.

LABORATORY DATA:
Laboratory Values:
- Complete Blood Count (CBC)
- Hemoglobin: 14.5 g/dL
- Leukocytes: 7,500 cells/mm^3
- Platelets: 250,000 cells/mm^3
- Serum Chemistry
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL
- Calcium: 9.6 mg/dL
- Arterial Blood Gas (ABG)
- pH: 7.42
- PaCO2: 38 mmHg
- PaO2: 75 mmHg
- Antinuclear Antibodies (ANA): Negative
- Rheumatoid Factor (RF): Negative

IMAGING:
Chest X-ray shows diffuse interstitial infiltrates. High-resolution CT scan of the chest demonstrates subpleural reticular changes and honeycombing without significant lymphadenopathy.

OTHER STUDIES:
Pulmonary function tests show a restrictive pattern with reduced diffusion capacity.

## Encounter 5: source 2023-07-15, practice 2024-08-10, ed, Emergency Medicine

Chief complaint: Sudden onset shortness of breath with pink frothy sputum

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset shortness of breath with pink frothy sputum

HISTORY OF PRESENT ILLNESS:
The patient with hypertension, hyperlipidemia, diabetes, and cardiopulmonary symptoms reports experiencing similar episodes over the past few months, usually at night.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Isolated pharmacologic pain management without immobilization (diagnosed 2021-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-07-15)
- Idiopathic Pulmonary Fibrosis (diagnosed 2022-07-15)
- Hypertension
- Type 2 Diabetes Mellitus

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Diagnostic cardiac catheterization (age 54)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64
- Mother: Type 2 diabetes mellitus with diabetic nephropathy
- Older brother: Hypertension

SOCIAL HISTORY:
History of smoking

VITAL SIGNS:
His blood pressure is 180/110 mmHg.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral crackles at the lung bases and a systolic murmur best heard over the right upper sternal border.

LABORATORY DATA:
His serum chemistry panel shows:

- Sodium: 140 mEq/L
- Potassium: 3.2 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 32 mEq/L
- Blood urea nitrogen: 28 mg/dL
- Creatinine: 1.8 mg/dL
- Glucose: 98 mg/dL

Urinalysis is unremarkable.

IMAGING:
A renal ultrasound with Doppler is performed.

## Encounter 6: source 2024-01-15, practice 2025-02-20, outpatient, Internal Medicine

Chief complaint: Routine health check requested by spouse

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health check requested by spouse

HISTORY OF PRESENT ILLNESS:
Patient with angina, IPF, renal stenosis returns. He reports that he feels “just fine” and only came in because his wife insisted.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Isolated pharmacologic pain management without immobilization (diagnosed 2021-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-07-15)
- Idiopathic Pulmonary Fibrosis (diagnosed 2022-07-15)
- Renal artery stenosis (diagnosed 2023-07-15)
- Hypertension
- Type 2 Diabetes Mellitus

He has a history of hypertension and hyperlipidemia but has not followed up for the past 2 years.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Diagnostic cardiac catheterization (age 54)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64
- Mother: Type 2 diabetes mellitus with diabetic nephropathy
- Older brother: Hypertension

SOCIAL HISTORY:
He works long hours as a truck driver and smokes about a pack of cigarettes daily. When asked about quitting, he says, “I’ve smoked for over 30 years—it helps with my stress. I don’t see the point in stopping now.”

## Encounter 7: source 2025-01-15, practice 2026-02-23, outpatient, Urology

Chief complaint: Three weeks of dark-red urine and left-sided flank discomfort with unintentional weight loss

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. James Thompson, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Three weeks of dark-red urine and left-sided flank discomfort with unintentional weight loss

HISTORY OF PRESENT ILLNESS:
This 60-year-old male with HTN, HLD, T2DM, stable angina, IPF, and prior renal artery stenosis describes a dull, persistent ache that worsens with movement, recent fatigue, and a 3.5-kg (8-lb) unintentional weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Isolated pharmacologic pain management without immobilization (diagnosed 2021-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-07-15)
- Idiopathic Pulmonary Fibrosis (diagnosed 2022-07-15)
- Renal artery stenosis (diagnosed 2023-07-15)
- Assessment of decision-making capacity (diagnosed 2024-01-15)
- Hypertension
- Type 2 Diabetes Mellitus

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Diagnostic cardiac catheterization (age 54)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 64
- Mother: Type 2 diabetes mellitus with diabetic nephropathy
- Older brother: Hypertension

SOCIAL HISTORY:
He has a 35-pack-year smoking history.

VITAL SIGNS:
On exam, blood pressure is 148/88 mm Hg and temperature is 37.1°C (98.8°F)

PHYSICAL EXAMINATION:
There is a palpable non-tender mass in the left upper quadrant.

LABORATORY DATA:
Laboratory studies show:
Hemoglobin 19.2 g/dL
Calcium 11.4 mg/dL
Creatinine 1.0 mg/dL
Urinalysis 3+ blood, no casts
