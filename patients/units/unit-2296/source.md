# synthetic_hospital patient 2296

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-11-09 (Mon) | outpatient | Dermatology |
| 1 | 2020-07-15 | 2021-04-28 (Wed) | outpatient | Otolaryngology |
| 2 | 2021-05-15 | 2022-03-08 (Tue) | outpatient | Pulmonary & Critical Care |
| 3 | 2022-05-15 | 2023-02-20 (Mon) | outpatient | Hematology & Oncology |
| 4 | 2023-05-15 | 2024-03-01 (Fri) | outpatient | Cardiology |
| 5 | 2024-05-15 | 2025-03-03 (Mon) | outpatient | Pulmonary & Critical Care |
| 6 | 2024-11-15 | 2025-09-08 (Mon) | inpatient | Internal Medicine |
| 7 | 2025-01-15 | 2025-11-04 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired farmer",
 "alcohol_use": "3-4 alcoholic beverages most evenings",
 "family_history": [
  "Father: Chronic obstructive pulmonary disease",
  "Father: Lung cancer",
  "Mother: Cardiovascular disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 45 pack-year history (1 pack per day)",
 "home_medications": [
  {
   "dose": "18 mcg inhaled once daily",
   "name": "Tiotropium"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for shortness of breath",
   "name": "Albuterol"
  },
  {
   "dose": "250/50 mcg inhaled twice daily",
   "name": "Fluticasone/Salmeterol"
  },
  {
   "dose": "40 mg oral once daily",
   "name": "Furosemide"
  },
  {
   "dose": "2 liters per minute via nasal cannula continuous",
   "name": "Oxygen"
  }
 ],
 "surgical_history": [
  "Surgical excision of squamous cell carcinoma, lower lip",
  "Direct laryngoscopy with biopsy and partial laryngectomy for laryngeal squamous cell carcinoma",
  "Chest tube thoracostomy for parapneumonic effusion"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (severe, GOLD stage D)",
  "Pulmonary hypertension",
  "Chronic right heart failure (Cor pulmonale)",
  "Secondary polycythemia",
  "History of squamous cell carcinoma of the larynx",
  "History of cutaneous squamous cell carcinoma of the lower lip"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease)", "Tricuspid Regurgitation", "Secondary polycythemia", "Squamous cell carcinoma of larynx", "Complicated parapneumonic effusion", "Cutaneous Squamous Cell Carcinoma (surgical indication)"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Tobacco use disorder"]

## Encounter 0: source 2020-01-15, practice 2020-11-09, outpatient, Dermatology

Chief complaint: Persistent lesion on lower lip present for several months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent lesion on lower lip present for several months

HISTORY OF PRESENT ILLNESS:
He reports that the lesion has been present for several months and has gradually increased in size.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (severe, GOLD stage D)
- Pulmonary hypertension
- Chronic right heart failure (Cor pulmonale)
- Secondary polycythemia
- History of squamous cell carcinoma of the larynx
- History of cutaneous squamous cell carcinoma of the lower lip

PAST SURGICAL HISTORY:
- Surgical excision of squamous cell carcinoma, lower lip
- Direct laryngoscopy with biopsy and partial laryngectomy for laryngeal squamous cell carcinoma
- Chest tube thoracostomy for parapneumonic effusion

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Furosemide 40 mg oral once daily
- Oxygen 2 liters per minute via nasal cannula continuous

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Father: Lung cancer
- Mother: Cardiovascular disease

SOCIAL HISTORY:
The patient has a history of extensive sun exposure due to his occupation as a farmer and admits to smoking a pack of cigarettes daily for the past 40 years.

REVIEW OF SYSTEMS:
He denies any recent weight loss, fever, or night sweats.

PHYSICAL EXAMINATION:
On physical examination, there is a 1 cm, firm, ulcerated nodule on the vermilion border of the lower lip with a small area of crusting. There is no palpable cervical lymphadenopathy.

PATHOLOGY:
A biopsy of the lesion is performed, and histopathology reveals invasive cords of squamous cells with keratin pearls.

## Encounter 1: source 2020-07-15, practice 2021-04-28, outpatient, Otolaryngology

Chief complaint: Progressive hoarseness and intermittent dysphagia for 3 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive hoarseness and intermittent dysphagia for 3 months

HISTORY OF PRESENT ILLNESS:
The patient returns to Otolaryngology six months following treatment for cutaneous squamous cell carcinoma of the lower lip. He presents with progressive hoarseness and intermittent dysphagia of three months duration. He has also noticed mild throat discomfort and a sensation of a lump when swallowing solids.

PAST MEDICAL HISTORY:
Active Problem List:
- Cutaneous Squamous Cell Carcinoma (surgical indication) (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (severe, GOLD stage D)
- Pulmonary hypertension
- Chronic right heart failure (Cor pulmonale)
- Secondary polycythemia
- History of squamous cell carcinoma of the larynx
- History of cutaneous squamous cell carcinoma of the lower lip

PAST SURGICAL HISTORY:
- Surgical excision of squamous cell carcinoma, lower lip
- Direct laryngoscopy with biopsy and partial laryngectomy for laryngeal squamous cell carcinoma
- Chest tube thoracostomy for parapneumonic effusion

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Furosemide 40 mg oral once daily
- Oxygen 2 liters per minute via nasal cannula continuous

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Father: Lung cancer
- Mother: Cardiovascular disease

SOCIAL HISTORY:
The patient has smoked 1.5 packs of cigarettes daily for 40 years and drinks 3–4 alcoholic beverages most evenings.

REVIEW OF SYSTEMS:
He denies fever, night sweats, or unintentional weight loss.

PHYSICAL EXAMINATION:
Physical examination reveals a husky voice and mild anterior neck tenderness without visible swelling. Indirect laryngoscopy demonstrates an irregular exophytic mass involving the left true vocal cord with limited mobility. No palpable neck masses are detected.

PATHOLOGY:
A biopsy of the lesion reveals invasive squamous cell carcinoma.

PLAN:
The patient is scheduled for staging studies, and the oncologist explains that lymphatic spread is the most likely initial route of metastasis for this tumor.

## Encounter 2: source 2021-05-15, practice 2022-03-08, outpatient, Pulmonary & Critical Care

Chief complaint: Progressive shortness of breath, chronic productive cough, and lower extremity swelling

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Emily Johnson, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath, chronic productive cough, and lower extremity swelling

HISTORY OF PRESENT ILLNESS:
Patient with severe COPD and laryngeal and lip SCC returns with 6-month history of progressive shortness of breath, chronic productive cough, and recent lower extremity swelling. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Cutaneous Squamous Cell Carcinoma (surgical indication) (diagnosed 2020-01-15)
- Squamous cell carcinoma of larynx (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (severe, GOLD stage D)
- Pulmonary hypertension
- Chronic right heart failure (Cor pulmonale)
- Secondary polycythemia
- History of squamous cell carcinoma of the larynx
- History of cutaneous squamous cell carcinoma of the lower lip

PAST SURGICAL HISTORY:
- Surgical excision of squamous cell carcinoma, lower lip
- Direct laryngoscopy with biopsy and partial laryngectomy for laryngeal squamous cell carcinoma
- Chest tube thoracostomy for parapneumonic effusion

MEDICATIONS:
Tiotropium, albuterol, and fluticasone/salmeterol

Home Medications:
- Furosemide 40 mg oral once daily
- Oxygen 2 liters per minute via nasal cannula continuous

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Father: Lung cancer
- Mother: Cardiovascular disease

SOCIAL HISTORY:
40-pack-year smoking history

VITAL SIGNS:
His blood pressure is 135/85 mmHg, heart rate is 98 bpm, respiratory rate is 22 breaths/min, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
He has jugular venous distension, a prominent P2 on auscultation, and a palpable right ventricular heave. His abdomen is soft with mild hepatomegaly, and there is 2+ pitting edema in both lower extremities up to the mid-shins.

LABORATORY DATA:
Hemoglobin: 15.2 g/dL
WBC count: 7,800/mm^3
Platelet count: 250,000/mm^3
BNP: 250 pg/mL
Serum creatinine: 1.1 mg/dL
AST: 40 U/L
ALT: 35 U/L
Albumin: 3.9 g/dL
INR: 1.0

IMAGING:
Chest X-ray shows hyperinflated lungs and a prominent pulmonary artery

OTHER STUDIES:
An ECG reveals right axis deviation and right ventricular hypertrophy

## Encounter 3: source 2022-05-15, practice 2023-02-20, outpatient, Hematology & Oncology

Chief complaint: Fatigue, headaches, mild dizziness and difficulty concentrating

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. David Park, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, headaches, mild dizziness and difficulty concentrating

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with severe COPD (GOLD stage D), pulmonary hypertension, chronic right heart failure, history of laryngeal and cutaneous squamous cell carcinoma, and secondary polycythemia who returns for follow-up evaluation. He presents with fatigue, headaches, and also reports mild dizziness and difficulty concentrating.

PAST MEDICAL HISTORY:
Active Problem List:
- Cutaneous Squamous Cell Carcinoma (surgical indication) (diagnosed 2020-01-15)
- Squamous cell carcinoma of larynx (diagnosed 2020-07-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2021-05-15)
- Chronic obstructive pulmonary disease (severe, GOLD stage D)
- Pulmonary hypertension
- Chronic right heart failure (Cor pulmonale)
- Secondary polycythemia
- History of squamous cell carcinoma of the larynx
- History of cutaneous squamous cell carcinoma of the lower lip

He has a history of chronic obstructive pulmonary disease.

PAST SURGICAL HISTORY:
- Surgical excision of squamous cell carcinoma, lower lip
- Direct laryngoscopy with biopsy and partial laryngectomy for laryngeal squamous cell carcinoma
- Chest tube thoracostomy for parapneumonic effusion

MEDICATIONS:
He takes managed with inhalers.

Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Furosemide 40 mg oral once daily
- Oxygen 2 liters per minute via nasal cannula continuous

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Father: Lung cancer
- Mother: Cardiovascular disease

SOCIAL HISTORY:
Continues to smoke one pack per day.

VITAL SIGNS:
His oxygen saturation on room air is 89%.

PHYSICAL EXAMINATION:
On examination, he has decreased breath sounds bilaterally and mild cyanosis of the lips.
Neurologic examination is normal. There is no splenomegaly or lymphadenopathy.

LABORATORY DATA:
Laboratory results:
Hemoglobin 19.0 g/dL
Hematocrit 57%
MCV 89 fL
Serum erythropoietin Elevated

## Encounter 4: source 2023-05-15, practice 2024-03-01, outpatient, Cardiology

Chief complaint: Increasing lower extremity swelling, exertional dyspnea, and progressive abdominal fullness

ENCOUNTER NOTE
Date: 2023-05-15
Provider: Dr. Aisha Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing lower extremity swelling, exertional dyspnea, and progressive abdominal fullness

HISTORY OF PRESENT ILLNESS:
The patient with chronic right heart failure returns with worsening symptoms. He has also noticed progressive abdominal fullness and decreased exercise tolerance over the past 3 months. He denies chest pain or orthopnea.

PAST MEDICAL HISTORY:
Active Problem List:
- Cutaneous Squamous Cell Carcinoma (surgical indication) (diagnosed 2020-01-15)
- Squamous cell carcinoma of larynx (diagnosed 2020-07-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2021-05-15)
- Secondary polycythemia (diagnosed 2022-05-15)
- Chronic obstructive pulmonary disease (severe, GOLD stage D)
- Pulmonary hypertension
- Chronic right heart failure (Cor pulmonale)
- History of squamous cell carcinoma of the larynx
- History of cutaneous squamous cell carcinoma of the lower lip

PAST SURGICAL HISTORY:
- Surgical excision of squamous cell carcinoma, lower lip
- Direct laryngoscopy with biopsy and partial laryngectomy for laryngeal squamous cell carcinoma
- Chest tube thoracostomy for parapneumonic effusion

MEDICATIONS:
He takes inhaled tiotropium and albuterol.

Home Medications:
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Furosemide 40 mg oral once daily
- Oxygen 2 liters per minute via nasal cannula continuous

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Father: Lung cancer
- Mother: Cardiovascular disease

SOCIAL HISTORY:
45-pack-year smoking history

VITAL SIGNS:
On physical examination, his temperature is 37.2°C (99°F), pulse is 92/min, blood pressure is 128/76 mm Hg, and respirations are 20/min

PHYSICAL EXAMINATION:
Jugular venous distension is observed at 8 cm above the sternal angle. There is bilateral pitting edema to the mid-shins and hepatomegaly. A loud P2 is noted on cardiac auscultation. Cardiac auscultation reveals a murmur.

OTHER STUDIES:
An ECG shows right axis deviation and a right bundle branch block.

## Encounter 5: source 2024-05-15, practice 2025-03-03, outpatient, Pulmonary & Critical Care

Chief complaint: Increasing shortness of breath, productive cough, and 4-kg weight gain

ENCOUNTER NOTE
Date: 2024-05-15
Provider: Dr. Emily Johnson, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing shortness of breath, productive cough, and 4-kg weight gain

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of severe COPD and chronic right heart failure followed since 2021. He reports increasing shortness of breath, productive cough, and 4-kg weight gain over the past month. He reports that his symptoms have progressively worsened despite adhering to his inhaler regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Cutaneous Squamous Cell Carcinoma (surgical indication) (diagnosed 2020-01-15)
- Squamous cell carcinoma of larynx (diagnosed 2020-07-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2021-05-15)
- Secondary polycythemia (diagnosed 2022-05-15)
- Tricuspid Regurgitation (diagnosed 2023-05-15)
- Chronic obstructive pulmonary disease (severe, GOLD stage D)
- Pulmonary hypertension
- Chronic right heart failure (Cor pulmonale)
- History of squamous cell carcinoma of the larynx
- History of cutaneous squamous cell carcinoma of the lower lip

PAST SURGICAL HISTORY:
- Surgical excision of squamous cell carcinoma, lower lip
- Direct laryngoscopy with biopsy and partial laryngectomy for laryngeal squamous cell carcinoma
- Chest tube thoracostomy for parapneumonic effusion

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Furosemide 40 mg oral once daily
- Oxygen 2 liters per minute via nasal cannula continuous

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Father: Lung cancer
- Mother: Cardiovascular disease

SOCIAL HISTORY:
Smoking: Current smoker, 45 pack-year history (1 pack per day). Alcohol: 3-4 alcoholic beverages most evenings. Occupation: Retired farmer.

PHYSICAL EXAMINATION:
On physical examination, you note an elevated jugular venous pressure, a prominent V wave, and a low-pitched, holosystolic murmur best heard along the left lower sternal border that increases with inspiration. The patient's abdomen is distended with a palpable, pulsatile liver. Lower extremity pitting edema is present bilaterally.

IMAGING:
An echocardiogram reveals right ventricular hypertrophy and dilation with tricuspid regurgitation.

OTHER STUDIES:
Right heart catheterization shows a pulmonary systolic pressure of 35 mm Hg.

## Encounter 6: source 2024-11-15, practice 2025-09-08, inpatient, Internal Medicine

Chief complaint: Worsening shortness of breath and bilateral leg swelling

ENCOUNTER NOTE
Date: 2024-11-15
Provider: Dr. James Wilson, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Worsening shortness of breath and bilateral leg swelling

PAST MEDICAL HISTORY:
Active Problem List:
- Cutaneous Squamous Cell Carcinoma (surgical indication) (diagnosed 2020-01-15)
- Squamous cell carcinoma of larynx (diagnosed 2020-07-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2021-05-15)
- Secondary polycythemia (diagnosed 2022-05-15)
- Tricuspid Regurgitation (diagnosed 2023-05-15)
- Chronic obstructive pulmonary disease (severe, GOLD stage D)
- Pulmonary hypertension
- Chronic right heart failure (Cor pulmonale)
- History of squamous cell carcinoma of the larynx
- History of cutaneous squamous cell carcinoma of the lower lip

He has a history of long-standing COPD and has required multiple hospitalizations for acute exacerbations.

PAST SURGICAL HISTORY:
- Surgical excision of squamous cell carcinoma, lower lip
- Direct laryngoscopy with biopsy and partial laryngectomy for laryngeal squamous cell carcinoma
- Chest tube thoracostomy for parapneumonic effusion

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Furosemide 40 mg oral once daily
- Oxygen 2 liters per minute via nasal cannula continuous

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Father: Lung cancer
- Mother: Cardiovascular disease

SOCIAL HISTORY:
Smoking: Current smoker, 45 pack-year history (1 pack per day). Alcohol: 3-4 alcoholic beverages most evenings. Occupation: Retired farmer.

PHYSICAL EXAMINATION:
On physical examination, he has elevated jugular venous pressure, bilateral lower extremity pitting edema, and a loud P2 component of the second heart sound. Lung auscultation reveals faint bibasilar crackles.

## Encounter 7: source 2025-01-15, practice 2025-11-04, ed, Emergency Medicine

Chief complaint: Increasing shortness of breath, productive cough with yellowish sputum, and fever

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Maria Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing shortness of breath, productive cough with yellowish sputum, and fever

HISTORY OF PRESENT ILLNESS:
This patient with severe COPD and cor pulmonale returns. He reports recent hospitalization for COPD exacerbation, during which he received antibiotics and steroids.

PAST MEDICAL HISTORY:
Active Problem List:
- Cutaneous Squamous Cell Carcinoma (surgical indication) (diagnosed 2020-01-15)
- Squamous cell carcinoma of larynx (diagnosed 2020-07-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2021-05-15)
- Secondary polycythemia (diagnosed 2022-05-15)
- Tricuspid Regurgitation (diagnosed 2023-05-15)
- Chronic obstructive pulmonary disease (severe, GOLD stage D)
- Pulmonary hypertension
- Chronic right heart failure (Cor pulmonale)
- History of squamous cell carcinoma of the larynx
- History of cutaneous squamous cell carcinoma of the lower lip

PAST SURGICAL HISTORY:
- Surgical excision of squamous cell carcinoma, lower lip
- Direct laryngoscopy with biopsy and partial laryngectomy for laryngeal squamous cell carcinoma
- Chest tube thoracostomy for parapneumonic effusion

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Furosemide 40 mg oral once daily
- Oxygen 2 liters per minute via nasal cannula continuous

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Father: Lung cancer
- Mother: Cardiovascular disease

SOCIAL HISTORY:
Smoking: Current smoker, 45 pack-year history (1 pack per day). Alcohol: 3-4 alcoholic beverages most evenings. Occupation: Retired farmer.

VITAL SIGNS:
Temperature is 38.7°C, heart rate is 102 beats per minute, respiratory rate is 26 breaths per minute, and oxygen saturation is 89% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds and dullness to percussion at the right lung base.

LABORATORY DATA:
Laboratory tests reveal leukocytosis with a left shift.

IMAGING:
Chest X-ray shows a right lower lobe consolidation with a blunted right costophrenic angle.

OTHER STUDIES:
Thoracentesis yields cloudy pleural fluid with the following analysis:

- pH: 7.10
- Glucose: 30 mg/dL
- LDH: 600 U/L (serum LDH: 200 U/L)
- Total protein: 4.5 g/dL (serum total protein: 7.0 g/dL)
- WBC: 20,000 cells/µL with 90% neutrophils
- Gram stain: Gram-positive cocci in clusters
