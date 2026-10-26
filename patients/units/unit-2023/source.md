# synthetic_hospital patient 2023

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex M, Non-Hispanic White, Commercial preferred provider organization (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-02-20 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2023-06-07 (Wed) | outpatient | Pulmonology |
| 2 | 2021-03-15 | 2024-04-25 (Thu) | outpatient | Gastroenterology |
| 3 | 2021-07-15 | 2024-08-30 (Fri) | outpatient | Internal Medicine |
| 4 | 2021-08-15 | 2024-09-27 (Fri) | outpatient | Pulmonology |
| 5 | 2021-11-15 | 2024-12-30 (Mon) | outpatient | Hematology & Oncology |
| 6 | 2022-05-15 | 2025-06-28 (Sat) | ed | Emergency Medicine |
| 7 | 2022-11-15 | 2025-12-23 (Tue) | outpatient | Urology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Seasonal pollen (tree/grass)",
  "No known drug allergies"
 ],
 "insurance": "Commercial preferred provider organization (PPO)",
 "occupation": "Long-haul truck driver",
 "alcohol_use": "Rare (1-2 drinks per year)",
 "family_history": [
  "Father: Lung cancer (diagnosed age 62)",
  "Mother: Hypertension, Type 2 diabetes",
  "Brother: Coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker with 35-40 pack-year history; multiple documented quit attempts with cessation dates variably reported as 2, 5, and 10 years prior to encounters",
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
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "90 mcg inhaler, 2 puffs every 4 hours PRN shortness of breath",
   "name": "Albuterol"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Omeprazole"
  },
  {
   "dose": "10 mg PO daily PRN allergies",
   "name": "Cetirizine"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (childhood)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Chronic obstructive pulmonary disease",
  "Gastroesophageal reflux disease",
  "Seasonal allergic rhinitis"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["Non-small cell lung cancer, right upper lobe", "Gastric adenocarcinoma", "Lung cancer screening candidate (high-risk smoker)", "Obstructive sleep apnea", "Bladder cancer"]
Comorbidities: ["Non-small cell lung cancer, right upper lobe", "History of recent pneumonia", "Family history of malignant neoplasm of lung", "Family history of breast cancer", "Cancer-related constitutional symptoms", "Chronic obstructive pulmonary disease", "Tobacco use disorder", "Cachexia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2023-02-20, outpatient, Internal Medicine

Chief complaint: Routine health maintenance visit and lung cancer screening discussion

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance visit and lung cancer screening discussion

HISTORY OF PRESENT ILLNESS:
He quit smoking 2 years ago after a chronic cough and shortness of breath prompted him to seek medical advice. He denies any current respiratory symptoms and is up to date with his vaccinations.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Chronic obstructive pulmonary disease
- Gastroesophageal reflux disease
- Seasonal allergic rhinitis

His past medical history is significant for hypertension, controlled with lisinopril, and hyperlipidemia, managed with atorvastatin.

PAST SURGICAL HISTORY:
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Albuterol 90 mcg inhaler, 2 puffs every 4 hours PRN shortness of breath
- Omeprazole 20 mg PO daily
- Cetirizine 10 mg PO daily PRN allergies

ALLERGIES:
Seasonal pollen (tree/grass), No known drug allergies

FAMILY HISTORY:
His family history is notable for lung cancer in his father, who was a heavy smoker.

SOCIAL HISTORY:
Smoking: Former smoker with 35-40 pack-year history; multiple documented quit attempts with cessation dates variably reported as 2, 5, and 10 years prior to encounters. Alcohol: Rare (1-2 drinks per year). Occupation: Long-haul truck driver.

VITAL SIGNS:
On physical examination, his blood pressure is 130/85 mmHg, heart rate is 78 beats per minute, respiratory rate is 16 breaths per minute, and oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
His lung examination is clear to auscultation bilaterally, and the rest of the physical examination is unremarkable.

ASSESSMENT AND PLAN:
Given this patient's history and current status,

## Encounter 1: source 2020-04-15, practice 2023-06-07, outpatient, Pulmonology

Chief complaint: Persistent fatigue, morning headaches, and declining concentration affecting work performance

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, morning headaches, and declining concentration affecting work performance

HISTORY OF PRESENT ILLNESS:
He returns three months after Internal Medicine lung cancer screening with HTN, HLD, COPD, GERD, and allergic rhinitis. He is a long-haul truck driver who consumes multiple energy drinks daily and reports poor sleep quality. His wife notes that he snores loudly and often appears to stop breathing during sleep, followed by gasping episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Chronic obstructive pulmonary disease
- Gastroesophageal reflux disease
- Seasonal allergic rhinitis

His past medical history includes hypertension and seasonal allergies.

PAST SURGICAL HISTORY:
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Albuterol 90 mcg inhaler, 2 puffs every 4 hours PRN shortness of breath
- Omeprazole 20 mg PO daily
- Cetirizine 10 mg PO daily PRN allergies

ALLERGIES:
Seasonal pollen (tree/grass), No known drug allergies

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 62)
- Mother: Hypertension, Type 2 diabetes
- Brother: Coronary artery disease

SOCIAL HISTORY:
He quit smoking 5 years ago after a 30 pack-year history.

REVIEW OF SYSTEMS:
He denies recent illness, chest pain, or cough.

PHYSICAL EXAMINATION:
The patient has a BMI of 36 kg/m² and a neck circumference of 18 inches. On exam, his blood pressure is 150/92 mm Hg, and his oropharynx shows narrowing without lesions. Cardiopulmonary examination is unremarkable.

LABORATORY DATA:
Arterial blood gas on room air reveals a pH of 7.36, a partial pressure of carbon dioxide (PaCO₂) of 50 mm Hg, and a partial pressure of oxygen (PaO₂) of 68 mm Hg.

## Encounter 2: source 2021-03-15, practice 2024-04-25, outpatient, Gastroenterology

Chief complaint: Progressive epigastric pain, early satiety, and 10-pound weight loss over 4 months

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive epigastric pain, early satiety, and 10-pound weight loss over 4 months

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old male with chronic GERD on daily omeprazole and high-risk smoking status previously evaluated for lung cancer screening and fatigue with morning headaches. He presents to the Gastroenterology clinic with a 4-month history of progressive epigastric pain, early satiety, and a 10-pound weight loss. He reports a feeling of fullness after eating small amounts of food and occasional postprandial vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-01-15)
- Obstructive sleep apnea (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Chronic obstructive pulmonary disease
- Gastroesophageal reflux disease
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Albuterol 90 mcg inhaler, 2 puffs every 4 hours PRN shortness of breath
- Omeprazole 20 mg PO daily
- Cetirizine 10 mg PO daily PRN allergies

ALLERGIES:
Seasonal pollen (tree/grass), No known drug allergies

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 62)
- Mother: Hypertension, Type 2 diabetes
- Brother: Coronary artery disease

SOCIAL HISTORY:
30-pack-year smoking history, although he quit smoking 10 years ago

PHYSICAL EXAMINATION:
On physical examination, there is mild tenderness to palpation in the epigastric region. No masses are palpable, and there is no lymphadenopathy.

LABORATORY DATA:
Laboratory studies show:
- Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 102 fL (Normal: 80-96 fL)
- Carcinoembryonic antigen (CEA): Elevated

IMAGING:
A CT scan of the abdomen and pelvis with contrast is performed for staging.

PATHOLOGY:
Biopsy confirms poorly differentiated adenocarcinoma

OTHER STUDIES:
An upper endoscopy reveals a 3 cm ulcerated mass in the gastric antrum

## Encounter 3: source 2021-07-15, practice 2024-08-30, outpatient, Internal Medicine

Chief complaint: Persistent cough and unexplained weight loss over the past 3 months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Jennifer Okafor, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough and unexplained weight loss over the past 3 months

HISTORY OF PRESENT ILLNESS:
The patient returns four months following gastric adenocarcinoma diagnosis, presenting to the clinic complaining of a persistent cough and unexplained weight loss over the past 3 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-01-15)
- Obstructive sleep apnea (diagnosed 2020-04-15)
- Gastric adenocarcinoma (diagnosed 2021-03-15)
- Hypertension
- Hyperlipidemia
- Chronic obstructive pulmonary disease
- Gastroesophageal reflux disease
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Albuterol 90 mcg inhaler, 2 puffs every 4 hours PRN shortness of breath
- Omeprazole 20 mg PO daily
- Cetirizine 10 mg PO daily PRN allergies

ALLERGIES:
Seasonal pollen (tree/grass), No known drug allergies

FAMILY HISTORY:
His family history is significant for lung cancer in his father.

SOCIAL HISTORY:
With a history of smoking

REVIEW OF SYSTEMS:
He denies any fever, night sweats, or hemoptysis.

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic. Lung auscultation reveals decreased breath sounds in the right upper lobe.

IMAGING:
A chest X-ray shows a mass in the right upper lobe,

PATHOLOGY:
A subsequent biopsy confirms non-small cell lung cancer.

OTHER STUDIES:
The patient is enrolled in a study that aims to identify potential risk factors for lung cancer. The study includes patients with lung cancer and a matched control group without lung cancer. Researchers collect data on the participants' smoking history, occupational exposures, and family history of cancer.

## Encounter 4: source 2021-08-15, practice 2024-09-27, outpatient, Pulmonology

Chief complaint: Persistent cough and unexplained weight loss over the past 3 months

ENCOUNTER NOTE
Date: 2021-08-15
Provider: Dr. James Patterson, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough and unexplained weight loss over the past 3 months

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old male with COPD and previously identified lung cancer screening candidacy who returns for continued evaluation of persistent cough and weight loss initially presented one month prior, following recent assessment for epigastric pain and similar weight loss. He denies any fever, night sweats, or hemoptysis.

PAST MEDICAL HISTORY:
Active Problem List:
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-01-15)
- Obstructive sleep apnea (diagnosed 2020-04-15)
- Gastric adenocarcinoma (diagnosed 2021-03-15)
- Hypertension
- Hyperlipidemia
- Chronic obstructive pulmonary disease
- Gastroesophageal reflux disease
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Albuterol 90 mcg inhaler, 2 puffs every 4 hours PRN shortness of breath
- Omeprazole 20 mg PO daily
- Cetirizine 10 mg PO daily PRN allergies

ALLERGIES:
Seasonal pollen (tree/grass), No known drug allergies

FAMILY HISTORY:
His family history is significant for lung cancer in his father.

SOCIAL HISTORY:
Smoking

PHYSICAL EXAMINATION:
On physical examination, you note a well-nourished male in no acute distress. Lung auscultation reveals decreased breath sounds in the right upper lobe.

IMAGING:
A chest X-ray shows a 3 cm mass in the right upper lobe. A subsequent CT scan confirms the presence of a solitary pulmonary nodule with spiculated margins.

PATHOLOGY:
A biopsy of the lesion is performed, and histopathology reveals non-small cell lung cancer (NSCLC).

## Encounter 5: source 2021-11-15, practice 2024-12-30, outpatient, Hematology & Oncology

Chief complaint: Follow-up for persistent cough and unintentional weight loss of 10 kg over past 3 months

ENCOUNTER NOTE
Date: 2021-11-15
Provider: Dr. David Kim, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up for persistent cough and unintentional weight loss of 10 kg over past 3 months

HISTORY OF PRESENT ILLNESS:
The patient returns for oncology follow-up of his lung and gastric malignancies. He reports persistent cough and unintentional weight loss of 10 kg over the past 3 months. He denies fever, night sweats, or hemoptysis.

PAST MEDICAL HISTORY:
Active Problem List:
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-01-15)
- Obstructive sleep apnea (diagnosed 2020-04-15)
- Gastric adenocarcinoma (diagnosed 2021-03-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2021-08-15)
- Hypertension
- Hyperlipidemia
- Chronic obstructive pulmonary disease
- Gastroesophageal reflux disease
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Albuterol 90 mcg inhaler, 2 puffs every 4 hours PRN shortness of breath
- Omeprazole 20 mg PO daily
- Cetirizine 10 mg PO daily PRN allergies

ALLERGIES:
Seasonal pollen (tree/grass), No known drug allergies

FAMILY HISTORY:
His family history is significant for lung cancer in his father.

SOCIAL HISTORY:
Smoking: Former smoker with 35-40 pack-year history; multiple documented quit attempts with cessation dates variably reported as 2, 5, and 10 years prior to encounters. Alcohol: Rare (1-2 drinks per year). Occupation: Long-haul truck driver.

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic. Lung auscultation reveals decreased breath sounds in the right upper lobe. No lymphadenopathy is palpable. The patient appears anxious and asks about the results of the biopsy. Before discussing the biopsy results,

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 13.2 g/dL (Normal: 13.8-17.2 g/dL)
- White blood cell count: 7,000/uL (Normal: 4,500-11,000/uL)
- Platelets: 250,000/uL (Normal: 150,000-450,000/uL)
- Sodium: 140 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 4.2 mEq/L (Normal: 3.5-5.1 mEq/L)
- Creatinine: 1.0 mg/dL (Normal: 0.6-1.2 mg/dL)

IMAGING:
A chest X-ray shows a 4 cm mass in the right upper lobe, and a CT scan confirms the presence of the mass with no evidence of metastasis.

PATHOLOGY:
A biopsy of the mass is performed, and the pathology report indicates non-small cell lung carcinoma.

## Encounter 6: source 2022-05-15, practice 2025-06-28, ed, Emergency Medicine

Chief complaint: 3-month history of unintentional weight loss, persistent cough, and new hemoptysis

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-month history of unintentional weight loss, persistent cough, and new hemoptysis

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old male with a history of gastric adenocarcinoma and non-small cell lung cancer, previously evaluated in Hematology and Oncology for similar complaints, who presents to the emergency department with ongoing symptoms. He denies fever, night sweats, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-01-15)
- Obstructive sleep apnea (diagnosed 2020-04-15)
- Gastric adenocarcinoma (diagnosed 2021-03-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2021-08-15)
- Hypertension
- Hyperlipidemia
- Chronic obstructive pulmonary disease
- Gastroesophageal reflux disease
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Tonsillectomy (childhood)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Albuterol 90 mcg inhaler, 2 puffs every 4 hours PRN shortness of breath
- Omeprazole 20 mg PO daily
- Cetirizine 10 mg PO daily PRN allergies

ALLERGIES:
Seasonal pollen (tree/grass), No known drug allergies

FAMILY HISTORY:
His family history is significant for lung cancer in his father.

SOCIAL HISTORY:
History of smoking

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic. Auscultation of the lungs reveals decreased breath sounds in the right upper lobe.

LABORATORY DATA:
Laboratory tests are unremarkable.

IMAGING:
A chest X-ray shows a 4 cm mass in the right upper lobe. A CT scan of the chest confirms the presence of the mass with mediastinal lymphadenopathy. A positron emission tomography (PET) scan using a radiolabeled glucose analog is performed to assess for metastatic disease. The PET scan shows increased uptake in the right upper lobe mass, mediastinal lymph nodes, liver, and brain. There is also activity noted in the kidneys and the urinary bladder.

## Encounter 7: source 2022-11-15, practice 2025-12-23, outpatient, Urology

Chief complaint: 2-month history of painless hematuria without clots

ENCOUNTER NOTE
Date: 2022-11-15
Provider: Dr. Robert Thompson, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
2-month history of painless hematuria without clots

HISTORY OF PRESENT ILLNESS:
Patient with history of gastric and lung cancer presents with two-month painless hematuria. He reports that the blood is present throughout micturition and has not noticed any clots. He denies any dysuria, urgency, or frequency.

PAST MEDICAL HISTORY:
Active Problem List:
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-01-15)
- Obstructive sleep apnea (diagnosed 2020-04-15)
- Gastric adenocarcinoma (diagnosed 2021-03-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2021-08-15)
- Hypertension
- Hyperlipidemia
- Chronic obstructive pulmonary disease
- Gastroesophageal reflux disease
- Seasonal allergic rhinitis

His past medical history includes hypertension,.

PAST SURGICAL HISTORY:
- Tonsillectomy (childhood)

MEDICATIONS:
He takes for which he takes hydrochlorothiazide,.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Albuterol 90 mcg inhaler, 2 puffs every 4 hours PRN shortness of breath
- Omeprazole 20 mg PO daily
- Cetirizine 10 mg PO daily PRN allergies

ALLERGIES:
Seasonal pollen (tree/grass), No known drug allergies

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 62)
- Mother: Hypertension, Type 2 diabetes
- Brother: Coronary artery disease

SOCIAL HISTORY:
A 30-pack-year smoking history, although he quit smoking 10 years ago.

PHYSICAL EXAMINATION:
On physical examination, there is no costovertebral angle tenderness, and the abdominal exam is unremarkable.

LABORATORY DATA:
His urinalysis results are as follows:

- Color: Red
- pH: 6.0
- Specific gravity: 1.010
- Red blood cells: >50/hpf
- White blood cells: 2-5/hpf
- Nitrites: Negative
- Leukocyte esterase: Negative
- Protein: Negative
