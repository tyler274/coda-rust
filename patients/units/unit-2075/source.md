# synthetic_hospital patient 2075

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 53 at first encounter, sex M, Non-Hispanic White, Private (Employer-sponsored Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-07-20 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-05-15 | 2020-11-16 (Mon) | outpatient | Pulmonology |
| 2 | 2021-05-15 | 2021-11-26 (Fri) | outpatient | Pulmonology |
| 3 | 2022-05-15 | 2022-11-13 (Sun) | ed | Emergency Medicine |
| 4 | 2023-05-15 | 2023-11-13 (Mon) | outpatient | Pulmonology |
| 5 | 2023-09-15 | 2024-03-25 (Mon) | outpatient | Oncology |
| 6 | 2024-03-15 | 2024-09-25 (Wed) | outpatient | Oncology |
| 7 | 2025-05-15 | 2025-11-13 (Thu) | outpatient | Pulmonology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private (Employer-sponsored Blue Cross Blue Shield)",
 "occupation": "Construction demolition laborer",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father: Lung cancer (diagnosed age 68, deceased)",
  "Mother: Chronic obstructive pulmonary disease",
  "Brother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker (1 pack per day since age 13; 40 pack-year history)",
 "home_medications": [
  {
   "dose": "250/50 mcg, 1 inhalation twice daily",
   "name": "Fluticasone propionate/salmeterol"
  },
  {
   "dose": "90 mcg inhaler, 2 puffs every 4-6 hours as needed for shortness of breath",
   "name": "Albuterol"
  },
  {
   "dose": "25 mg by mouth once daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 12)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD)",
  "Hypertension",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 53
}
```

Primary diagnoses: ["Chronic obstructive pulmonary disease", "Digital Clubbing", "Stage IIIB Non-Small Cell Lung Cancer", "Lung cancer screening candidate (high-risk smoker)", "Pulmonary aspergillosis", "Lead poisoning", "Non-small cell lung cancer, right upper lobe", "Tobacco use disorder"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Tobacco use disorder", "History of recent pneumonia"]

## Encounter 0: source 2020-01-15, practice 2020-07-20, outpatient, Internal Medicine

Chief complaint: Annual physical examination and routine check-up

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Annual physical examination and routine check-up

HISTORY OF PRESENT ILLNESS:
He has no complaints

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD)
- Hypertension
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
He is currently taking hydrochlorothiazide for his blood pressure.

Home Medications:
- Fluticasone propionate/salmeterol 250/50 mcg, 1 inhalation twice daily
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours as needed for shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Chronic obstructive pulmonary disease
- Brother: Hypertension

SOCIAL HISTORY:
He works in construction and admits to not wearing a mask when working in dusty environments

VITAL SIGNS:
His blood pressure is 138/86 mmHg, heart rate is 78 bpm, and respiratory rate is 16 breaths per minute

PHYSICAL EXAMINATION:
His lungs are clear to auscultation, and the rest of the examination is unremarkable

LABORATORY DATA:
- Hemoglobin: 15 g/dL
- WBC count: 7,000/mm^3
- Platelets: 250,000/mm^3
- Fasting glucose: 96 mg/dL
- Serum creatinine: 1.0 mg/dL
- Total cholesterol: 220 mg/dL
- LDL cholesterol: 150 mg/dL
- HDL cholesterol: 30 mg/dL
- Triglycerides: 200 mg/dL

IMAGING:
A chest X-ray shows no acute disease

PLAN:
The patient is interested in participating in a study that is recruiting individuals with different levels of exposure to airborne particulates. The study aims to compare the incidence of chronic obstructive pulmonary disease (COPD) between those with high exposure and those with low exposure. Given the patient's occupation and habits, he is considered to be in the high-exposure group.

## Encounter 1: source 2020-05-15, practice 2020-11-16, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath and daily cough with yellowish sputum

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath and daily cough with yellowish sputum

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of chronic obstructive pulmonary disease, last seen four months prior for routine examination and tobacco use counseling. He reports that he frequently coughs up yellowish sputum, especially in the mornings, and that climbing a single flight of stairs now leaves him winded. He denies fever, chills, or chest pain but mentions having had several chest infections requiring antibiotics in the past year.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Fluticasone propionate/salmeterol 250/50 mcg, 1 inhalation twice daily
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours as needed for shortness of breath
- Hydrochlorothiazide 25 mg by mouth once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Chronic obstructive pulmonary disease
- Brother: Hypertension

SOCIAL HISTORY:
He has smoked a pack of cigarettes per day for over 30 years and says he “isn’t ready to quit yet.” He works in construction and often handles materials that generate dust, though he does not wear a mask regularly.

PHYSICAL EXAMINATION:
On physical exam, he has a barrel-shaped chest, decreased breath sounds, and scattered rhonchi. Fingernails show evidence of nicotine staining. There is no clubbing or cyanosis.

LABORATORY DATA:
Sputum cultures are negative, and blood work is unremarkable.

IMAGING:
Chest imaging shows hyperinflated lungs and flattened diaphragms.

OTHER STUDIES:
Pulmonary function testing reveals airflow limitation with incomplete reversibility after bronchodilator administration. A 6-minute walk test reveals exertional dyspnea with a drop in oxygen saturation.

PLAN:
The physician discusses smoking cessation and long-term inhaler therapy. The patient is referred for pulmonary rehabilitation.

## Encounter 2: source 2021-05-15, practice 2021-11-26, outpatient, Pulmonology

Chief complaint: Routine check-up with concerns regarding lung cancer risk

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up with concerns regarding lung cancer risk

HISTORY OF PRESENT ILLNESS:
He returns for follow-up of his COPD with no complaints but concern regarding lung cancer risk given his smoking history.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (diagnosed 2020-05-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Fluticasone propionate/salmeterol 250/50 mcg, 1 inhalation twice daily
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours as needed for shortness of breath
- Hydrochlorothiazide 25 mg by mouth once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Chronic obstructive pulmonary disease
- Brother: Hypertension

SOCIAL HISTORY:
History of smoking

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

PLAN:
You discuss the potential use of low-dose computed tomography (LDCT) for lung cancer screening. The patient inquires about the accuracy of the test. You explain that the sensitivity and specificity of LDCT can be adjusted by changing the cutoff values for what is considered a positive result. For instance, defining a smaller nodule size as positive would increase the sensitivity but might decrease the specificity. The patient asks how this would affect his chances of actually having cancer if he had a positive test.

## Encounter 3: source 2022-05-15, practice 2022-11-13, ed, Emergency Medicine

Chief complaint: Six weeks of worsening crampy abdominal pain, constipation, and irritability

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Six weeks of worsening crampy abdominal pain, constipation, and irritability

HISTORY OF PRESENT ILLNESS:
The patient with COPD and tobacco use disorder presents for evaluation. Over the past month, coworkers report increasing irritability and difficulty concentrating during production tasks. The patient denies recent travel or dietary changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (diagnosed 2020-05-15)
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2021-05-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Fluticasone propionate/salmeterol 250/50 mcg, 1 inhalation twice daily
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours as needed for shortness of breath
- Hydrochlorothiazide 25 mg by mouth once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Chronic obstructive pulmonary disease
- Brother: Hypertension

SOCIAL HISTORY:
The patient is a current smoker with a 30-pack-year history.

PHYSICAL EXAMINATION:
Examination shows mild epigastric tenderness without rebound and slowed speech with intermittent confusion, though orientation remains intact.

LABORATORY DATA:
Laboratory evaluation reveals hemoglobin 9.8 g/dL, MCV 74 fL, serum iron 85 μg/dL, TIBC 360 μg/dL, transferrin saturation 24%, and ferritin 110 ng/mL. Liver enzymes and renal function are within normal limits.

IMAGING:
Abdominal X-ray shows no obstruction.

OTHER STUDIES:
Peripheral blood smear is shown below.

## Encounter 4: source 2023-05-15, practice 2023-11-13, outpatient, Pulmonology

Chief complaint: Three-month history of right shoulder pain with recent right hand tingling and weakness

ENCOUNTER NOTE
Date: 2023-05-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of right shoulder pain with recent right hand tingling and weakness

HISTORY OF PRESENT ILLNESS:
The patient returns with shoulder pain after lung cancer screening. He also reports a persistent cough with occasional streaks of blood. He denies any recent trauma or injury to the shoulder.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (diagnosed 2020-05-15)
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2021-05-15)
- Lead poisoning (diagnosed 2022-05-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Fluticasone propionate/salmeterol 250/50 mcg, 1 inhalation twice daily
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours as needed for shortness of breath
- Hydrochlorothiazide 25 mg by mouth once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Chronic obstructive pulmonary disease
- Brother: Hypertension

SOCIAL HISTORY:
With a 40-pack-year smoking history

PHYSICAL EXAMINATION:
On physical examination, ptosis and miosis are noted on the right side, along with decreased sweating on the right side of his face. The patient's right hand grip strength is diminished compared to the left.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 14.2 g/dL
- White blood cell count: 7,800 cells/mm^3
- Platelet count: 250,000 cells/mm^3
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL
- Calcium: 9.8 mg/dL

IMAGING:
Chest X-ray reveals a mass in the apex of the right lung.

## Encounter 5: source 2023-09-15, practice 2024-03-25, outpatient, Oncology

Chief complaint: Routine follow-up examination for recently diagnosed lung malignancy

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. James Okafor, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up examination for recently diagnosed lung malignancy

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of lung malignancy with COPD and tobacco use. He reports chronic cough and mild shortness of breath for several years, and denies recent weight loss, fever, chest pain, or hemoptysis.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (diagnosed 2020-05-15)
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2021-05-15)
- Lead poisoning (diagnosed 2022-05-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2023-05-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension

He was diagnosed with chronic bronchitis 4 years ago.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
His current medications include an inhaled bronchodilator and a long-acting corticosteroid.

Home Medications:
- Fluticasone propionate/salmeterol 250/50 mcg, 1 inhalation twice daily
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours as needed for shortness of breath
- Hydrochlorothiazide 25 mg by mouth once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Chronic obstructive pulmonary disease
- Brother: Hypertension

SOCIAL HISTORY:
Continues to smoke a pack of cigarettes daily

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
He appears comfortable at rest. Pulmonary auscultation reveals scattered expiratory wheezes and a prolonged expiratory phase. On physical examination, the physician notes bulbous enlargement of the distal fingers with loss of the normal nail angle and a spongy consistency of the nail beds. There is no cyanosis, peripheral edema, or digital tenderness.

## Encounter 6: source 2024-03-15, practice 2024-09-25, outpatient, Oncology

Chief complaint: Persistent cough, shortness of breath, and ten-pound weight loss

ENCOUNTER NOTE
Date: 2024-03-15
Provider: Dr. James Okafor, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough, shortness of breath, and ten-pound weight loss

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of his previously diagnosed lung cancer. He reports a 10-pound weight loss over the past two months and occasional hemoptysis.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (diagnosed 2020-05-15)
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2021-05-15)
- Lead poisoning (diagnosed 2022-05-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2023-05-15)
- Digital Clubbing (diagnosed 2023-09-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Fluticasone propionate/salmeterol 250/50 mcg, 1 inhalation twice daily
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours as needed for shortness of breath
- Hydrochlorothiazide 25 mg by mouth once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Chronic obstructive pulmonary disease
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Current smoker (1 pack per day since age 13; 40 pack-year history). Alcohol: Occasional (1-2 drinks per month). Occupation: Construction demolition laborer.

VITAL SIGNS:
His temperature is 37.2°C, blood pressure is 135/85 mmHg, pulse is 88/min, and respiratory rate is 20/min.

PHYSICAL EXAMINATION:
Physical examination reveals decreased breath sounds and dullness to percussion at the right lung base.

IMAGING:
Chest X-ray shows a right lower lobe mass.

PATHOLOGY:
A biopsy of the mass is performed and reveals non-small cell lung cancer (NSCLC).

OTHER STUDIES:
The patient is enrolled in a case-control study to evaluate the association between NSCLC and exposure to asbestos. The following contingency table is provided:

|                    | Cases (NSCLC) | Controls (No NSCLC) |
|--------------------|---------------|---------------------|
| Exposed to asbestos|      70       |         30          |
| Not exposed        |     130       |        270          |

## Encounter 7: source 2025-05-15, practice 2025-11-13, outpatient, Pulmonology

Chief complaint: Four-month history of progressive weight loss, cough, fatigue, and occasional hemoptysis

ENCOUNTER NOTE
Date: 2025-05-15
Provider: Dr. Priya Patel, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Four-month history of progressive weight loss, cough, fatigue, and occasional hemoptysis

HISTORY OF PRESENT ILLNESS:
The patient with Stage IIIB non-small cell lung cancer and COPD returns to Pulmonology presenting with a 4-month history of progressive weight loss, cough, and fatigue. He also reports occasional hemoptysis over the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (diagnosed 2020-05-15)
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2021-05-15)
- Lead poisoning (diagnosed 2022-05-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2023-05-15)
- Digital Clubbing (diagnosed 2023-09-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2024-03-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Fluticasone propionate/salmeterol 250/50 mcg, 1 inhalation twice daily
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours as needed for shortness of breath
- Hydrochlorothiazide 25 mg by mouth once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (diagnosed age 68, deceased)
- Mother: Chronic obstructive pulmonary disease
- Brother: Hypertension

SOCIAL HISTORY:
He has a 40-pack-year smoking history and quit smoking 2 years ago.

VITAL SIGNS:
His temperature is 37.2°C (99°F), blood pressure is 130/80 mmHg, pulse is 88/min, and respiratory rate is 20/min.

PHYSICAL EXAMINATION:
On physical examination, he is cachectic and has decreased breath sounds at the lung apices.

LABORATORY DATA:
Laboratory studies show a normal complete blood count and basic metabolic panel.

IMAGING:
A chest X-ray reveals bilateral upper lobe cavitary lesions. A subsequent high-resolution CT scan of the chest confirms multiple cavitary lesions in the apices of both lungs, one of which contains a mass-like lesion suggestive of an aspergilloma.

PLAN:
Aspergillus IgG serology is ordered.
