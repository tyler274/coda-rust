# synthetic_hospital patient 1994

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Medicare (disability) and Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-03-17 (Fri) | outpatient | Pulmonology |
| 1 | 2020-04-15 | 2023-06-19 (Mon) | outpatient | Hematology & Oncology |
| 2 | 2021-01-15 | 2024-03-18 (Mon) | outpatient | Nephrology |
| 3 | 2021-07-15 | 2024-09-23 (Mon) | outpatient | General Surgery |
| 4 | 2022-01-15 | 2025-03-16 (Sun) | ed | Emergency Medicine |
| 5 | 2022-07-15 | 2025-09-15 (Mon) | ed | Emergency Medicine |
| 6 | 2022-09-15 | 2025-11-25 (Tue) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-03-30 (Mon) | icu | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (disability) and Medicaid",
 "occupation": "Disabled former construction worker",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Myocardial infarction at age 62, hypertension",
  "Mother: Emphysema, deceased age 70",
  "Brother: Bipolar disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 40 pack-year history (1 pack per day)",
 "home_medications": [
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  },
  {
   "dose": "50 mg orally twice daily",
   "name": "Metoprolol tartrate"
  },
  {
   "dose": "20 mg orally daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg orally daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg orally daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "300 mg orally three times daily",
   "name": "Lithium carbonate"
  },
  {
   "dose": "81 mg orally daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg orally nightly",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Inguinal hernia repair (elective, date unspecified)",
  "Diagnostic lymph node biopsy (left cervical)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) stage unspecified",
  "Hypertension",
  "Coronary artery disease",
  "Bipolar I disorder (stable on lithium for 10 years)",
  "Tobacco use disorder",
  "Malignant neoplasm of cervical lymph nodes (left side)"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Massive Hemoptysis", "Tobacco use disorder", "Beta-blocker overdose", "Malignant neoplasm of lymph nodes", "Chronic obstructive pulmonary disease", "Lithium nephrotoxicity", "Ventricular fibrillation (refractory)"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Tobacco use disorder", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2023-03-17, outpatient, Pulmonology

Chief complaint: Persistent cough and shortness of breath that has worsened over the past 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough and shortness of breath that has worsened over the past 6 months

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) stage unspecified
- Hypertension
- Coronary artery disease
- Bipolar I disorder (stable on lithium for 10 years)
- Tobacco use disorder
- Malignant neoplasm of cervical lymph nodes (left side)

PAST SURGICAL HISTORY:
- Inguinal hernia repair (elective, date unspecified)
- Diagnostic lymph node biopsy (left cervical)

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Tiotropium 18 mcg inhaled daily
- Metoprolol tartrate 50 mg orally twice daily
- Lisinopril 20 mg orally daily
- Amlodipine 5 mg orally daily
- Lithium carbonate 300 mg orally three times daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Emphysema, deceased age 70
- Brother: Bipolar disorder

SOCIAL HISTORY:
30-pack-year smoking history

REVIEW OF SYSTEMS:
He denies any fever, night sweats, or weight loss

VITAL SIGNS:
Blood pressure is 135/85 mmHg, heart rate is 78 bpm, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Barrel-chested appearance and decreased breath sounds with prolonged expiration

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 15 g/dL
- WBC count: 7,000/mm^3
- Platelet count: 250,000/mm^3
- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.0 mg/dL

IMAGING:
Chest X-ray shows hyperinflation of the lungs with flattened diaphragms

OTHER STUDIES:
Pulmonary function tests reveal a decreased FEV1/FVC ratio and a normal total lung capacity

## Encounter 1: source 2020-04-15, practice 2023-06-19, outpatient, Hematology & Oncology

Chief complaint: Painless, progressively enlarging mass in the left side of his neck for 2 months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Rodriguez, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Painless, progressively enlarging mass in the left side of his neck for 2 months

HISTORY OF PRESENT ILLNESS:
The patient, who established care in Pulmonology three months ago for chronic respiratory symptoms, returns for evaluation of a new concern. He presents with a 2-month history of a painless, progressively enlarging mass in the left side of his neck. He denies any recent infections, fever, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) stage unspecified
- Hypertension
- Coronary artery disease
- Bipolar I disorder (stable on lithium for 10 years)
- Tobacco use disorder
- Malignant neoplasm of cervical lymph nodes (left side)

He has a history of chronic obstructive pulmonary disease (COPD) and hypertension.

PAST SURGICAL HISTORY:
- Inguinal hernia repair (elective, date unspecified)
- Diagnostic lymph node biopsy (left cervical)

MEDICATIONS:
He takes he takes tiotropium and lisinopril.

Home Medications:
- Metoprolol tartrate 50 mg orally twice daily
- Amlodipine 5 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Lithium carbonate 300 mg orally three times daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Emphysema, deceased age 70
- Brother: Bipolar disorder

SOCIAL HISTORY:
40-pack-year smoking history

PHYSICAL EXAMINATION:
On physical examination, a firm, non-tender, 3 cm lymph node is palpated in the left cervical chain. There is no overlying skin change, and no other lymphadenopathy is appreciated. Flexible fiberoptic laryngoscopy reveals a small, ulcerated lesion on the left lateral border of the tongue. The remainder of the oropharyngeal mucosa appears normal.

PATHOLOGY:
A fine needle aspiration (FNA) biopsy of the cervical lymph node is performed, and the cytology is consistent with squamous cell carcinoma.

## Encounter 2: source 2021-01-15, practice 2024-03-18, outpatient, Nephrology

Chief complaint: Routine follow-up with increased fatigue, frequent urination, and nocturia

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with increased fatigue, frequent urination, and nocturia

HISTORY OF PRESENT ILLNESS:
The patient returns for nephrology evaluation amid decade-long lithium therapy for bipolar disorder, following prior management of COPD and malignant cervical lymphadenopathy. He reports feeling stable on his current regimen of lithium, which he has been taking for the past 10 years. He denies any recent manic or depressive episodes. However, he mentions increased fatigue and a more frequent need to urinate, especially at night.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Malignant neoplasm of lymph nodes (diagnosed 2020-04-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) stage unspecified
- Hypertension
- Coronary artery disease
- Bipolar I disorder (stable on lithium for 10 years)
- Tobacco use disorder
- Malignant neoplasm of cervical lymph nodes (left side)

PAST SURGICAL HISTORY:
- Inguinal hernia repair (elective, date unspecified)
- Diagnostic lymph node biopsy (left cervical)

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Metoprolol tartrate 50 mg orally twice daily
- Lisinopril 20 mg orally daily
- Amlodipine 5 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Lithium carbonate 300 mg orally three times daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Emphysema, deceased age 70
- Brother: Bipolar disorder

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history (1 pack per day). Alcohol: Social, 1-2 drinks per week. Occupation: Disabled former construction worker.

PHYSICAL EXAMINATION:
On physical examination, his blood pressure is 130/85 mmHg, and he has no edema.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum creatinine: 2.1 mg/dL (elevated)
- Blood urea nitrogen (BUN): 35 mg/dL (elevated)
- Serum lithium: 1.2 mEq/L (within therapeutic range)
- Estimated glomerular filtration rate (eGFR): 45 mL/min/1.73m^2 (decreased)
- Urinalysis: No proteinuria or hematuria

ASSESSMENT AND PLAN:
Given the patient's renal function and medication history,

## Encounter 3: source 2021-07-15, practice 2024-09-23, outpatient, General Surgery

Chief complaint: Preoperative evaluation for elective inguinal hernia repair

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James O'Brien, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Preoperative evaluation for elective inguinal hernia repair

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Malignant neoplasm of lymph nodes (diagnosed 2020-04-15)
- Lithium nephrotoxicity (diagnosed 2021-01-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) stage unspecified
- Hypertension
- Coronary artery disease
- Bipolar I disorder (stable on lithium for 10 years)
- Tobacco use disorder
- Malignant neoplasm of cervical lymph nodes (left side)

PAST SURGICAL HISTORY:
- Inguinal hernia repair (elective, date unspecified)
- Diagnostic lymph node biopsy (left cervical)

MEDICATIONS:
He takes lisinopril for his hypertension.

Home Medications:
- Tiotropium 18 mcg inhaled daily
- Metoprolol tartrate 50 mg orally twice daily
- Amlodipine 5 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Lithium carbonate 300 mg orally three times daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally nightly

ALLERGIES:
He has no known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Emphysema, deceased age 70
- Brother: Bipolar disorder

SOCIAL HISTORY:
He reports a 30-pack-year smoking history and currently smokes a pack of cigarettes daily.

VITAL SIGNS:
On physical examination, his blood pressure is 138/82 mmHg, heart rate is 78 beats per minute, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
His lungs have mild expiratory wheezes bilaterally, and his abdomen is soft with a reducible inguinal hernia.

LABORATORY DATA:
Laboratory tests are as follows:

- Hemoglobin: 14.2 g/dL
- White blood cell count: 7.2 x 10^3/uL
- Platelets: 250 x 10^3/uL
- Fasting glucose: 98 mg/dL
- Serum creatinine: 1.0 mg/dL

ASSESSMENT AND PLAN:
Given the patient&#x27;s current smoking status and upcoming elective surgery,

## Encounter 4: source 2022-01-15, practice 2025-03-16, ed, Emergency Medicine

Chief complaint: Acute increased shortness of breath and productive cough for the past week

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute increased shortness of breath and productive cough for the past week

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Malignant neoplasm of lymph nodes (diagnosed 2020-04-15)
- Lithium nephrotoxicity (diagnosed 2021-01-15)
- Tobacco use disorder (diagnosed 2021-07-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) stage unspecified
- Hypertension
- Coronary artery disease
- Bipolar I disorder (stable on lithium for 10 years)
- Malignant neoplasm of cervical lymph nodes (left side)

PAST SURGICAL HISTORY:
- Inguinal hernia repair (elective, date unspecified)
- Diagnostic lymph node biopsy (left cervical)

MEDICATIONS:
He takes currently on tiotropium and metoprolol.

Home Medications:
- Metoprolol tartrate 50 mg orally twice daily
- Lisinopril 20 mg orally daily
- Amlodipine 5 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Lithium carbonate 300 mg orally three times daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Emphysema, deceased age 70
- Brother: Bipolar disorder

SOCIAL HISTORY:
40-pack-year smoking history

VITAL SIGNS:
Blood pressure is 150/90 mmHg, heart rate is 88 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 89% on room air

PHYSICAL EXAMINATION:
Physical examination reveals diffuse wheezes and prolonged expiration.

LABORATORY DATA:
Arterial blood gas analysis on room air shows:
- pH: 7.32
- PaCO2: 55 mmHg
- PaO2: 60 mmHg
- HCO3-: 28 mEq/L

IMAGING:
Chest X-ray shows hyperinflated lungs and increased bronchovascular markings.

PLAN:
The patient is started on a regimen of systemic corticosteroids and antibiotics. A randomized controlled trial is being conducted to compare a new bronchodilator with the current standard of care in patients with COPD exacerbations. The patient expresses interest in participating.

## Encounter 5: source 2022-07-15, practice 2025-09-15, ed, Emergency Medicine

Chief complaint: Altered mental status, lethargy, and confusion noted by wife over the past few days

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Altered mental status, lethargy, and confusion noted by wife over the past few days

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with COPD and bipolar disorder presents with altered mental status. His wife reports that he has been more lethargic over the past few days and was found confused and mumbling incoherently this morning.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Malignant neoplasm of lymph nodes (diagnosed 2020-04-15)
- Lithium nephrotoxicity (diagnosed 2021-01-15)
- Tobacco use disorder (diagnosed 2021-07-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) stage unspecified
- Hypertension
- Coronary artery disease
- Bipolar I disorder (stable on lithium for 10 years)
- Malignant neoplasm of cervical lymph nodes (left side)

PAST SURGICAL HISTORY:
- Inguinal hernia repair (elective, date unspecified)
- Diagnostic lymph node biopsy (left cervical)

MEDICATIONS:
He takes metoprolol, tiotropium, and amlodipine.

Home Medications:
- Metoprolol tartrate 50 mg orally twice daily
- Lisinopril 20 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Lithium carbonate 300 mg orally three times daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Emphysema, deceased age 70
- Brother: Bipolar disorder

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history (1 pack per day). Alcohol: Social, 1-2 drinks per week. Occupation: Disabled former construction worker.

VITAL SIGNS:
Blood pressure is 80/50 mmHg, heart rate is 45 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Wheezing on auscultation of the lungs and cool extremities

LABORATORY DATA:
Fingerstick glucose is 60 mg/dL

OTHER STUDIES:
An ECG shows sinus bradycardia with no ischemic changes

PLAN:
Intravenous fluids and atropine have been administered with no significant improvement in blood pressure or heart rate

## Encounter 6: source 2022-09-15, practice 2025-11-25, ed, Emergency Medicine

Chief complaint: Sudden onset massive hemoptysis, coughing up approximately 100 mL of bright red blood over 2 hours

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset massive hemoptysis, coughing up approximately 100 mL of bright red blood over 2 hours

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old male with a history of COPD and malignant neoplasm of the left cervical lymph nodes who presents to the emergency department with a 2-hour history of massive hemoptysis. He reports that the hemoptysis began suddenly and estimates that he has coughed up approximately 100 mL of bright red blood. He denies chest pain, fever, or recent travel. The patient's hemoptysis continues, and he appears increasingly anxious.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Malignant neoplasm of lymph nodes (diagnosed 2020-04-15)
- Lithium nephrotoxicity (diagnosed 2021-01-15)
- Tobacco use disorder (diagnosed 2021-07-15)
- Beta-blocker overdose (diagnosed 2022-07-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) stage unspecified
- Hypertension
- Coronary artery disease
- Bipolar I disorder (stable on lithium for 10 years)
- Malignant neoplasm of cervical lymph nodes (left side)

PAST SURGICAL HISTORY:
- Inguinal hernia repair (elective, date unspecified)
- Diagnostic lymph node biopsy (left cervical)

MEDICATIONS:
His medications include tiotropium and metoprolol.

Home Medications:
- Metoprolol tartrate 50 mg orally twice daily
- Lisinopril 20 mg orally daily
- Amlodipine 5 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Lithium carbonate 300 mg orally three times daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Emphysema, deceased age 70
- Brother: Bipolar disorder

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history (1 pack per day). Alcohol: Social, 1-2 drinks per week. Occupation: Disabled former construction worker.

VITAL SIGNS:
On examination, his blood pressure is 145/90 mmHg, heart rate is 98 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds and coarse crackles in the right lower lobe. The rest of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 9,000 /µL
- Platelet count: 250,000 /µL
- Prothrombin time (PT): 12 seconds (INR 1.0)
- Partial thromboplastin time (PTT): 30 seconds
- BUN: 18 mg/dL
- Creatinine: 1.0 mg/dL

IMAGING:
Chest X-ray shows a dense consolidation in the right lower lobe.

## Encounter 7: source 2023-01-15, practice 2026-03-30, icu, Cardiology

Chief complaint: Sudden collapse at train station following chest pain and confusion, requiring CPR and defibrillation

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Liu, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Sudden collapse at train station following chest pain and confusion, requiring CPR and defibrillation

HISTORY OF PRESENT ILLNESS:
This 59-year-old male with coronary artery disease, chronic obstructive pulmonary disease, and recent massive hemoptysis and beta-blocker overdose is brought to the emergency department after collapsing suddenly at a train station. Bystanders report he had been clutching his chest and appeared confused before losing consciousness. He received immediate CPR, and paramedics delivered multiple shocks via automated external defibrillator (AED).

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Malignant neoplasm of lymph nodes (diagnosed 2020-04-15)
- Lithium nephrotoxicity (diagnosed 2021-01-15)
- Tobacco use disorder (diagnosed 2021-07-15)
- Beta-blocker overdose (diagnosed 2022-07-15)
- Massive Hemoptysis (diagnosed 2022-09-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) stage unspecified
- Hypertension
- Coronary artery disease
- Bipolar I disorder (stable on lithium for 10 years)
- Malignant neoplasm of cervical lymph nodes (left side)

His medical history includes long-standing hypertension and stable coronary artery disease.

PAST SURGICAL HISTORY:
- Inguinal hernia repair (elective, date unspecified)
- Diagnostic lymph node biopsy (left cervical)

MEDICATIONS:
He takes aspirin, lisinopril, and a statin.

Home Medications:
- Tiotropium 18 mcg inhaled daily
- Metoprolol tartrate 50 mg orally twice daily
- Amlodipine 5 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Lithium carbonate 300 mg orally three times daily
- Atorvastatin 40 mg orally nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62, hypertension
- Mother: Emphysema, deceased age 70
- Brother: Bipolar disorder

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history (1 pack per day). Alcohol: Social, 1-2 drinks per week. Occupation: Disabled former construction worker.

PHYSICAL EXAMINATION:
Upon arrival at the ED, he is unresponsive and without spontaneous circulation.

LABORATORY DATA:
Lab values are pending

IMAGING:
Bedside echocardiogram shows no coordinated ventricular contraction

OTHER STUDIES:
ECG findings are shown below.
