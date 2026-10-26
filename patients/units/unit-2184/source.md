# synthetic_hospital patient 2184

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare Part A and B.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-11-18 (Thu) | outpatient | Pulmonology |
| 1 | 2020-07-15 | 2022-06-06 (Mon) | outpatient | Pulmonology |
| 2 | 2021-07-15 | 2023-05-19 (Fri) | outpatient | Cardiology |
| 3 | 2022-07-15 | 2024-05-27 (Mon) | ed | Emergency Medicine |
| 4 | 2023-01-15 | 2024-11-26 (Tue) | outpatient | Endocrinology |
| 5 | 2023-07-15 | 2025-05-22 (Thu) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2025-11-28 (Fri) | outpatient | Endocrinology |
| 7 | 2024-07-15 | 2026-05-27 (Wed) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfamethoxazole-trimethoprim (anaphylaxis)",
  "Iodinated contrast media (urticaria)"
 ],
 "insurance": "Medicare Part A and B",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Social, 1-2 beers per week",
 "substance_use": "Occasional intranasal cocaine use, last use 1 week prior to presentation",
 "family_history": [
  "Father: Lung cancer (squamous cell), deceased at age 68",
  "Mother: Hypertension, type 2 diabetes mellitus, alive age 89",
  "One older brother: Chronic obstructive pulmonary disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 50 pack-years; quit 6 months ago after multiple previous quit attempts",
 "home_medications": [
  {
   "dose": "250/50 mcg, 1 inhalation twice daily",
   "name": "Fluticasone propionate/salmeterol xinafoate dry powder inhaler"
  },
  {
   "dose": "90 mcg, 2 puffs every 4-6 hours as needed for dyspnea",
   "name": "Albuterol sulfate metered-dose inhaler"
  },
  {
   "dose": "18 mcg inhaled once daily",
   "name": "Tiotropium bromide inhalation powder"
  },
  {
   "dose": "10 mg by mouth daily",
   "name": "Lisinopril"
  },
  {
   "dose": "500 mg by mouth twice daily with meals",
   "name": "Metformin hydrochloride"
  },
  {
   "dose": "40 mg by mouth every morning",
   "name": "Furosemide"
  },
  {
   "dose": "25 mg by mouth daily",
   "name": "Spironolactone"
  }
 ],
 "surgical_history": [
  "Right upper lobectomy with mediastinal lymphadenectomy",
  "Diagnostic bronchoscopy with endobronchial ultrasound-guided transbronchial needle aspiration (EBUS-TBNA)",
  "Implantable central venous port placement"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage 3",
  "Pulmonary hypertension",
  "Cor pulmonale with chronic right heart failure",
  "Hypertension",
  "Type 2 Diabetes Mellitus, diagnosed within past 6 months",
  "Non-small cell lung cancer (adenocarcinoma), right upper lobe, status post lobectomy",
  "Extensive-stage small cell lung carcinoma",
  "Paraneoplastic Cushing syndrome secondary to ectopic adrenocorticotropic hormone (ACTH) secretion",
  "History of syndrome of inappropriate antidiuretic hormone secretion (SIADH)"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Ectopic ACTH production", "Syndrome of inappropriate antidiuretic hormone secretion (SIADH)", "Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease)", "Chronic obstructive pulmonary disease", "Non-small cell lung cancer, right upper lobe", "Extensive-stage small cell lung carcinoma", "Cocaine-induced cerebral vasculopathy"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Tobacco use disorder", "Suspected malignant neoplasm of lung", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-11-18, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath and chronic productive cough

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath and chronic productive cough

HISTORY OF PRESENT ILLNESS:
He reports a chronic productive cough with white sputum. He denies any chest pain, fever, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage 3
- Pulmonary hypertension
- Cor pulmonale with chronic right heart failure
- Hypertension
- Type 2 Diabetes Mellitus, diagnosed within past 6 months
- Non-small cell lung cancer (adenocarcinoma), right upper lobe, status post lobectomy
- Extensive-stage small cell lung carcinoma
- Paraneoplastic Cushing syndrome secondary to ectopic adrenocorticotropic hormone (ACTH) secretion
- History of syndrome of inappropriate antidiuretic hormone secretion (SIADH)

His past medical history is significant for hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Right upper lobectomy with mediastinal lymphadenectomy
- Diagnostic bronchoscopy with endobronchial ultrasound-guided transbronchial needle aspiration (EBUS-TBNA)
- Implantable central venous port placement

MEDICATIONS:
He takes hydrochlorothiazide, metformin.

Home Medications:
- Fluticasone propionate/salmeterol xinafoate dry powder inhaler 250/50 mcg, 1 inhalation twice daily
- Albuterol sulfate metered-dose inhaler 90 mcg, 2 puffs every 4-6 hours as needed for dyspnea
- Tiotropium bromide inhalation powder 18 mcg inhaled once daily
- Lisinopril 10 mg by mouth daily
- Metformin hydrochloride 500 mg by mouth twice daily with meals
- Furosemide 40 mg by mouth every morning
- Spironolactone 25 mg by mouth daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis), Iodinated contrast media (urticaria)

FAMILY HISTORY:
- Father: Lung cancer (squamous cell), deceased at age 68
- Mother: Hypertension, type 2 diabetes mellitus, alive age 89
- One older brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
40-pack-year smoking history

VITAL SIGNS:
His oxygen saturation on room air is 88%

PHYSICAL EXAMINATION:
On physical examination, you note an increased anteroposterior diameter of the chest, decreased breath sounds, and prolonged expiration with wheezes.

LABORATORY DATA:
Arterial blood gas analysis on room air shows:

- pH: 7.42
- PaCO2: 48 mmHg
- PaO2: 55 mmHg
- HCO3-: 30 mEq/L

IMAGING:
Chest X-ray reveals hyperinflated lungs with flattened diaphragms.

## Encounter 1: source 2020-07-15, practice 2022-06-06, outpatient, Pulmonology

Chief complaint: Persistent cough and unintentional weight loss of 10 pounds over the past 3 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough and unintentional weight loss of 10 pounds over the past 3 months

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of persistent cough and unintentional weight loss, previously seen in January for progressive dyspnea and chronic productive cough. He denies any fever, night sweats, or hemoptysis.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage 3
- Pulmonary hypertension
- Cor pulmonale with chronic right heart failure
- Hypertension
- Type 2 Diabetes Mellitus, diagnosed within past 6 months
- Non-small cell lung cancer (adenocarcinoma), right upper lobe, status post lobectomy
- Extensive-stage small cell lung carcinoma
- Paraneoplastic Cushing syndrome secondary to ectopic adrenocorticotropic hormone (ACTH) secretion
- History of syndrome of inappropriate antidiuretic hormone secretion (SIADH)

His past medical history is significant for hypertension, controlled with lisinopril, and mild COPD.

PAST SURGICAL HISTORY:
- Right upper lobectomy with mediastinal lymphadenectomy
- Diagnostic bronchoscopy with endobronchial ultrasound-guided transbronchial needle aspiration (EBUS-TBNA)
- Implantable central venous port placement

MEDICATIONS:
Home Medications:
- Fluticasone propionate/salmeterol xinafoate dry powder inhaler 250/50 mcg, 1 inhalation twice daily
- Albuterol sulfate metered-dose inhaler 90 mcg, 2 puffs every 4-6 hours as needed for dyspnea
- Tiotropium bromide inhalation powder 18 mcg inhaled once daily
- Lisinopril 10 mg by mouth daily
- Metformin hydrochloride 500 mg by mouth twice daily with meals
- Furosemide 40 mg by mouth every morning
- Spironolactone 25 mg by mouth daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis), Iodinated contrast media (urticaria)

FAMILY HISTORY:
- Father: Lung cancer (squamous cell), deceased at age 68
- Mother: Hypertension, type 2 diabetes mellitus, alive age 89
- One older brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
50-pack-year smoking history

VITAL SIGNS:
On physical examination, he is afebrile, blood pressure is 135/85 mmHg, heart rate is 78 beats per minute, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds in the right upper lobe.

IMAGING:
A chest X-ray shows a 3 cm mass in the right upper lobe, and a subsequent CT scan confirms the presence of a solitary pulmonary nodule with spiculated margins.

PATHOLOGY:
A biopsy of the lesion indicates non-small cell lung cancer.

OTHER STUDIES:
Preoperative assessment includes pulmonary function tests.

Pulmonary Function Test Results:
- FEV1: 1.8 L (65% of predicted)
- FVC: 2.8 L (70% of predicted)
- FEV1/FVC ratio: 64%
- DLCO: 60% of predicted

Given the patient&#x27;s history and test results,

PLAN:
The patient is referred to a thoracic surgeon for evaluation for possible lung resection.

## Encounter 2: source 2021-07-15, practice 2023-05-19, outpatient, Cardiology

Chief complaint: Increasing fatigue and swelling in both legs over the past several weeks

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue and swelling in both legs over the past several weeks

HISTORY OF PRESENT ILLNESS:
The patient returns for outpatient cardiology follow-up in the setting of known cor pulmonale with chronic right heart failure, extensive-stage small cell lung carcinoma, and paraneoplastic Cushing syndrome secondary to ectopic ACTH secretion. He reports a sense of tightness in his shoes by the end of the day and notes that walking short distances now leaves him more winded than usual. He denies chest pain, paroxysmal nocturnal dyspnea, or needing to prop himself up at night to breathe. He has a long-standing history of cough productive of white sputum, which has progressively worsened over the years.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage 3
- Pulmonary hypertension
- Cor pulmonale with chronic right heart failure
- Hypertension
- Type 2 Diabetes Mellitus, diagnosed within past 6 months
- Non-small cell lung cancer (adenocarcinoma), right upper lobe, status post lobectomy
- Extensive-stage small cell lung carcinoma
- Paraneoplastic Cushing syndrome secondary to ectopic adrenocorticotropic hormone (ACTH) secretion
- History of syndrome of inappropriate antidiuretic hormone secretion (SIADH)

His past medical history is significant for hypertension and "lung problems," but he does not recall the specifics.

PAST SURGICAL HISTORY:
- Right upper lobectomy with mediastinal lymphadenectomy
- Diagnostic bronchoscopy with endobronchial ultrasound-guided transbronchial needle aspiration (EBUS-TBNA)
- Implantable central venous port placement

MEDICATIONS:
Home Medications:
- Fluticasone propionate/salmeterol xinafoate dry powder inhaler 250/50 mcg, 1 inhalation twice daily
- Albuterol sulfate metered-dose inhaler 90 mcg, 2 puffs every 4-6 hours as needed for dyspnea
- Tiotropium bromide inhalation powder 18 mcg inhaled once daily
- Lisinopril 10 mg by mouth daily
- Metformin hydrochloride 500 mg by mouth twice daily with meals
- Furosemide 40 mg by mouth every morning
- Spironolactone 25 mg by mouth daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis), Iodinated contrast media (urticaria)

FAMILY HISTORY:
- Father: Lung cancer (squamous cell), deceased at age 68
- Mother: Hypertension, type 2 diabetes mellitus, alive age 89
- One older brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
He has smoked two packs of cigarettes daily since the age of 15.

VITAL SIGNS:
On physical examination, his temperature is 36.7°C (98.1°F), blood pressure is 128/76 mm Hg, pulse is 90/min, and respiratory rate is 18/min with an oxygen saturation of 92% on room air.

PHYSICAL EXAMINATION:
Jugular venous pressure (JVP) is 4cm above the sternal angle, and there is a right ventricular heave on palpation. Cardiac auscultation reveals a prominent second heart sound. Bibasilar breath sounds are decreased with occasional wheezing. There is bilateral pitting edema in the lower extremities to the mid-shin.

IMAGING:
An echocardiogram shows normal left ventricular size and function, with right ventricular hypertrophy and mild tricuspid regurgitation.

OTHER STUDIES:
Pulmonary artery catheterization reveals a normal pulmonary capillary wedge pressure.

## Encounter 3: source 2022-07-15, practice 2024-05-27, ed, Emergency Medicine

Chief complaint: Progressive cough, shortness of breath, and unintentional weight loss with chest tightness and hoarseness

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Jennifer Williams, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive cough, shortness of breath, and unintentional weight loss with chest tightness and hoarseness

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with COPD, prior right upper lobectomy for non-small cell lung cancer, and cor pulmonale who presents to the emergency department due to progressive cough, shortness of breath, and unintentional weight loss over the past 3 months. He also reports intermittent chest tightness, difficulty sleeping due to coughing fits, and recent hoarseness. He denies hemoptysis, fever, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2020-07-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2021-07-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage 3
- Pulmonary hypertension
- Cor pulmonale with chronic right heart failure
- Hypertension
- Type 2 Diabetes Mellitus, diagnosed within past 6 months
- Non-small cell lung cancer (adenocarcinoma), right upper lobe, status post lobectomy
- Extensive-stage small cell lung carcinoma
- Paraneoplastic Cushing syndrome secondary to ectopic adrenocorticotropic hormone (ACTH) secretion
- History of syndrome of inappropriate antidiuretic hormone secretion (SIADH)

His medical history is significant for chronic obstructive pulmonary disease and hypertension.

PAST SURGICAL HISTORY:
- Right upper lobectomy with mediastinal lymphadenectomy
- Diagnostic bronchoscopy with endobronchial ultrasound-guided transbronchial needle aspiration (EBUS-TBNA)
- Implantable central venous port placement

MEDICATIONS:
Home Medications:
- Fluticasone propionate/salmeterol xinafoate dry powder inhaler 250/50 mcg, 1 inhalation twice daily
- Albuterol sulfate metered-dose inhaler 90 mcg, 2 puffs every 4-6 hours as needed for dyspnea
- Tiotropium bromide inhalation powder 18 mcg inhaled once daily
- Lisinopril 10 mg by mouth daily
- Metformin hydrochloride 500 mg by mouth twice daily with meals
- Furosemide 40 mg by mouth every morning
- Spironolactone 25 mg by mouth daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis), Iodinated contrast media (urticaria)

FAMILY HISTORY:
- Father: Lung cancer (squamous cell), deceased at age 68
- Mother: Hypertension, type 2 diabetes mellitus, alive age 89
- One older brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
He has a 50-pack-year smoking history and continues to smoke daily.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical exam, he appears thin and mildly dyspneic. He has decreased breath sounds and dullness to percussion over the right upper lung field. No cervical or axillary lymphadenopathy is noted. Neurologic exam is normal.

LABORATORY DATA:
Laboratory studies are significant for sodium of 126 mmol/L and serum osmolality of 260 mOsm/kg.

IMAGING:
Chest x-ray shows a large right hilar mass, and CT scan of the chest reveals a 5.5-cm centrally located mass obstructing the right mainstem bronchus with bulky mediastinal lymphadenopathy. MRI of the brain shows two subcentimeter enhancing lesions in the frontal lobe. CT abdomen reveals scattered liver nodules consistent with metastases.

PATHOLOGY:
A biopsy is shown.

## Encounter 4: source 2023-01-15, practice 2024-11-26, outpatient, Endocrinology

Chief complaint: Progressive weakness, fatigue, unintentional weight gain, and new difficulty controlling blood glucose over the past four months

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive weakness, fatigue, unintentional weight gain, and new difficulty controlling blood glucose over the past four months

HISTORY OF PRESENT ILLNESS:
The patient presents for evaluation in the context of his extensive-stage small cell lung carcinoma and COPD with cor pulmonale. He reports feeling increasingly tired when climbing stairs to his apartment and notices that his face appears fuller than usual in recent photos. He has also developed easy bruising on his arms and legs and mild difficulty controlling his blood glucose despite no prior history of diabetes.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2020-07-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2021-07-15)
- Extensive-stage small cell lung carcinoma (diagnosed 2022-07-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage 3
- Pulmonary hypertension
- Cor pulmonale with chronic right heart failure
- Hypertension
- Type 2 Diabetes Mellitus, diagnosed within past 6 months
- Non-small cell lung cancer (adenocarcinoma), right upper lobe, status post lobectomy
- Paraneoplastic Cushing syndrome secondary to ectopic adrenocorticotropic hormone (ACTH) secretion
- History of syndrome of inappropriate antidiuretic hormone secretion (SIADH)

His medical history is significant for chronic obstructive pulmonary disease, for which he uses inhalers as needed.

PAST SURGICAL HISTORY:
- Right upper lobectomy with mediastinal lymphadenectomy
- Diagnostic bronchoscopy with endobronchial ultrasound-guided transbronchial needle aspiration (EBUS-TBNA)
- Implantable central venous port placement

MEDICATIONS:
Home Medications:
- Fluticasone propionate/salmeterol xinafoate dry powder inhaler 250/50 mcg, 1 inhalation twice daily
- Albuterol sulfate metered-dose inhaler 90 mcg, 2 puffs every 4-6 hours as needed for dyspnea
- Tiotropium bromide inhalation powder 18 mcg inhaled once daily
- Lisinopril 10 mg by mouth daily
- Metformin hydrochloride 500 mg by mouth twice daily with meals
- Furosemide 40 mg by mouth every morning
- Spironolactone 25 mg by mouth daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis), Iodinated contrast media (urticaria)

FAMILY HISTORY:
- Father: Lung cancer (squamous cell), deceased at age 68
- Mother: Hypertension, type 2 diabetes mellitus, alive age 89
- One older brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
He smoked one pack of cigarettes daily for 45 years but quit last year.

VITAL SIGNS:
On examination, blood pressure is 150/92 mmHg and BMI is 26 kg/m².

PHYSICAL EXAMINATION:
Physical examination shows central adiposity with thin extremities and scattered ecchymoses over the forearms. Chest auscultation reveals decreased breath sounds in the right upper lung field.

LABORATORY DATA:
Laboratory evaluation prior to dexamethasone administration shows:
Plasma cortisol Elevated
Plasma ACTH                        Elevated

PLAN:
Further evaluation includes a high-dose dexamethasone suppression test.

## Encounter 5: source 2023-07-15, practice 2025-05-22, ed, Emergency Medicine

Chief complaint: Worsening fatigue, mild unsteadiness when walking, and difficulty concentrating

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening fatigue, mild unsteadiness when walking, and difficulty concentrating

HISTORY OF PRESENT ILLNESS:
The 70-year-old male with extensive-stage small cell lung carcinoma and paraneoplastic Cushing syndrome presents with worsening fatigue, mild unsteadiness when walking, and difficulty concentrating over the past week. He reports decreased appetite and mild nausea but no vomiting or diarrhea. He was recently hospitalized for pneumonia treated with antibiotics.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2020-07-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2021-07-15)
- Extensive-stage small cell lung carcinoma (diagnosed 2022-07-15)
- Ectopic ACTH production (diagnosed 2023-01-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage 3
- Pulmonary hypertension
- Cor pulmonale with chronic right heart failure
- Hypertension
- Type 2 Diabetes Mellitus, diagnosed within past 6 months
- Non-small cell lung cancer (adenocarcinoma), right upper lobe, status post lobectomy
- Paraneoplastic Cushing syndrome secondary to ectopic adrenocorticotropic hormone (ACTH) secretion
- History of syndrome of inappropriate antidiuretic hormone secretion (SIADH)

PAST SURGICAL HISTORY:
- Right upper lobectomy with mediastinal lymphadenectomy
- Diagnostic bronchoscopy with endobronchial ultrasound-guided transbronchial needle aspiration (EBUS-TBNA)
- Implantable central venous port placement

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Fluticasone propionate/salmeterol xinafoate dry powder inhaler 250/50 mcg, 1 inhalation twice daily
- Albuterol sulfate metered-dose inhaler 90 mcg, 2 puffs every 4-6 hours as needed for dyspnea
- Tiotropium bromide inhalation powder 18 mcg inhaled once daily
- Metformin hydrochloride 500 mg by mouth twice daily with meals
- Furosemide 40 mg by mouth every morning
- Spironolactone 25 mg by mouth daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis), Iodinated contrast media (urticaria)

FAMILY HISTORY:
- Father: Lung cancer (squamous cell), deceased at age 68
- Mother: Hypertension, type 2 diabetes mellitus, alive age 89
- One older brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
He quit smoking 5 years ago after a 40-pack-year history.

VITAL SIGNS:
Temperature is 36.7°C (98.1°F), blood pressure is 130/82 mmHg, pulse is 78/min, and BMI is 24 kg/m²

PHYSICAL EXAMINATION:
Mucous membranes are moist, and there is no peripheral edema or jugular venous distension. Neurological examination reveals mild confusion but is otherwise normal.

IMAGING:
A chest x-ray today reveals a right hilar mass with associated mediastinal lymphadenopathy.

## Encounter 6: source 2024-01-15, practice 2025-11-28, outpatient, Endocrinology

Chief complaint: Progressive fatigue and difficulty rising from a chair over the past month

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. David Kim, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and difficulty rising from a chair over the past month

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of previously diagnosed ectopic ACTH production and presents with a 1-month history of progressive fatigue and difficulty rising from a chair. He has also noted easy bruising and recent weight gain.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2020-07-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2021-07-15)
- Extensive-stage small cell lung carcinoma (diagnosed 2022-07-15)
- Ectopic ACTH production (diagnosed 2023-01-15)
- Syndrome of inappropriate antidiuretic hormone secretion (SIADH) (diagnosed 2023-07-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage 3
- Pulmonary hypertension
- Cor pulmonale with chronic right heart failure
- Hypertension
- Type 2 Diabetes Mellitus, diagnosed within past 6 months
- Non-small cell lung cancer (adenocarcinoma), right upper lobe, status post lobectomy
- Paraneoplastic Cushing syndrome secondary to ectopic adrenocorticotropic hormone (ACTH) secretion

PAST SURGICAL HISTORY:
- Right upper lobectomy with mediastinal lymphadenectomy
- Diagnostic bronchoscopy with endobronchial ultrasound-guided transbronchial needle aspiration (EBUS-TBNA)
- Implantable central venous port placement

MEDICATIONS:
Home Medications:
- Fluticasone propionate/salmeterol xinafoate dry powder inhaler 250/50 mcg, 1 inhalation twice daily
- Albuterol sulfate metered-dose inhaler 90 mcg, 2 puffs every 4-6 hours as needed for dyspnea
- Tiotropium bromide inhalation powder 18 mcg inhaled once daily
- Lisinopril 10 mg by mouth daily
- Metformin hydrochloride 500 mg by mouth twice daily with meals
- Furosemide 40 mg by mouth every morning
- Spironolactone 25 mg by mouth daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis), Iodinated contrast media (urticaria)

FAMILY HISTORY:
- Father: Lung cancer (squamous cell), deceased at age 68
- Mother: Hypertension, type 2 diabetes mellitus, alive age 89
- One older brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
Smoking: Former smoker, 50 pack-years; quit 6 months ago after multiple previous quit attempts. Alcohol: Social, 1-2 beers per week. Occupation: Retired construction foreman.

VITAL SIGNS:
His blood pressure is 160/92 mm Hg

PHYSICAL EXAMINATION:
Physical examination reveals proximal muscle weakness in both upper and lower extremities.

LABORATORY DATA:
Fasting glucose is 162 mg/dL.

IMAGING:
A chest x-ray reveals a left hilar mass,

PATHOLOGY:
Biopsy of the lesion demonstrates sheets of small, round cells with hyperchromatic nuclei and scant cytoplasm. The tumor stains positive for chromogranin A and synaptophysin. No intercellular bridges or keratin pearls are observed. There is absent glandular morphology.

ASSESSMENT AND PLAN:
Further workup confirms a paraneoplastic syndrome. Increased release of a hormone with

## Encounter 7: source 2024-07-15, practice 2026-05-27, icu, Emergency Medicine

Chief complaint: Increasing confusion and lethargy over the past 24 hours

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Increasing confusion and lethargy over the past 24 hours

HISTORY OF PRESENT ILLNESS:
The patient, with extensive-stage small cell lung carcinoma, paraneoplastic Cushing syndrome, and prior SIADH, is brought to the emergency department by his family for increasing confusion and lethargy over the past 24 hours. His family reports that he has been less responsive and has not been taking his medications regularly for the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2020-07-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2021-07-15)
- Extensive-stage small cell lung carcinoma (diagnosed 2022-07-15)
- Ectopic ACTH production (diagnosed 2023-01-15)
- Syndrome of inappropriate antidiuretic hormone secretion (SIADH) (diagnosed 2023-07-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage 3
- Pulmonary hypertension
- Cor pulmonale with chronic right heart failure
- Hypertension
- Type 2 Diabetes Mellitus, diagnosed within past 6 months
- Non-small cell lung cancer (adenocarcinoma), right upper lobe, status post lobectomy
- Paraneoplastic Cushing syndrome secondary to ectopic adrenocorticotropic hormone (ACTH) secretion

PAST SURGICAL HISTORY:
- Right upper lobectomy with mediastinal lymphadenectomy
- Diagnostic bronchoscopy with endobronchial ultrasound-guided transbronchial needle aspiration (EBUS-TBNA)
- Implantable central venous port placement

MEDICATIONS:
Home Medications:
- Fluticasone propionate/salmeterol xinafoate dry powder inhaler 250/50 mcg, 1 inhalation twice daily
- Albuterol sulfate metered-dose inhaler 90 mcg, 2 puffs every 4-6 hours as needed for dyspnea
- Tiotropium bromide inhalation powder 18 mcg inhaled once daily
- Lisinopril 10 mg by mouth daily
- Metformin hydrochloride 500 mg by mouth twice daily with meals
- Furosemide 40 mg by mouth every morning
- Spironolactone 25 mg by mouth daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (anaphylaxis), Iodinated contrast media (urticaria)

FAMILY HISTORY:
- Father: Lung cancer (squamous cell), deceased at age 68
- Mother: Hypertension, type 2 diabetes mellitus, alive age 89
- One older brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
50-pack-year smoking history

VITAL SIGNS:
His vital signs are as follows: blood pressure 90/60 mmHg, heart rate 110 bpm, respiratory rate 28 breaths/min, and oxygen saturation of 88% on room air.

PHYSICAL EXAMINATION:
On examination, the patient is obtunded, with a Glasgow Coma Scale score of 8. Physical examination reveals bilateral wheezing, use of accessory muscles for breathing, and peripheral cyanosis.

LABORATORY DATA:
Arterial blood gas shows pH 7.25, PaCO2 70 mmHg, PaO2 55 mmHg, HCO3- 26 mEq/L, and SaO2 88%.

IMAGING:
Despite aggressive management, the patient's neurological status does not improve, and a subsequent CT scan of the head reveals a large intracranial hemorrhage with midline shift.

PLAN:
The patient is intubated for airway protection and mechanical ventilation is initiated. The prognosis is deemed poor, and after discussions with the family, they agree to withdraw life-sustaining treatments in accordance with the patient's previously expressed wishes.
