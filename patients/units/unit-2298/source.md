# synthetic_hospital patient 2298

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare Part A and B, no prescription drug coverage (Part D).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-12-01 (Thu) | outpatient | Urology |
| 1 | 2020-04-15 | 2023-03-03 (Fri) | outpatient | Infectious Disease |
| 2 | 2020-07-15 | 2023-06-12 (Mon) | outpatient | Pulmonary & Critical Care |
| 3 | 2020-09-15 | 2023-08-01 (Tue) | outpatient | Oncology |
| 4 | 2020-11-15 | 2023-10-06 (Fri) | follow_up | Oncology |
| 5 | 2021-03-15 | 2024-02-06 (Tue) | outpatient | Neurology |
| 6 | 2023-01-15 | 2025-12-01 (Mon) | inpatient | Cardiology |
| 7 | 2023-03-15 | 2026-02-09 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare Part A and B, no prescription drug coverage (Part D)",
 "occupation": "Retired tire manufacturing plant worker",
 "alcohol_use": "Occasional",
 "family_history": [
  "Father: Myocardial infarction at age 68",
  "Mother: Lung cancer",
  "Brother: Prostate cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Active smoker, 45 pack-year history",
 "home_medications": [
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "50 mg daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg daily",
   "name": "Furosemide"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours PRN",
   "name": "Albuterol"
  },
  {
   "dose": "7.5 mg intramuscular monthly",
   "name": "Leuprolide"
  },
  {
   "dose": "15 mg four times daily",
   "name": "Amifampridine"
  },
  {
   "dose": "5 mg oral every 6 hours PRN pain",
   "name": "Oxycodone"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention (date and details unknown)",
  "Transurethral resection of bladder tumor"
 ],
 "chronic_conditions": [
  "Ischemic cardiomyopathy with reduced ejection fraction",
  "Coronary artery disease status post myocardial infarction",
  "Chronic obstructive pulmonary disease",
  "History of pulmonary tuberculosis (treated, inactive cavitary lesions)",
  "Metastatic prostate adenocarcinoma",
  "Urothelial carcinoma of the bladder",
  "Small cell lung cancer, right upper lobe",
  "Lambert-Eaton myasthenic syndrome"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Acute Decompensated Heart Failure with Pulmonary Edema", "Cauda equina syndrome", "Bladder cancer", "Suspected malignant neoplasm of lung", "Pulmonary aspergillosis", "Lambert-Eaton myasthenic syndrome", "Non-small cell lung cancer, right upper lobe"]
Comorbidities: ["Tobacco use disorder", "Malignancy-associated hypercalcemia"]

## Encounter 0: source 2020-01-15, practice 2022-12-01, outpatient, Urology

Chief complaint: Intermittent painless hematuria for 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent painless hematuria for 6 months

HISTORY OF PRESENT ILLNESS:
He reports intermittent blood in his urine over the past 6 months but denies any pain, dysuria, urgency, frequency, or flank discomfort. denies trauma or recent instrumentation

PAST MEDICAL HISTORY:
Active Problem List:
- Ischemic cardiomyopathy with reduced ejection fraction
- Coronary artery disease status post myocardial infarction
- Chronic obstructive pulmonary disease
- History of pulmonary tuberculosis (treated, inactive cavitary lesions)
- Metastatic prostate adenocarcinoma
- Urothelial carcinoma of the bladder
- Small cell lung cancer, right upper lobe
- Lambert-Eaton myasthenic syndrome

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (date and details unknown)
- Transurethral resection of bladder tumor

MEDICATIONS:
He is.

Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Metoprolol succinate 50 mg daily
- Lisinopril 10 mg daily
- Furosemide 40 mg daily
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours PRN
- Leuprolide 7.5 mg intramuscular monthly
- Amifampridine 15 mg four times daily
- Oxycodone 5 mg oral every 6 hours PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Lung cancer
- Brother: Prostate cancer

SOCIAL HISTORY:
He has smoked on and off for 40 years and worked during this time in rubber and tire manufacturing

VITAL SIGNS:
Vital signs are unremarkable

PHYSICAL EXAMINATION:
His physical exam and vital signs are unremarkable

LABORATORY DATA:
Urinalysis confirms hematuria but is negative for nitrites, leukocyte esterase, or proteinuria. Serum creatinine and BUN are normal.

## Encounter 1: source 2020-04-15, practice 2023-03-03, outpatient, Infectious Disease

Chief complaint: Intermittent cough and streaks of blood in sputum with weight loss

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Okafor, MD
Department: Infectious Disease
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent cough and streaks of blood in sputum with weight loss

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old male with metastatic prostate adenocarcinoma, urothelial carcinoma of the bladder, and small cell lung cancer who was last evaluated by Urology in January for hematuria. He returns with intermittent cough and streaks of blood in sputum and has also experienced unintentional weight loss and fatigue over the past several months.

PAST MEDICAL HISTORY:
Active Problem List:
- Bladder cancer (diagnosed 2020-01-15)
- Ischemic cardiomyopathy with reduced ejection fraction
- Coronary artery disease status post myocardial infarction
- Chronic obstructive pulmonary disease
- History of pulmonary tuberculosis (treated, inactive cavitary lesions)
- Metastatic prostate adenocarcinoma
- Urothelial carcinoma of the bladder
- Small cell lung cancer, right upper lobe
- Lambert-Eaton myasthenic syndrome

He has a history of treated pulmonary tuberculosis from decades ago.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (date and details unknown)
- Transurethral resection of bladder tumor

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Metoprolol succinate 50 mg daily
- Lisinopril 10 mg daily
- Furosemide 40 mg daily
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours PRN
- Leuprolide 7.5 mg intramuscular monthly
- Amifampridine 15 mg four times daily
- Oxycodone 5 mg oral every 6 hours PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Lung cancer
- Brother: Prostate cancer

SOCIAL HISTORY:
He has smoked one pack of cigarettes daily for 40 years.

REVIEW OF SYSTEMS:
He does not report fevers, chills, or night sweats currently.

PHYSICAL EXAMINATION:
Physical examination shows digital clubbing and decreased breath sounds in the right upper lobe. The patient is hemodynamically stable and not in acute distress. There is no evidence of lymphadenopathy or systemic signs of infection.

IMAGING:
Chest x-ray reveals a cavitary lesion in the upper lobe of the right lung with a radiodense mass inside. Further imaging confirms the presence of a mobile mass within the preexisting cavity.

## Encounter 2: source 2020-07-15, practice 2023-06-12, outpatient, Pulmonary & Critical Care

Chief complaint: Persistent cough and recent onset of blood-tinged sputum

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Jennifer Walsh, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough and recent onset of blood-tinged sputum

HISTORY OF PRESENT ILLNESS:
Patient with small cell lung cancer and prior pulmonary aspergillosis presents with 3-month history of persistent cough and recent onset of blood-tinged sputum. He also reports 10-pound weight loss and intermittent night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Bladder cancer (diagnosed 2020-01-15)
- Pulmonary aspergillosis (diagnosed 2020-04-15)
- Ischemic cardiomyopathy with reduced ejection fraction
- Coronary artery disease status post myocardial infarction
- Chronic obstructive pulmonary disease
- History of pulmonary tuberculosis (treated, inactive cavitary lesions)
- Metastatic prostate adenocarcinoma
- Urothelial carcinoma of the bladder
- Small cell lung cancer, right upper lobe
- Lambert-Eaton myasthenic syndrome

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (date and details unknown)
- Transurethral resection of bladder tumor

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Metoprolol succinate 50 mg daily
- Lisinopril 10 mg daily
- Furosemide 40 mg daily
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours PRN
- Leuprolide 7.5 mg intramuscular monthly
- Amifampridine 15 mg four times daily
- Oxycodone 5 mg oral every 6 hours PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Lung cancer
- Brother: Prostate cancer

SOCIAL HISTORY:
45-pack-year smoking history

PHYSICAL EXAMINATION:
On examination, he appears cachectic. Auscultation of the lungs reveals decreased breath sounds over the right upper lobe.

LABORATORY DATA:
Laboratory findings are notable for hypercalcemia.

IMAGING:
A chest X-ray shows a right hilar mass. A CT scan of the chest confirms the presence of a 4 cm mass in the right hilum with associated mediastinal lymphadenopathy.

## Encounter 3: source 2020-09-15, practice 2023-08-01, outpatient, Oncology

Chief complaint: Persistent cough, fatigue, and difficulty swallowing solid foods

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Robert Kim, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough, fatigue, and difficulty swallowing solid foods

HISTORY OF PRESENT ILLNESS:
The patient returns for pulmonary follow-up. He also reports unintentional weight loss and difficulty swallowing solid foods.

PAST MEDICAL HISTORY:
Active Problem List:
- Bladder cancer (diagnosed 2020-01-15)
- Pulmonary aspergillosis (diagnosed 2020-04-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-07-15)
- Ischemic cardiomyopathy with reduced ejection fraction
- Coronary artery disease status post myocardial infarction
- Chronic obstructive pulmonary disease
- History of pulmonary tuberculosis (treated, inactive cavitary lesions)
- Metastatic prostate adenocarcinoma
- Urothelial carcinoma of the bladder
- Small cell lung cancer, right upper lobe
- Lambert-Eaton myasthenic syndrome

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (date and details unknown)
- Transurethral resection of bladder tumor

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Metoprolol succinate 50 mg daily
- Lisinopril 10 mg daily
- Furosemide 40 mg daily
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours PRN
- Leuprolide 7.5 mg intramuscular monthly
- Amifampridine 15 mg four times daily
- Oxycodone 5 mg oral every 6 hours PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Lung cancer
- Brother: Prostate cancer

SOCIAL HISTORY:
He has smoked one pack of cigarettes daily for the past 45 years.

REVIEW OF SYSTEMS:
He denies any hemoptysis or recent respiratory infections.

PHYSICAL EXAMINATION:
On physical examination, he has decreased breath sounds over the right lung field and digital clubbing.

LABORATORY DATA:
Laboratory studies:
Hemoglobin 13.4 g/dL
WBCs 7,200 /mm³
Platelets 275,000 /mm³
Serum sodium 139 mEq/L
Serum potassium 4.1 mEq/L
Serum calcium 12.2 mg/dL
Serum creatinine 0.9 mg/dL

IMAGING:
Chest imaging reveals a centrally located cavitary mass in the right upper lobe.

PLAN:
Bronchoscopy is scheduled for further evaluation.

## Encounter 4: source 2020-11-15, practice 2023-10-06, follow_up, Oncology

Chief complaint: Follow-up appointment for recently diagnosed lung cancer

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. Robert Kim, MD
Department: Oncology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up appointment for recently diagnosed lung cancer

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up evaluation in the context of recently diagnosed small cell bronchogenic carcinoma, following prior encounters for suspected pulmonary malignancy and diagnostic workup. He was recently diagnosed with small cell bronchogenic carcinoma.

PAST MEDICAL HISTORY:
Active Problem List:
- Bladder cancer (diagnosed 2020-01-15)
- Pulmonary aspergillosis (diagnosed 2020-04-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-07-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2020-09-15)
- Ischemic cardiomyopathy with reduced ejection fraction
- Coronary artery disease status post myocardial infarction
- Chronic obstructive pulmonary disease
- History of pulmonary tuberculosis (treated, inactive cavitary lesions)
- Metastatic prostate adenocarcinoma
- Urothelial carcinoma of the bladder
- Small cell lung cancer, right upper lobe
- Lambert-Eaton myasthenic syndrome

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (date and details unknown)
- Transurethral resection of bladder tumor

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Metoprolol succinate 50 mg daily
- Lisinopril 10 mg daily
- Furosemide 40 mg daily
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours PRN
- Leuprolide 7.5 mg intramuscular monthly
- Amifampridine 15 mg four times daily
- Oxycodone 5 mg oral every 6 hours PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Lung cancer
- Brother: Prostate cancer

SOCIAL HISTORY:
Heavy smoking

## Encounter 5: source 2021-03-15, practice 2024-02-06, outpatient, Neurology

Chief complaint: Progressive difficulty rising from chairs, climbing stairs, and lifting objects

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Priya Patel, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty rising from chairs, climbing stairs, and lifting objects

HISTORY OF PRESENT ILLNESS:
The patient returns for neurologic evaluation in the setting of small cell lung cancer. He presents with progressive difficulty rising from chairs, climbing stairs, and lifting objects over the past 4 weeks, reporting occasional dry mouth but no diplopia, dysphagia, or sensory changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Bladder cancer (diagnosed 2020-01-15)
- Pulmonary aspergillosis (diagnosed 2020-04-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-07-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2020-09-15)
- Ischemic cardiomyopathy with reduced ejection fraction
- Coronary artery disease status post myocardial infarction
- Chronic obstructive pulmonary disease
- History of pulmonary tuberculosis (treated, inactive cavitary lesions)
- Metastatic prostate adenocarcinoma
- Urothelial carcinoma of the bladder
- Small cell lung cancer, right upper lobe
- Lambert-Eaton myasthenic syndrome

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (date and details unknown)
- Transurethral resection of bladder tumor

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Metoprolol succinate 50 mg daily
- Lisinopril 10 mg daily
- Furosemide 40 mg daily
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours PRN
- Leuprolide 7.5 mg intramuscular monthly
- Amifampridine 15 mg four times daily
- Oxycodone 5 mg oral every 6 hours PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Lung cancer
- Brother: Prostate cancer

SOCIAL HISTORY:
He has a 45-pack-year smoking history

PHYSICAL EXAMINATION:
On examination, he has symmetric proximal muscle weakness affecting both upper and lower extremities. His deep tendon reflexes are diminished but improve after brief muscle exercise. Muscle strength also improves with repeated testing. Sensory exam is normal.

IMAGING:
A chest radiograph shows a left hilar mass.

OTHER STUDIES:
Electrophysiologic studies reveal a reduced baseline muscle response that incrementally increases with high-frequency repetitive nerve stimulation.

## Encounter 6: source 2023-01-15, practice 2025-12-01, inpatient, Cardiology

Chief complaint: Acutely worsening shortness of breath and orthopnea

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Henderson, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Acutely worsening shortness of breath and orthopnea

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with known ischemic cardiomyopathy with reduced ejection fraction, coronary artery disease status post myocardial infarction, chronic obstructive pulmonary disease, metastatic prostate adenocarcinoma, history of urothelial carcinoma of the bladder, small cell lung cancer of the right upper lobe, and Lambert-Eaton myasthenic syndrome who presents with acutely worsening respiratory symptoms. He was previously evaluated for bladder cancer in January 2020, diagnosed with lung cancer in September 2020, and developed Lambert-Eaton myasthenic syndrome by March 2021. He states that symptoms initially began months ago with mild breathlessness during exercise, such as walking up a hill or carrying groceries. Over time, he began to feel short of breath performing basic tasks like showering or dressing, and now he becomes winded simply moving from one room to another in his home. He reports that he currently sleeps with three pillows under his head and often wakes up gasping for air in the middle of the night. Over the past three days, his legs have become increasingly swollen, and he has noticed tightness in his shoes by the afternoon. He denies chest pain, fever, or cough. He has also felt more fatigued, occasionally dizzy when standing, and finds it difficult to eat full meals due to early satiety and bloating. His appetite has decreased. He notes that for the past two days, he has felt as though his breathing becomes worse when lying flat, forcing him to sleep upright in a recliner. He denies hemoptysis, wheezing, or known lung disease.

PAST MEDICAL HISTORY:
Active Problem List:
- Bladder cancer (diagnosed 2020-01-15)
- Pulmonary aspergillosis (diagnosed 2020-04-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-07-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2020-09-15)
- Lambert-Eaton myasthenic syndrome (diagnosed 2021-03-15)
- Ischemic cardiomyopathy with reduced ejection fraction
- Coronary artery disease status post myocardial infarction
- Chronic obstructive pulmonary disease
- History of pulmonary tuberculosis (treated, inactive cavitary lesions)
- Metastatic prostate adenocarcinoma
- Urothelial carcinoma of the bladder
- Small cell lung cancer, right upper lobe

He mentions that he had a heart attack "several years ago" but doesn’t recall what treatment was done.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (date and details unknown)
- Transurethral resection of bladder tumor

MEDICATIONS:
He has not taken his medications regularly due to cost and admits to not refilling his prescriptions in over a month.

Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Metoprolol succinate 50 mg daily
- Lisinopril 10 mg daily
- Furosemide 40 mg daily
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours PRN
- Leuprolide 7.5 mg intramuscular monthly
- Amifampridine 15 mg four times daily
- Oxycodone 5 mg oral every 6 hours PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Lung cancer
- Brother: Prostate cancer

SOCIAL HISTORY:
He has smoked a pack of cigarettes daily for 40 years and drinks alcohol occasionally.

PHYSICAL EXAMINATION:
On arrival, he appears mildly distressed and is breathing rapidly. He has a mildly elevated jugular venous pressure and bilateral pitting edema of the lower extremities to the mid-shin. Lung auscultation reveals bilateral basilar crackles without wheezing. His breathing worsens when lying flat for the exam. There are no signs of infection, and cardiac auscultation reveals a displaced point of maximal impulse.

IMAGING:
Chest x-ray demonstrates bilateral vascular congestion and small pleural effusions.

OTHER STUDIES:
ECG shows sinus tachycardia with Q waves in the inferior leads.

## Encounter 7: source 2023-03-15, practice 2026-02-09, ed, Emergency Medicine

Chief complaint: Acute worsening lower back pain, leg weakness, and urinary retention

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute worsening lower back pain, leg weakness, and urinary retention

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with a history of metastatic prostate adenocarcinoma, small cell lung cancer, and Lambert-Eaton myasthenic syndrome who presents to the Emergency Department with acute worsening lower back pain. He reports that the pain began as a dull ache in the lumbar region, but has progressively increased in intensity and is now associated with shooting pains down both legs. He also notes recent difficulty initiating urination and a sensation of incomplete bladder emptying. Over the past two days, he has developed numbness in the inner thighs and buttocks.

PAST MEDICAL HISTORY:
Active Problem List:
- Bladder cancer (diagnosed 2020-01-15)
- Pulmonary aspergillosis (diagnosed 2020-04-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-07-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2020-09-15)
- Lambert-Eaton myasthenic syndrome (diagnosed 2021-03-15)
- Acute Decompensated Heart Failure with Pulmonary Edema (diagnosed 2023-01-15)
- Ischemic cardiomyopathy with reduced ejection fraction
- Coronary artery disease status post myocardial infarction
- Chronic obstructive pulmonary disease
- History of pulmonary tuberculosis (treated, inactive cavitary lesions)
- Metastatic prostate adenocarcinoma
- Urothelial carcinoma of the bladder
- Small cell lung cancer, right upper lobe

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention (date and details unknown)
- Transurethral resection of bladder tumor

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Metoprolol succinate 50 mg daily
- Lisinopril 10 mg daily
- Furosemide 40 mg daily
- Tiotropium 18 mcg inhaled daily
- Albuterol 90 mcg inhaled every 4-6 hours PRN
- Leuprolide 7.5 mg intramuscular monthly
- Amifampridine 15 mg four times daily
- Oxycodone 5 mg oral every 6 hours PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Lung cancer
- Brother: Prostate cancer

SOCIAL HISTORY:
He has a 30-pack-year smoking history

VITAL SIGNS:
Vital signs are within normal limits

PHYSICAL EXAMINATION:
Neurologic exam reveals decreased patellar and Achilles reflexes bilaterally and reduced sensation over the perineum. Motor strength is 3/5 in the lower extremities bilaterally. Rectal tone is decreased.
