# synthetic_hospital patient 2190

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-11-24 (Thu) | outpatient | Pulmonology |
| 1 | 2020-07-15 | 2023-05-24 (Wed) | outpatient | Gastroenterology |
| 2 | 2021-01-15 | 2023-12-10 (Sun) | ed | Emergency Medicine |
| 3 | 2021-07-15 | 2024-05-31 (Fri) | ed | Cardiology |
| 4 | 2022-01-15 | 2024-11-24 (Sun) | ed | Emergency Medicine |
| 5 | 2022-03-15 | 2025-02-06 (Thu) | follow_up | Pulmonology |
| 6 | 2022-07-15 | 2025-06-06 (Fri) | outpatient | Pulmonology |
| 7 | 2023-01-15 | 2025-11-24 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Social alcohol use (1-2 drinks on weekends)",
 "family_history": [
  "Father: Lung cancer, diagnosed age 72",
  "Mother: Colorectal cancer, diagnosed age 68",
  "Brother: Myocardial infarction at age 65",
  "Sister: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "40 pack-year history (1 pack per day for 40 years), current smoker",
 "home_medications": [
  {
   "dose": "18 mcg inhaled once daily",
   "name": "Tiotropium"
  },
  {
   "dose": "50 mcg inhaled twice daily",
   "name": "Salmeterol"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for wheezing/shortness of breath",
   "name": "Albuterol"
  },
  {
   "dose": "40 mg by mouth once daily",
   "name": "Furosemide"
  },
  {
   "dose": "20 mg by mouth once daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg by mouth daily for 5 days during COPD exacerbations (occasional use)",
   "name": "Prednisone"
  }
 ],
 "surgical_history": [
  "Diagnostic colonoscopy with biopsy",
  "CT-guided lung biopsy"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage D)",
  "Cor pulmonale (pulmonary hypertension with right heart failure)",
  "Heart failure with reduced ejection fraction (HFrEF)",
  "Hypertension",
  "Non-small cell lung cancer, right upper lobe",
  "Metastatic adenocarcinoma of the sigmoid colon with hepatic metastases"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["COPD exacerbation with acute hypercapnic respiratory failure without altered mental status", "Chronic hypoxemic respiratory failure", "Chronic obstructive pulmonary disease", "Non-small cell lung cancer, right upper lobe", "Metastatic sigmoid colon adenocarcinoma with liver metastases", "Heart failure with reduced ejection fraction"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Tobacco use disorder", "Cachexia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-11-24, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath over the past 6 months with chronic productive cough

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath over the past 6 months with chronic productive cough

HISTORY OF PRESENT ILLNESS:
He reports a chronic productive cough with white sputum. He denies any chest pain, fever, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage D)
- Cor pulmonale (pulmonary hypertension with right heart failure)
- Heart failure with reduced ejection fraction (HFrEF)
- Hypertension
- Non-small cell lung cancer, right upper lobe
- Metastatic adenocarcinoma of the sigmoid colon with hepatic metastases

His past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Diagnostic colonoscopy with biopsy
- CT-guided lung biopsy

MEDICATIONS:
He takes for which he takes hydrochlorothiazide.

Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Salmeterol 50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing/shortness of breath
- Furosemide 40 mg by mouth once daily
- Lisinopril 20 mg by mouth once daily
- Prednisone 40 mg by mouth daily for 5 days during COPD exacerbations (occasional use)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, diagnosed age 72
- Mother: Colorectal cancer, diagnosed age 68
- Brother: Myocardial infarction at age 65
- Sister: Hypertension

SOCIAL HISTORY:
40-pack-year smoking history

VITAL SIGNS:
His oxygen saturation on room air is 92%

PHYSICAL EXAMINATION:
On physical examination, you note a barrel-chested appearance, decreased breath sounds, and prolonged expiration with an occasional wheeze

OTHER STUDIES:
Pulmonary function tests reveal a decreased FEV1/FVC ratio that does not significantly improve with bronchodilator administration

## Encounter 1: source 2020-07-15, practice 2023-05-24, outpatient, Gastroenterology

Chief complaint: Progressive right upper quadrant pain and unexplained weight loss of 10 kg

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive right upper quadrant pain and unexplained weight loss of 10 kg

HISTORY OF PRESENT ILLNESS:
The patient with severe COPD and cardiopulmonary comorbidities returns for evaluation following his January 2020 Pulmonology assessment. He reports a 6-month history of progressive right upper quadrant pain and unexplained weight loss of 10 kg. He reports a change in bowel habits, with episodes of constipation alternating with diarrhea.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage D)
- Cor pulmonale (pulmonary hypertension with right heart failure)
- Heart failure with reduced ejection fraction (HFrEF)
- Hypertension
- Non-small cell lung cancer, right upper lobe
- Metastatic adenocarcinoma of the sigmoid colon with hepatic metastases

PAST SURGICAL HISTORY:
- Diagnostic colonoscopy with biopsy
- CT-guided lung biopsy

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Salmeterol 50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing/shortness of breath
- Furosemide 40 mg by mouth once daily
- Lisinopril 20 mg by mouth once daily
- Prednisone 40 mg by mouth daily for 5 days during COPD exacerbations (occasional use)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, diagnosed age 72
- Mother: Colorectal cancer, diagnosed age 68
- Brother: Myocardial infarction at age 65
- Sister: Hypertension

SOCIAL HISTORY:
40-pack-year smoking history

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic. His abdomen is soft, with a palpable mass in the right lower quadrant and no rebound tenderness.

LABORATORY DATA:
Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- White blood cell count: 7,500/mm³ (Normal: 4,500-11,000/mm³)
- Platelets: 225,000/mm³ (Normal: 150,000-450,000/mm³)
- Carcinoembryonic antigen (CEA): 22 ng/mL (Normal: <3 ng/mL)
- Liver function tests: Mildly elevated AST and ALT

IMAGING:
A CT scan of the abdomen and pelvis shows a circumferential mass in the sigmoid colon with partial luminal obstruction and several hypodense lesions in the liver, the largest measuring 3 cm in diameter.

## Encounter 2: source 2021-01-15, practice 2023-12-10, ed, Emergency Medicine

Chief complaint: Increased shortness of breath and productive cough with greenish sputum

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increased shortness of breath and productive cough with greenish sputum

HISTORY OF PRESENT ILLNESS:
A 68-year-old male with severe COPD and metastatic cancer returns to the emergency department. He presents with increased shortness of breath and productive cough with greenish sputum for the past three days.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Metastatic sigmoid colon adenocarcinoma with liver metastases (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage D)
- Cor pulmonale (pulmonary hypertension with right heart failure)
- Heart failure with reduced ejection fraction (HFrEF)
- Hypertension
- Non-small cell lung cancer, right upper lobe
- Metastatic adenocarcinoma of the sigmoid colon with hepatic metastases

He has a known history of chronic obstructive pulmonary disease (COPD).

PAST SURGICAL HISTORY:
- Diagnostic colonoscopy with biopsy
- CT-guided lung biopsy

MEDICATIONS:
He is on tiotropium.

Home Medications:
- Salmeterol 50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing/shortness of breath
- Furosemide 40 mg by mouth once daily
- Lisinopril 20 mg by mouth once daily
- Prednisone 40 mg by mouth daily for 5 days during COPD exacerbations (occasional use)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, diagnosed age 72
- Mother: Colorectal cancer, diagnosed age 68
- Brother: Myocardial infarction at age 65
- Sister: Hypertension

SOCIAL HISTORY:
With a 40-pack-year smoking history

VITAL SIGNS:
His temperature is 37.8°C, blood pressure is 145/90 mmHg, heart rate is 102 beats per minute, and respiratory rate is 26 breaths per minute. His oxygen saturation is 88% on room air. Two hours later, his oxygen saturation improves to 96%, and his respiratory rate decreases to 12 breaths per minute.

PHYSICAL EXAMINATION:
On examination, he has diffuse wheezes and decreased breath sounds at the bases bilaterally. He becomes drowsy.

LABORATORY DATA:
Arterial blood gas (ABG) on room air shows:

- pH: 7.32
- PaCO2: 55 mmHg
- PaO2: 58 mmHg
- HCO3-: 29 mEq/L
- SaO2: 88%

Repeat ABG on 2 liters of oxygen per nasal cannula shows:

- pH: 7.28
- PaCO2: 65 mmHg
- PaO2: 110 mmHg
- HCO3-: 30 mEq/L
- SaO2: 96%

PLAN:
He is started on antibiotics for a COPD exacerbation and supplemental oxygen is initiated.

## Encounter 3: source 2021-07-15, practice 2024-05-31, ed, Cardiology

Chief complaint: Worsening fatigue, shortness of breath, orthopnea, and ankle swelling

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Williams, MD
Department: Cardiology
Visit Type: ed

CHIEF COMPLAINT:
Worsening fatigue, shortness of breath, orthopnea, and ankle swelling

HISTORY OF PRESENT ILLNESS:
The patient with severe COPD, cor pulmonale, and heart failure returns to the emergency department six months following his prior hypoxemic respiratory failure with worsening cardiopulmonary symptoms. He states that he gets winded just walking across his apartment and has been sleeping poorly due to the need to prop himself upright on several pillows at night to breathe comfortably. He has noticed some swelling in his ankles and occasional dry coughing fits when lying flat. He denies fevers, chills, or sputum production.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Metastatic sigmoid colon adenocarcinoma with liver metastases (diagnosed 2020-07-15)
- Chronic hypoxemic respiratory failure (diagnosed 2021-01-15)
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage D)
- Cor pulmonale (pulmonary hypertension with right heart failure)
- Heart failure with reduced ejection fraction (HFrEF)
- Hypertension
- Non-small cell lung cancer, right upper lobe
- Metastatic adenocarcinoma of the sigmoid colon with hepatic metastases

His past medical history includes hypertension diagnosed in his 40s,.

PAST SURGICAL HISTORY:
- Diagnostic colonoscopy with biopsy
- CT-guided lung biopsy

MEDICATIONS:
He takes for which he takes a calcium channel blocker.

Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Salmeterol 50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing/shortness of breath
- Furosemide 40 mg by mouth once daily
- Lisinopril 20 mg by mouth once daily
- Prednisone 40 mg by mouth daily for 5 days during COPD exacerbations (occasional use)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, diagnosed age 72
- Mother: Colorectal cancer, diagnosed age 68
- Brother: Myocardial infarction at age 65
- Sister: Hypertension

SOCIAL HISTORY:
He has smoked one pack of cigarettes per day for 40 years and drinks alcohol socially.

VITAL SIGNS:
On examination, his blood pressure is 156/92 mm Hg, and his heart rate is 88/min.

PHYSICAL EXAMINATION:
He appears tired and slightly dyspneic at rest. Jugular venous pressure is elevated to the angle of the mandible. Lung auscultation reveals bibasilar crackles, and cardiac exam demonstrates a displaced apical impulse and an S3 gallop. There is bilateral pitting edema in the lower extremities.

IMAGING:
Chest x-ray reveals prominent pulmonary vasculature and mild interstitial markings.

OTHER STUDIES:
A Swan-Ganz catheter is placed to measure hemodynamic parameters.

PLAN:
He is scheduled for an echocardiogram, and blood work is pending.

## Encounter 4: source 2022-01-15, practice 2024-11-24, ed, Emergency Medicine

Chief complaint: 3-day history of increased shortness of breath, productive cough with greenish sputum, and wheezing

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Lisa Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-day history of increased shortness of breath, productive cough with greenish sputum, and wheezing

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Metastatic sigmoid colon adenocarcinoma with liver metastases (diagnosed 2020-07-15)
- Chronic hypoxemic respiratory failure (diagnosed 2021-01-15)
- Heart failure with reduced ejection fraction (diagnosed 2021-07-15)
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage D)
- Cor pulmonale (pulmonary hypertension with right heart failure)
- Heart failure with reduced ejection fraction (HFrEF)
- Hypertension
- Non-small cell lung cancer, right upper lobe
- Metastatic adenocarcinoma of the sigmoid colon with hepatic metastases

He has a known history of chronic obstructive pulmonary disease (COPD).

PAST SURGICAL HISTORY:
- Diagnostic colonoscopy with biopsy
- CT-guided lung biopsy

MEDICATIONS:
He is on tiotropium.

Home Medications:
- Salmeterol 50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing/shortness of breath
- Furosemide 40 mg by mouth once daily
- Lisinopril 20 mg by mouth once daily
- Prednisone 40 mg by mouth daily for 5 days during COPD exacerbations (occasional use)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, diagnosed age 72
- Mother: Colorectal cancer, diagnosed age 68
- Brother: Myocardial infarction at age 65
- Sister: Hypertension

SOCIAL HISTORY:
With a 40-pack-year smoking history

VITAL SIGNS:
His temperature is 37.8°C (100°F), blood pressure is 145/90 mmHg, heart rate is 110 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
On examination, he is using accessory muscles to breathe and has bilateral wheezes and prolonged expiration.

LABORATORY DATA:
Arterial blood gas shows a pH of 7.32, PaCO2 of 50 mmHg, and PaO2 of 60 mmHg. His white blood cell count is 15,000/µL.

IMAGING:
Chest X-ray reveals no signs of consolidation, but there is hyperinflation of the lungs.

## Encounter 5: source 2022-03-15, practice 2025-02-06, follow_up, Pulmonology

Chief complaint: Follow-up after recent hospitalization for COPD exacerbation with persistent cough and unintentional weight loss

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Ahmed Hassan, MD
Department: Pulmonology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up after recent hospitalization for COPD exacerbation with persistent cough and unintentional weight loss

HISTORY OF PRESENT ILLNESS:
Returning after his January COPD exacerbation, he has been experiencing persistent cough and unintentional weight loss over the past 3 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Metastatic sigmoid colon adenocarcinoma with liver metastases (diagnosed 2020-07-15)
- Chronic hypoxemic respiratory failure (diagnosed 2021-01-15)
- Heart failure with reduced ejection fraction (diagnosed 2021-07-15)
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2022-01-15)
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage D)
- Cor pulmonale (pulmonary hypertension with right heart failure)
- Heart failure with reduced ejection fraction (HFrEF)
- Hypertension
- Non-small cell lung cancer, right upper lobe
- Metastatic adenocarcinoma of the sigmoid colon with hepatic metastases

PAST SURGICAL HISTORY:
- Diagnostic colonoscopy with biopsy
- CT-guided lung biopsy

MEDICATIONS:
He takes tiotropium, salmeterol, and occasional prednisone for COPD exacerbations.

Home Medications:
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing/shortness of breath
- Furosemide 40 mg by mouth once daily
- Lisinopril 20 mg by mouth once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, diagnosed age 72
- Mother: Colorectal cancer, diagnosed age 68
- Brother: Myocardial infarction at age 65
- Sister: Hypertension

SOCIAL HISTORY:
History of smoking

REVIEW OF SYSTEMS:
He denies any fever, night sweats, or hemoptysis.

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic. Lung auscultation reveals decreased breath sounds in the right upper lobe with dullness to percussion. No wheezing or crackles are noted.

IMAGING:
A chest X-ray shows a right upper lobe mass, and a subsequent CT scan confirms a 4 cm spiculated mass with associated mediastinal lymphadenopathy.

PATHOLOGY:
A biopsy of the mass is performed, and pathology reveals non-small cell lung cancer.

PLAN:
The patient is scheduled for a follow-up appointment to discuss the results.

## Encounter 6: source 2022-07-15, practice 2025-06-06, outpatient, Pulmonology

Chief complaint: Increasing shortness of breath and productive cough with white sputum

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing shortness of breath and productive cough with white sputum

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Metastatic sigmoid colon adenocarcinoma with liver metastases (diagnosed 2020-07-15)
- Chronic hypoxemic respiratory failure (diagnosed 2021-01-15)
- Heart failure with reduced ejection fraction (diagnosed 2021-07-15)
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2022-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2022-03-15)
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage D)
- Cor pulmonale (pulmonary hypertension with right heart failure)
- Heart failure with reduced ejection fraction (HFrEF)
- Hypertension
- Metastatic adenocarcinoma of the sigmoid colon with hepatic metastases

He has a history of hypertension and chronic obstructive pulmonary disease (COPD).

PAST SURGICAL HISTORY:
- Diagnostic colonoscopy with biopsy
- CT-guided lung biopsy

MEDICATIONS:
His current medications include tiotropium, albuterol as needed, and lisinopril.

Home Medications:
- Salmeterol 50 mcg inhaled twice daily
- Furosemide 40 mg by mouth once daily
- Prednisone 40 mg by mouth daily for 5 days during COPD exacerbations (occasional use)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, diagnosed age 72
- Mother: Colorectal cancer, diagnosed age 68
- Brother: Myocardial infarction at age 65
- Sister: Hypertension

SOCIAL HISTORY:
With a 40-pack-year smoking history

VITAL SIGNS:
His heart rate is 98 bpm, and blood pressure is 145/90 mm Hg.

PHYSICAL EXAMINATION:
On examination, he appears to be in mild respiratory distress, with an increased respiratory rate of 22 breaths per minute. Auscultation of the lungs reveals diffuse wheezing and prolonged expiration.
The patient has a 2+ pitting edema in his lower extremities and jugular venous distension.

LABORATORY DATA:
Laboratory values are as follows:

- Hematocrit: 57%
- Arterial blood gas (ABG):
- pH: 7.36
- PaCO2: 50 mm Hg
- PaO2: 56 mm Hg
- Pulse oximetry: 87% on room air

## Encounter 7: source 2023-01-15, practice 2025-11-24, ed, Emergency Medicine

Chief complaint: 2-day history of increasing shortness of breath and productive cough with greenish sputum

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of increasing shortness of breath and productive cough with greenish sputum

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with severe COPD (GOLD Stage D), cor pulmonale, heart failure with reduced ejection fraction, and metastatic malignancies returns for evaluation of recurrent respiratory symptoms. He presents with a 2-day history of increasing shortness of breath and a productive cough with greenish sputum.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Metastatic sigmoid colon adenocarcinoma with liver metastases (diagnosed 2020-07-15)
- Chronic hypoxemic respiratory failure (diagnosed 2021-01-15)
- Heart failure with reduced ejection fraction (diagnosed 2021-07-15)
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2022-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2022-03-15)
- Chronic obstructive pulmonary disease (COPD), severe (GOLD Stage D)
- Cor pulmonale (pulmonary hypertension with right heart failure)
- Heart failure with reduced ejection fraction (HFrEF)
- Hypertension
- Metastatic adenocarcinoma of the sigmoid colon with hepatic metastases

Cor pulmonale diagnosed 1 year ago.

PAST SURGICAL HISTORY:
- Diagnostic colonoscopy with biopsy
- CT-guided lung biopsy

MEDICATIONS:
He takes tiotropium, salmeterol, and furosemide.

Home Medications:
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing/shortness of breath
- Lisinopril 20 mg by mouth once daily
- Prednisone 40 mg by mouth daily for 5 days during COPD exacerbations (occasional use)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, diagnosed age 72
- Mother: Colorectal cancer, diagnosed age 68
- Brother: Myocardial infarction at age 65
- Sister: Hypertension

SOCIAL HISTORY:
40-pack-year smoking history

VITAL SIGNS:
Respiratory rate of 28 breaths per minute

PHYSICAL EXAMINATION:
On examination, he is tachypneic, uses accessory muscles to breathe, and has bilateral lower extremity edema. Lung auscultation reveals diffuse wheezing and prolonged expiration.

LABORATORY DATA:
Arterial blood gas (ABG) shows: pH: 7.32, PaCO2: 60 mmHg, PaO2: 55 mmHg, HCO3-: 32 mEq/L. Serum creatinine is 1.8 mg/dL, increased from a baseline of 1.0 mg/dL a month ago. Urinalysis is normal.
