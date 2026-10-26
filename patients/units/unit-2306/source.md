# synthetic_hospital patient 2306

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 64 at first encounter, sex M, Non-Hispanic White, Medicare and supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-06-27 (Mon) | outpatient | Pulmonary and Critical Care |
| 1 | 2020-04-15 | 2022-09-16 (Fri) | outpatient | Pulmonary and Critical Care |
| 2 | 2020-09-15 | 2023-02-27 (Mon) | outpatient | Gastroenterology |
| 3 | 2021-05-15 | 2023-10-20 (Fri) | outpatient | Urology |
| 4 | 2022-01-15 | 2024-06-28 (Fri) | outpatient | Pulmonary and Critical Care |
| 5 | 2022-07-15 | 2024-12-27 (Fri) | outpatient | Pulmonary and Critical Care |
| 6 | 2023-01-15 | 2025-06-18 (Wed) | ed | Emergency Medicine |
| 7 | 2023-11-15 | 2026-04-27 (Mon) | outpatient | Urology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfa drugs (rash)",
  "Shellfish"
 ],
 "insurance": "Medicare and supplemental private insurance",
 "occupation": "Retired shipyard mechanic and metal fabricator",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Father: Lung cancer (diagnosed age 72, deceased)",
  "Mother: Type 2 diabetes mellitus and pancreatic cancer (diagnosed age 78)",
  "Brother: Prostate cancer (diagnosed age 68)",
  "Paternal grandfather: Bladder cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-years, quit 3 years ago",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "18 mcg inhaled once daily",
   "name": "Tiotropium bromide"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for dyspnea",
   "name": "Albuterol sulfate"
  },
  {
   "dose": "0.4 mg PO once daily at bedtime",
   "name": "Tamsulosin"
  },
  {
   "dose": "650 mg PO every 6 hours as needed for pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "2 L/minute nasal cannula nocturnal",
   "name": "Supplemental oxygen"
  }
 ],
 "surgical_history": [
  "Appendectomy (age 34)",
  "Right knee arthroscopy (age 52)"
 ],
 "chronic_conditions": [
  "Combined pulmonary fibrosis and emphysema (CPFE)",
  "Chronic obstructive pulmonary disease with emphysema (diagnosed age 56)",
  "Idiopathic pulmonary fibrosis",
  "Diabetes mellitus (new-onset, secondary to pancreatic insufficiency)",
  "Benign prostatic hyperplasia with lower urinary tract symptoms",
  "Tobacco use disorder in sustained remission"
 ],
 "age_at_first_encounter": 64
}
```

Primary diagnoses: ["Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation", "Prostate Cancer (Suspected)", "Bladder cancer", "Idiopathic Pulmonary Fibrosis", "Hyperchloremic metabolic acidosis due to excess normal saline infusion", "Suspected malignant neoplasm of lung"]
Comorbidities: ["Cachexia", "Tobacco use disorder"]

## Encounter 0: source 2020-01-15, practice 2022-06-27, outpatient, Pulmonary and Critical Care

Chief complaint: Progressive shortness of breath over the past year with difficulty climbing stairs and dry cough

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonary and Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath over the past year with difficulty climbing stairs and dry cough

HISTORY OF PRESENT ILLNESS:
He says he has difficulty climbing stairs and often has a dry cough. He denies fever, hemoptysis, chest pain, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Combined pulmonary fibrosis and emphysema (CPFE)
- Chronic obstructive pulmonary disease with emphysema (diagnosed age 56)
- Idiopathic pulmonary fibrosis
- Diabetes mellitus (new-onset, secondary to pancreatic insufficiency)
- Benign prostatic hyperplasia with lower urinary tract symptoms
- Tobacco use disorder in sustained remission

PAST SURGICAL HISTORY:
- Appendectomy (age 34)
- Right knee arthroscopy (age 52)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Tiotropium bromide 18 mcg inhaled once daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for dyspnea
- Tamsulosin 0.4 mg PO once daily at bedtime
- Acetaminophen 650 mg PO every 6 hours as needed for pain
- Supplemental oxygen 2 L/minute nasal cannula nocturnal

ALLERGIES:
Sulfa drugs (rash), Shellfish

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 72, deceased)
- Mother: Type 2 diabetes mellitus and pancreatic cancer (diagnosed age 78)
- Brother: Prostate cancer (diagnosed age 68)
- Paternal grandfather: Bladder cancer

SOCIAL HISTORY:
He has a 40-pack-year smoking history but quit 3 years ago

VITAL SIGNS:
Temperature 36.9°C (98.4°F), blood pressure 126/ 78 mm Hg, pulse 82/min, respiratory rate 20/min, oxygen saturation 94% on room air

PHYSICAL EXAMINATION:
Pulmonary auscultation reveals bilateral end-inspiratory crackles at the bases. There is no clubbing, cyanosis, or peripheral edema.

IMAGING:
A high-resolution CT of the lung is shown .

OTHER STUDIES:
Pulmonary function testing reveals decreased FEV1, normal FEV1/FVC ratio, reduced total lung capacity and DLCO

## Encounter 1: source 2020-04-15, practice 2022-09-16, outpatient, Pulmonary and Critical Care

Chief complaint: Progressive dyspnea, dry cough, and fatigue over 8 months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonary and Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea, dry cough, and fatigue over 8 months

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Combined pulmonary fibrosis and emphysema (CPFE)
- Chronic obstructive pulmonary disease with emphysema (diagnosed age 56)
- Diabetes mellitus (new-onset, secondary to pancreatic insufficiency)
- Benign prostatic hyperplasia with lower urinary tract symptoms
- Tobacco use disorder in sustained remission

PAST SURGICAL HISTORY:
- Appendectomy (age 34)
- Right knee arthroscopy (age 52)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Tiotropium bromide 18 mcg inhaled once daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for dyspnea
- Tamsulosin 0.4 mg PO once daily at bedtime
- Acetaminophen 650 mg PO every 6 hours as needed for pain
- Supplemental oxygen 2 L/minute nasal cannula nocturnal

ALLERGIES:
Sulfa drugs (rash), Shellfish

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 72, deceased)
- Mother: Type 2 diabetes mellitus and pancreatic cancer (diagnosed age 78)
- Brother: Prostate cancer (diagnosed age 68)
- Paternal grandfather: Bladder cancer

SOCIAL HISTORY:
He has a 40-pack-year smoking history and worked as a shipyard mechanic for 30 years.

PHYSICAL EXAMINATION:
Physical exam reveals clubbing of the fingers and bibasilar end-inspiratory crackles.

IMAGING:
High-resolution CT scan of the chest reveals subpleural reticulation, honeycombing, and traction bronchiectasis predominantly in the lower lobes. There is no evidence of pleural or supra-diaphragmatic thickening, plaques, or calcification.

OTHER STUDIES:
Pulmonary function testing shows a decreased FVC and FEV 1 , with a normal FEV1/FVC ratio. DLCO is decreased.

## Encounter 2: source 2020-09-15, practice 2023-02-27, outpatient, Gastroenterology

Chief complaint: 3-month history of epigastric pain radiating to the back, unexplained weight loss of 15 pounds, and new-onset diabetes

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of epigastric pain radiating to the back, unexplained weight loss of 15 pounds, and new-onset diabetes

HISTORY OF PRESENT ILLNESS:
The patient is a 64-year-old male with combined pulmonary fibrosis and emphysema, chronic obstructive pulmonary disease, and idiopathic pulmonary fibrosis previously evaluated in Pulmonary and Critical Care in January and April 2020, who now presents to Gastroenterology with a three-month history of epigastric pain radiating to the back, fifteen pounds of unexplained weight loss, and new-onset diabetes. He denies any changes in bowel habits or jaundice.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Combined pulmonary fibrosis and emphysema (CPFE)
- Chronic obstructive pulmonary disease with emphysema (diagnosed age 56)
- Diabetes mellitus (new-onset, secondary to pancreatic insufficiency)
- Benign prostatic hyperplasia with lower urinary tract symptoms
- Tobacco use disorder in sustained remission

PAST SURGICAL HISTORY:
- Appendectomy (age 34)
- Right knee arthroscopy (age 52)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Tiotropium bromide 18 mcg inhaled once daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for dyspnea
- Tamsulosin 0.4 mg PO once daily at bedtime
- Acetaminophen 650 mg PO every 6 hours as needed for pain
- Supplemental oxygen 2 L/minute nasal cannula nocturnal

ALLERGIES:
Sulfa drugs (rash), Shellfish

FAMILY HISTORY:
His sister was diagnosed with pancreatic cancer at the age of 58.

SOCIAL HISTORY:
With a 40-pack-year smoking history

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic. His abdomen is soft, with mild tenderness in the epigastric region without palpable masses.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 12.1 g/dL (normal: 13.8-17.2 g/dL)
- Total bilirubin: 1.2 mg/dL (normal: 0.3-1.2 mg/dL)
- Alkaline phosphatase: 120 U/L (normal: 40-130 U/L)
- CA 19-9: 560 U/mL (normal: 0-37 U/mL)
- Fasting blood glucose: 156 mg/dL (normal: 70-99 mg/dL)

IMAGING:
An abdominal CT scan shows a 3 cm mass in the head of the pancreas with no evidence of metastasis.

## Encounter 3: source 2021-05-15, practice 2023-10-20, outpatient, Urology

Chief complaint: 2-week history of painless gross hematuria

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. David Kim, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of painless gross hematuria

HISTORY OF PRESENT ILLNESS:
This patient with CPFE and prior pancreatic cancer presents for hematuria. He denies any recent infections or use of medications known to cause cystitis.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-09-15)
- Combined pulmonary fibrosis and emphysema (CPFE)
- Chronic obstructive pulmonary disease with emphysema (diagnosed age 56)
- Diabetes mellitus (new-onset, secondary to pancreatic insufficiency)
- Benign prostatic hyperplasia with lower urinary tract symptoms
- Tobacco use disorder in sustained remission

PAST SURGICAL HISTORY:
- Appendectomy (age 34)
- Right knee arthroscopy (age 52)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Tiotropium bromide 18 mcg inhaled once daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for dyspnea
- Tamsulosin 0.4 mg PO once daily at bedtime
- Acetaminophen 650 mg PO every 6 hours as needed for pain
- Supplemental oxygen 2 L/minute nasal cannula nocturnal

ALLERGIES:
Sulfa drugs (rash), Shellfish

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 72, deceased)
- Mother: Type 2 diabetes mellitus and pancreatic cancer (diagnosed age 78)
- Brother: Prostate cancer (diagnosed age 68)
- Paternal grandfather: Bladder cancer

SOCIAL HISTORY:
He has a 40-pack-year smoking history and worked as a metal worker until his retirement 5 years ago.

VITAL SIGNS:
His blood pressure is 130/85 mmHg

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Urinalysis confirms gross hematuria without evidence of infection. Serum creatinine is 1.1 mg/dL, and blood urea nitrogen is 14 mg/dL.

## Encounter 4: source 2022-01-15, practice 2024-06-28, outpatient, Pulmonary and Critical Care

Chief complaint: Fatigue during the day

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonary and Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue during the day

HISTORY OF PRESENT ILLNESS:
The patient returns to the pulmonary clinic for routine follow-up of his combined pulmonary fibrosis and emphysema and idiopathic pulmonary fibrosis, in the context of prior diagnoses of pancreatic adenocarcinoma and bladder cancer. He presents for ongoing management of his chronic lung disease. He uses supplemental oxygen at night and walks slowly with the help of a cane. He reports that he has not had any recent hospitalizations and has been feeling about the same as usual. His main complaint is fatigue during the day, which he attributes to poor sleep. He has a daily productive cough, but no increase in sputum volume or change in color.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-09-15)
- Bladder cancer (diagnosed 2021-05-15)
- Combined pulmonary fibrosis and emphysema (CPFE)
- Chronic obstructive pulmonary disease with emphysema (diagnosed age 56)
- Diabetes mellitus (new-onset, secondary to pancreatic insufficiency)
- Benign prostatic hyperplasia with lower urinary tract symptoms
- Tobacco use disorder in sustained remission

He has a long history of tobacco use and was diagnosed with emphysema 8 years ago.

PAST SURGICAL HISTORY:
- Appendectomy (age 34)
- Right knee arthroscopy (age 52)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Tiotropium bromide 18 mcg inhaled once daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for dyspnea
- Tamsulosin 0.4 mg PO once daily at bedtime
- Acetaminophen 650 mg PO every 6 hours as needed for pain
- Supplemental oxygen 2 L/minute nasal cannula nocturnal

ALLERGIES:
Sulfa drugs (rash), Shellfish

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 72, deceased)
- Mother: Type 2 diabetes mellitus and pancreatic cancer (diagnosed age 78)
- Brother: Prostate cancer (diagnosed age 68)
- Paternal grandfather: Bladder cancer

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 3 years ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired shipyard mechanic and metal fabricator.

VITAL SIGNS:
His respiratory rate is 16/min, and his oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
On exam, he is alert, speaking in full sentences, and not in respiratory distress. Physical exam reveals decreased breath sounds bilaterally and mild wheezing. There is no peripheral edema or jugular venous distention.

LABORATORY DATA:
Arterial blood gas is obtained as part of routine follow-up.

IMAGING:
A chest x-ray shows flattened diaphragms and increased retrosternal air space.

## Encounter 5: source 2022-07-15, practice 2024-12-27, outpatient, Pulmonary and Critical Care

Chief complaint: Progressive muscle weakness over the past few months with difficulty climbing stairs

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonary and Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive muscle weakness over the past few months with difficulty climbing stairs

HISTORY OF PRESENT ILLNESS:
Patient with CPFE and prior pancreatic and bladder malignancies returns with progressive weakness. He reports difficulty climbing stairs and rising from chairs, but states his strength improves with repeated effort. He also complains of a persistent cough and a 6.8-kg (15-lb) unintentional weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-09-15)
- Bladder cancer (diagnosed 2021-05-15)
- Hyperchloremic metabolic acidosis due to excess normal saline infusion (diagnosed 2022-01-15)
- Combined pulmonary fibrosis and emphysema (CPFE)
- Chronic obstructive pulmonary disease with emphysema (diagnosed age 56)
- Diabetes mellitus (new-onset, secondary to pancreatic insufficiency)
- Benign prostatic hyperplasia with lower urinary tract symptoms
- Tobacco use disorder in sustained remission

PAST SURGICAL HISTORY:
- Appendectomy (age 34)
- Right knee arthroscopy (age 52)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Tiotropium bromide 18 mcg inhaled once daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for dyspnea
- Tamsulosin 0.4 mg PO once daily at bedtime
- Acetaminophen 650 mg PO every 6 hours as needed for pain
- Supplemental oxygen 2 L/minute nasal cannula nocturnal

ALLERGIES:
Sulfa drugs (rash), Shellfish

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 72, deceased)
- Mother: Type 2 diabetes mellitus and pancreatic cancer (diagnosed age 78)
- Brother: Prostate cancer (diagnosed age 68)
- Paternal grandfather: Bladder cancer

SOCIAL HISTORY:
He has a 40-pack-year smoking history.

PHYSICAL EXAMINATION:
On physical exam, deep tendon reflexes are diminished in the lower extremities but improve after exercise.

IMAGING:
A chest x-ray reveals a right hilar mass

PATHOLOGY:
A biopsy is obtained. The histology is shown below:

## Encounter 6: source 2023-01-15, practice 2025-06-18, ed, Emergency Medicine

Chief complaint: Confusion, fatigue, and nausea developed over the past week

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Confusion, fatigue, and nausea developed over the past week

HISTORY OF PRESENT ILLNESS:
The patient with combined pulmonary fibrosis and emphysema and suspected malignant neoplasm of the lung presents to the emergency department with confusion, fatigue, and nausea that have developed over the past week. Despite fluid restriction, his sodium levels remain low, and he continues to experience mild confusion.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-09-15)
- Bladder cancer (diagnosed 2021-05-15)
- Hyperchloremic metabolic acidosis due to excess normal saline infusion (diagnosed 2022-01-15)
- Suspected malignant neoplasm of lung (diagnosed 2022-07-15)
- Combined pulmonary fibrosis and emphysema (CPFE)
- Chronic obstructive pulmonary disease with emphysema (diagnosed age 56)
- Diabetes mellitus (new-onset, secondary to pancreatic insufficiency)
- Benign prostatic hyperplasia with lower urinary tract symptoms
- Tobacco use disorder in sustained remission

PAST SURGICAL HISTORY:
- Appendectomy (age 34)
- Right knee arthroscopy (age 52)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Tiotropium bromide 18 mcg inhaled once daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for dyspnea
- Tamsulosin 0.4 mg PO once daily at bedtime
- Acetaminophen 650 mg PO every 6 hours as needed for pain
- Supplemental oxygen 2 L/minute nasal cannula nocturnal

ALLERGIES:
Sulfa drugs (rash), Shellfish

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 72, deceased)
- Mother: Type 2 diabetes mellitus and pancreatic cancer (diagnosed age 78)
- Brother: Prostate cancer (diagnosed age 68)
- Paternal grandfather: Bladder cancer

SOCIAL HISTORY:
His medical history is significant for chronic smoking.

PHYSICAL EXAMINATION:
On physical examination, he appears euvolemic without peripheral edema, orthostatic hypotension, or jugular venous distention.

LABORATORY DATA:
Laboratory studies reveal a serum sodium of 120 mEq/L, serum osmolality of 260 mOsm/kg, and urine osmolality of 500 mOsm/kg.

IMAGING:
A recent chest CT scan showed a central lung mass with hilar lymphadenopathy.

## Encounter 7: source 2023-11-15, practice 2026-04-27, outpatient, Urology

Chief complaint: Worsening back pain and weight loss

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. Lisa Thompson, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening back pain and weight loss

HISTORY OF PRESENT ILLNESS:
The patient with combined pulmonary fibrosis and emphysema and prior bladder cancer returns for urological follow-up with worsening back pain and weight loss. He has a history of urinary hesitancy and nocturia.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-09-15)
- Bladder cancer (diagnosed 2021-05-15)
- Hyperchloremic metabolic acidosis due to excess normal saline infusion (diagnosed 2022-01-15)
- Suspected malignant neoplasm of lung (diagnosed 2022-07-15)
- Combined pulmonary fibrosis and emphysema (CPFE)
- Chronic obstructive pulmonary disease with emphysema (diagnosed age 56)
- Diabetes mellitus (new-onset, secondary to pancreatic insufficiency)
- Benign prostatic hyperplasia with lower urinary tract symptoms
- Tobacco use disorder in sustained remission

PAST SURGICAL HISTORY:
- Appendectomy (age 34)
- Right knee arthroscopy (age 52)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Tiotropium bromide 18 mcg inhaled once daily
- Albuterol sulfate 90 mcg inhaled every 4-6 hours as needed for dyspnea
- Tamsulosin 0.4 mg PO once daily at bedtime
- Acetaminophen 650 mg PO every 6 hours as needed for pain
- Supplemental oxygen 2 L/minute nasal cannula nocturnal

ALLERGIES:
Sulfa drugs (rash), Shellfish

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 72, deceased)
- Mother: Type 2 diabetes mellitus and pancreatic cancer (diagnosed age 78)
- Brother: Prostate cancer (diagnosed age 68)
- Paternal grandfather: Bladder cancer

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 3 years ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired shipyard mechanic and metal fabricator.

PHYSICAL EXAMINATION:
Prostate exam reveals a firm, nodular prostate.

LABORATORY DATA:
Serum prostate-specific antigen is markedly elevated.

IMAGING:
Bone scan reveals multiple areas of increased uptake in the lumbar spine and pelvis.

PATHOLOGY:
A biopsy confirms adenocarcinoma.

PLAN:
He is started on pharmacologic therapy that initially increases serum testosterone levels before resulting in desensitization.
